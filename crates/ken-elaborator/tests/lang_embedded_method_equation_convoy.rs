//! Embedded index-equation convoy for a coherent-frame nested indexed match.
//!
//! Promise class: durable invariant. True structurally recursive equalities
//! elaborate without trust; a false successor proof reports the Refl endpoint
//! mismatch in the arm rather than an incidental embedded-index rejection.
//! See spec/30-surface/34-data-match.md §§3.2, 3.5 and
//! spec/10-kernel/16-observational.md §2.2.

use ken_elaborator::{ElabEnv, ElabError};

// The adjacent coherent-frame acceptance test measured 3 MiB as the minimum
// for this unoptimized prelude workload, with peak 1,982,464 bytes. Eight MiB
// provides more than four times the measured peak and is local to these rows.
const ELABORATION_STACK_BYTES: usize = 8 * 1024 * 1024;

fn on_elaboration_stack(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(ELABORATION_STACK_BYTES)
        .spawn(f)
        .expect("spawn stated-stack elaboration")
        .join()
        .expect("stated-stack elaboration panicked");
}

fn env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    for declaration in [
        "data DualFin : Nat -> Type where { DualFZ : (n : Nat) -> DualFin (Suc n); DualFS : (n : Nat) -> DualFin n -> DualFin (Suc n) }",
        "data DualEnv (a : Type) : Nat -> Type where { DualNil : DualEnv a Zero; DualCons : (n : Nat) -> a -> DualEnv a n -> DualEnv a (Suc n) }",
        "fn dual_fin_to_nat (n : Nat) (i : DualFin n) : Nat = match i { DualFZ m ↦ Zero; DualFS m j ↦ Suc (dual_fin_to_nat m j) }",
        "fn dual_lookup (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : a = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ x }; DualFS m j ↦ match xs { DualCons k x rest ↦ dual_lookup a m rest j } }",
        "fn dual_lookup_peer (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : a = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ x }; DualFS m j ↦ match xs { DualCons k x rest ↦ dual_lookup_peer a m rest j } }",
        "fn dual_nth (a : Type) (n : Nat) (xs : DualEnv a n) (i : Nat) : Option a = match i { Zero ↦ match xs { DualNil ↦ None a; DualCons m x rest ↦ Some a x }; Suc j ↦ match xs { DualNil ↦ None a; DualCons m x rest ↦ dual_nth a m rest j } }",
        "data DualTagged (a : Type) : Type where { DualTag : Nat -> a -> DualTagged a }",
    ] {
        env.elaborate_decl(declaration).expect("coherent-frame fixture");
    }
    env
}

const T1: &str = "theorem t1 (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : Equal (DualTagged a) (DualTag a (dual_fin_to_nat n i) (dual_lookup a n xs i)) (DualTag a (dual_fin_to_nat n i) (dual_lookup a n xs i)) = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ Refl }; DualFS m j ↦ match xs { DualCons k x rest ↦ Refl } }";
const T2: &str = "theorem t2 (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : Equal (DualTagged a) (DualTag a (dual_fin_to_nat n i) (dual_lookup a n xs i)) (DualTag a (dual_fin_to_nat n i) (dual_lookup_peer a n xs i)) = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ Refl }; DualFS m j ↦ match xs { DualCons k x rest ↦ t2 a m rest j } }";
const T3: &str = "theorem t3 (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : Equal (DualTagged (Option a)) (DualTag (Option a) (dual_fin_to_nat n i) (dual_nth a n xs (dual_fin_to_nat n i))) (DualTag (Option a) (dual_fin_to_nat n i) (Some a (dual_lookup a n xs i))) = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ Refl }; DualFS m j ↦ match xs { DualCons k x rest ↦ t3 a m rest j } }";
const NAMED_FALSE: &str = "theorem dual_recursive_false (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : Equal (DualTagged a) (DualTag a (dual_fin_to_nat n i) (dual_lookup a n xs i)) (DualTag a Zero (dual_lookup a n xs i)) = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ Refl }; DualFS m j ↦ match xs { DualCons k x rest ↦ Refl } }";
const F4: &str = "theorem f4 (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : Equal (DualTagged a) (DualTag a (dual_fin_to_nat n i) (dual_lookup a n xs i)) (DualTag a Zero (dual_lookup_peer a n xs i)) = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ Refl }; DualFS m j ↦ match xs { DualCons k x rest ↦ Refl } }";

fn check_true(source: &str) {
    let mut env = env();
    let trust = env.env.trusted_base();
    env.elaborate_decl(source)
        .expect("true embedded equality must elaborate");
    assert_eq!(env.env.trusted_base(), trust, "no added trust");
}

fn check_false(source: &str) {
    let mut env = env();
    let trust = env.env.trusted_base();
    let error = env
        .elaborate_decl(source)
        .expect_err("false recursive step must reject");
    let ElabError::TypeMismatch { span, reason } = error else {
        panic!("must reject the false Refl, not an earlier index frame: {error:?}");
    };
    assert_eq!(
        reason,
        "Refl: the two sides of the goal are not convertible"
    );
    let dual_fs = source.rfind("DualFS m j ↦ match xs").expect("DualFS arm");
    assert!(
        span.start >= dual_fs && span.end <= source.len(),
        "false step must lie inside DualFS: {span:?}"
    );
    assert_eq!(env.env.trusted_base(), trust, "no added trust");
}

#[test]
fn true_identical_nested_arms() {
    on_elaboration_stack(|| check_true(T1));
}

#[test]
fn true_recursive_peer_comparison() {
    on_elaboration_stack(|| check_true(T2));
}

#[test]
fn true_recursive_option_comparison() {
    on_elaboration_stack(|| check_true(T3));
}

#[test]
fn false_successor_is_rejected_at_refl() {
    on_elaboration_stack(|| {
        check_true(T1); // The twin takes the same route but has equal endpoints.
        check_false(NAMED_FALSE);
        check_false(F4);
    });
}
