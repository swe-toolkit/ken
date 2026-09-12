# Parsing

> **Availability:** partial. **Authority:** explanatory.

The [parser](../../../../crates/ken-elaborator/src/parser.rs) turns the lexer’s
token stream into surface abstract syntax. `parse_decls` repeatedly parses a
declaration group until end of file, and rejects an anonymous program or package
boundary that appears after another declaration. The public parser entry point
runs lexing first, so a lexical error prevents syntax construction.

The parser records source structure, not meaning. It builds declarations,
expressions, types, patterns, and their spans, but it does not assign a binding
to a name or decide whether a constructed term has a type. A malformed sequence
is refused with a parser error rather than passed as partially understood
syntax.

The surface grammar is specified in the [grammar
chapter](../../../../spec/30-surface/32-grammar.md). This page is not an
alternative grammar: it points from that contract to the implementation that
constructs the surface tree. The next stage, [resolution](resolution.md), gives
that tree its binding structure.
