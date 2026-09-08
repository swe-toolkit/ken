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
        "DecimalDigit",
        "NumericErrorKind",
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
        "EmptyInput",
        "InvalidDigit",
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
