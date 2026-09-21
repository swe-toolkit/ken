//! Canonical token spelling (`31 §1b`, `31 §1d`).
//!
//! Canonicalization consumes B1's lossless token/trivia partition.  It never
//! scans source bytes for alias-shaped substrings: notation is selected from
//! the parsed token kind, while every other token and all trivia retain their
//! original source lexeme. The sole sanctioned re-lexing exception is
//! `canonicalize_lexed_tokens`, used only for non-parseable `ken ignore` and
//! `ken reject` fence bodies.

use std::borrow::Cow;

use crate::error::{ElabError, Span};
use crate::lexer::{
    formatter_format_character_repairs, FormatCharacterEdit, Lexer, Token, ValidatedSource,
};
use crate::lossless::{parse_lossless, FormattableSource, SourcePieceKind};

/// Formatter-only rewrite plus the map from rewritten coordinates back to the
/// caller's original source. The repair observation itself stays lexer-internal;
/// this type carries only its edit plan and resulting text.
pub(crate) struct FormatterRewrite<'s> {
    source: Cow<'s, str>,
    edits: Vec<FormatCharacterEdit>,
}

impl FormatterRewrite<'_> {
    pub(crate) fn source(&self) -> &str {
        self.source.as_ref()
    }

    fn original_offset(&self, offset: usize, end_bias: bool) -> usize {
        let mut delta: isize = 0;
        for edit in &self.edits {
            let rewritten_start = (edit.span.start as isize + delta) as usize;
            let rewritten_end = rewritten_start + edit.replacement.len();
            if offset < rewritten_start {
                return (offset as isize - delta) as usize;
            }
            if offset < rewritten_end {
                return if end_bias {
                    edit.span.end
                } else {
                    edit.span.start
                };
            }
            if offset == rewritten_end {
                return edit.span.end;
            }
            delta += edit.replacement.len() as isize - (edit.span.end - edit.span.start) as isize;
        }
        (offset as isize - delta) as usize
    }

    fn original_span(&self, span: Span) -> Span {
        Span::new(
            self.original_offset(span.start, false),
            self.original_offset(span.end, true),
        )
    }

    /// Map every error the lexer/parser can raise while formatting back across
    /// the repair plan. Later elaboration errors never arise in this pipeline.
    pub(crate) fn original_error(&self, error: ElabError) -> ElabError {
        map_formatter_error_spans(error, |span| self.original_span(span))
    }
}

/// Transform spans on every error the lexer or parser can produce. Formatter
/// callers use this once for rewrite coordinates and, in literate source, once
/// more for fence-body coordinates.
pub(crate) fn map_formatter_error_spans(
    error: ElabError,
    mut map: impl FnMut(Span) -> Span,
) -> ElabError {
    match error {
        ElabError::ParseError { msg, span } => ElabError::ParseError {
            msg,
            span: map(span),
        },
        ElabError::NonAsciiIdentifierCharacter { character, span } => {
            ElabError::NonAsciiIdentifierCharacter {
                character,
                span: map(span),
            }
        }
        ElabError::RawFormatCharacter { character, span } => ElabError::RawFormatCharacter {
            character,
            span: map(span),
        },
        ElabError::InvalidEscape { span, reason } => ElabError::InvalidEscape {
            span: map(span),
            reason,
        },
        ElabError::ForeignNameControlCharacter {
            which,
            character,
            span,
        } => ElabError::ForeignNameControlCharacter {
            which,
            character,
            span: map(span),
        },
        ElabError::NamedBoundaryHeader { name, span } => ElabError::NamedBoundaryHeader {
            name,
            span: map(span),
        },
        ElabError::UnknownCapabilityFamily { family, span } => ElabError::UnknownCapabilityFamily {
            family,
            span: map(span),
        },
        ElabError::InvalidCapabilityAuthority {
            family,
            authority,
            span,
        } => ElabError::InvalidCapabilityAuthority {
            family,
            authority,
            span: map(span),
        },
        ElabError::DuplicateCapabilityFamily { family, span } => {
            ElabError::DuplicateCapabilityFamily {
                family,
                span: map(span),
            }
        }
        ElabError::PackageCapabilitiesNotAllowed { span } => {
            ElabError::PackageCapabilitiesNotAllowed { span: map(span) }
        }
        ElabError::MutationOutsideSpace { construct, span } => ElabError::MutationOutsideSpace {
            construct,
            span: map(span),
        },
        ElabError::UnsupportedSpacePlacement { placement, span } => {
            ElabError::UnsupportedSpacePlacement {
                placement,
                span: map(span),
            }
        }
        ElabError::InvalidFixityPrecedence { written, span } => {
            ElabError::InvalidFixityPrecedence {
                written,
                span: map(span),
            }
        }
        other => other,
    }
}

/// Build the formatter-only inverse of raw `Cf` literal spellings. Edits are
/// observed over original coordinates and applied from right to left.
pub(crate) fn rewrite_format_characters_for_formatter(src: &str) -> FormatterRewrite<'_> {
    let edits = formatter_format_character_repairs(src);
    debug_assert!(edits
        .windows(2)
        .all(|pair| pair[0].span.end <= pair[1].span.start));
    if edits.is_empty() {
        return FormatterRewrite {
            source: Cow::Borrowed(src),
            edits,
        };
    }

    let mut rewritten = src.to_owned();
    for edit in edits.iter().rev() {
        rewritten.replace_range(edit.span.start..edit.span.end, &edit.replacement);
    }
    FormatterRewrite {
        source: Cow::Owned(rewritten),
        edits,
    }
}

/// The blessed spelling for an unambiguous notation token kind.
pub(crate) fn canonical_token_spelling(token: &Token) -> Option<&'static str> {
    match token {
        Token::Arrow => Some("→"),
        Token::MapsTo => Some("↦"),
        Token::Lambda => Some("λ"),
        Token::PropEq => Some("≡"),
        Token::Le => Some("≤"),
        Token::Ge => Some("≥"),
        Token::Ne => Some("≠"),
        Token::And => Some("∧"),
        Token::Or => Some("∨"),
        Token::FlowsTo => Some("⊑"),
        Token::Times => Some("×"),
        Token::TruncBar => Some("‖"),
        _ => None,
    }
}

/// Canonicalize notation spellings over an already-parsed B1 source stream.
///
/// Layout, identifiers, keywords, literals, comments, foreign string payloads,
/// and temporal formula bodies are replayed from their original source spans.
pub fn canonicalize_tokens(source: &dyn FormattableSource) -> String {
    let mut out = String::with_capacity(source.source().len());
    let mut temporal_brace_depth: Option<usize> = None;
    let mut temporal_pending_brace = false;

    for piece in source.pieces() {
        let lexeme = &source.source()[piece.span.start..piece.span.end];
        let SourcePieceKind::Token(token_index) = piece.kind else {
            out.push_str(lexeme);
            continue;
        };
        let token = &source.tokens()[token_index].kind;

        // Temporal formula text is a protected payload.  Its braces and every
        // token between them are replayed verbatim; the declaration keyword
        // and name remain ordinary stored spellings as well.
        if temporal_pending_brace {
            out.push_str(lexeme);
            if matches!(token, Token::LBrace) {
                temporal_pending_brace = false;
                temporal_brace_depth = Some(1);
            }
            continue;
        }
        if let Some(depth) = temporal_brace_depth.as_mut() {
            out.push_str(lexeme);
            match token {
                Token::LBrace => *depth += 1,
                Token::RBrace if *depth == 1 => temporal_brace_depth = None,
                Token::RBrace => *depth -= 1,
                _ => {}
            }
            continue;
        }
        if matches!(token, Token::KwTemporal) {
            temporal_pending_brace = true;
            out.push_str(lexeme);
            continue;
        }

        if let Some(canonical) = canonical_token_spelling(token) {
            out.push_str(canonical);
        } else {
            out.push_str(lexeme);
        }
    }

    out
}

fn recovery_cursor_after(src: &str, cursor: usize, relative_end: usize) -> usize {
    let minimum = src[cursor..]
        .chars()
        .next()
        .map(char::len_utf8)
        .unwrap_or(1);
    let mut next = cursor
        .saturating_add(relative_end.max(minimum))
        .min(src.len());
    while next < src.len() && !src.is_char_boundary(next) {
        next += 1;
    }
    next
}

/// Canonicalize recognizable notation tokens in a fragment that need not
/// parse. Unrecognized bytes, layout, comments, strings, and temporal payloads
/// are replayed verbatim. This is the narrow B4 fallback for incomplete
/// `` ```ken ignore `` and syntax-erroring `` ```ken reject `` bodies.
pub fn canonicalize_lexed_tokens(src: &str) -> Result<String, ElabError> {
    let rewrite = rewrite_format_characters_for_formatter(src);
    let rewritten = rewrite.source();

    // Validate once at the whole-source boundary after the formatter-only
    // repair. Recovery lexers below are suffix views of this proof, never
    // independent sources with a fresh BOM offset or a validation bypass.
    let source = ValidatedSource::new(rewritten).map_err(|error| rewrite.original_error(error))?;
    let mut tokens = Vec::new();
    let mut cursor = 0usize;
    while cursor < rewritten.len() {
        let mut lexer = Lexer::new(source.suffix_from(cursor));
        match lexer.next_token() {
            Ok((Token::Eof, _)) => break,
            Ok((token, span)) => {
                let absolute = Span::new(cursor + span.start, cursor + span.end);
                cursor = recovery_cursor_after(rewritten, cursor, span.end);
                tokens.push((token, absolute));
            }
            Err(ElabError::ParseError { span, .. }) => {
                cursor = recovery_cursor_after(rewritten, cursor, span.end);
            }
            Err(error) => {
                let whole_fragment_error = map_formatter_error_spans(error, |span| {
                    Span::new(cursor + span.start, cursor + span.end)
                });
                return Err(rewrite.original_error(whole_fragment_error));
            }
        }
    }

    let mut replacements = Vec::new();
    let mut temporal_brace_depth: Option<usize> = None;
    let mut temporal_pending_brace = false;
    for (token, span) in tokens {
        if temporal_pending_brace {
            if matches!(token, Token::LBrace) {
                temporal_pending_brace = false;
                temporal_brace_depth = Some(1);
            }
            continue;
        }
        if let Some(depth) = temporal_brace_depth.as_mut() {
            match token {
                Token::LBrace => *depth += 1,
                Token::RBrace if *depth == 1 => temporal_brace_depth = None,
                Token::RBrace => *depth -= 1,
                _ => {}
            }
            continue;
        }
        if matches!(token, Token::KwTemporal) {
            temporal_pending_brace = true;
            continue;
        }
        if let Some(canonical) = canonical_token_spelling(&token) {
            replacements.push((span.start..span.end, canonical));
        }
    }

    let mut out = rewritten.to_owned();
    for (range, canonical) in replacements.into_iter().rev() {
        out.replace_range(range, canonical);
    }
    Ok(out)
}

/// Normalize notation in a syntactically valid Ken unit.
///
/// The stable legacy signature is retained for callers.  Invalid fragments
/// have no parsed token roles, so they are returned byte-for-byte rather than
/// being subjected to a raw-text fallback.
pub fn canonical_unicode(src: &str) -> String {
    match parse_lossless(src) {
        Ok(source) => canonicalize_tokens(source.as_ref()),
        Err(_) => src.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recovery_cursor_rounds_an_interior_byte_to_the_next_utf8_boundary() {
        let source = "x😀 ->";
        let next = recovery_cursor_after(source, 0, 2);
        assert_eq!(next, 1 + '😀'.len_utf8());
        assert!(source.is_char_boundary(next));
    }

    #[test]
    fn token_kind_table_is_exhaustive_for_current_notation_variants() {
        let cases = [
            (Token::Arrow, "→"),
            (Token::MapsTo, "↦"),
            (Token::Lambda, "λ"),
            (Token::PropEq, "≡"),
            (Token::Le, "≤"),
            (Token::Ge, "≥"),
            (Token::Ne, "≠"),
            (Token::And, "∧"),
            (Token::Or, "∨"),
            (Token::FlowsTo, "⊑"),
            (Token::Times, "×"),
            (Token::TruncBar, "‖"),
        ];
        for (token, spelling) in cases {
            assert_eq!(canonical_token_spelling(&token), Some(spelling));
        }
    }
}
