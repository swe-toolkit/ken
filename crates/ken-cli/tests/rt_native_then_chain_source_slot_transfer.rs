//! Native/interpreter parity for a three-deep user-level FS then-chain.
#![cfg(target_os = "linux")]

use std::time::{Duration, Instant};

const SOURCE: &str = include_str!("fixtures/rt_native_then_chain_source_slot_transfer.ken");
const CHILD_SCENARIO: &str = "KEN_RT_THEN_CHAIN_CHILD_SCENARIO";

fn run_scenario(present: bool) {
    use std::os::unix::ffi::OsStrExt as _;
    let root = tempfile::tempdir().expect("isolated filesystem root");
    std::fs::write(root.path().join("held.bin"), b"held").expect("second action's file");
    if present {
        std::fs::write(root.path().join("switch.bin"), b"switch").expect("first action's file");
    }
    let roots =
        [std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")];
    let mut profile = ken_runtime::boundary_resource_profile::starter_smoke_profile();
    // The present path executes three full metadata operations and measured
    // 85 persistent nodes at return. The starter's 64 stopped at request 65.
    profile.persistent.nodes = 96;
    let compiled = ken_cli::build_native_program_with_roots(
        SOURCE,
        ken_cli::SourceFormat::Ken,
        &roots,
        "rt_native_then_chain_source_slot_transfer",
        root.path().join("native"),
        profile,
    )
    .expect("the checked Strict program builds as a native artifact");
    let native = ken_runtime::run_bound_process_effect_observation(
        &compiled.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: root.path().to_owned(),
            plan_hash: compiled.plan_transport_hash,
        },
    )
    .expect("native program runs to an observed result");
    let mut host = ken_interp::PosixHost::new_at(root.path());
    let interpreted = ken_cli::run_program_effect_observation(
        SOURCE,
        ken_cli::SourceFormat::Ken,
        &roots,
        &[],
        &[],
        root.path().as_os_str().as_bytes(),
        &mut host,
    )
    .expect("identical Strict source runs through the interpreter");
    let pairs = |obs: &ken_runtime::EffectObservation| {
        obs.effect_trace
            .iter()
            .map(|event| (event.request.clone(), event.outcome.clone()))
            .collect::<Vec<_>>()
    };
    let operations = |obs: &ken_runtime::EffectObservation| {
        obs.effect_trace
            .iter()
            .map(|event| event.operation)
            .collect::<Vec<_>>()
    };
    let expected = if present {
        vec![
            ken_runtime::HostOpV1::FsMetadata,
            ken_runtime::HostOpV1::FsMetadata,
            ken_runtime::HostOpV1::FsMetadata,
            ken_runtime::HostOpV1::ConsoleWrite,
        ]
    } else {
        vec![
            ken_runtime::HostOpV1::FsMetadata,
            ken_runtime::HostOpV1::ConsoleWrite,
            ken_runtime::HostOpV1::ConsoleWrite,
        ]
    };
    assert_eq!(
        operations(&interpreted),
        expected,
        "reference reaches the intended arms"
    );
    assert_eq!(
        operations(&native),
        expected,
        "native reaches the same arms; terminal error: {:?}",
        native.terminal_error
    );
    println!(
        "THEN_CHAIN_SCENARIO present={present} interpreted={:?} native={:?}",
        pairs(&interpreted),
        pairs(&native)
    );
    assert_eq!(
        pairs(&native),
        pairs(&interpreted),
        "request/outcome pairs must agree"
    );
    assert_eq!(native.exit_status, interpreted.exit_status);
    assert_eq!(native.exit_status, if present { 0 } else { 42 });
    assert_eq!(native.stdout, interpreted.stdout);
    assert_eq!(
        native.stdout,
        if present {
            b"result-ok".as_slice()
        } else {
            b"failresult-error".as_slice()
        }
    );
}

/// Child isolation ensures a native fault or infinite loop cannot hang the
/// parent parity gate or prevent the second scenario from being diagnosed.
#[test]
fn then_chain_parity_child() {
    let Some(scenario) = std::env::var_os(CHILD_SCENARIO) else {
        return;
    };
    let present = match scenario.to_string_lossy().as_ref() {
        "present" => true,
        "absent" => false,
        other => panic!("unknown child scenario {other}"),
    };
    std::thread::Builder::new()
        .name(format!("then-chain-{scenario:?}"))
        .stack_size(16 * 1024 * 1024)
        .spawn(move || run_scenario(present))
        .expect("parity worker")
        .join()
        .expect("parity worker completed");
}

/// Promise class: durable invariant. MEASURED: the exact Strict source's
/// native and interpreter effect pairs, exit code and console bytes on both
/// switch states. CLAIMED: creation-site slot construction preserves the
/// user then-chain's observable sequencing. THE GAP: this finite fixture
/// does not cover all source constructors or all owner splits; the two
/// generic guards remain fail-closed for those shapes.
#[test]
fn then_chain_switch_states_match_with_bounded_wait() {
    for scenario in ["present", "absent"] {
        let mut child = std::process::Command::new(std::env::current_exe().expect("test binary"))
            .args(["--exact", "then_chain_parity_child", "--nocapture"])
            .env(CHILD_SCENARIO, scenario)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("isolated parity process");
        let start = Instant::now();
        let status = loop {
            if let Some(status) = child.try_wait().expect("poll child") {
                break status;
            }
            if start.elapsed() >= Duration::from_secs(480) {
                child.kill().expect("kill timed-out parity process");
                let output = child.wait_with_output().expect("collect timed-out child");
                panic!(
                    "{scenario} timed out: {} {}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );
            }
            std::thread::sleep(Duration::from_millis(50));
        };
        let output = child
            .wait_with_output()
            .expect("collect isolated parity process");
        assert!(
            status.success(),
            "{scenario} child failed: stdout={} stderr={}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains(&format!(
                "THEN_CHAIN_SCENARIO present={}",
                scenario == "present"
            )),
            "{scenario} child did not run its parity row: {stdout}"
        );
        println!("{stdout}");
    }
}
