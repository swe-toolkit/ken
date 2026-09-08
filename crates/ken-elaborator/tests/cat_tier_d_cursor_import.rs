//! Tier-D DC + Doc -> Cursor publication and strict-import controls.

#[path = "support/catalog_or.rs"]
mod catalog_or;
#[path = "support/catalog_publication.rs"]
mod catalog_publication;

use std::collections::BTreeSet;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::GlobalId;

const DIAGNOSTICS_CORE: &str = "Capability.Diagnostics.Core";
const DIAGNOSTICS_CORE_SOURCE: &str =
    include_str!("../../../catalog/packages/Capability/Diagnostics/Core.ken.md");

fn names(items: &[&str]) -> BTreeSet<String> {
    items.iter().map(|item| (*item).to_owned()).collect()
}

fn provider_modules(module: &str) -> &'static [&'static str] {
    match module {
        DIAGNOSTICS_CORE => &["Core.Classes.LawfulClasses"],
        _ => &[],
    }
}

fn load_module(module: &str) -> (ElabEnv, BTreeSet<GlobalId>) {
    let mut env = ElabEnv::new().expect("base environment");
    for provider in provider_modules(module) {
        env.elaborate_module_from_roots(&[catalog_or::catalog_root()], provider)
            .unwrap_or_else(|error| panic!("{module} provider {provider} must load: {error:?}"));
    }
    let before_trust = env.env.trusted_base();
    let before_classes = env.class_env.class_entries().count();
    let before_instances = env.class_env.instances.len();
    let owned = env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], module)
        .unwrap_or_else(|error| panic!("{module} must roots-load standalone: {error:?}"))
        .into_iter()
        .collect();
    assert_eq!(
        env.env.trusted_base(),
        before_trust,
        "{module} publication and imports must add no trust"
    );
    assert_eq!(
        env.class_env.class_entries().count(),
        before_classes,
        "{module} must not mint a class"
    );
    assert_eq!(
        env.class_env.instances.len(),
        before_instances,
        "{module} must not mint an instance"
    );
    (env, owned)
}

fn assert_selective_identities(module: &str, surfaces: &BTreeSet<String>) {
    let (mut env, _) = load_module(module);
    let canonical_before = surfaces
        .iter()
        .map(|surface| env.globals[&format!("{module}.{surface}")])
        .collect::<Vec<_>>();
    let selections = surfaces
        .iter()
        .enumerate()
        .map(|(index, surface)| format!("{surface} as cat_tier_d_selected_{index}"))
        .collect::<Vec<_>>()
        .join(", ");
    env.elaborate_file(&format!("import {module} ({selections})"))
        .unwrap_or_else(|error| panic!("{module} public surface must import together: {error:?}"));
    for (surface, before) in surfaces.iter().zip(canonical_before) {
        assert_eq!(
            env.globals[&format!("{module}.{surface}")],
            before,
            "selective import must not remint {module}.{surface}"
        );
    }
}

fn assert_private(module: &str, surface: &str) {
    let (mut env, _) = load_module(module);
    match env.elaborate_file(&format!("import {module} ({surface})")) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{module}.{surface}"))
        }
        other => panic!("{module}.{surface} must stay private, got {other:?}"),
    }
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: the roots loader queries every publishable Diagnostics.Core
/// declaration and constructor, and the successful surface equals the exact
/// downstream union measured before migration. CLAIMED: publication exposes
/// precisely the diagnostic carrier API its clients consume. THE GAP: generated
/// dictionaries are outside the query population, but this module declares no
/// class or instance and `load_module` pins that fact.
#[test]
fn diagnostics_core_loader_visible_inventory_is_exact() {
    let expected = names(&[
        "ArgumentOrigin",
        "ByteRange",
        "ConfigKeyOrigin",
        "Diagnostic",
        "DiagnosticCode",
        "EnvironmentOrigin",
        "MkByteRange",
        "MkDiagnostic",
        "MkDiagnosticCode",
        "Origin",
        "SourceId",
        "SourceOrigin",
        "byte_range_end",
        "byte_range_start",
        "diagnostic_code",
        "diagnostic_origin",
        "origin_argument_index",
        "origin_range_end",
        "origin_range_start",
        "origin_source_id",
    ]);
    assert_eq!(
        catalog_publication::published_module_surfaces(
            DIAGNOSTICS_CORE_SOURCE,
            DIAGNOSTICS_CORE,
            "diagnostics_core",
        ),
        expected
    );
    assert_selective_identities(DIAGNOSTICS_CORE, &expected);
}

/// Promise class: durable invariant.
///
/// MEASURED: Diagnostics.Core roots-loads with zero trust/class/instance growth,
/// while an unexported neighboring constructor remains unavailable to a real
/// selective-import client. CLAIMED: the already-standalone module's increment
/// changes visibility only and retains its private owner face. THE GAP: exact
/// byte preservation of published bodies is verified by the one-shot git
/// differential rather than frozen into a source-text test.
#[test]
fn diagnostics_core_publication_is_visibility_only() {
    let _ = load_module(DIAGNOSTICS_CORE);
    assert_private(DIAGNOSTICS_CORE, "MkSourceId");
    assert_private(DIAGNOSTICS_CORE, "environment_origin");
}
