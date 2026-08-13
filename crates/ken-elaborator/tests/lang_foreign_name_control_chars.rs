//! LANG-FOREIGN-NAME-CONTROL-CHARS -- `foreign` symbol/library names reject
//! embedded Unicode control characters, at exactly the two `parse_foreign_decl`
//! extraction sites (`parser.rs`), never in the shared lexer. Architect
//! follow-up on LANG-SURFACE-LITERAL-ESCAPES: making `Token::Str`
//! escape-decoded made `\0` (and every other control character) newly
//! expressible inside these two names, a C-ABI name-truncation vector once a
//! loader consumer lands. Producer-side hygiene only -- not a well-formed-
//! C-symbol-name policy, no consumer exists today.

use ken_elaborator::ElabEnv;
use ken_elaborator::ElabError;

fn err_of(src: &str) -> ElabError {
    ElabEnv::new()
        .expect("prelude init")
        .elaborate_decl(src)
        .expect_err("must reject")
}

/// AC-1 (symbol half), AC-3: the rejecting fixture uses the source ESCAPE
/// `\0`, not a raw control byte -- `\0` in source is exactly the case
/// LANG-SURFACE-LITERAL-ESCAPES made newly expressible; a literal NUL byte
/// in the fixture file would exercise a path that was already reachable
/// and prove nothing about the regression.
#[test]
fn symbol_name_containing_escaped_nul_rejects() {
    let err = err_of(r#"foreign f : Int -> Int = "sym\0bol" "lib""#);
    match err {
        ElabError::ForeignNameControlCharacter { which, character, .. } => {
            assert_eq!(which, "symbol");
            assert_eq!(character, '\0');
        }
        other => panic!("expected ForeignNameControlCharacter, got {other:?}"),
    }
}

/// AC-1 (library half): a check on only the symbol name is the same defect
/// with a smaller surface, and the library name is the one that reaches a
/// loader path first.
#[test]
fn library_name_containing_escaped_nul_rejects() {
    let err = err_of(r#"foreign f : Int -> Int = "sym" "li\0b""#);
    match err {
        ElabError::ForeignNameControlCharacter { which, character, .. } => {
            assert_eq!(which, "library");
            assert_eq!(character, '\0');
        }
        other => panic!("expected ForeignNameControlCharacter, got {other:?}"),
    }
}

/// AC-2 -- the positive control, and the no-lexer mutation proof: an
/// ORDINARY string literal containing `\0` (nothing to do with `foreign`)
/// must still elaborate. This is what proves the check landed at the two
/// parse sites and not in `lexer.rs`'s shared decode path -- a rejection
/// test alone cannot distinguish "checked at the right site" from
/// "the lexer now refuses `\0` everywhere", and the latter would be an
/// unauthorized language change this frame exists to prevent.
#[test]
fn ordinary_string_literal_with_nul_still_elaborates() {
    let mut env = ElabEnv::new().expect("prelude init");
    env.elaborate_decl("const has_nul : String = \"a\\0b\"")
        .expect("an ordinary string literal containing \\0 must still elaborate");
}

/// A control character other than NUL also rejects -- the check is
/// "any Unicode control character", not a NUL-specific special case.
#[test]
fn non_nul_control_character_also_rejects() {
    // \u{7} is BEL, a C0 control character distinct from NUL.
    let err = err_of(r#"foreign f : Int -> Int = "sym\u{7}bol" "lib""#);
    assert!(matches!(err, ElabError::ForeignNameControlCharacter { character: '\u{7}', .. }));
}

/// AC-5: the span points at the offending string literal, not at the
/// `foreign` keyword or the declaration as a whole.
#[test]
fn span_points_at_the_offending_literal_not_the_foreign_keyword() {
    let src = r#"foreign f : Int -> Int = "sym\0bol" "lib""#;
    let literal_start = src.find('"').unwrap();
    let literal_end = src[literal_start + 1..].find('"').unwrap() + literal_start + 2;
    match err_of(src) {
        ElabError::ForeignNameControlCharacter { span, .. } => {
            assert_eq!(
                (span.start, span.end),
                (literal_start, literal_end),
                "span must cover exactly the symbol string literal"
            );
            assert!(span.start > 0, "span must not start at the `foreign` keyword");
        }
        other => panic!("expected ForeignNameControlCharacter, got {other:?}"),
    }
}

/// A `foreign` declaration whose names contain no control character is
/// unaffected -- the existing acceptance surface is unchanged.
#[test]
fn ordinary_foreign_names_still_accept() {
    ElabEnv::new()
        .expect("prelude init")
        .elaborate_decl(r#"foreign f : Int -> Int = "sqrt" "m" pure"#)
        .expect("an ordinary foreign declaration must still elaborate");
}
