//! `LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE` acceptance.
//!
//! Spec: `spec/30-surface/32-grammar.md:393` — the five leading forms "reject
//! at their leading token when ungrouped after an application head".
//!
//! **Every assertion here is on the error's IDENTITY — which production raised
//! it — never on `span.start`.** The previous candidate for this node asserted
//! the coordinate and went 9/9 green against an implementation that did not
//! reject at all: deleting `Token::KwIf` from `can_start_atom_expr` makes the
//! argument loop *break*, and the leftover token is then reported as a stray by
//! the declaration parser **at the same coordinate**. A continuation predicate
//! can only stop; it can never reject. A stray token is always reported at
//! itself, so a position assertion cannot discriminate and no fixture would
//! have made it bite.
//!
//! Promise classes: the rejection and its producer are a durable invariant (it
//! is what `:393` pins). The diagnostic's wording is a normative compatibility
//! vector only in the marker below — tests key on `ARGUMENT_LOOP_MARKER`, so
//! the prose may be reworded without touching the assertions.

use ken_elaborator::parser::parse_decls;
use ken_elaborator::ElabError;
use std::path::{Path, PathBuf};

/// The phrase that identifies the ARGUMENT LOOP as the producer. No other
/// production emits it; the declaration parser's stray-token error enumerates
/// declaration keywords instead.
const ARGUMENT_LOOP_MARKER: &str = "not an application_atom";

/// The declaration parser's stray-token signature — what the DEFECTIVE
/// implementation produces. Asserted absent, so the two are told apart rather
/// than merely "an error occurred".
const DECL_PARSER_SIGNATURE: &str = "expected 'const'";

const UNGROUPED_IF_ARG: &str = "const k : Nat = keep if c then a else b";

fn parse_error(src: &str) -> (String, usize) {
    match parse_decls(src).expect_err("must reject") {
        ElabError::ParseError { msg, span } => (msg, span.start),
        other => panic!("expected a parse error, got {other:?}"),
    }
}

/// AC-IF-REJECTS-AFFIRMATIVELY. The rejection is raised by the argument loop.
///
/// This is the assertion the previous candidate could not make. It fails
/// against an implementation that merely removes `KwIf` from the predicate —
/// that one still errors, and still errors at the `if`, but the producer is the
/// declaration parser.
#[test]
fn ac_if_rejects_affirmatively_from_the_argument_loop() {
    let (msg, _) = parse_error(UNGROUPED_IF_ARG);
    assert!(
        msg.contains(ARGUMENT_LOOP_MARKER),
        "rejection must come from the argument loop, got: {msg}"
    );
    assert!(
        !msg.contains(DECL_PARSER_SIGNATURE),
        "a declaration-parser stray-token error means the loop broke instead of \
         rejecting — the defect this node replaces. Got: {msg}"
    );
}

/// The coordinate is still correct — but asserted only AFTER the producer, and
/// never on its own. Kept to show the position claim holds, not as the gate.
#[test]
fn the_affirmative_rejection_is_located_at_the_if() {
    let (msg, start) = parse_error(UNGROUPED_IF_ARG);
    assert!(msg.contains(ARGUMENT_LOOP_MARKER), "producer first: {msg}");
    assert_eq!(start, UNGROUPED_IF_ARG.find("if").expect("fixture has `if`"));
}

/// AC-IF-REJECTS controls. A candidate that made `if` unparseable everywhere
/// would satisfy the criterion above and break the language, so both surviving
/// positions are asserted.
#[test]
fn grouped_and_leading_if_are_unaffected() {
    parse_decls("const k : Nat = keep (if c then a else b)")
        .expect("a GROUPED if is a legal application argument");
    parse_decls("const k : Nat = if c then a else b")
        .expect("a LEADING if is dispatched before the argument loop");
}

/// AC-GUARD-UNBROKEN. The negative control for the five-loop trap.
///
/// `can_start_pattern` (`:2540`) has `KwIf` absent for the OPPOSITE reason:
/// after a pattern, `if` legitimately opens a match-arm guard, and the loop
/// breaking is the mechanism by which guards parse. Converting THAT stop into a
/// rejection — the natural misreading of "a continuation predicate cannot
/// reject, so make it reject" — rejects every guarded arm.
///
/// `can_start_atom_pat` is `can_start_pattern() && !is_contextual_ident("as")`,
/// so an edit to `can_start_pattern` reaches the `:2590` loop too, with only
/// one of the two in the diff a reviewer reads. This fixture is the control for
/// both, and must exist even though nobody intends to touch that loop.
#[test]
fn ac_guard_unbroken_guarded_match_arms_still_parse() {
    parse_decls("const k : Nat = match z { A x if c |-> b ; B |-> d }")
        .expect("a guarded match arm must still parse");
    parse_decls("const k : Nat = match z { A x |-> b }")
        .expect("an unguarded arm must still parse");
}

/// AC-INLINE-KEN-CENSUS. The fixture population includes Ken written inside
/// Rust test files, which a sweep keyed on `.ken` / `.ken.md` cannot reach.
///
/// Four such files were found by four separate CI failures on the previous
/// candidate and never by a census. The known-answer control is
/// `lang_surface_if.rs`: it contains an `if` in application-argument position,
/// so a census that does not return it is measured blind rather than clean.
#[test]
fn ac_inline_ken_census_reaches_rust_test_sources() {
    let tests_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let mut with_inline_if_argument: Vec<PathBuf> = Vec::new();

    for entry in std::fs::read_dir(&tests_dir).expect("tests dir").flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|e| e != "rs") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        // An application head followed by an ungrouped `if`, inside a Rust
        // string literal. Deliberately NOT anchored on a leading quote: the
        // shape routinely sits on a `\`-continuation line that carries none,
        // which is one of the two blind spots that made the earlier grep for
        // this population return zero against files that demonstrably had it.
        if text.contains(" if ")
            && text
                .lines()
                .any(|l| l.contains("= ") && l.contains(" if ") && !l.contains("(if "))
        {
            with_inline_if_argument.push(path);
        }
    }

    let names: Vec<String> = with_inline_if_argument
        .iter()
        .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        .collect();

    assert!(
        names.iter().any(|n| n == "lang_surface_if.rs"),
        "KNOWN-ANSWER CONTROL FAILED: the census must reach lang_surface_if.rs, \
         which carries `if` in application-argument position. A census that \
         misses it is blind, and its zero is not a measurement. Found: {names:?}"
    );
}
