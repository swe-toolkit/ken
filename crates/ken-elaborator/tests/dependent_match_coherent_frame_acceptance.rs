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
    env.elaborate_decl(
        "data DualTagged (a : Type) : Type where { \
           DualTag : Nat -> a -> DualTagged a }",
    )
    .expect("DualTagged");
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
fn recursive_step_only_false_is_rejected_after_true_base_elaborates() {
    on_elaboration_stack(|| {
        let mut env = env();
        env.elaborate_decl(
            "theorem dual_recursive_false_base \
               (a : Type) (m : Nat) (xs : DualEnv a (Suc m)) : \
               Equal (DualTagged a) \
                 (DualTag a \
                   (dual_fin_to_nat (Suc m) (DualFZ m)) \
                   (dual_lookup a (Suc m) xs (DualFZ m))) \
                 (DualTag a Zero \
                   (dual_lookup a (Suc m) xs (DualFZ m))) = Refl",
        )
        .expect("the DualFZ-restricted true base must elaborate");

        let error = env
            .elaborate_decl(
                "theorem dual_recursive_false \
                   (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : \
                   Equal (DualTagged a) \
                     (DualTag a (dual_fin_to_nat n i) (dual_lookup a n xs i)) \
                     (DualTag a Zero (dual_lookup a n xs i)) = \
                 match i { \
                   DualFZ m ↦ match xs { DualCons k x rest ↦ Refl }; \
                   DualFS m j ↦ match xs { DualCons k x rest ↦ Refl } }",
            )
            .expect_err("the recursive-only false branch must not be admitted");
        assert!(
            matches!(
                error,
                ElabError::KernelRejected {
                    error: KernelError::TypeMismatch { .. },
                    ..
                }
            ),
            "the recursive-only false branch must fail at kernel type checking, found {error:?}"
        );
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

fn install_forced_telescope_fixture(env: &mut ElabEnv) {
    env.elaborate_decl(
        "theorem telescope_eq_trans (a : Type) (x : a) (y : a) (z : a) \
           (h1 : Equal a x y) (h2 : Equal a y z) : Equal a x z = \
         J (λz2 _. Equal a x z2) h1 h2",
    )
    .expect("telescope_eq_trans");
    env.elaborate_decl(
        "fn dual_env_relation \
           (a : Type) (n : Nat) (xs : DualEnv a n) : Omega = \
         (i : DualFin n) -> \
           Equal (Option a) \
             (dual_nth a n xs (dual_fin_to_nat n i)) \
             (Some a (dual_lookup a n xs i))",
    )
    .expect("dual_env_relation");
    env.elaborate_decl(
        "theorem dual_env_relation_after_anchor \
           (a : Type) (n : Nat) (xs : DualEnv a n) \
           (anchor : Equal Nat n n) \
           (relation : dual_env_relation a n xs) : \
           dual_env_relation a n xs = relation",
    )
    .expect("dual_env_relation_after_anchor");
}

/// Promise class: durable invariant. A forced index must rebase every coupled
/// return-telescope argument, while an unrelated argument stays unchanged.
#[test]
fn forced_index_telescope_relation_extension_elaborates() {
    on_elaboration_stack(|| {
        let mut env = env();
        install_forced_telescope_fixture(&mut env);
        let trusted_before = env.env.trusted_base();
        env.elaborate_decl(
            "theorem dual_env_relation_cons_index \
               (a : Type) (n : Nat) (anchor : Equal Nat n n) \
               (x : a) (i : DualFin (Suc n)) : \
               (unrelated : Option a) -> \
               (xs : DualEnv a n) -> \
               dual_env_relation a n xs -> \
               Equal (Option a) \
                 (dual_nth a (Suc n) (DualCons a n x xs) \
                   (dual_fin_to_nat (Suc n) i)) \
                 (Some a \
                   (dual_lookup a (Suc n) (DualCons a n x xs) i)) = \
             match i { \
               DualFZ m ↦ λunrelated. λxs. λrelation. Refl; \
               DualFS m j ↦ λunrelated. λxs. λrelation. \
                 telescope_eq_trans (Option a) \
                   (dual_nth a m xs (dual_fin_to_nat m j)) \
                   (dual_nth a m xs (dual_fin_to_nat m j)) \
                   (Some a (dual_lookup a m xs j)) \
                   Refl \
                   (dual_env_relation_after_anchor \
                     a m xs anchor relation j) }"
        )
        .expect("forced-index return telescope must use predecessor views");
        env.elaborate_decl(
            "theorem dual_env_relation_cons \
               (a : Type) (n : Nat) (anchor : Equal Nat n n) \
               (xs : DualEnv a n) (x : a) \
               (relation : dual_env_relation a n xs) : \
               dual_env_relation a (Suc n) (DualCons a n x xs) = \
             λi. dual_env_relation_cons_index \
               a n anchor x i (None a) xs relation",
        )
        .expect("the forced-index helper must retain its public telescope");
        assert_eq!(
            env.env.trusted_base(),
            trusted_before,
            "forced-index telescope rebasing must add no trust"
        );
    });
}

/// Promise class: durable invariant. The same forced-index telescope path has
/// a true `DualFZ` base but cannot consume the predecessor relation as a proof
/// of the false `DualFS` goal.
#[test]
fn forced_index_telescope_recursive_step_only_false_stays_kernel_rejected() {
    on_elaboration_stack(|| {
        let mut env = env();
        install_forced_telescope_fixture(&mut env);
        env.elaborate_decl(
            "fn forced_telescope_expected \
               (a : Type) (n : Nat) (x : a) \
               (i : DualFin (Suc n)) : Option a = \
             match i { DualFZ m ↦ Some a x; DualFS m j ↦ None a }",
        )
        .expect("forced_telescope_expected");
        env.elaborate_decl(
            "theorem forced_telescope_false_base \
               (a : Type) (n : Nat) (x : a) : \
               (xs : DualEnv a n) -> \
               dual_env_relation a n xs -> \
               Equal (Option a) \
                 (dual_nth a (Suc n) (DualCons a n x xs) Zero) \
                 (forced_telescope_expected a n x (DualFZ n)) = \
             λxs. λrelation. Refl",
        )
        .expect("the forced telescope false control's base must be true");
        let error = env
            .elaborate_decl(
                "theorem forced_telescope_recursive_false \
                   (a : Type) (n : Nat) (x : a) \
                   (i : DualFin (Suc n)) : \
                   (xs : DualEnv a n) -> \
                   dual_env_relation a n xs -> \
                   Equal (Option a) \
                     (dual_nth a (Suc n) (DualCons a n x xs) \
                       (dual_fin_to_nat (Suc n) i)) \
                     (forced_telescope_expected a n x i) = \
                 match i { \
                   DualFZ m ↦ λxs. λrelation. Refl; \
                   DualFS m j ↦ λxs. λrelation. relation j }",
            )
            .expect_err("the recursive-only false telescope goal must reject");
        assert!(
            matches!(
                error,
                ElabError::KernelRejected {
                    error: KernelError::TypeMismatch { .. },
                    ..
                }
            ),
            "the recursive-only false telescope branch must fail at kernel type checking, found {error:?}"
        );
    });
}
