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
  (file : Resource FsHandle) (buffer : BufferHandle)
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

proc rt_branched_file (file : Resource FsHandle)
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

#[test]
fn two_arm_plain_match_over_runtime_var_reaches_recursive_unit_body_route1() {
    let root = tempfile::tempdir().expect("temporary native-build root");
    let (result, route1) = ken_runtime::with_branched_scrutinee_unit_body_route1(|| {
        ken_cli::build_native_program(
            BRANCHED_SCRUTINEE_SOURCE,
            ken_cli::SourceFormat::Ken,
            "rt_branched_scrutinee_unit_body",
            root.path(),
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
    let root = tempfile::tempdir().expect("temporary native-build root");
    let (_, rows) = ken_runtime::with_branched_scrutinee_unit_body_route1(|| {
        ken_runtime::with_branched_scrutinee_unit_body_match_branch_entry_suppressed(|| {
            ken_cli::build_native_program(
                BRANCHED_SCRUTINEE_SOURCE,
                ken_cli::SourceFormat::Ken,
                "rt_branched_scrutinee_unit_body_mutation",
                root.path(),
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
