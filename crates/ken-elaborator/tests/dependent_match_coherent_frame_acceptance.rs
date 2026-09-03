//! LANG-DEPELIM-NESTED-COUPLED-INDEX-COHERENT-FRAME acceptance.
//!
//! Promise class: durable invariant. Dependent elimination convoys every
//! correlated component of an embedded indexed eliminator into one motive
//! frame. Valid cross-structure equalities elaborate, while a genuinely false
//! reachable branch remains rejected.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::KernelError;

// This fixture exercises nested dependent eliminators through the unoptimized
// prelude path. On this exact test, the 2 MiB spawned-thread default overflows
// and 3 MiB is the measured minimum viable stack. The elaborator's documented
// unoptimized peak is 1,982,464 bytes; 8 MiB provides more than 4x that peak
// and more than 5 MiB beyond the measured minimum. Builder::stack_size makes
// the provision local and independent of ambient RUST_MIN_STACK.
const ELABORATION_STACK_BYTES: usize = 8 * 1024 * 1024;

fn on_elaboration_stack<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    std::thread::Builder::new()
        .stack_size(ELABORATION_STACK_BYTES)
        .spawn(f)
        .expect("spawn stated-stack elaboration")
        .join()
        .expect("stated-stack elaboration panicked")
}

fn env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base env");
    env.elaborate_decl(
        "data DualFin : Nat -> Type where { \
           DualFZ : (n : Nat) -> DualFin (Suc n); \
           DualFS : (n : Nat) -> DualFin n -> DualFin (Suc n) }",
    )
    .expect("DualFin");
    env.elaborate_decl(
        "data DualEnv (a : Type) : Nat -> Type where { \
           DualNil : DualEnv a Zero; \
           DualCons : (n : Nat) -> a -> DualEnv a n -> DualEnv a (Suc n) }",
    )
    .expect("DualEnv");
    env.elaborate_decl(
        "fn dual_fin_to_nat (n : Nat) (i : DualFin n) : Nat = \
         match i { DualFZ m ↦ Zero; DualFS m j ↦ Suc (dual_fin_to_nat m j) }",
    )
    .expect("dual_fin_to_nat");
    env.elaborate_decl(
        "fn dual_lookup (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : a = \
         match i { \
           DualFZ m ↦ match xs { DualCons k x rest ↦ x }; \
           DualFS m j ↦ match xs { DualCons k x rest ↦ dual_lookup a m rest j } }",
    )
    .expect("dual_lookup");
    env.elaborate_decl(
        "fn dual_lookup_peer (a : Type) (n : Nat) (xs : DualEnv a n) \
           (i : DualFin n) : a = \
         match i { \
           DualFZ m ↦ match xs { DualCons k x rest ↦ x }; \
           DualFS m j ↦ match xs { \
             DualCons k x rest ↦ dual_lookup_peer a m rest j } }",
    )
    .expect("dual_lookup_peer");
    env.elaborate_decl(
        "fn dual_nth (a : Type) (n : Nat) (xs : DualEnv a n) (i : Nat) : Option a = \
         match i { \
           Zero ↦ match xs { \
             DualNil ↦ None a; DualCons m x rest ↦ Some a x }; \
           Suc j ↦ match xs { \
             DualNil ↦ None a; DualCons m x rest ↦ dual_nth a m rest j } }",
    )
    .expect("dual_nth");
    env
}

#[test]
fn cross_structure_coupled_nested_elaborates() {
    on_elaboration_stack(|| {
        let mut env = env();
        let trusted_before = env.env.trusted_base();
        env.elaborate_decl(
            "theorem dual_option (a : Type) (n : Nat) (xs : DualEnv a n) \
           (i : DualFin n) : \
           Equal (Option a) (dual_nth a n xs (dual_fin_to_nat n i)) \
             (Some a (dual_lookup a n xs i)) = \
         match i { \
           DualFZ m ↦ match xs { DualCons k x rest ↦ Refl }; \
           DualFS m j ↦ match xs { DualCons k x rest ↦ \
             dual_option a m rest j } }",
        )
        .expect("the full embedded-eliminator convoy must elaborate");
        assert_eq!(
            env.env.trusted_base(),
            trusted_before,
            "the coherent-frame proof must add no trusted declaration"
        );
    });
}

#[test]
fn direct_same_context_control_stays_elaborable() {
    on_elaboration_stack(|| {
        let mut env = env();
        env.elaborate_decl(
            "theorem dual_direct (a : Type) (n : Nat) (xs : DualEnv a n) \
           (i : DualFin n) : Equal a (dual_lookup a n xs i) (dual_lookup a n xs i) = \
         match i { DualFZ m ↦ Refl; DualFS m j ↦ Refl }",
        )
        .expect("the direct control must remain green");
    });
}

#[test]
fn nontrivial_direct_fin_index_elaborates() {
    on_elaboration_stack(|| {
        let mut env = env();
        env.elaborate_decl(
            "theorem dual_direct_peer \
           (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : \
           Equal (Option a) \
             (Some a (dual_lookup_peer a n xs i)) \
             (Some a (dual_lookup a n xs i)) = \
         match i { \
           DualFZ m ↦ match xs { DualCons k x rest ↦ Refl }; \
           DualFS m j ↦ match xs { DualCons k x rest ↦ \
             dual_direct_peer a m rest j } }",
        )
        .expect("direct-index cross-structure results must share one frame");
    });
}

#[test]
fn single_elimination_cross_structure_elaborates() {
    on_elaboration_stack(|| {
        let mut env = env();
        env.elaborate_decl(
            "theorem dual_option_zero (a : Type) (m : Nat) \
           (xs : DualEnv a (Suc m)) : \
           Equal (Option a) (dual_nth a (Suc m) xs Zero) \
             (Some a (dual_lookup a (Suc m) xs (DualFZ m))) = \
         match xs { DualCons k x rest ↦ Refl }",
        )
        .expect("one indexed elimination must convoy both dependent results");
    });
}

#[test]
fn trivial_coupled_equality_stays_elaborable() {
    on_elaboration_stack(|| {
        let mut env = env();
        env.elaborate_decl(
            "theorem dual_option_trivial \
           (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : \
           Equal (Option a) \
             (dual_nth a n xs (dual_fin_to_nat n i)) \
             (dual_nth a n xs (dual_fin_to_nat n i)) = \
         match i { \
           DualFZ m ↦ match xs { DualCons k x rest ↦ Refl }; \
           DualFS m j ↦ match xs { DualCons k x rest ↦ Refl } }",
        )
        .expect("the same-side control must remain green");
    });
}

#[test]
fn genuinely_false_reachable_branch_stays_rejected() {
    on_elaboration_stack(|| {
        let mut env = env();
        let error = env
            .elaborate_decl(
                "theorem dual_false (a : Type) (n : Nat) (xs : DualEnv a n) \
               (i : DualFin n) : \
               Equal (Option a) (Some a (dual_lookup a n xs i)) (None a) = \
             match i { \
               DualFZ m ↦ match xs { DualCons k x rest ↦ Refl }; \
               DualFS m j ↦ match xs { DualCons k x rest ↦ Refl } }",
            )
            .expect_err("a reachable Some = None branch must not be admitted");
        assert!(
            matches!(
                error,
                ElabError::KernelRejected {
                    error: KernelError::TypeMismatch { .. },
                    ..
                }
            ),
            "the underivable control must fail at kernel type checking, found {error:?}"
        );
    });
}
