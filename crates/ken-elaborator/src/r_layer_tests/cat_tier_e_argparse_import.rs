//! Tier-E ArgParse publication and strict-import controls.

#[path = "../../tests/support/catalog_or.rs"]
mod catalog_or;
#[path = "../../tests/support/catalog_publication.rs"]
mod catalog_publication;

use std::collections::{BTreeMap, BTreeSet};

use ken_elaborator::{literate, parser, Decl as SurfaceDecl, ElabEnv, ElabError, ImportKind};
use ken_kernel::{Decl, GlobalId, Term};

const ARGPARSE: &str = "Application.CommandLine.ArgParse";
const ARGPARSE_SOURCE: &str =
    include_str!("../../../../catalog/packages/Application/CommandLine/ArgParse.ken.md");
const SCHEMA: &str = "Application.Input.Schema";
const SCHEMA_SOURCE: &str =
    include_str!("../../../../catalog/packages/Application/Input/Schema.ken.md");

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
        "schema_fields",
        "SchemaFieldRejected",
        "MkSchemaIssue",
        "SchemaFieldAccepted",
        "SchemaIssue",
        "schema_issue_origin",
        "schema_issue_code",
        "schema_expected_issue_list",
        "schema_observed_issue_list",
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
        "argparse_option_schema_field",
        "argparse_positional_schema_field",
        "command_schema",
        "command_schema::fields_in_spec_order",
        "argparse_run",
        "command_help",
        "command_help::rendered_from_schema",
        "program_help",
    ])
}

fn private_surface() -> Vec<&'static str> {
    vec![
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
        "argparse_option_schema_fields",
        "option_schema_fields_map",
        "argparse_option_value_chars",
        "argparse_options_chars",
        "argparse_parse_tokens",
        "argparse_parsed_command",
        "argparse_positional_chars",
        "argparse_positional_schema_fields",
        "positional_schema_fields_map",
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
        "option_description",
        "option_mode",
        "option_name",
        "option_short_name",
        "positional_name",
        "positional_required",
        "program_commands",
        "program_description",
        "program_name",
        "argparse_accepted_error_order",
        "argparse_argument_bytes",
        "argparse_cons_accepted_expected",
        "argparse_cons_accepted_observation",
        "argparse_cons_bytes_step",
        "argparse_cons_diagnostic_head",
        "argparse_cons_diagnostic_step",
        "argparse_cons_diagnostic_tail",
        "argparse_cons_flag_expected",
        "argparse_cons_flag_observation",
        "argparse_cons_input_bytes_bridge",
        "argparse_cons_input_bytes_view",
        "argparse_cons_option_expected",
        "argparse_cons_option_observation",
        "argparse_cons_outcome_view",
        "argparse_cons_positional_expected",
        "argparse_cons_positional_observation",
        "argparse_cons_rejected_bytes",
        "argparse_cons_valid_flag_case",
        "argparse_cons_valid_option_case",
        "argparse_cons_valid_option_expected_case",
        "argparse_cons_valid_positional_case",
        "argparse_cons_valid_positional_expected_case",
        "argparse_cons_view_bytes",
        "argparse_expected_bytes_if_valid",
        "argparse_expected_missing_diagnostics",
        "argparse_false_bytes_case",
        "argparse_input_value_bytes",
        "argparse_long_prefix_view",
        "argparse_missing_bytes",
        "argparse_missing_bytes_for_schema_result",
        "argparse_missing_diagnostic_projection",
        "argparse_missing_option_value_code",
        "argparse_missing_result_view",
        "argparse_none_bytes_case",
        "argparse_none_diagnostic_false",
        "argparse_none_diagnostic_step",
        "argparse_none_diagnostic_true",
        "argparse_observation_transport",
        "argparse_observed_bytes",
        "argparse_observed_diagnostics",
        "argparse_parse_tokens::diagnostic_sequence_base",
        "argparse_parse_tokens::diagnostic_sequence_step",
        "argparse_parse_tokens::full_validation_cons_bridge",
        "argparse_parse_tokens::value_bytes_from_input",
        "argparse_prepend_bytes",
        "argparse_prepend_valid_bytes",
        "argparse_rejected_error_order",
        "argparse_rejected_valid_case",
        "argparse_unexpected_positional_code",
        "argparse_unknown_bytes_case",
        "argparse_unknown_option_code",
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
        "Core.Logic.Transport",
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
            "Core.Logic.Transport".to_owned(),
            names(&["cong", "sym", "trans"]),
        ),
        (
            "Data.Collections.Derived".to_owned(),
            names(&["list_append", "map"]),
        ),
        (
            "Data.Collections.NonEmpty".to_owned(),
            names(&[
                "NonEmpty",
                "nonempty_append",
                "nonempty_cons",
                "nonempty_map",
                "nonempty_to_list",
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
/// MEASURED: loader queries resolve the coherent CLI-driver API and its help laws
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
        .filter(|(_, surface)| !surface.contains("::"))
        .map(|(index, surface)| format!("{surface} as cat_tier_e_argparse_{index}"))
        .collect::<Vec<_>>()
        .join(", ");
    env.elaborate_file(&format!("import {ARGPARSE} ({selections})"))
        .expect("the complete ArgParse public surface must import together");
    for surface in expected.iter().filter(|surface| !surface.contains("::")) {
        assert_eq!(
            env.globals[&format!("{ARGPARSE}.{surface}")],
            canonical[surface],
            "selective import must preserve {ARGPARSE}.{surface}'s canonical identity"
        );
    }
}

/// Promise class: durable invariant.
///
/// MEASURED: an external client instantiates the attached field-order proof on
/// one option followed by one positional, while an empty-fields conclusion is
/// rejected. CLAIMED: the law is usable for an inhabited, nonempty spec and
/// proves content, not just a selectable proof name. THE GAP: this exercise
/// does not prove the parser byte or diagnostic claims.
#[test]
fn argparse_help_order_law_is_usable_for_nonempty_specs() {
    let (mut env, _, _) = load_argparse();
    env.elaborate_file(
        r#"
import Application.CommandLine.ArgParse
  (OptionSpec, MkOptionSpec, FlagOption, PositionalSpec, MkPositionalSpec,
    CommandSpec, MkCommandSpec, command_schema, argparse_option_schema_field,
    argparse_positional_schema_field)
import Application.Input.Schema (SchemaField, schema_fields)
import Data.Collections.Derived (list_append, map)

const flag : OptionSpec = MkOptionSpec "verbose" (None String) FlagOption "report more"
const input : PositionalSpec = MkPositionalSpec "file" True
const options : List OptionSpec = Cons OptionSpec flag (Nil OptionSpec)
const positionals : List PositionalSpec = Cons PositionalSpec input (Nil PositionalSpec)
const command : CommandSpec = MkCommandSpec "run" "desc" options positionals

theorem inhabited_help_order
  : Equal (List SchemaField)
    (schema_fields (command_schema command))
    (list_append SchemaField
      (map OptionSpec SchemaField argparse_option_schema_field options)
      (map PositionalSpec SchemaField argparse_positional_schema_field positionals)) =
  command_schema::fields_in_spec_order "run" "desc" options positionals
"#,
    )
    .expect("strict client must apply attached law to an inhabited mixed spec");
    let wrong = env.elaborate_file(
        "theorem help_fields_cannot_disappear : Equal (List SchemaField) \
         (schema_fields (command_schema command)) \
         (Nil SchemaField) = Refl",
    );
    assert!(
        matches!(
            wrong,
            Err(ElabError::KernelRejected { .. }) | Err(ElabError::TypeMismatch { .. })
        ),
        "a nonempty help schema cannot be proved empty: {wrong:?}"
    );
}

/// Promise class: durable invariant for the checked parser obligations.
///
/// MEASURED: roots-loaded private attached proofs are transparent, quantify
/// over the arbitrary parser inputs, and have distinct `Equal` endpoints: the
/// left observes the real parser and the right names the independently stated
/// byte, missing-issue, or single-token diagnostic projection. CLAIMED: the
/// parser's proof obligations have not been replaced by reflexive filler or
/// disconnected from the original parser. THE GAP: this pins their raw checked
/// types, not every reduction step; the kernel re-checks their proof bodies.
#[test]
fn argparse_checked_laws_retain_real_parser_and_distinct_endpoints() {
    let (env, _, _) = load_argparse();
    let parser_id = env.globals[&format!("{ARGPARSE}.argparse_parse_tokens")];
    for (law, binder_count, expected_right) in [
        (
            "value_bytes_from_input",
            4,
            vec!["argparse_input_value_bytes"],
        ),
        (
            "diagnostic_sequence_base",
            3,
            vec!["argparse_expected_missing_diagnostics"],
        ),
        (
            "diagnostic_sequence_step",
            5,
            vec![
                "argparse_cons_diagnostic_head",
                "argparse_cons_diagnostic_tail",
            ],
        ),
    ] {
        let name = format!("{ARGPARSE}.argparse_parse_tokens::{law}");
        let id = env.globals[&name];
        let Decl::Transparent { ty, .. } = env.env.lookup(id).expect("attached law loaded") else {
            panic!("{name} must have a checked proof body, not an assumption");
        };
        let mut proposition = ty;
        let mut binders = 0;
        while let Term::Pi(_, result) = proposition {
            binders += 1;
            proposition = result;
        }
        assert_eq!(binders, binder_count, "{name} must remain generic");
        let Term::App(equal_left, right) = proposition else {
            panic!("{name} must state an equality");
        };
        let Term::App(equal_carrier, left) = equal_left.as_ref() else {
            panic!("{name} must state two equality endpoints");
        };
        let Term::App(equal, _) = equal_carrier.as_ref() else {
            panic!("{name} must state a carrier for its equality");
        };
        assert_eq!(equal.as_ref(), &Term::const_(env.globals["Equal"], vec![]));
        assert_ne!(left, right, "{name} cannot be a reflexive placeholder");
        let mut left_ids = BTreeSet::new();
        collect_term_globals(left, &mut left_ids);
        assert!(
            left_ids.contains(&parser_id),
            "{name} must observe the original parser"
        );
        let mut right_ids = BTreeSet::new();
        collect_term_globals(right, &mut right_ids);
        for expected in expected_right {
            assert!(
                right_ids.contains(&env.globals[&format!("{ARGPARSE}.{expected}")]),
                "{name} must retain its independent {expected} projection"
            );
        }
    }
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: every non-prelude, non-owned identity in checked ArgParse terms is
/// in the expected provider closure, including Schema, Decoder, and checked
/// transport proofs. The Schema identities belong to Schema's public surface. CLAIMED:
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
        "Application.Input.Schema.schema_fields",
        "Application.Input.Schema.schema_help",
        "Application.Input.Schema.schema_issue_code",
        "Application.Input.Schema.schema_issue_origin",
        "Application.Input.Schema.schema_expected_issue_list",
        "Application.Input.Schema.schema_observed_issue_list",
        "Application.Input.Schema.schema_validate_fields",
        "Application.Input.Schema.schema_validate_fields::invalid_issue_sequence",
        "Core.Logic.Transport.cong",
        "Core.Logic.Transport.sym",
        "Core.Logic.Transport.trans",
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
        "Data.Collections.Derived.map",
        "Data.Collections.NonEmpty.NonEmpty",
        "Data.Collections.NonEmpty.nonempty_append::list_view",
        "Data.Collections.NonEmpty.nonempty_cons",
        "Data.Collections.NonEmpty.nonempty_map",
        "Data.Collections.NonEmpty.nonempty_map::list_view",
        "Data.Collections.NonEmpty.nonempty_to_list",
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
    assert_eq!(
        external,
        expected_ids,
        "unexpected references: {:?}",
        external
            .difference(&expected_ids)
            .map(|id| (
                *id,
                env.globals
                    .iter()
                    .filter(|(_, val)| *val == id)
                    .map(|(name, _)| name.as_str())
                    .collect::<Vec<_>>()
            ))
            .collect::<Vec<_>>()
    );
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
    let schema_public =
        catalog_publication::published_module_surfaces(SCHEMA_SOURCE, SCHEMA, "argparse_schema");
    assert!(schema_names.is_subset(&schema_public));
    let selections = schema_imports().into_iter().collect::<Vec<_>>().join(", ");
    env.elaborate_file(&format!("import {SCHEMA} ({selections})"))
        .expect("all directly consumed Schema names must import together");
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
/// while all remaining implementation names reject through real external imports.
/// CLAIMED: ArgParse changes only dependency and coherent CLI-driver visibility.
/// THE GAP: exact body preservation is a one-shot object diff.
#[test]
fn argparse_publication_is_visibility_only() {
    let (mut env, _, _) = load_argparse();
    for surface in private_surface() {
        // Attached proofs have qualified `::` names, which selective-import
        // syntax cannot spell. Their parent is private and their identities
        // are checked by the raw-proposition control below.
        if surface.contains("::") {
            assert!(env.globals.contains_key(&format!("{ARGPARSE}.{surface}")));
            continue;
        }
        match env.elaborate_file(&format!("import {ARGPARSE} ({surface})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{ARGPARSE}.{surface}"));
            }
            other => panic!("{ARGPARSE}.{surface} must stay private, got {other:?}"),
        }
    }
}
