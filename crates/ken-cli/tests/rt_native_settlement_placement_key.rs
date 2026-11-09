//! RNSPK AC-1c: the production Process gate precedes native object emission.
//! Promise class: transition sentinel. B2 retires these refusal expectations
//! when its native Process gate opens, then promotes the same F sources to the
//! boundary-IH trap expectation. These tests do not claim to reach that trap.
//! MEASURED: each checked F source receives the typed Process-lane refusal,
//! names the seven unavailable operations, and emits no native artifact.
//! CLAIMED: B2, not an accidental planner failure, currently blocks production
//! native execution of all three RNSPK F sources.
//! THE GAP: the diagnostic C-28 overlay is required to measure the trap; these
//! production rows cannot observe it while the Process gate remains closed.

#![cfg(all(target_os = "linux", target_env = "gnu"))]

use ken_elaborator::compiler_driver::NativeProgramBuildError;

const ARGV: &str = include_str!("fixtures/px10_argv_env_pipe.ken");
const LIVE: &str = include_str!("fixtures/px10_live_child.ken");
const NULL_STDOUT: &str = include_str!("fixtures/rnspk_v_argv_stdout_null.ken");

fn assert_production_process_refusal(name: &str, raw: &str) {
    let root = tempfile::tempdir().expect("isolated native output root");
    let source = raw
        .replace(
            "PX10_CHILD_PROBE_PATH",
            env!("CARGO_BIN_EXE_px10-child-probe"),
        )
        .replace(
            "PX10_READY_FILE",
            &root.path().join("ready").display().to_string(),
        );
    assert!(source.contains("Process"), "{name}: missing Process grant");
    assert!(
        !source.contains("PX10_CHILD_PROBE_PATH"),
        "{name}: unresolved probe"
    );
    assert!(
        !source.contains("PX10_READY_FILE"),
        "{name}: unresolved ready path"
    );
    let error = ken_cli::build_native_program(
        &source,
        ken_cli::SourceFormat::Ken,
        name,
        root.path(),
        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
    )
    .expect_err("F must not emit an artifact before B2 opens Process");
    let NativeProgramBuildError::Unavailable(lane) = error else {
        panic!("{name}: expected the typed Process gate, got {error:?}");
    };
    assert_eq!(lane.lane, "host_effect_lowering_unavailable", "{name}");
    assert!(
        lane.reason.contains("declared Process capability"),
        "{name}: {lane:?}"
    );
    assert!(
        lane.reason.contains("RepresentedUnavailable"),
        "{name}: {lane:?}"
    );
    for op in [
        "ResourceStreamRead",
        "ResourceStreamWrite",
        "ProcessPlanOpen",
        "ProcessPlanPush",
        "ProcessSpawn",
        "ProcessTakePipe",
        "ProcessWait",
    ] {
        assert!(lane.reason.contains(op), "{name}: missing {op} in {lane:?}");
    }
    assert_eq!(
        std::fs::read_dir(root.path()).expect("output root").count(),
        0,
        "{name}: the gate cannot leave a partial native artifact"
    );
}

#[test]
fn f_argv_env_pipe_refuses_at_production_process_gate() {
    assert_production_process_refusal("rnspk_f_argv", ARGV);
}

#[test]
fn f_v_argv_stdout_null_refuses_at_production_process_gate() {
    assert_production_process_refusal("rnspk_f_v_null", NULL_STDOUT);
}

#[test]
fn f_live_child_refuses_at_production_process_gate() {
    assert_production_process_refusal("rnspk_f_live", LIVE);
}
