//! Acceptance pins for `LANG-MOD-OR-CANONICAL-HOME`, implementing the module
//! identity and flat-Sigma rules in `spec/30-surface/33-declarations.md` §3-4.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::ElabEnv;
use ken_kernel::{Level, Term};

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

/// Promise class: normative compatibility vector. The canonical catalog
/// declaration retains the former prelude's exact parameter, result, and
/// constructor-field shape while receiving a newly allocated catalog identity.
///
/// **MEASURED:** the real strict roots loader admits `Core.Logic.Or`, exposes
/// its three public identities, and emits the exact kernel inductive telescope.
/// **CLAIMED:** the catalog family is the field-for-field proof-relevant
/// replacement for the retired prelude family. **THE GAP:** matching names or
/// declaration counts would not establish the sort and field identity; the
/// assertions inspect the emitted `InductiveDecl` by its fresh `GlobalId`.
#[test]
fn core_logic_or_loads_standalone_with_exact_relevant_shape() {
    let mut env = ElabEnv::new().expect("base environment");
    assert!(!env.globals.contains_key("Or"));
    assert!(!env.globals.contains_key("Inl"));
    assert!(!env.globals.contains_key("Inr"));
    let pre_catalog_decls = env.env.decls().count();
    let trusted_before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();

    env.elaborate_module_from_roots_strict(&[catalog_root()], "Core.Logic.Or")
        .expect("Core.Logic.Or must load standalone through strict roots");

    let or_id = env.globals["Core.Logic.Or.Or"];
    let inl_id = env.globals["Core.Logic.Or.Inl"];
    let inr_id = env.globals["Core.Logic.Or.Inr"];
    assert!(or_id.0 as usize >= pre_catalog_decls);
    let or_decl = env
        .env
        .inductive(or_id)
        .expect("catalog Or must be an inductive family");
    assert_eq!(or_decl.params, vec![Term::omega(Level::Zero); 2]);
    assert!(or_decl.indices.is_empty());
    assert_eq!(or_decl.level, Level::Zero);
    assert_eq!(or_decl.constructors.len(), 2);
    assert_eq!(or_decl.constructors[0].id, inl_id);
    assert_eq!(or_decl.constructors[0].args, vec![Term::var(1)]);
    assert_eq!(or_decl.constructors[1].id, inr_id);
    assert_eq!(or_decl.constructors[1].args, vec![Term::var(0)]);
    assert_eq!(
        trusted_before,
        env.env.trusted_base().into_iter().collect(),
        "ordinary catalog data and exports add no trusted declaration",
    );
}
