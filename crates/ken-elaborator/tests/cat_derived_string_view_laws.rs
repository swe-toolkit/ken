//! Checked owner-local consumers for the five private Derived String and List laws.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, Term};

const DERIVED: &str = "Data.Collections.Derived";

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn owned_id(env: &ElabEnv, owned: &[GlobalId], name: &str) -> GlobalId {
    let qualified = format!("{DERIVED}.{name}");
    let id = *env
        .globals
        .get(&qualified)
        .unwrap_or_else(|| panic!("missing Derived law {qualified}"));
    assert!(owned.contains(&id), "{qualified} must be loader-owned");
    id
}

fn references(term: &Term, id: GlobalId) -> usize {
    usize::from(matches!(term, Term::Const { id: found, .. } if *found == id))
        + term
            .children()
            .into_iter()
            .map(|child| references(child, id))
            .sum::<usize>()
}

fn owner_bindings(env: &ElabEnv) -> BTreeMap<String, GlobalId> {
    let prefix = format!("{DERIVED}.");
    env.globals
        .iter()
        .filter(|(name, _)| name.starts_with(&prefix))
        .map(|(name, id)| (name.clone(), *id))
        .collect()
}

/// Promise class: durable checked-proof invariant.
///
/// MEASURED: real roots loading adds no trust beyond the direct providers;
/// each private theorem is transparent and its generic owner example applies
/// its loader-owned identity in a checked proof body. A fresh client refuses
/// every private import. Checked example/reject fences preserve the owner's
/// qualified binding map and trust. CLAIMED: five private laws admit generic
/// typed use without a new assumption or export. THE GAP: a reference can be
/// present yet irrelevant to the stated equation; compile-preserving count-law
/// filler mutations must fail the unchanged generic owner consumers.
#[test]
fn derived_private_view_laws_check_generic_owner_uses_without_trust() {
    let examples = [
        ("derived_example_length_drop_generic", "length_drop"),
        (
            "derived_example_concat_char_count_generic",
            "concat_char_count",
        ),
        (
            "derived_example_slice_char_count_generic",
            "slice_char_count",
        ),
        ("derived_example_concat_view_generic", "concat_view"),
        ("derived_example_slice_view_generic", "slice_view"),
    ];
    let mut env = ElabEnv::new().expect("cold elaborator");
    let roots = &[catalog_root()];
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
    let provider_trust: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let owned = env
        .elaborate_module_from_roots(roots, DERIVED)
        .expect("Derived must roots-load over its direct providers");
    let loaded_trust: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(loaded_trust, provider_trust, "Derived must add no trust");

    let bindings_before = owner_bindings(&env);
    for (example, law) in examples {
        let id = owned_id(&env, &owned, law);
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{law} must be a checked transparent theorem"
        );
        assert!(
            !env.globals.contains_key(example)
                && !bindings_before.contains_key(&format!("{DERIVED}.{example}")),
            "{example} must not be tangled into the owner"
        );
    }
    env.execute_loaded_entry_checked_fences(DERIVED)
        .expect("five generic owner examples and the paired rejects must check");
    assert_eq!(owner_bindings(&env), bindings_before);
    assert_eq!(
        env.env.trusted_base().into_iter().collect::<BTreeSet<_>>(),
        loaded_trust,
        "checked examples must not add trust"
    );
    let mut outside = ElabEnv::new().expect("fresh client elaborator");
    outside
        .elaborate_module_from_roots(roots, DERIVED)
        .expect("Derived provider must load for an unrelated client");
    for (_, law) in examples {
        match outside.elaborate_file(&format!("import {DERIVED} ({law})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{DERIVED}.{law}"));
            }
            Err(other) => panic!("private {law} must refuse as UnboundName: {other:?}"),
            Ok(_) => panic!("private {law} must not be importable"),
        }
    }

    for (example, law) in examples {
        let law_id = owned_id(&env, &owned, law);
        let example_id = *env
            .globals
            .get(example)
            .unwrap_or_else(|| panic!("{example} must elaborate in the owner fence"));
        assert!(
            !owned.contains(&example_id),
            "{example} must not be a shipped declaration"
        );
        let Some(Decl::Transparent { body, .. }) = env.env.lookup(example_id) else {
            panic!("{example} must be a transparent checked theorem");
        };
        assert!(
            references(body, law_id) > 0,
            "{example} must use the loader-owned {law} in its proof"
        );
    }
}
