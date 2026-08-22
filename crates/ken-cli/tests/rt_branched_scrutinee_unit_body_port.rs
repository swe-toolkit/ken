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

    assert_eq!(route1.len(), 1, "the resolver must still be entered once");
    assert!(
        !route1[0].route1,
        "D2 must advance past route 1: {route1:?}"
    );
    assert!(
        route1[0].match_branch_entered,
        "D2 must enter the plain Match branch"
    );
    // `RT-BRANCH-LOCAL-DECLARED-CALLABLE` `D1`: both arms are walked. Measured
    // at the bucket filter, arm 0 constructs `ITree::Ret` while the selected
    // case is `ITree::Vis` — the exact `Ret`/`Vis` asymmetry the node was cut
    // for. `Ret` is out of the bucket, so its missing recursive position no
    // longer vetoes, and arm 1 (`ITree::Vis`, in the bucket) is reached. A
    // revert of the partition returns this to 1, which is what makes the
    // assertion discriminating rather than decorative.
    assert_eq!(
        route1[0].match_arms_walked, 2,
        "D1 must walk past the out-of-bucket arm 0 and reach arm 1: {route1:?}"
    );
    eprintln!(
        "RT_BRANCHED_SCRUTINEE_UNIT_BODY_ROUTE1 entered={} route1={} match_arms_walked={}",
        route1.len(),
        route1.iter().filter(|row| row.route1).count(),
        route1.iter().map(|row| row.match_arms_walked).sum::<usize>(),
    );
    let error = result.expect_err("D2 exposes the next refusal");
    eprintln!("RT_BRANCHED_SCRUTINEE_UNIT_BODY_D2_ADVANCED {error:?}");
    // AMENDED by `RT-CAPTURE-CONTEXT-FRAME-EMIT` `D2`.
    //
    // This clause pinned the BoundaryCarrier refusal as the state this witness
    // advanced *to*. That was correct while the recursive position had no
    // declared callable to resolve to. `RT-CAPTURE-CONTEXT-FRAME-EMIT` `D2`
    // constructs the generated context's frame at the creation site and
    // supplies it at the retarget, so the position now resolves and the call is
    // emitted — the carried-residual guard is no longer reached at all.
    //
    // The pin's PROPERTY is unchanged and is the reason it is amended rather
    // than deleted: this witness advances past route 1, walks both arms, and
    // then refuses. What moved is only which refusal is the terminal one, and
    // it moved because a stop was CLEARED. Asserting the absence keeps the
    // "advanced past" claim as an absence, which is what it always meant.
    let reason = format!("{error:?}");
    assert!(
        !reason.contains("a carried recursive hypothesis is an eliminated value, not a callable"),
        "the carried-residual guard must no longer be this witness's stop — the \
         recursive position resolves to a declared callable now: {error:?}"
    );
    // TRANSITION SENTINEL. The terminal state is now a host-effect seat
    // refusal, in a different subsystem and under a different law: `D2` supplies
    // the callee's frame and says nothing about the phase of the call's RESULT,
    // which crosses a function boundary and carries only the word. Designed to
    // go red when THAT boundary is addressed; the deliverable then is to
    // re-measure the terminal state, not to widen this clause.
    // REPOINTED AGAIN by `RT-EXACTINT-CARRIED-OBSERVE`. Every repoint has moved
    // this pin STRICTLY DOWNSTREAM — ConstructorTag, ResourceRelease/
    // ResourceScalar, ExactIntU64, now the FsReadAt Arg(2) reply-path gate — so
    // the absence above holds each time, and more strongly. The direction is the
    // safety argument; a repoint UPSTREAM would quietly gut it.
    assert!(
        reason.contains(
            "seat Argument(2) of FsReadAt needs ResourceScalar, which this release can \
             observe only in a specialized template"
        ),
        "the expected terminal state after `RT-RESOURCE-RELEASE-CARRIED-OBSERVE` is the \
         `FsReadAt` Arg(2) buffer reply-path gate — a different reader, the next node's — which \
         is what keeps the absence above non-vacuous: \
         {error:?}"
    );
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
    assert!(!rows[0].route1, "the old route-1 observation remains satisfied");
    assert!(
        !rows[0].match_branch_entered,
        "the pre-recorder suppression is a positive control for branch-entry recording"
    );
}
