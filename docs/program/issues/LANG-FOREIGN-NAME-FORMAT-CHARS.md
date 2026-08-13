---
id: LANG-FOREIGN-NAME-FORMAT-CHARS
title: "Unicode Cf format characters -- bidi overrides, zero-width joiners, U+FEFF -- are a visual-spoofing vector at the same `foreign`-name trust boundary the Cc control-character check just closed, and they are a DIFFERENT vector: not truncation but two distinct declarations rendering identically to the reviewer doing the check"
status: draft
owner: language
size: XS
gate: none
depends_on: []
blocks: []
github: null
origin: Architect finding at evt_3aeg25e7b35mc while approving LANG-FOREIGN-NAME-CONTROL-CHARS (dec_79sd3nnqvkrvx), explicitly non-blocking and explicitly needing an owner. Filed by the Steward 2026-08-13 rather than left in prose, because the finding this whole arc came from was about an obligation landing in nobody's node.
---

## What this is

`LANG-FOREIGN-NAME-CONTROL-CHARS` (#2128) rejects `char::is_control()` in
`foreign` symbol and library names — exactly Unicode `Cc`, U+0000-U+001F and
U+007F-U+009F. That is the right set for what it claimed: NUL for truncation,
LF/CR for line-oriented injection, ESC for terminal sequences in diagnostics.

**It does not cover `Cf` (format), and should not have** — the frame scoped it
to control characters and the candidate is honest about its own boundary.
`Cf` is U+202E and the bidi-override set, U+200B, U+200D, U+FEFF.

## Why it is a different vector, not a wider version of the same one

`Cc` is about the **bytes** diverging from the declared name — truncation at a
NUL, where declared and effective differ silently.

`Cf` is about the **rendering** diverging from the bytes. A `foreign` name
carrying a bidi override displays in a reviewer's editor as something other
than what it is, and **two distinct `foreign` declarations can be made to
render identically.** This is the Trojan Source class.

**It matters more here than for most strings**, and that is the whole argument:
a reviewer reading

```
foreign f = "printf" from "libc"
```

is performing exactly the check that visual spoofing defeats. The mitigation
for a C-ABI name is human review of the declaration, so an attack on rendering
is an attack on the mitigation itself.

## What is measured, and the one thing that widens this later

**Ken identifiers are ASCII-only today.** `SURF-IDENT-TR39` (merged) measured
that the lexer's confusable-resistance is satisfied vacuously by an ASCII-only
identifier rule, with spec `31 §2`'s blessed Unicode letters unimplemented.
⇒ **The Trojan Source vector is not reachable through identifiers**, which is
why this node is scoped to string-literal-derived names rather than to the
source file.

**`SPEC-IDENT-BLESSED` is the event that changes that.** If blessed Unicode
letters land in identifiers, `Cf` becomes a source-wide concern and this node
is the wrong shape for it — at that point the question is a lexer-level or
formatter-level policy, not a `foreign`-name check. **Whoever takes that node
should read this one first.**

## The scope question, which is the reason this is `draft`

Three dispositions are available and they are not equivalent:

1. **Reject `Cf` at the same two parse sites**, mirroring the `Cc` check. XS,
   consistent, and defensible.
2. **Reject a narrower set** — the bidi overrides only (U+202A-U+202E,
   U+2066-U+2069) — on the ground that U+200D and U+FEFF have legitimate uses
   in text and none in a C symbol name. Arguably more correct and harder to
   state.
3. **Explicitly decline**, and record that `foreign` names may carry `Cf`, with
   the reason. **This is a real option**, not a failure: there is still no
   production consumer, and an unowned obligation is what this arc exists to
   prevent — a recorded decision not to cover it discharges that just as well
   as a check does.

**The Steward has not chosen among them and is not going to by default.** The
`Cc` node's own licensing says it *"discharges no validation the eventual
loader consumer owes"*; the same is true here, and picking option 1 because it
looks symmetric with the node next door is how a partial fix acquires the
appearance of a complete one.

## Not this node

- Widening the `Cc` check, revisiting `is_control()`, or touching `lexer.rs`.
  **The placement reasoning in `LANG-FOREIGN-NAME-CONTROL-CHARS` binds here
  unchanged:** `lexer.rs:229` decodes every string literal, and a check there
  would forbid these characters in ordinary string data.
- Identifier confusables or TR39 — that is `SURF-IDENT-TR39`'s lane and it is
  merged.
- Normalization of any kind, or defining a well-formed C symbol name.
- Any claim that this makes the C-ABI boundary safe.
