//! PX10 interpreter-first child resource smoke. Promise: durable invariant.
//! MEASURED: declared Process reaches the shared host dispatcher in the
//! interpreter, completes a child bracket and waits for normal exit.
//! CLAIMED: a public plan can execute under the declared Process grant.
//! THE GAP: this smoke uses null stdio; AC-1 pins exact pipe bytes separately.

#![cfg(all(target_os = "linux", target_env = "gnu"))]

const SMOKE: &str = include_str!("fixtures/px10_child_wait_smoke.ken");

#[test]
fn declared_process_smoke_executes_and_reaps_in_interpreter() {
    let root = tempfile::tempdir().expect("isolated interpreter root");
    let mut host = ken_interp::PosixHost::new_at(root.path());
    let observed = ken_cli::run_program_effect_observation(
        SMOKE,
        ken_cli::SourceFormat::Ken,
        &[],
        &[],
        root.path().as_os_str().as_encoded_bytes(),
        &mut host,
    )
    .expect("checked Process program executes in the interpreter");
    assert_eq!(observed.exit_status, 0, "{:?}", observed.effect_trace);
    assert_eq!(observed.terminal_error, None);
    assert_eq!(
        observed.effect_trace.iter().map(|event| event.operation).collect::<Vec<_>>(),
        [
            ken_host::HostOpV1::ProcessPlanOpen,
            ken_host::HostOpV1::ProcessPlanPush,
            ken_host::HostOpV1::ProcessPlanPush,
            ken_host::HostOpV1::ProcessPlanPush,
            ken_host::HostOpV1::ProcessPlanPush,
            ken_host::HostOpV1::ProcessSpawn,
            ken_host::HostOpV1::ProcessWait,
            ken_host::HostOpV1::ResourceRelease,
        ],
    );
    assert!(matches!(
        observed.effect_trace[6].outcome,
        ken_host::CanonicalOutcomeV1::Success(ken_host::CanonicalReplyV1::ChildExit(
            ken_host::ChildExitV1::Exited(0)
        ))
    ));
}

/// Promise: durable invariant. MEASURED: a native build of the same checked
/// Process source refuses with an identified unavailable operation before an
/// artifact exists. CLAIMED: native Process never silently falls through to
/// the unfinished bracket planner. THE GAP: a scratch build with the successor
/// return-fold repair must yield this same refusal, not a changed planner text.
#[test]
fn native_process_op_is_unavailable_before_object_emission() {
    use ken_elaborator::compiler_driver::NativeProgramBuildError;

    let root = tempfile::tempdir().expect("isolated native output root");
    let error = ken_cli::build_native_program(
        SMOKE,
        ken_cli::SourceFormat::Ken,
        "px10_spawn_plan_wait_smoke",
        root.path(),
        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
    )
    .expect_err("Process must not produce a partial native artifact");
    let NativeProgramBuildError::Unavailable(lane) = error else {
        panic!("expected typed host-op availability refusal, found: {error:?}");
    };
    assert_eq!(lane.lane, "host_effect_lowering_unavailable");
    assert!(lane.reason.contains("ProcessPlanOpen"), "{lane:?}");
    assert!(lane.reason.contains("RepresentedUnavailable"), "{lane:?}");
    assert_eq!(std::fs::read_dir(root.path()).expect("output root").count(), 0);
}
