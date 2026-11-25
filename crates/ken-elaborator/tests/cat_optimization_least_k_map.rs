//! Spec 72 §4.8: least-k selection commutes with mapping candidates.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const DERIVED: &str = "Data.Collections.Derived";
const ARGMIN: &str = "Algorithm.Optimization.Argmin";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")
}

fn trusted_ids(env: &ElabEnv) -> BTreeSet<GlobalId> {
    env.env.trusted_base().into_iter().collect()
}

fn loaded() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Classes.LawfulClasses",
        "Core.Function.Combinators",
        "Core.Logic.Or",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Collections.List",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("provider {provider} roots-loads: {error:?}"));
    }
    let before = trusted_ids(&env);
    let derived_owned = env
        .elaborate_module_from_roots(&[root()], DERIVED)
        .expect("Derived roots-loads");
    let after_derived = trusted_ids(&env);
    assert_eq!(before, after_derived, "Derived adds no trusted identity");
    env.elaborate_module_from_roots(&[root()], "Data.Collections.NonEmpty")
        .expect("NonEmpty roots-loads after Derived");
    let before_argmin = trusted_ids(&env);
    assert_eq!(
        after_derived, before_argmin,
        "NonEmpty adds no trusted identity"
    );
    let argmin_owned = env
        .elaborate_module_from_roots(&[root()], ARGMIN)
        .expect("Argmin roots-loads");
    let after_argmin = trusted_ids(&env);
    eprintln!(
        "Trusted GlobalIds (providers → Derived → NonEmpty → Argmin): {} → {} → {} → {}",
        before.len(),
        after_derived.len(),
        before_argmin.len(),
        after_argmin.len()
    );
    assert_eq!(
        before_argmin, after_argmin,
        "Argmin adds no trusted identity"
    );
    for (module, law, owned) in [
        (DERIVED, "insert::map", &derived_owned),
        (DERIVED, "sort::map", &derived_owned),
        (DERIVED, "take::map", &derived_owned),
        (ARGMIN, "least_k_map", &argmin_owned),
    ] {
        let qualified = format!("{module}.{law}");
        let id = env.globals[&qualified];
        assert!(owned.contains(&id), "{qualified} is declared by its owner");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{qualified} is a checked Transparent proof"
        );
    }
    env
}

fn has_const(body: &Term, id: GlobalId) -> bool {
    let mut pending = vec![body];
    while let Some(term) = pending.pop() {
        if matches!(term, Term::Const { id: found, .. } if *found == id) {
            return true;
        }
        pending.extend(term.children());
    }
    false
}

fn assert_cites(env: &ElabEnv, client: &str, owner: &str) {
    let (_, body) = env
        .env
        .transparent_body(env.globals[client])
        .expect("checked client body");
    assert!(
        has_const(&body, env.globals[owner]),
        "{client} cites {owner}"
    );
}

const OPEN: &str = r#"import Algorithm.Optimization.Argmin (least_k, least_k_map)
import Data.Collections.Derived (map, insert, sort, take)
import Core.Function.Combinators (comp)

theorem open_least_k_map
      (a : Type) (b : Type) (f : a → b) (cost : b → Nat) (k : Nat) (xs : List a)
    : Equal (List b)
        (least_k b cost k (map a b f xs))
        (map a b f (least_k a (comp a b Nat cost f) k xs)) =
  least_k_map a b f cost k xs

theorem open_insert_map
      (a : Type) (b : Type) (f : a → b) (le1 : a → a → Bool) (le2 : b → b → Bool)
      (same_le : (x : a) → (y : a) → Equal Bool (le1 x y) (le2 (f x) (f y)))
      (x : a) (xs : List a)
    : Equal (List b) (map a b f (insert a le1 x xs)) (insert b le2 (f x) (map a b f xs)) =
  (proof map for insert) a b f le1 le2 same_le x xs

theorem open_sort_map
      (a : Type) (b : Type) (f : a → b) (le1 : a → a → Bool) (le2 : b → b → Bool)
      (same_le : (x : a) → (y : a) → Equal Bool (le1 x y) (le2 (f x) (f y)))
      (xs : List a)
    : Equal (List b) (map a b f (sort a le1 xs)) (sort b le2 (map a b f xs)) =
  (proof map for sort) a b f le1 le2 same_le xs

theorem open_take_map
      (a : Type) (b : Type) (f : a → b) (n : Nat) (xs : List a)
    : Equal (List b) (map a b f (take a n xs)) (take b n (map a b f xs)) =
  (proof map for take) a b f n xs
"#;

const ACCEPTED: &str = r#"import Algorithm.Optimization.Argmin (least_k, least_k_map)
import Data.Collections.Derived (map)
import Core.Function.Combinators (comp)

const one : Nat = Suc Zero
const two : Nat = Suc one

fn identity_cost (n : Nat) : Nat = n

fn reverse_three (n : Nat) : Nat =
  match n {
    Zero ↦ two;
    Suc m ↦
      match m {
        Zero ↦ one;
        Suc r ↦ Zero
      }
  }

const xs : List Nat = Cons Nat Zero (Cons Nat one (Cons Nat two (Nil Nat)))
const zero_only : List Nat = Cons Nat Zero (Nil Nat)
const two_only : List Nat = Cons Nat two (Nil Nat)

theorem v_mapped : Equal (List Nat) (least_k Nat identity_cost one (map Nat Nat reverse_three xs)) zero_only = Refl
theorem v_composed : Equal (List Nat)
    (map Nat Nat reverse_three (least_k Nat (comp Nat Nat Nat identity_cost reverse_three) one xs)) zero_only = Refl
theorem v_uncomposed : Equal (List Nat)
    (map Nat Nat reverse_three (least_k Nat identity_cost one xs)) two_only = Refl

theorem accepted_map : Equal (List Nat)
    (least_k Nat identity_cost one (map Nat Nat reverse_three xs))
    (map Nat Nat reverse_three (least_k Nat (comp Nat Nat Nat identity_cost reverse_three) one xs)) =
  least_k_map Nat Nat reverse_three identity_cost one xs
"#;

const REFUSED: &str = r#"theorem refused_uncomposed_cost : Equal (List Nat)
    (least_k Nat identity_cost one (map Nat Nat reverse_three xs))
    (map Nat Nat reverse_three (least_k Nat identity_cost one xs)) =
  least_k_map Nat Nat reverse_three identity_cost one xs
"#;

/// Source: Spec 72 §4.8 and Architect AC-0/AC-T. Durable invariant.
/// Measured: exact provider-first identity equality at each load boundary,
/// ownership and Transparency, and four open selective-import clients that
/// cite the four public law Consts. Claimed: these generic types remain
/// available, with no new trust. Gap: this does not audit provider trust.
/// Adding proved laws or refactoring proofs is compatible; adding an axiom,
/// hiding a law, or strengthening the premise breaks the contract.
#[test]
fn open_map_laws_are_owned_transparent_and_add_no_trust() {
    let mut env = loaded();
    env.elaborate_file(OPEN).expect("open clients check");
    for (client, owner) in [
        ("open_least_k_map", format!("{ARGMIN}.least_k_map")),
        ("open_insert_map", format!("{DERIVED}.insert::map")),
        ("open_sort_map", format!("{DERIVED}.sort::map")),
        ("open_take_map", format!("{DERIVED}.take::map")),
    ] {
        assert_cites(&env, client, &owner);
    }
}

/// Source: Spec 72 §4.8 and AC-2. Durable invariant.
/// Measured: mapped and composed selection yield [0], uncomposed selection
/// yields [2], and the closed equality cites the owned law. Claimed: the
/// selection can move across map when cost is composed. Gap: these three
/// closed values are fixture-specific; open types are checked separately.
/// Proof rewrites are compatible; losing the reversed ranking breaks this
/// example. Replacing the law call with direct Refl would bypass the pin.
#[test]
fn mapped_and_composed_routes_select_zero_uncomposed_selects_two() {
    let mut env = loaded();
    env.elaborate_file(ACCEPTED)
        .expect("independent value controls and closed law application check");
    assert_cites(&env, "accepted_map", &format!("{ARGMIN}.least_k_map"));
}

/// Source: Spec 72 §4.8 and AC-2. Durable invariant.
/// Measured: after a complete accepted prefix, changing only the right cost
/// to the uncomposed cost rejects the same law application with a kernel
/// TypeMismatch. Claimed: the law does not justify dropping the composition.
/// Gap: span containment locates the declaration, not the mismatched subterm.
/// Different diagnostic prose and offsets are compatible; accepting the false
/// equality or failing earlier at import or parse time is not.
#[test]
fn uncomposed_cost_refuses_at_the_same_law_application() {
    let mut env = loaded();
    env.elaborate_file(ACCEPTED).expect("fresh accepted prefix");
    let parsed = parse_decls(REFUSED).expect("refusal parses");
    let [SourceDecl::TheoremDecl {
        name, span: claim, ..
    }] = parsed.as_slice()
    else {
        panic!("one refused theorem");
    };
    assert_eq!(name, "refused_uncomposed_cost");
    let token = REFUSED.rfind("least_k_map").expect("law token");
    match env.elaborate_file(REFUSED) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if claim.start <= span.start
            && span.start <= token
            && token + "least_k_map".len() <= span.end
            && span.end <= claim.end =>
        {
            eprintln!("Uncomposed cost: KernelRejected(TypeMismatch) at {span:?}");
        }
        other => panic!("must reject at the uncomposed law application: {other:?}"),
    }
}

/// Source: Architect AC-0, ruling 2 (publication boundary). Durable invariant.
/// Measured: each qualified selective import refuses as UnboundName in a
/// fresh client of the loaded production module. Claimed: the decision
/// helpers are not a public API. Gap: not an exhaustive Derived roster;
/// the exact publication test covers that separately. Removing or rewriting
/// a private helper is compatible; making one importable is not.
#[test]
fn insert_map_decision_helpers_are_private() {
    for helper in [
        "derived_map_insert_left",
        "derived_map_insert_right",
        "derived_map_insert_choice",
    ] {
        let mut client = loaded();
        let source = format!("import {DERIVED} ({helper})\n");
        match client.elaborate_file(&source) {
            Err(ElabError::UnboundName { name, .. }) if name == format!("{DERIVED}.{helper}") => {}
            other => panic!("{helper} must refuse selective import: {other:?}"),
        }
    }
}
