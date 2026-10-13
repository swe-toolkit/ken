//! C1 premises depending on C2 must follow their dependencies in a match.
//! Promise class: durable invariant. Spec 34 §3.2, §3.5; Architect D0.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::KernelError;

// The 1,982,464-byte measured prelude peak has more than fourfold headroom.
const ELABORATION_STACK_BYTES: usize = 8 * 1024 * 1024;

fn on_elaboration_stack(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(ELABORATION_STACK_BYTES)
        .spawn(f)
        .expect("spawn stated-stack elaboration")
        .join()
        .expect("stated-stack elaboration panicked");
}

const DUAL_FIN: &str = "data DualFin : Nat -> Type where { DualFZ : (n : Nat) -> DualFin (Suc n); DualFS : (n : Nat) -> DualFin n -> DualFin (Suc n) }";
const DUAL_ENV: &str = "data DualEnv (a : Type) : Nat -> Type where { DualNil : DualEnv a Zero; DualCons : (n : Nat) -> a -> DualEnv a n -> DualEnv a (Suc n) }";
const FIN_TO_NAT: &str = "fn dual_fin_to_nat (n : Nat) (i : DualFin n) : Nat = match i { DualFZ m ↦ Zero; DualFS m j ↦ Suc (dual_fin_to_nat m j) }";
const LOOKUP: &str = "fn dual_lookup (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : a = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ x }; DualFS m j ↦ match xs { DualCons k x rest ↦ dual_lookup a m rest j } }";
const L: &str = "dual_lookup a (Suc m) xs (DualFZ m)";
const H: &str = "(h : Equal a ({L}) y)";
const PHI: &str = "(phi : Equal a ({L}) y → Prop)";
const GOAL: &str = "Equal a ({L}) y";

fn env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    for fixture in [DUAL_FIN, DUAL_ENV, FIN_TO_NAT, LOOKUP] {
        env.elaborate_decl(fixture)
            .unwrap_or_else(|error| panic!("fixture {fixture}: {error:?}"));
    }
    env
}

fn row(name: &str, binders: &str, goal: &str, leaf: &str) -> String {
    let binders = binders.replace("{L}", L);
    let goal = goal.replace("{L}", L);
    format!("theorem {name} (a : Type) (m : Nat) (xs : DualEnv a (Suc m)) (y : a) {binders} : {goal} = match xs {{ DualCons k x rest ↦ {leaf} }}")
}

fn checks(source: String) {
    on_elaboration_stack(move || {
        let mut env = env();
        let trust = env.env.trusted_base();
        env.elaborate_decl(&source)
            .unwrap_or_else(|error| panic!("must check: {source}: {error:?}"));
        assert_eq!(env.env.trusted_base(), trust, "no added trusted identity");
    });
}

fn refuses(source: String, kernel_refusal: bool) {
    on_elaboration_stack(move || {
        let mut env = env();
        let trust = env.env.trusted_base();
        let error = env.elaborate_decl(&source).expect_err("must refuse");
        if kernel_refusal {
            assert!(
                matches!(
                    error,
                    ElabError::KernelRejected {
                        error: KernelError::TypeMismatch { .. },
                        ..
                    }
                ),
                "unchanged kernel refusal: {error:?}"
            );
        } else {
            assert!(
                matches!(
                    error,
                    ElabError::TypeMismatch { .. }
                        | ElabError::KernelRejected {
                            error: KernelError::TypeMismatch { .. },
                            ..
                        }
                ),
                "typed refusal, never Internal: {error:?}"
            );
        }
        assert_eq!(env.env.trusted_base(), trust, "no added trusted identity");
    });
}

// MEASURED: each named proof checks. CLAIMED: C2-dependent C1 entries follow
// companions in the telescope. THE GAP: AC-3 independently restores the old
// partition, and S1 alone distinguishes fixpoint from one-pass placement.
#[test]
fn p2_c1_after_c2_checks() {
    checks(row("p2", &format!("{H} {PHI} (r : phi h)"), GOAL, "h"));
}

#[test]
fn p4_reordered_c2_checks() {
    checks(row("p4", &format!("{PHI} {H} (r : phi h)"), GOAL, "h"));
}

#[test]
fn p6_refl_checks() {
    checks(row(
        "p6",
        &format!("{H} {PHI} (r : phi h)"),
        "Equal a ({L}) ({L})",
        "Refl",
    ));
}

#[test]
fn s1_transitive_c1_after_c2_checks() {
    checks(row(
        "s1",
        &format!("{H} {PHI} (r : phi h) (psi : phi h → Prop) (s : psi r)"),
        GOAL,
        "h",
    ));
}

#[test]
fn s2_mixed_c1_and_c2_checks() {
    checks(row(
        "s2",
        &format!("(pe : DualEnv a (Suc m) → Prop) (q : pe xs) {H} (rr : Equal a ({{L}}) y → pe xs → Prop) (r : rr h q)"),
        GOAL,
        "h",
    ));
}

// MEASURED: a different goal cannot be proved by the real hypothesis.
// CLAIMED: reordered premises never fabricate proof of a false endpoint.
// THE GAP: both negatives share the positive convoy but do not enumerate
// every possible false goal; the kernel still checks every generated proof.
#[test]
fn p2_false_endpoint_refuses_typed() {
    refuses(
        row(
            "p2f",
            &format!("(z : a) {H} {PHI} (r : phi h)"),
            "Equal a ({L}) z",
            "h",
        ),
        false,
    );
}

#[test]
fn s5_transitive_false_endpoint_refuses_typed() {
    refuses(
        row(
            "s5",
            &format!("(z : a) {H} {PHI} (r : phi h) (psi : phi h → Prop) (s : psi r)"),
            "Equal a ({L}) z",
            "h",
        ),
        false,
    );
}

#[test]
fn s4_independent_c1_after_dependent_c1_checks() {
    checks(row(
        "s4",
        &format!("{H} {PHI} (r : phi h) (w : a)"),
        GOAL,
        "h",
    ));
}

#[test]
fn h5_hypothesis_alone_checks() {
    checks(row("h5", H, GOAL, "h"));
}

#[test]
fn p2_without_r_checks() {
    checks(row("p2nr", &format!("{H} {PHI}"), GOAL, "h"));
}

#[test]
fn p2_without_match_checks() {
    checks(format!("theorem p2nm (a : Type) (m : Nat) (xs : DualEnv a (Suc m)) (y : a) (h : Equal a ({L}) y) (phi : Equal a ({L}) y → Prop) (r : phi h) : Equal a ({L}) y = h"));
}

#[test]
fn p7_independent_inner_c1_checks() {
    checks(row(
        "p7",
        &format!("(pe : DualEnv a (Suc m) → Prop) {H} (q : pe xs)"),
        GOAL,
        "h",
    ));
}

#[test]
fn p5_other_proposition_retains_kernel_refusal() {
    refuses(
        row("p5", &format!("{H} {PHI} (r : phi h)"), "phi h", "r"),
        true,
    );
}
