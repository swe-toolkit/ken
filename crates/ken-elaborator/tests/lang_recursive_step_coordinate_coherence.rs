//! Recursive indexed goals and their moved fields use one transport.
//! Promise class: durable invariant. Spec 34 §3.5 and 39 §2.1;
//! Architect RSC D0 evt_5hzbjtwpgm827.

use ken_elaborator::{ElabEnv, ElabError};

// The measured 1,982,464-byte prelude peak has more than fourfold headroom.
const ELABORATION_STACK_BYTES: usize = 8 * 1024 * 1024;

fn on_elaboration_stack(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(ELABORATION_STACK_BYTES)
        .spawn(f)
        .expect("spawn stated-stack elaboration")
        .join()
        .expect("stated-stack elaboration panicked");
}

const FIXTURES: &[&str] = &[
    "data DualFin : Nat -> Type where { DualFZ : (n : Nat) -> DualFin (Suc n); DualFS : (n : Nat) -> DualFin n -> DualFin (Suc n) }",
    "data DualEnv (a : Type) : Nat -> Type where { DualNil : DualEnv a Zero; DualCons : (n : Nat) -> a -> DualEnv a n -> DualEnv a (Suc n) }",
    "fn dual_fin_to_nat (n : Nat) (i : DualFin n) : Nat = match i { DualFZ m ↦ Zero; DualFS m j ↦ Suc (dual_fin_to_nat m j) }",
    "fn dual_lookup (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : a = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ x }; DualFS m j ↦ match xs { DualCons k x rest ↦ dual_lookup a m rest j } }",
    "fn dual_lookup_peer (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : a = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ x }; DualFS m j ↦ match xs { DualCons k x rest ↦ dual_lookup_peer a m rest j } }",
    "fn dual_nth (a : Type) (n : Nat) (xs : DualEnv a n) (i : Nat) : Option a = match i { Zero ↦ match xs { DualNil ↦ None a; DualCons m x rest ↦ Some a x }; Suc j ↦ match xs { DualNil ↦ None a; DualCons m x rest ↦ dual_nth a m rest j } }",
    "data DualTagged (a : Type) : Type where { DualTag : Nat -> a -> DualTagged a }",
    // Checked Ken proof, not an axiom; `cong` is not in the prelude.
    "theorem cong (ty : Type) (ty2 : Type) (x : ty) (y : ty) (f : ty → ty2) (p : Eq ty x y) : Eq ty2 (f x) (f y) = J (λy' _. Eq ty2 (f x) (f y')) Refl p",
];

fn fresh_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    let trust = env.env.trusted_base();
    for source in FIXTURES {
        env.elaborate_decl(source)
            .unwrap_or_else(|error| panic!("fixture {source}: {error:?}"));
    }
    assert_eq!(env.env.trusted_base(), trust, "fixtures add no trust");
    env
}

fn checks(source: &str) {
    let mut env = fresh_env();
    let trust = env.env.trusted_base();
    env.elaborate_decl(source)
        .unwrap_or_else(|error| panic!("true proof must check: {source}: {error:?}"));
    assert_eq!(env.env.trusted_base(), trust, "no new trusted identity");
}

// A positive route for the recursive step: `cong` is checked and in scope.
// MEASURED: CTL with `Proved` at DualFZ and `cong` at DualFS checks without
// added trust. CLAIMED: the empty large-convoy method falls through to an
// ordinary dependent-branch check instead of surfacing Internal. THE GAP:
// a method that does build a goal transport takes a different path, guarded
// by the T2/T3/dual_option controls and H1/K2 acceptance tests.
#[test]
fn step_reachable_cong_control_checks_without_new_trust() {
    on_elaboration_stack(|| {
        checks("theorem ctl (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : Equal Nat (dual_fin_to_nat n i) (dual_fin_to_nat n i) = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ Proved }; DualFS m j ↦ match xs { DualCons k x rest ↦ cong Nat Nat (dual_fin_to_nat m j) (dual_fin_to_nat m j) (λv. Suc v) (ctl a m rest j) } }");
    });
}

#[test]
fn false_cong_endpoint_refuses_typed_without_new_trust() {
    on_elaboration_stack(|| {
        let mut env = fresh_env();
        let trust = env.env.trusted_base();
        let false_goal = "theorem ctlf (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : Equal Nat (dual_fin_to_nat n i) Zero = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ Proved }; DualFS m j ↦ match xs { DualCons k x rest ↦ cong Nat Nat (dual_fin_to_nat m j) Zero (λv. Suc v) (ctlf a m rest j) } }";
        let error = env.elaborate_decl(false_goal).expect_err("false endpoint");
        assert!(
            matches!(
                error,
                ElabError::KernelRejected { .. } | ElabError::TypeMismatch { .. }
            ),
            "false endpoint must refuse, never Internal: {error:?}"
        );
        assert_eq!(env.env.trusted_base(), trust, "no new trusted identity");
    });
}

// MEASURED: the three already-supported recursive proof shapes still check
// independently with no additional trust. CLAIMED: the coordinate repair
// does not regress direct recursive IH or transport-bearing proofs. THE GAP:
// these three shapes are not exhaustive over every dependent motive.
#[test]
fn existing_t2_t3_and_dual_option_still_check_without_new_trust() {
    on_elaboration_stack(|| {
        checks("theorem t2 (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : Equal (DualTagged a) (DualTag a Zero (dual_lookup a n xs i)) (DualTag a Zero (dual_lookup_peer a n xs i)) = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ Refl }; DualFS m j ↦ match xs { DualCons k x rest ↦ t2 a m rest j } }");
        checks("theorem t3 (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : Equal (DualTagged (Option a)) (DualTag (Option a) (dual_fin_to_nat n i) (dual_nth a n xs (dual_fin_to_nat n i))) (DualTag (Option a) (dual_fin_to_nat n i) (Some a (dual_lookup a n xs i))) = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ Refl }; DualFS m j ↦ match xs { DualCons k x rest ↦ t3 a m rest j } }");
        checks("theorem dual_option (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : Equal (Option a) (dual_nth a n xs (dual_fin_to_nat n i)) (Some a (dual_lookup a n xs i)) = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ Refl }; DualFS m j ↦ match xs { DualCons k x rest ↦ dual_option a m rest j } }");
    });
}
