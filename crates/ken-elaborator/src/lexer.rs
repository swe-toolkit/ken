//! V0/V1/L1/L2/L7 lexer (`31 §8`, `21 §6.1`, `35 §4.1`, `34`, `38 §2`).
//!
//! Recognises the token subset for G1 (V0), V1 spec-annotation keywords,
//! L1 numeric literals (integer, float, decimal with `d`-suffix, float32 with
//! `f32`-suffix), infix arithmetic operators `+`, `+%`, `*`, `==`,
//! L2 sum-type/pattern-match keywords (`data`, `match`, `def`, `|->`
//! arm separators), and L7 `foreign` declaration tokens (`38 §2.1`, `(oracle)`
//! keyword spellings). `type` is reserved (SURF-def-refinement; `33 §1`)
//! and no longer a declaration keyword. Whitespace and `-- …` line
//! comments are skipped.

use crate::error::{ElabError, Span};
use num_bigint::BigInt;

/// A V0/V1/L1 token.
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // V0 keywords
    KwConst,
    KwFn,
    KwProc,
    KwLet,
    KwIn,
    KwIf,
    KwThen,
    KwElse,
    KwType,
    // V1 keywords
    KwRequires,
    KwEnsures,
    KwProve,
    KwLaw,
    KwOld,
    KwSpace,
    KwMut,
    KwBecomes,
    // L2 keywords
    KwData,         // "data" — inductive type declaration
    KwMatch,        // "match" — pattern matching
    KwDef,          // "def"   — surface definition (refinement/alias); was "type"
    KwTypeReserved, // "type" — reserved (SURF-def-refinement); not a decl keyword, not an identifier
    // L7 keywords (`38 §2.1`, spellings are `(oracle)`)
    KwForeign,
    // Lc keywords (`33 §5`, `39 §6`)
    KwRecord,   // "record"   — named-field record declaration
    KwClass,    // "class"    — typeclass declaration
    KwInstance, // "instance" — instance declaration
    KwDerive,   // "derive"   — auto-derive request
    KwWhere,    // "where"    — constraint list in class/instance/declaration
    // B2 keywords (`72 §4`, spellings are `(oracle)`/`OQ-syntax`)
    KwTemporal, // "temporal" — a delegated temporal-obligation block
    // ES3 keywords (`33 §3-4` — modules/imports/visibility)
    KwModule,      // "module" — module namespace declaration
    KwImport,      // "import" — qualified/aliased/selective import
    KwExport,      // "export" — facade/in-scope re-export declaration
    KwUseReserved, // "use" — reserved (ADR-0015); retired open import
    KwPub,         // "pub" — visibility export marker
    // N4 admission-boundary keywords (`33 §3.2.1`, §5.5.1)
    KwProgram, // "program" — anonymous multi-package admission root
    KwPackage, // "package" — anonymous package admission boundary
    KwAdmits,  // "admits"  — direct instance-provider package list
    /// "capabilities" — program effect-family authority declarations
    KwCapabilities,
    // SURF-named-proof-claims keywords (`33 §8`)
    KwProp,  // "prop"   — proposition-family claim shape
    KwTheorem, // "theorem"  — standalone checked theorem
    KwAxiom, // "axiom"  — named postulate declaration sugar
    KwProof, // "proof"  — attached checked theorem / selector
    // V0 punctuation
    LParen,
    RParen,
    Colon,
    DoubleColon,
    Eq,
    Dot,
    Arrow,
    Lambda,
    Semicolon,
    // V1 punctuation
    LBrace,
    RBrace,
    Pipe,
    // L7 punctuation (foreign effect-row list + string attributes)
    LBracket,    // `[`
    RBracket,    // `]`
    Comma,       // `,`
    Str(String), // `"…"` (escape-decoded) or `"""…"""` (raw) — also carries
                 // `foreign` decl symbol/library names; one escape repertoire
                 // for every `Token::Str` consumer (D0, `31 §3`)
    CharLit(char), // `'…'` — escape-decoded, exactly one Unicode scalar
    ByteStr(Vec<u8>), // `b"…"` — escape-decoded ASCII body + `\xHH` bytes
    // L1 arithmetic operators
    Plus,        // `+`  — type-directed infix addition
    PlusPercent, // `+%` — explicit wrapping add
    Minus,       // `-`  — type-directed infix subtraction (VAL2 #11)
    Star,        // `*`  — type-directed infix multiply
    EqEq,        // `==` — structural equality
    PropEq,      // `===` / `≡` — propositional equality notation
    Le,          // `<=` / `≤`
    Ge,          // `>=` / `≥`
    Ne,          // `/=` / `≠`
    And,         // `/\` / `∧`
    Or,          // `\/` / `∨`
    Member,      // `∈` — membership notation; distinct from keyword `in`
    FlowsTo,     // `<:` / `⊑`
    Join,        // `⊔`
    Meet,        // `⊓`
    Times,       // `><` / `×`
    // L2 punctuation
    MapsTo, // `|->` / `↦` — match arm separator
    // K2 punctuation (`16 §6`, LANG-TRUNCATION-SURFACE-SYNTAX)
    TruncBar, // `‖` / `||` — propositional-truncation formation delimiter,
              // paired: `‖A‖` / `||A||`. A genuine new token (not a
              // sugar-identifier), so no user-declared name can ever
              // collide with it.
    // L1 numeric literal tokens
    IntLit(BigInt),       // integer literal too large for u32
    FloatLit(f64),        // decimal or hexadecimal f64: `3.14`, `1e-9`, `0x1p-3`
    DecimalLit(BigInt, i32), // `d`-suffix: coeff × 10^exp; e.g. `0.1d` → (1,-1)
    Float32Lit(f32),      // `f32`-suffix: `1.5f32`
    // Atoms
    Ident(String), // lowercase-initial term variable
    ConId(String), // uppercase-initial base type / constructor
    Nat(u32),      // small non-negative integer (≤ u32::MAX); also a level digit
    Eof,
}

/// One fully-parsed escape production (`31 §3`), before literal-kind
/// gating. Shape and value parsing is identical regardless of which literal
/// kind is scanning; each kind's body-scan function decides which variants
/// its own repertoire accepts.
#[derive(Clone, Copy, Debug, PartialEq)]
enum EscapeShape {
    /// `\\ \" \' \0 \n \r \t` — accepted by every literal kind.
    Common(char),
    /// `\u{H…H}` — well-shaped, in-range, non-surrogate scalar.
    Unicode(char),
    /// `\xHH` — well-shaped byte.
    Byte(u8),
}

pub struct Lexer<'s> {
    src: &'s str,
    pos: usize,
    previous_token_was_dot: bool,
}

/// One classified comment form (`31 §5`), returned by [`classify_comment`]
/// alongside the byte offset immediately after its close. Shared between
/// `Lexer::skip_ws_comments` and `lossless::append_trivia`'s independent
/// rescan of the same source bytes, so the two cannot disagree about which
/// comment starts at a position, or where it ends, by construction
/// (LANG-COMMENT-CLASSIFIER-SHARED D1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CommentKind {
    /// `-- …` (`31 §5`).
    Line,
    /// `--- …` (`31 §5`, D2).
    DocLine,
    /// `{- … -}`, nestable (`31 §5`, D1).
    Block,
    /// `{-- … --}` (`31 §5`, D2); deliberately non-nesting.
    DocBlock,
}

/// Which block form failed to close before the shared classifier's `end`
/// bound. Only the two block forms can fail this way -- a line/doc-line
/// comment always ends at `\n` or `end` and never errors -- so this type
/// carries no `Line`/`DocLine` case, making an "unterminated line comment"
/// unrepresentable rather than an unreachable match arm.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BlockCommentForm {
    Block,
    DocBlock,
}

/// A block-form comment scan reached `end` before finding its close.
/// Each caller turns this into the `ElabError` appropriate to what `end`
/// means there: the lexer's `end` is real end-of-source, so this is a
/// genuine unterminated comment (`ElabError::ParseError`); the lossless
/// layer's `end` is the boundary the lexer already accepted as trivia, so
/// reaching it there means the two scanners disagree, not a user-facing
/// error (`ElabError::Internal`).
pub(crate) struct UnterminatedComment {
    pub(crate) form: BlockCommentForm,
    pub(crate) start: usize,
}

/// Classify the comment form starting at `src[pos..end]`, or `None` if `pos`
/// is not the start of any comment -- whitespace handling is each caller's
/// own concern (`31 §5`). Bounded to `end`: the lexer passes `src.len()`
/// (EOF); the lossless layer passes the far edge of the inter-token gap it
/// is rescanning. That bound is safe for the lexer to share, not merely
/// convenient: `materialize_partition` establishes that an inter-token
/// region contains whole trivia, so a comment never straddles a token
/// boundary, and the lexer's own bound (EOF) and the lossless layer's bound
/// (the next token's start) coincide on every comment either one actually
/// scans.
///
/// Specific-before-general, and this is the one site it happens at
/// (LANG-COMMENT-CLASSIFIER-SHARED AC-1): `{--` before `{-`, since `{--`
/// partially matches `{-` on its first two characters. `---` is not a
/// separate top-level arm -- see the reasoning at the `--` arm below.
pub(crate) fn classify_comment(
    src: &str,
    pos: usize,
    end: usize,
) -> Result<Option<(CommentKind, usize)>, UnterminatedComment> {
    if src[pos..end].starts_with("{--") {
        let next = scan_doc_block_comment_end(src, pos, end)?;
        Ok(Some((CommentKind::DocBlock, next)))
    } else if src[pos..end].starts_with("{-") {
        let next = scan_nested_block_comment_end(src, pos, end)?;
        Ok(Some((CommentKind::Block, next)))
    } else if src[pos..end].starts_with("--") {
        // Covers both `--` and `---` -- a doc line comment is a line
        // comment whose text happens to start with a dash; both scan
        // identically to end-of-line/EOF (a line comment cannot nest and
        // cannot fail, `31 §5`). Folded into one arm rather than ordered as
        // two, so there is no `---`-before-`--` ordering to duplicate: the
        // kind distinction below is a discriminant read AFTER the shared
        // end computation, not a second dispatch point.
        let next = src[pos..end].find('\n').map_or(end, |offset| pos + offset);
        let kind = if src[pos..end].starts_with("---") {
            CommentKind::DocLine
        } else {
            CommentKind::Line
        };
        Ok(Some((kind, next)))
    } else {
        Ok(None)
    }
}

/// `{- … -}`, nestable (`31 §5`, D1). Depth starts at 1 after the opening
/// `{-`; each further `{-` increments, each `-}` decrements, and the
/// comment ends the instant depth reaches 0. A nested `{--` partially
/// matches this `{-` check (2 of its 3 characters), incrementing depth with
/// the extra `-` left as ordinary body content on the next iteration --
/// deliberate, not a gap: only the outer, block-opening position is
/// classification-sensitive to `{--` vs `{-`; once inside a `{- -}` body,
/// only balance (`{-`/`-}`) matters.
fn scan_nested_block_comment_end(
    src: &str,
    start: usize,
    end: usize,
) -> Result<usize, UnterminatedComment> {
    let mut pos = start + 2; // consumed opening '{-'
    let mut depth: usize = 1;
    while pos < end {
        if src[pos..end].starts_with("{-") {
            pos += 2;
            depth += 1;
            continue;
        }
        if src[pos..end].starts_with("-}") {
            pos += 2;
            depth -= 1;
            if depth == 0 {
                return Ok(pos);
            }
            continue;
        }
        let ch = src[pos..end]
            .chars()
            .next()
            .expect("pos < end on a valid str slice always yields a char");
        pos += ch.len_utf8();
    }
    Err(UnterminatedComment {
        form: BlockCommentForm::Block,
        start,
    })
}

/// `{-- … --}` (`31 §5`, D2). Deliberately NOT nesting -- the spec marks
/// only the plain block form "(nestable)"; a doc block scans for the first
/// literal `--}` and treats everything else, including anything that looks
/// like a nested `{-`/`{--`, as ordinary body content.
fn scan_doc_block_comment_end(
    src: &str,
    start: usize,
    end: usize,
) -> Result<usize, UnterminatedComment> {
    let mut pos = start + 3; // consumed opening '{--'
    while pos < end {
        if src[pos..end].starts_with("--}") {
            return Ok(pos + 3);
        }
        let ch = src[pos..end]
            .chars()
            .next()
            .expect("pos < end on a valid str slice always yields a char");
        pos += ch.len_utf8();
    }
    Err(UnterminatedComment {
        form: BlockCommentForm::DocBlock,
        start,
    })
}

impl<'s> Lexer<'s> {
    pub fn new(src: &'s str) -> Self {
        Self {
            src,
            pos: 0,
            previous_token_was_dot: false,
        }
    }

    fn cur(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.cur()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    /// Skip whitespace and every comment form (`31 §5`): line `-- …`, doc
    /// line `--- …`, nestable block `{- … -}`, and doc block `{-- … --}`.
    /// Classification and both end-scanners live in the shared
    /// [`classify_comment`] (LANG-COMMENT-CLASSIFIER-SHARED D1/D2), the same
    /// function `lossless.rs::append_trivia` rescans with independently --
    /// making the two scanners' agreement about where a comment ends
    /// structural rather than merely tested-against. This loop only skips
    /// whitespace and advances `self.pos` by whatever the classifier
    /// reports, discarding the comment kind it does not need.
    /// `{--}` is an opener, not a closed comment -- `}` alone is not the
    /// doc-block closer `--}` -- so the shortest empty doc block comment is
    /// `{----}` (opener `{--` immediately followed by closer `--}`,
    /// LANG-COMMENT-CLASSIFIER-SHARED AC-9).
    fn skip_ws_comments(&mut self) -> Result<(), ElabError> {
        loop {
            while self.cur().map(|c| c.is_whitespace()).unwrap_or(false) {
                self.advance();
            }
            match classify_comment(self.src, self.pos, self.src.len()) {
                Ok(Some((_kind, next))) => self.pos = next,
                Ok(None) => break,
                Err(UnterminatedComment { form, start }) => {
                    let msg = match form {
                        BlockCommentForm::Block => "unterminated block comment",
                        BlockCommentForm::DocBlock => "unterminated doc block comment",
                    };
                    return Err(ElabError::ParseError {
                        msg: msg.to_string(),
                        span: Span::new(start, self.src.len()),
                    });
                }
            }
        }
        Ok(())
    }

    fn is_ascii_ident_continue(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_' || c == '\''
    }

    // ── Literal-escape scanning (`31 §3`, LANG-SURFACE-LITERAL-ESCAPES) ────
    //
    // One scanner, one error (`ElabError::InvalidEscape`), one span rule,
    // shared by every non-raw literal kind (String, Char, byte-string).
    // `scan_escape` parses a production's SHAPE and VALUE uniformly,
    // independent of which kind is asking; each kind's body-scan function
    // gates the returned `EscapeShape` against its own repertoire (the
    // "wrong-kind, well-shaped escape" case in the seed) and is the only
    // place that knows what its decoded content means.

    /// Consume one character if it is not the enclosing literal's own
    /// closing delimiter or a line boundary, returning it so the caller can
    /// fold it into an escape's error span. Returns `None` at a boundary
    /// (closing delimiter, `\n`, or EOF) WITHOUT consuming it -- error spans
    /// exclude the interrupting boundary (`31 §3`).
    fn consume_escape_offender(&mut self, closing: char) -> Option<char> {
        match self.cur() {
            Some(c) if c != closing && c != '\n' => {
                self.advance();
                Some(c)
            }
            _ => None,
        }
    }

    /// Scan one escape production immediately after a consumed backslash.
    /// `backslash_start` anchors the error span; `closing` is the enclosing
    /// literal's own delimiter (`"` for String/byte-string, `'` for Char),
    /// needed only to tell a boundary apart from an ordinary-but-wrong body
    /// character while scanning `\u{...}`/`\x..`'s interior.
    fn scan_escape(
        &mut self,
        backslash_start: usize,
        closing: char,
    ) -> Result<EscapeShape, ElabError> {
        match self.cur() {
            Some('\\') => {
                self.advance();
                Ok(EscapeShape::Common('\\'))
            }
            Some('"') => {
                self.advance();
                Ok(EscapeShape::Common('"'))
            }
            Some('\'') => {
                self.advance();
                Ok(EscapeShape::Common('\''))
            }
            Some('0') => {
                self.advance();
                Ok(EscapeShape::Common('\0'))
            }
            Some('n') => {
                self.advance();
                Ok(EscapeShape::Common('\n'))
            }
            Some('r') => {
                self.advance();
                Ok(EscapeShape::Common('\r'))
            }
            Some('t') => {
                self.advance();
                Ok(EscapeShape::Common('\t'))
            }
            Some('u') => {
                self.advance(); // consume 'u'
                if self.cur() != Some('{') {
                    self.consume_escape_offender(closing);
                    return Err(
                        self.invalid_escape(backslash_start, "malformed \\u escape: expected '{'")
                    );
                }
                self.advance(); // consume '{'
                let mut digits = String::new();
                loop {
                    match self.cur() {
                        Some(c) if c.is_ascii_hexdigit() && digits.len() < 6 => {
                            self.advance();
                            digits.push(c);
                        }
                        Some(c) if c.is_ascii_hexdigit() => {
                            self.advance(); // the 7th digit is unambiguously ordinary content
                            return Err(self.invalid_escape(
                                backslash_start,
                                "unicode escape has more than six hex digits",
                            ));
                        }
                        Some('}') if digits.is_empty() => {
                            self.advance(); // '}' is what reveals the empty escape; include it
                            return Err(
                                self.invalid_escape(backslash_start, "empty unicode escape")
                            );
                        }
                        Some('}') => {
                            self.advance();
                            let value = u32::from_str_radix(&digits, 16)
                                .expect("digits is 1-6 ASCII hex chars");
                            if value > 0x10FFFF || (0xD800..=0xDFFF).contains(&value) {
                                return Err(self.invalid_escape(
                                    backslash_start,
                                    "unicode escape is not a valid scalar value",
                                ));
                            }
                            let ch = char::from_u32(value).expect("range checked above");
                            return Ok(EscapeShape::Unicode(ch));
                        }
                        _ => {
                            self.consume_escape_offender(closing);
                            return Err(
                                self.invalid_escape(backslash_start, "malformed unicode escape")
                            );
                        }
                    }
                }
            }
            Some('x') => {
                self.advance(); // consume 'x'
                let Some(d1) = (match self.cur() {
                    Some(c) if c.is_ascii_hexdigit() => {
                        self.advance();
                        Some(c)
                    }
                    _ => None,
                }) else {
                    self.consume_escape_offender(closing);
                    return Err(self.invalid_escape(
                        backslash_start,
                        "malformed byte escape: expected two hex digits",
                    ));
                };
                let Some(d2) = (match self.cur() {
                    Some(c) if c.is_ascii_hexdigit() => {
                        self.advance();
                        Some(c)
                    }
                    _ => None,
                }) else {
                    self.consume_escape_offender(closing);
                    return Err(self.invalid_escape(
                        backslash_start,
                        "malformed byte escape: expected two hex digits",
                    ));
                };
                let value = u8::from_str_radix(&format!("{d1}{d2}"), 16)
                    .expect("d1/d2 are ASCII hex digits");
                Ok(EscapeShape::Byte(value))
            }
            // A line boundary right after the backslash is a BOUNDARY, not a
            // discriminator to consume -- the enclosing literal's own
            // closing delimiter is handled above (`\"`/`\'` are valid common
            // escapes in every kind, so `"`/`'` never reach this arm).
            Some('\n') => Err(
                self.invalid_escape(backslash_start, "incomplete escape before line boundary"),
            ),
            Some(other) => {
                self.advance();
                Err(self.invalid_escape(backslash_start, &format!("unrecognized escape '\\{other}'")))
            }
            None => Err(self.invalid_escape(backslash_start, "incomplete escape at end of input")),
        }
    }

    fn invalid_escape(&self, backslash_start: usize, reason: &str) -> ElabError {
        ElabError::InvalidEscape {
            span: Span::new(backslash_start, self.pos),
            reason: reason.to_string(),
        }
    }

    /// Scan an ordinary escaped string body up to its closing `"`. `start`
    /// is the position of the OPENING quote (used only for the unterminated-
    /// literal error's span).
    fn scan_string_body(&mut self, start: usize) -> Result<String, ElabError> {
        let mut s = String::new();
        loop {
            match self.cur() {
                None | Some('\n') => {
                    return Err(ElabError::ParseError {
                        msg: "unterminated string literal".to_string(),
                        span: Span::new(start, self.pos),
                    });
                }
                Some('"') => {
                    self.advance();
                    return Ok(s);
                }
                Some('\\') => {
                    let backslash_start = self.pos;
                    self.advance();
                    match self.scan_escape(backslash_start, '"')? {
                        EscapeShape::Common(c) | EscapeShape::Unicode(c) => s.push(c),
                        EscapeShape::Byte(_) => {
                            return Err(ElabError::InvalidEscape {
                                span: Span::new(backslash_start, self.pos),
                                reason: "\\xHH is only valid in a byte string".to_string(),
                            });
                        }
                    }
                }
                Some(c) => {
                    self.advance();
                    s.push(c);
                }
            }
        }
    }

    /// Scan a character literal body up to its closing `'`, then enforce the
    /// exactly-one-scalar cardinality rule (`31 §3`) -- a validity check
    /// applied AFTER decoding, distinct from `InvalidEscape` and not pinned
    /// to a specific name/span by the seed.
    fn scan_char_body(&mut self, start: usize) -> Result<char, ElabError> {
        let mut s = String::new();
        loop {
            match self.cur() {
                None | Some('\n') => {
                    return Err(ElabError::ParseError {
                        msg: "unterminated character literal".to_string(),
                        span: Span::new(start, self.pos),
                    });
                }
                Some('\'') => {
                    self.advance();
                    break;
                }
                Some('\\') => {
                    let backslash_start = self.pos;
                    self.advance();
                    match self.scan_escape(backslash_start, '\'')? {
                        EscapeShape::Common(c) | EscapeShape::Unicode(c) => s.push(c),
                        EscapeShape::Byte(_) => {
                            return Err(ElabError::InvalidEscape {
                                span: Span::new(backslash_start, self.pos),
                                reason: "\\xHH is only valid in a byte string".to_string(),
                            });
                        }
                    }
                }
                Some(c) => {
                    self.advance();
                    s.push(c);
                }
            }
        }
        let mut chars = s.chars();
        match (chars.next(), chars.next()) {
            (Some(c), None) => Ok(c),
            _ => Err(ElabError::ParseError {
                msg: format!(
                    "character literal must contain exactly one scalar, found {}",
                    s.chars().count()
                ),
                span: Span::new(start, self.pos),
            }),
        }
    }

    /// Scan a byte-string body up to its closing `"`. Unescaped body
    /// characters must be ASCII (`31 §3`); a non-ASCII unescaped scalar is a
    /// distinct, unpinned diagnostic, never implicit UTF-8 encoding.
    fn scan_byte_string_body(&mut self, start: usize) -> Result<Vec<u8>, ElabError> {
        let mut bytes = Vec::new();
        loop {
            match self.cur() {
                None | Some('\n') => {
                    return Err(ElabError::ParseError {
                        msg: "unterminated byte string literal".to_string(),
                        span: Span::new(start, self.pos),
                    });
                }
                Some('"') => {
                    self.advance();
                    return Ok(bytes);
                }
                Some('\\') => {
                    let backslash_start = self.pos;
                    self.advance();
                    match self.scan_escape(backslash_start, '"')? {
                        EscapeShape::Common(c) => bytes.push(c as u8),
                        EscapeShape::Byte(b) => bytes.push(b),
                        EscapeShape::Unicode(_) => {
                            return Err(ElabError::InvalidEscape {
                                span: Span::new(backslash_start, self.pos),
                                reason: "\\u{...} is not valid in a byte string".to_string(),
                            });
                        }
                    }
                }
                Some(c) if c.is_ascii() => {
                    self.advance();
                    bytes.push(c as u8);
                }
                Some(c) => {
                    let char_start = self.pos;
                    self.advance();
                    return Err(ElabError::ParseError {
                        msg: format!("non-ASCII character '{c}' in byte string literal"),
                        span: Span::new(char_start, self.pos),
                    });
                }
            }
        }
    }

    /// Scan a raw triple-quoted string body up to its closing `"""`. No
    /// escape processing: a backslash is ordinary content (`31 §3`, D4).
    fn scan_raw_triple_string_body(&mut self, start: usize) -> Result<String, ElabError> {
        let mut s = String::new();
        loop {
            if self.src[self.pos..].starts_with("\"\"\"") {
                self.advance();
                self.advance();
                self.advance();
                return Ok(s);
            }
            match self.cur() {
                None => {
                    return Err(ElabError::ParseError {
                        msg: "unterminated raw string literal".to_string(),
                        span: Span::new(start, self.pos),
                    });
                }
                Some(c) => {
                    self.advance();
                    s.push(c);
                }
            }
        }
    }

    pub fn next_token(&mut self) -> Result<(Token, Span), ElabError> {
        let result = self.next_token_inner()?;
        self.previous_token_was_dot = matches!(&result.0, Token::Dot);
        Ok(result)
    }

    fn next_token_inner(&mut self) -> Result<(Token, Span), ElabError> {
        self.skip_ws_comments()?;
        let start = self.pos;

        let c = match self.cur() {
            None => return Ok((Token::Eof, Span::new(start, start))),
            Some(c) => c,
        };

        // Single-char and multi-char punctuation
        match c {
            '(' => {
                self.advance();
                return Ok((Token::LParen, Span::new(start, self.pos)));
            }
            ')' => {
                self.advance();
                return Ok((Token::RParen, Span::new(start, self.pos)));
            }
            '{' => {
                self.advance();
                return Ok((Token::LBrace, Span::new(start, self.pos)));
            }
            '}' => {
                self.advance();
                return Ok((Token::RBrace, Span::new(start, self.pos)));
            }
            '[' => {
                self.advance();
                return Ok((Token::LBracket, Span::new(start, self.pos)));
            }
            ']' => {
                self.advance();
                return Ok((Token::RBracket, Span::new(start, self.pos)));
            }
            ',' => {
                self.advance();
                return Ok((Token::Comma, Span::new(start, self.pos)));
            }
            '"' if self.src[self.pos..].starts_with("\"\"\"") => {
                self.advance();
                self.advance();
                self.advance(); // consume opening '"""'
                let s = self.scan_raw_triple_string_body(start)?;
                return Ok((Token::Str(s), Span::new(start, self.pos)));
            }
            '"' => {
                self.advance(); // consume opening '"'
                let s = self.scan_string_body(start)?;
                return Ok((Token::Str(s), Span::new(start, self.pos)));
            }
            '\'' => {
                self.advance(); // consume opening '\''
                let c = self.scan_char_body(start)?;
                return Ok((Token::CharLit(c), Span::new(start, self.pos)));
            }
            '|' => {
                self.advance();
                if self.src[self.pos..].starts_with("->") {
                    self.advance();
                    self.advance();
                    return Ok((Token::MapsTo, Span::new(start, self.pos)));
                }
                // ASCII spelling of `‖` (16 §6): two adjacent `|` with no
                // intervening whitespace. Checked after `|->` so that
                // ambiguity is impossible; a lone `|` remains `Pipe` (match
                // arm separator), unaffected.
                if self.cur() == Some('|') {
                    self.advance();
                    return Ok((Token::TruncBar, Span::new(start, self.pos)));
                }
                return Ok((Token::Pipe, Span::new(start, self.pos)));
            }
            ';' => {
                self.advance();
                return Ok((Token::Semicolon, Span::new(start, self.pos)));
            }
            ':' => {
                self.advance();
                if self.cur() == Some(':') {
                    self.advance();
                    return Ok((Token::DoubleColon, Span::new(start, self.pos)));
                }
                return Ok((Token::Colon, Span::new(start, self.pos)));
            }
            '=' => {
                self.advance();
                if self.cur() == Some('=') {
                    self.advance();
                    if self.cur() == Some('=') {
                        self.advance();
                        return Ok((Token::PropEq, Span::new(start, self.pos)));
                    }
                    return Ok((Token::EqEq, Span::new(start, self.pos)));
                }
                return Ok((Token::Eq, Span::new(start, self.pos)));
            }
            '.' => {
                self.advance();
                return Ok((Token::Dot, Span::new(start, self.pos)));
            }
            '\\' => {
                self.advance();
                if self.cur() == Some('/') {
                    self.advance();
                    return Ok((Token::Or, Span::new(start, self.pos)));
                }
                return Ok((Token::Lambda, Span::new(start, self.pos)));
            }
            'λ' => {
                self.advance();
                return Ok((Token::Lambda, Span::new(start, self.pos)));
            }
            '→' => {
                self.advance();
                return Ok((Token::Arrow, Span::new(start, self.pos)));
            }
            '↦' => {
                self.advance();
                return Ok((Token::MapsTo, Span::new(start, self.pos)));
            }
            '≡' => {
                self.advance();
                return Ok((Token::PropEq, Span::new(start, self.pos)));
            }
            '≤' => {
                self.advance();
                return Ok((Token::Le, Span::new(start, self.pos)));
            }
            '≥' => {
                self.advance();
                return Ok((Token::Ge, Span::new(start, self.pos)));
            }
            '≠' => {
                self.advance();
                return Ok((Token::Ne, Span::new(start, self.pos)));
            }
            '∧' => {
                self.advance();
                return Ok((Token::And, Span::new(start, self.pos)));
            }
            '∨' => {
                self.advance();
                return Ok((Token::Or, Span::new(start, self.pos)));
            }
            '⊑' => {
                self.advance();
                return Ok((Token::FlowsTo, Span::new(start, self.pos)));
            }
            '⊔' => {
                self.advance();
                return Ok((Token::Join, Span::new(start, self.pos)));
            }
            '⊓' => {
                self.advance();
                return Ok((Token::Meet, Span::new(start, self.pos)));
            }
            '×' => {
                self.advance();
                return Ok((Token::Times, Span::new(start, self.pos)));
            }
            '‖' => {
                self.advance();
                return Ok((Token::TruncBar, Span::new(start, self.pos)));
            }
            'Ω' => {
                self.advance();
                return Ok((
                    Token::ConId("Omega".to_string()),
                    Span::new(start, self.pos),
                ));
            }
            'Σ' => {
                self.advance();
                return Ok((
                    Token::ConId("Sigma".to_string()),
                    Span::new(start, self.pos),
                ));
            }
            'Π' => {
                self.advance();
                return Ok((Token::ConId("Pi".to_string()), Span::new(start, self.pos)));
            }
            '∀' => {
                self.advance();
                return Ok((
                    Token::Ident("forall".to_string()),
                    Span::new(start, self.pos),
                ));
            }
            '∃' => {
                self.advance();
                return Ok((
                    Token::Ident("exists".to_string()),
                    Span::new(start, self.pos),
                ));
            }
            '¬' => {
                self.advance();
                return Ok((Token::Ident("not".to_string()), Span::new(start, self.pos)));
            }
            '∈' => {
                self.advance();
                return Ok((Token::Member, Span::new(start, self.pos)));
            }
            'ℓ' => {
                self.advance();
                return Ok((
                    Token::Ident("level".to_string()),
                    Span::new(start, self.pos),
                ));
            }
            '+' => {
                self.advance();
                if self.cur() == Some('%') {
                    self.advance();
                    return Ok((Token::PlusPercent, Span::new(start, self.pos)));
                }
                return Ok((Token::Plus, Span::new(start, self.pos)));
            }
            '*' => {
                self.advance();
                return Ok((Token::Star, Span::new(start, self.pos)));
            }
            '-' => {
                self.advance();
                if self.cur() == Some('>') {
                    self.advance();
                    return Ok((Token::Arrow, Span::new(start, self.pos)));
                }
                return Ok((Token::Minus, Span::new(start, self.pos)));
            }
            '<' => {
                self.advance();
                if self.cur() == Some('=') {
                    self.advance();
                    return Ok((Token::Le, Span::new(start, self.pos)));
                }
                if self.cur() == Some(':') {
                    self.advance();
                    return Ok((Token::FlowsTo, Span::new(start, self.pos)));
                }
            }
            '>' => {
                self.advance();
                if self.cur() == Some('=') {
                    self.advance();
                    return Ok((Token::Ge, Span::new(start, self.pos)));
                }
                if self.cur() == Some('<') {
                    self.advance();
                    return Ok((Token::Times, Span::new(start, self.pos)));
                }
            }
            '/' => {
                self.advance();
                if self.cur() == Some('=') {
                    self.advance();
                    return Ok((Token::Ne, Span::new(start, self.pos)));
                }
                if self.cur() == Some('\\') {
                    self.advance();
                    return Ok((Token::And, Span::new(start, self.pos)));
                }
            }
            _ => {}
        }

        // Numeric literals: starts with a digit
        if c.is_ascii_digit() {
            if self.src[self.pos..].starts_with("0x") || self.src[self.pos..].starts_with("0X") {
                let mut token_tail = String::new();
                let mut exponent = false;
                let mut exp_sign = false;
                for ch in self.src[self.pos + 2..].chars() {
                    if !exponent {
                        if ch.is_ascii_hexdigit() || ch == '_' || ch == '.' { token_tail.push(ch); continue; }
                        if ch == 'p' || ch == 'P' { exponent = true; token_tail.push(ch); continue; }
                        break;
                    }
                    if !exp_sign && (ch == '+' || ch == '-') { exp_sign = true; token_tail.push(ch); continue; }
                    if ch.is_ascii_digit() || ch == '_' { exp_sign = true; token_tail.push(ch); continue; }
                    break;
                }
                if token_tail.chars().any(|c| c == '.' || c == 'p' || c == 'P') {
                    return self.lex_hex_float(start);
                }
                return self.lex_radix_integer(start);
            }
            if self.src[self.pos..].starts_with("0b") || self.src[self.pos..].starts_with("0B") || self.src[self.pos..].starts_with("0o") || self.src[self.pos..].starts_with("0O") { return self.lex_radix_integer(start); }
            return self.lex_numeric(start);
        }

        // Byte string `b"…"` (`31 §3`) -- must precede identifier scanning,
        // since 'b' is otherwise an ordinary lowercase identifier start.
        if c == 'b' && self.src[self.pos..].starts_with("b\"") {
            self.advance(); // consume 'b'
            self.advance(); // consume opening '"'
            let bytes = self.scan_byte_string_body(start)?;
            return Ok((Token::ByteStr(bytes), Span::new(start, self.pos)));
        }

        // Identifiers and keywords
        if c.is_ascii_alphabetic() || c == '_' {
            let mut s = String::new();
            while self
                .cur()
                .map(Self::is_ascii_ident_continue)
                .unwrap_or(false)
            {
                s.push(self.advance().unwrap());
            }
            let tok = match s.as_str() {
                "const" => Token::KwConst,
                "fn" => Token::KwFn,
                "proc" => Token::KwProc,
                "let" => Token::KwLet,
                "in" => Token::KwIn,
                "if" => Token::KwIf,
                "then" => Token::KwThen,
                "else" => Token::KwElse,
                "Type" => Token::KwType,
                "requires" => Token::KwRequires,
                "ensures" => Token::KwEnsures,
                "prove" => Token::KwProve,
                "law" => Token::KwLaw,
                "old" => Token::KwOld,
                "space" => Token::KwSpace,
                "mut" => Token::KwMut,
                "becomes" => Token::KwBecomes,
                "data" => Token::KwData,
                "match" => Token::KwMatch,
                "def" => Token::KwDef,
                "type" => Token::KwTypeReserved,
                "foreign" => Token::KwForeign,
                "temporal" => Token::KwTemporal,
                "record" => Token::KwRecord,
                "class" => Token::KwClass,
                "instance" => Token::KwInstance,
                "derive" => Token::KwDerive,
                "where" => Token::KwWhere,
                "module" => Token::KwModule,
                "import" => Token::KwImport,
                "export" => Token::KwExport,
                "use" => Token::KwUseReserved,
                "pub" => Token::KwPub,
                "program" => Token::KwProgram,
                "package" => Token::KwPackage,
                "admits" => Token::KwAdmits,
                "capabilities" => Token::KwCapabilities,
                "prop" => Token::KwProp,
                "theorem" => Token::KwTheorem,
                "axiom" => Token::KwAxiom,
                "proof" => Token::KwProof,
                // `true`/`false` -- Bool literals (`31 §3`, `:512`). The
                // prelude's `Bool` already has `True`/`False` constructors
                // (prelude.rs), so this is a pure lexical spelling: route
                // straight to the existing ConId path rather than adding a
                // literal token/AST/elaboration path. `True`/`False`
                // (capitalized) already resolve via the fallthrough below,
                // unaffected -- this adds the lowercase spelling alongside it.
                "true" => Token::ConId("True".to_string()),
                "false" => Token::ConId("False".to_string()),
                "l" => Token::Ident("level".to_string()),
                _ => {
                    let first = s.chars().next().unwrap();
                    if first.is_ascii_uppercase() {
                        Token::ConId(s)
                    } else {
                        Token::Ident(s)
                    }
                }
            };
            return Ok((tok, Span::new(start, self.pos)));
        }

        // Shape B (`SURF-IDENT-TR39-R1`): identifier letters remain ASCII-only
        // deliberately. Blessed Unicode notation has already been recognized by
        // the operator cases above; any other alphabetic scalar is an identifier
        // candidate and receives a typed error rather than a generic parse error.
        if c.is_alphabetic() {
            return Err(ElabError::NonAsciiIdentifierCharacter {
                character: c,
                span: Span::new(start, start + c.len_utf8()),
            });
        }

        Err(ElabError::ParseError {
            msg: format!("unexpected character '{}'", c),
            span: Span::new(start, start + c.len_utf8()),
        })
    }

    /// Lex a numeric literal starting at `start`.
    /// Handles: integer, large-integer, float, decimal (`d`-suffix),
    /// float32 (`f32`-suffix).
    fn lex_numeric(&mut self, start: usize) -> Result<(Token, Span), ElabError> {
        // Read integer part
        let mut int_str = String::new();
        while self.cur().map(|c| c.is_ascii_digit() || c == '_').unwrap_or(false) {
            let c = self.advance().unwrap();
            if c == '_' {
                if !self.cur().map(|n| n.is_ascii_digit()).unwrap_or(false)
                    || int_str.is_empty()
                {
                    return Err(ElabError::ParseError { msg: "digit separator must occur between digits".into(), span: Span::new(start, self.pos) });
                }
            } else { int_str.push(c); }
        }

        // Optional fractional part
        let mut has_dot = false;
        let mut frac_str = String::new();
        let mut frac_places: i32 = 0;
        // A numeric token after an emitted `.` is a positional-projection
        // index, even when whitespace or a comment separates the two tokens.
        // Keeping this decision in the lexer preserves both halves of the
        // lexical contract: `p.1.2` is two projections, while an ordinary
        // `3.14` remains one float literal.
        let follows_dot = self.previous_token_was_dot;
        if !follows_dot
            && self.cur() == Some('.')
            && self.src[self.pos + 1..]
                .chars()
                .next()
                .map(|c| c.is_ascii_digit())
                .unwrap_or(false)
        {
            self.advance(); // consume '.'
            has_dot = true;
            while self.cur().map(|c| c.is_ascii_digit() || c == '_').unwrap_or(false) {
                let c = self.advance().unwrap();
                if c == '_' {
                    if !self.cur().map(|n| n.is_ascii_digit()).unwrap_or(false) || frac_str.is_empty() {
                        return Err(ElabError::ParseError { msg: "digit separator must occur between digits".into(), span: Span::new(start, self.pos) });
                    }
                } else { frac_str.push(c); frac_places += 1; }
            }
        }
        if self.cur() == Some('.')
            && self.src[self.pos + 1..].starts_with('_')
        {
            return Err(ElabError::ParseError { msg: "digit separator must occur between digits".into(), span: Span::new(start, self.pos + 2) });
        }

        // Optional exponent (for FloatLit only)
        let mut exp_str = String::new();
        if self.cur() == Some('e') || self.cur() == Some('E') {
            exp_str.push(self.advance().unwrap());
            if self.cur() == Some('+') || self.cur() == Some('-') {
                exp_str.push(self.advance().unwrap());
            }
            while self.cur().map(|c| c.is_ascii_digit() || c == '_').unwrap_or(false) {
                let c = self.advance().unwrap();
                if c == '_' {
                    if !self.cur().map(|n| n.is_ascii_digit()).unwrap_or(false)
                        || !exp_str.chars().last().is_some_and(|c| c.is_ascii_digit())
                    {
                        return Err(ElabError::ParseError {
                            msg: "digit separator must occur between digits".into(),
                            span: Span::new(start, self.pos),
                        });
                    }
                } else {
                    exp_str.push(c);
                }
            }
            if exp_str.len() == 1 || (exp_str.len() == 2 && matches!(exp_str.as_bytes()[1], b'+' | b'-')) {
                return Err(ElabError::ParseError {
                    msg: "exponent requires at least one digit".into(),
                    span: Span::new(start, self.pos),
                });
            }
        }

        if !exp_str.is_empty()
            && (self.cur() == Some('d') || self.src[self.pos..].starts_with("f32"))
        {
            return Err(ElabError::ParseError {
                msg: "exponent suffix combinations are not supported".into(),
                span: Span::new(start, self.pos),
            });
        }

        // Check for `d` suffix → Decimal
        if self.cur() == Some('d')
            && !self.src[self.pos + 1..]
                .chars()
                .next()
                .map(Self::is_ascii_ident_continue)
                .unwrap_or(false)
        {
            self.advance(); // consume 'd'
            let coeff_str = format!("{}{}", int_str, frac_str);
            let coeff: BigInt = coeff_str.parse().map_err(|_| ElabError::ParseError {
                msg: format!("decimal literal coefficient too large: {}", coeff_str),
                span: Span::new(start, self.pos),
            })?;
            let exp: i32 = -frac_places;
            return Ok((Token::DecimalLit(coeff, exp), Span::new(start, self.pos)));
        }

        // Check for `f32` suffix → Float32Lit
        if self.src[self.pos..].starts_with("f32")
            && !self.src[self.pos + 3..]
                .chars()
                .next()
                .map(Self::is_ascii_ident_continue)
                .unwrap_or(false)
        {
            self.advance();
            self.advance();
            self.advance(); // consume "f32"
            let s = if has_dot {
                format!("{}.{}", int_str, frac_str)
            } else {
                int_str.clone()
            };
            let f: f32 = s.parse().map_err(|_| ElabError::ParseError {
                msg: "invalid float literal".into(), span: Span::new(start, self.pos)
            })?;
            return Ok((Token::Float32Lit(f), Span::new(start, self.pos)));
        }

        // Float if has dot or exponent
        if has_dot || !exp_str.is_empty() {
            let s = if exp_str.is_empty() {
                format!("{}.{}", int_str, frac_str)
            } else if has_dot {
                format!("{}.{}{}", int_str, frac_str, exp_str)
            } else {
                format!("{}{}", int_str, exp_str)
            };
            let f: f64 = s.parse().map_err(|_| ElabError::ParseError {
                msg: "invalid float literal".into(), span: Span::new(start, self.pos)
            })?;
            return Ok((Token::FloatLit(f), Span::new(start, self.pos)));
        }

        // Plain integer
        let n: BigInt = int_str.parse().map_err(|_| ElabError::ParseError {
            msg: format!("integer literal too large: {}", int_str),
            span: Span::new(start, self.pos),
        })?;
        if let Ok(nat) = int_str.parse::<u32>() {
            Ok((Token::Nat(nat), Span::new(start, self.pos)))
        } else {
            Ok((Token::IntLit(n), Span::new(start, self.pos)))
        }
    }

    fn lex_radix_integer(&mut self, start: usize) -> Result<(Token, Span), ElabError> {
        self.advance();
        let base_ch = self.advance().unwrap();
        let radix = match base_ch.to_ascii_lowercase() { 'x' => 16, 'b' => 2, 'o' => 8, _ => unreachable!() };
        let mut digits = String::new();
        while let Some(c) = self.cur() {
            if c == '_' || c.is_ascii_hexdigit() {
                self.advance();
                if c == '_' {
                    if digits.is_empty() || !self.cur().map(|n| n.is_ascii_hexdigit()).unwrap_or(false) {
                        return Err(ElabError::ParseError { msg: "digit separator must occur between digits".into(), span: Span::new(start, self.pos) });
                    }
                } else { digits.push(c); }
            } else { break; }
        }
        if digits.is_empty() || !digits.chars().all(|c| c.to_digit(radix).is_some()) {
            return Err(ElabError::ParseError { msg: "invalid radix integer".into(), span: Span::new(start, self.pos) });
        }
        let n = BigInt::parse_bytes(digits.as_bytes(), radix).ok_or_else(|| ElabError::ParseError { msg: "invalid radix integer".into(), span: Span::new(start, self.pos) })?;
        if let Ok(nat) = n.to_string().parse::<u32>() { Ok((Token::Nat(nat), Span::new(start, self.pos))) } else { Ok((Token::IntLit(n), Span::new(start, self.pos))) }
    }

    fn lex_hex_float(&mut self, start: usize) -> Result<(Token, Span), ElabError> {
        self.advance(); self.advance();
        let mut digits = String::new();
        let mut frac = 0i32;
        let mut after_dot = false;
        while let Some(c) = self.cur() {
            if c == '.' { if after_dot { break; } after_dot = true; self.advance(); continue; }
            if c == '_' { self.advance(); if digits.is_empty() || !self.src[..self.pos - 1].chars().last().map(|p| p.is_ascii_hexdigit()).unwrap_or(false) || !self.cur().map(|n| n.is_ascii_hexdigit()).unwrap_or(false) { return Err(ElabError::ParseError { msg: "digit separator must occur between digits".into(), span: Span::new(start, self.pos) }); } continue; }
            if let Some(_) = c.to_digit(16) { self.advance(); digits.push(c); if after_dot { frac += 1; } } else { break; }
        }
        if digits.is_empty() || self.cur().map(|c| c == 'p' || c == 'P').unwrap_or(false) == false { return Err(ElabError::ParseError { msg: "hex float requires p exponent".into(), span: Span::new(start, self.pos) }); }
        self.advance(); let mut sign = 1i32; if self.cur() == Some('+') { self.advance(); } else if self.cur() == Some('-') { sign = -1; self.advance(); }
        let mut exp = String::new(); while self.cur().map(|c| c.is_ascii_digit() || c == '_').unwrap_or(false) { let c=self.advance().unwrap(); if c=='_' { if exp.is_empty() || !exp.chars().last().unwrap().is_ascii_digit() || !self.cur().map(|n| n.is_ascii_digit()).unwrap_or(false) { return Err(ElabError::ParseError { msg:"digit separator must occur between digits".into(), span:Span::new(start,self.pos)}); } } else { exp.push(c); } }
        if exp.is_empty() { return Err(ElabError::ParseError { msg:"hex float exponent requires digits".into(), span:Span::new(start,self.pos)}); }
        let mant = BigInt::parse_bytes(digits.as_bytes(),16).unwrap();
        let binary_exp = sign.checked_mul(exp.parse::<i32>().map_err(|_| ElabError::ParseError { msg:"hex exponent out of range".into(), span:Span::new(start,self.pos) })?).and_then(|e| e.checked_sub(4 * frac)).ok_or_else(|| ElabError::ParseError { msg:"hex exponent out of range".into(), span:Span::new(start,self.pos) })?;
        let value = Self::hex_mantissa_to_f64(&mant, binary_exp).ok_or_else(|| ElabError::ParseError { msg:"hex float out of range".into(), span:Span::new(start,self.pos) })?;
        Ok((Token::FloatLit(value), Span::new(start,self.pos)))
    }

fn hex_mantissa_to_f64(m: &BigInt, shift: i32) -> Option<f64> {
    let k = m.bits() as i32; if k == 0 { return Some(0.0); }
    let mut e = k - 1 + shift; if e > 1023 { return Some(f64::INFINITY); }
    if e < -1022 {
        let s = shift + 1074;
        let q = if s >= 0 { m << s } else {
            let n = -s; let mut q = m >> n; let r = m - (&q << n); let half = BigInt::from(1) << (n - 1);
            if r > half || (r == half && q.to_string().parse::<u64>().ok().is_some_and(|v| v & 1 == 1)) { q += 1; } q
        };
        let bits: u64 = q.to_string().parse().ok()?; return Some(f64::from_bits(bits));
    }
    let keep = if k > 53 { m >> (k - 53) } else { m << (53 - k) };
    let mut q: u64 = keep.to_string().parse().ok()?;
    if k > 53 {
        let r = m - (&keep << (k - 53)); let half = BigInt::from(1) << (k - 54);
        if r > half || (r == half && (q & 1) == 1) { q += 1; }
        if q == (1u64 << 53) { q >>= 1; e += 1; }
    }
    if e > 1023 { return Some(f64::INFINITY); }
    Some(f64::from_bits((((e + 1023) as u64) << 52) | (q & ((1u64 << 52) - 1))))
}

    /// Lex the entire source into a token+span list (including the `Eof`
    /// sentinel).
    pub fn lex(src: &'s str) -> Result<Vec<(Token, Span)>, ElabError> {
        let mut lx = Self::new(src);
        let mut out = Vec::new();
        loop {
            let (tok, span) = lx.next_token()?;
            let done = tok == Token::Eof;
            out.push((tok, span));
            if done {
                break;
            }
        }
        Ok(out)
    }
}
