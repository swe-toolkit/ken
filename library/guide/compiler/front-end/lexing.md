# Lexing

> **Availability:** partial. **Authority:** explanatory.

The [lexer](../../../../crates/ken-elaborator/src/lexer.rs) reads source text
into tokens paired with source spans. Its public `Lexer::lex` operation continues
until it emits the end-of-file token, so the parser receives a complete ordered
token stream rather than raw characters.

Token recognition is not syntax construction. The lexer identifies punctuation,
keywords, identifiers, literals, and operator runs, while skipping whitespace
and supported comment forms. It rejects malformed lexical forms at this stage:
for example, an unterminated block comment or an invalid literal escape returns
an elaborator error with the span that exposed it.

The lexer does not decide whether a token sequence is a declaration or whether a
name denotes a declaration. Those questions belong to [parsing](parsing.md) and
[resolution](resolution.md). The accepted spellings and lexical rules are
normative only in the
[lexical specification](../../../../spec/30-surface/31-lexical.md); this page
identifies the implementation that applies them.
