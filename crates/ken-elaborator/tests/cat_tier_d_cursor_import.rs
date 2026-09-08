//! Tier-D DC + Doc -> Cursor publication and strict-import controls.

#[path = "support/catalog_or.rs"]
mod catalog_or;
#[path = "support/catalog_publication.rs"]
mod catalog_publication;

use std::collections::BTreeSet;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl as KernelDecl, GlobalId, Term};

const DIAGNOSTICS_CORE: &str = "Capability.Diagnostics.Core";
const DIAGNOSTICS_CORE_SOURCE: &str =
    include_str!("../../../catalog/packages/Capability/Diagnostics/Core.ken.md");
const FORMATTING_DOC: &str = "Capability.Formatting.Doc";
const FORMATTING_DOC_SOURCE: &str =
    include_str!("../../../catalog/packages/Capability/Formatting/Doc.ken.md");

fn names(items: &[&str]) -> BTreeSet<String> {
    items.iter().map(|item| (*item).to_owned()).collect()
}

fn provider_modules(module: &str) -> &'static [&'static str] {
    match module {
        DIAGNOSTICS_CORE => &["Core.Classes.LawfulClasses"],
        FORMATTING_DOC => &[
            "Core.Classes.LawfulClasses",
            "Core.Logic.Or",
            "Core.Logic.Transport",
            "Data.Collections.Derived",
            "Data.Numeric.Nat.Arithmetic",
        ],
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

fn term_mentions(term: &Term, target: GlobalId) -> bool {
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
            .any(|child| term_mentions(child, target)),
    }
}

fn assert_providers_consumed(module: &str, providers: &[&str]) {
    let (env, owned) = load_module(module);
    for provider in providers {
        let provider_id = env.globals[*provider];
        let consumers = owned
            .iter()
            .filter(|id| match env.env.lookup(**id) {
                Some(KernelDecl::Transparent { ty, body, .. }) => {
                    term_mentions(ty, provider_id) || term_mentions(body, provider_id)
                }
                _ => false,
            })
            .count();
        assert!(
            consumers > 0,
            "{module} must consume the canonical provider {provider}"
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

/// Promise class: normative compatibility vector.
///
/// MEASURED: every publishable Formatting.Doc declaration, constructor, and
/// attached proof is queried through the roots loader; exactly the six-name
/// downstream surface resolves in one selective client. CLAIMED: Doc publishes
/// only its current carrier/construction boundary. THE GAP: declaration queries
/// do not show provider use, which the sibling identity test covers.
#[test]
fn formatting_doc_loader_visible_inventory_is_exact() {
    let expected = names(&["Concat", "Doc", "Group", "Line", "Text", "text_string"]);
    assert_eq!(
        catalog_publication::published_module_surfaces(
            FORMATTING_DOC_SOURCE,
            FORMATTING_DOC,
            "formatting_doc",
        ),
        expected
    );
    assert_selective_identities(FORMATTING_DOC, &expected);
}

/// Promise class: durable invariant.
///
/// MEASURED: standalone roots loading preserves trust and class populations,
/// every declared lower-tier provider is mentioned by a checked Doc definition,
/// and private constructors/functions remain unimportable. CLAIMED: Doc's five
/// imports replace ambient value resolution without widening its public API.
/// THE GAP: import necessity is established by the per-item population-side
/// removal campaign rather than this positive structural observation.
#[test]
fn formatting_doc_imports_are_canonical_and_visibility_only() {
    assert_providers_consumed(
        FORMATTING_DOC,
        &[
            "Core.Classes.LawfulClasses.leq_nat",
            "Core.Logic.Or.Inl",
            "Core.Logic.Or.Inr",
            "Core.Logic.Or.Or",
            "Core.Logic.Transport.cong",
            "Core.Logic.Transport.sym",
            "Core.Logic.Transport.trans",
            "Data.Collections.Derived.length",
            "Data.Collections.Derived.list_append",
            "Data.Numeric.Nat.Arithmetic.add",
        ],
    );
    assert_private(FORMATTING_DOC, "Nest");
    assert_private(FORMATTING_DOC, "doc_content");
}
