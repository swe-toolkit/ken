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
// WHAT THIS ROW DOES NOT OBSERVE. It was named
// ..._and_filters_reserved_input and it arranged a caller-supplied
// KEN_HOST_OBSERVATION_PATH of "caller-controlled". Both are removed, because
// the reserved-input half was never observable here and the arrangement that
// suggested it was was inert:
//
//   - Removing the KEN_HOST_OBSERVATION_PATH filter from the C shim's
//     environment() loop alone reds the row -- but at the harness's
//     run_bound_process_effect_observation expect, with Io(NotFound): the
//     arena capacity is sized by main's separate count of the same key, so the
//     two disagree, the process returns 1, and no trace file is written. The
//     row noticed a crash, not a filtering decision.
//   - Removing the filter CONSISTENTLY from both loops leaves the row GREEN.
//     Nothing here reads the Ken program's view of its environment.
//
// The first mutation is what makes the second conclusive: deleting a branch
// that is never taken cannot change the arena accounting, so the red proves the
// filter branch is TAKEN on this row's path. The second green is therefore
// "reached and unobserved", not "never reached".
//
// AND THE ARRANGEMENT WAS INERT, not merely unasserted. In
// crates/ken-runtime/src/object_linker_packaging.rs,
// run_bound_process_effect_observation_with_stdin,
// .envs(options.environment) is followed by .env("KEN_HOST_OBSERVATION_PATH",
// &trace_path) -- the launcher overwrites the caller's value before the child
// starts, so no process on this path ever saw "caller-controlled". The setup
// could not have been observed even by an assertion that tried. Deleting it is
// what removes the defect; renaming alone would have left it in the body.
#[test]
fn linked_route_exposes_real_ordered_role_labelled_bindings() {
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
        environment: Vec::new(),
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
