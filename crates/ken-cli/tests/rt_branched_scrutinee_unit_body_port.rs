//! `RT-BRANCHED-SCRUTINEE-UNIT-BODY-PORT` D1 witness.
//!
//! MEASURED: the observer records resolver entry, plain-Match branch entry, successful
//! plain-Match arm-body lookup, and the direct non-`Construct` route-1 return
//! while this checked Ken source lowers.
//! CLAIMED: D2 enters the carried child's owning plain `Match`, walks its arms,
//! then advances past route 1.
//!
//! ⭐ **Updated by `RT-BRANCH-LOCAL-DECLARED-CALLABLE` `D1`.** Before that cut
//! the walk STOPPED at arm 0, because arm 0 constructs a different constructor
//! than the selected case and its missing recursive position vetoed the whole
//! source. The branch-local partition makes an out-of-bucket arm non-vetoing, so
//! the walk now reaches arm 1. The number below is that property, not a
//! snapshot: it goes back to 1 exactly if the partition is reverted.

#![cfg(target_os = "linux")]

const BRANCHED_SCRUTINEE_SOURCE: &str = r#"program capabilities FS AFull
fn rt_branched_body (_buffer : BufferHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult Unit Unit) (ResourceBodyOk Unit Unit MkUnit)

proc rt_branched_endpoint_buffer
  (file : Resource ResourceKind.FsHandle) (buffer : BufferHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError ReadProgress) (ResourceBodyResult Unit Unit)
    (readAt AFull file (0 : Int) buffer
      (MkBufferWindow (8 : Int) (4 : Int)))
    (\outcome. match outcome {
      Err error |-> Ret (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit);
      Ok progress |-> Ret (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (ResourceBodyResult Unit Unit) (ResourceBodyOk Unit Unit MkUnit)
    })

proc rt_branched_after_buffer
  (outcome : Result ResourceError (ResourceBracketResult Unit Unit))
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  match outcome {
    Err error |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit);
    Ok bracket |-> Ret (Coproduct (FSOp AFull) AmbientOp)
      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
      (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit)
  }

proc rt_branched_file (file : Resource ResourceKind.FsHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result ResourceError (ResourceBracketResult Unit Unit))
    (ResourceBodyResult Unit Unit)
    (withBuffer AFull Unit Unit (8 : Int) (rt_branched_endpoint_buffer file))
    (\outcome. rt_branched_after_buffer outcome)

proc rt_branched_done
  (outcome : Result FileError (ResourceBracketResult Unit Unit))
  : HostIO AFull ExitCode visits [FS] =
  match outcome {
    Err error |-> host_exit AFull (Failure 71);
    Ok bracket |-> host_exit AFull (Failure 72)
  }

proc rt_branched_stage (cap : Cap AFull)
  : HostIO AFull ExitCode visits [FS] =
  bind (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (Result FileError (ResourceBracketResult Unit Unit)) ExitCode
    (withResource AFull Unit Unit cap (bytes_encode "source")
      ResourceRead rt_branched_file)
    (\outcome. rt_branched_done outcome)

proc main (_input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match caps {
    MkProgramCaps cap |-> rt_branched_stage cap
  }
"#;

// ── STACK FLOOR, AND WHAT IT COSTS ──────────────────────────────────────────
//
// These tests drive the elaborator's recursive descent and had NO stack grant:
// they ran on libtest's ambient 2 MiB thread and passed on an ACCIDENTAL
// margin, which an unrelated elaborator change then consumed. That is the same
// shape as the guard deleted from `abi_s6_mapping_surface_native` — a test
// whose need sits just under the ambient default has no signal of its own
// until something else spends the remainder, and the arriving change wears the
// blame.
//
// MEASURED, both arms built and the binaries invoked directly, bisected on
// `RUST_MIN_STACK` at 64 KiB resolution:
//
//     without the change   1984 KiB     with it   2112 KiB     delta 128 KiB
//     libtest ambient      2048 KiB     -> 1984 < 2048 < 2112, so it tips
//
// The floor below is DERIVED RATHER THAN CHOSEN, and written as arithmetic so
// the spend is legible. The property the old 3.1% margin never had was that
// anybody could see what they were spending.
//
// THIS IS A STOPGAP AND ITS SUBJECT IS NOT THIS FILE. `1984 KiB` is consumed
// before any test-specific work begins and is IDENTICAL across three files in
// two crates — a shared dominating consumer. Raising floors buys time; it does
// not address a baseline that eats 97% of the ambient stack.
const MEASURED_NEED_KIB: usize = 2112;
/// The bisection could not resolve finer than this, so the true need may be
/// up to one step above what was observed.
const BISECTION_RESOLUTION_KIB: usize = 64;
/// One elaborator change of the size that tipped these cost 128 KiB here.
const OBSERVED_INCREMENT_KIB: usize = 128;
/// How many further such changes this floor is buying room for. State the
/// number rather than a round total: this is the quantity being spent.
const INCREMENTS_OF_HEADROOM: usize = 8;
const TEST_STACK_BYTES: usize = (MEASURED_NEED_KIB
    + BISECTION_RESOLUTION_KIB
    + INCREMENTS_OF_HEADROOM * OBSERVED_INCREMENT_KIB)
    * 1024;

/// Run a test body on a thread with the floor above.
///
/// The thread is NAMED: an overflow on an unnamed `Builder` reports
/// `thread '<unknown>'`, which cost this investigation a step when exactly
/// that happened elsewhere. A future overflow here names its own test.
fn with_stack_floor(name: &str, body: impl FnOnce() + Send + 'static) {
    let handle = std::thread::Builder::new()
        .name(name.to_string())
        .stack_size(TEST_STACK_BYTES)
        .spawn(body)
        .expect("spawning the floored test body must succeed");
    if let Err(payload) = handle.join() {
        std::panic::resume_unwind(payload);
    }
}

#[test]
fn two_arm_plain_match_over_runtime_var_reaches_recursive_unit_body_route1() {
    with_stack_floor(
        "two_arm_plain_match_over_runtime_var_reaches_recursive_unit_body_route1",
        two_arm_plain_match_over_runtime_var_reaches_recursive_unit_body_route1_body,
    );
}

fn two_arm_plain_match_over_runtime_var_reaches_recursive_unit_body_route1_body() {
    let root = tempfile::tempdir().expect("temporary native-build root");
    let (result, route1) = ken_runtime::with_branched_scrutinee_unit_body_route1(|| {
        ken_cli::build_native_program(
            BRANCHED_SCRUTINEE_SOURCE,
            ken_cli::SourceFormat::Ken,
            "rt_branched_scrutinee_unit_body",
            root.path(),
            ken_runtime::boundary_resource_profile::starter_smoke_profile(),
        )
    });

    // M6 completes the whole program, so this observer now sees later composed
    // and transport-driven resolver invocations too. Select the original
    // semantic row by the properties this test has always asserted: it enters
    // the plain-Match branch, walks both arms, and advances past direct route 1.
    // Do not freeze the incidental total row count.
    let intended: Vec<_> = route1
        .iter()
        .filter(|row| !row.route1 && row.match_branch_entered)
        .collect();
    assert_eq!(
        intended.len(),
        1,
        "exactly one resolver row must be the branched-scrutinee path that advances \
         past route 1: {route1:?}"
    );
    let intended = intended[0];
    // `RT-BRANCH-LOCAL-DECLARED-CALLABLE` `D1`: both arms are walked. Measured
    // at the bucket filter, arm 0 constructs `ITree::Ret` while the selected
    // case is `ITree::Vis` — the exact `Ret`/`Vis` asymmetry the node was cut
    // for. `Ret` is out of the bucket, so its missing recursive position no
    // longer vetoes, and arm 1 (`ITree::Vis`, in the bucket) is reached. A
    // revert of the partition returns this to 1, which is what makes the
    // assertion discriminating rather than decorative.
    assert_eq!(
        intended.match_arms_walked, 2,
        "D1 must walk past the out-of-bucket arm 0 and reach arm 1: {intended:?}"
    );
    eprintln!(
        "RT_BRANCHED_SCRUTINEE_UNIT_BODY_ROUTE1 entered={} route1={} match_arms_walked={}",
        1,
        usize::from(intended.route1),
        intended.match_arms_walked,
    );
    result
        .expect("M6 must complete after preserving the branched-scrutinee path through both arms");
}

#[test]
fn suppressing_match_branch_entry_is_a_recorder_positive_control() {
    with_stack_floor(
        "suppressing_match_branch_entry_is_a_recorder_positive_control",
        suppressing_match_branch_entry_is_a_recorder_positive_control_body,
    );
}

fn suppressing_match_branch_entry_is_a_recorder_positive_control_body() {
    let root = tempfile::tempdir().expect("temporary native-build root");
    let (_, rows) = ken_runtime::with_branched_scrutinee_unit_body_route1(|| {
        ken_runtime::with_branched_scrutinee_unit_body_match_branch_entry_suppressed(|| {
            ken_cli::build_native_program(
                BRANCHED_SCRUTINEE_SOURCE,
                ken_cli::SourceFormat::Ken,
                "rt_branched_scrutinee_unit_body_mutation",
                root.path(),
                ken_runtime::boundary_resource_profile::starter_smoke_profile(),
            )
        })
    });
    assert_eq!(rows.len(), 1, "the resolver entry remains observable");
    assert!(
        !rows[0].route1,
        "the old route-1 observation remains satisfied"
    );
    assert!(
        !rows[0].match_branch_entered,
        "the pre-recorder suppression is a positive control for branch-entry recording"
    );
}
