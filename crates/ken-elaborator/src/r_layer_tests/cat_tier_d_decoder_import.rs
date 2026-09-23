//! Tier-D Parsing.Decoder publication and strict-import controls.

#[path = "../../tests/support/catalog_or.rs"]
mod catalog_or;
#[path = "../../tests/support/catalog_publication.rs"]
mod catalog_publication;

use std::collections::BTreeSet;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl as KernelDecl, GlobalId, Term};

const DIAGNOSTICS_CORE: &str = "Capability.Diagnostics.Core";
const PARSING_CURSOR: &str = "Capability.Parsing.Cursor";
const PARSING_DECODER: &str = "Capability.Parsing.Decoder";
const PARSING_DECODER_SOURCE: &str =
    include_str!("../../../../catalog/packages/Capability/Parsing/Decoder.ken.md");

fn names(items: &[&str]) -> BTreeSet<String> {
    items.iter().map(|item| (*item).to_owned()).collect()
}

struct LoadedDecoder {
    env: ElabEnv,
    diagnostics_owned: BTreeSet<GlobalId>,
    cursor_owned: BTreeSet<GlobalId>,
    decoder_owned: BTreeSet<GlobalId>,
}

fn load_decoder() -> LoadedDecoder {
    let root = catalog_or::catalog_root();
    let mut env = ElabEnv::new().expect("base environment");
    let diagnostics_owned = env
        .elaborate_module_from_roots(std::slice::from_ref(&root), DIAGNOSTICS_CORE)
        .expect("Diagnostics.Core provider must roots-load")
        .into_iter()
        .collect();
    let cursor_owned = env
        .elaborate_module_from_roots(std::slice::from_ref(&root), PARSING_CURSOR)
        .expect("Parsing.Cursor provider must roots-load")
        .into_iter()
        .collect();
    let before_trust = env.env.trusted_base();
    let before_classes = env.class_env.class_entries().count();
    let before_instances = env.class_env.instances.len();
    let decoder_owned = env
        .elaborate_module_from_roots(std::slice::from_ref(&root), PARSING_DECODER)
        .expect("Parsing.Decoder must roots-load standalone")
        .into_iter()
        .collect();
    assert_eq!(
        env.env.trusted_base(),
        before_trust,
        "Decoder publication and import must add no trust"
    );
    assert_eq!(
        env.class_env.class_entries().count(),
        before_classes,
        "Decoder must not mint a class"
    );
    assert_eq!(
        env.class_env.instances.len(),
        before_instances,
        "Decoder must not mint an instance"
    );
    LoadedDecoder {
        env,
        diagnostics_owned,
        cursor_owned,
        decoder_owned,
    }
}

fn referenced_globals(term: &Term, found: &mut BTreeSet<GlobalId>) {
    match term {
        Term::Const { id, .. } | Term::IndFormer { id, .. } | Term::Constructor { id, .. } => {
            found.insert(*id);
        }
        Term::Elim { fam, .. } => {
            found.insert(*fam);
        }
        _ => {}
    }
    for child in term.children() {
        referenced_globals(child, found);
    }
}

fn decoder_references(loaded: &LoadedDecoder) -> BTreeSet<GlobalId> {
    let mut referenced = BTreeSet::new();
    for id in &loaded.decoder_owned {
        match loaded.env.env.lookup(*id) {
            Some(KernelDecl::Transparent { ty, body, .. }) => {
                referenced_globals(ty, &mut referenced);
                referenced_globals(body, &mut referenced);
            }
            Some(KernelDecl::Opaque { ty, .. } | KernelDecl::Primitive { ty, .. }) => {
                referenced_globals(ty, &mut referenced);
            }
            Some(KernelDecl::Inductive(inductive)) => {
                referenced_globals(&inductive.former_type, &mut referenced);
                for parameter in &inductive.params {
                    referenced_globals(parameter, &mut referenced);
                }
                for index in &inductive.indices {
                    referenced_globals(index, &mut referenced);
                }
                for constructor in &inductive.constructors {
                    referenced_globals(&constructor.type_, &mut referenced);
                    for argument in &constructor.args {
                        referenced_globals(argument, &mut referenced);
                    }
                    for index in &constructor.target_indices {
                        referenced_globals(index, &mut referenced);
                    }
                }
            }
            None => panic!("Decoder-owned GlobalId must resolve"),
        }
    }
    referenced
}

fn assert_selective_identities(surfaces: &BTreeSet<String>) {
    let mut loaded = load_decoder();
    let canonical_before = surfaces
        .iter()
        .map(|surface| loaded.env.globals[&format!("{PARSING_DECODER}.{surface}")])
        .collect::<Vec<_>>();
    let selections = surfaces
        .iter()
        .enumerate()
        .map(|(index, surface)| format!("{surface} as cat_tier_d_decoder_selected_{index}"))
        .collect::<Vec<_>>()
        .join(", ");
    loaded
        .env
        .elaborate_file(&format!("import {PARSING_DECODER} ({selections})"))
        .expect("Decoder public surface must import together");
    for (surface, before) in surfaces.iter().zip(canonical_before) {
        assert_eq!(
            loaded.env.globals[&format!("{PARSING_DECODER}.{surface}")],
            before,
            "selective import must not remint {PARSING_DECODER}.{surface}"
        );
    }
}

fn assert_private(surface: &str) {
    let mut loaded = load_decoder();
    match loaded
        .env
        .elaborate_file(&format!("import {PARSING_DECODER} ({surface})"))
    {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{PARSING_DECODER}.{surface}"));
        }
        other => panic!("{PARSING_DECODER}.{surface} must stay private, got {other:?}"),
    }
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: the roots loader queries every publishable Decoder declaration and
/// constructor and the successful surface equals the declared carrier,
/// combinator, and checked preservation-law inventory. CLAIMED: Decoder
/// publishes exactly that API to its clients. THE GAP: generated
/// dictionaries are outside the query population, but this module declares no
/// class or instance and `load_decoder` pins that fact.
#[test]
fn parsing_decoder_loader_visible_inventory_is_exact() {
    let expected = names(&[
        "Decoded",
        "Decoder",
        "DecoderError",
        "DecoderFailed",
        "DecoderNonBacktrackable",
        "DecoderRejected",
        "DecoderResult",
        "DecoderPreserves",
        "decoder_alt",
        "decoder_alt_preserves",
        "decoder_alt_propagates_nonbacktrackable",
        "decoder_alt_rejection_uses_second",
        "decoder_bind",
        "decoder_bind_preserves",
        "decoder_error_location",
        "decoder_fail",
        "decoder_fail_preserves",
        "decoder_many",
        "decoder_many_preserves",
        "decoder_pure",
        "decoder_pure_preserves",
        "decoder_recursive",
        "decoder_recursive_preserves",
        "decoder_satisfy",
        "decoder_satisfy_preserves",
        "decoder_seq",
        "decoder_seq_preserves",
    ]);
    assert_eq!(
        catalog_publication::published_module_surfaces(
            PARSING_DECODER_SOURCE,
            PARSING_DECODER,
            "parsing_decoder",
        ),
        expected
    );
    assert_selective_identities(&expected);
}

/// Promise class: durable invariant.
///
/// MEASURED: Decoder roots-loads after its published Cursor provider with zero
/// trust/class/instance growth, and the checked Decoder declarations reference
/// exactly the six D0-measured Cursor-owned identities and no Diagnostics.Core
/// identity directly. CLAIMED: the sole Capability edge is Cursor -> Decoder
/// and every direct provider dependency is canonical. THE GAP: the three
/// compiler conveniences retained by strict-resolution D0 are outside the
/// catalog-provider claim; individual import necessity is established by the
/// population-side removal campaign.
#[test]
fn parsing_decoder_imports_exact_canonical_cursor_surface() {
    let loaded = load_decoder();
    let referenced = decoder_references(&loaded);
    let actual_cursor = referenced
        .intersection(&loaded.cursor_owned)
        .copied()
        .collect::<BTreeSet<_>>();
    let expected_cursor = [
        "CursorOps",
        "cursor_advance",
        "cursor_locate",
        "cursor_nat_lt",
        "cursor_peek",
        "cursor_remaining",
    ]
    .into_iter()
    .map(|surface| loaded.env.globals[&format!("{PARSING_CURSOR}.{surface}")])
    .collect::<BTreeSet<_>>();
    assert_eq!(actual_cursor, expected_cursor);
    assert!(
        referenced.is_disjoint(&loaded.diagnostics_owned),
        "Decoder must not acquire a direct Diagnostics.Core edge"
    );
}

/// Promise class: durable invariant.
///
/// MEASURED: real selective-import clients cannot name any private sibling
/// constructor, combinator worker, convenience combinator, or fuel-law helper.
/// CLAIMED: publication does not expose implementation state beyond the
/// deliberate checked preservation-law inventory. THE GAP: new clients may
/// require a deliberate additive publication; the exact inventory must then
/// move rather than silently inherit visibility.
#[test]
fn parsing_decoder_unconsumed_siblings_remain_private() {
    for surface in [
        "DecoderZeroProgress",
        "DecoderFuelExhausted",
        "decoder_map",
        "decoder_token",
        "decoder_many_fuel",
        "decoder_some",
        "decoder_recursive_fuel",
        "DecoderProgress",
        "DecoderConsumesAll",
        "DecoderRejectsOnlyAtEnd",
        "DecoderManyConsumesAllLaw",
    ] {
        assert_private(surface);
    }
}

/// Promise class: durable invariant.
///
/// MEASURED: a strict client imports the real Parsing cursor, Source and
/// ValidSpan along with Decoder's public proof surface, then constructs
/// preservation terms for fail, pure, alt, many and a recursive layer that
/// actually invokes its recursive argument. CLAIMED: the provider's laws are
/// usable over actual client carriers without exposing private Decoder
/// constructors or fuel helpers. THE GAP: this public-API fixture does not
/// identify the held Parsing branch's private ByteCursorBounded predicate.
#[test]
fn decoder_preservation_laws_elaborate_for_parsing_source_and_span_client() {
    let mut loaded = load_decoder();
    loaded
        .env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], "Capability.Parsing.Parsing")
        .expect("Parsing source/span client must roots-load");
    loaded
        .env
        .elaborate_file(
            r#"
            import Capability.Parsing.Parsing (Source, Span, ByteCursor, ValidSpan, byte_cursor_ops)
            import Capability.Parsing.Cursor (cursor_locate)
            import Capability.Parsing.Decoder
              (Decoder, DecoderPreserves,
                decoder_alt, decoder_alt_preserves,
                decoder_fail, decoder_fail_preserves,
                decoder_many, decoder_many_preserves,
                decoder_pure, decoder_pure_preserves,
                decoder_recursive, decoder_recursive_preserves)

            fn cursor_location_valid (s : Source) (cur : ByteCursor) : Prop =
              ValidSpan s (cursor_locate ByteCursor UInt8 Span byte_cursor_ops cur)

            theorem location_sound (s : Source)
                : (cur : ByteCursor)
                  → cursor_location_valid s cur
                  → ValidSpan s (cursor_locate ByteCursor UInt8 Span byte_cursor_ops cur) =
              λcur. λgood. good

            theorem fail_preserves (s : Source)
                : DecoderPreserves
                    ByteCursor Span Bool (cursor_location_valid s) (ValidSpan s)
                    (decoder_fail ByteCursor UInt8 Span Bool byte_cursor_ops) =
              decoder_fail_preserves
                ByteCursor UInt8 Span Bool byte_cursor_ops
                (cursor_location_valid s) (ValidSpan s) (location_sound s)

            theorem pure_preserves (s : Source) (value : Bool)
                : DecoderPreserves
                    ByteCursor Span Bool (cursor_location_valid s) (ValidSpan s)
                    (decoder_pure ByteCursor Span Bool value) =
              decoder_pure_preserves
                ByteCursor Span Bool (cursor_location_valid s) (ValidSpan s) value

            fn recursive_layer
                  (recur : Decoder ByteCursor Span Bool)
                : Decoder ByteCursor Span Bool =
              decoder_alt
                ByteCursor Span Bool
                (decoder_fail ByteCursor UInt8 Span Bool byte_cursor_ops)
                recur

            theorem alt_preserves (s : Source) (value : Bool)
                : DecoderPreserves
                    ByteCursor Span Bool (cursor_location_valid s) (ValidSpan s)
                    (decoder_alt
                      ByteCursor Span Bool
                      (decoder_fail ByteCursor UInt8 Span Bool byte_cursor_ops)
                      (decoder_pure ByteCursor Span Bool value)) =
              decoder_alt_preserves
                ByteCursor Span Bool (cursor_location_valid s) (ValidSpan s)
                (decoder_fail ByteCursor UInt8 Span Bool byte_cursor_ops)
                (decoder_pure ByteCursor Span Bool value)
                (fail_preserves s)
                (pure_preserves s value)

            theorem many_preserves (s : Source)
                : DecoderPreserves
                    ByteCursor Span (List Bool) (cursor_location_valid s) (ValidSpan s)
                    (decoder_many
                      ByteCursor UInt8 Span Bool byte_cursor_ops
                      (decoder_fail ByteCursor UInt8 Span Bool byte_cursor_ops)) =
              decoder_many_preserves
                ByteCursor UInt8 Span Bool byte_cursor_ops
                (decoder_fail ByteCursor UInt8 Span Bool byte_cursor_ops)
                (cursor_location_valid s) (ValidSpan s)
                (location_sound s) (fail_preserves s)

            theorem recursive_preserves (s : Source)
                : DecoderPreserves
                    ByteCursor Span Bool (cursor_location_valid s) (ValidSpan s)
                    (decoder_recursive
                      ByteCursor UInt8 Span Bool byte_cursor_ops recursive_layer) =
              decoder_recursive_preserves
                ByteCursor UInt8 Span Bool byte_cursor_ops recursive_layer
                (cursor_location_valid s) (ValidSpan s) (location_sound s)
                (λrecur.
                  λrecur_preserves.
                    decoder_alt_preserves
                      ByteCursor Span Bool (cursor_location_valid s) (ValidSpan s)
                      (decoder_fail ByteCursor UInt8 Span Bool byte_cursor_ops)
                      recur
                      (fail_preserves s)
                      recur_preserves)
            "#,
        )
        .expect("public Decoder preservation laws must work for Parsing's source/span carriers");
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: a concrete one-element-remaining cursor with a zero-progress
/// repeated step produces a non-backtrackable failure, and `decoder_alt` leaves
/// that exact outcome unchanged instead of running a successful fallback.
/// CLAIMED: zero progress is a real, distinct non-backtracking arm. THE GAP:
/// the general public theorem quantifies over any fatal error; this closed
/// fixture pins one reachable producer of that error without naming its
/// intentionally private constructor.
#[test]
fn decoder_alt_zero_progress_keeps_failure_instead_of_fallback() {
    let mut loaded = load_decoder();
    loaded
        .env
        .elaborate_file(
            r#"
            import Capability.Parsing.Cursor (CursorOps, MkCursorOps)
            import Capability.Parsing.Decoder
              (DecoderError, DecoderResult, Decoded, DecoderFailed, DecoderRejected,
                DecoderNonBacktrackable, decoder_alt, decoder_many, decoder_pure)

            data StalledCursor = MkStalledCursor
            fn stalled_remaining (cur : StalledCursor) : Nat = Suc Zero
            fn stalled_peek (cur : StalledCursor) : Option UInt8 = None UInt8
            fn stalled_advance (cur : StalledCursor) : StalledCursor = cur
            fn stalled_locate (cur : StalledCursor) : Nat = Zero
            const stalled_ops : CursorOps StalledCursor UInt8 Nat =
              MkCursorOps
                StalledCursor UInt8 Nat
                stalled_remaining stalled_peek stalled_advance stalled_locate

            const stalled_many : DecoderResult StalledCursor Nat (List Bool) =
              decoder_many
                StalledCursor UInt8 Nat Bool stalled_ops
                (decoder_pure StalledCursor Nat Bool True)
                MkStalledCursor

            fn error_or_rejected
                  (outcome : DecoderResult StalledCursor Nat (List Bool))
                : DecoderError Nat =
              match outcome {
                Decoded values next ↦ DecoderRejected Nat Zero;
                DecoderFailed err ↦ err
              }

            theorem stalled_error_is_nonbacktrackable :
                DecoderNonBacktrackable Nat (error_or_rejected stalled_many) =
              Proved

            theorem alt_preserves_the_stalled_failure
                : Equal
                    (DecoderResult StalledCursor Nat (List Bool))
                    (decoder_alt
                      StalledCursor Nat (List Bool)
                      (decoder_many
                        StalledCursor UInt8 Nat Bool stalled_ops
                        (decoder_pure StalledCursor Nat Bool True))
                      (decoder_pure StalledCursor Nat (List Bool) (Nil Bool))
                      MkStalledCursor)
                    stalled_many =
              Proved
            "#,
        )
        .expect("a nonprogress error must not backtrack to a successful alternative");
}
