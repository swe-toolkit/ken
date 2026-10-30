//! PX10 child resource smoke. Promise class: durable invariant.
//! MEASURED: the same checked `withChild`/wait source reaches the shared host
//! dispatcher through both the interpreter and linked native artifact and
//! returns a completed bracket without a leaked child.
//! CLAIMED: a declared Process grant admits typed spawn and cached wait on
//! both engines. THE GAP: this smoke uses null stdio and one normal-exit path;
//! separate AC-1/AC-2 rows must still pin pipe bytes, signal exit and live
//! descriptor and signal preconditions before the candidate is released.

#![cfg(all(target_os = "linux", target_env = "gnu"))]

const SMOKE: &str = include_str!("fixtures/px10_child_wait_smoke.ken");

#[test]
fn declared_process_smoke_executes_and_reaps_on_both_engines() {
    let build = tempfile::tempdir().expect("isolated build root");
    let native_root = tempfile::tempdir().expect("isolated native root");
    let interp_root = tempfile::tempdir().expect("isolated interpreter root");
    let artifact = ken_cli::build_native_program(
        SMOKE,
        ken_cli::SourceFormat::Ken,
        "px10_spawn_plan_wait_smoke",
        build.path(),
        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
    )
    .expect("PX10 checked source builds a native process artifact");
    let native = ken_runtime::run_bound_process_effect_observation(
        &artifact.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: native_root.path().to_owned(),
            plan_hash: artifact.plan_transport_hash,
        },
    )
    .expect("linked PX10 artifact executes");
    let mut host = ken_interp::PosixHost::new_at(interp_root.path());
    let interp = ken_cli::run_program_effect_observation(
        SMOKE,
        ken_cli::SourceFormat::Ken,
        &[],
        &[],
        interp_root.path().as_os_str().as_encoded_bytes(),
        &mut host,
    )
    .expect("reference PX10 program executes");
    for (name, lane) in [("native", &native), ("interpreter", &interp)] {
        assert_eq!(lane.exit_status, 0, "{name}: {:?}", lane.effect_trace);
        assert_eq!(lane.terminal_error, None, "{name}");
        assert_eq!(
            lane.effect_trace.iter().map(|event| event.operation).collect::<Vec<_>>(),
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
            "{name}: op identity sequence",
        );
        assert!(matches!(
            lane.effect_trace[6].outcome,
            ken_host::CanonicalOutcomeV1::Success(ken_host::CanonicalReplyV1::ChildExit(
                ken_host::ChildExitV1::Exited(0)
            ))
        ));
    }
    let events = |lane: &ken_runtime::EffectObservation| {
        lane.effect_trace.iter()
            .map(|event| (event.operation, event.request.clone(), event.outcome.clone()))
            .collect::<Vec<_>>()
    };
    assert_eq!(events(&interp), events(&native));
}
