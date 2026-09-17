//! Tier-E ArgParse publication and strict-import controls.

#[path = "support/catalog_or.rs"]
mod catalog_or;
#[path = "support/catalog_publication.rs"]
mod catalog_publication;

use std::collections::{BTreeMap, BTreeSet};

use ken_elaborator::{Decl as SurfaceDecl, ElabEnv, ElabError, ImportKind, literate, parser};
use ken_kernel::{Decl, GlobalId, Term};

const ARGPARSE: &str = "Application.CommandLine.ArgParse";
const ARGPARSE_SOURCE: &str =
    include_str!("../../../catalog/packages/Application/CommandLine/ArgParse.ken.md");
const SCHEMA: &str = "Application.Input.Schema";
const SCHEMA_SOURCE: &str =
    include_str!("../../../catalog/packages/Application/Input/Schema.ken.md");

fn names(items: &[&str]) -> BTreeSet<String> {
    items.iter().map(|item| (*item).to_owned()).collect()
}

fn schema_imports() -> BTreeSet<String> {
    names(&[
        "SchemaField",
        "MkSchemaField",
        "SchemaOptional",
        "SchemaFlag",
        "SchemaBytes",
        "SchemaRequired",
        "Schema",
        "MkSchema",
        "SchemaFieldCheck",
        "schema_field_presence",
        "SchemaFieldRejected",
        "MkSchemaIssue",
        "SchemaFieldAccepted",
        "SchemaIssue",
        "schema_issue_origin",
        "schema_issue_code",
        "schema_validate_fields",
        "schema_help",
    ])
}

fn public_surface() -> BTreeSet<String> {
    names(&[
        "OptionMode",
        "FlagOption",
        "ValueOption",
        "OptionSpec",
        "MkOptionSpec",
        "PositionalSpec",
        "MkPositionalSpec",
        "CommandSpec",
        "MkCommandSpec",
        "ProgramSpec",
        "MkProgramSpec",
        "ParsedArgument",
        "ParsedFlag",
        "ParsedOption",
        "ParsedPositional",
        "ParsedCommand",
        "MkParsedCommand",
        "argparse_run",
        "command_help",
        "program_help",
    ])
}

fn private_surface() -> [&'static str; 44] {
    [
        "argparse_byte_matches_char",
        "argparse_cons_validations",
        "argparse_diagnostic",
        "argparse_error",
        "argparse_find_command",
        "argparse_find_option",
        "argparse_has_prefix_chars",
        "argparse_long_chars",
        "argparse_matches_chars",
        "argparse_missing_field_check",
        "argparse_missing_positionals",
        "argparse_name_decoder",
        "argparse_option_chars",
        "argparse_option_matches",
        "argparse_option_schema_field",
        "argparse_option_schema_fields",
        "argparse_option_value_chars",
        "argparse_options_chars",
        "argparse_parse_tokens",
        "argparse_parsed_command",
        "argparse_positional_chars",
        "argparse_positional_schema_field",
        "argparse_positional_schema_fields",
        "argparse_positionals_chars",
        "argparse_schema_issue_diagnostic",
        "argparse_short_chars",
        "argparse_single_cursor",
        "argparse_subcommand_chars",
        "argparse_subcommands_chars",
        "argparse_valid_argument",
        "command_description",
        "command_name",
        "command_options",
        "command_positionals",
        "command_schema",
        "option_description",
        "option_mode",
        "option_name",
        "option_short_name",
        "positional_name",
        "positional_required",
        "program_commands",
        "program_description",
        "program_name",
    ]
}

fn load_argparse() -> (ElabEnv, BTreeSet<GlobalId>, BTreeSet<GlobalId>) {
    let mut env = ElabEnv::new().expect("base environment");
    let mut base_ids = env.globals.values().copied().collect::<BTreeSet<_>>();
    base_ids.extend(env.env.declarations().iter().map(Decl::id));
    for provider in [
        "Application.Input.Schema",
        "Capability.Diagnostics.Core",
        "Capability.Formatting.Doc",
        "Capability.Parsing.Cursor",
        "Capability.Parsing.Decoder",
        "Data.Collections.Derived",
        "Data.Collections.NonEmpty",
        "Data.Sums.Validation",
    ] {
        env.elaborate_module_from_roots(&[catalog_or::catalog_root()], provider)
            .unwrap_or_else(|error| {
                panic!("ArgParse provider {provider} must roots-load: {error:?}")
            });
    }
    let provider_numeric_ids = env.num_values.keys().copied().collect::<BTreeSet<_>>();
    let before_trust = env.env.trusted_base();
    let before_classes = env.class_env.class_entries().count();
    let before_instances = env.class_env.instances.len();
    let mut owned = env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], ARGPARSE)
        .expect("ArgParse must roots-load through declared providers")
        .into_iter()
        .collect::<BTreeSet<_>>();
    owned.extend(
        env.globals
            .iter()
            .filter(|(name, _)| name.starts_with("Application.CommandLine.ArgParse."))
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
        "ArgParse imports and publication must add no trust"
    );
    assert_eq!(
        env.class_env.class_entries().count(),
        before_classes,
        "ArgParse must not mint a class"
    );
    assert_eq!(
        env.class_env.instances.len(),
        before_instances,
        "ArgParse must not mint an instance"
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
/// MEASURED: the real parser returns exactly the eight D0-ledger module/name
/// sets. CLAIMED: ArgParse declares every provider dependency and no unused
/// edge. THE GAP: AST equality establishes the interface; the checked identity
/// test separately establishes the provider closure used by compiled terms.
#[test]
fn argparse_selective_import_ledger_is_exact() {
    let extracted = literate::extract_ken_md(ARGPARSE_SOURCE).expect("ArgParse extraction");
    let declarations = parser::parse_decls(&extracted.source).expect("ArgParse source must parse");
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
                panic!("ArgParse dependency imports must all be selective")
            }
            _ => None,
        })
        .collect::<BTreeMap<_, _>>();
    let expected = BTreeMap::from([
        ("Application.Input.Schema".to_owned(), schema_imports()),
        (
            "Capability.Diagnostics.Core".to_owned(),
            names(&[
                "Diagnostic",
                "MkDiagnostic",
                "ArgumentOrigin",
                "MkByteRange",
                "MkDiagnosticCode",
            ]),
        ),
        (
            "Capability.Formatting.Doc".to_owned(),
            names(&["Doc", "Text"]),
        ),
        (
            "Capability.Parsing.Cursor".to_owned(),
            names(&[
                "ArgCursor",
                "ArgLocation",
                "arg_cursor_ops",
                "arg_cursor_start",
                "arg_length",
                "cursor_remaining",
            ]),
        ),
        (
            "Capability.Parsing.Decoder".to_owned(),
            names(&[
                "Decoder",
                "decoder_pure",
                "decoder_bind",
                "decoder_satisfy",
                "DecoderFailed",
                "Decoded",
            ]),
        ),
        (
            "Data.Collections.Derived".to_owned(),
            names(&["list_append"]),
        ),
        (
            "Data.Collections.NonEmpty".to_owned(),
            names(&[
                "NonEmpty",
                "nonempty_cons",
                "nonempty_map",
                "Semigroup_instance_NonEmpty",
            ]),
        ),
        (
            "Data.Sums.Validation".to_owned(),
            names(&[
                "Invalid",
                "Valid",
                "Validation",
                "validation_ap",
                "validation_map",
            ]),
        ),
    ]);
    assert_eq!(actual, expected);
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: loader queries resolve exactly the coherent 20-name CLI-driver API
/// to canonical ArgParse identities. CLAIMED: callers can construct command
/// specifications, inspect parsed results, parse arguments, and render both
/// command and program help without access to implementation traversals. THE
/// GAP: publication queries do not establish provider provenance, which the
/// checked-identity control measures independently.
#[test]
fn argparse_loader_visible_inventory_is_exact() {
    let expected = public_surface();
    assert_eq!(
        catalog_publication::published_module_surfaces(ARGPARSE_SOURCE, ARGPARSE, "argparse"),
        expected
    );
    let (mut env, _, _) = load_argparse();
    let canonical = expected
        .iter()
        .map(|surface| {
            (
                surface.clone(),
                env.globals[&format!("{ARGPARSE}.{surface}")],
            )
        })
        .collect::<BTreeMap<_, _>>();
    let selections = expected
        .iter()
        .enumerate()
        .map(|(index, surface)| format!("{surface} as cat_tier_e_argparse_{index}"))
        .collect::<Vec<_>>()
        .join(", ");
    env.elaborate_file(&format!("import {ARGPARSE} ({selections})"))
        .expect("the complete ArgParse public surface must import together");
    for surface in &expected {
        assert_eq!(
            env.globals[&format!("{ARGPARSE}.{surface}")],
            canonical[surface],
            "selective import must preserve {ARGPARSE}.{surface}'s canonical identity"
        );
    }
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: every non-prelude, non-owned identity in checked ArgParse terms is
/// one of the 49 D0 identities, including Schema and Decoder carrier reach; all
/// 20 Schema identities belong to Schema's published 26-name surface. CLAIMED:
/// ArgParse has no undeclared provider, mis-cut Schema dependency, or unexpected
/// Tier-E edge. THE GAP: unused source imports are covered by the exact ledger.
#[test]
fn argparse_checked_provider_and_schema_closure_is_exact() {
    let (mut env, owned, base_ids) = load_argparse();
    let expected_names = names(&[
        "Application.Input.Schema.MkSchema",
        "Application.Input.Schema.MkSchemaField",
        "Application.Input.Schema.MkSchemaIssue",
        "Application.Input.Schema.Schema",
        "Application.Input.Schema.SchemaBytes",
        "Application.Input.Schema.SchemaField",
        "Application.Input.Schema.SchemaFieldAccepted",
        "Application.Input.Schema.SchemaFieldCheck",
        "Application.Input.Schema.SchemaFieldRejected",
        "Application.Input.Schema.SchemaFlag",
        "Application.Input.Schema.SchemaIssue",
        "Application.Input.Schema.SchemaOptional",
        "Application.Input.Schema.SchemaPresence",
        "Application.Input.Schema.SchemaRequired",
        "Application.Input.Schema.SchemaValueShape",
        "Application.Input.Schema.schema_field_presence",
        "Application.Input.Schema.schema_help",
        "Application.Input.Schema.schema_issue_code",
        "Application.Input.Schema.schema_issue_origin",
        "Application.Input.Schema.schema_validate_fields",
        "Capability.Diagnostics.Core.ArgumentOrigin",
        "Capability.Diagnostics.Core.Diagnostic",
        "Capability.Diagnostics.Core.MkByteRange",
        "Capability.Diagnostics.Core.MkDiagnostic",
        "Capability.Diagnostics.Core.MkDiagnosticCode",
        "Capability.Formatting.Doc.Doc",
        "Capability.Formatting.Doc.Text",
        "Capability.Parsing.Cursor.ArgCursor",
        "Capability.Parsing.Cursor.ArgLocation",
        "Capability.Parsing.Cursor.arg_cursor_ops",
        "Capability.Parsing.Cursor.arg_cursor_start",
        "Capability.Parsing.Cursor.arg_length",
        "Capability.Parsing.Cursor.cursor_remaining",
        "Capability.Parsing.Decoder.Decoder",
        "Capability.Parsing.Decoder.DecoderError",
        "Capability.Parsing.Decoder.DecoderResult",
        "Capability.Parsing.Decoder.decoder_bind",
        "Capability.Parsing.Decoder.decoder_pure",
        "Capability.Parsing.Decoder.decoder_satisfy",
        "Data.Collections.Derived.list_append",
        "Data.Collections.NonEmpty.NonEmpty",
        "Data.Collections.NonEmpty.nonempty_cons",
        "Data.Collections.NonEmpty.nonempty_map",
        "Data.Sums.Validation.Invalid",
        "Data.Sums.Validation.Valid",
        "Data.Sums.Validation.Validation",
        "Data.Sums.Validation.validation_ap",
        "Data.Sums.Validation.validation_map",
        "Semigroup_instance_Data.Collections.NonEmpty.NonEmpty",
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
    assert_eq!(schema_names.len(), 20);
    let schema_public =
        catalog_publication::published_module_surfaces(SCHEMA_SOURCE, SCHEMA, "argparse_schema");
    assert!(schema_names.is_subset(&schema_public));
    let selections = schema_imports().into_iter().collect::<Vec<_>>().join(", ");
    env.elaborate_file(&format!("import {SCHEMA} ({selections})"))
        .expect("all 18 directly consumed Schema names must import together");
    match env.elaborate_file(&format!("import {SCHEMA} (schema_field_shape)")) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, "Application.Input.Schema.schema_field_shape");
        }
        other => panic!("Schema's internal shape accessor must reject, got {other:?}"),
    }
    assert_eq!(
        env.globals["string_to_list_char"], env.prelude_env.string_to_list_char_id,
        "ArgParse String decomposition must remain a prelude primitive"
    );
}

/// Promise class: durable invariant.
///
/// MEASURED: roots loading preserves trust, class, and instance populations
/// while all 44 implementation names reject through real external imports.
/// CLAIMED: ArgParse changes only dependency and coherent CLI-driver visibility.
/// THE GAP: exact body preservation is a one-shot object diff.
#[test]
fn argparse_publication_is_visibility_only() {
    let (mut env, _, _) = load_argparse();
    for surface in private_surface() {
        match env.elaborate_file(&format!("import {ARGPARSE} ({surface})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{ARGPARSE}.{surface}"));
            }
            other => panic!("{ARGPARSE}.{surface} must stay private, got {other:?}"),
        }
    }
}
