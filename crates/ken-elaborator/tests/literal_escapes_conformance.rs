//! `conformance/surface/literals/seed-escapes.md` -- the six seed rows,
//! driven exactly as the seed writes them (LANG-SURFACE-LITERAL-ESCAPES
//! AC-1). One test function per case id; case ids are quoted in each
//! function's doc comment so a reader can align a failure back to the seed
//! row rather than to this file's own paraphrase.

use ken_elaborator::lexer::Lexer;
use ken_elaborator::{ElabEnv, ElabError, NumericLitVal};
use ken_interp::eval::{eval, EvalStore, EvalVal};
use ken_kernel::Decl;

/// Elaborate one top-level `const` declaration and evaluate its body against
/// an existing environment, seeding the literal side-table from the
/// elaborator's own `num_values` (never a hand-built literal value --
/// `[[conformance-hand-feeds-the-deliverable]]`). Callers sweeping many
/// fixtures reuse one `env` rather than paying `ElabEnv::new()`'s
/// prelude-rebuild cost per fixture.
fn eval_const_in(env: &mut ElabEnv, src: &str) -> EvalVal {
    let r = env.elaborate_decl_v1(src).expect("must elaborate");
    let mut store = EvalStore::new();
    let mkdecimalpair_id = env.prelude_env.mkdecimalpair_id;
    for (id, v) in &env.num_values {
        let val = match v {
            NumericLitVal::Int(n) => EvalVal::from(n.clone()),
            NumericLitVal::Float(f) => EvalVal::Float(*f),
            NumericLitVal::Float32(f) => EvalVal::Float32(*f),
            NumericLitVal::Decimal { coeff, exp } => {
                ken_interp::decimal_value(mkdecimalpair_id, coeff.clone(), *exp)
            }
            NumericLitVal::Str(s) => EvalVal::Str(s.clone()),
            NumericLitVal::Bytes(b) => EvalVal::Bytes(b.clone()),
        };
        store.num_values.insert(*id, val);
    }
    match env.env.lookup(r.def_id) {
        Some(Decl::Transparent { body, .. }) => eval(&[], body, &env.env, &mut store),
        other => panic!("expected a checked Transparent const, got {:?}", other.map(|_| ())),
    }
}

/// Single-fixture convenience wrapper over [`eval_const_in`] with a fresh
/// prelude.
fn eval_const(src: &str) -> EvalVal {
    eval_const_in(&mut ElabEnv::new().expect("prelude init"), src)
}

fn elaborate_err(src: &str) -> ElabError {
    ElabEnv::new()
        .expect("prelude init")
        .elaborate_decl(src)
        .expect_err("must reject")
}

fn assert_invalid_escape(err: &ElabError, expect_span: (usize, usize)) {
    match err {
        ElabError::InvalidEscape { span, .. } => {
            assert_eq!((span.start, span.end), expect_span, "InvalidEscape span")
        }
        other => panic!("expected InvalidEscape, got {other:?}"),
    }
}

/// surface/literals/common-escape-matrix-decodes-exactly
#[test]
fn common_escape_matrix_decodes_exactly() {
    let matrix: &[(char, char, u8)] = &[
        ('\\', '\\', 0x5C),
        ('"', '"', 0x22),
        ('\'', '\'', 0x27),
        ('0', '\0', 0x00),
        ('n', '\n', 0x0A),
        ('r', '\r', 0x0D),
        ('t', '\t', 0x09),
    ];
    let mut env = ElabEnv::new().expect("prelude");
    for (i, &(spelling, scalar, byte)) in matrix.iter().enumerate() {
        // String, one escape, one decoded scalar.
        let str_src = format!("const escape_str_{i} : String = \"\\{spelling}\"");
        match eval_const_in(&mut env, &str_src) {
            EvalVal::Str(s) => assert_eq!(s.as_str().chars().count(), 1, "{str_src}"),
            other => panic!("{str_src} -> expected Str, got {other:?}"),
        }
        // Char, exactly the one decoded scalar.
        let char_src = format!("const escape_char_{i} : Char = '\\{spelling}'");
        match eval_const_in(&mut env, &char_src) {
            EvalVal::Int(n) => assert_eq!(n, scalar as i64, "{char_src}"),
            EvalVal::BigInt(n) => assert_eq!(n, num_bigint::BigInt::from(scalar as u32), "{char_src}"),
            other => panic!("{char_src} -> expected Int (Char is {{c:Int|isScalar c}}), got {other:?}"),
        }
        // Byte string, exactly the one decoded byte.
        let bytes_src = format!("const escape_bytes_{i} : Bytes = b\"\\{spelling}\"");
        match eval_const_in(&mut env, &bytes_src) {
            EvalVal::Bytes(bs) => assert_eq!(bs, vec![byte], "{bytes_src}"),
            other => panic!("{bytes_src} -> expected Bytes, got {other:?}"),
        }
    }
}

/// surface/literals/escape-repertoire-is-closed-and-kind-selected
#[test]
fn escape_repertoire_is_closed_and_kind_selected() {
    let common: &[u8] = b"\\\"'0nrt";
    // Exhaustive sweep: every ASCII discriminator, for each of the three
    // non-raw kinds. Only the seven common escapes are universal; `u`
    // completes only in String/Char, `x` only in byte-string; everything
    // else -- including a discriminator that never completes a shape --
    // rejects with InvalidEscape and no literal token.
    // One prelude build for the whole sweep (a fresh `ElabEnv::new()` per of
    // ~380 iterations is pure prelude-rebuild overhead) -- each iteration
    // declares a uniquely-named const so accumulated globals never collide.
    let mut env = ElabEnv::new().expect("prelude");
    for disc in 0u8..128 {
        let disc_char = disc as char;
        if disc_char == 'u' || disc_char == 'x' {
            continue; // covered by the well-shaped probes below
        }
        let accepted = common.contains(&disc);

        let str_src = format!("const t_str_{disc} : String = \"\\{disc_char}\"");
        let str_res = env.elaborate_decl(&str_src);
        assert_eq!(str_res.is_ok(), accepted, "String \\{disc_char}: {str_res:?}");

        let char_src = format!("const t_char_{disc} : Char = '\\{disc_char}'");
        let char_res = env.elaborate_decl(&char_src);
        assert_eq!(char_res.is_ok(), accepted, "Char \\{disc_char}: {char_res:?}");

        let bytes_src = format!("const t_bytes_{disc} : Bytes = b\"\\{disc_char}\"");
        let bytes_res = env.elaborate_decl(&bytes_src);
        assert_eq!(bytes_res.is_ok(), accepted, "Bytes \\{disc_char}: {bytes_res:?}");

        if !accepted {
            for res in [str_res, char_res, bytes_res] {
                assert!(
                    matches!(res.unwrap_err(), ElabError::InvalidEscape { .. }),
                    "\\{disc_char} must reject specifically with InvalidEscape"
                );
            }
        }
    }

    // `\q` -- named unrecognized control; primary span exactly `\q`.
    let q_src = "const t : String = \"\\q\"";
    let q_err = elaborate_err(q_src);
    let q_start = q_src.find('\\').unwrap();
    assert_invalid_escape(&q_err, (q_start, q_start + 2));

    // well-shaped `\u{41}` in all three kinds: accepted in String/Char,
    // rejected (wrong-kind, full-escape span) in a byte string.
    assert!(ElabEnv::new().unwrap().elaborate_decl("const t : String = \"\\u{41}\"").is_ok());
    assert!(ElabEnv::new().unwrap().elaborate_decl("const t : Char = '\\u{41}'").is_ok());
    let u_wrong_kind = elaborate_err("const t : Bytes = b\"\\u{41}\"");
    match &u_wrong_kind {
        ElabError::InvalidEscape { span, .. } => {
            let src = "const t : Bytes = b\"\\u{41}\"";
            let start = src.find('\\').unwrap();
            assert_eq!((span.start, span.end), (start, start + 6), "\\u{{41}} complete-escape span");
        }
        other => panic!("expected InvalidEscape, got {other:?}"),
    }

    // well-shaped `\x41` in all three kinds: accepted in byte-string,
    // rejected (wrong-kind, full-escape span) in String/Char.
    assert!(ElabEnv::new().unwrap().elaborate_decl("const t : Bytes = b\"\\x41\"").is_ok());
    for (kind, src) in [
        ("String", "const t : String = \"\\x41\""),
        ("Char", "const t : Char = '\\x41'"),
    ] {
        let err = elaborate_err(src);
        match &err {
            ElabError::InvalidEscape { span, .. } => {
                let start = src.find('\\').unwrap();
                assert_eq!((span.start, span.end), (start, start + 4), "{kind} \\x41 complete-escape span");
            }
            other => panic!("{kind}: expected InvalidEscape, got {other:?}"),
        }
    }
}

/// surface/literals/unicode-escape-shape-scalar-and-char-cardinality
#[test]
fn unicode_escape_shape_scalar_and_char_cardinality() {
    // Valid escapes decode to the exact scalar, in both String and Char.
    for (escape, scalar) in [("\\u{0}", 0u32), ("\\u{1F600}", 0x1F600), ("\\u{10FFFF}", 0x10FFFF)] {
        let str_src = format!("const t : String = \"{escape}\"");
        match eval_const(&str_src) {
            EvalVal::Str(s) => {
                let mut it = s.as_str().chars();
                assert_eq!(it.next(), char::from_u32(scalar), "{str_src}");
                assert_eq!(it.next(), None);
            }
            other => panic!("{str_src} -> {other:?}"),
        }
        let char_src = format!("const t : Char = '{escape}'");
        match eval_const(&char_src) {
            EvalVal::Int(n) => assert_eq!(n as u32, scalar, "{char_src}"),
            EvalVal::BigInt(n) => assert_eq!(n, num_bigint::BigInt::from(scalar), "{char_src}"),
            other => panic!("{char_src} -> {other:?}"),
        }
    }

    // Malformed shapes: exact spans, boundary excluded where applicable.
    for (escape, tail_len) in [("\\u{}", 4), ("\\u{0000041}", 10), ("\\u{4_}", 5), ("\\u{G}", 4)] {
        let src = format!("const t : String = \"{escape}\"");
        let err = elaborate_err(&src);
        let start = src.find('\\').unwrap();
        assert_invalid_escape(&err, (start, start + tail_len));
    }

    // Well-shaped invalid-scalar: span is the COMPLETE escape (including `}`).
    for escape in ["\\u{D800}", "\\u{DFFF}", "\\u{110000}"] {
        let src = format!("const t : String = \"{escape}\"");
        let err = elaborate_err(&src);
        let start = src.find('\\').unwrap();
        assert_invalid_escape(&err, (start, start + escape.len()));
    }

    // Cardinality: one decoded scalar accepts; empty/two-scalar reject
    // (name/span not pinned by the seed -- just confirm rejection).
    assert!(ElabEnv::new().unwrap().elaborate_decl("const t : Char = 'A'").is_ok());
    assert!(ElabEnv::new().unwrap().elaborate_decl("const t : Char = ''").is_err());
    assert!(ElabEnv::new().unwrap().elaborate_decl("const t : Char = 'AB'").is_err());
}

/// surface/literals/byte-string-ascii-and-x-domain
#[test]
fn byte_string_ascii_and_x_domain() {
    // All 256 byte escapes, upper- and lower-case hex.
    let mut env = ElabEnv::new().expect("prelude");
    for byte in 0u16..256 {
        let byte = byte as u8;
        for (case, hex) in [("upper", format!("{byte:02X}")), ("lower", format!("{byte:02x}"))] {
            let src = format!("const t_{byte}_{case} : Bytes = b\"\\x{hex}\"");
            match eval_const_in(&mut env, &src) {
                EvalVal::Bytes(bs) => assert_eq!(bs, vec![byte], "{src}"),
                other => panic!("{src} -> {other:?}"),
            }
        }
    }

    // `b"\x41BC"` -- no greedy third-digit consumption.
    match eval_const(r#"const t : Bytes = b"\x41BC""#) {
        EvalVal::Bytes(bs) => assert_eq!(bs, vec![0x41, b'B', b'C']),
        other => panic!("{other:?}"),
    }

    // Ordinary permitted unescaped ASCII body.
    match eval_const(r#"const t : Bytes = b"ABC""#) {
        EvalVal::Bytes(bs) => assert_eq!(bs, vec![b'A', b'B', b'C']),
        other => panic!("{other:?}"),
    }

    // Unescaped non-ASCII scalar rejects (name/span not pinned).
    assert!(ElabEnv::new().unwrap().elaborate_decl("const t : Bytes = b\"\u{e9}\"").is_err());

    // Malformed: `b"\x4"` (incomplete, closing quote excluded) and
    // `b"\xG0"` (non-hex, span excludes the trailing 0).
    let short = elaborate_err(r#"const t : Bytes = b"\x4""#);
    {
        let src = r#"const t : Bytes = b"\x4""#;
        let start = src.find('\\').unwrap();
        assert_invalid_escape(&short, (start, start + 3));
    }
    let nonhex = elaborate_err(r#"const t : Bytes = b"\xG0""#);
    {
        let src = r#"const t : Bytes = b"\xG0""#;
        let start = src.find('\\').unwrap();
        assert_invalid_escape(&nonhex, (start, start + 3));
    }
}

/// surface/literals/raw-triple-backslashes-are-data
#[test]
fn raw_triple_backslashes_are_data() {
    let src = "const t : String = \"\"\"\\n\\q\\u{D800}\\xGG\\\\\"\"\"";
    match eval_const(src) {
        EvalVal::Str(s) => assert_eq!(s.as_str(), "\\n\\q\\u{D800}\\xGG\\\\"),
        other => panic!("{other:?}"),
    }
    // No InvalidEscape raised at all -- confirmed at the lexer directly.
    assert!(Lexer::lex(src).is_ok());
}

/// surface/literals/invalid-escape-span-precedes-unterminated
#[test]
fn invalid_escape_span_precedes_unterminated() {
    // String/delimiter leg.
    let s_src = "const t : String = \"\\u{41\"";
    let s_err = elaborate_err(s_src);
    let s_start = s_src.find('\\').unwrap();
    assert_invalid_escape(&s_err, (s_start, s_start + 5));
    // Char/line-boundary leg.
    {
        let err = Lexer::lex("'\\\n").unwrap_err();
        assert_invalid_escape(&err, (1, 2));
    }
    // Bytes/EOF leg.
    {
        let err = Lexer::lex("b\"\\x4").unwrap_err();
        assert_invalid_escape(&err, (2, 5));
    }

    // No-pending-escape twins: retain ordinary unterminated behavior, not
    // reclassified as InvalidEscape.
    for src in ["\"abc", "'abc", "b\"abc"] {
        match Lexer::lex(src).unwrap_err() {
            ElabError::ParseError { msg, .. } => assert!(msg.contains("unterminated"), "{src}: {msg}"),
            other => panic!("{src}: expected ordinary unterminated, got {other:?}"),
        }
    }
}

/// LANG-SURFACE-LITERAL-ESCAPES AC-7 -- D0's decision, committed as a
/// control. D0 measured zero existing `foreign` symbol/library name (in
/// `crates/` or the prelude) containing a backslash, so there was no live
/// compatibility conflict; the decision taken is that escapes apply to
/// `Token::Str` uniformly -- one repertoire, every consumer, no distinct
/// non-escaping path for `foreign` names. Verified structurally too:
/// `parse_foreign_decl` (parser.rs) extracts `symbol`/`library` directly
/// from `Token::Str(s)`, never through `Expr::EStr`/the general expression
/// path, so decoding happens once, in the lexer, before any consumer sees
/// the string -- there is no separate place a non-escaping path could live.
#[test]
fn d0_foreign_names_decode_escapes_uniformly() {
    use ken_elaborator::parser::parse_decls;
    use ken_elaborator::Decl;

    let decls = parse_decls("foreign escaped : Int -> Int = \"sym\\tbol\" \"li\\\\b\"")
        .expect("foreign decl with escaped symbol/library names must parse");
    assert_eq!(decls.len(), 1);
    match &decls[0] {
        Decl::ForeignDecl { symbol, library, .. } => {
            assert_eq!(symbol, "sym\tbol", "the symbol name must be escape-decoded");
            assert_eq!(library, "li\\b", "the library name must be escape-decoded");
        }
        other => panic!("expected ForeignDecl, got {other:?}"),
    }
}
