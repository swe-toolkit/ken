//! Finite certificates and deterministic-automaton reachability.
//!
//! Contract: spec/50-stdlib/61-formal-languages.md §2.
//! Promise class: durable semantic invariants for the public laws and decisions.
//! Concrete constructors are test fixtures; shortest-witness order is not fixed.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::ElabEnv;
use ken_interp::eval::{eval, EvalStore, EvalVal};
use ken_kernel::Decl;

const FINITE: &str = "Data.Finite.Finite";
const REACHABILITY: &str = "Algorithm.FormalLanguages.Reachability";
const VECTOR: &str = "Data.Vector.Vector";
const DERIVED: &str = "Data.Collections.Derived";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn checked_bool(env: &ElabEnv, name: &str) -> bool {
    let id = env.globals[name];
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(id) else {
        panic!("{name} must have a checked, transparent body");
    };
    match eval(&[], body, &env.env, &mut EvalStore::new()) {
        EvalVal::Ctor { id, args, .. } if id == env.numeric_env.bool_true_id && args.is_empty() => {
            true
        }
        EvalVal::Ctor { id, args, .. }
            if id == env.numeric_env.bool_false_id && args.is_empty() =>
        {
            false
        }
        other => panic!("{name} must compute to a Boolean constructor, got {other:?}"),
    }
}

/// MEASURED: isolated roots-loading checks four transparent public laws,
/// fresh generic Ken clients apply them at independently stated §2.3 types,
/// and neither package changes the full provider-closure trust inventory.
/// CLAIMED: the checked decision laws have their specified general types and
/// add no local trust. THE GAP: generic clients check the stated contract, not
/// a particular proof structure; execution is tested separately.
#[test]
fn finite_and_reachability_load_with_four_checked_laws_and_no_local_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        VECTOR,
        DERIVED,
        "Algorithm.FormalLanguages.Dfa",
        "Core.Logic.Or",
        "Core.Logic.Transport",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Order",
        "Data.Collections.List",
        "Data.Sums.Combinators",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} must load: {error:?}"));
    }
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let finite_owned = env
        .elaborate_module_from_roots(&[root()], FINITE)
        .expect("Finite must load from the checked Vector and Derived providers");
    assert_eq!(
        env.env.trusted_base().into_iter().collect::<BTreeSet<_>>(),
        before,
        "Finite adds no trusted assumption"
    );
    let reach_owned = env
        .elaborate_module_from_roots(&[root()], REACHABILITY)
        .expect("Reachability and its completeness proof must roots-load");
    assert_eq!(
        env.env.trusted_base().into_iter().collect::<BTreeSet<_>>(),
        before,
        "Reachability adds no trusted assumption"
    );
    assert!(finite_owned.contains(&env.globals[&format!("{FINITE}.Finite")]));
    assert_ne!(
        env.globals[&format!("{FINITE}.Finite")],
        env.globals[&format!("{VECTOR}.Fin")],
        "Finite evidence must not mint a duplicate bounded-index type"
    );
    for operation in ["is_empty", "accepted_word", "reachable", "find_word"] {
        let id = env.globals[&format!("{REACHABILITY}.{operation}")];
        assert!(reach_owned.contains(&id));
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    for law in [
        "is_empty_rejects",
        "accepted_word_accepts",
        "find_word_sound",
        "find_word_complete",
    ] {
        let id = env.globals[&format!("{REACHABILITY}.{law}")];
        assert!(reach_owned.contains(&id), "{law} must be package-owned");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{law} must be a kernel-checked transparent proof"
        );
    }
    for provider in [
        format!("{VECTOR}.Fin"),
        format!("{DERIVED}.list_elem"),
        "Algorithm.FormalLanguages.Dfa.Dfa".to_owned(),
    ] {
        assert!(env.globals.contains_key(&provider));
        assert!(!reach_owned.contains(&env.globals[&provider]));
    }
    env.elaborate_file(&format!(
        "import {FINITE} (Finite, MkFinite, elements, covers, fin_finite, \
         fin_elements, fin_elements_cover, pair_finite)\n\
         import {REACHABILITY} (is_empty, accepted_word, reachable, find_word, \
         is_empty_rejects, accepted_word_accepts, find_word_sound, find_word_complete)"
    ))
    .expect("the complete public finite-decision surface must be importable");
    env.elaborate_file(
        "import Algorithm.FormalLanguages.Dfa (Dfa, run, accepts)
         theorem checked_sound
           (q : Type) (a : Type) (fq : Finite q) (fa : Finite a)
           (d : Dfa q a) (target : q → Bool) (s : q) (w : List a)
           : Equal (Option (List a)) (find_word q a fq fa d target s) (Some (List a) w)
             → Equal Bool (target (run q a d s w)) True =
           find_word_sound q a fq fa d target s w
         theorem checked_complete
           (q : Type) (a : Type) (fq : Finite q) (fa : Finite a)
           (d : Dfa q a) (target : q → Bool) (s : q) (w : List a)
           : Equal Bool (target (run q a d s w)) True
             → Equal Bool (reachable q a fq fa d target s) True =
           find_word_complete q a fq fa d target s w
         theorem checked_accepted
           (q : Type) (a : Type) (fq : Finite q) (fa : Finite a)
           (d : Dfa q a) (w : List a)
           : Equal (Option (List a)) (accepted_word q a fq fa d) (Some (List a) w)
             → Equal Bool (accepts q a d w) True =
           accepted_word_accepts q a fq fa d w
         theorem checked_empty
           (q : Type) (a : Type) (fq : Finite q) (fa : Finite a)
           (d : Dfa q a) (w : List a)
           : Equal Bool (is_empty q a fq fa d) True
             → Equal Bool (accepts q a d w) False =
           is_empty_rejects q a fq fa d w",
    )
    .expect("all four universal public laws must inhabit the independent §2.3 client types");
}

/// MEASURED: the real interpreter evaluates the checked finite-state search
/// on a nonempty language, an unreachable final state and a disjoint-language
/// intersection with a pair_finite certificate. The returned Some carries a
/// word independently passed to the real accepts operation. CLAIMED: the
/// decision gives these observable answers under the public API. THE GAP:
/// finite concrete vectors supplement, but never replace, universal proofs.
#[test]
fn finite_state_witness_empty_and_disjoint_intersection_compute() {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], REACHABILITY)
        .expect("the decision package and transitive providers must roots-load");
    env.elaborate_file(
        "import Data.Vector.Vector (Fin, FZero, FSuc)
         import Data.Finite.Finite (Finite, fin_finite, pair_finite)
         import Algorithm.FormalLanguages.Dfa
           (Dfa, MkDfa, complement, intersection, accepts)
         import Algorithm.FormalLanguages.Reachability (is_empty, accepted_word)

         const states : Finite (Fin (Suc (Suc Zero))) = fin_finite (Suc (Suc Zero))
         const alphabet : Finite (Fin (Suc Zero)) = fin_finite (Suc Zero)

         const empty_only : Dfa (Fin (Suc (Suc Zero))) (Fin (Suc Zero)) =
           MkDfa (Fin (Suc (Suc Zero))) (Fin (Suc Zero))
             (λs. λx. FSuc (Suc Zero) (FZero Zero))
             (FZero (Suc Zero))
             (λs. match s { FZero n ↦ True; FSuc n j ↦ False })

         const nonempty : Dfa (Fin (Suc (Suc Zero))) (Fin (Suc Zero)) =
           complement (Fin (Suc (Suc Zero))) (Fin (Suc Zero)) empty_only

         const no_final : Dfa (Fin (Suc (Suc Zero))) (Fin (Suc Zero)) =
           MkDfa (Fin (Suc (Suc Zero))) (Fin (Suc Zero))
             (λs. λx. FSuc (Suc Zero) (FZero Zero))
             (FZero (Suc Zero))
             (λs. False)

         const nonempty_is_empty : Bool =
           is_empty (Fin (Suc (Suc Zero))) (Fin (Suc Zero))
             states alphabet nonempty
         const no_final_is_empty : Bool =
           is_empty (Fin (Suc (Suc Zero))) (Fin (Suc Zero))
             states alphabet no_final
         const witness : Option (List (Fin (Suc Zero))) =
           accepted_word (Fin (Suc (Suc Zero))) (Fin (Suc Zero))
             states alphabet nonempty
         const witness_accepted : Bool =
           match witness {
             None ↦ False;
             Some word ↦
               accepts (Fin (Suc (Suc Zero))) (Fin (Suc Zero)) nonempty word
           }
         const disjoint_is_empty : Bool =
           is_empty
             (Pair (Fin (Suc (Suc Zero))) (Fin (Suc (Suc Zero))))
             (Fin (Suc Zero))
             (pair_finite (Fin (Suc (Suc Zero))) (Fin (Suc (Suc Zero))) states states)
             alphabet
             (intersection (Fin (Suc (Suc Zero))) (Fin (Suc (Suc Zero)))
               (Fin (Suc Zero)) empty_only nonempty)",
    )
    .expect("finite-state examples must elaborate against the real packages");

    assert!(!checked_bool(&env, "nonempty_is_empty"));
    assert!(checked_bool(&env, "no_final_is_empty"));
    assert!(checked_bool(&env, "witness_accepted"));
    assert!(checked_bool(&env, "disjoint_is_empty"));

    let Some(Decl::Transparent { body, .. }) = env.env.lookup(env.globals["witness"]) else {
        panic!("accepted_word witness must have a checked transparent body");
    };
    match eval(&[], body, &env.env, &mut EvalStore::new()) {
        EvalVal::Ctor { id, args, .. } if id == env.prelude_env.some_id && args.len() == 2 => {
            assert!(
                matches!(&args[1], EvalVal::Ctor { id, .. } if *id == env.prelude_env.cons_id),
                "the accepted word is nonempty in this fixture"
            );
        }
        other => panic!("nonempty automaton must return Some word, got {other:?}"),
    }
}
