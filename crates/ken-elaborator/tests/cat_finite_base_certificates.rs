//! Finite witnesses for the built-in Unit and Bool carriers.
//!
//! Contract: spec/50-stdlib/61-formal-languages.md §3.2.
//! Promise class: durable coverage and enumeration invariants. The exact list
//! order is an implementation choice; only complete coverage is prescribed.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::ElabEnv;
use ken_interp::eval::{eval, EvalStore, EvalVal};
use ken_kernel::Decl;

const FINITE: &str = "Data.Finite.Finite";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn evaluated_list(env: &ElabEnv, name: &str) -> EvalVal {
    let id = env.globals[name];
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(id) else {
        panic!("{name} must be a checked transparent list");
    };
    eval(&[], body, &env.env, &mut EvalStore::new())
}

/// MEASURED: after its actual Vector and Derived provider closure loads,
/// Finite's trust inventory stays identical; fresh clients selectively import
/// both public values and use `covers` at each constructor. CLAIMED: checked
/// coverage for Unit and both Bool values adds no assumption. THE GAP: this
/// tests the reusable certificates, not the later NFA subset construction.
#[test]
fn unit_and_bool_values_cover_their_entire_carriers_without_new_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in ["Data.Vector.Vector", "Data.Collections.Derived"] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} must load: {error:?}"));
    }
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let owned = env
        .elaborate_module_from_roots(&[root()], FINITE)
        .expect("new built-in certificates must roots-load");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(after, before, "Finite certificates add no local trust");
    for name in ["unit_finite", "bool_finite"] {
        let id = env.globals[&format!("{FINITE}.{name}")];
        assert!(owned.contains(&id), "{name} must be Finite-owned");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} must have a checked body"
        );
    }
    env.elaborate_file(
        "import Data.Finite.Finite
           (Finite, elements, covers, unit_finite, bool_finite, pair_finite)
         import Data.Collections.Derived (list_elem)
         theorem unit_covered :
           list_elem Unit MkUnit (elements Unit unit_finite) =
           covers Unit unit_finite MkUnit
         theorem true_covered :
           list_elem Bool True (elements Bool bool_finite) =
           covers Bool bool_finite True
         theorem false_covered :
           list_elem Bool False (elements Bool bool_finite) =
           covers Bool bool_finite False
         const joint : Finite (Pair Bool Unit) =
           pair_finite Bool Unit bool_finite unit_finite
         const unit_elements : List Unit = elements Unit unit_finite
         const bool_elements : List Bool = elements Bool bool_finite",
    )
    .expect("both public certificates must cover all constructor cases");

    let mut remaining = evaluated_list(&env, "unit_elements");
    let mut saw_unit = false;
    loop {
        match remaining {
            EvalVal::Ctor { id, .. } if id == env.prelude_env.nil_id => break,
            EvalVal::Ctor { id, args, .. } if id == env.prelude_env.cons_id => {
                assert!(
                    matches!(&args[1], EvalVal::Ctor { id, .. } if *id == env.globals["MkUnit"])
                );
                saw_unit = true;
                remaining = args[2].clone();
            }
            other => panic!("finite Unit list has wrong shape: {other:?}"),
        }
    }
    assert!(saw_unit, "the unique Unit inhabitant must be enumerated");

    let mut remaining = evaluated_list(&env, "bool_elements");
    let mut seen = BTreeSet::new();
    loop {
        match remaining {
            EvalVal::Ctor { id, args, .. } if id == env.prelude_env.nil_id => break,
            EvalVal::Ctor { id, args, .. } if id == env.prelude_env.cons_id => {
                let EvalVal::Ctor { id, .. } = &args[1] else {
                    panic!("finite Bool list must contain Bool constructors");
                };
                if *id == env.numeric_env.bool_true_id {
                    seen.insert(true);
                } else if *id == env.numeric_env.bool_false_id {
                    seen.insert(false);
                } else {
                    panic!("finite Bool list contained another constructor");
                }
                remaining = args[2].clone();
            }
            other => panic!("finite Bool list has wrong shape: {other:?}"),
        }
    }
    assert_eq!(seen, BTreeSet::from([false, true]));
}
