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

use ken_elaborator::lexer::{Lexer, Token};
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

// Every behavioural fixture in this file, named once and shared with the
// one-directional sweep below. Hoisted deliberately: a second hand-written list
// in the sweep could drift from the fixtures the behavioural tests actually
// run, and "every fixture is swept" would become a claim instead of a property.
const GROUPED_IF_ARG: &str = "const k : Nat = keep (if c then a else b)";
const LEADING_IF: &str = "const k : Nat = if c then a else b";
const GUARDED_ARM: &str = "const k : Nat = match z { A x if c |-> b ; B |-> d }";
const UNGUARDED_ARM: &str = "const k : Nat = match z { A x |-> b }";

/// Fixtures whose behaviour other tests in this file assert. The sweep runs the
/// one-directional check over exactly these, so no fixture can be exercised
/// behaviourally without also being checked for a false negative.
const FILE_FIXTURES: [&str; 5] = [
    UNGROUPED_IF_ARG,
    GROUPED_IF_ARG,
    LEADING_IF,
    GUARDED_ARM,
    UNGUARDED_ARM,
];

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
    parse_decls(GROUPED_IF_ARG).expect("a GROUPED if is a legal application argument");
    parse_decls(LEADING_IF).expect("a LEADING if is dispatched before the argument loop");
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
    parse_decls(GUARDED_ARM).expect("a guarded match arm must still parse");
    parse_decls(UNGUARDED_ARM).expect("an unguarded arm must still parse");
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
/// **Driven by the real lexer, not a char-class approximation.** Three rounds
/// of this control failed the same way: `" if "` minus `"(if "` admitted
/// leading `if`, `else if` chains and Rust match guards; the repair then
/// dropped every dotted head; the repair after that would have dropped the six
/// reserved glyph names. A text heuristic has as many failure shapes as the
/// grammar has atom productions, and each fix was "add the spelling I just
/// found" rather than closing the class.
///
/// The structural characterisation closes it: **the token immediately before
/// `KwIf` must be one that ENDS AN ATOM.** Then
///
///   `keep if …`      Ident   before KwIf  -> forbidden
///   `M.f if …`       Ident   (dotted names lex as segments) -> forbidden
///   `f::g Bool if …` ConId   -> forbidden
///   `x ≤ y if …`     Ident   -> forbidden
///   `keep (if …)`    LParen  before KwIf  -> legal, grouped
///   `= if …`         Eq      -> legal, leading
///   `… else if …`    KwElse  -> legal, chain
///
/// and the spelling of the head never enters the test at all.
fn has_argument_position_if(fragment: &str) -> bool {
    let Ok(tokens) = Lexer::lex(fragment) else {
        // Not lexable as Ken (a Rust line, a `{placeholder}` template). Not a
        // Ken fixture of this shape.
        return false;
    };
    tokens
        .windows(2)
        .any(|pair| matches!(pair[1].0, Token::KwIf) && ends_an_atom(&pair[0].0))
}

/// Whether a token can END an application atom — i.e. whether an argument list
/// could legitimately continue after it.
///
/// Enumerated over the token alphabet rather than over source spellings, which
/// is what makes this closed against SPELLING: a name lexes to `Ident` /
/// `ConId` however it is written — plain, dotted, `::`-qualified.
///
/// **Every arm is backed by a parser-verified case, and two candidate arms were
/// REMOVED because the parser refuted them:**
///
///   `f ≤ if …`     the six glyph names, and `Operator` — parser ACCEPTS,
///                  so a glyph before `if` is NOT this shape. Including them
///                  made the predicate over-broad.
///   `f [1] if …`   `RBracket` — rejects from a DIFFERENT producer, so it is
///                  not evidence for this one.
///
/// Honest residual: `Nat`, `Str`, `CharLit` are verified representatives of the
/// literal class; `IntLit`, `FloatLit`, `DecimalLit`, `Float32Lit`, `ByteStr`
/// are the same atom production and are carried on that class argument, not on
/// their own cases.
fn ends_an_atom(token: &Token) -> bool {
    matches!(
        token,
        Token::Ident(_)
            | Token::ConId(_)
            // literals
            | Token::Nat(_)
            | Token::IntLit(_)
            | Token::FloatLit(_)
            | Token::DecimalLit(_, _)
            | Token::Float32Lit(_)
            | Token::Str(_)
            | Token::CharLit(_)
            | Token::ByteStr(_)
            // closers of a grouped / record atom
            | Token::RParen
            | Token::RBrace
    )
}

/// The discriminating table — now the ANTI-DEGENERACY guard.
///
/// Its role changed with the one-directional standard. Under "no false
/// negatives" alone, `fn has_argument_position_if(_) -> bool { true }` is
/// trivially SOUND and completely useless: it misses nothing because it flags
/// everything, and it would satisfy the soundness test, the corpus sweep and
/// the census control. **These negatives are the only thing standing between
/// the predicate and that degenerate implementation** — they bound the
/// false-positive rate that the soundness direction deliberately does not.
///
/// So the accepted false positive (match guards) is enumerated as a case, while
/// everything else legal stays asserted-negative. "Over-flagging is acceptable"
/// is a licence for ONE named shape, not a general one.
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
    // QUALIFIED heads. The flat char test dropped these silently; QA built the
    // first and it genuinely reaches the rejection.
    assert!(has_argument_position_if(
        "const k : Nat = Data.Numeric.Nat.Arithmetic if c then a else b"
    ));
    assert!(has_argument_position_if(
        "const k : Nat = Module.func if c then a else b"
    ));
    assert!(has_argument_position_if(
        "const k : Nat = compare_raw::eq_sound Bool if c then a else b"
    ));
    // QA's reserved-glyph fragment. NOTE what this does and does not test: the
    // token before `KwIf` is `y`, an Ident — so it exercises the Ident arm, not
    // any glyph arm. It was added believing it covered the glyphs, and a
    // mutation dropping every glyph arm left it GREEN, which is what exposed
    // both the decorative fixture and the wrongness of those arms.
    assert!(has_argument_position_if(
        "const k : Nat = x \u{2264} y if c then a else b"
    ));
    assert!(has_argument_position_if(
        "const k : Nat = f (g x) if c then a else b"
    ));

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
        // a keyword must not enter through a dotted spelling either
        "const k : Nat = if.then if c then a else b",
    ] {
        assert!(
            !has_argument_position_if(legal),
            "must NOT be flagged as an argument-position `if`: {legal}"
        );
    }
}

/// The closure argument, ONE-DIRECTIONAL: the predicate must never MISS a
/// forbidden-shape site. It is allowed to over-flag.
///
/// **This is a scope-of-requirement decision, not a weakened test.** The census
/// finds migration CANDIDATES for a reviewer; it does not assert ground truth.
/// A false positive costs one glance at a flagged line. A false NEGATIVE is the
/// failure this whole node exists to prevent — four sites were found by four
/// separate CI failures and never by a census.
///
/// Requiring exact equivalence instead would mean distinguishing
/// application-argument position from match-guard position, which needs real
/// bracket and pattern tracking — reimplementing enough of the parser to
/// guarantee a fifth and sixth false shape later. Three rounds of patching
/// `ends_an_atom` are the evidence for that.
///
/// So: `parser rejects with ARGUMENT_LOOP_MARKER` **implies** `predicate`.
/// The converse is not required and is documented where it fails.
#[test]
fn predicate_never_misses_a_forbidden_shape() {
    for fragment in [
        "const k : Nat = keep if c then a else b",
        "const k : Nat = f x if c then a else b",
        "const k : Nat = Data.Numeric.Nat.Arithmetic if c then a else b",
        "const k : Nat = Module.func if c then a else b",
        "const k : Nat = compare_raw::eq_sound Bool if c then a else b",
        "const k : Nat = x \u{2264} y if c then a else b",
        "const k : Nat = f (g x) if c then a else b",
        "const k : Nat = f 42 if c then a else b",
        "const k : Nat = f True if c then a else b",
        "const k : Nat = f \"s\" if c then a else b",
        "const k : Nat = f 'c' if c then a else b",
        "const k : Nat = f { a = 1 } if c then a else b",
        // legal or otherwise-rejected — no obligation either way, listed so a
        // future reader sees they were considered
        "const k : Nat = keep (if c then a else b)",
        "const when_true : Int = if True then 11 else 22",
        "const outer_else : Int = if False then 1 else if True then 2 else 3",
        "const let_if : Int = let x : Int = if False then 7 else 8 in x",
        "const plain : Nat = f x y",
        "const k : Nat = f \u{2264} if c then a else b",
        "const k : Nat = f [1] if c then a else b",
    ] {
        let parser_rejects_from_the_loop = match parse_decls(fragment) {
            Err(ElabError::ParseError { msg, .. }) => msg.contains(ARGUMENT_LOOP_MARKER),
            _ => false,
        };
        if parser_rejects_from_the_loop {
            assert!(
                has_argument_position_if(fragment),
                "FALSE NEGATIVE: the parser rejects this from the argument loop \
                 and the census would not flag it: {fragment}"
            );
        }
    }
}

/// A DOCUMENTED, ACCEPTED false positive.
///
/// `A x if c |-> b` is a legal match-arm guard. The token before `KwIf` is `x`,
/// an Ident, so the predicate flags it — and separating this from an
/// application argument needs the pattern context the predicate deliberately
/// does not track.
///
/// **This fixture asserts the over-flagging rather than eliminating it**, so the
/// behaviour is a recorded property instead of a surprise. A reviewer reading
/// census output sees a guard, recognises it, moves on. If that ever stops
/// being acceptable — if the census gains a consumer that acts on its output
/// without a reader — this test is the place that has to change, and it will
/// fail loudly rather than silently mislead.
#[test]
fn match_guards_are_an_accepted_false_positive() {
    let guard = GUARDED_ARM;
    assert!(
        parse_decls(guard).is_ok(),
        "the fixture must be LEGAL, or it is not a false positive"
    );
    assert!(
        has_argument_position_if(guard),
        "if the predicate stops flagging this, the accepted-false-positive note \
         is stale and should be removed"
    );
}

/// No false negative anywhere in the test corpus — the sweep, not the table.
///
/// The table above is fragments I thought of. This walks every Ken fragment in
/// every `tests/*.rs` file and holds the same one-directional standard against
/// the population that actually exists. QA's point that a sweep must cover
/// EXISTING fixtures and not just new ones is what this discharges.
#[test]
fn no_false_negative_across_the_test_corpus() {
    let tests_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let mut checked = 0usize;
    let mut misses: Vec<String> = Vec::new();

    for entry in std::fs::read_dir(&tests_dir).expect("tests dir").flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|e| e != "rs") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        for line in text.lines() {
            for fragment in line.split('"').skip(1).step_by(2) {
                let rejected_from_loop = match parse_decls(fragment) {
                    Err(ElabError::ParseError { msg, .. }) => msg.contains(ARGUMENT_LOOP_MARKER),
                    _ => false,
                };
                if rejected_from_loop {
                    checked += 1;
                    if !has_argument_position_if(fragment) {
                        misses.push(format!("{}: {fragment}", path.display()));
                    }
                }
            }
        }
    }

    assert!(misses.is_empty(), "FALSE NEGATIVES in the corpus: {misses:#?}");
    // Positive control: a sweep that examined nothing proves nothing.
    assert!(
        checked > 0,
        "the sweep found NO forbidden-shape fragment anywhere, so it cannot have \
         demonstrated the absence of misses — the instrument did not reach"
    );
}

/// QA condition (2): EVERY fixture this file exercises behaviourally, run
/// through the one-directional check — not only the match guard QA found.
///
/// It iterates `FILE_FIXTURES`, the same constants the behavioural tests use,
/// so a fixture cannot be added to this file, asserted on, and left out of the
/// soundness check. The classification of each is printed so a reader can see
/// which are forbidden-shape and which are legal, rather than inferring it.
#[test]
fn every_file_fixture_passes_the_one_directional_check() {
    let mut forbidden = 0usize;
    for fixture in FILE_FIXTURES {
        let rejected_from_loop = match parse_decls(fixture) {
            Err(ElabError::ParseError { msg, .. }) => msg.contains(ARGUMENT_LOOP_MARKER),
            _ => false,
        };
        if rejected_from_loop {
            forbidden += 1;
            assert!(
                has_argument_position_if(fixture),
                "FALSE NEGATIVE on a fixture this file asserts behaviour for: {fixture}"
            );
        }
        println!(
            "  {:<12} predicate={:<5} {}",
            if rejected_from_loop { "FORBIDDEN" } else { "legal" },
            has_argument_position_if(fixture),
            fixture
        );
    }
    // Reach control: if none of the file's fixtures is forbidden-shape, this
    // test asserted nothing and must not read as evidence.
    assert!(
        forbidden > 0,
        "no fixture in FILE_FIXTURES is forbidden-shape, so the one-directional \
         check was vacuous over all of them"
    );
}

