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
    KwView,
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
    Str(String), // `"…"` — symbol name / library name in `foreign` decls
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
    // L1 numeric literal tokens
    IntLit(BigInt),       // integer literal too large for u32
    FloatLit(f64),        // decimal-point float: `3.14`, `1e-9`
    DecimalLit(BigInt, i32), // `d`-suffix: coeff × 10^exp; e.g. `0.1d` → (1,-1)
    Float32Lit(f32),      // `f32`-suffix: `1.5f32`
    // Atoms
    Ident(String), // lowercase-initial term variable
    ConId(String), // uppercase-initial base type / constructor
    Nat(u32),      // small non-negative integer (≤ u32::MAX); also a level digit
    Eof,
}

pub struct Lexer<'s> {
    src: &'s str,
    pos: usize,
    previous_token_was_dot: bool,
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

    fn skip_ws_comments(&mut self) {
        loop {
            while self.cur().map(|c| c.is_whitespace()).unwrap_or(false) {
                self.advance();
            }
            if self.src[self.pos..].starts_with("--") {
                while self.cur().map(|c| c != '\n').unwrap_or(false) {
                    self.advance();
                }
            } else {
                break;
            }
        }
    }

    fn is_ascii_ident_continue(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_' || c == '\''
    }

    pub fn next_token(&mut self) -> Result<(Token, Span), ElabError> {
        let result = self.next_token_inner()?;
        self.previous_token_was_dot = matches!(&result.0, Token::Dot);
        Ok(result)
    }

    fn next_token_inner(&mut self) -> Result<(Token, Span), ElabError> {
        self.skip_ws_comments();
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
            '"' => {
                self.advance(); // consume opening '"'
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
                            self.advance(); // consume closing '"'
                            break;
                        }
                        Some(c) => {
                            self.advance();
                            s.push(c);
                        }
                    }
                }
                return Ok((Token::Str(s), Span::new(start, self.pos)));
            }
            '|' => {
                self.advance();
                if self.src[self.pos..].starts_with("->") {
                    self.advance();
                    self.advance();
                    return Ok((Token::MapsTo, Span::new(start, self.pos)));
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
            if self.src[self.pos..].starts_with("0x")
                || self.src[self.pos..].starts_with("0X")
                || self.src[self.pos..].starts_with("0b")
                || self.src[self.pos..].starts_with("0B")
                || self.src[self.pos..].starts_with("0o")
                || self.src[self.pos..].starts_with("0O")
            {
                if self.src[self.pos + 2..].chars().any(|c| c == '.' || c == 'p' || c == 'P') {
                    return self.lex_hex_float(start);
                }
                return self.lex_radix_integer(start);
            }
            return self.lex_numeric(start);
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
                "view" => Token::KwView,
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
            if c == '_' { self.advance(); if digits.is_empty() || !self.cur().map(|n| n.is_ascii_hexdigit()).unwrap_or(false) { return Err(ElabError::ParseError { msg: "digit separator must occur between digits".into(), span: Span::new(start, self.pos) }); } continue; }
            if let Some(_) = c.to_digit(16) { self.advance(); digits.push(c); if after_dot { frac += 1; } } else { break; }
        }
        if digits.is_empty() || self.cur().map(|c| c == 'p' || c == 'P').unwrap_or(false) == false { return Err(ElabError::ParseError { msg: "hex float requires p exponent".into(), span: Span::new(start, self.pos) }); }
        self.advance(); let mut sign = 1i32; if self.cur() == Some('+') { self.advance(); } else if self.cur() == Some('-') { sign = -1; self.advance(); }
        let mut exp = String::new(); while self.cur().map(|c| c.is_ascii_digit() || c == '_').unwrap_or(false) { let c=self.advance().unwrap(); if c=='_' { if exp.is_empty() || !exp.chars().last().unwrap().is_ascii_digit() || !self.cur().map(|n| n.is_ascii_digit()).unwrap_or(false) { return Err(ElabError::ParseError { msg:"digit separator must occur between digits".into(), span:Span::new(start,self.pos)}); } } else { exp.push(c); } }
        if exp.is_empty() { return Err(ElabError::ParseError { msg:"hex float exponent requires digits".into(), span:Span::new(start,self.pos)}); }
        let mant = BigInt::parse_bytes(digits.as_bytes(),16).unwrap();
        let binary_exp = sign.checked_mul(exp.parse::<i32>().map_err(|_| ElabError::ParseError { msg:"hex exponent out of range".into(), span:Span::new(start,self.pos) })?).and_then(|e| e.checked_sub(4 * frac)).ok_or_else(|| ElabError::ParseError { msg:"hex exponent out of range".into(), span:Span::new(start,self.pos) })?;
        let value = hex_mantissa_to_f64(&mant, binary_exp).ok_or_else(|| ElabError::ParseError { msg:"hex float out of range".into(), span:Span::new(start,self.pos) })?;
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
    Some(f64::from_bits(((e + 1023) as u64 << 52) | (q & ((1u64 << 52) - 1))))
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
