//! Consumer-view acceptance of the four private generic Derived sort laws.
//! The real roots-loaded package owns the checked proof terms; aliases in this
//! test environment alone let a generic consumer inspect their entire types.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use ken_elaborator::{foreign::trusted_base_delta, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

fn references(term: &Term, target: GlobalId) -> usize {
    usize::from(matches!(term, Term::Const { id, .. } if *id == target))
        + term
            .children()
            .into_iter()
            .map(|child| references(child, target))
            .sum::<usize>()
}

fn load() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], "Data.Collections.Derived")
        .expect("roots-load real Derived and its providers");
    for (module, names) in [
        (
            "Data.Collections.Derived",
            &[
                "count",
                "Perm",
                "insert",
                "sort",
                "insert::count",
                "sort::perm",
                "insert::sorted",
                "sort::sorted",
            ][..],
        ),
        (
            "Core.Classes.LawfulClasses",
            &["IsTrue", "bool_or", "leq_nat"][..],
        ),
    ] {
        for name in names {
            let id = env.globals[&format!("{module}.{name}")];
            env.globals.insert((*name).to_owned(), id);
        }
    }
    env
}

/// Promise class: durable checked contract. Raw kernel types bind each proof
/// to Derived's own operation; consumers verify all four complete telescopes.
#[test]
fn generic_sort_laws_are_attached_to_the_derived_owners() {
    let mut env = load();
    let insert = env.globals["Data.Collections.Derived.insert"];
    let sort = env.globals["Data.Collections.Derived.sort"];
    for (name, operation) in [
        ("insert::count", insert),
        ("sort::perm", sort),
        ("insert::sorted", insert),
        ("sort::sorted", sort),
    ] {
        let id = env.globals[&format!("Data.Collections.Derived.{name}")];
        let ty = match env.env.lookup(id) {
            Some(Decl::Transparent { ty, .. }) => ty,
            other => panic!("{name} must be a transparent checked proof: {other:?}"),
        };
        assert!(
            references(ty, operation) > 0,
            "{name} must type its own Derived operation"
        );
        assert!(
            trusted_base_delta(&env.env, id).is_empty(),
            "{name} must add no postulate or primitive to its proof closure"
        );
    }
    for consumer in [
        "theorem count_insert_consumer \
         (a : Type) (le : a → a → Bool) (x : a) (xs : List a) \
         (eqf : a → a → Bool) (q : a) : \
         Equal Nat (count a eqf q (Cons a x xs)) (count a eqf q (insert a le x xs)) = \
         insert::count a le x xs eqf q",
        "theorem perm_sort_consumer \
         (a : Type) (le : a → a → Bool) (xs : List a) (eqf : a → a → Bool) : \
         Perm a eqf xs (sort a le xs) = sort::perm a le xs eqf",
        "theorem sorted_insert_consumer \
         (a : Type) (le : a → a → Bool) \
         (total : (x : a) → (y : a) → IsTrue (bool_or (le x y) (le y x))) \
         (x : a) (xs : List a) : \
         is_sorted a le xs → is_sorted a le (insert a le x xs) = \
         insert::sorted a le total x xs",
        "theorem sorted_sort_consumer \
         (a : Type) (le : a → a → Bool) \
         (total : (x : a) → (y : a) → IsTrue (bool_or (le x y) (le y x))) \
         (xs : List a) : is_sorted a le (sort a le xs) = \
         sort::sorted a le total xs",
    ] {
        env.elaborate_decl(consumer)
            .unwrap_or_else(|error| panic!("full generic consumer must elaborate: {error:?}"));
    }
}

/// Promise class: semantic discriminator. Both comparators produce the same
/// sorted data. One closed conjunction term works only when the resulting
/// adjacent pair is ordered; rejection occurs after the shared fixture elaborates.
#[test]
fn a_non_total_comparator_sorts_the_fixture_but_not_its_proof() {
    let mut env = load();
    env.elaborate_file(
        "fn le_none (x : Nat) (y : Nat) : Bool = False\n\
         const cat_sort_xs : List Nat = \
           Cons Nat (Suc Zero) (Cons Nat Zero (Nil Nat))\n\
         const cat_sort_result : List Nat = \
           Cons Nat Zero (Cons Nat (Suc Zero) (Nil Nat))",
    )
    .expect("the entire shared fixture must elaborate before either verdict");
    for (name, predicate) in [
        ("sort_none_same_result", "sort Nat le_none cat_sort_xs"),
        ("sort_nat_same_result", "sort Nat leq_nat cat_sort_xs"),
    ] {
        env.elaborate_decl(&format!(
            "theorem {name} : Equal (List Nat) ({predicate}) cat_sort_result = Refl"
        ))
        .unwrap_or_else(|error| panic!("{name} concrete sort must agree: {error:?}"));
    }
    let closed_sorted_proof = "and_intro Top Top Proved Proved";
    env.elaborate_decl(&format!(
        "theorem good_sorted : is_sorted Nat leq_nat cat_sort_result = {closed_sorted_proof}"
    ))
    .expect("the sorted Nat instance must accept the shared closed proof term");
    let rejected = env
        .elaborate_decl(&format!(
            "theorem bad_sorted : is_sorted Nat le_none (sort Nat le_none cat_sort_xs) = {closed_sorted_proof}"
        ))
        .expect_err("False on both elements must not prove sortedness");
    assert!(
        matches!(
            rejected,
            ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                ..
            }
        ),
        "the false closed proof, not the fixture, must be rejected: {rejected:?}"
    );
}
