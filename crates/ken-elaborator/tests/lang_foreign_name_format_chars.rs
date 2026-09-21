use ken_elaborator::{
    format::canonicalize_lexed_tokens,
    format_ken_md,
    lexer::{Lexer, Token},
    lossless::{parse_lossless, TriviaKind},
    Decl, ElabError, Span,
};

const BIDI_OVERRIDE: char = '\u{202E}';

fn assert_raw_format_character(source: &str, character: char) {
    let start = source
        .find(character)
        .expect("the fixture must contain the raw format character");
    match Lexer::lex(source).expect_err("raw Cf source must be rejected") {
        ElabError::RawFormatCharacter {
            character: actual,
            span,
        } => {
            assert_eq!(actual, character);
            assert_eq!(span, Span::new(start, start + character.len_utf8()));
        }
        other => panic!("expected RawFormatCharacter, got {other:?}"),
    }
}

#[test]
fn raw_bidi_override_rejects_in_every_required_source_context() {
    let fixtures = [
        format!("-- before{BIDI_OVERRIDE}after\nfn f : Type = Type"),
        format!("{{- outer {{- before{BIDI_OVERRIDE}after -}} outer -}}\nfn f : Type = Type"),
        format!("fn{BIDI_OVERRIDE} f : Type = Type"),
        format!("\"before{BIDI_OVERRIDE}after\""),
        format!("\"\"\"before{BIDI_OVERRIDE}after\"\"\""),
    ];

    for source in fixtures {
        assert_raw_format_character(&source, BIDI_OVERRIDE);
    }
}

#[test]
fn unlisted_format_category_members_use_the_same_guard() {
    for character in ['\u{00AD}', '\u{061C}'] {
        let comment = format!("-- before{character}after\nfn f : Type = Type");
        let string = format!("\"before{character}after\"");
        assert_raw_format_character(&comment, character);
        assert_raw_format_character(&string, character);
    }
}

#[test]
fn raw_character_and_byte_literal_bodies_use_the_whole_source_guard() {
    assert_raw_format_character(&format!("'{BIDI_OVERRIDE}'"), BIDI_OVERRIDE);
    assert_raw_format_character(&format!("b\"{BIDI_OVERRIDE}\""), BIDI_OVERRIDE);
}

#[test]
fn reports_the_first_raw_format_character() {
    let first = '\u{061C}';
    assert_raw_format_character(&format!("before{first}middle{BIDI_OVERRIDE}after"), first);
}

#[test]
fn formatter_recovery_route_uses_the_same_whole_source_guard() {
    let source = format!("-- a{BIDI_OVERRIDE}b\n@");
    let start = source.find(BIDI_OVERRIDE).unwrap();
    match canonicalize_lexed_tokens(&source)
        .expect_err("formatter recovery must reject raw Cf before comment dispatch")
    {
        ElabError::RawFormatCharacter { character, span } => {
            assert_eq!(character, BIDI_OVERRIDE);
            assert_eq!(span, Span::new(start, start + BIDI_OVERRIDE.len_utf8()));
        }
        other => panic!("expected RawFormatCharacter, got {other:?}"),
    }
}

#[test]
fn formatter_recovery_rejects_later_offset_feff_before_suffix_scanning() {
    let source = format!("x {0} @", '\u{FEFF}');
    let start = source.find('\u{FEFF}').unwrap();
    match canonicalize_lexed_tokens(&source)
        .expect_err("recovery root validation must reject later U+FEFF")
    {
        ElabError::RawFormatCharacter { character, span } => {
            assert_eq!(character, '\u{FEFF}');
            assert_eq!(span, Span::new(start, start + '\u{FEFF}'.len_utf8()));
        }
        other => panic!("expected RawFormatCharacter, got {other:?}"),
    }
}

#[test]
fn literate_hard_errors_rebase_to_document_spans_in_every_role() {
    for opener in ["```ken", "```ken example", "```ken ignore", "```ken reject"] {
        for body in [
            format!("-- before{BIDI_OVERRIDE}after\n@"),
            format!("const raw : String = \"\"\"before{BIDI_OVERRIDE}after\"\"\""),
            format!("fn{BIDI_OVERRIDE} f : Type = Type"),
        ] {
            let source = format!("prose\n{opener}\n{body}\n```\n");
            let start = source.find(BIDI_OVERRIDE).unwrap();
            match format_ken_md(&source).expect_err("non-repairable raw Cf must reject") {
                ElabError::RawFormatCharacter { character, span } => {
                    assert_eq!(character, BIDI_OVERRIDE);
                    assert_eq!(span, Span::new(start, start + BIDI_OVERRIDE.len_utf8()));
                }
                other => panic!("expected RawFormatCharacter, got {other:?}"),
            }
        }
    }
}

#[test]
fn literate_recovery_preserves_visible_escape_data() {
    for role in ["ignore", "reject"] {
        let source = format!("prose\n```ken {role}\n\"\\u{{202E}}\"\n@\n```\n");
        assert_eq!(
            format_ken_md(&source).expect("visible escape data must survive recovery"),
            source
        );
    }
}

#[test]
fn escaped_format_character_round_trips_as_literal_data() {
    let tokens = Lexer::lex("\"\\u{202E}\"").expect("visible escape must remain accepted");
    assert_eq!(tokens[0].0, Token::Str(BIDI_OVERRIDE.to_string()));
}

#[test]
fn formatter_inverse_preserves_string_and_character_values() {
    let second = '\u{061C}';
    let string_source =
        format!("const s : String = \"before{BIDI_OVERRIDE}middle{second}after\"\n");
    let formatted = ken_elaborator::layout::format_ken(&string_source)
        .expect("ordinary string payload must auto-escape");
    assert!(
        formatted.contains("\"before\\u{202E}middle\\u{61C}after\""),
        "{formatted}"
    );
    assert!(Lexer::lex(&formatted).unwrap().iter().any(|(token, _)| {
        token == &Token::Str(format!("before{BIDI_OVERRIDE}middle{second}after"))
    }));
    assert_eq!(
        ken_elaborator::layout::format_ken(&formatted).unwrap(),
        formatted
    );

    let character_source = format!("const c : Char = '{BIDI_OVERRIDE}'\n");
    let formatted = ken_elaborator::layout::format_ken(&character_source)
        .expect("character payload must auto-escape");
    assert!(formatted.contains("'\\u{202E}'"), "{formatted}");
    assert!(Lexer::lex(&formatted)
        .unwrap()
        .iter()
        .any(|(token, _)| token == &Token::CharLit(BIDI_OVERRIDE)));

    let markdown =
        format!("prose\n```ken\n{string_source}```\n```ken example\n{character_source}```\n");
    let formatted = format_ken_md(&markdown).expect("literate literals must auto-fix");
    assert!(
        formatted.contains("\"before\\u{202E}middle\\u{61C}after\""),
        "{formatted}"
    );
    assert!(formatted.contains("'\\u{202E}'"), "{formatted}");
    assert_eq!(format_ken_md(&formatted).unwrap(), formatted);
}

#[test]
fn formatter_split_keeps_non_literal_occurrences_as_typed_errors() {
    for source in [
        format!("-- before{BIDI_OVERRIDE}after\nconst x : Nat = Zero\n"),
        format!("fn{BIDI_OVERRIDE} f : Type = Type\n"),
        format!("const raw : String = \"\"\"before{BIDI_OVERRIDE}after\"\"\"\n"),
        format!("const bytes : Bytes = b\"{BIDI_OVERRIDE}\"\n"),
    ] {
        let start = source.find(BIDI_OVERRIDE).unwrap();
        match ken_elaborator::layout::format_ken(&source)
            .expect_err("non-literal raw Cf must remain a formatter hard error")
        {
            ElabError::RawFormatCharacter { character, span } => {
                assert_eq!(character, BIDI_OVERRIDE);
                assert_eq!(span, Span::new(start, start + BIDI_OVERRIDE.len_utf8()));
            }
            other => panic!("expected RawFormatCharacter, got {other:?}"),
        }
    }
}

#[test]
fn byte_string_inverse_uses_only_a_representable_byte_escape() {
    let source = format!("const b : Bytes = b\"{}\"\n", '\u{00AD}');
    let formatted = ken_elaborator::layout::format_ken(&source)
        .expect("representable byte payload must auto-escape");
    assert!(formatted.contains("b\"\\xAD\""), "{formatted}");
    assert!(Lexer::lex(&formatted)
        .unwrap()
        .iter()
        .any(|(token, _)| token == &Token::ByteStr(vec![0xAD])));
}

#[test]
fn literate_recovery_auto_fixes_literal_payloads_and_keeps_canonicalization() {
    for role in ["ignore", "reject"] {
        let source = format!(
            "prose\n```ken {role}\nconst s : String = \"{BIDI_OVERRIDE}\"\nfn unfinished (f : Nat -> Nat) : Nat =\n```\n"
        );
        let formatted = format_ken_md(&source).expect("literal payload must auto-fix in recovery");
        assert!(formatted.contains("\"\\u{202E}\""), "{formatted}");
        assert!(formatted.contains("Nat → Nat"), "{formatted}");
    }
}

#[test]
fn formatter_diagnostics_map_back_across_prior_repairs() {
    let source = format!("const s : String = \"{BIDI_OVERRIDE}\"\nconst t : String = \"\\q\"\n");
    let start = source.find("\\q").unwrap();
    match ken_elaborator::layout::format_ken(&source)
        .expect_err("the later invalid escape must retain original coordinates")
    {
        ElabError::InvalidEscape { span, .. } => assert_eq!(span, Span::new(start, start + 2)),
        other => panic!("expected InvalidEscape, got {other:?}"),
    }

    let markdown = format!(
        "prose\n```ken example\nconst s : String = \"{BIDI_OVERRIDE}\"\nconst t : String = \"\\q\"\n```\n"
    );
    let start = markdown.find("\\q").unwrap();
    match format_ken_md(&markdown)
        .expect_err("the later lexical error must retain document coordinates")
    {
        ElabError::InvalidEscape { span, .. } => assert_eq!(span, Span::new(start, start + 2)),
        other => panic!("expected InvalidEscape, got {other:?}"),
    }
}

#[test]
fn formatter_recovery_typed_error_uses_whole_fragment_coordinates() {
    let source = "\"ok\" \"\\q\"";
    let start = source.find("\\q").unwrap();
    match canonicalize_lexed_tokens(source)
        .expect_err("the later typed error must include the recovery cursor")
    {
        ElabError::InvalidEscape { span, .. } => assert_eq!(span, Span::new(start, start + 2)),
        other => panic!("expected InvalidEscape, got {other:?}"),
    }
}

fn assert_literate_recovery_composes_coordinates(role: &str) {
    let markdown = format!("prose\n```ken {role}\n\"{BIDI_OVERRIDE}\" \"ok\" \"\\q\"\n```\n");
    let start = markdown.find("\\q").unwrap();
    match format_ken_md(&markdown)
        .expect_err("the later typed error must retain full Markdown coordinates")
    {
        ElabError::InvalidEscape { span, .. } => {
            assert_eq!(span, Span::new(start, start + 2));
        }
        other => panic!("expected InvalidEscape, got {other:?}"),
    }
}

#[test]
fn ignore_recovery_composes_suffix_rewrite_and_document_coordinates() {
    assert_literate_recovery_composes_coordinates("ignore");
}

#[test]
fn reject_recovery_composes_suffix_rewrite_and_document_coordinates() {
    assert_literate_recovery_composes_coordinates("reject");
}

#[test]
fn hard_error_after_prior_repair_maps_to_document_coordinates() {
    let markdown = format!(
        "prose\n```ken\nconst s : String = \"{BIDI_OVERRIDE}\"\n-- later{0}hard\n```\n",
        '\u{061C}'
    );
    let start = markdown.find('\u{061C}').unwrap();
    match format_ken_md(&markdown)
        .expect_err("the later hard error must map through rewrite and fence coordinates")
    {
        ElabError::RawFormatCharacter { character, span } => {
            assert_eq!(character, '\u{061C}');
            assert_eq!(span, Span::new(start, start + '\u{061C}'.len_utf8()));
        }
        other => panic!("expected RawFormatCharacter, got {other:?}"),
    }
}

#[test]
fn formatter_recovery_advances_on_multibyte_boundaries() {
    let source = "😀 ->";
    assert_eq!(
        canonicalize_lexed_tokens(source).expect("recovery cursor must stay on UTF-8 boundaries"),
        "😀 →"
    );
}

#[test]
fn raw_twin_of_escaped_format_character_is_rejected() {
    assert_raw_format_character(&format!("\"{BIDI_OVERRIDE}\""), BIDI_OVERRIDE);
}

#[test]
fn offset_zero_feff_is_consumed_as_a_bom() {
    let source = "fn f : Type = Type";
    let plain: Vec<_> = Lexer::lex(source)
        .expect("plain source must lex")
        .into_iter()
        .map(|(token, _)| token)
        .collect();
    let with_bom: Vec<_> = Lexer::lex(&format!("\u{FEFF}{source}"))
        .expect("offset-zero U+FEFF must be consumed as a BOM")
        .into_iter()
        .map(|(token, _)| token)
        .collect();
    assert_eq!(with_bom, plain);
}

#[test]
fn lossless_and_layout_consumers_account_for_the_leading_bom() {
    let plain_source = "fn f : Type = Type";
    let bom_source = format!("\u{FEFF}{plain_source}");
    let plain = parse_lossless(plain_source).expect("plain source must parse losslessly");
    let with_bom = parse_lossless(&bom_source).expect("leading BOM must parse losslessly");

    let plain_tokens: Vec<_> = plain.tokens().iter().map(|token| &token.kind).collect();
    let bom_tokens: Vec<_> = with_bom.tokens().iter().map(|token| &token.kind).collect();
    assert_eq!(bom_tokens, plain_tokens);

    match (&plain.typed_decls()[0], &with_bom.typed_decls()[0]) {
        (
            Decl::ViewDecl {
                name: plain_name, ..
            },
            Decl::ViewDecl { name: bom_name, .. },
        ) => assert_eq!(bom_name, plain_name),
        other => panic!("expected matching function declarations, got {other:?}"),
    }

    let bom_items: Vec<_> = with_bom
        .trivia()
        .iter()
        .filter(|item| item.kind == TriviaKind::Bom)
        .collect();
    assert_eq!(bom_items.len(), 1);
    assert_eq!(bom_items[0].span, Span::new(0, '\u{FEFF}'.len_utf8()));
    assert!(with_bom.comment_attachments().is_empty());

    let mut cursor = 0;
    for piece in with_bom.pieces() {
        assert_eq!(piece.span.start, cursor);
        cursor = piece.span.end;
    }
    assert_eq!(cursor, bom_source.len());
    assert_eq!(with_bom.reconstruct(), bom_source);

    let plain_formatted =
        ken_elaborator::layout::format_ken(plain_source).expect("plain source must format");
    let bom_formatted = ken_elaborator::layout::format_ken(&bom_source)
        .expect("leading BOM must survive the lossless/layout consumer");
    assert_eq!(bom_formatted, plain_formatted);
}

#[test]
fn feff_after_offset_zero_is_rejected() {
    assert_raw_format_character(&format!(" {0}fn f : Type = Type", '\u{FEFF}'), '\u{FEFF}');
}

#[test]
fn category_data_version_matches_the_pinned_workspace_unicode_data() {
    assert_eq!(unicode_normalization::UNICODE_VERSION, (17, 0, 0));
}
