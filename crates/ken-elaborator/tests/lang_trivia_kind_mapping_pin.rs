//! LANG-TRIVIA-KIND-MAPPING-PIN -- `From<CommentKind> for TriviaKind`
//! (`lossless.rs:45-54`) is the sole place a classification becomes a
//! behaviour; the compiler closes the completeness axis (no `_` arm) but the
//! per-arm mapping was asserted nowhere. `ac3` in
//! `lang_surface_block_comments.rs` pins the doc rule on a configuration
//! where the doc rule and the positional heuristic agree (comment on its own
//! line), so a `Block`/`DocBlock` or `Line`/`DocLine` transposition compiles
//! and reds nothing there. The DISCRIMINATING configuration is a comment on
//! the SAME LINE after a declaration, with a following declaration -- there
//! the doc rule (`Leading`, following) and the positional heuristic
//! (`Trailing`, preceding) split, so only one mechanism can be responsible
//! for the observed answer.
//!
//! This file establishes CLASS membership in that configuration, not
//! per-arm identity: `d1` shows both doc forms (`DocLine`, `DocBlock`) take
//! the doc rule; `d2` shows an ordinary form (`Block`) takes the positional
//! rule. `CommentKind` is `pub(crate)` (`lexer.rs`), so an integration test
//! in `crates/ken-elaborator/tests/` cannot name it and cannot assert the
//! `From` impl's arms directly -- behaviour alone separates a cross-class
//! pair (doc vs. ordinary) but not a within-class one (`DocLine`/`DocBlock`,
//! or `Line`/`Block`).
//!
//! **The arm-level claim -- all four `CommentKind` variants mapped to their
//! distinct `TriviaKind`, asserted directly, one assertion per arm -- lives
//! in `comment_kind_mapping_tests` beside `From<CommentKind> for TriviaKind`
//! in `src/lossless.rs`, the only place all four arms are nameable.** Do not
//! widen this file's claim back to the arm level; it was never achievable
//! from an integration test, not merely unproven here
//! (LANG-COMMENT-POPULATION-PARITY D5).

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

/// D2 -- the symmetric direction, ordinary-block form: in the SAME
/// discriminating configuration (comment on the same line after a
/// declaration, with a following declaration), an ORDINARY block comment
/// must still take the positional `Trailing` answer, homed on the
/// PRECEDING declaration -- not the doc rule's `Leading`/following. This is
/// the ordinary-class counterpart to `d1`'s doc-class rows -- together they
/// establish the CLASS split behaviorally (see the module doc); the
/// per-arm claim lives in `comment_kind_mapping_tests` in `src/lossless.rs`.
#[test]
fn d2_same_line_after_ordinary_block_comment_attaches_trailing_to_preceding_declaration() {
    let src = "const a : Int = 1 {- note -}\nconst b : Int = 2\n";
    let source = parse_lossless(src).unwrap_or_else(|e| panic!("{src:?}: {e:?}"));
    let comment_start = src.find("{-").unwrap();
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
        .find(|a| a.comment_span.start == comment_start)
        .unwrap_or_else(|| panic!("comment in {src:?} must have an attachment"));
    assert_eq!(attachment.placement, CommentPlacement::Trailing, "{src:?}");
    // The `Trailing` home is the smallest enclosing span of the PRECEDING
    // token (`smallest_enclosing` in `attach_comments`), which for a bare
    // literal RHS is the value expression's own (narrower) span rather than
    // the whole `const` decl's span -- so the correct check is containment
    // within `a`, not equality with `a`'s outer span (`ac3`/`d1`'s `next`
    // case does resolve to the outer span exactly, since nothing narrower
    // starts at a following declaration's first token; the two directions
    // are not symmetric in this respect).
    assert!(
        a_span.start <= attachment.home_span.start && attachment.home_span.end <= a_span.end,
        "{src:?}: ordinary same-line-after comment's home ({:?}) must lie WITHIN the \
         PRECEDING decl (a, {a_span:?})",
        attachment.home_span
    );
    assert_ne!(
        attachment.home_span, b_span,
        "{src:?}: must NOT take the doc rule's Leading/following answer"
    );
}
