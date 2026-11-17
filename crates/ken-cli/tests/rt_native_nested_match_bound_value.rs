//! An outer computational-match PipeRead binding used inside a nested match.
//! The same checked source must release the exact pipe before waiting on both
//! the interpreter and the native Process path.

#![cfg(all(target_os = "linux", target_env = "gnu"))]

const NESTED_READ: &str = include_str!("fixtures/rt_native_nested_bound_pipe_read.ken");

fn roots() -> [std::path::PathBuf; 1] {
    [std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")]
}

/// Promise class: durable invariant. MEASURED: identical reference/native
/// terminal observations and complete ordered canonical effects, including
/// exact PipeRead settlement before ProcessWait. CLAIMED: transporting an
/// outer-match-bound resource through a nested match preserves its authority.
/// THE GAP: this one Process source does not cover every checked-IH transport.
#[test]
fn nested_bound_pipe_read_matches_interpreter_and_settles_before_wait() {
    use std::time::{Duration, Instant};
    let root = tempfile::tempdir().expect("isolated Q root");
    let ready = root.path().join("q-child-pid");
    let ready_text = ready.to_str().expect("UTF-8 temp directory");
    assert!(
        !ready_text.contains(['"', '$', ' ', '\n']),
        "safe child pid path"
    );
    // Match the landed F live-child control's RootExecution grant and
    // 80-node invocation profile; the fixture changes only nested use.
    let source = NESTED_READ
        .replacen(
            "program capabilities FS AFull, Process\n",
            "program capabilities FS AFull, Process, RootExecution Allow\n",
            1,
        )
        .replace("PX10_READY_FILE", ready_text);
    assert!(source.contains("RootExecution Allow"));
    let cwd = root.path().to_path_buf();
    let source_for_reference = source.clone();
    // The early-close Process worker measured 2324 KiB of resident stack;
    // 16 MiB leaves 14060 KiB headroom to that baseline. This fixture's
    // deeper peak is not claimed from that measurement.
    let reference = std::thread::Builder::new()
        .name("rt-nested-bound-reference".to_owned())
        .stack_size(16 * 1024 * 1024)
        .spawn(move || {
            let mut host = ken_interp::PosixHost::new_at(&cwd);
            ken_cli::run_program_effect_observation(
                &source_for_reference,
                ken_cli::SourceFormat::Ken,
                &roots(),
                &[],
                &[],
                cwd.as_os_str().as_encoded_bytes(),
                &mut host,
            )
            .map_err(|error| format!("{error:?}"))
        })
        .expect("Q reference worker");
    let deadline = Instant::now() + Duration::from_secs(45);
    let pid = loop {
        if let Ok(text) = std::fs::read_to_string(&ready) {
            break text.parse::<libc::pid_t>().expect("child pid");
        }
        assert!(
            Instant::now() < deadline,
            "Q child failed to signal readiness"
        );
        std::thread::sleep(Duration::from_millis(10));
    };
    let result = unsafe { libc::kill(pid, libc::SIGTERM) };
    assert_eq!(result, 0, "terminate ready child before its wait");
    let observation = reference
        .join()
        .expect("Q reference worker completes")
        .expect("Q checked reference executes");
    assert_eq!(observation.exit_status, 0, "Q reference success");
    assert_eq!(
        observation.terminal_error, None,
        "Q reference no terminal fault"
    );
    std::fs::remove_file(&ready).expect("clear reference child's ready signal");
    let ready_for_native = ready.clone();
    let (built_tx, built_rx) = std::sync::mpsc::channel();
    let native = std::thread::Builder::new()
        .name("rt-nested-bound-native".to_owned())
        .stack_size(16 * 1024 * 1024)
        .spawn(move || {
            let mut profile = ken_runtime::boundary_resource_profile::starter_smoke_profile();
            profile.invocation.nodes = 80;
            // The repaired route's measured native peak requested 259 words
            // against the starter's 256. Provision a stated 512-word runtime
            // baseline rather than misclassifying capacity as a parity fault.
            profile.invocation.words = 512;
            let built = ken_cli::build_native_program_with_roots(
                &source,
                ken_cli::SourceFormat::Ken,
                &roots(),
                "rt_native_nested_bound_pipe_read",
                root.path().join("native"),
                profile,
            )
            .expect("nested bound resource native build");
            built_tx.send(()).expect("signal native build complete");
            ken_runtime::run_bound_process_effect_observation(
                &built.artifact,
                &ken_runtime::NativeEffectRunOptionsV1 {
                    arguments: Vec::new(),
                    environment: Vec::new(),
                    cwd: root.path().to_owned(),
                    plan_hash: built.plan_transport_hash,
                },
            )
            .expect("Q native execution")
        })
        .expect("Q native worker");
    built_rx
        .recv_timeout(Duration::from_secs(900))
        .expect("Q native build completes");
    let deadline = Instant::now() + Duration::from_secs(45);
    let native_pid = loop {
        if let Ok(text) = std::fs::read_to_string(&ready_for_native) {
            break text.parse::<libc::pid_t>().expect("native child pid");
        }
        if native.is_finished() {
            let ended = native.join().expect("Q native worker early exit");
            panic!("Q native returned before child readiness: {ended:?}");
        }
        assert!(Instant::now() < deadline, "Q native child failed readiness");
        std::thread::sleep(Duration::from_millis(10));
    };
    assert_eq!(unsafe { libc::kill(native_pid, libc::SIGTERM) }, 0);
    let native = native.join().expect("native worker completes");
    assert_eq!(native.exit_status, observation.exit_status);
    assert_eq!(native.terminal_exit, observation.terminal_exit);
    assert_eq!(native.terminal_error, observation.terminal_error);
    assert_eq!(native.effect_trace, observation.effect_trace);
    use ken_host::{
        CanonicalOutcomeV1 as Outcome, CanonicalReplyV1 as Reply, HostOpV1 as Op,
        ResourceBindingRole as Role, ResourceKindV1 as Kind,
    };
    let acquired = native
        .effect_trace
        .iter()
        .find(|event| {
            event.operation == Op::ProcessTakePipe
                && matches!(
                    &event.outcome,
                    Outcome::Success(Reply::ResourceAcquired {
                        resource_kind: Kind::PipeRead,
                        ..
                    })
                )
        })
        .expect("Q takes a PipeRead");
    let id = match &acquired.outcome {
        Outcome::Success(Reply::ResourceAcquired { identity, .. }) => *identity,
        _ => unreachable!("Q acquisition filtered by its typed result"),
    };
    assert!(acquired.resource_bindings.contains(&(Role::Buffer, id)));
    let release = native
        .effect_trace
        .iter()
        .position(|event| {
            event.operation == Op::ResourceRelease
                && event.resource_bindings.contains(&(Role::Target, id))
        })
        .expect("Q releases exact taken PipeRead");
    let wait = native
        .effect_trace
        .iter()
        .position(|event| event.operation == Op::ProcessWait)
        .expect("Q waits for its child");
    assert!(release < wait, "PipeRead release precedes ProcessWait");
}
