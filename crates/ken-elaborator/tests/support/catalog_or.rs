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

pub fn load_core_logic_compare(env: &mut ElabEnv) {
    env.elaborate_module_from_roots(&[catalog_root()], "Core.Logic.Compare")
        .expect("the Core.Logic comparison provider closure must load");

    // Older acceptance fixtures elaborate a catalog consumer as a flat source
    // and then append declarations in the same synthetic scope. Preserve that
    // harness shape while reusing the real providers' exact GlobalIds.
    for (module, names) in [
        (
            "Core.Logic.OrdResult",
            &[
                "OrdResult",
                "Lt",
                "Gt",
                "ord_eq",
                "ord_lt",
                "ord_gt",
                "ord_result_leq",
                "ord_result_dispatch2",
                "ord_result_elim",
                "ord_result_elim2",
            ][..],
        ),
        (
            "Core.Logic.Compare",
            &[
                "pair_compare",
                "pair_compare_result_of",
                "pair_compare_lt_cases",
                "list_eq",
                "list_compare",
            ][..],
        ),
    ] {
        for name in names {
            let id = env.globals[&format!("{module}.{name}")];
            env.globals.insert((*name).to_owned(), id);
        }
    }
}

pub fn expose_module(env: &mut ElabEnv, module: &str) {
    let prefix = format!("{module}.");
    let aliases: Vec<_> = env
        .globals
        .iter()
        .filter_map(|(name, id)| {
            name.strip_prefix(&prefix)
                .map(|suffix| (suffix.to_owned(), *id))
        })
        .collect();
    env.globals.extend(aliases);
}

pub fn load_derived_fixture(env: &mut ElabEnv) {
    env.elaborate_module_from_roots(&[catalog_root()], "Core.Classes.LawfulClasses")
        .expect("Derived's canonical Nat-order dependency must roots-load");
    let provider_state = env.module_state.clone();
    env.elaborate_module_from_roots(&[catalog_root()], "Data.Collections.Derived")
        .expect("Data.Collections.Derived must load through its real provider closure");

    // These legacy fixture suites append declarations in a synthetic flat
    // scope. Bind each real module declaration's exact GlobalId under its old
    // fixture spelling; no duplicate catalog declaration is elaborated. The
    // provider state preserves the pre-D6 class-owner context for later raw
    // instance fixtures while Derived itself imports the exact same identities.
    expose_module(env, "Core.Classes.LawfulClasses");
    expose_module(env, "Data.Collections.Derived");
    env.module_state = provider_state;
}

/// Assert at a measured legacy-fixture boundary that the shared Derived loader
/// retained its canonical class owner. Re-loading the provider must be a no-op:
/// if `load_derived_fixture` restores a state from before LawfulClasses, the
/// attempted reload reaches the duplicate-instance failure this control guards.
pub fn assert_derived_fixture_retains_lawfulclasses(env: &mut ElabEnv) {
    let loaded_before = env.loaded_module_count();
    env.elaborate_module_from_roots(&[catalog_root()], "Core.Classes.LawfulClasses")
        .expect("the shared Derived fixture must retain its canonical class owner");
    assert_eq!(
        env.loaded_module_count(),
        loaded_before,
        "LawfulClasses must already be loaded at the legacy-fixture boundary"
    );

    let provider = env.globals["Core.Classes.LawfulClasses.leq_nat"];
    assert_eq!(
        env.globals["leq_nat"], provider,
        "the retained class owner must preserve the canonical provider identity"
    );
}

pub fn expose_core_logic_transport(env: &mut ElabEnv) {
    for name in ["cong", "sym", "trans"] {
        let id = env.globals[&format!("Core.Logic.Transport.{name}")];
        env.globals.insert(name.to_owned(), id);
    }
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
