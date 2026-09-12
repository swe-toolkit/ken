# Source to checked core

> **Availability:** partial. **Authority:** explanatory.

The front end turns Ken source into declarations that the kernel can admit. Its
[crate entry module](../../../crates/ken-elaborator/src/lib.rs) states the
implemented sequence: lexing, parsing, resolution, elaboration, and kernel
checking. These stages have different jobs. Parsing establishes source
structure, resolution relates names to declarations, and elaboration produces
explicit core terms before the kernel checks them.

The front end does not make source text trusted. `ElabEnv` owns the
surface-level environment and calls the kernel checker when it discharges a
certificate. The kernel remains the authority for type checking and proof
validity; the elaborator prepares terms and metadata for that authority.

## Input and dispatch

The [CLI dispatcher](../../../crates/ken-cli/src/main.rs) accepts a source path
for `ken check` and `ken run`, creates an elaboration environment, and selects
an input route. A `.ken.md` path uses the literate-source route; another path
uses the ordinary source route. Catalog-addressed paths instead enter the roots
loader, which loads the addressed module and runs its checked fences.

Reading a path does not admit its text. The dispatcher's shared elaboration
helper returns only after its selected route has elaborated the input. An
unreadable path, initialization failure, or elaboration error reports an error
rather than continuing to execution. `ken check` stops there. `ken run` first
uses the same input route, then asks the interpreter to evaluate an entry point;
the route itself neither establishes an entry point nor execution.

## Lexing and parsing

The [lexer](../../../crates/ken-elaborator/src/lexer.rs) reads source text into
tokens paired with source spans. `Lexer::lex` continues through the end-of-file
token, giving the parser a complete ordered stream rather than raw characters.
It recognizes punctuation, keywords, identifiers, literals, and operator runs,
while skipping whitespace and supported comments. An unterminated block comment
or invalid literal escape is refused here with its source span.

Token recognition is not syntax construction. The
[parser](../../../crates/ken-elaborator/src/parser.rs) turns the token stream
into surface abstract syntax. `parse_decls` repeatedly parses declaration
groups through end of file and refuses an anonymous program or package boundary
after a declaration. The public parser entry point runs lexing first, so a
lexical failure prevents syntax construction.

Parsing records declarations, expressions, types, patterns, and spans, but it
does not bind a name or decide whether a term has a type. A malformed token
sequence is refused as a parser error instead of proceeding as partially
understood syntax. The [lexical](../../../spec/30-surface/31-lexical.md) and
[grammar](../../../spec/30-surface/32-grammar.md) specifications are normative;
this page identifies the implementation route that applies them.

## Resolution

The [resolver](../../../crates/ken-elaborator/src/resolve.rs) translates surface
declarations and expressions into resolved forms. `resolve_decls` processes a
declaration sequence with shared unit definitions, producing resolved
rather than source-only declarations. Its scope records local bindings and
assigns de Bruijn-style positions to locally bound variables.

A local scope miss becomes `RCon`, retaining the name for elaboration's later
global lookup; it is not a general resolver-stage refusal. `resolve_expr_ctx`
does have a narrow refusal for `result` outside its permitted contract context.
The elaborator is the later consumer that consults its global environment and
can refuse an unknown global name before it constructs core terms. The
[declarations specification](../../../spec/30-surface/33-declarations.md) is
the normative source for names and modules.

## Elaboration and admission

The elaboration environment parses a file, expands its module structure, and
elaborates resulting declarations in order. Later declarations can consequently
refer to declarations already elaborated in that file. The
[elaboration implementation](../../../crates/ken-elaborator/src/elab.rs) turns
resolved forms into kernel-core terms.

Elaboration prepares a term for checking; it is not the authority that makes a
source claim valid. The environment's certificate-discharge path calls the
kernel checker, and a failed check leaves the certificate undischarged. The
kernel remains the authority for type checking and proof validity. The
[elaboration specification](../../../spec/30-surface/39-elaboration.md) defines
the language contract; this page explains the implementation route applying it.

## What follows admission

The [compiler driver](../../../crates/ken-elaborator/src/compiler_driver.rs) can
collect admitted declarations into a `CheckedCorePackage`. That package carries
stable symbols and semantic metadata for later consumers, rather than passing
raw source text to erasure. It is not evidence that native lowering or an
executable artifact occurred. Read [Artifacts and erasure](artifacts-and-erasure.md)
for the boundary it creates, or [The trusted kernel](kernel.md) for the checker
that admits core terms.
