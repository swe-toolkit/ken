//! Lexer-level unit coverage for `31 §3`'s escape scanner
//! (LANG-SURFACE-LITERAL-ESCAPES), complementary to
//! `literal_escapes_conformance.rs`'s seed-mirroring acceptance tests: exact
//! token payloads and exact `InvalidEscape` spans, one property per test,
//! independent of elaboration.

use ken_elaborator::{ElabError, ElabEnv};
use ken_elaborator::lexer::{Lexer, Token};

fn lex_one(src: &str) -> Token {
    let toks = Lexer::lex(src).expect("must lex");
    assert_eq!(toks.len(), 2, "expected exactly one token + Eof, got {toks:?}");
    toks[0].0.clone()
}

fn invalid_escape_span(src: &str) -> (usize, usize) {
    match Lexer::lex(src).unwrap_err() {
        ElabError::InvalidEscape { span, .. } => (span.start, span.end),
        other => panic!("expected InvalidEscape, got {other:?} for {src:?}"),
    }
}

#[test]
fn common_escapes_decode_in_every_kind() {
    assert_eq!(lex_one(r#""a\nb""#), Token::Str("a\nb".to_string()));
    assert_eq!(lex_one(r#"'\n'"#), Token::CharLit('\n'));
    assert_eq!(lex_one(r#"b"a\tb""#), Token::ByteStr(vec![b'a', 0x09, b'b']));
}

#[test]
fn unicode_escape_decodes_in_string_and_char() {
    assert_eq!(lex_one(r#""\u{1F600}""#), Token::Str("\u{1F600}".to_string()));
    assert_eq!(lex_one(r#"'\u{41}'"#), Token::CharLit('A'));
}

#[test]
fn x41bc_does_not_greedily_consume_a_third_digit() {
    assert_eq!(
        lex_one(r#"b"\x41BC""#),
        Token::ByteStr(vec![0x41, b'B', b'C'])
    );
}

#[test]
fn unrecognized_discriminator_span_is_exactly_backslash_and_char() {
    let (start, end) = invalid_escape_span(r#""\q""#);
    assert_eq!((start, end), (1, 3));
}

#[test]
fn wrong_kind_well_shaped_escape_spans_the_complete_escape() {
    let (start, end) = invalid_escape_span(r#"b"\u{41}""#);
    assert_eq!((start, end), (2, 8));
}

#[test]
fn incomplete_escape_takes_precedence_over_unterminated() {
    let (start, end) = invalid_escape_span("\"\\u{41");
    assert_eq!((start, end), (1, 6));
}

#[test]
fn ordinary_unterminated_literal_is_unaffected() {
    match Lexer::lex("\"abc").unwrap_err() {
        ElabError::ParseError { msg, .. } => assert!(msg.contains("unterminated")),
        other => panic!("expected ordinary unterminated ParseError, got {other:?}"),
    }
}

#[test]
fn raw_triple_string_performs_no_escape_processing() {
    let toks = Lexer::lex("\"\"\"\\n\\q\\u{D800}\\xGG\\\\\"\"\"").expect("must lex");
    assert_eq!(toks[0].0, Token::Str("\\n\\q\\u{D800}\\xGG\\\\".to_string()));
}

#[test]
fn char_cardinality_rejects_empty_and_multi_scalar() {
    Lexer::lex("''").unwrap_err();
    Lexer::lex("'ab'").unwrap_err();
}

#[test]
fn malformed_unicode_escape_spans_end_at_the_offending_character() {
    // `\u{}` -- span exactly `\u{}` (empty is revealed by the closing brace,
    // so it is included).
    assert_eq!(invalid_escape_span("\"\\u{}\""), (1, 5), "\\u{{}}");
    // `\u{0000041}` -- 7 digits; span excludes the closing brace.
    assert_eq!(invalid_escape_span("\"\\u{0000041}\""), (1, 11), "\\u{{0000041}}");
    // `\u{4_}` -- the underscore is consumed and included; brace excluded.
    assert_eq!(invalid_escape_span("\"\\u{4_}\""), (1, 6), "\\u{{4_}}");
    // `\u{G}` -- the first non-hex character is consumed and included.
    assert_eq!(invalid_escape_span("\"\\u{G}\""), (1, 5), "\\u{{G}}");
}

#[test]
fn well_shaped_invalid_scalar_spans_the_complete_escape() {
    assert_eq!(invalid_escape_span("\"\\u{D800}\""), (1, 9), "\\u{{D800}}");
    assert_eq!(invalid_escape_span("\"\\u{DFFF}\""), (1, 9), "\\u{{DFFF}}");
    assert_eq!(invalid_escape_span("\"\\u{110000}\""), (1, 11), "\\u{{110000}}");
}

#[test]
fn byte_escape_malformed_spans_end_at_the_offending_character() {
    // `b"\x4"` -- string closes right after the first digit; boundary excluded.
    assert_eq!(invalid_escape_span("b\"\\x4\""), (2, 5), "\\x4 (incomplete)");
    // `b"\xG0"` -- span excludes the trailing 0.
    assert_eq!(invalid_escape_span("b\"\\xG0\""), (2, 5), "\\xG");
}

#[test]
fn incomplete_escape_three_leg_matrix() {
    // String/delimiter leg.
    assert_eq!(invalid_escape_span("\"\\u{41\""), (1, 6));
    // Char/line-boundary leg -- backslash immediately followed by a newline.
    let err = Lexer::lex("'\\\n").unwrap_err();
    match err {
        ElabError::InvalidEscape { span, .. } => {
            assert_eq!((span.start, span.end), (1, 2), "span must be exactly the backslash");
        }
        other => panic!("expected InvalidEscape, got {other:?}"),
    }
    // Bytes/EOF leg -- input ends with no closing quote at all.
    assert_eq!(invalid_escape_span("b\"\\x4"), (2, 5));
}

#[test]
fn no_pending_escape_twins_stay_ordinary_unterminated() {
    for src in ["\"abc", "'abc", "b\"abc"] {
        match Lexer::lex(src).unwrap_err() {
            ElabError::ParseError { msg, .. } => {
                assert!(msg.contains("unterminated"), "{src:?} -> {msg}");
            }
            other => panic!("{src:?} must stay ordinary-unterminated, got {other:?}"),
        }
    }
}

#[test]
fn char_and_bytes_elaborate_end_to_end() {
    let mut env = ElabEnv::new().expect("base env");
    env.elaborate_decl("const lit_char : Char = 'A'")
        .expect("char literal must elaborate");
    assert!(env.globals.contains_key("lit_char"));

    let mut env = ElabEnv::new().expect("base env");
    env.elaborate_decl(r#"const lit_bytes : Bytes = b"\x41BC""#)
        .expect("byte string literal must elaborate");
    assert!(env.globals.contains_key("lit_bytes"));

    let mut env = ElabEnv::new().expect("base env");
    env.elaborate_decl(r#"const lit_str : String = "hi\n""#)
        .expect("ordinary string literal must still elaborate");
}
