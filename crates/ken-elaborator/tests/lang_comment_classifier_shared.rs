//! LANG-COMMENT-CLASSIFIER-SHARED -- the semantic `Lexer::skip_ws_comments`
//! and the lossless `append_trivia` rescan now call one shared classifier
//! (`lexer::classify_comment`) instead of each reimplementing comment
//! dispatch, so their agreement about a comment's kind and end is
//! structural rather than merely tested-against. This file holds the D4
//! residual the Architect named separately: a direct offset comparison
//! that survives even if the classifier's internal shape changes.
//!
//! `LANG-SURFACE-BLOCK-COMMENTS`'s own `lex_agrees_with_lossless` compares
//! only `is_ok()` booleans, so it is blind to a divergence that flips
//! neither acceptance; `reconstruct() == src` is byte-preserving under
//! either scanner's attribution of the same bytes, so it is blind too.
//! Both a same-acceptance/different-end divergence and a same-acceptance/
//! different-kind divergence need a witness that reads an actual offset or
//! an actual kind, which is what AC-3 below (offsets) and the amended
//! `ac8_doc_block_opener_closer_edge_cases` in the predecessor's file
//! (kinds, via accept/reject) both are.

use ken_elaborator::lexer::Lexer;
use ken_elaborator::lossless::parse_lossless;

/// AC-3 -- for a comment placed at the very start of the source, the
/// lexer's own view of "where the comment ends" is the start of the very
/// next token it produces (nothing else can sit between them, since
/// `skip_ws_comments` only stops once `classify_comment` returns `None`);
/// the lossless layer's view is the `end` of the trivia entry it records
/// for that same comment. This asserts the two offsets are equal,
/// directly -- not via `is_ok()`, and not via round-trip reconstruction.
#[test]
fn ac3_lexer_and_lossless_comment_end_are_the_same_offset() {
    for src in [
        "{- {- inner -} outer -}const t : Int = 1",
        "{- {- {- triple -} -} -}const t : Int = 1",
        "{-- doc --}const t : Int = 1",
        "{-- {- not nested: ordinary body inside a doc block -} still doc --}const t : Int = 1",
        "-- line comment, nothing after it",
        "--- doc line comment, nothing after it",
    ] {
        let tokens = Lexer::lex(src).unwrap_or_else(|e| panic!("{src:?} must lex: {e:?}"));
        let lexer_end = tokens
            .first()
            .unwrap_or_else(|| panic!("{src:?} must produce at least the Eof token"))
            .1
            .start;
        let source = parse_lossless(src).unwrap_or_else(|e| panic!("{src:?} must parse: {e:?}"));
        let comment = source
            .trivia()
            .iter()
            .find(|t| t.span.start == 0)
            .unwrap_or_else(|| panic!("{src:?} must retain a leading comment as trivia"));
        assert_eq!(
            comment.span.end, lexer_end,
            "{src:?}: lossless comment end must equal the lexer's own next-token start"
        );
    }
}

/// AC-7 (severable rider) -- strengthen the sibling
/// `LANG-FOREIGN-NAME-CONTROL-CHARS` positive control, which was keyed on
/// absence-of-error where the property is presence-of-value (Adversary
/// finding `evt_cxbze6z3yns8`). `const` dispatches to `parse_view_decl`, so
/// a `const` declaration is a `Decl::ViewDecl` whose `body` is
/// `Expr::EStr`; assert that decoded value directly, matching the idiom
/// `d0_foreign_names_decode_escapes_uniformly` already uses one file over.
/// No new accessor.
#[test]
fn ac7_ordinary_string_positive_control_asserts_the_decoded_value() {
    use ken_elaborator::parser::parse_decls;
    use ken_elaborator::{Decl, Expr};

    let decls = parse_decls("const has_nul : String = \"a\\0b\"")
        .expect("an ordinary string literal containing \\0 must still parse");
    assert_eq!(decls.len(), 1);
    match &decls[0] {
        Decl::ViewDecl {
            body: Expr::EStr(value, _),
            ..
        } => {
            assert_eq!(
                value, "a\0b",
                "the decoded string value must retain the NUL byte"
            );
        }
        other => panic!("expected a ViewDecl with an EStr body, got {other:?}"),
    }
}
