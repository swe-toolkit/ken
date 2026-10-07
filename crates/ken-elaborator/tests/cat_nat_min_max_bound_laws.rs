//! Private meet and join proofs keep their complete kernel-checked claims.

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
        .expect("Nat Order must roots-load with its tightness laws");
    let with_order: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(with_order, providers, "Nat Order must add no trust");
    (env, owned)
}

fn applied(head: Term, args: impl IntoIterator<Item = Term>) -> Term {
    args.into_iter().fold(head, Term::app)
}

fn assert_bound_proposition(greatest: bool) {
    let (env, owned) = load_order();
    let (operation, law) = if greatest {
        ("min", "greatest")
    } else {
        ("max", "least")
    };
    let id = env.globals[&format!("{ORDER}.{operation}::{law}")];
    assert!(owned.contains(&id), "{law} must be owned by Nat Order");
    let Decl::Transparent { ty, .. } = env.env.lookup(id).expect("law must load") else {
        panic!("{operation}::{law} must be a checked proof, not an assumption");
    };

    let global = |name: &str| Term::const_(env.globals[name], vec![]);
    let nat = Term::indformer(env.globals["Nat"], vec![]);
    let leq =
        |left: Term, right: Term| applied(global(&format!("{LAWFUL}.leq_nat")), [left, right]);
    let is_true = |value: Term| applied(global(&format!("{LAWFUL}.IsTrue")), [value]);
    // After k,m,n: k=2, m=1, n=0. The two arrow binders shift these
    // indices to k=4, m=3, n=2 at the conclusion.
    let first = if greatest {
        is_true(leq(Term::var(2), Term::var(1)))
    } else {
        is_true(leq(Term::var(1), Term::var(2)))
    };
    let second = if greatest {
        is_true(leq(Term::var(3), Term::var(1)))
    } else {
        is_true(leq(Term::var(1), Term::var(3)))
    };
    let operation_result = applied(
        global(&format!("{ORDER}.{operation}")),
        [Term::var(3), Term::var(2)],
    );
    let conclusion = if greatest {
        is_true(leq(Term::var(4), operation_result))
    } else {
        is_true(leq(operation_result, Term::var(4)))
    };
    let expected = Term::pi(
        nat.clone(),
        Term::pi(
            nat.clone(),
            Term::pi(nat, Term::pi(first, Term::pi(second, conclusion))),
        ),
    );
    assert_eq!(
        ty, &expected,
        "{operation}::{law}'s raw checked proposition changed"
    );
}

/// Promise class: durable checked-proposition invariant.
/// MEASURED: a roots-loaded, Order-owned transparent proof's raw five-binder
/// type contains the canonical leq_nat, IsTrue and min GlobalIds, with the
/// left and right assumptions and the tight lower-bound conclusion at exact
/// de Bruijn positions. CLAIMED: any common lower bound lies below min.
/// THE GAP: the pin checks the statement and a checked body, not execution.
#[test]
fn min_greatest_has_exact_checked_proposition() {
    assert_bound_proposition(true);
}

/// Promise class: durable checked-proposition invariant.
/// MEASURED: the same independent type construction checks max::least's
/// upper-bound premises and the reversed order in its conclusion, using the
/// checked identities and exact binder positions. CLAIMED: max is below every
/// common upper bound. THE GAP: imported trust is checked separately.
#[test]
fn max_least_has_exact_checked_proposition() {
    assert_bound_proposition(false);
}

/// Promise class: durable visibility and trust invariant.
/// MEASURED: a public attached law remains usable from a selective import;
/// each new private attached law is unavailable to that same kind of client,
/// and the exact trusted GlobalId set is unchanged by roots-loading Order.
/// CLAIMED: tightness adds no export or trust. THE GAP: imported providers
/// still carry their own trust; this checks only Order's incremental delta.
#[test]
fn tightness_laws_stay_private_without_new_trust() {
    let (mut env, _) = load_order();
    env.elaborate_file(
        "import Data.Numeric.Nat.Order (min, max, leq_nat, IsTrue as NatOrderIsTrue)\n\
         theorem min_public_control (m : Nat) (n : Nat)\n\
           : NatOrderIsTrue (leq_nat (min m n) m) = (proof leq_left for min) m n\n\
         theorem max_public_control (m : Nat) (n : Nat)\n\
           : NatOrderIsTrue (leq_nat m (max m n)) = (proof left_leq for max) m n",
    )
    .expect("existing public min/max bounds must remain selectable");
    for (operation, law, statement) in [
        (
            "min",
            "greatest",
            "NatOrderIsTrue (leq_nat k m) → NatOrderIsTrue (leq_nat k n) → NatOrderIsTrue (leq_nat k (min m n))",
        ),
        (
            "max",
            "least",
            "NatOrderIsTrue (leq_nat m k) → NatOrderIsTrue (leq_nat n k) → NatOrderIsTrue (leq_nat (max m n) k)",
        ),
    ] {
        let client = format!(
            "import Data.Numeric.Nat.Order ({operation}, leq_nat, IsTrue as NatOrderIsTrue)\n\
             theorem private_bound (k : Nat) (m : Nat) (n : Nat)\n\
               : {statement} =\n\
               λfirst. λsecond. (proof {law} for {operation}) k m n first second"
        );
        match env.elaborate_file(&client) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{ORDER}.{operation}::{law}"));
            }
            Err(other) => {
                panic!("private {operation}::{law} import failed for another reason: {other:?}")
            }
            Ok(_) => panic!("private {operation}::{law} was unexpectedly reachable"),
        }
    }
}
