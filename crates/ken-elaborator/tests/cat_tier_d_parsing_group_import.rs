//! Tier-D Parsing-group publication, provider, and sibling-edge controls.

#[path = "support/catalog_or.rs"]
mod catalog_or;
#[path = "support/catalog_publication.rs"]
mod catalog_publication;

use std::collections::BTreeSet;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, Term};

const DC: &str = "Capability.Diagnostics.Core";
const CURSOR: &str = "Capability.Parsing.Cursor";
const DECODER: &str = "Capability.Parsing.Decoder";
const NUMERIC: &str = "Capability.Parsing.Numeric";
const PARSING: &str = "Capability.Parsing.Parsing";
const ARGUMENTS: &str = "Capability.Process.Arguments";
const TRANSPORT: &str = "Core.Logic.Transport";
const NUMERIC_SOURCE: &str =
    include_str!("../../../catalog/packages/Capability/Parsing/Numeric.ken.md");
const PARSING_SOURCE: &str =
    include_str!("../../../catalog/packages/Capability/Parsing/Parsing.ken.md");

fn names(items: &[&str]) -> BTreeSet<String> {
    items.iter().map(|item| (*item).to_owned()).collect()
}

fn module_surface_names(env: &ElabEnv, module: &str, ids: &BTreeSet<GlobalId>) -> BTreeSet<String> {
    let prefix = format!("{module}.");
    env.globals
        .iter()
        .filter_map(|(name, id)| {
            (ids.contains(id) && name.starts_with(&prefix)).then(|| {
                name.strip_prefix(&prefix)
                    .expect("checked prefix")
                    .to_owned()
            })
        })
        .collect()
}

fn load(env: &mut ElabEnv, module: &str) -> BTreeSet<GlobalId> {
    let before = env.globals.values().copied().collect::<BTreeSet<_>>();
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], module)
        .unwrap_or_else(|error| panic!("{module} must roots-load: {error:?}"));
    env.globals
        .values()
        .copied()
        .filter(|id| !before.contains(id))
        .collect()
}

fn module_ids(env: &ElabEnv, module: &str) -> BTreeSet<GlobalId> {
    let prefix = format!("{module}.");
    env.globals
        .iter()
        .filter_map(|(name, id)| name.starts_with(&prefix).then_some(*id))
        .collect()
}

fn expose(env: &mut ElabEnv, module: &str) {
    catalog_or::expose_module(env, module);
}

fn term_refs(term: &Term, refs: &mut BTreeSet<GlobalId>) {
    match term {
        Term::Const { id, .. } | Term::IndFormer { id, .. } | Term::Constructor { id, .. } => {
            refs.insert(*id);
        }
        Term::Elim { fam, .. } => {
            refs.insert(*fam);
        }
        _ => {}
    }
    for child in term.children() {
        term_refs(child, refs);
    }
}

fn owned_refs(env: &ElabEnv, owned: &BTreeSet<GlobalId>) -> BTreeSet<GlobalId> {
    let mut refs = BTreeSet::new();
    for id in owned {
        match env.env.lookup(*id) {
            Some(Decl::Transparent { ty, body, .. }) => {
                term_refs(ty, &mut refs);
                term_refs(body, &mut refs);
            }
            Some(Decl::Opaque { ty, .. } | Decl::Primitive { ty, .. }) => term_refs(ty, &mut refs),
            Some(Decl::Inductive(inductive)) => {
                term_refs(&inductive.former_type, &mut refs);
                for term in inductive.params.iter().chain(&inductive.indices) {
                    term_refs(term, &mut refs);
                }
                for constructor in &inductive.constructors {
                    term_refs(&constructor.type_, &mut refs);
                    for term in constructor.args.iter().chain(&constructor.target_indices) {
                        term_refs(term, &mut refs);
                    }
                }
            }
            None => {}
        }
    }
    refs
}

struct DirectNumeric {
    env: ElabEnv,
    transport: BTreeSet<GlobalId>,
    diagnostics: BTreeSet<GlobalId>,
    numeric: BTreeSet<GlobalId>,
}

fn direct_numeric() -> DirectNumeric {
    let mut env = ElabEnv::new().expect("base environment");
    load(&mut env, TRANSPORT);
    let transport = module_ids(&env, TRANSPORT);
    load(&mut env, DC);
    let diagnostics = module_ids(&env, DC);
    let before_trust = env.env.trusted_base();
    let before_classes = env.class_env.class_entries().count();
    let before_instances = env.class_env.instances.len();
    let numeric = load(&mut env, NUMERIC);
    assert_eq!(env.env.trusted_base(), before_trust);
    assert_eq!(env.class_env.class_entries().count(), before_classes);
    assert_eq!(env.class_env.instances.len(), before_instances);
    DirectNumeric {
        env,
        transport,
        diagnostics,
        numeric,
    }
}

struct LoadedNumeric {
    env: ElabEnv,
    transport: BTreeSet<GlobalId>,
    diagnostics: BTreeSet<GlobalId>,
    cursor: BTreeSet<GlobalId>,
    decoder: BTreeSet<GlobalId>,
    parsing_sibling: BTreeSet<GlobalId>,
    arguments_sibling: BTreeSet<GlobalId>,
    numeric: BTreeSet<GlobalId>,
}

fn loaded_numeric() -> LoadedNumeric {
    let mut env = ElabEnv::new().expect("base environment");
    load(&mut env, TRANSPORT);
    let transport = module_ids(&env, TRANSPORT);
    expose(&mut env, TRANSPORT);
    for module in [
        "Core.Classes.LawfulClasses",
        "Data.Collections.Derived",
        "Data.Numeric.Nat.Order",
    ] {
        load(&mut env, module);
        expose(&mut env, module);
    }
    load(&mut env, DC);
    let diagnostics = module_ids(&env, DC);
    expose(&mut env, DC);
    load(&mut env, CURSOR);
    let cursor = module_ids(&env, CURSOR);
    expose(&mut env, CURSOR);
    load(&mut env, DECODER);
    let decoder = module_ids(&env, DECODER);
    expose(&mut env, DECODER);
    load(&mut env, PARSING);
    let parsing_sibling = module_ids(&env, PARSING);
    expose(&mut env, PARSING);
    load(&mut env, ARGUMENTS);
    let arguments_sibling = module_ids(&env, ARGUMENTS);
    expose(&mut env, ARGUMENTS);
    let before_trust = env.env.trusted_base();
    let before_classes = env.class_env.class_entries().count();
    let before_instances = env.class_env.instances.len();
    let numeric = load(&mut env, NUMERIC);
    assert_eq!(env.env.trusted_base(), before_trust);
    assert_eq!(env.class_env.class_entries().count(), before_classes);
    assert_eq!(env.class_env.instances.len(), before_instances);
    LoadedNumeric {
        env,
        transport,
        diagnostics,
        cursor,
        decoder,
        parsing_sibling,
        arguments_sibling,
        numeric,
    }
}

struct DirectParsing {
    env: ElabEnv,
    lawful: BTreeSet<GlobalId>,
    derived: BTreeSet<GlobalId>,
    nat_order: BTreeSet<GlobalId>,
    diagnostics: BTreeSet<GlobalId>,
    cursor: BTreeSet<GlobalId>,
    decoder: BTreeSet<GlobalId>,
    numeric_sibling: BTreeSet<GlobalId>,
    arguments_sibling: BTreeSet<GlobalId>,
    parsing: BTreeSet<GlobalId>,
}

fn direct_parsing() -> DirectParsing {
    let mut env = ElabEnv::new().expect("base environment");
    load(&mut env, "Core.Classes.LawfulClasses");
    let lawful = module_ids(&env, "Core.Classes.LawfulClasses");
    load(&mut env, "Data.Collections.Derived");
    let derived = module_ids(&env, "Data.Collections.Derived");
    load(&mut env, "Data.Numeric.Nat.Order");
    let nat_order = module_ids(&env, "Data.Numeric.Nat.Order");
    load(&mut env, DC);
    let diagnostics = module_ids(&env, DC);
    load(&mut env, CURSOR);
    let cursor = module_ids(&env, CURSOR);
    load(&mut env, DECODER);
    let decoder = module_ids(&env, DECODER);
    load(&mut env, NUMERIC);
    let numeric_sibling = module_ids(&env, NUMERIC);
    let before_trust = env.env.trusted_base();
    let before_classes = env.class_env.class_entries().count();
    let before_instances = env.class_env.instances.len();
    let parsing = load(&mut env, PARSING);
    assert_eq!(env.env.trusted_base(), before_trust);
    assert_eq!(env.class_env.class_entries().count(), before_classes + 1);
    assert_eq!(env.class_env.instances.len(), before_instances);
    for module in [
        "Core.Classes.LawfulClasses",
        "Data.Collections.Derived",
        CURSOR,
    ] {
        expose(&mut env, module);
    }
    let arguments_sibling = load(&mut env, ARGUMENTS);
    DirectParsing {
        env,
        lawful,
        derived,
        nat_order,
        diagnostics,
        cursor,
        decoder,
        numeric_sibling,
        arguments_sibling,
        parsing,
    }
}

fn assert_private(surface: &str) {
    let mut loaded = direct_numeric();
    match loaded
        .env
        .elaborate_file(&format!("import {NUMERIC} ({surface})"))
    {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{NUMERIC}.{surface}"))
        }
        other => panic!("{NUMERIC}.{surface} must stay private, got {other:?}"),
    }
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: every publishable Numeric declaration and constructor is queried
/// through the roots loader, and the successful set equals the package's exact
/// declared public API. CLAIMED: the first Parsing-group partial publishes only
/// Numeric's reader-facing contract. THE GAP: future additive API changes must
/// deliberately update this compatibility vector.
#[test]
fn parsing_numeric_loader_visible_inventory_is_exact() {
    let expected = names(&[
        "char_to_digit",
        "numeric_argument_origin",
        "parse_digits_at",
        "parse_formatted_digits",
        "parse_int",
        "parse_int_chars",
        "parse_nat",
        "parse_nat_chars",
    ]);
    assert_eq!(
        catalog_publication::published_module_surfaces(NUMERIC_SOURCE, NUMERIC, "parsing_numeric"),
        expected
    );
    let mut loaded = direct_numeric();
    let imports = expected.iter().cloned().collect::<Vec<_>>().join(", ");
    loaded
        .env
        .elaborate_file(&format!("import {NUMERIC} ({imports})"))
        .expect("the exact Numeric surface must import together");
}

/// Promise class: durable invariant.
///
/// MEASURED: Numeric's checked references intersect its normalized provider
/// closure at twelve Diagnostics.Core and two Transport identities, while
/// remaining disjoint from Cursor, Decoder, and both group siblings; loading
/// adds no trust, class, or instance. The ten direct Diagnostics.Core imports
/// induce two additional checked identities, `byte_range_start` and
/// `byte_range_end`, through the transparent `origin_range_start` and
/// `origin_range_end` bodies. CLAIMED: Numeric's complete catalog-value closure
/// is explicit, already published, and independent of the other group members.
/// THE GAP: per-item necessity is supplied by the production-side removal
/// campaign.
#[test]
fn parsing_numeric_provider_closure_is_exact_and_sibling_disjoint() {
    let direct = direct_numeric();
    let refs = owned_refs(&direct.env, &direct.numeric);
    let actual_diagnostics = refs
        .intersection(&direct.diagnostics)
        .copied()
        .collect::<BTreeSet<_>>();
    let actual_transport = refs
        .intersection(&direct.transport)
        .copied()
        .collect::<BTreeSet<_>>();
    assert_eq!(
        module_surface_names(&direct.env, DC, &actual_diagnostics),
        names(&[
            "ArgumentOrigin",
            "Diagnostic",
            "DiagnosticCode",
            "byte_range_end",
            "byte_range_start",
            "MkByteRange",
            "MkDiagnostic",
            "MkDiagnosticCode",
            "Origin",
            "origin_argument_index",
            "origin_range_end",
            "origin_range_start",
        ])
    );
    assert_eq!(
        module_surface_names(&direct.env, TRANSPORT, &actual_transport),
        names(&["cong", "trans"])
    );
    let loaded = loaded_numeric();
    let refs = owned_refs(&loaded.env, &loaded.numeric);
    for (module, owned) in [
        (CURSOR, &loaded.cursor),
        (DECODER, &loaded.decoder),
        (PARSING, &loaded.parsing_sibling),
        (ARGUMENTS, &loaded.arguments_sibling),
    ] {
        assert!(
            refs.is_disjoint(owned),
            "Numeric must remain disjoint from {module}"
        );
    }
}

/// Promise class: durable invariant.
///
/// MEASURED: real selective-import clients cannot name representative private
/// constructors, proof helpers, or formatting workers. CLAIMED: publication is
/// visibility-only and does not expose Numeric implementation state. THE GAP:
/// the exact inventory test covers every other publishable sibling.
#[test]
fn parsing_numeric_implementation_siblings_remain_private() {
    for name in [
        "NumericErrorKind",
        "EmptyInput",
        "InvalidDigit",
        "DecimalDigit",
        "MkDecimalDigit",
        "numeric_argument_origin_index_faithful",
        "numeric_error_code",
        "numeric_diagnostic",
        "decimal_digit_value",
        "format_digits",
        "format_digits_roundtrip",
        "parse_digit_result",
        "show_digits",
    ] {
        assert_private(name);
    }
}

fn assert_parsing_private(surface: &str) {
    let mut loaded = direct_parsing();
    match loaded
        .env
        .elaborate_file(&format!("import {PARSING} ({surface})"))
    {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{PARSING}.{surface}"))
        }
        other => panic!("{PARSING}.{surface} must stay private, got {other:?}"),
    }
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: every publishable Parsing declaration and constructor is queried
/// through the roots loader, and the successful set equals the coherent public
/// API. A strict client imports that whole surface and constructs every plain
/// transparent carrier it needs. CLAIMED: published types have a usable
/// producer/observer path rather than an inert or callable-looking boundary.
/// THE GAP: Source construction is class-mediated and separately checked by the
/// class registry; ByteCursor stays abstract because parser_from_decoder
/// constructs it while byte_cursor_ops supplies the public decoder dictionary.
#[test]
fn parsing_module_loader_visible_inventory_is_exact_and_coherent() {
    let expected = names(&[
        "BAnd",
        "BFalse",
        "BNot",
        "BTrue",
        "BoolExpr",
        "ByteCursor",
        "Failed",
        "FailedValid",
        "IsUtf8",
        "LessEqNat",
        "LessEqNat::refl",
        "LessEqNat::zero_left",
        "Located",
        "MkLocated",
        "MkParseError",
        "MkSpan",
        "MkSyntax",
        "ParseError",
        "ParseResult",
        "ParseResultValid",
        "Parsed",
        "ParsedValid",
        "Parser",
        "ParserLaws",
        "ParserSourceLocal",
        "ParserTotal",
        "ParserValid",
        "Source",
        "Span",
        "Syntax",
        "ValidLocated",
        "ValidLocatedList",
        "ValidSpan",
        "ValidSyntax",
        "byte_cursor_ops",
        "erase_spans",
        "error_source",
        "error_span",
        "format_bool_expr",
        "located_source",
        "located_span",
        "located_value",
        "parse_bool_expr",
        "parser_fail",
        "parser_from_decoder",
        "parser_pure",
        "print_bool_expr",
        "source_bytes",
        "source_bytes::utf8",
        "source_id",
        "source_length",
        "span_end",
        "span_origin",
        "span_start",
        "span_to_byte_range",
        "syntax_children",
        "syntax_root",
        "valid_zero_width_span",
    ]);
    assert_eq!(
        catalog_publication::published_module_surfaces(PARSING_SOURCE, PARSING, "parsing_module"),
        expected
    );
    let mut loaded = direct_parsing();
    let imports = expected
        .iter()
        .filter(|surface| !surface.contains("::"))
        .cloned()
        .collect::<Vec<_>>()
        .join(", ");
    loaded
        .env
        .elaborate_file(&format!("import {PARSING} ({imports})"))
        .expect("the exact Parsing surface must import together");
    loaded
        .env
        .elaborate_file(
            "import Capability.Diagnostics.Core (SourceId)\n\
             import Capability.Parsing.Parsing \
               (BAnd, BFalse, BNot, BTrue, BoolExpr, Failed, Located, MkLocated, \
                MkParseError, MkSpan, MkSyntax, ParseError, ParseResult, Parsed, \
                Span, Syntax)\n\
             const strict_span : Span = MkSpan Zero Zero\n\
             fn strict_located (source : SourceId) : Located Bool = \
               MkLocated Bool source strict_span True\n\
             fn strict_error (source : SourceId) : ParseError = \
               MkParseError source strict_span\n\
             fn strict_success (source : SourceId) : ParseResult Bool = \
               Parsed Bool True strict_span Zero\n\
             fn strict_failure (source : SourceId) : ParseResult Bool = \
               Failed Bool (strict_error source)\n\
             const strict_expr : BoolExpr = BAnd BTrue (BNot BFalse)\n\
             fn strict_syntax (source : SourceId) : Syntax BoolExpr = \
               MkSyntax BoolExpr \
                 (MkLocated BoolExpr source strict_span strict_expr) \
                 (Nil (Located BoolExpr))",
        )
        .expect("plain Parsing carriers must be constructible by a strict client");
    let source = loaded
        .env
        .class_env
        .class("Source")
        .expect("strict Parsing must publish its one Source class identity");
    assert_eq!(
        source.projection.field_names,
        vec!["source_id_field", "source_bytes_field", "source_utf8_field"]
    );
}

/// Promise class: durable invariant.
///
/// MEASURED: Parsing's checked declarations intersect Diagnostics.Core, Cursor,
/// Decoder, and its already-published lower-tier providers at the exact named
/// GlobalIds below, while both group-sibling intersections are empty. Loading
/// adds the single existing Source class and no instance or trust. CLAIMED: the
/// complete catalog dependency closure is explicit and the group remains a set
/// of independent releasable modules. THE GAP: per-import necessity is supplied
/// by the production-side removal campaign.
#[test]
fn parsing_module_provider_closure_is_exact_and_sibling_disjoint() {
    let loaded = direct_parsing();
    let refs = owned_refs(&loaded.env, &loaded.parsing);
    let intersection_names = |module: &str, owned: &BTreeSet<GlobalId>| {
        let ids = refs.intersection(owned).copied().collect::<BTreeSet<_>>();
        module_surface_names(&loaded.env, module, &ids)
    };
    assert_eq!(
        intersection_names(DC, &loaded.diagnostics),
        names(&[
            "ByteRange",
            "MkByteRange",
            "Origin",
            "SourceId",
            "SourceOrigin",
            "byte_range_end",
            "byte_range_start",
            "origin_source_id",
        ])
    );
    assert_eq!(
        intersection_names(CURSOR, &loaded.cursor),
        names(&["CursorOps", "MkCursorOps"])
    );
    assert_eq!(
        intersection_names(DECODER, &loaded.decoder),
        names(&[
            "Decoded",
            "Decoder",
            "DecoderError",
            "DecoderFailed",
            "DecoderRejected",
            "DecoderResult",
            "decoder_alt",
            "decoder_error_location",
            "decoder_fail",
            "decoder_many",
            "decoder_pure",
            "decoder_recursive",
            "decoder_satisfy",
            "decoder_seq",
        ])
    );
    assert_eq!(
        intersection_names("Core.Classes.LawfulClasses", &loaded.lawful),
        names(&["leq_nat"])
    );
    assert_eq!(
        intersection_names("Data.Collections.Derived", &loaded.derived),
        names(&["bytes_nat_length", "list_append", "nth"])
    );
    assert_eq!(
        intersection_names("Data.Numeric.Nat.Order", &loaded.nat_order),
        names(&["sub"])
    );
    assert!(
        refs.is_disjoint(&loaded.numeric_sibling),
        "Parsing must remain disjoint from Numeric"
    );
    assert!(
        refs.is_disjoint(&loaded.arguments_sibling),
        "Parsing must remain disjoint from Process.Arguments"
    );
}

/// Promise class: durable invariant.
///
/// MEASURED: real selective-import clients cannot name the abstract cursor
/// constructor, internal cursor operations, proof helpers, law intermediates,
/// or grammar workers. CLAIMED: the published surface exposes the package
/// contract without its implementation machinery. THE GAP: exact inventory
/// equality covers every other declaration and constructor.
#[test]
fn parsing_module_implementation_siblings_remain_private() {
    for name in [
        "MkByteCursor",
        "byte_cursor_source",
        "byte_cursor_position",
        "byte_cursor_remaining",
        "byte_cursor_peek",
        "byte_cursor_advance",
        "byte_cursor_locate",
        "span_to_byte_range_faithful",
        "span_origin_source_faithful",
        "decoder_parse_error",
        "ParseResultTotal",
        "ParseResultSourceLocal",
        "bool_expr_eq",
        "syntax_leaf",
        "byte_code_decoder",
        "complete_bool_decoder",
    ] {
        assert_parsing_private(name);
    }
}
