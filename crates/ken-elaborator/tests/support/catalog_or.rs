use std::path::PathBuf;

use ken_elaborator::{modules::ModuleState, ElabEnv};
use ken_kernel::{Decl, Level, Term};

pub fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

pub fn load_core_logic_or(env: &mut ElabEnv) {
    env.elaborate_module_from_roots_strict(&[catalog_root()], "Core.Logic.Or")
        .expect("Core.Logic.Or must load through strict catalog resolution");
}

/// Snapshot the provider-only module state before loading a consumer's legacy
/// dependency environment. Restoring it immediately before the real consumer
/// prevents an earlier dependency's selective import from laundering a missing
/// import in the consumer under test.
pub fn core_logic_or_module_state(env: &ElabEnv) -> ModuleState {
    assert!(!env.globals.contains_key("Or"));
    assert!(!env.globals.contains_key("Inl"));
    assert!(!env.globals.contains_key("Inr"));
    env.module_state.clone()
}

pub fn restore_core_logic_or_module_state(env: &mut ElabEnv, state: &ModuleState) {
    env.module_state = state.clone();
    env.globals.remove("Or");
    env.globals.remove("Inl");
    env.globals.remove("Inr");
}

fn applied_head(term: &Term) -> &Term {
    match term {
        Term::App(function, _) => applied_head(function),
        Term::Pi(_, codomain) => applied_head(codomain),
        other => other,
    }
}

/// Promise class: durable invariant.
///
/// **MEASURED:** after a caller loads a real catalog source from `include_str!`,
/// this inspects a transparent witness resolved in that unit and compares its
/// result head with the provider's exact `GlobalId` and Omega-to-Type shape.
/// Existing consumer declarations supply the witness where one exists; the
/// otherwise-unused EmptyDec import gets a probe appended to the same real unit.
/// **CLAIMED:** the real consumer resolves its proof-relevant disjunction to the
/// one canonical provider. **THE GAP:** this helper cannot establish that the
/// caller loaded a real source, so every call sits directly after that source's
/// production elaboration path rather than behind a synthetic replacement.
pub fn assert_transparent_result_uses_core_logic_or(env: &ElabEnv, name: &str) {
    let or_id = env.globals["Core.Logic.Or.Or"];
    let or_decl = env.env.inductive(or_id).expect("canonical Or family");
    assert_eq!(or_decl.params, vec![Term::omega(Level::Zero); 2]);
    assert_eq!(or_decl.level, Level::Zero);

    let id = *env
        .globals
        .get(name)
        .unwrap_or_else(|| panic!("missing real consumer witness `{name}`"));
    let Decl::Transparent { ty, .. } = env
        .env
        .lookup(id)
        .unwrap_or_else(|| panic!("missing declaration for `{name}`"))
    else {
        panic!("real consumer witness `{name}` must be transparent");
    };
    assert!(
        matches!(applied_head(ty), Term::IndFormer { id, .. } if *id == or_id),
        "real consumer witness `{name}` must use the canonical catalog Or GlobalId"
    );
}
