//! LANG-SURFACE-BLOCK-COMMENTS -- block comments, doc comments, and
//! following-declaration attachment (`31 §5`). The node's central risk is
//! two independent scanners (the semantic `Lexer::skip_ws_comments`, which
//! skips comments, and `lossless.rs`'s `append_trivia`, which rescans and
//! retains them) disagreeing about where a comment ends; AC-1 is the
//! control for that.

use ken_elaborator::lexer::{Lexer, Token};
use ken_elaborator::lossless::{parse_lossless, CommentPlacement};
use ken_elaborator::{ElabEnv, ElabError};

fn lex_agrees_with_lossless(src: &str) -> Result<(), String> {
    let lex_ok = Lexer::lex(src).is_ok();
    let lossless_ok = parse_lossless(src).is_ok();
    if lex_ok != lossless_ok {
        return Err(format!(
            "scanners disagree on {src:?}: Lexer::lex ok={lex_ok}, parse_lossless ok={lossless_ok}"
        ));
    }
    Ok(())
}

fn roundtrip(src: &str) {
    lex_agrees_with_lossless(src).unwrap();
    let source = parse_lossless(src).unwrap_or_else(|e| panic!("{src:?} must parse: {e:?}"));
    assert_eq!(source.reconstruct(), src, "round-trip must reproduce {src:?} exactly");
}

/// AC-1 -- the two scanners agree, over a corpus covering every form, both
/// nesting levels, and both the successful and unterminated cases. A test
/// that only checks "it parses" cannot see a disagreement, because parsing
/// consults only the semantic lexer's answer -- this asserts BOTH the
/// content-level round-trip (`reconstruct() == src`, proving the lossless
/// side matches) AND that a source the lexer accepts, `parse_lossless` also
/// accepts (and vice versa for rejections), so neither scanner silently
/// diverges from the other in either direction.
#[test]
fn ac1_two_scanners_agree_over_the_corpus() {
    // `parse_lossless` parses a whole program (declarations), so every
    // fixture is declaration-shaped, not a bare expression fragment.
    // Successful forms: reconstruct byte-identically.
    for src in [
        "const t : Int = 1",
        "-- x\nconst t : Int = 1",
        "--- doc\nconst t : Int = 1",
        "const t : Int = {- x -} 1",
        "const t : Int = {- {- inner -} outer -} 1",
        "const t : Int = {-- doc --} 1",
        "const t : Int = 1 -- trailing",
        "const t : Int = 1\n--- doc for nothing after it",
        "const t : Int = {- a -}{- b -} 1",
        "-- a\n--- b\n{- c -}\n{-- d --}\nconst t : Int = 1",
    ] {
        roundtrip(src);
    }
    // Unterminated forms: both scanners must refuse, never disagree.
    for src in [
        "const t : Int = {- unterminated",
        "const t : Int = {- {- -}",
        "const t : Int = {-- unterminated",
        "const t : Int = {-- {- -} still unterminated",
    ] {
        lex_agrees_with_lossless(src).unwrap();
        assert!(Lexer::lex(src).is_err(), "{src:?} must be rejected by the lexer");
    }
}

/// AC-2 -- nesting at two levels minimum. One level cannot distinguish
/// "nests" from "scans to the first `-}`" -- both give the same answer on
/// `{- x -}`; the two-level case is the only witness.
#[test]
fn ac2_two_level_nesting() {
    // `{- {- -} -}` is ONE comment: both inner and outer close, net token
    // stream has nothing left over from the comment.
    assert_eq!(
        Lexer::lex("{- {- -} -} 1").unwrap().into_iter().map(|(t, _)| t).collect::<Vec<_>>(),
        vec![Token::Nat(1), Token::Eof]
    );
    roundtrip("const t : Int = {- {- -} -} 1");

    // `{- {- -}` is UNTERMINATED, not a complete comment -- only the inner
    // `{- -}` closed; the outer opener has no matching close.
    let err = Lexer::lex("{- {- -}").unwrap_err();
    match err {
        ElabError::ParseError { msg, .. } => assert!(msg.contains("unterminated"), "{msg}"),
        other => panic!("expected ParseError, got {other:?}"),
    }
    match parse_lossless("const t : Int = {- {- -}") {
        Err(_) => {}
        Ok(_) => panic!("lossless layer must also reject `{{- {{- -}}` as unterminated"),
    }
}

/// AC-3 -- doc attachment is to the FOLLOWING declaration, discriminatingly.
/// The comment sits between two declarations, so attaching to the preceding
/// one would give a different (wrong) answer; a comment with nothing before
/// it is not this control.
#[test]
fn ac3_doc_comment_attaches_to_following_declaration_discriminatingly() {
    for (doc_open, doc_close) in [("---", ""), ("{--", " --}")] {
        let src = format!(
            "const a : Int = 1\n{doc_open} doc for b{doc_close}\nconst b : Int = 2\n"
        );
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
        assert_eq!(attachment.home_span, b_span, "{src:?}: must attach to the FOLLOWING decl (b)");
        assert_ne!(attachment.home_span, a_span, "{src:?}: must NOT attach to the preceding decl (a)");
    }
}

/// AC-4 -- unterminated is an error, in both scanners, with a span (not
/// merely "something failed").
#[test]
fn ac4_unterminated_errors_have_spans_in_both_scanners() {
    // Nested block: span is the opening `{-` through EOF.
    let src = "1 {- open";
    let open_at = src.find("{-").unwrap();
    match Lexer::lex(src).unwrap_err() {
        ElabError::ParseError { msg, span } => {
            assert!(msg.contains("unterminated block comment"));
            assert_eq!((span.start, span.end), (open_at, src.len()));
        }
        other => panic!("{other:?}"),
    }
    match parse_lossless(src) {
        Err(_) => {}
        Ok(_) => panic!("lossless layer must also reject an unterminated block comment"),
    }

    // Doc block: span is the opening `{--` through EOF.
    let src = "1 {-- open";
    let open_at = src.find("{--").unwrap();
    match Lexer::lex(src).unwrap_err() {
        ElabError::ParseError { msg, span } => {
            assert!(msg.contains("unterminated doc block comment"));
            assert_eq!((span.start, span.end), (open_at, src.len()));
        }
        other => panic!("{other:?}"),
    }
    match parse_lossless(src) {
        Err(_) => {}
        Ok(_) => panic!("lossless layer must also reject an unterminated doc block comment"),
    }
}

/// AC-5 -- the prefix relations, enumerated with a control per entry.
#[test]
fn ac5_prefix_relations_enumerated() {
    // `--` -- ordinary line comment: consumes to end of line, nothing left.
    assert_eq!(
        Lexer::lex("1 -- x\n2").unwrap().into_iter().map(|(t, _)| t).collect::<Vec<_>>(),
        vec![Token::Nat(1), Token::Nat(2), Token::Eof],
        "--"
    );

    // `---` -- doc line comment: same shape as `--`, tagged DocLineComment.
    let src = "1\n--- doc\n2";
    assert_eq!(
        Lexer::lex(src).unwrap().into_iter().map(|(t, _)| t).collect::<Vec<_>>(),
        vec![Token::Nat(1), Token::Nat(2), Token::Eof],
        "---"
    );
    let decl_src = "const a : Int = 1\n--- doc\nconst b : Int = 2\n";
    let source = parse_lossless(decl_src).unwrap();
    assert!(
        source.trivia().iter().any(|t| decl_src[t.span.start..t.span.end].starts_with("---")),
        "--- must be retained as trivia"
    );

    // `----` -- the doc-line marker `---` plus a literal `-` as the FIRST
    // character of the comment's own text; not a fourth comment form.
    assert_eq!(
        Lexer::lex("1\n---- text\n2").unwrap().into_iter().map(|(t, _)| t).collect::<Vec<_>>(),
        vec![Token::Nat(1), Token::Nat(2), Token::Eof],
        "----"
    );

    // `{-` -- ordinary nestable block comment: opens, needs a matching `-}`.
    assert_eq!(
        Lexer::lex("1 {- x -} 2").unwrap().into_iter().map(|(t, _)| t).collect::<Vec<_>>(),
        vec![Token::Nat(1), Token::Nat(2), Token::Eof],
        "open-block-marker"
    );

    // `{--` -- doc block comment: opens, needs a matching `--}`, non-nesting.
    assert_eq!(
        Lexer::lex("1 {-- x --} 2").unwrap().into_iter().map(|(t, _)| t).collect::<Vec<_>>(),
        vec![Token::Nat(1), Token::Nat(2), Token::Eof],
        "open-doc-block-marker"
    );

    // `{---` -- the doc-block opener `{--` plus a literal `-` as the FIRST
    // character of the comment's own text; still closed by `--}`.
    assert_eq!(
        Lexer::lex("1 {--- x --} 2").unwrap().into_iter().map(|(t, _)| t).collect::<Vec<_>>(),
        vec![Token::Nat(1), Token::Nat(2), Token::Eof],
        "open-doc-block-marker-plus-dash"
    );

    // `{-}` -- opens an ORDINARY block comment via `{-`; the lone `}` is not
    // `-}`, so it does not close and the comment is unterminated at EOF.
    match Lexer::lex("{-}").unwrap_err() {
        ElabError::ParseError { msg, .. } => assert!(msg.contains("unterminated"), "{{-}}: {msg}"),
        other => panic!("{{-}}: {other:?}"),
    }
}

/// AC-6 -- the brace neighbours still parse, adjacent in the same program
/// as a block comment: refinement types, `class`, `instance`, `module`, and
/// (landed) record literals.
#[test]
fn ac6_brace_neighbours_coexist_with_a_block_comment() {
    let src = "{- a leading block comment -}\n\
               def Nonneg = { n : Int | Equal Int n n }\n\
               class C A { op : A -> A }\n\
               instance C Int { op = \\x. x }\n\
               module M { const ok : Bool = true }\n\
               record Point { x : Int, y : Int }\n\
               const p : Point = { x = 1, y = 2 }\n";
    roundtrip(src);
}

/// AC-7 -- the formatter round-trips a block comment byte-identically.
/// Leading position, where the existing `push_comments_between`/
/// `with_comments` mechanism already renders any comment kind today (an
/// ordinary line/block comment placed INSIDE an expression is dropped by
/// the pre-existing formatter, unrelated to this WP and out of its banned
/// "new layout rule" scope -- verified true for line comments too).
#[test]
fn ac7_formatter_round_trips_a_block_comment() {
    use ken_elaborator::layout::format_ken;
    let src = "{- leading -}\nconst a : Int = 1\n";
    let out1 = format_ken(src).expect("must format");
    let out2 = format_ken(&out1).expect("must re-format");
    assert_eq!(out1, out2, "format must be idempotent with a block comment present");
    assert!(out1.contains("{- leading -}"), "the block comment text must survive formatting: {out1:?}");
}

/// AC-D0 -- `true`/`false` elaborate at `Bool`; `trueish` still resolves as
/// an ordinary identifier (proving the lexical spelling did not shadow the
/// general identifier-scanning rule).
#[test]
fn acd0_bool_literals_and_trueish_preserved() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl("const b1 : Bool = true").expect("true must elaborate at Bool");
    env.elaborate_decl("const b2 : Bool = false").expect("false must elaborate at Bool");

    assert_eq!(
        Lexer::lex("trueish").unwrap()[0].0,
        Token::Ident("trueish".to_string()),
        "trueish must tokenize as a plain identifier"
    );
    assert_eq!(Lexer::lex("true").unwrap()[0].0, Token::ConId("True".to_string()));
    assert_eq!(Lexer::lex("false").unwrap()[0].0, Token::ConId("False".to_string()));

    // `trueish` reaches ordinary name resolution (fails only because it is
    // genuinely undefined, the same failure any unbound name gets -- not
    // because it was intercepted as a keyword).
    let mut env = ElabEnv::new().expect("prelude");
    match env.elaborate_decl("const t : Bool = trueish") {
        Err(ElabError::UnresolvedCon { name, .. }) => assert_eq!(name, "trueish"),
        other => panic!("expected UnresolvedCon(\"trueish\"), got {other:?}"),
    }
}

/// AC-8 (frame amendment, `origin/main=1b2b4326` / PR #2134) -- the
/// doc-block opener/closer boundary, pinned in a new function; no
/// predecessor assertion above this one is edited. These characterize
/// existing semantics only; nothing here changes comment behaviour.
///
/// The rows are also LANG-COMMENT-CLASSIFIER-SHARED's AC-4 discriminator:
/// swapping `{-` ahead of `{--` in the shared classifier makes `{--}` and
/// `{---}` stop erroring (a bare `{-`-opened block would close on the
/// trailing `-}` each already contains), which is a stronger, more legible
/// red than a kind-only divergence -- proven by temporary mutation and
/// reverted before landing, recorded in the handback rather than kept as a
/// permanent test.
#[test]
fn ac8_doc_block_opener_closer_edge_cases() {
    // `{--}` -- the doc-block OPENER `{--` plus a bare `}`; `}` alone is
    // not the doc-block closer `--}`, so this never closes and is
    // unterminated through EOF.
    let src = "{--}";
    match Lexer::lex(src).unwrap_err() {
        ElabError::ParseError { msg, span } => {
            assert!(msg.contains("unterminated doc block comment"), "{msg}");
            assert_eq!((span.start, span.end), (0, src.len()));
        }
        other => panic!("{other:?}"),
    }
    match parse_lossless(src) {
        Err(_) => {}
        Ok(_) => panic!("lossless layer must also reject `{{--}}` as unterminated"),
    }

    // `{---}` -- opener `{--` plus `-}`, still short of the three-character
    // closer `--}`; also unterminated through EOF.
    let src = "{---}";
    match Lexer::lex(src).unwrap_err() {
        ElabError::ParseError { msg, span } => {
            assert!(msg.contains("unterminated doc block comment"), "{msg}");
            assert_eq!((span.start, span.end), (0, src.len()));
        }
        other => panic!("{other:?}"),
    }
    match parse_lossless(src) {
        Err(_) => {}
        Ok(_) => panic!("lossless layer must also reject `{{---}}` as unterminated"),
    }

    // `{----}` -- opener `{--` immediately followed by its own closer
    // `--}`: the shortest possible empty doc block comment.
    assert_eq!(
        Lexer::lex("{----}")
            .unwrap()
            .into_iter()
            .map(|(t, _)| t)
            .collect::<Vec<_>>(),
        vec![Token::Eof],
        "{{----}} must be one complete, empty doc block comment"
    );
    roundtrip("{----}const t : Int = 1");

    // `{----} 1` -- same empty doc block, followed by ordinary content; the
    // `1` survives as an ordinary token, unaffected by the (empty)
    // preceding comment.
    assert_eq!(
        Lexer::lex("{----} 1")
            .unwrap()
            .into_iter()
            .map(|(t, _)| t)
            .collect::<Vec<_>>(),
        vec![Token::Nat(1), Token::Eof],
        "{{----}} 1 must leave 1 as an ordinary token"
    );

    // `{-} 1 -}` -- the fail-open witness. `{-` opens an ORDINARY nested
    // block comment (not `{--`, since the third character is `}` not `-`);
    // the bare `}` right after does not close it -- only `-}` does -- so
    // the comment stays open, silently consuming ` 1 ` as ordinary body
    // content until the first `-}` at the end. The `1` is therefore absent
    // from the token stream entirely, not merely "commented around".
    assert_eq!(
        Lexer::lex("{-} 1 -}")
            .unwrap()
            .into_iter()
            .map(|(t, _)| t)
            .collect::<Vec<_>>(),
        vec![Token::Eof],
        "{{-}} 1 -}} must consume the 1 as comment body, not leave it as a token"
    );
}
