//! RNSPK AC-1c: Process execution parity after native promotion.
//! MEASURED: each F fixture executes on both real engines with the same
//! terminal result and complete ordered canonical requests, outcomes and
//! bindings; seven plan pushes precede Buffer1's final settlement. B2
//! promotes ResourceStreamRead and five Process operations; ResourceStreamWrite
//! remains native-unavailable until its checked source home and follow-on.
//! CLAIMED: B1a's native Process route preserves the interpreter's checked
//! effects, bracket release order and result on argv, null and live paths.
//! THE GAP: these three checked inputs do not cover every Process program.
//! The seven B1a option-(iii) residuals remain explicit: P1 handler
//! ownership, carried return-clause dropping, retired-flat M4 crossing,
//! retained-unit graph-target admission, captured-neighbor proof widening,
//! unitless deferred drive, and ordinary execute-then-resume P2 veto.
//! MEASURED: B1a D0 and M-B1A found no candidate population for these guards
//! on their original fixtures. CLAIMED: B2's Process promotion does not
//! weaken those guards. THE GAP: these F rows do not measure their populations
//! on other B2 programs; Steward still owns witness-or-deletion follow-on.

#![cfg(all(target_os = "linux", target_env = "gnu"))]

const ARGV: &str = include_str!("fixtures/px10_argv_env_pipe.ken");
const LIVE: &str = include_str!("fixtures/px10_live_child.ken");
const NULL_STDOUT: &str = include_str!("fixtures/rnspk_v_argv_stdout_null.ken");

fn b2_native_profile() -> ken_runtime::boundary_resource_profile::BoundaryResourceProfileV3 {
    // Explicit deployment policy, not a change to the starter. Scratch
    // observation at the emitted allocator's successful capacity-checked
    // stores measured peak Invocation Nodes: argv 73, live 68, null 57;
    // peak Words: argv 249, live 240, null 225 (all below starter 256).
    // Starter Nodes are 64. Round the maximum peak up to the next multiple
    // of 16: 80 leaves 7 nodes of headroom above argv's observed 73.
    // Every other limit, including Invocation Words, stays at starter.
    let mut profile = ken_runtime::boundary_resource_profile::starter_smoke_profile();
    profile.invocation.nodes = 80;
    profile
}

fn spawn_live_terminator(
    name: &str,
    ready: std::path::PathBuf,
) -> Option<std::thread::JoinHandle<()>> {
    (name == "live").then(|| {
        std::thread::spawn(move || {
            for _ in 0..600 {
                if let Ok(text) = std::fs::read_to_string(&ready) {
                    let pid = text.trim().parse::<u32>().expect("live child PID");
                    std::thread::sleep(std::time::Duration::from_millis(750));
                    let status = std::process::Command::new("kill")
                        .args(["-TERM", &pid.to_string()])
                        .status()
                        .expect("controlled live-child termination");
                    assert!(status.success(), "live child termination failed: {status}");
                    return;
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            panic!("live child never published the ready file");
        })
    })
}

fn assert_f_native_interpreter_parity(name: &'static str, raw: &'static str) {
    // The exact F argv source needed 8 MiB in the previous test harness;
    // a stated 16 MiB thread preserves at least 8 MiB of headroom.
    std::thread::Builder::new()
        .name(format!("rnspk-f-{name}"))
        .stack_size(16 * 1024 * 1024)
        .spawn(move || compare_f_engines(name, raw))
        .expect("F parity worker")
        .join()
        .expect("F parity worker completed");
}

fn compare_f_engines(name: &str, raw: &str) {
    use ken_host::{HostOpV1 as Op, ResourceBindingRole as Role};
    let root = tempfile::Builder::new()
        .prefix("rnspk-f-native-")
        .tempdir()
        .expect("isolated Process root");
    let ready = root.path().join("ready");
    let source = raw
        .replacen(
            "program capabilities FS AFull, Process\n",
            "program capabilities FS AFull, Process, RootExecution Allow\n",
            1,
        )
        .replace(
            "PX10_CHILD_PROBE_PATH",
            env!("CARGO_BIN_EXE_px10-child-probe"),
        )
        .replace("PX10_READY_FILE", &ready.display().to_string());
    assert!(source.contains("RootExecution Allow"), "{name}: grant header moved");
    assert!(!source.contains("PX10_CHILD_PROBE_PATH"));
    assert!(!source.contains("PX10_READY_FILE"));
    let roots =
        [std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")];
    let reference_terminator = spawn_live_terminator(name, ready.clone());
    let mut host = ken_interp::PosixHost::new_at(root.path());
    let reference = ken_cli::run_program_effect_observation(
        &source,
        ken_cli::SourceFormat::Ken,
        &roots,
        &[],
        &[],
        root.path().as_os_str().as_encoded_bytes(),
        &mut host,
    )
    .unwrap_or_else(|error| panic!("{name}: checked interpreter: {error:?}"));
    if let Some(handle) = reference_terminator {
        handle.join().expect("reference child terminator");
        std::fs::remove_file(&ready).expect("remove prior ready file before native run");
    }
    let built = ken_cli::build_native_program_with_roots(
        &source,
        ken_cli::SourceFormat::Ken,
        &roots,
        name,
        root.path().join("native"),
        b2_native_profile(),
    )
    .unwrap_or_else(|error| panic!("{name}: native Process build: {error:?}"));
    let native_terminator = spawn_live_terminator(name, ready);
    let native = ken_runtime::run_bound_process_effect_observation(
        &built.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: root.path().to_owned(),
            plan_hash: built.plan_transport_hash,
        },
    )
    .unwrap_or_else(|error| panic!("{name}: native Process execution: {error:?}"));
    if let Some(handle) = native_terminator {
        handle.join().expect("native child terminator");
    }
    assert_eq!(
        native.exit_status, reference.exit_status,
        "{name}: exit status"
    );
    assert_eq!(
        native.terminal_exit, reference.terminal_exit,
        "{name}: terminal class"
    );
    assert_eq!(
        native.terminal_error, reference.terminal_error,
        "{name}: terminal error"
    );
    assert_eq!(
        native.effect_trace, reference.effect_trace,
        "{name}: ordered canonical effects"
    );
    let trace = &reference.effect_trace;
    assert_eq!(
        trace
            .iter()
            .filter(|event| event.operation == Op::ProcessPlanPush)
            .count(),
        7
    );
    let buffer = trace.first().expect("buffer acquisition starts F's trace");
    assert_eq!(buffer.operation, Op::BufferAllocate);
    let buffer_identity = buffer
        .resource_bindings
        .iter()
        .find(|(role, _)| *role == Role::Target)
        .expect("buffer acquisition binds a target identity")
        .1;
    let releases = trace
        .iter()
        .enumerate()
        .filter(|(_, event)| {
            event.operation == Op::ResourceRelease
                && event
                    .resource_bindings
                    .contains(&(Role::Target, buffer_identity))
        })
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    assert_eq!(
        releases,
        [trace.len() - 1],
        "{name}: Buffer1 must release last"
    );
    let finishing_operation = if name == "null" {
        Op::ProcessTakePipe
    } else {
        Op::ProcessWait
    };
    assert!(
        trace
            .iter()
            .any(|event| event.operation == finishing_operation),
        "{name}: checked child finish operation is missing"
    );
}

#[test]
fn f_argv_env_pipe_matches_fresh_interpreter() {
    assert_f_native_interpreter_parity("argv", ARGV);
}

#[test]
fn f_v_argv_stdout_null_matches_fresh_interpreter() {
    assert_f_native_interpreter_parity("null", NULL_STDOUT);
}

#[test]
fn f_live_child_matches_fresh_interpreter() {
    assert_f_native_interpreter_parity("live", LIVE);
}

/// Promise class: durable capacity discriminator. MEASURED: the same checked
/// argv program that passes the F parity row builds and executes under the
/// exact starter profile, then reports typed Invocation Nodes exhaustion at
/// the starter's 64-node ceiling. CLAIMED: B2's single Nodes raise is needed
/// for native parity, not an incidental widening. THE GAP: this one row does
/// not measure other limits; the three F rows run with them all at starter.
#[test]
fn r_nodes_starter_argv_refuses_on_invocation_nodes() {
    // The identical argv F source needed a stated 16 MiB worker (measured
    // 8 MiB on the earlier fixture), not the ambient harness stack.
    std::thread::Builder::new()
        .name("rnspk-r-nodes".to_owned())
        .stack_size(16 * 1024 * 1024)
        .spawn(|| {
            let root = tempfile::tempdir().expect("isolated native capacity root");
            let source = ARGV
                .replacen(
                    "program capabilities FS AFull, Process\n",
                    "program capabilities FS AFull, Process, RootExecution Allow\n",
                    1,
                )
                .replace(
                    "PX10_CHILD_PROBE_PATH",
                    env!("CARGO_BIN_EXE_px10-child-probe"),
                );
            assert!(source.contains("RootExecution Allow"));
            assert!(!source.contains("PX10_CHILD_PROBE_PATH"));
            let roots = [std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../catalog/packages")];
            let built = ken_cli::build_native_program_with_roots(
                &source,
                ken_cli::SourceFormat::Ken,
                &roots,
                "rnspk_r_nodes_argv",
                root.path().join("native"),
                ken_runtime::boundary_resource_profile::starter_smoke_profile(),
            )
            .expect("R-NODES native artifact builds under starter policy");
            let observed = ken_runtime::run_bound_process_effect_observation(
                &built.artifact,
                &ken_runtime::NativeEffectRunOptionsV1 {
                    arguments: Vec::new(),
                    environment: Vec::new(),
                    cwd: root.path().to_owned(),
                    plan_hash: built.plan_transport_hash,
                },
            )
            .expect("R-NODES native artifact runs and returns a typed observation");
            assert_ne!(observed.exit_status, 0, "starter did not refuse: {observed:?}");
            assert!(observed.effect_trace.iter().any(|event|
                event.operation == ken_host::HostOpV1::ProcessSpawn));
            match observed.terminal_error {
                Some(ken_runtime::TerminalErrorV1::CapacityExhausted(fault)) => {
                    assert_eq!(fault.scope, ken_runtime::CapacityScopeV1::Invocation);
                    assert_eq!(fault.resource, ken_runtime::CapacityResourceV1::Nodes);
                    assert_eq!(fault.limit, 64);
                    assert!(fault.requested > fault.limit, "{fault:?}");
                    eprintln!("B2 R-NODES requested={} limit={}", fault.requested, fault.limit);
                }
                other => panic!("R-NODES expected typed Invocation/Nodes fault, got {other:?}"),
            }
        })
        .expect("R-NODES stated worker")
        .join()
        .expect("R-NODES worker completed");
}
