//! Public least-k selection and its checked list-prefix providers.
//!
//! Source: `spec/50-stdlib/72-optimization.md` §4 and
//! `conformance/stdlib/optimization/seed-least-k.md` (accepted prefixes,
//! eight public law instances and two isolated false-result refusals).
//!
//! Promise class: durable semantic invariant. The public selector is a stable
//! cost-ordered prefix, not a candidate-equality decision or a new trust root.
//! The closed vectors are fixed examples; the generic public laws are the
//! checked guarantee for all candidate types, bounds and costs.

use std::{collections::BTreeSet, path::PathBuf};

use ken_elaborator::{ElabEnv, ElabError};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn load_provider() -> ElabEnv {
    let mut env = ElabEnv::new().expect("fresh elaborator");
    let roots = [root()];
    for provider in [
        "Core.Classes.LawfulClasses",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Order",
        "Data.Collections.List",
        "Data.Collections.Derived",
        "Data.Collections.NonEmpty",
    ] {
        env.elaborate_module_from_roots(&roots, provider)
            .unwrap_or_else(|error| panic!("{provider} must roots-load: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    env.elaborate_module_from_roots(&roots, "Algorithm.Optimization.Argmin")
        .expect("least-k provider checks through the real roots loader");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(before, after, "least-k adds no trusted identity");
    env
}

const PUBLIC_CLIENT: &str = r#"
import Algorithm.Optimization.Argmin
  (argmin, cost_le, least_k, least_k_length, least_k_member,
    least_k_sorted, least_k_ties_earliest, least_k_cheaper_class_selected,
    least_k_separation, least_k_one_nil, least_k_one_cons)
import Core.Classes.LawfulClasses (leq_nat)
import Data.Collections.Derived
  (take, filter, eq_from_ord, is_sorted as derived_sorted, length, list_elem)
import Data.Collections.NonEmpty (nonempty_cons)
import Data.Numeric.Nat.Order (min, lt_nat)

theorem checked_length (a : Type) (cost : a → Nat) (k : Nat) (xs : List a)
  : Equal Nat (length a (least_k a cost k xs)) (min k (length a xs)) =
  least_k_length a cost k xs

theorem checked_member (a : Type) (cost : a → Nat) (k : Nat) (xs : List a) (y : a)
  : list_elem a y (least_k a cost k xs) → list_elem a y xs =
  least_k_member a cost k xs y

theorem checked_sorted (a : Type) (cost : a → Nat) (k : Nat) (xs : List a)
  : derived_sorted a (cost_le a cost) (least_k a cost k xs) =
  least_k_sorted a cost k xs

theorem checked_ties (a : Type) (cost : a → Nat) (k : Nat) (xs : List a) (w : a)
  : Equal (List a)
      (filter a (eq_from_ord a (cost_le a cost) w) (least_k a cost k xs))
      (take a
        (length a (filter a (eq_from_ord a (cost_le a cost) w) (least_k a cost k xs)))
        (filter a (eq_from_ord a (cost_le a cost) w) xs)) =
  least_k_ties_earliest a cost k xs w

theorem checked_cheaper_class
    (a : Type) (cost : a → Nat) (k : Nat) (xs : List a) (y : a) (z : a)
  : list_elem a y (least_k a cost k xs)
    → list_elem a z xs
    → Equal Bool (lt_nat (cost z) (cost y)) True
    → Equal (List a)
        (filter a (eq_from_ord a (cost_le a cost) z) (least_k a cost k xs))
        (filter a (eq_from_ord a (cost_le a cost) z) xs) =
  least_k_cheaper_class_selected a cost k xs y z

theorem checked_separation
    (a : Type) (cost : a → Nat) (k : Nat) (xs : List a) (y : a) (z : a)
  : list_elem a y (least_k a cost k xs)
    → list_elem a z xs
    → (list_elem a z (least_k a cost k xs) → Bottom)
    → IsTrue (leq_nat (cost y) (cost z)) =
  least_k_separation a cost k xs y z

theorem checked_one_nil (a : Type) (cost : a → Nat)
  : Equal (List a) (least_k a cost (Suc Zero) (Nil a)) (Nil a) =
  least_k_one_nil a cost

theorem checked_one_cons (a : Type) (cost : a → Nat) (x : a) (rest : List a)
  : Equal (List a)
      (least_k a cost (Suc Zero) (Cons a x rest))
      (Cons a (argmin a cost (nonempty_cons a x rest)) (Nil a)) =
  least_k_one_cons a cost x rest
"#;

#[test]
fn public_generic_laws_check_without_a_trusted_identity_delta() {
    let mut env = load_provider();
    env.elaborate_file(PUBLIC_CLIENT)
        .expect("all eight public declarations retain their exact generic types");
}

const ITEM_FIXTURE: &str = r#"
import Algorithm.Optimization.Argmin (least_k)

data Item = A | B | C | D
const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
fn item_cost (x : Item) : Nat = match x { A ↦ two; B ↦ one; C ↦ two; D ↦ three }
const items : List Item = Cons Item A (Cons Item B (Cons Item C (Cons Item D (Nil Item))))
const chosen_two : List Item = Cons Item B (Cons Item A (Nil Item))
const chosen_three : List Item = Cons Item B (Cons Item A (Cons Item C (Nil Item)))
const chosen_all : List Item = Cons Item B (Cons Item A (Cons Item C (Cons Item D (Nil Item))))
"#;

#[test]
fn stable_prefix_accepts_empty_two_three_and_oversized_bounds() {
    // MEASURED: independent labelled values at four bounds reduce to the
    // specified ordered prefixes. CLAIMED: the selector obeys the closed
    // cost/tie/boundary behavior. THE GAP: these are finite instances, not
    // a general proof; the public checked laws cover the generic obligation.
    for (name, claim) in [
        (
            "two_earliest",
            "Equal (List Item) (least_k Item item_cost two items) chosen_two = Refl",
        ),
        (
            "three_ties",
            "Equal (List Item) (least_k Item item_cost three items) chosen_three = Refl",
        ),
        (
            "zero",
            "Equal (List Item) (least_k Item item_cost Zero items) (Nil Item) = Proved",
        ),
        (
            "nine_full",
            "Equal (List Item) (least_k Item item_cost (Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc Zero))))))))) items) chosen_all = Refl",
        ),
    ] {
        let mut env = load_provider();
        env.elaborate_file(&format!("{ITEM_FIXTURE}\ntheorem observed_{name} : {claim}\n"))
            .unwrap_or_else(|error| panic!("{name} must be accepted: {error:?}"));
    }
}

#[test]
fn sorted_prefix_and_equal_cost_class_controls() {
    // MEASURED: closed sortedness and both equal-cost class prefixes check
    // against the same selector as the accepted value rows. CLAIMED: the
    // selected prefix is sorted and preserves ties without duplication.
    // THE GAP: finite classes are not a proof for every cost function; the
    // public generic sorted and earliest-tie laws carry that obligation.
    let class_a = "eq_from_ord Item (cost_le Item item_cost) A";
    let class_b = "eq_from_ord Item (cost_le Item item_cost) B";
    for (name, claim, proof) in [
        (
            "sorted",
            "derived_sorted Item (cost_le Item item_cost) (least_k Item item_cost two items)".to_owned(),
            "and_intro (Equal Bool (cost_le Item item_cost B A) True) (derived_sorted Item (cost_le Item item_cost) (Cons Item A (Nil Item))) Proved Proved",
        ),
        (
            "cheapest_class",
            format!("Equal (List Item) (filter Item ({class_b}) (least_k Item item_cost two items)) (take Item (length Item (filter Item ({class_b}) (least_k Item item_cost two items))) (filter Item ({class_b}) items))"),
            "Refl",
        ),
        (
            "later_tied_class",
            format!("Equal (List Item) (filter Item ({class_a}) (least_k Item item_cost two items)) (take Item (length Item (filter Item ({class_a}) (least_k Item item_cost two items))) (filter Item ({class_a}) items))"),
            "Refl",
        ),
        (
            "cheaper_class_complete",
            format!("Equal (List Item) (filter Item ({class_a}) (least_k Item item_cost three items)) (filter Item ({class_a}) items)"),
            "Refl",
        ),
    ] {
        let mut env = load_provider();
        let client = format!(
            "import Core.Logic.Conjunction (and_intro)\n\
             import Algorithm.Optimization.Argmin (cost_le)\n\
             import Data.Collections.Derived (filter, take, length, eq_from_ord, is_sorted as derived_sorted)\n\
             {ITEM_FIXTURE}\n\
             theorem selected_{name} : {claim} = {proof}\n"
        );
        env.elaborate_file(&client)
            .unwrap_or_else(|error| panic!("{name} must satisfy the closed law: {error:?}"));
    }
}

#[test]
fn costlier_and_later_tie_claims_refuse_at_their_own_refl() {
    // MEASURED: changing only the k=2 endpoint from its accepted [B,A]
    // to [B,D] or [B,C] rejects the same Refl proof form at that theorem.
    // CLAIMED: neither a costlier replacement nor a later equal-cost value
    // can masquerade as the first two stable minima. THE GAP: the exact
    // Refl span excludes an unrelated import, fixture or syntax refusal;
    // the generic laws establish the open property.
    for (name, claim) in [
        (
            "costlier",
            "Equal (List Item) (least_k Item item_cost two items) (Cons Item B (Cons Item D (Nil Item))) = Refl",
        ),
        (
            "later_tie",
            "Equal (List Item) (least_k Item item_cost two items) (Cons Item B (Cons Item C (Nil Item))) = Refl",
        ),
    ] {
        let mut env = load_provider();
        let source = format!("{ITEM_FIXTURE}\ntheorem refused_{name} : {claim}\n");
        let error = env
            .elaborate_file(&source)
            .expect_err(&format!("{name}: false selection claim must refuse"));
        match error {
            ElabError::TypeMismatch { span, reason } => {
                assert_eq!(reason, "Refl expects an `Eq`-shaped goal");
                assert_eq!(&source[span.start..span.end], "Refl");
            }
            other => panic!("{name} refused at an unrelated error: {other:?}"),
        }
    }
}
