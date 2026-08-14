//! LANG-TRIVIA-KIND-MAPPING-PIN -- `From<CommentKind> for TriviaKind`
//! (`lossless.rs:45-54`) is the sole place a classification becomes a
//! behaviour; the compiler closes the completeness axis (no `_` arm) but the
//! per-arm mapping is asserted nowhere. `ac3` in
//! `lang_surface_block_comments.rs` pins the doc rule on a configuration
//! where the doc rule and the positional heuristic agree (comment on its own
//! line), so a `Block`/`DocBlock` or `Line`/`DocLine` transposition compiles
//! and reds nothing there. This file pins the DISCRIMINATING configuration:
//! a doc comment on the SAME LINE after a declaration, with a following
//! declaration -- there the doc rule (`Leading`, following) and the
//! positional heuristic (`Trailing`, preceding) split.

use ken_elaborator::lossless::{parse_lossless, CommentPlacement};

/// D1/AC-1 -- same-line-after doc-comment attachment, both doc forms. The
/// doc rule returns before the positional `same_line_after` check is ever
/// reached, so this must attach `Leading` to the FOLLOWING declaration even
/// though the comment sits on the same line as the PRECEDING one -- the
/// opposite of what the positional heuristic alone would give.
#[test]
fn d1_same_line_after_doc_comment_attaches_leading_to_following_declaration() {
    for (doc_open, doc_close) in [("---", ""), ("{--", " --}")] {
        let src = format!("const a : Int = 1 {doc_open} doc for b{doc_close}\nconst b : Int = 2\n");
        let source = parse_lossless(&src).unwrap_or_else(|e| panic!("{src:?}: {e:?}"));
        let doc_start = src.find(doc_open).unwrap();
        let decl_span = |prefix: &str| {
            source
                .typed_decls()
                .iter()
                .find(|d| src[d.span().start..d.span().end].starts_with(prefix))
                .unwrap_or_else(|| panic!("decl `{prefix}` must exist in {src:?}"))
                .span()
                .clone()
        };
        let a_span = decl_span("const a");
        let b_span = decl_span("const b");
        let attachment = source
            .comment_attachments()
            .iter()
            .find(|a| a.comment_span.start == doc_start)
            .unwrap_or_else(|| panic!("doc comment in {src:?} must have an attachment"));
        assert_eq!(attachment.placement, CommentPlacement::Leading, "{src:?}");
        assert_eq!(
            attachment.home_span, b_span,
            "{src:?}: same-line-after doc comment must still attach to the FOLLOWING decl (b)"
        );
        assert_ne!(
            attachment.home_span, a_span,
            "{src:?}: must NOT fall through to the positional Trailing/preceding answer"
        );
    }
}
