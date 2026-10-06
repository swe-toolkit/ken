//! The single checked List length identity has a trust-free base provider.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::ElabEnv;
use ken_kernel::{Decl, GlobalId, Term};

const BASE: &str = "Data.Collections.List";
const DERIVED: &str = "Data.Collections.Derived";

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn mentions_const(term: &Term, provider: GlobalId) -> bool {
    matches!(term, Term::Const { id, .. } if *id == provider)
        || term
            .children()
            .into_iter()
            .any(|child| mentions_const(child, provider))
}

/// Promise class: durable invariant. MEASURED: an independently fresh compiler
/// base and a cold roots-loaded List provider have identical trusted-ID sets;
/// Derived's selective re-export and a direct base import produce two checked
/// clients that cite exactly the same transparent provider GlobalId. CLAIMED:
/// clients can use one canonical length without loading Derived's five inherited
/// assumptions. THE GAP: the tested clients cover selective imports and their
/// checked bodies; other packages' uses are checked by their own loader suites.
#[test]
fn cold_list_length_and_derived_reexport_share_the_checked_provider_identity() {
    let independent = ElabEnv::new().expect("independent compiler base");
    let before: BTreeSet<_> = independent.env.trusted_base().into_iter().collect();
    let mut env = ElabEnv::new().expect("cold List provider env");
    let base_owned = env
        .elaborate_module_from_roots(&[catalog_root()], BASE)
        .expect("trust-free base List must load alone");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        after, before,
        "base List length must inherit only compiler trust"
    );
    let length = env.globals["Data.Collections.List.length"];
    assert!(
        base_owned.contains(&length),
        "the base module must own length"
    );
    assert!(
        matches!(env.env.lookup(length), Some(Decl::Transparent { .. })),
        "List length must be a checked, transparent fold"
    );
    assert!(!after.contains(&length), "List length must not be trusted");

    let derived_owned = env
        .elaborate_module_from_roots(&[catalog_root()], DERIVED)
        .expect("Derived must import and re-export the base definition");
    assert!(
        !derived_owned.contains(&length),
        "Derived must not mint a second length identity"
    );
    env.elaborate_file(
        "import Data.Collections.List (length as direct_length)\n\
         import Data.Collections.Derived (length as derived_length)\n\
         fn length_from_base (xs : List Bool) : Nat = direct_length Bool xs\n\
         fn length_from_derived (xs : List Bool) : Nat = derived_length Bool xs",
    )
    .expect("both provider and re-export must be importable in one client");
    for name in ["length_from_base", "length_from_derived"] {
        let client = env.globals[name];
        let (_, body) = env
            .env
            .transparent_body(client)
            .expect("length client must be checked and transparent");
        assert!(
            mentions_const(&body, length),
            "{name} must cite the exact base-provider GlobalId"
        );
    }
    assert!(
        !env.globals.contains_key("Data.Collections.Derived.length"),
        "a re-export must not masquerade as a locally owned global"
    );
}
