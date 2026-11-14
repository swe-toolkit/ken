//! PX10 B2 C3: a real self-recursive performed-call scrutinee must refuse
//! before native emission; unrelated recursive calls remain executable.

#![cfg(all(target_os = "linux", target_env = "gnu"))]

const P2: &str = include_str!("fixtures/px10_recursive_performed_p2.ken");
const P5: &str = include_str!("fixtures/px10_recursive_performed_p5.ken");
const P7: &str = include_str!("fixtures/px10_recursive_performed_p7.ken");
const P8: &str = include_str!("fixtures/px10_recursive_performed_p8.ken");
const P10: &str = include_str!("fixtures/px10_recursive_performed_p10.ken");
const VLET2: &str = include_str!("fixtures/px10_recursive_performed_vlet2.ken");
const VMATCH2: &str = include_str!("fixtures/px10_recursive_performed_vmatch2.ken");
const VMATCH_MIXED: &str = include_str!("fixtures/px10_recursive_performed_vmatch_mixed.ken");

fn with_native_grant(source: &str) -> String {
    let granted = source.replacen(
        "program capabilities FS APartial\n",
        "program capabilities FS APartial, RootExecution Allow\n",
        1,
    );
    assert_ne!(granted, source, "the fixture capability header moved");
    granted
}

fn roots() -> [std::path::PathBuf; 1] {
    [std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")]
}

/// Promise class: durable fail-closed refusal. MEASURED: the real recursive
/// p10/p5/p7 checked sources refuse on the exact erasure cause before native
/// emission. CLAIMED: a performed self-call hidden by a checked invocation
/// wrapper cannot enter the native planner as a bare computational scrutinee.
/// THE GAP: these three sources exercise the recursive wrapper; other checked
/// metadata wrappers are classified exhaustively by the production predicate.
fn assert_self_recursive_scrutinee_refuses(name: &'static str, raw: &'static str) {
    std::thread::Builder::new()
        .name(format!("px10-b2-c3-refuse-{name}"))
        // Scratch 16 MiB workers measured 1560 KiB resident stack on these
        // refusal fixtures; stated 4 MiB leaves 2536 KiB headroom.
        .stack_size(4 * 1024 * 1024)
        .spawn(move || {
            let root = tempfile::tempdir().expect("isolated recursive refusal root");
            let error = ken_cli::build_native_program_with_roots(
                &with_native_grant(raw),
                ken_cli::SourceFormat::Ken,
                &roots(),
                name,
                root.path().join("native"),
                ken_runtime::boundary_resource_profile::starter_smoke_profile(),
            )
            .expect_err("a performed recursive self-call cannot be lowered as a scrutinee");
            assert!(
                matches!(
                    &error,
                    ken_elaborator::compiler_driver::NativeProgramBuildError::Erasure(
                        ken_elaborator::erasure::ErasureError::ExpressionLowering {
                            lane: "performed_call_scrutinee_not_direct",
                            ..
                        }
                    )
                ),
                "{name}: wrong build refusal: {error:?}"
            );
        })
        .expect("stated recursive refusal worker")
        .join()
        .expect("recursive refusal worker completed");
}

#[test]
fn p10_self_recursive_bind_scrutinee_refuses_before_native_emission() {
    assert_self_recursive_scrutinee_refuses("p10", P10);
}

#[test]
fn p5_self_recursive_bind_scrutinee_refuses_before_native_emission() {
    assert_self_recursive_scrutinee_refuses("p5", P5);
}

#[test]
fn p7_self_recursive_bind_scrutinee_refuses_before_native_emission() {
    assert_self_recursive_scrutinee_refuses("p7", P7);
}

/// Promise class: durable fail-closed refusal. MEASURED: all three decreasing
/// variants execute on the checked interpreter and refuse before native
/// emission. V-MATCH2 reaches the Match result-position arm: disabling only
/// that arm lets it build; disabling only ComputationalMatch does not. The
/// mixed Match has one pure result and one recursive performed call; replacing
/// Match's `any` with `all` lets it build, so the refusal depends on ANY case.
/// V-LET2 refuses as a direct Call after its Let is removed before erasure;
/// it is not a witness of the Let arm. CLAIMED: a performed call in one
/// surviving Match case result cannot pass the native erasure guard even if
/// the other case is pure. THE GAP: If, ComputationalMatch, and Let are
/// classified by the exhaustive predicate but unwitnessed by these rows.
/// Scratch V-LETIN also refused with Let false; no Let reachability is claimed.
fn assert_result_position_variant_refuses(name: &'static str, raw: &'static str) {
    std::thread::Builder::new()
        .name(format!("px10-b2-c3-result-{name}"))
        // Scratch 16 MiB workers on the preceding p10 controls used under
        // 2 MiB resident; 16 MiB provisions the same recursive baseline.
        .stack_size(16 * 1024 * 1024)
        .spawn(move || {
            let root = tempfile::tempdir().expect("isolated result-position root");
            let source = with_native_grant(raw);
            let mut host = ken_interp::PosixHost::new_at(root.path());
            let reference = ken_cli::run_program_effect_observation(
                &source,
                ken_cli::SourceFormat::Ken,
                &roots(),
                &[],
                &[],
                root.path().as_os_str().as_encoded_bytes(),
                &mut host,
            )
            .unwrap_or_else(|error| panic!("{name}: interpreter must elaborate: {error:?}"));
            assert_eq!(reference.exit_status, 0, "{name}: {reference:?}");
            assert!(
                reference.stdout.ends_with(b"end\n"),
                "{name}: {reference:?}"
            );
            assert!(!reference.effect_trace.is_empty(), "{name}: effect ran");
            let mut profile = ken_runtime::boundary_resource_profile::starter_smoke_profile();
            profile.invocation.nodes = 80; // B2's measured-only native policy.
            let error = ken_cli::build_native_program_with_roots(
                &source,
                ken_cli::SourceFormat::Ken,
                &roots(),
                name,
                root.path().join("native"),
                profile,
            )
            .expect_err("result-position performed call must refuse before emission");
            assert!(
                matches!(
                    &error,
                    ken_elaborator::compiler_driver::NativeProgramBuildError::Erasure(
                        ken_elaborator::erasure::ErasureError::ExpressionLowering {
                            lane: "performed_call_scrutinee_not_direct",
                            ..
                        }
                    )
                ),
                "{name}: wrong native build refusal: {error:?}"
            );
        })
        .expect("stated result-position worker")
        .join()
        .expect("result-position worker completed");
}

#[test]
fn vlet2_result_position_performed_call_refuses_before_native_emission() {
    assert_result_position_variant_refuses("vlet2", VLET2);
}

#[test]
fn vmatch2_result_position_performed_call_refuses_before_native_emission() {
    assert_result_position_variant_refuses("vmatch2", VMATCH2);
}

#[test]
fn vmatch_mixed_pure_and_performed_case_refuses_before_native_emission() {
    assert_result_position_variant_refuses("vmatch_mixed", VMATCH_MIXED);
}

/// Promise class: durable differential. MEASURED: the print-only p2 and the
/// different-performing-member p8 run against a fresh checked interpreter
/// and a native artifact with equal terminal result, stdout and complete
/// ordered canonical effect trace. CLAIMED: the C3 guard rejects the missed
/// self-call shape without rejecting other recursion and performed calls.
/// THE GAP: the checked fixture set is bounded; it does not claim every
/// recursive effectful program now has executable native parity.
fn assert_recursive_control_parity(name: &'static str, raw: &'static str) {
    std::thread::Builder::new()
        .name(format!("px10-b2-c3-parity-{name}"))
        // Scratch 16 MiB workers measured 1564 KiB resident stack on p2/p8;
        // stated 4 MiB leaves 2532 KiB headroom, not an ambient-stack claim.
        .stack_size(4 * 1024 * 1024)
        .spawn(move || {
            let root = tempfile::tempdir().expect("isolated recursive parity root");
            let source = with_native_grant(raw);
            let mut host = ken_interp::PosixHost::new_at(root.path());
            let reference = ken_cli::run_program_effect_observation(
                &source,
                ken_cli::SourceFormat::Ken,
                &roots(),
                &[],
                &[],
                root.path().as_os_str().as_encoded_bytes(),
                &mut host,
            )
            .unwrap_or_else(|error| panic!("{name}: interpreter: {error:?}"));
            let built = ken_cli::build_native_program_with_roots(
                &source,
                ken_cli::SourceFormat::Ken,
                &roots(),
                name,
                root.path().join("native"),
                ken_runtime::boundary_resource_profile::starter_smoke_profile(),
            )
            .unwrap_or_else(|error| panic!("{name}: native build: {error:?}"));
            let native = ken_runtime::run_bound_process_effect_observation(
                &built.artifact,
                &ken_runtime::NativeEffectRunOptionsV1 {
                    arguments: Vec::new(),
                    environment: Vec::new(),
                    cwd: root.path().to_owned(),
                    plan_hash: built.plan_transport_hash,
                },
            )
            .unwrap_or_else(|error| panic!("{name}: native run: {error:?}"));
            assert_eq!(reference.exit_status, 0, "{name}: {reference:?}");
            assert_eq!(
                native.exit_status, reference.exit_status,
                "{name}: native exit"
            );
            assert!(
                reference.stdout.ends_with(b"end\n"),
                "{name}: {reference:?}"
            );
            assert_eq!(native.stdout, reference.stdout, "{name}: stdout");
            assert_eq!(native.terminal_exit, reference.terminal_exit);
            assert_eq!(native.terminal_error, reference.terminal_error);
            assert_eq!(
                native.effect_trace, reference.effect_trace,
                "{name}: effects"
            );
            assert!(!native.effect_trace.is_empty(), "{name}: an effect ran");
        })
        .expect("stated recursive parity worker")
        .join()
        .expect("recursive parity worker completed");
}

#[test]
fn p2_print_only_self_recursion_keeps_native_parity() {
    assert_recursive_control_parity("p2", P2);
}

#[test]
fn p8_other_performing_member_keeps_native_parity() {
    assert_recursive_control_parity("p8", P8);
}
