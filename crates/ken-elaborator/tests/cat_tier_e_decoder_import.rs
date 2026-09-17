//! Tier-E configuration Decoder publication and strict-import controls.

#[path = "support/catalog_or.rs"]
mod catalog_or;
#[path = "support/catalog_publication.rs"]
mod catalog_publication;

use std::collections::{BTreeMap, BTreeSet};

use ken_elaborator::{Decl as SurfaceDecl, ElabEnv, ElabError, ImportKind, literate, parser};
use ken_kernel::{Decl, GlobalId, Term};

const DECODER: &str = "Application.Configuration.Decoder";
const DECODER_SOURCE: &str =
    include_str!("../../../catalog/packages/Application/Configuration/Decoder.ken.md");
const SCHEMA: &str = "Application.Input.Schema";
const SCHEMA_SOURCE: &str =
    include_str!("../../../catalog/packages/Application/Input/Schema.ken.md");

fn names(items: &[&str]) -> BTreeSet<String> {
    items.iter().map(|item| (*item).to_owned()).collect()
}

fn schema_imports() -> BTreeSet<String> {
    names(&[
        "MkSchemaIssue",
        "Schema",
        "SchemaField",
        "SchemaFieldCheck",
        "SchemaIssue",
        "SchemaValidation",
        "schema_check_presence",
        "schema_field_accept",
        "schema_field_name",
        "schema_field_presence",
        "schema_fields",
        "schema_help",
        "schema_issue_code",
        "schema_issue_origin",
        "schema_validate",
    ])
}

fn public_surface() -> BTreeSet<String> {
    names(&[
        "decode_config_entries",
        "decode_process_environment",
        "env_config_help",
    ])
}

fn private_surface() -> [&'static str; 18] {
    [
        "EnvConfigOrigin",
        "EnvVariableOrigin",
        "ConfigEntryOrigin",
        "config_field_check",
        "config_field_origin",
        "decode_environment_entries",
        "env_config_entry_key",
        "env_config_entry_value",
        "env_config_field_check",
        "env_config_issue_diagnostic",
        "env_config_lookup",
        "env_config_lookup_choice",
        "env_config_missing_field",
        "env_config_origin_to_origin",
        "env_config_validation",
        "env_config_values",
        "environment_field_check",
        "environment_field_origin",
    ]
}

fn load_decoder() -> (ElabEnv, BTreeSet<GlobalId>, BTreeSet<GlobalId>) {
    let mut env = ElabEnv::new().expect("base environment");
    let mut base_ids = env.globals.values().copied().collect::<BTreeSet<_>>();
    base_ids.extend(env.env.declarations().iter().map(Decl::id));
    for provider in [
        "Application.Input.Schema",
        "Capability.Diagnostics.Core",
        "Capability.Formatting.Doc",
        "Capability.Process.Environment",
        "Core.Classes.LawfulClasses",
        "Data.Collections.NonEmpty",
        "Data.Sums.Validation",
    ] {
        env.elaborate_module_from_roots(&[catalog_or::catalog_root()], provider)
            .unwrap_or_else(|error| {
                panic!("configuration Decoder provider {provider} must roots-load: {error:?}")
            });
    }
    let provider_numeric_ids = env.num_values.keys().copied().collect::<BTreeSet<_>>();
    let before_trust = env.env.trusted_base();
    let before_classes = env.class_env.class_entries().count();
    let before_instances = env.class_env.instances.len();
    let mut owned = env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], DECODER)
        .expect("configuration Decoder must roots-load through declared providers")
        .into_iter()
        .collect::<BTreeSet<_>>();
    owned.extend(
        env.globals
            .iter()
            .filter(|(name, _)| name.starts_with("Application.Configuration.Decoder."))
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
        "configuration Decoder imports and publication must add no trust"
    );
    assert_eq!(
        env.class_env.class_entries().count(),
        before_classes,
        "configuration Decoder must not mint a class"
    );
    assert_eq!(
        env.class_env.instances.len(),
        before_instances,
        "configuration Decoder must not mint an instance"
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
/// MEASURED: the real parser returns exactly the seven D0-ledger module/name
/// sets. CLAIMED: Decoder declares every provider dependency and no unused edge.
/// THE GAP: AST equality establishes the interface; checked identity closure
/// separately establishes that every imported name is retained by compiled terms.
#[test]
fn decoder_selective_import_ledger_is_exact() {
    let extracted = literate::extract_ken_md(DECODER_SOURCE).expect("Decoder extraction");
    let declarations = parser::parse_decls(&extracted.source).expect("Decoder source must parse");
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
                panic!("Decoder dependency imports must all be selective")
            }
            _ => None,
        })
        .collect::<BTreeMap<_, _>>();
    let expected = BTreeMap::from([
        ("Application.Input.Schema".to_owned(), schema_imports()),
        (
            "Capability.Diagnostics.Core".to_owned(),
            names(&[
                "ConfigKeyOrigin",
                "Diagnostic",
                "EnvironmentOrigin",
                "MkDiagnostic",
                "MkDiagnosticCode",
                "Origin",
            ]),
        ),
        ("Capability.Formatting.Doc".to_owned(), names(&["Doc"])),
        (
            "Capability.Process.Environment".to_owned(),
            names(&["process_environment"]),
        ),
        (
            "Core.Classes.LawfulClasses".to_owned(),
            names(&["bytes_deceq_eq"]),
        ),
        (
            "Data.Collections.NonEmpty".to_owned(),
            names(&["NonEmpty", "nonempty_map"]),
        ),
        (
            "Data.Sums.Validation".to_owned(),
            names(&["Invalid", "Valid", "Validation"]),
        ),
    ]);
    assert_eq!(actual, expected);
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: loader queries resolve exactly the three D0-consumed driver names,
/// and checked external wrappers retain all three canonical Decoder identities.
/// CLAIMED: callers can decode process environments, decode config entries, and
/// render matching schema help without access to implementation traversals. THE
/// GAP: provider provenance is measured independently by checked identity closure.
#[test]
fn decoder_loader_visible_inventory_is_exact_and_usable() {
    let expected = public_surface();
    assert_eq!(
        catalog_publication::published_module_surfaces(DECODER_SOURCE, DECODER, "decoder"),
        expected
    );
    let (mut env, _, _) = load_decoder();
    let canonical = expected
        .iter()
        .map(|surface| {
            (
                surface.clone(),
                env.globals[&format!("{DECODER}.{surface}")],
            )
        })
        .collect::<BTreeMap<_, _>>();
    env.elaborate_file(
        r#"
        import Application.Configuration.Decoder
          (decode_config_entries as decoder_client_config,
            decode_process_environment as decoder_client_process,
            env_config_help as decoder_client_help)
        import Application.Input.Schema (Schema)
        import Capability.Diagnostics.Core (Diagnostic)
        import Capability.Formatting.Doc (Doc)
        import Data.Collections.NonEmpty (NonEmpty)
        import Data.Sums.Validation (Validation)
        fn decoder_process_client (schema : Schema) (input : ProcessInput)
            : Validation (NonEmpty Diagnostic) (List Bytes) =
          decoder_client_process schema input
        fn decoder_config_client (schema : Schema) (entries : List (Prod Bytes Bytes))
            : Validation (NonEmpty Diagnostic) (List Bytes) =
          decoder_client_config schema entries
        fn decoder_help_client (schema : Schema) : Doc = decoder_client_help schema
        "#,
    )
    .expect("the complete Decoder driver surface must import and type-check together");
    let mut referenced = BTreeSet::new();
    for client in [
        "decoder_process_client",
        "decoder_config_client",
        "decoder_help_client",
    ] {
        let identity = env.globals[client];
        collect_decl_globals(
            env.env.lookup(identity).expect("checked Decoder client"),
            &mut referenced,
        );
    }
    for (surface, identity) in canonical {
        assert!(
            referenced.contains(&identity),
            "checked external clients must retain {DECODER}.{surface}'s canonical identity"
        );
    }
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: every non-prelude, non-owned identity in checked Decoder terms is
/// exactly one of the 29 D0 identities, and all 15 Schema identities belong to
/// Schema's published 26-name surface. CLAIMED: Decoder has no undeclared
/// provider, mis-cut Schema dependency, or unexpected Tier-E edge. THE GAP:
/// unused source imports are covered by the exact parsed ledger.
#[test]
fn decoder_checked_provider_and_schema_closure_is_exact() {
    let (mut env, owned, base_ids) = load_decoder();
    let expected_names = names(&[
        "Application.Input.Schema.MkSchemaIssue",
        "Application.Input.Schema.Schema",
        "Application.Input.Schema.SchemaField",
        "Application.Input.Schema.SchemaFieldCheck",
        "Application.Input.Schema.SchemaIssue",
        "Application.Input.Schema.SchemaValidation",
        "Application.Input.Schema.schema_check_presence",
        "Application.Input.Schema.schema_field_accept",
        "Application.Input.Schema.schema_field_name",
        "Application.Input.Schema.schema_field_presence",
        "Application.Input.Schema.schema_fields",
        "Application.Input.Schema.schema_help",
        "Application.Input.Schema.schema_issue_code",
        "Application.Input.Schema.schema_issue_origin",
        "Application.Input.Schema.schema_validate",
        "Capability.Diagnostics.Core.ConfigKeyOrigin",
        "Capability.Diagnostics.Core.Diagnostic",
        "Capability.Diagnostics.Core.EnvironmentOrigin",
        "Capability.Diagnostics.Core.MkDiagnostic",
        "Capability.Diagnostics.Core.MkDiagnosticCode",
        "Capability.Diagnostics.Core.Origin",
        "Capability.Formatting.Doc.Doc",
        "Capability.Process.Environment.process_environment",
        "Core.Classes.LawfulClasses.bytes_deceq_eq",
        "Data.Collections.NonEmpty.NonEmpty",
        "Data.Collections.NonEmpty.nonempty_map",
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
        .filter(|(name, identity)| name.contains('.') && external.contains(identity))
        .map(|(name, _)| name.clone())
        .collect::<BTreeSet<_>>();
    assert_eq!(observed_names, expected_names);

    let schema_names = expected_names
        .iter()
        .filter_map(|name| name.strip_prefix("Application.Input.Schema."))
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    assert_eq!(schema_names, schema_imports());
    let schema_public =
        catalog_publication::published_module_surfaces(SCHEMA_SOURCE, SCHEMA, "decoder_schema");
    assert_eq!(schema_public.len(), 26);
    assert!(schema_names.is_subset(&schema_public));
    env.elaborate_file(&format!(
        "import {SCHEMA} ({})",
        schema_imports().into_iter().collect::<Vec<_>>().join(", ")
    ))
    .expect("all 15 directly consumed Schema names must import together");
    match env.elaborate_file(&format!("import {SCHEMA} (schema_field_shape)")) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, "Application.Input.Schema.schema_field_shape");
        }
        other => panic!("Schema's internal shape accessor must reject, got {other:?}"),
    }
}

/// Promise class: durable invariant.
///
/// MEASURED: roots loading preserves trust, class, and instance populations
/// while all 18 implementation names reject through real external imports.
/// CLAIMED: Decoder changes only declared dependencies and coherent driver
/// visibility. THE GAP: exact body preservation is a one-shot object diff.
#[test]
fn decoder_publication_is_visibility_only() {
    let (mut env, _, _) = load_decoder();
    for surface in private_surface() {
        match env.elaborate_file(&format!("import {DECODER} ({surface})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{DECODER}.{surface}"));
            }
            other => panic!("{DECODER}.{surface} must stay private, got {other:?}"),
        }
    }
}
