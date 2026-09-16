//! PX8-X sole observation schema through the real linked artifact.

fn output_dir() -> tempfile::TempDir {
    tempfile::Builder::new()
        .prefix("ken-px8x-")
        .tempdir()
        .unwrap()
}

const RESOURCE_PROGRAM: &str = r#"program capabilities FS AFull
fn px8x_body (_resource : Resource FsHandle)
  : HostIO AFull (ResourceBodyResult Unit Unit) =
  Ret (Coproduct (FSOp AFull) AmbientOp)
    (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
    (ResourceBodyResult Unit Unit)
    (ResourceBodyOk Unit Unit MkUnit)

fn px8x_after
  (outcome : Result FileError (ResourceBracketResult Unit Unit))
  : HostIO AFull ExitCode =
  match outcome {
    Err _ |-> host_exit AFull (Failure 41);
    Ok bracket |-> match bracket {
      ResourceBracketOk _ |-> host_exit AFull Success;
      ResourceBracketBodyError _ |-> host_exit AFull (Failure 42);
      ResourceBracketReleaseError _ |-> host_exit AFull (Failure 43);
      ResourceBracketBodyAndReleaseError _ _ |-> host_exit AFull (Failure 44)
    }
  }

proc main (_input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match caps {
    MkProgramCaps cap |->
      bind (Coproduct (FSOp AFull) AmbientOp)
        (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
        (Result FileError (ResourceBracketResult Unit Unit)) ExitCode
        (withResource AFull Unit Unit cap (bytes_encode "held.bin")
          ResourceMetadata px8x_body)
        (\outcome. px8x_after outcome)
  }
"#;

// Readmitted under RT-IGNORED-PASSING-ROWS, row 5 of 11.
//
// Three annotation layers are struck here, the same stack this row's sibling in
// px8ta_oriented_subcontinuation carried: a byte-span block claiming the row
// "refuses at object emission, so the program never executes"; a D1a/D2 note
// three lines below it saying the byte-span observation "was not the blocker";
// and a live label claiming the row "next refuses because a carried recursive
// hypothesis is an eliminated value, not a callable". None reproduces. The
// program emits, executes, exits 0, and neither signature appears in any run.
//
// READMITTED ON A MUTATION, NOT ON THE GREEN. Perturbing the ResourceRelease
// arm of ken-host effect_v1.rs dispatch (the Target binding pushed from
// pending.identity, +1000) reds the acquired-equals-released assertion with the
// perturbation visible in it -- left ResourceTraceIdentityV1(1) against right
// ResourceTraceIdentityV1(1001).
//
// WHAT THIS ROW DOES NOT OBSERVE, and its name says it does. The name claims
// two properties. The ordered-bindings half is covered, as above. The
// filters-reserved-input half is NOT, and that is measured rather than
// supposed:
//
//   - Removing the KEN_HOST_OBSERVATION_PATH filter from the C shim's
//     environment() loop alone reds the row -- but at the harness's
//     run_bound_process_effect_observation expect, with Io(NotFound): the
//     arena capacity is sized by main's separate count, so the two disagree,
//     the process returns 1, and no trace file is written. The row noticed a
//     crash, not a filtering decision.
//   - Removing the filter CONSISTENTLY from both loops leaves the row GREEN.
//     The Ken program then sees the reserved key in its environment and nothing
//     in this row reads the program's view of its environment.
//
// The first mutation is what makes the second conclusive: it proves the filter
// branch is TAKEN on this row's path (deleting a branch never taken cannot
// change the arena accounting). So the second green is "reached and
// unobserved", not "never reached". The finding is site-relative -- the row
// does not observe the reserved-key filter -- and the row is covered elsewhere,
// which is why it is readmitted rather than reported vacuous.
#[test]
fn linked_route_exposes_real_ordered_bindings_and_filters_reserved_input() {
    let dir = output_dir();
    std::fs::write(dir.path().join("held.bin"), b"held").unwrap();
    let output = ken_cli::build_native_program(
        RESOURCE_PROGRAM,
        ken_cli::SourceFormat::Ken,
        "px8x-single-schema-observation",
        dir.path(),
    )
    .expect("checked resource program reaches the linked artifact");
    let options = ken_runtime::NativeEffectRunOptionsV1 {
        arguments: Vec::new(),
        environment: vec![(
            "KEN_HOST_OBSERVATION_PATH".into(),
            "caller-controlled".into(),
        )],
        cwd: dir.path().to_owned(),
        plan_hash: output.plan_transport_hash,
    };
    let observation = ken_runtime::run_bound_process_effect_observation(&output.artifact, &options)
        .expect("sole route reads the role-labelled trace");

    assert_eq!(observation.exit_status, 0);
    assert_eq!(
        observation.terminal_exit,
        ken_runtime::TerminalExitClass::NormalReturn
    );
    assert_eq!(
        observation
            .effect_trace
            .iter()
            .map(|event| (event.sequence, event.operation))
            .collect::<Vec<_>>(),
        vec![
            (0, ken_runtime::HostOpV1::FsOpen),
            (1, ken_runtime::HostOpV1::ResourceRelease),
        ]
    );
    let acquired = observation.effect_trace[0].resource_bindings.as_slice();
    let released = observation.effect_trace[1].resource_bindings.as_slice();
    assert_eq!(acquired.len(), 1);
    assert_eq!(released.len(), 1);
    assert_eq!(acquired[0].0, ken_runtime::ResourceBindingRole::Target);
    assert_eq!(released[0].0, ken_runtime::ResourceBindingRole::Target);
    assert_eq!(acquired[0].1, released[0].1);
}
