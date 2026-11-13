//! Nat compare's completeness proofs keep their exact checked claims and visibility.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, Term};

const ORDER: &str = "Data.Numeric.Nat.Order";
const LAWFUL: &str = "Core.Classes.LawfulClasses";

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn load_order() -> (ElabEnv, Vec<GlobalId>) {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        LAWFUL,
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
    ] {
        env.elaborate_module_from_roots(&[catalog_root()], provider)
            .unwrap_or_else(|error| panic!("{provider} must roots-load: {error:?}"));
    }
    let providers: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let owned = env
        .elaborate_module_from_roots(&[catalog_root()], ORDER)
        .expect("Nat Order must roots-load with its comparison proofs");
    let with_order: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(with_order, providers, "Nat Order must add no trust");
    (env, owned)
}

fn applied(head: Term, args: impl IntoIterator<Item = Term>) -> Term {
    args.into_iter().fold(head, Term::app)
}

fn mentions_global(term: &Term, target: GlobalId) -> bool {
    match term {
        Term::Const { id, .. } | Term::IndFormer { id, .. } | Term::Constructor { id, .. }
            if *id == target =>
        {
            true
        }
        Term::Elim { fam, .. } if *fam == target => true,
        _ => term
            .children()
            .into_iter()
            .any(|child| mentions_global(child, target)),
    }
}

fn assert_comparison_proposition(law: &str) {
    let (env, owned) = load_order();
    let id = env.globals[&format!("{ORDER}.compare::{law}")];
    assert!(owned.contains(&id), "compare::{law} must be Order-owned");
    let Decl::Transparent { ty, .. } = env.env.lookup(id).expect("law must load") else {
        panic!("compare::{law} must be a checked proof, not an assumption");
    };

    let global = |name: &str| Term::const_(env.globals[name], vec![]);
    let constructor = |name: &str| Term::constructor(env.globals[name], vec![]);
    let nat = Term::indformer(env.globals["Nat"], vec![]);
    let bool_ty = Term::indformer(env.globals["Bool"], vec![]);
    for name in ["OrdResult", "Lt", "Eq", "Gt"] {
        assert!(
            !env.globals.contains_key(&format!("{ORDER}.{name}")),
            "Nat Order must not own a second {name}"
        );
    }
    let ord_result = Term::indformer(env.globals["Core.Logic.OrdResult.OrdResult"], vec![]);
    let compare_id = env.globals[&format!("{ORDER}.compare")];
    let lt_nat_id = env.globals[&format!("{ORDER}.lt_nat")];
    assert!(owned.contains(&compare_id) && owned.contains(&lt_nat_id));
    let comparison =
        |left: Term, right: Term| applied(global(&format!("{ORDER}.compare")), [left, right]);
    let strict =
        |left: Term, right: Term| applied(global(&format!("{ORDER}.lt_nat")), [left, right]);
    let equality =
        |carrier: Term, left: Term, right: Term| applied(global("Equal"), [carrier, left, right]);
    let lt = constructor("Core.Logic.OrdResult.Lt");
    let eq = constructor("Core.Logic.OrdResult.Eq");
    let gt = constructor("Core.Logic.OrdResult.Gt");

    let expected = match law {
        "self_eq" => {
            let conclusion = equality(ord_result, comparison(Term::var(0), Term::var(0)), eq);
            Term::pi(nat, conclusion)
        }
        "from_lt" => {
            // Under a,b: a=1, b=0. After the premise: a=2, b=1.
            let premise = equality(
                bool_ty,
                strict(Term::var(1), Term::var(0)),
                constructor("True"),
            );
            let conclusion = equality(ord_result, comparison(Term::var(2), Term::var(1)), lt);
            Term::pi(nat.clone(), Term::pi(nat, Term::pi(premise, conclusion)))
        }
        "flip_lt" => {
            // Under a,b: a=1, b=0. After the premise: a=2, b=1.
            let premise = equality(
                ord_result.clone(),
                comparison(Term::var(1), Term::var(0)),
                lt,
            );
            let conclusion = equality(ord_result, comparison(Term::var(1), Term::var(2)), gt);
            Term::pi(nat.clone(), Term::pi(nat, Term::pi(premise, conclusion)))
        }
        "swap" => {
            let flip = global("Core.Logic.OrdResult.ord_result_flip");
            let conclusion = equality(
                ord_result.clone(),
                comparison(Term::var(0), Term::var(1)),
                applied(flip, [comparison(Term::var(1), Term::var(0))]),
            );
            Term::pi(nat.clone(), Term::pi(nat, conclusion))
        }
        _ => panic!("unexpected law {law}"),
    };
    assert_eq!(
        ty, &expected,
        "compare::{law}'s raw checked proposition changed"
    );
}

/// Promise class: durable checked-proposition invariant.
/// MEASURED: the roots-loaded, Order-owned transparent proof has the raw
/// one-binder type with the canonical Core.Logic.OrdResult identities, and
/// compare applied twice to the same de Bruijn variable. CLAIMED: comparing
/// a Nat with itself returns Eq. THE GAP: the pin checks the proposition and
/// checked body, not execution cost or the outcome of a mutated operation.
#[test]
fn compare_self_eq_has_exact_checked_proposition() {
    assert_comparison_proposition("self_eq");
}

/// Promise class: durable checked-proposition invariant.
/// MEASURED: the raw three-binder type contains the exact lt_nat, True,
/// compare, Bool and canonical Core.Logic.OrdResult identities at the specified binder indices.
/// CLAIMED: a true strict order forces Lt. THE GAP: imported provider trust
/// is checked separately; it is not erased by this private proof.
#[test]
fn compare_from_lt_has_exact_checked_proposition() {
    assert_comparison_proposition("from_lt");
}

/// Promise class: durable checked-proposition invariant.
/// MEASURED: the raw three-binder type uses the same compare GlobalId with
/// reversed Nat indices only in the conclusion, and exact canonical Core.Logic.OrdResult identities.
/// CLAIMED: a Lt comparison reverses to Gt. THE GAP: checking the statement
/// and body does not measure a public client or execution complexity.
#[test]
fn compare_flip_lt_has_exact_checked_proposition() {
    assert_comparison_proposition("flip_lt");
}

/// Promise class: durable checked-proposition invariant.
/// MEASURED: the raw two-binder type uses the canonical Core flip and Order
/// compare identities at a=var 1, b=var 0, reversing the latter in the goal.
/// CLAIMED: swapping the arguments flips the result. THE GAP: the pin checks
/// the proposition and checked body, not execution cost.
#[test]
fn compare_swap_has_exact_checked_proposition() {
    assert_comparison_proposition("swap");
}

/// Promise class: durable visibility and trust invariant.
/// MEASURED: self_eq and swap are selectable at their checked types;
/// from_lt and flip_lt are UnboundName to the same sort of client; the private
/// probes are stated at the laws' exact types, so a reachable law checks and
/// reaches the Ok arm. Loading Order preserves its providers' exact trusted set.
/// CLAIMED: Nat Order's public compare laws are the agreement laws plus
/// self_eq and swap, and the premise-carrying completeness proofs stay private.
/// THE GAP: imported providers retain their own inherited trust.
#[test]
fn comparison_law_visibility_is_exact_without_new_trust() {
    let (mut env, _) = load_order();
    env.elaborate_file(
        "import Data.Numeric.Nat.Order (lt_nat)\n\
         theorem public_strict_control (n : Nat)\n\
           : Equal Bool (lt_nat n (Suc n)) True = (proof self_suc for lt_nat) n",
    )
    .expect("existing public strict-order law must remain selectable");
    for (law, client) in [
        (
            "from_lt",
            "import Data.Numeric.Nat.Order (compare, lt_nat)\n\
             import Core.Logic.OrdResult (OrdResult, Lt)\n\
             theorem private_from_lt_probe (a : Nat) (b : Nat)\n\
               : Equal Bool (lt_nat a b) True → Equal OrdResult (compare a b) Lt\n\
               = (proof from_lt for compare) a b",
        ),
        (
            "flip_lt",
            "import Data.Numeric.Nat.Order (compare)\n\
             import Core.Logic.OrdResult (OrdResult, Lt, Gt)\n\
             theorem private_flip_lt_probe (a : Nat) (b : Nat)\n\
               : Equal OrdResult (compare a b) Lt → Equal OrdResult (compare b a) Gt\n\
               = (proof flip_lt for compare) a b",
        ),
    ] {
        match env.elaborate_file(client) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{ORDER}.compare::{law}"));
            }
            Err(other) => panic!("private compare::{law} failed for another reason: {other:?}"),
            Ok(_) => panic!("private compare::{law} unexpectedly reachable"),
        }
    }
    env.elaborate_file(
        "import Data.Numeric.Nat.Order (compare)\n\
         import Core.Logic.OrdResult (OrdResult, Eq, ord_result_flip)\n\
         theorem public_self_eq_probe (n : Nat)\n\
           : Equal OrdResult (compare n n) Eq = (proof self_eq for compare) n\n\
         theorem public_swap_probe (a : Nat) (b : Nat)\n\
           : Equal OrdResult (compare b a) (ord_result_flip (compare a b))\n\
           = (proof swap for compare) a b",
    )
    .expect("public compare::self_eq and compare::swap must be selectable");
    for (probe, law) in [
        ("public_self_eq_probe", "self_eq"),
        ("public_swap_probe", "swap"),
    ] {
        let probe_id = env.globals[probe];
        let law_id = env.globals[&format!("{ORDER}.compare::{law}")];
        let Decl::Transparent { body, .. } = env.env.lookup(probe_id).expect("client must load")
        else {
            panic!("{probe} must be a checked transparent proof");
        };
        assert!(
            mentions_global(body, law_id),
            "{probe} must use the Order-owned compare::{law} law"
        );
    }
}
