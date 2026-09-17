//! Acceptance for `LANG-BARE-OPERATOR-ATOM-REJECTION`.
//!
//! `spec/30-surface/32-grammar.md` Section 3: an ungrouped `operator_name` is
//! admitted only as an `operator_prefix` head with at least one following atom.
//! With zero following atoms it rejects AT ITS LEADING TOKEN.
//!
//! WHY THESE TESTS ASSERT AUTHORSHIP AND NOT ONLY POSITION. Before this node,
//! a bare `<+>` parsed to `EVar` and the leftover tokens were reported by some
//! later production -- and a stray-token error is reported AT THE STRAY TOKEN,
//! so `span.start` is IDENTICAL under the defective and the correct
//! implementation. A span is a coordinate; the requirement is about which
//! production raised the error. The predecessor node on this pin shipped
//! exactly that defect with a green `span.start` assertion on top of it.
//!
//! Every rejection test therefore asserts a PAIR: the span start AND the
//! producing production, the latter via a marker only the atom parser emits.
//! `DECL_PARSER_SIGNATURE` is the negative half -- the message that the
//! defective implementation produces at the same coordinate.

use ken_elaborator::parser::parse_decls;
use ken_elaborator::ElabError;

/// Emitted only by the atom parser's `operator_name` arm.
const ATOM_PRODUCTION_MARKER: &str = "is an operator name, not an atom";

/// Emitted by the file-level declaration parser when it trips over tokens a
/// previous production declined to consume. This is what the DEFECTIVE
/// implementation reports, at the same coordinate as the correct one.
const DECL_PARSER_SIGNATURE: &str = "expected 'const'";

fn reject(src: &str) -> (String, usize) {
    match parse_decls(src) {
        Ok(decls) => panic!("expected a rejection, parsed {decls:?}: {src}"),
        Err(ElabError::ParseError { msg, span }) => (msg, span.start),
        Err(other) => panic!("expected a ParseError, got {other:?}: {src}"),
    }
}

/// The divergent shape in eight distinct ENCLOSURES -- and, read by PARSE
/// POSITION, all eight are the same position: expression-initial. Each reaches
/// the atom parser's `operator_name` arm because the operator STARTS a
/// sub-expression. A fix closing only the declaration body fails these; a fix
/// closing only the expression-initial position passes all eight.
///
/// That is stated plainly because the enclosure axis is the tempting one and it
/// is not the axis that matters. The OTHER position an ungrouped operator could
/// occupy -- trailing an application head -- is covered by
/// `ARGUMENT_POSITION`, below, and it does NOT reach this arm.
const DIVERGENT: [(&str, &str); 8] = [
    ("decl body", "const k : Nat = <+>"),
    ("decl body, glyph", "const k : Nat = \u{2264}"),
    ("let binding value", "const k : Nat = let z = <+> in z"),
    ("lambda body", "const k : Nat = \\ x . <+>"),
    ("if branch", "const k : Nat = if c then <+> else g"),
    ("match arm body", "const k : Nat = match x { A |-> <+> }"),
    ("record field", "const k : Nat = { a = <+> }"),
    ("tuple component", "const k : Nat = f (a, <+>)"),
];

/// Forms this node must leave working. The first two are the whole reason the
/// grouped case is a separate production rather than an `RParen` lookahead:
/// `(a, <+>)` above also has `)` after the operator, and must still reject.
const PRESERVED: [(&str, &str); 11] = [
    ("grouped", "const k : Nat = (<+>)"),
    ("grouped glyph", "const k : Nat = (\u{2264})"),
    ("doubly grouped", "const k : Nat = ((<+>))"),
    ("prefix head, one atom", "const k : Nat = <+> Zero"),
    ("prefix head, two atoms", "const k : Nat = <+> Zero One"),
    ("prefix head, glyph", "const k : Nat = \u{2264} Zero One"),
    ("infix spine", "const k : Nat = x <+> y"),
    ("infix spine, glyph", "const k : Nat = x \u{2264} y"),
    (
        "infix spine, two operators",
        "const k : Nat = x \u{2264} y \u{2227} z",
    ),
    ("grouped then applied", "const k : Nat = (<+>) a b"),
    (
        "grouped as an argument",
        "const k : Nat = map (\u{2264}) xs",
    ),
];

/// The trailing-argument position, `f <+>`. This is the other half of the
/// conformance row's ground ("neither is an `application_atom` or a complete
/// `operator_prefix`"), and it does NOT route through the narrowed arm.
///
/// MEASURED, and it is neither of the two paths one would guess:
///
///     f <+>       REJECTS  "expected an expression, found Eof"  span 21
///                          (the operator sits at 18)
///     f <+> g     PARSES   EInfixSpine { operands: [f, g],
///                                        operators: [User("<+>")] }
///
/// The second line is the one that explains the first. `can_start_atom_expr`
/// does not admit an operator token, so the argument loop BREAKS rather than
/// taking `<+>` as an argument; `parse_mixed_infix_expr` then claims the token
/// as an INFIX OPERATOR and fails looking for its right operand.
///
/// So there is no "operator in argument position" reading in this grammar to
/// reject: `f <+>` is an INCOMPLETE INFIX EXPRESSION. The rejection is real but
/// it is raised elsewhere, at the missing operand rather than at the operator.
///
/// This is pinned rather than left unmeasured because the conformance row cites
/// `application_atom`, and because an argument-loop change would silently move
/// these.
const ARGUMENT_POSITION: [(&str, &str); 4] = [
    ("bare, at eof", "const k : Nat = f <+>"),
    ("bare glyph, at eof", "const k : Nat = f \u{2264}"),
    ("identifier head", "const k : Nat = keep <+>"),
    ("inside a group", "const k : Nat = f (g <+>)"),
];

#[test]
fn ac_argument_position_rejects_but_not_via_this_arm() {
    for (what, src) in ARGUMENT_POSITION {
        let (msg, start) = reject(src);
        let op = src
            .find("<+>")
            .or_else(|| src.find('\u{2264}'))
            .expect("fixture has an operator");

        assert!(
            !msg.contains(ATOM_PRODUCTION_MARKER),
            "{what}: expected this position NOT to reach the operator_name arm, \
             but it did -- the grammar changed and the conformance row's \
             narrowing needs re-reading: {msg:?}"
        );
        assert!(
            !msg.contains(DECL_PARSER_SIGNATURE),
            "{what}: leftover tokens reached the declaration parser: {msg:?}"
        );
        assert!(
            start > op,
            "{what}: rejected at {start}, at or before the operator at {op}; \
             this position rejects at the MISSING OPERAND, which is after it"
        );
    }

    // The discriminator, and the reason the rows above read as they do: the
    // same token between two atoms is an infix operator. That is why it is
    // never available as an argument, and why `f <+>` is an incomplete infix
    // expression rather than an operator misused as an atom.
    let decls = parse_decls("const k : Nat = f <+> g").expect("infix must parse");
    let rendered = format!("{decls:?}");
    assert!(
        rendered.contains("EInfixSpine"),
        "if this stops being an infix spine, ARGUMENT_POSITION's whole \
         explanation is void: {rendered}"
    );
}

#[test]
fn ac_bare_operator_rejects_affirmatively() {
    for (position, src) in DIVERGENT {
        let (msg, _) = reject(src);
        assert!(
            msg.contains(ATOM_PRODUCTION_MARKER),
            "{position}: rejected, but not by the atom parser -- {msg:?}"
        );
    }
}

/// AC-SPAN. The load-bearing one. Both halves in one assertion per fixture,
/// because either alone passes under the defect: the coordinate is shared, and
/// authorship without a position does not show it rejected at the LEADING
/// token rather than somewhere later in the same expression.
#[test]
fn ac_span_rejection_is_at_the_leading_token_and_raised_by_the_atom_parser() {
    for (position, src) in DIVERGENT {
        let expected = src
            .find("<+>")
            .or_else(|| src.find('\u{2264}'))
            .unwrap_or_else(|| panic!("{position}: fixture has no operator"));

        let (msg, start) = reject(src);

        assert_eq!(
            start, expected,
            "{position}: rejected at {start}, not at the operator's leading token {expected}"
        );
        assert!(
            msg.contains(ATOM_PRODUCTION_MARKER),
            "{position}: right coordinate, wrong production -- {msg:?}"
        );
        assert!(
            !msg.contains(DECL_PARSER_SIGNATURE),
            "{position}: this is the leftover-token error the defect produces \
             at this same coordinate -- {msg:?}"
        );
    }
}

#[test]
fn ac_grouped_and_prefix_head_forms_are_preserved() {
    for (what, src) in PRESERVED {
        let decls = parse_decls(src)
            .unwrap_or_else(|e| panic!("{what}: must still parse, got {e:?} -- {src}"));
        assert!(!decls.is_empty(), "{what}: parsed to nothing -- {src}");
    }
}

/// The infix path at `parse_mixed_infix_expr` is out of scope and must be
/// byte-unchanged in effect. An infix operator is not an operand, so it never
/// routes through the narrowed atom arm; this pins that, because if it ever
/// did, every infix expression in the corpus would reject.
#[test]
fn ac_infix_path_untouched() {
    for src in [
        "const k : Nat = x <+> y",
        "const k : Nat = x \u{2264} y",
        "const k : Nat = x \u{2264} y \u{2227} z",
    ] {
        let decls = parse_decls(src).unwrap_or_else(|e| panic!("infix must parse: {e:?} -- {src}"));
        let rendered = format!("{decls:?}");
        assert!(
            rendered.contains("EInfixSpine"),
            "an operator between atoms is an infix spine, not an atom: {src}"
        );
        assert!(
            !rendered.contains("EVar(\"<+>\")") && !rendered.contains("EVar(\"\u{2264}\")"),
            "the infix operator became an OPERAND, which means it routed \
             through the atom arm: {src}"
        );
    }
}

/// Anti-degeneracy. Every fixture above is a `const` declaration, so a change
/// that broke `const` parsing outright would redden the preserved rows and
/// could be mistaken for the divergent rows passing for the right reason.
/// These two rows fix the baseline: ordinary code with no operator at all is
/// untouched in both directions.
#[test]
fn ac_baseline_unrelated_sources_are_unaffected() {
    for src in [
        "const k : Nat = f a b",
        "const k : Nat = a + b",
        "const k : Nat = let z = f a in z",
    ] {
        parse_decls(src).unwrap_or_else(|e| panic!("baseline must parse: {e:?} -- {src}"));
    }
    // And a genuinely malformed source still fails, so "everything parses" is
    // not how the preserved rows are passing.
    assert!(parse_decls("const k : Nat =").is_err());
}
