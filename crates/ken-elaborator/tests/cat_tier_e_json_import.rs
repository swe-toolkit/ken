//! Tier-E Json publication and strict-import controls.

#[path = "support/catalog_or.rs"]
mod catalog_or;
#[path = "support/catalog_publication.rs"]
mod catalog_publication;

use std::collections::{BTreeMap, BTreeSet};

use ken_elaborator::{Decl as SurfaceDecl, ElabEnv, ElabError, ImportKind, literate, parser};
use ken_kernel::{Decl, GlobalId, Term};

const JSON: &str = "Data.Serialization.Json";
const JSON_SOURCE: &str = include_str!("../../../catalog/packages/Data/Serialization/Json.ken.md");

fn names(items: &[&str]) -> BTreeSet<String> {
    items.iter().map(|item| (*item).to_owned()).collect()
}

fn public_surface() -> BTreeSet<String> {
    names(&[
        "Json",
        "JsonNull",
        "JsonBool",
        "JsonNumber",
        "JsonString",
        "JsonArray",
        "JsonObject",
        "char_cursor_ops",
        "char_cursor_peek_has_remaining",
        "char_cursor_advance_progress",
        "char_cursor_end_valid",
        "char_cursor_laws",
    ])
}

fn load_json() -> (ElabEnv, BTreeSet<GlobalId>, BTreeSet<GlobalId>) {
    let mut env = ElabEnv::new().expect("base environment");
    let mut base_ids = env.globals.values().copied().collect::<BTreeSet<_>>();
    base_ids.extend(env.env.declarations().iter().map(Decl::id));
    for provider in ["Data.Collections.Derived", "Capability.Parsing.Cursor"] {
        env.elaborate_module_from_roots(&[catalog_or::catalog_root()], provider)
            .unwrap_or_else(|error| panic!("Json provider {provider} must roots-load: {error:?}"));
    }
    let before_trust = env.env.trusted_base();
    let before_classes = env.class_env.class_entries().count();
    let before_instances = env.class_env.instances.len();
    let owned = env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], JSON)
        .expect("Data.Serialization.Json must roots-load through declared providers")
        .into_iter()
        .collect();
    assert_eq!(
        env.env.trusted_base(),
        before_trust,
        "Json imports and publication must add no trust"
    );
    assert_eq!(
        env.class_env.class_entries().count(),
        before_classes,
        "Json must not mint a class"
    );
    assert_eq!(
        env.class_env.instances.len(),
        before_instances,
        "Json must not mint an instance"
    );
    (env, owned, base_ids)
}

fn collect_term_globals(term: &Term, out: &mut BTreeSet<GlobalId>) {
    match term {
        Term::Const { id, .. } | Term::IndFormer { id, .. } | Term::Constructor { id, .. } => {
            out.insert(*id);
        }
        Term::Elim { fam, .. } => {
            out.insert(*fam);
        }
        _ => {}
    }
    for child in term.children() {
        collect_term_globals(child, out);
    }
}

fn collect_decl_globals(declaration: &Decl, out: &mut BTreeSet<GlobalId>) {
    match declaration {
        Decl::Transparent { ty, body, .. } => {
            collect_term_globals(ty, out);
            collect_term_globals(body, out);
        }
        Decl::Opaque { ty, .. } | Decl::Primitive { ty, .. } => {
            collect_term_globals(ty, out);
        }
        Decl::Inductive(inductive) => {
            for term in &inductive.params {
                collect_term_globals(term, out);
            }
            for term in &inductive.indices {
                collect_term_globals(term, out);
            }
            collect_term_globals(&inductive.former_type, out);
            for constructor in &inductive.constructors {
                for term in &constructor.args {
                    collect_term_globals(term, out);
                }
                for term in &constructor.target_indices {
                    collect_term_globals(term, out);
                }
                collect_term_globals(&constructor.type_, out);
            }
        }
    }
}

fn assert_private(surface: &str) {
    let (mut env, _, _) = load_json();
    match env.elaborate_file(&format!("import {JSON} ({surface})")) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{JSON}.{surface}"));
        }
        other => panic!("{JSON}.{surface} must stay private, got {other:?}"),
    }
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: the real Ken parser reads Json's module dependency interface and
/// returns exactly the two D0-ledger module/name sets. CLAIMED: Json adopts the
/// measured selective imports without an unused addition or ambient residue.
/// THE GAP: AST equality establishes the declared dependency interface but not
/// that each name reaches checked code, which the provider-identity test covers.
#[test]
fn json_selective_import_ledger_is_exact() {
    let extracted = literate::extract_ken_md(JSON_SOURCE).expect("Json literate extraction");
    let declarations = parser::parse_decls(&extracted.source).expect("Json source must parse");
    let actual = declarations
        .iter()
        .filter_map(|declaration| match declaration.unwrap_pub() {
            SurfaceDecl::ImportDecl {
                module,
                kind: ImportKind::Selective(items),
                ..
            } => Some((
                module.clone(),
                items
                    .iter()
                    .map(|item| item.name.clone())
                    .collect::<BTreeSet<_>>(),
            )),
            SurfaceDecl::ImportDecl { .. } => {
                panic!("Json dependency imports must all be selective")
            }
            _ => None,
        })
        .collect::<BTreeMap<_, _>>();
    let expected = BTreeMap::from([
        (
            "Capability.Parsing.Cursor".to_string(),
            names(&[
                "CursorOps",
                "MkCursorOps",
                "cursor_nat_lt",
                "CursorPeekHasRemaining",
                "CursorAdvanceProgress",
                "CursorEndValid",
                "CursorLaws",
            ]),
        ),
        ("Data.Collections.Derived".to_string(), names(&["length"])),
    ]);
    assert_eq!(actual, expected);
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: the roots loader queries every declaration and constructor and a
/// real selective-import client resolves exactly the twelve-name package API to
/// the canonical Json identities. CLAIMED: Json publishes precisely its usable
/// carrier, dictionary, and law-witness surface. THE GAP: declaration queries
/// do not prove provider provenance, which the sibling checked-core identity
/// test measures independently.
#[test]
fn json_loader_visible_inventory_is_exact() {
    let expected = public_surface();
    assert_eq!(
        catalog_publication::published_module_surfaces(JSON_SOURCE, JSON, "json"),
        expected
    );

    let (mut env, _, _) = load_json();
    let canonical = expected
        .iter()
        .map(|surface| (surface.clone(), env.globals[&format!("{JSON}.{surface}")]))
        .collect::<BTreeMap<_, _>>();
    let selections = expected
        .iter()
        .enumerate()
        .map(|(index, surface)| format!("{surface} as cat_tier_e_json_{index}"))
        .collect::<Vec<_>>()
        .join(", ");
    env.elaborate_file(&format!("import {JSON} ({selections})"))
        .expect("the complete Json public surface must import together");
    for surface in &expected {
        assert_eq!(
            env.globals[&format!("{JSON}.{surface}")],
            canonical[surface],
            "selective import must preserve {JSON}.{surface}'s canonical identity"
        );
    }
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: every non-base identity in Json's checked declarations is exactly
/// one of the D0-measured Derived or Cursor identities, including the three
/// Cursor selectors retained inside normalized law types. CLAIMED: Json has no
/// undeclared provider or unexpected Tier-E edge. THE GAP: checked-core identity
/// closure cannot distinguish an unused extra source import, while the strict
/// roots loader and the evidence-frontier parsed-edge census cover that residue.
#[test]
fn json_checked_provider_identity_closure_is_exact() {
    let (env, owned, base_ids) = load_json();
    let expected_names = names(&[
        "Capability.Parsing.Cursor.CursorAdvanceProgress",
        "Capability.Parsing.Cursor.CursorEndValid",
        "Capability.Parsing.Cursor.CursorLaws",
        "Capability.Parsing.Cursor.CursorOps",
        "Capability.Parsing.Cursor.CursorPeekHasRemaining",
        "Capability.Parsing.Cursor.MkCursorOps",
        "Capability.Parsing.Cursor.cursor_advance",
        "Capability.Parsing.Cursor.cursor_nat_lt",
        "Capability.Parsing.Cursor.cursor_peek",
        "Capability.Parsing.Cursor.cursor_remaining",
        "Data.Collections.Derived.length",
    ]);
    let expected_ids = expected_names
        .iter()
        .map(|name| env.globals[name])
        .collect::<BTreeSet<_>>();

    let mut resolved = BTreeSet::new();
    for identity in &owned {
        collect_decl_globals(
            env.env
                .lookup(*identity)
                .unwrap_or_else(|| panic!("missing owned Json identity {identity:?}")),
            &mut resolved,
        );
    }
    let external = resolved
        .difference(&owned)
        .copied()
        .collect::<BTreeSet<_>>()
        .difference(&base_ids)
        .copied()
        .collect::<BTreeSet<_>>();
    assert_eq!(external, expected_ids);

    let observed_names = env
        .globals
        .iter()
        .filter(|(name, identity)| {
            external.contains(identity)
                && (name.starts_with("Capability.Parsing.Cursor.")
                    || name.starts_with("Data.Collections.Derived."))
        })
        .map(|(name, _)| name.clone())
        .collect::<BTreeSet<_>>();
    assert_eq!(observed_names, expected_names);
}

/// Promise class: durable invariant.
///
/// MEASURED: standalone roots loading preserves trust, class, and instance
/// populations while each implementation helper and the induction lemma rejects
/// through a real external selective import. CLAIMED: publication changes only
/// Json's declared usable boundary and keeps implementation plumbing private.
/// THE GAP: exact carrier and proof-body preservation is a one-shot object diff,
/// not a permanent source-text test.
#[test]
fn json_publication_is_visibility_only() {
    let _ = load_json();
    for surface in [
        "char_cursor_remaining",
        "char_cursor_peek",
        "char_cursor_advance",
        "char_cursor_locate",
        "char_cursor_lt_suc",
    ] {
        assert_private(surface);
    }
}
