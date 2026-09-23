//! Tier-E Schema publication and strict-import controls.

#[path = "../../tests/support/catalog_or.rs"]
mod catalog_or;
#[path = "../../tests/support/catalog_publication.rs"]
mod catalog_publication;

use std::collections::{BTreeMap, BTreeSet};

use ken_elaborator::{literate, parser, Decl as SurfaceDecl, ElabEnv, ElabError, ImportKind};
use ken_kernel::{Decl, GlobalId, Term};

const SCHEMA: &str = "Application.Input.Schema";
const SCHEMA_SOURCE: &str =
    include_str!("../../../../catalog/packages/Application/Input/Schema.ken.md");

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
        "schema_validate_fields::accepted_tail_invalid",
        "schema_validate_fields::valid_coverage",
        "schema_validate_fields::invalid_issue_sequence",
        "schema_observed_issue_list",
        "schema_expected_issue_list",
        "schema_validate",
        "schema_help",
    ])
}

fn private_surface() -> [&'static str; 18] {
    [
        "schema_cons_accepted_invalid_issue_order",
        "schema_cons_accepted_valid_issue_order",
        "schema_cons_rejected_invalid_issue_order",
        "schema_cons_rejected_valid_issue_order",
        "schema_documentation",
        "schema_field_detail_chars",
        "schema_field_documentation",
        "schema_field_help_chars",
        "schema_field_label_chars",
        "schema_field_reject",
        "schema_field_shape",
        "schema_fields_help_chars",
        "schema_head_issue_list",
        "schema_name",
        "schema_presence_chars",
        "schema_shape_chars",
        "schema_validation_cons",
        "schema_validation_cons_issue_order",
    ]
}

fn load_schema() -> (ElabEnv, BTreeSet<GlobalId>, BTreeSet<GlobalId>) {
    let mut env = ElabEnv::new().expect("base environment");
    let mut base_ids = env.globals.values().copied().collect::<BTreeSet<_>>();
    base_ids.extend(env.env.declarations().iter().map(Decl::id));
    for provider in [
        "Capability.Formatting.Doc",
        "Core.Logic.Transport",
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
            "Core.Logic.Transport".to_string(),
            names(&["cong", "trans"]),
        ),
        (
            "Data.Collections.Derived".to_string(),
            names(&["list_append", "nth"]),
        ),
        (
            "Data.Collections.NonEmpty".to_string(),
            names(&[
                "NonEmpty",
                "nonempty_append",
                "nonempty_cons",
                "nonempty_to_list",
            ]),
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
/// MEASURED: the roots loader observes the exact published inventory and a
/// selective-import client resolves direct names to canonical Schema identities,
/// and each attached proof's subject is present in that selector set. CLAIMED:
/// Schema publishes precisely this carrier, constructor, accessor, traversal,
/// and attached-proof surface. THE GAP: this arm does not independently exercise
/// expression-level attached-proof resolution through an imported subject alias.
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
    let direct_surfaces = expected
        .iter()
        .filter(|surface| !surface.contains("::"))
        .cloned()
        .collect::<BTreeSet<_>>();
    for attached in expected.iter().filter(|surface| surface.contains("::")) {
        let subject = attached
            .split_once("::")
            .expect("attached Schema surface must name its subject")
            .0;
        assert!(
            direct_surfaces.contains(subject),
            "attached Schema surface {attached} requires direct subject {subject}"
        );
    }
    let selections = direct_surfaces
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

/// Promise class: durable invariant.
///
/// MEASURED: a strict consumer applies the whole-sequence law to two rejected
/// fields and cannot prove its expected list contains only one issue. CLAIMED:
/// the exported proof vocabulary carries all invalid issues in order, not only
/// the first. THE GAP: this fixture is a witness; the general attached law is
/// what quantifies over arbitrary fields and inspectors.
#[test]
fn schema_issue_sequence_law_has_a_two_rejection_client() {
    let (mut env, _, _) = load_schema();
    env.elaborate_file(
        r#"
import Application.Input.Schema
  (SchemaField, MkSchemaField, SchemaOptional, SchemaFlag,
    SchemaFieldCheck, SchemaFieldRejected, SchemaIssue, MkSchemaIssue,
    schema_validate_fields, schema_observed_issue_list, schema_expected_issue_list)

const field : SchemaField = MkSchemaField "one" SchemaOptional SchemaFlag "one"
const issue : SchemaIssue Nat = MkSchemaIssue Nat Zero "missing"
const two_fields : List SchemaField =
  Cons SchemaField field (Cons SchemaField field (Nil SchemaField))
fn reject_field (ignored : SchemaField) : SchemaFieldCheck Nat Bool =
  SchemaFieldRejected Nat Bool issue

theorem all_issues_retain_order
  : Equal (List (SchemaIssue Nat))
    (schema_observed_issue_list Nat Bool
      (schema_validate_fields Nat Bool reject_field two_fields))
    (schema_expected_issue_list Nat Bool reject_field two_fields) =
  schema_validate_fields::invalid_issue_sequence Nat Bool reject_field two_fields
"#,
    )
    .expect("strict consumer must apply ordered issue law to two rejections");
    let wrong = env.elaborate_file(
        "theorem one_issue_is_two : Equal (List (SchemaIssue Nat)) \
         (schema_expected_issue_list Nat Bool reject_field two_fields) \
         (Cons (SchemaIssue Nat) issue (Nil (SchemaIssue Nat))) = Refl",
    );
    assert!(
        matches!(
            wrong,
            Err(ElabError::KernelRejected { .. }) | Err(ElabError::TypeMismatch { .. })
        ),
        "two rejected fields cannot produce a singleton expected list: {wrong:?}"
    );
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: every non-prelude, non-owned identity in Schema's checked
/// declarations is in the exact expected provider closure.
/// CLAIMED: Schema has no undeclared provider or unexpected Tier-E edge. THE
/// GAP: checked-core identity closure cannot detect an unused import, while the
/// exact parsed-import ledger covers that residual.
#[test]
fn schema_checked_provider_identity_closure_is_exact() {
    let (env, owned, base_ids) = load_schema();
    let expected_names = names(&[
        "Capability.Formatting.Doc.Doc",
        "Capability.Formatting.Doc.Text",
        "Core.Logic.Transport.cong",
        "Core.Logic.Transport.trans",
        "Data.Collections.Derived.list_append",
        "Data.Collections.Derived.nth",
        "Data.Collections.NonEmpty.NonEmpty",
        "Data.Collections.NonEmpty.nonempty_append",
        "Data.Collections.NonEmpty.nonempty_cons",
        "Data.Collections.NonEmpty.nonempty_to_list",
        "Data.Collections.NonEmpty.nonempty_append::list_view",
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
                    || name.starts_with("Core.Logic.Transport.")
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
/// populations while private implementation and proof helpers reject through
/// real external selective imports. The exact public-inventory
/// arm closes over the added private theorem helpers. CLAIMED: Schema's migration
/// changes only dependency and usable-client visibility. THE GAP: exact body
/// preservation is a one-shot object diff rather than a permanent source-text
/// test.
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
