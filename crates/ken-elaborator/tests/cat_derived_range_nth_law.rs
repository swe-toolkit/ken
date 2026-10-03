//! The private range contents law is checked over the real Derived owner.

use std::collections::BTreeSet;

use ken_elaborator::ElabEnv;
use ken_kernel::{Decl, Term};

#[path = "support/catalog_or.rs"]
mod catalog_or;

const DERIVED: &str = "Data.Collections.Derived";

fn applied(head: Term, args: impl IntoIterator<Item = Term>) -> Term {
    args.into_iter().fold(head, Term::app)
}

/// Promise class: durable checked proposition invariant.
///
/// MEASURED: a roots-loaded, owner-owned transparent proof has the complete
/// raw Π proposition, including the in-bounds hypothesis, the `nth`/`range`
/// provider identities and both indexed `Equal` endpoints. Loading Derived
/// adds no trust beyond its direct providers. CLAIMED: the private theorem
/// proves that every in-bounds element of `range n` equals its index, with
/// no added assumption. THE GAP: the raw proposition and kernel-checked
/// transparency pin this exact statement, not evaluation costs or export.
#[test]
fn range_nth_has_the_exact_raw_checked_proposition() {
    let mut env = ElabEnv::new().expect("cold elaborator");
    let roots = &[catalog_or::catalog_root()];
    for provider in [
        "Core.Function.Combinators",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
        "Core.Logic.Compare",
        "Core.Classes.LawfulClasses",
        "Core.Logic.Or",
        "Core.Logic.OrdResult",
        "Core.Logic.Transport",
    ] {
        env.elaborate_module_from_roots(roots, provider)
            .unwrap_or_else(|error| panic!("direct provider {provider}: {error:?}"));
    }
    let trust_before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let owned = env
        .elaborate_module_from_roots(roots, DERIVED)
        .expect("Derived must roots-load with the checked range law");
    let trust_after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(trust_after, trust_before, "Derived must add no trust");

    let law_id = env.globals[&format!("{DERIVED}.range_nth")];
    assert!(owned.contains(&law_id), "the law must be owned by Derived");
    let Decl::Transparent { ty, .. } = env.env.lookup(law_id).expect("range_nth must load") else {
        panic!("range_nth must be kernel-checked, not an assumption");
    };

    let global = |name: &str| Term::const_(env.globals[name], vec![]);
    let ctor = |name: &str| Term::constructor(env.globals[name], vec![]);
    let nat = Term::indformer(env.globals["Nat"], vec![]);
    let option_nat = applied(
        Term::indformer(env.globals["Option"], vec![]),
        [nat.clone()],
    );
    let range = env.globals[&format!("{DERIVED}.range")];
    let nth = env.globals[&format!("{DERIVED}.nth")];
    for id in [range, nth] {
        assert!(owned.contains(&id), "range and nth must be Derived-owned");
    }

    // The bound is formed under n, i; the endpoints are under n, i, below.
    let below = applied(
        global("Core.Classes.LawfulClasses.IsTrue"),
        [applied(
            global("Core.Classes.LawfulClasses.leq_nat"),
            [applied(ctor("Suc"), [Term::var(0)]), Term::var(1)],
        )],
    );
    let lhs = applied(
        Term::const_(nth, vec![]),
        [
            nat.clone(),
            Term::var(1),
            applied(Term::const_(range, vec![]), [Term::var(2)]),
        ],
    );
    let rhs = applied(ctor("Some"), [nat.clone(), Term::var(1)]);
    let equality = applied(global("Equal"), [option_nat, lhs, rhs]);
    let expected = Term::pi(nat.clone(), Term::pi(nat, Term::pi(below, equality)));
    assert_eq!(ty, &expected, "range_nth's raw checked proposition changed");
}
