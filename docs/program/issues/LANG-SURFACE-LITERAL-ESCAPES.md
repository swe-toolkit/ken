---
id: LANG-SURFACE-LITERAL-ESCAPES
title: "The lexer performs NO escape processing -- its single string form pushes every character verbatim, so the escape repertoire that SPEC-LITERAL-ESCAPE-PIN just closed is entirely unimplemented, and Char literals, byte literals, byte strings and raw triple strings do not exist at all despite String, Char and Bytes all being built prelude targets"
status: merged
owner: language
size: M
gate: none
depends_on: []
blocks: []
github: 2119
origin: Unblocked by SPEC-LITERAL-ESCAPE-PIN merging at PR #1947 (spec fbdc0268, conformance seed conformance/surface/literals/seed-escapes.md). Research surface-gap sweep evt_6qeeebh5m3fba ranked character, byte and string-escape literals as three M-shaped closures blocked on that pin; Steward measured the lexer at origin/main 762a9b44 and cut them as ONE node -- see "Why this is one node and not three". Steward-filed (agents cannot create tracked work per COORDINATION 2).
---

## The gap, measured at `762a9b44`

**The lexer has exactly one string form and it processes no escapes.** The scan
loop in `crates/ken-elaborator/src/lexer.rs` (around `:214-230`) is:

```rust
Some('"')  => { self.advance(); break; }        // closing quote
Some(c)    => { self.advance(); s.push(c); }    // EVERYTHING else, verbatim
None | Some('\n') => { /* unterminated string literal, with a span */ }
```

⇒ **A backslash is just a character.** `"\n"` is a two-character string
containing a backslash and an `n`. Nothing rejects, nothing decodes, and there
is no `InvalidEscape` anywhere in the file.

| form | spec | state at `762a9b44` |
|---|---|---|
| `"…"` with escapes | pinned | **string exists, escapes absent** |
| `'c'` Char literal | pinned | absent |
| `b"…"` byte string | pinned | absent |
| `\xHH` byte escape | pinned | absent |
| raw triple string | pinned | absent |

**The semantic targets are built.** `String`, `Char` and `Bytes` all resolve in
the prelude (`prelude.rs:1292`, `:1302`, `:1502`). This is scanner and token
work, not type-system work.

**`Token::Str` is currently load-bearing elsewhere.** It carries the symbol and
library names in `foreign` declarations (`lexer.rs:88`). Turning on escape
processing changes what a `foreign` name containing a backslash means — that is
a real compatibility question and the frame makes it an acceptance criterion
rather than a discovery.

## Why this is one node and not three

The research sweep ranked **character**, **byte** and **string-escape** literals
as three separate M-shaped closures, and the spec presents them as three topics.
**That is a taxonomy of the spec section, not of the work.**

All three are **one scanner**, **one `InvalidEscape` error**, and **one span
rule**. `SPEC-LITERAL-ESCAPE-PIN` deliberately closed the repertoire as a single
kind-selected table precisely so the kinds share a decision procedure. Cutting
three nodes would put three rings through the same function, review the same
scanner three times, and create three-way contention on one file — while the
pin's central property, that the repertoire is **closed and selected by kind**,
is only testable across the kinds *together*.

⇒ Subsume, do not proliferate (`docs/PRINCIPLES.md`). The three kinds are
deliverables inside one node.

## What the pin settles, so none of it is re-litigated here

Landed at PR #1947; the normative text is `spec/30-surface/31-lexical.md` and
the six discriminating rows are
`conformance/surface/literals/seed-escapes.md`.

- The repertoire is **closed and selected by literal kind** — every unlisted
  sequence rejects **by construction**, not by enumeration.
- Unicode escapes are **one to six** ASCII hex digits and must denote a
  **non-surrogate** scalar.
- `\xHH` is **byte-only and fixed-width at exactly two hex digits**. `b"\x41BC"`
  is bytes `0x41 0x42 0x43` — **no greedy lookahead.**
- Unescaped byte-string content is **ASCII-only**. Raw triple strings perform
  **no** escape processing.
- Unrecognized, wrong-kind, malformed, or invalid-scalar escapes raise
  **`InvalidEscape`** and emit **no literal token**.
- The span **begins at the backslash** and ends immediately after the last
  offending character, **excluding** an interrupting delimiter, line boundary or
  EOF.
- **Lane-owned precedence:** once the backslash commits the lexer to an escape
  production, a literal ending before that production completes raises
  `InvalidEscape`, **not** the existing unterminated-literal error. Every other
  unterminated-literal behaviour is unchanged and deliberately unnamed.

**The precedence clause is the one that was nearly got wrong, and the conformance
seed records why.** The pin's first candidate was green under an implementation
that kept every completed and malformed Char escape correct while routing a Char
ending mid-escape to the unterminated path. It took a Char/line-boundary fixture
to exclude that. **Row 6 of the seed is that control — do not treat it as
redundant with the String and byte-string legs.**

## Frame

`docs/program/wp/LANG-SURFACE-LITERAL-ESCAPES.md` — deliverables, the
conformance-seed binding, acceptance criteria with controls, excluded scope,
stop conditions and contention.

## Not this node

The formatter's rendering of the new literal forms beyond round-trip
preservation, numeric literal work, and `0x[…]` carrier ownership — all
explicitly disclaimed by the pin. Block and doc comments are
[[LANG-SURFACE-BLOCK-COMMENTS]].
