//! Consumer-view controls for the private Derived filter-membership laws.
//! The fixture roots-loads the real package, then exposes its private checked
//! identities only inside the test environment; no catalog export is added.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

fn reference_count(term: &Term, target: GlobalId) -> usize {
    usize::from(matches!(term, Term::Const { id, .. } if *id == target))
        + term
            .children()
            .into_iter()
            .map(|child| reference_count(child, target))
            .sum::<usize>()
}

fn load() -> (ElabEnv, GlobalId) {
    let mut env = ElabEnv::new().expect("base environment");
    let prelude_filter = env.globals["filter"];
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], "Data.Collections.Derived")
        .expect("Derived must load its real provider closure");
    for (module, names) in [
        (
            "Data.Collections.Derived",
            &["mem", "mem_filter", "mem_filter_sound"][..],
        ),
        ("Core.Classes.LawfulClasses", &["IsTrue", "bool_and"][..]),
    ] {
        for name in names {
            let id = env.globals[&format!("{module}.{name}")];
            env.bind_session_name(name, id)
                .expect("checked filter-law provider alias");
        }
    }
    assert_eq!(env.globals["filter"], prelude_filter);
    (env, prelude_filter)
}

/// Promise class: durable checked contract. The raw types, not documentation
/// spellings, cite the installed prelude filter; generic consumer applications
/// verify the two complete binder lists and their distinct conclusions.
#[test]
fn private_law_statements_resolve_the_installed_filter() {
    let (mut env, installed_filter) = load();
    for name in ["mem_filter", "mem_filter_sound"] {
        let id = env.globals[&format!("Data.Collections.Derived.{name}")];
        let ty = match env.env.lookup(id) {
            Some(Decl::Transparent { ty, .. }) => ty,
            other => panic!("{name} must remain a checked transparent theorem: {other:?}"),
        };
        assert!(
            reference_count(ty, installed_filter) > 0,
            "{name}'s raw type must use installed prelude filter"
        );
    }
    assert!(!env.globals.contains_key("Data.Collections.Derived.filter"));
    env.elaborate_decl(
        "theorem filter_membership_generic_consumer \
         (a : Type) (eqf : a → a → Bool) (p : a → Bool) (x : a) \
         (compat : (y : a) → IsTrue (eqf x y) → Equal Bool (p y) (p x)) \
         (xs : List a) \
         : Equal Bool (mem a eqf x (filter a p xs)) \
             (bool_and (mem a eqf x xs) (p x)) = \
         mem_filter a eqf p x compat xs",
    )
    .expect("the generic compatibility law must have its checked contract");
    env.elaborate_decl(
        "theorem filter_sound_generic_consumer \
         (a : Type) (eqf : a → a → Bool) (p : a → Bool) (x : a) (xs : List a) \
         (membership : IsTrue (mem a eqf x (filter a p xs))) \
         : IsTrue (mem a eqf x xs) = \
         mem_filter_sound a eqf p x xs membership",
    )
    .expect("the generic soundness law must have its checked contract");
}

/// Promise class: durable semantic discriminator. On the same x and xs, an
/// arbitrary comparator that matches Zero while p disagrees with x gives
/// unequal endpoints; Nat structural equality gives equal endpoints. Rejection
/// must arise at the false proof, after every fixture declaration elaborates.
#[test]
fn compatibility_premise_distinguishes_true_and_false_instances() {
    let (mut env, _) = load();
    env.elaborate_file(
        "fn cat_eq_any (x : Nat) (y : Nat) : Bool = True\n\
         fn cat_eq_nat (x : Nat) (y : Nat) : Bool = \
           match x { Zero ↦ match y { Zero ↦ True; Suc k ↦ False }; \
                     Suc k ↦ match y { Zero ↦ False; Suc j ↦ cat_eq_nat k j } }\n\
         fn cat_is_zero (y : Nat) : Bool = \
           match y { Zero ↦ True; Suc k ↦ False }\n\
         const cat_x : Nat = Suc Zero\n\
         const cat_xs : List Nat = Cons Nat Zero (Nil Nat)",
    )
    .expect("shared, nontrivial fixture must elaborate");

    for (name, equation) in [
        (
            "unconstrained_left_true",
            "Equal Bool (mem Nat cat_eq_any cat_x (filter Nat cat_is_zero cat_xs)) True",
        ),
        (
            "unconstrained_right_false",
            "Equal Bool (bool_and (mem Nat cat_eq_any cat_x cat_xs) (cat_is_zero cat_x)) False",
        ),
        (
            "nat_equality_left_false",
            "Equal Bool (mem Nat cat_eq_nat cat_x (filter Nat cat_is_zero cat_xs)) False",
        ),
        (
            "nat_equality_right_false",
            "Equal Bool (bool_and (mem Nat cat_eq_nat cat_x cat_xs) (cat_is_zero cat_x)) False",
        ),
    ] {
        env.elaborate_decl(&format!("theorem {name} : {equation} = Proved"))
            .unwrap_or_else(|error| panic!("{name} must reduce as stated: {error:?}"));
    }
    env.elaborate_decl(
        "theorem compatible_nat_equality_instance \
         : Equal Bool (mem Nat cat_eq_nat cat_x (filter Nat cat_is_zero cat_xs)) \
             (bool_and (mem Nat cat_eq_nat cat_x cat_xs) (cat_is_zero cat_x)) = Proved",
    )
    .expect("the compatible Nat equality instance must accept Proved");

    let rejected = env
        .elaborate_decl(
            "theorem incompatible_unconstrained_instance \
             : Equal Bool (mem Nat cat_eq_any cat_x (filter Nat cat_is_zero cat_xs)) \
                 (bool_and (mem Nat cat_eq_any cat_x cat_xs) (cat_is_zero cat_x)) = Proved",
        )
        .expect_err("the incompatible no-compat equation must be false");
    assert!(
        matches!(
            rejected,
            ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                ..
            }
        ),
        "the failure must reach the false proof obligation, got {rejected:?}"
    );
}
