//! Tier-E Schema publication and strict-import controls.

#[path = "support/catalog_or.rs"]
mod catalog_or;
#[path = "support/catalog_publication.rs"]
mod catalog_publication;

use std::collections::{BTreeMap, BTreeSet};

use ken_elaborator::{Decl as SurfaceDecl, ElabEnv, ElabError, ImportKind, literate, parser};
use ken_kernel::{Decl, GlobalId, Term};

const SCHEMA: &str = "Application.Input.Schema";
const SCHEMA_SOURCE: &str =
    include_str!("../../../catalog/packages/Application/Input/Schema.ken.md");

fn names(items: &[&str]) -> BTreeSet<String> {
    items.iter().map(|item| (*item).to_owned()).collect()
}

fn public_surface() -> BTreeSet<String> {
    names(&[
        "SchemaPresence",
        "SchemaRequired",
        "SchemaOptional",
        "SchemaValueShape",
        "SchemaFlag",
        "SchemaBytes",
        "SchemaField",
        "MkSchemaField",
        "Schema",
        "MkSchema",
        "SchemaIssue",
        "MkSchemaIssue",
        "SchemaFieldCheck",
        "SchemaFieldAccepted",
        "SchemaFieldRejected",
        "SchemaValidation",
        "schema_field_name",
        "schema_field_presence",
        "schema_fields",
        "schema_field_accept",
        "schema_check_presence",
        "schema_issue_origin",
        "schema_issue_code",
        "schema_validate_fields",
        "schema_validate",
        "schema_help",
    ])
}

fn private_surface() -> [&'static str; 12] {
    [
        "schema_documentation",
        "schema_field_detail_chars",
        "schema_field_documentation",
        "schema_field_help_chars",
        "schema_field_label_chars",
        "schema_field_reject",
        "schema_field_shape",
        "schema_fields_help_chars",
        "schema_name",
        "schema_presence_chars",
        "schema_shape_chars",
        "schema_validation_cons",
    ]
}

fn load_schema() -> (ElabEnv, BTreeSet<GlobalId>, BTreeSet<GlobalId>) {
    let mut env = ElabEnv::new().expect("base environment");
    let mut base_ids = env.globals.values().copied().collect::<BTreeSet<_>>();
    base_ids.extend(env.env.declarations().iter().map(Decl::id));
    for provider in [
        "Capability.Formatting.Doc",
        "Data.Collections.Derived",
        "Data.Collections.NonEmpty",
        "Data.Sums.Validation",
    ] {
        env.elaborate_module_from_roots(&[catalog_or::catalog_root()], provider)
            .unwrap_or_else(|error| {
                panic!("Schema provider {provider} must roots-load: {error:?}")
            });
    }
    let provider_numeric_ids = env.num_values.keys().copied().collect::<BTreeSet<_>>();
    let before_trust = env.env.trusted_base();
    let before_classes = env.class_env.class_entries().count();
    let before_instances = env.class_env.instances.len();
    let mut owned = env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], SCHEMA)
        .expect("Application.Input.Schema must roots-load through declared providers")
        .into_iter()
        .collect::<BTreeSet<_>>();
    owned.extend(
        env.globals
            .iter()
            .filter(|(name, _)| name.starts_with("Application.Input.Schema."))
            .map(|(_, identity)| *identity),
    );
    owned.extend(
        env.num_values
            .keys()
            .filter(|identity| !provider_numeric_ids.contains(identity))
            .copied(),
    );
    assert_eq!(
        env.env.trusted_base(),
        before_trust,
        "Schema imports and publication must add no trust"
    );
    assert_eq!(
        env.class_env.class_entries().count(),
        before_classes,
        "Schema must not mint a class"
    );
    assert_eq!(
        env.class_env.instances.len(),
        before_instances,
        "Schema must not mint an instance"
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

/// Promise class: normative compatibility vector.
///
/// MEASURED: the real parser returns exactly the four D0-ledger module/name
/// sets. CLAIMED: Schema declares every provider dependency and no unused edge.
/// THE GAP: AST equality establishes the module interface; the checked-core
/// identity test separately establishes that every imported name is consumed.
#[test]
fn schema_selective_import_ledger_is_exact() {
    let extracted = literate::extract_ken_md(SCHEMA_SOURCE).expect("Schema literate extraction");
    let declarations = parser::parse_decls(&extracted.source).expect("Schema source must parse");
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
                panic!("Schema dependency imports must all be selective")
            }
            _ => None,
        })
        .collect::<BTreeMap<_, _>>();
    let expected = BTreeMap::from([
        (
            "Capability.Formatting.Doc".to_string(),
            names(&["Doc", "Text"]),
        ),
        (
            "Data.Collections.Derived".to_string(),
            names(&["list_append"]),
        ),
        (
            "Data.Collections.NonEmpty".to_string(),
            names(&["NonEmpty", "nonempty_append", "nonempty_cons"]),
        ),
        (
            "Data.Sums.Validation".to_string(),
            names(&["Invalid", "Valid", "Validation"]),
        ),
    ]);
    assert_eq!(actual, expected);
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: the roots loader queries every declaration and constructor and a
/// real selective-import client resolves exactly the 26-name ArgParse/Decoder
/// union to canonical Schema identities. CLAIMED: Schema publishes precisely
/// its two real clients' usable carrier, constructor, accessor, and traversal
/// surface. THE GAP: publication queries do not prove provider provenance,
/// which the sibling checked-core identity test measures independently.
#[test]
fn schema_loader_visible_inventory_is_exact() {
    let expected = public_surface();
    assert_eq!(
        catalog_publication::published_module_surfaces(SCHEMA_SOURCE, SCHEMA, "schema"),
        expected
    );

    let (mut env, _, _) = load_schema();
    let canonical = expected
        .iter()
        .map(|surface| (surface.clone(), env.globals[&format!("{SCHEMA}.{surface}")]))
        .collect::<BTreeMap<_, _>>();
    let selections = expected
        .iter()
        .enumerate()
        .map(|(index, surface)| format!("{surface} as cat_tier_e_schema_{index}"))
        .collect::<Vec<_>>()
        .join(", ");
    env.elaborate_file(&format!("import {SCHEMA} ({selections})"))
        .expect("the complete Schema public surface must import together");
    for surface in &expected {
        assert_eq!(
            env.globals[&format!("{SCHEMA}.{surface}")],
            canonical[surface],
            "selective import must preserve {SCHEMA}.{surface}'s canonical identity"
        );
    }
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: every non-prelude, non-owned identity in Schema's checked
/// declarations is exactly one of the nine D0-measured provider identities.
/// CLAIMED: Schema has no undeclared provider or unexpected Tier-E edge. THE
/// GAP: checked-core identity closure cannot detect an unused import, while the
/// exact parsed-import ledger covers that residual.
#[test]
fn schema_checked_provider_identity_closure_is_exact() {
    let (env, owned, base_ids) = load_schema();
    let expected_names = names(&[
        "Capability.Formatting.Doc.Doc",
        "Capability.Formatting.Doc.Text",
        "Data.Collections.Derived.list_append",
        "Data.Collections.NonEmpty.NonEmpty",
        "Data.Collections.NonEmpty.nonempty_append",
        "Data.Collections.NonEmpty.nonempty_cons",
        "Data.Sums.Validation.Invalid",
        "Data.Sums.Validation.Valid",
        "Data.Sums.Validation.Validation",
    ]);
    let expected_ids = expected_names
        .iter()
        .map(|name| env.globals[name])
        .collect::<BTreeSet<_>>();

    let mut resolved = BTreeSet::new();
    for identity in &owned {
        if let Some(declaration) = env.env.lookup(*identity) {
            collect_decl_globals(declaration, &mut resolved);
        }
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
                && (name.starts_with("Capability.Formatting.Doc.")
                    || name.starts_with("Data.Collections.Derived.")
                    || name.starts_with("Data.Collections.NonEmpty.")
                    || name.starts_with("Data.Sums.Validation."))
        })
        .map(|(name, _)| name.clone())
        .collect::<BTreeSet<_>>();
    assert_eq!(observed_names, expected_names);
    assert_eq!(
        env.globals["string_to_list_char"], env.prelude_env.string_to_list_char_id,
        "Schema's String decomposition must remain a prelude primitive"
    );
}

/// Promise class: durable invariant.
///
/// MEASURED: standalone roots loading preserves trust, class, and instance
/// populations while all twelve non-client helpers reject through real external
/// selective imports. CLAIMED: Schema's migration changes only dependency and
/// usable-client visibility. THE GAP: exact body preservation is a one-shot
/// object diff rather than a permanent source-text test.
#[test]
fn schema_publication_is_visibility_only() {
    let (mut env, _, _) = load_schema();
    for surface in private_surface() {
        match env.elaborate_file(&format!("import {SCHEMA} ({surface})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{SCHEMA}.{surface}"));
            }
            other => panic!("{SCHEMA}.{surface} must stay private, got {other:?}"),
        }
    }
}
