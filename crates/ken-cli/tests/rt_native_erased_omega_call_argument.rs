//! Native/ref interpreter parity for recursive calls with checked-erased Ω
//! operands. All five fixtures return distinguishable process exits via an
//! injective small-Nat classifier; the two recursive proof-carrying variants
//! must not silently drop their recursive call (4 becomes 2 if they do).

#![cfg(target_os = "linux")]

const PLAIN: &str = include_str!("fixtures/rt_native_erased_omega_call_plain.ken");
const REQUIRES: &str = include_str!("fixtures/rt_native_erased_omega_call_requires.ken");
const REQUIRES_DROPPED: &str =
    include_str!("fixtures/rt_native_erased_omega_call_requires_dropped.ken");
const REFINED: &str = include_str!("fixtures/rt_native_erased_omega_call_refined.ken");
const REFINED_NONRECURSIVE: &str =
    include_str!("fixtures/rt_native_erased_omega_call_refnonrec.ken");
const EXPLICIT_OMEGA: &str = include_str!("fixtures/rt_native_erased_omega_call_explicit.ken");

fn assert_native_and_reference_exit(source: &str, name: &str, expected_exit: i32) {
    let mut host = ken_interp::CaptureHost::new(Vec::new());
    let interpreted = ken_cli::run_program_effect_observation(
        source,
        ken_cli::SourceFormat::Ken,
        &[b"ken".to_vec()],
        &[],
        b"/",
        &mut host,
    )
    .unwrap_or_else(|error| panic!("{name}: interpreter must run: {error:?}"));
    assert_eq!(
        interpreted.exit_status, expected_exit,
        "{name}: interpreter must return the selected Nat's exit"
    );
    assert_eq!(
        interpreted.terminal_error, None,
        "{name}: interpreter must have no abnormal terminal error"
    );

    let root = tempfile::Builder::new()
        .prefix("ken-rt-native-erased-omega-")
        .tempdir()
        .expect("native output directory");
    let output = ken_cli::build_native_program(
        source,
        ken_cli::SourceFormat::Ken,
        name,
        root.path(),
        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
    )
    .unwrap_or_else(|error| panic!("{name}: native build must succeed: {error:?}"));
    let native = ken_runtime::run_bound_process_effect_observation(
        &output.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: root.path().to_owned(),
            plan_hash: output.plan_transport_hash,
        },
    )
    .unwrap_or_else(|error| panic!("{name}: native artifact must run: {error:?}"));
    assert_eq!(
        native.terminal_error, None,
        "{name}: native must have no abnormal terminal error"
    );
    assert_eq!(
        native.exit_status, expected_exit,
        "{name}: native result must retain the recursive contribution"
    );
    assert_eq!(
        native, interpreted,
        "{name}: executor observations diverged"
    );
}

/// Promise class: durable invariant. MEASURED: the ordinary recursive Nat
/// program exits 4 natively and through the interpreter. CLAIMED: filtering
/// checked-erased operands does not perturb a non-proof recursive call. THE
/// GAP: the classifier observes the small Nat through its exit code, not an
/// exported Nat value; it distinguishes a dropped call (2) from the full 4.
#[test]
fn plain_recursive_nat_preserves_native_reference_exit_four() {
    assert_native_and_reference_exit(PLAIN, "rt-erased-omega-plain", 4);
}

/// Promise class: durable invariant. MEASURED: a `requires Top` recursive
/// callee exits 4 through both executors. CLAIMED: its checked-erased proof
/// operand has no native runtime slot, and the recursive body is still run.
/// THE GAP: the exit classifier observes only the computed small Nat; dropping
/// the recursive call makes it 2 and must fail this exact-exit assertion.
#[test]
fn requires_proof_recursive_call_exits_four_without_a_runtime_proof() {
    assert_native_and_reference_exit(REQUIRES, "rt-erased-omega-requires", 4);
}

/// Promise class: durable invariant. MEASURED: replacing only the recursive
/// call in `requires` with the constant Zero yields exit 2 on both executors,
/// not the original 4. CLAIMED: the original exact-exit pin distinguishes
/// actually running recursion from a build that silently drops it. THE GAP:
/// this control varies the Ken program, not the native lowerer's behaviour;
/// the QA-owned lowering mutation still supplies independent causal evidence.
#[test]
fn requires_proof_dropped_recursion_control_exits_two() {
    assert_native_and_reference_exit(REQUIRES_DROPPED, "rt-erased-omega-drop-recursion", 2);
}

/// Promise class: durable invariant. MEASURED: a refined-parameter recursive
/// worker exits 4 through both executors. CLAIMED: checked Ω erasure preserves
/// worker recursion. THE GAP: this pins the selected refined worker's call
/// route, not every refinement shape; dropping its recursion yields exit 2.
#[test]
fn refined_parameter_recursive_worker_exits_four() {
    assert_native_and_reference_exit(REFINED, "rt-erased-omega-refined", 4);
}

/// Promise class: durable invariant. MEASURED: the refined but nonrecursive
/// control exits 3 through both executors. CLAIMED: the fix does not perturb
/// an unrelated refined call. THE GAP: the row does not reach a recursive
/// erased-Ω call, so it is a control rather than an erasure-pin substitute.
#[test]
fn refined_parameter_nonrecursive_control_exits_three() {
    assert_native_and_reference_exit(REFINED_NONRECURSIVE, "rt-erased-omega-refnonrec", 3);
}

/// Promise class: durable invariant. MEASURED: a recursive call spelling out
/// the erased `(p : Top)` parameter exits 4 through both executors. CLAIMED:
/// call-site filtering handles a directly explicit Ω binder, not just `requires`
/// and refinement desugarings. THE GAP: this is a distinct operand origin but
/// still reaches the same measured inner lowering arm; arms (a)/(b) are not
/// claimed covered by these probes.
#[test]
fn explicit_omega_recursive_call_exits_four() {
    assert_native_and_reference_exit(EXPLICIT_OMEGA, "rt-erased-omega-explicit", 4);
}
