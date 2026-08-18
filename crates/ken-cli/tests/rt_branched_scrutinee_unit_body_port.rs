//! `RT-BRANCHED-SCRUTINEE-UNIT-BODY-PORT` D1 witness.
//!
//! MEASURED: the observer records resolver entry, plain-Match branch entry, successful
//! plain-Match arm-body lookup, and the direct non-`Construct` route-1 return
//! while this checked Ken source lowers.
//! CLAIMED: D2 enters the carried child's owning plain `Match`, walks its first
//! arm, then advances past route 1.

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
    assert_eq!(
        route1[0].match_arms_walked, 1,
        "D4 must walk arm 0 after entering the plain Match: {route1:?}"
    );
    eprintln!(
        "RT_BRANCHED_SCRUTINEE_UNIT_BODY_ROUTE1 entered={} route1={} match_arms_walked={}",
        route1.len(),
        route1.iter().filter(|row| row.route1).count(),
        route1.iter().map(|row| row.match_arms_walked).sum::<usize>(),
    );
    let error = result.expect_err("D2 exposes the next refusal");
    eprintln!("RT_BRANCHED_SCRUTINEE_UNIT_BODY_D2_ADVANCED {error:?}");
    assert!(
        format!("{error:?}").contains("recursive position is outside its source constructor"),
        "D2 must advance to the constructor-arity refusal: {error:?}"
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
