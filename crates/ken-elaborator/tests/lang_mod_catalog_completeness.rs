//! LANG-MOD-CATALOG-COMPLETENESS partial provider-identity acceptance.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::ElabEnv;
use ken_kernel::{Decl, GlobalId, Term};

const ORD_RESULT_MODULE: &str = "Core.Logic.OrdResult";
const COMPARE_MODULE: &str = "Core.Logic.Compare";
const DERIVED_MODULE: &str = "Data.Collections.Derived";
const LAWFUL_MODULE: &str = "Core.Classes.LawfulClasses";

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
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

/// Promise class: durable invariant.
///
/// MEASURED: the real strict roots loader produces one transparent canonical
/// constructor family and aliases that point at its exact constructor IDs.
/// CLAIMED: OrdResult publication preserves identity and adds no trust. THE GAP:
/// this test does not claim that every catalog consumer is strict-ready.
#[test]
fn canonical_ord_result_is_strict_standalone_and_exports_one_identity() {
    let mut env = ElabEnv::new().expect("base environment");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_module_from_roots_strict(&[catalog_root()], ORD_RESULT_MODULE)
        .expect("canonical OrdResult must load standalone under strict resolution");

    let ty = env.globals["Core.Logic.OrdResult.OrdResult"];
    let lt = env.globals["Core.Logic.OrdResult.Lt"];
    let eq = env.globals["Core.Logic.OrdResult.Eq"];
    let gt = env.globals["Core.Logic.OrdResult.Gt"];
    assert!(matches!(env.env.lookup(ty), Some(Decl::Inductive { .. })));

    for (alias, constructor) in [("ord_eq", eq), ("ord_lt", lt), ("ord_gt", gt)] {
        let alias = env.globals[&format!("Core.Logic.OrdResult.{alias}")];
        let (_, body) = env
            .env
            .transparent_body(alias)
            .expect("OrdResult alias must remain transparent");
        assert!(
            matches!(body, Term::Constructor { id, .. } if id == constructor),
            "alias must reuse its canonical constructor identity"
        );
    }

    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "OrdResult must add zero trusted authority");
}

/// Promise class: durable invariant.
///
/// MEASURED: the real Derived roots load reaches the canonical OrdResult and
/// Compare globals, has no competing Derived identities, and reloads providers
/// idempotently. CLAIMED: the consumer closure reuses its providers. THE GAP:
/// later catalog leaves may still have unrelated unresolved dependencies.
#[test]
fn real_derived_consumer_reuses_canonical_logic_providers() {
    let mut env = ElabEnv::new().expect("base environment");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_module_from_roots(&[catalog_root()], DERIVED_MODULE)
        .expect("real Derived consumer must legacy-roots-load through OrdResult");

    let canonical = env.globals["Core.Logic.OrdResult.OrdResult"];
    assert!(matches!(
        env.env.lookup(canonical),
        Some(Decl::Inductive { .. })
    ));
    assert!(env
        .globals
        .contains_key("Core.Logic.OrdResult.ord_result_leq"));
    assert!(!env
        .globals
        .contains_key("Data.Collections.Derived.ord_result_leq"));
    assert!(!env
        .globals
        .contains_key("Data.Collections.Derived.OrdResult"));
    assert!(!env.globals.contains_key("Data.Numeric.Nat.Order.OrdResult"));

    for name in [
        "pair_compare",
        "pair_compare_result_of",
        "pair_compare_lt_cases",
        "list_compare",
        "list_eq",
    ] {
        assert!(
            env.globals
                .contains_key(&format!("Core.Logic.Compare.{name}")),
            "canonical Compare provider must own {name}"
        );
        assert!(
            !env.globals
                .contains_key(&format!("Data.Collections.Derived.{name}")),
            "Derived must not retain a competing {name} identity"
        );
    }

    let loaded_before = env.loaded_module_count();
    env.elaborate_module_from_roots(&[catalog_root()], ORD_RESULT_MODULE)
        .expect("reloading OrdResult must be idempotent");
    env.elaborate_module_from_roots(&[catalog_root()], COMPARE_MODULE)
        .expect("reloading Compare must be idempotent");
    assert_eq!(env.loaded_module_count(), loaded_before);

    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "the real consumer closure must add no trust");
}

/// Promise class: durable invariant.
///
/// MEASURED: real LawfulClasses terms use canonical comparison and its two local
/// ordinary proof globals, while all foreign attached spellings are absent.
/// CLAIMED: attachment ownership stays with the defining module and adds no
/// proof trust. THE GAP: Order remains held on the separate Nat provider.
#[test]
fn lawful_local_pair_proofs_do_not_extend_the_derived_subject_namespace() {
    let mut env = ElabEnv::new().expect("base environment");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_module_from_roots(&[catalog_root()], LAWFUL_MODULE)
        .expect("LawfulClasses must load through its real provider closure");

    let eq_sound = env.globals["Core.Classes.LawfulClasses.pair_compare_eq_sound"];
    let lt_asym = env.globals["Core.Classes.LawfulClasses.pair_compare_lt_asym"];
    let antisym = env.globals["Core.Classes.LawfulClasses.pair_ord_leq::antisym"];
    let (_, body) = env
        .env
        .transparent_body(antisym)
        .expect("pair_ord_leq::antisym must be a checked transparent proof");
    assert!(mentions_global(&body, eq_sound));
    assert!(mentions_global(&body, lt_asym));

    let canonical_pair_compare = env.globals["Core.Logic.Compare.pair_compare"];
    assert!(env
        .globals
        .contains_key("Core.Logic.Compare.pair_compare::eq"));
    assert!(env
        .globals
        .contains_key("Core.Logic.Compare.pair_compare::eq_cases"));
    let pair_ord_leq = env.globals["Core.Classes.LawfulClasses.pair_ord_leq"];
    let (_, body) = env
        .env
        .transparent_body(pair_ord_leq)
        .expect("pair_ord_leq must remain transparent");
    assert!(mentions_global(&body, canonical_pair_compare));

    for forbidden in [
        "Data.Collections.Derived.pair_compare::eq",
        "Data.Collections.Derived.pair_compare::eq_cases",
        "Data.Collections.Derived.pair_compare::eq_sound",
        "Data.Collections.Derived.pair_compare::lt_asym",
        "Core.Logic.Compare.pair_compare::eq_sound",
        "Core.Logic.Compare.pair_compare::lt_asym",
    ] {
        assert!(
            !env.globals.contains_key(forbidden),
            "consumer-local proof must not extend the provider namespace"
        );
    }

    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert!(!after.contains(&eq_sound));
    assert!(!after.contains(&lt_asym));
    assert!(
        after.is_superset(&before),
        "loading must preserve the existing trusted base"
    );
}
