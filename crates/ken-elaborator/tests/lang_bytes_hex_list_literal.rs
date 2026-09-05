//! `LANG-BYTES-HEX-LIST-LITERAL` lexer and elaboration acceptance.
//!
//! Spec sources: `31-lexical.md §3` and `38-ffi-io.md §1.1` / AC1.
//! Promise class: durable invariants. Alternate layout inside the brackets
//! remains a spec question; these tests pin the safe contiguous surface only.

use ken_elaborator::lexer::{Lexer, Token};
use ken_elaborator::{ElabEnv, ElabError, NumericLitVal};
use ken_kernel::{Decl as KernelDecl, GlobalId, Term};

fn tokens(source: &str) -> Vec<Token> {
    Lexer::lex(source)
        .expect("source must lex")
        .into_iter()
        .map(|(token, _)| token)
        .collect()
}

fn lexical_error(source: &str) -> String {
    match Lexer::lex(source) {
        Err(ElabError::ParseError { msg, .. }) => msg,
        other => panic!("expected a lexical ParseError for {source:?}, got {other:?}"),
    }
}

fn assert_bytes_literal(env: &ElabEnv, id: GlobalId, expected: &[u8]) {
    let bytes_ty = Term::const_(env.globals["Bytes"], vec![]);
    let Some(KernelDecl::Transparent { ty, body, .. }) = env.env.lookup(id) else {
        panic!("Bytes literal declaration must be transparent")
    };
    assert_eq!(ty, &bytes_ty, "literal must retain exact Bytes type");
    let Term::Const {
        id: literal,
        level_args,
    } = body
    else {
        panic!("Bytes literal body must be one checked literal constant, got {body:?}")
    };
    assert!(
        level_args.is_empty(),
        "Bytes literal carries no universe arguments"
    );
    match env.num_values.get(literal) {
        Some(NumericLitVal::Bytes(bytes)) => assert_eq!(
            bytes, expected,
            "elaborated literal must carry the exact byte vector"
        ),
        other => panic!("literal store must contain Bytes, got {other:?}"),
    }
}

/// MEASURED: lower/upper `x` prefixes and lower/upper digits all produce the
/// existing ByteStr discriminant and exact vector, including the zero-pair
/// case. CLAIMED: the bracket discriminator composes with the existing
/// case-insensitive `0x` dispatch. THE GAP: token identity alone does not prove
/// downstream Bytes elaboration; the following test inspects the kernel form.
#[test]
fn bracketed_hex_pairs_lex_to_existing_byte_string_token() {
    for source in ["0x[deadbeef]", "0X[DEADBEEF]"] {
        assert_eq!(
            tokens(source),
            vec![Token::ByteStr(vec![0xde, 0xad, 0xbe, 0xef]), Token::Eof]
        );
    }
    assert_eq!(tokens("0x[]"), vec![Token::ByteStr(vec![]), Token::Eof]);
}

/// MEASURED: bracketed and quoted spellings elaborate to transparent constants
/// of exact type Bytes whose literal stores carry the same vector. CLAIMED:
/// `0x[…]` is only a second spelling of the existing Bytes value. THE GAP:
/// literal-store equality does not assert runtime formatting, which is outside
/// this lexer-only WP.
#[test]
fn bracketed_and_quoted_bytes_have_the_same_elaborated_type_and_value() {
    let mut env = ElabEnv::new().expect("base environment");
    let bracketed = env
        .elaborate_decl("const bracketedBytes : Bytes = 0x[deadbeef]")
        .expect("bracketed Bytes literal must elaborate");
    let quoted = env
        .elaborate_decl(r#"const quotedBytes : Bytes = b"\xde\xad\xbe\xef""#)
        .expect("quoted Bytes control must elaborate");

    assert_bytes_literal(&env, bracketed, &[0xde, 0xad, 0xbe, 0xef]);
    assert_bytes_literal(&env, quoted, &[0xde, 0xad, 0xbe, 0xef]);
}

/// MEASURED: the one-character `[` fork leaves both unbracketed numeric paths
/// on their established token kinds and values. CLAIMED: Bytes recognition
/// does not conflate the normative Int/Float spellings. THE GAP: these are the
/// direct collision cases; the existing numeric suites retain their broader
/// boundary coverage and are run alongside this test.
#[test]
fn unbracketed_hex_integer_and_float_forms_are_unchanged() {
    assert_eq!(tokens("0xFF"), vec![Token::Nat(255), Token::Eof]);
    assert_eq!(tokens("0Xff"), vec![Token::Nat(255), Token::Eof]);
    assert_eq!(tokens("0x1p-3"), vec![Token::FloatLit(0.125), Token::Eof]);
    assert_eq!(tokens("0X1P-3"), vec![Token::FloatLit(0.125), Token::Eof]);
}

/// MEASURED: each malformed class reaches its dedicated byte-list diagnostic,
/// while internal whitespace follows the frame's safe strict-contiguous
/// interpretation. CLAIMED: malformed Bytes syntax is never mislabeled as a
/// radix integer. THE GAP: whether whitespace should eventually be admitted is
/// normatively unsettled and this rejection does not resolve that question.
#[test]
fn malformed_hex_byte_lists_name_the_bytes_form_specifically() {
    let cases = [
        (
            "0x[abc]",
            "hex byte-list literal requires an even number of digits",
        ),
        (
            "0x[dg]",
            "invalid character 'g' in hex byte-list literal; expected contiguous hexadecimal digits",
        ),
        (
            "0x[de ad]",
            "invalid character ' ' in hex byte-list literal; expected contiguous hexadecimal digits",
        ),
        ("0x[", "unterminated hex byte-list literal"),
    ];
    for (source, expected) in cases {
        let msg = lexical_error(source);
        assert_eq!(msg, expected, "wrong diagnostic for {source:?}");
        assert!(
            !msg.contains("radix integer"),
            "Bytes spelling must not be diagnosed as an integer"
        );
    }
}
