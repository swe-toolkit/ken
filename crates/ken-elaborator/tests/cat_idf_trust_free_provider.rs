//! CAT-IDF-TRUST-FREE-PROVIDER: cold, non-preloaded trust closure.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

/// Promise class: durable invariant. A fresh compiler environment and a
/// separate fresh roots-loaded Vector environment have identical trusted-base
/// sets, not merely identical counts or a zero delta over preloaded providers.
/// MEASURED: every trusted identity in the cold Vector closure against the
/// independent compiler floor. CLAIMED: importing Vector adds no trust.
/// THE GAP: the kernel's trusted_base API is the authoritative population;
/// no hand-maintained name inventory is used as a proxy for that population.
#[test]
fn cold_vector_trust_set_equals_separately_fresh_compiler_base() {
    let compiler_base = ElabEnv::new().expect("independent compiler base");
    let base: BTreeSet<_> = compiler_base.env.trusted_base().into_iter().collect();

    let mut vector = ElabEnv::new().expect("fresh cold Vector environment");
    vector
        .elaborate_module_from_roots(&[catalog_root()], "Data.Vector.Vector")
        .expect("cold Vector roots load and all checked fences");
    let loaded: BTreeSet<_> = vector.env.trusted_base().into_iter().collect();
    let added: Vec<_> = loaded.difference(&base).copied().collect();
    let removed: Vec<_> = base.difference(&loaded).copied().collect();
    eprintln!(
        "cold Vector trust: compiler {}, loaded {}, added {added:?}, removed {removed:?}",
        base.len(), loaded.len()
    );
    assert_eq!(
        loaded, base,
        "cold Vector closure differs from compiler base (base {}, loaded {}, added {added:?}, removed {removed:?})",
        base.len(), loaded.len()
    );
}

/// Promise class: durable invariant. The one published identity and
/// composition pair is checked in a provider whose cold closure is trust-free.
/// MEASURED: roots-loaded provider trust set, its transparent definitions,
/// successful selective import and the absence of the old LF publication.
/// CLAIMED: consumers have one canonical source for these functions.
/// THE GAP: the consumer sweep tests each checked call site separately.
#[test]
fn combinators_are_trust_free_canonical_checked_imports() {
    let compiler_base = ElabEnv::new().expect("independent compiler base");
    let base: BTreeSet<_> = compiler_base.env.trusted_base().into_iter().collect();
    let mut env = ElabEnv::new().expect("fresh provider environment");
    env.elaborate_module_from_roots(&[catalog_root()], "Core.Function.Combinators")
        .expect("new provider must roots-load");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(after, base, "cold function provider must add no trust");
    for name in ["idf", "comp"] {
        let id = env.globals[&format!("Core.Function.Combinators.{name}")];
        assert!(
            env.env.transparent_body(id).is_some(),
            "{name} must be checked"
        );
    }
    env.elaborate_module_from_roots(&[catalog_root()], "Core.Classes.LawfulFunctors")
        .expect("class laws must load with canonical combinators");
    for name in ["idf", "comp"] {
        assert!(
            !env.globals
                .contains_key(&format!("Core.Classes.LawfulFunctors.{name}")),
            "old class provider must not publish a second {name}"
        );
    }
    env.elaborate_file("import Core.Function.Combinators (idf, comp)")
        .expect("both canonical definitions must import directly");
    for name in ["idf", "comp"] {
        match env.elaborate_file(&format!("import Core.Classes.LawfulFunctors ({name})")) {
            Err(ElabError::UnboundName { name: rejected, .. }) => {
                assert_eq!(rejected, format!("Core.Classes.LawfulFunctors.{name}"));
            }
            other => panic!("old {name} provider must reject selective imports: {other:?}"),
        }
    }
}
