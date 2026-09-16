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
/// candidate and never by a census.
///
/// **The known answer is this suite's own `UNGROUPED_IF_ARG`, deliberately.**
/// The first version of this control keyed on `lang_surface_if.rs` "because it
/// contains an `if` in application-argument position" — and the same commit
/// that shipped the control MIGRATED that `if` to the grouped form, so the
/// control went on passing on legal leading-position `if`s and on one Rust
/// `match` guard. **A known answer that the node's own fix can migrate away is
/// not a known answer.** Worse, tightening the predicate correctly would have
/// REDDENED it, so it was wired to fail on its own improvement.
/// `UNGROUPED_IF_ARG` carries an argument-position `if` for exactly as long as
/// this node exists and no fix of this node can remove it.
#[test]
fn ac_inline_ken_census_reaches_rust_test_sources() {
    let tests_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let mut sites: Vec<(String, usize, String)> = Vec::new();

    for entry in std::fs::read_dir(&tests_dir).expect("tests dir").flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|e| e != "rs") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        for (number, line) in text.lines().enumerate() {
            // Test the KEN inside the string literals, not the Rust line. The
            // Rust syntax around it (`:`, `"`, `=`) is what let a Rust `match`
            // guard satisfy the previous version.
            for fragment in line.split('"').skip(1).step_by(2) {
                if has_argument_position_if(fragment) {
                    sites.push((name.clone(), number + 1, fragment.to_owned()));
                }
            }
        }
    }

    // The control is on the matching LINE, not the filename: the occurrence is
    // the evidence, and a file can stop carrying one without anything failing.
    assert!(
        sites.iter().any(|(_, _, frag)| frag.contains(UNGROUPED_IF_ARG)),
        "KNOWN-ANSWER CONTROL FAILED: the census must return the line carrying \
         UNGROUPED_IF_ARG. A census that misses a member it is guaranteed to \
         contain is blind, and its zero is not a measurement. Found: {sites:?}"
    );
}

/// An application head followed by an UNGROUPED `if` — the shape `32 §3`
/// forbids as an application argument.
///
/// Not `" if "` minus `"(if "`. That test admitted three populations it should
/// not have: a leading `if` (`= if c then ...`), an `else if` chain, and Rust's
/// own `match` guards. The head must be a run of ordinary identifiers, so a
/// keyword in it disqualifies the occurrence.
fn has_argument_position_if(fragment: &str) -> bool {
    const KEN_KEYWORDS: [&str; 10] = [
        "if", "then", "else", "let", "in", "match", "fn", "proc", "const", "where",
    ];
    let Some(equals) = fragment.find("= ") else {
        return false;
    };
    let rest = &fragment[equals + 2..];
    // Every occurrence, not just the first: an excluded one earlier in the
    // fragment must not hide a real one after it.
    rest.match_indices(" if ").any(|(at, _)| {
        let head = rest[..at].trim();
        !head.is_empty()
            && head
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == ' ')
            && head.split_whitespace().all(|t| !KEN_KEYWORDS.contains(&t))
    })
}

/// The predicate's discriminating table — the evidence that the repaired
/// control does not merely PASS but passes for its stated reason.
///
/// Each negative is a population that satisfied the previous version. They are
/// listed as cases rather than described, so a future loosening reddens here
/// instead of silently re-admitting them.
#[test]
fn argument_position_predicate_admits_only_the_forbidden_shape() {
    assert!(
        has_argument_position_if(UNGROUPED_IF_ARG),
        "the forbidden shape must be admitted"
    );
    assert!(has_argument_position_if("const k : Nat = f x if c then a else b"));

    for legal in [
        // leading `if` — legal, and the largest false population before
        "const when_true : Int = if True then 11 else 22",
        // `else if` chain — the head before ` if ` is all-alphanumeric, so
        // only the keyword check excludes it
        "const outer_else : Int = if False then 1 else if True then 2 else 3",
        // grouped — the migration target; must not be re-flagged
        "const k : Nat = keep (if c then a else b)",
        // `let` RHS in leading position
        "const let_if : Int = let x : Int = if False then 7 else 8 in x",
        // a RUST match guard, which the previous version counted as Ken
        "matches!(e, ElabError::AmbiguousReference { ref name, .. } if name == \"True\")",
        // no `if` at all
        "const plain : Nat = f x y",
    ] {
        assert!(
            !has_argument_position_if(legal),
            "must NOT be flagged as an argument-position `if`: {legal}"
        );
    }
}

