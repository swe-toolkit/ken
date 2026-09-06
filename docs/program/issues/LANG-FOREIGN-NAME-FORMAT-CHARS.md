---
id: LANG-FOREIGN-NAME-FORMAT-CHARS
title: "whole-source lexical policy for Unicode Cf format characters (bidi overrides, zero-width joiners, U+FEFF) -- a Trojan-Source lint over ALL Ken source (comments and string literals, and identifiers once blessed letters land), NOT a check at the two `foreign`-name sites, because a bidi override is expressible anywhere in source and the deception is of a human reading it"
status: draft
owner: language
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: Architect finding at evt_3aeg25e7b35mc while approving LANG-FOREIGN-NAME-CONTROL-CHARS (dec_79sd3nnqvkrvx), explicitly non-blocking and explicitly needing an owner. Filed by the Steward 2026-08-13 rather than left in prose, because the finding this whole arc came from was about an obligation landing in nobody's node.
---

> # RECUT 2026-09-06 — WHOLE-SOURCE LEXICAL POLICY (operator concurred).
> #
> # Operator ruling 2026-09-06, verbatim: "concur. reframe as whole-source lexical
> # policy." This answers the routed threat-model question ("whose reading is the
> # threat model?", below): the reader includes humans in terminals and web views,
> # so the Trojan-Source spoofing vector has a real victim, AND the honest shape of
> # closing it is a whole-source policy, NOT the two-`foreign`-name-site fix this
> # node's 2026-08-13 measurement ruled out (that protects two strings of the whole
> # surface, since a bidi override is expressible in any Ken comment or string
> # literal today). The node is recut: title + scope now the whole-source lint
> # (Rust's post-CVE-2021-42574 `text_direction_codepoint_in_literal` shape) over
> # all source; `gate: operator → none` (the operator answered); `size XS → M`. The
> # body below is retained as the analysis that grounds this shape.
> #
> # WHAT IS ROUTED, AND WHAT IS NOT YET BUILDABLE (like the node-3 floor fork). The
> # DESIGN is a spec/Architect lexer-level policy, not a parser patch, and it must
> # be settled before a build ring executes (§2c front-load): (1) the threat model
> # stated (which readers/renderers, reject vs. warn); (2) the exact `Cf` codepoint
> # set in scope (bidi override class U+202A-U+202E/U+2066-U+2069, U+200B/U+200D,
> # U+FEFF — and whether U+200D/U+FEFF, which have legitimate uses, are rejected or
> # only balanced/annotated); (3) where in the lexer the lint sits so it covers
> # comments (`skip_ws_comments`) and string literals (`lexer.rs:229`) without
> # forbidding these codepoints in ordinary string DATA that legitimately needs
> # them; (4) the interaction with `SPEC-IDENT-BLESSED` (once blessed Unicode
> # letters land in identifiers, this becomes source-wide including identifiers).
> # Routed to Spec enclave + Architect for that policy. IMPLEMENTATION (the lexer
> # lint) is then a language-ring node, queued behind the current match-pattern
> # work — it does NOT jump the lane-2 serial queue. `status: draft` until the
> # policy settles; the Steward frames the buildable node from the enclave's policy.

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

## The scope question — DISPOSITION 1 IS RULED OUT, and it is ruled out by measurement

**Updated 2026-08-13 after an Adversary pass (`evt_cxbze6z3yns8`) and a
Steward measurement of the lexer.** This node opened with three live
dispositions and no reason attached to any of them. **That state is itself a
defect** — an open node with no recorded reasoning reads to a later reader as
an implied obligation. One disposition is now closed on evidence.

**Ruled out: rejecting `Cf` at the two `foreign` parse sites.**

The `Cc` check earned its placement on a *mechanical* failure with a *named
future consumer*: a NUL makes the declared and effective names silently differ
at the ABI boundary, and the fix is producer-side hygiene against known C
string behaviour. **`Cf` has neither property.** A bidi override does not
truncate — `dlsym` resolves the name exactly, and declared and effective names
are identical. **The deception is of a human reading source, not of the
loader.**

Two consequences, and the second is decisive:

1. **It would widen a guard past its own stated rationale.** The doc comment,
   the error variant and its `Display` all justify the check by truncation. A
   `Cf` arm would sit under a reason that does not cover it — a check whose
   justification and population have come apart, leaving the next reader the
   wrong reason for the wrong half.
2. **Spoofing is not specific to `foreign` names, and the Steward's original
   scoping rationale for this node was incomplete.** It rested on
   `SURF-IDENT-TR39` establishing ASCII-only identifiers — true, and it does
   rule out the identifier route. **It says nothing about comments or ordinary
   string literals, which is where the rest of the surface is.** Measured at
   `c0a2ae77`: `lexer.rs`'s `skip_ws_comments` consumes every character up to
   `\n` with no filtering, and the lexer contains **zero** occurrences of
   `is_control`, `202E`, `FEFF` or `200B`. ⇒ **A bidi override is expressible
   in any Ken comment and any string literal today.**

⇒ Rejecting `Cf` in two `foreign` names would protect two strings out of the
whole language surface, **and it would have no placement argument at all.**
This arc spent its effort establishing *why* the `Cc` check belongs at the
parse sites rather than the lexer; for `Cf` that reasoning inverts — nothing
makes `foreign` names the spoofing-relevant position, they are merely where
the `Cc` finding happened to arrive. **Scoping `Cf` here would be a
smaller-surface version of the defect the `Cc` node avoided.**

### What remains: two dispositions, and a question that decides between them

- **A whole-source lexical policy with a stated threat model.** This is a
  spec/Architect decision, not a parser patch. The shape is Rust's
  post-CVE-2021-42574 `text_direction_codepoint_in_literal` — a lint over all
  source, not a check on two strings.
- **Close this node with the reason recorded**, namely that the truncation
  argument does not extend to spoofing, so the `Cc` node is **not** evidence
  for a `Cf` node.

> ### The deciding question, which the Steward cannot answer and is routing
>
> **Whose reading is the threat model?** Ken source is read by agents and
> rendered by tooling. **If the reader is an agent consuming bytes, a bidi
> override deceives nobody.** If it is a human in a terminal or a web view, it
> may.
>
> **That answer determines whether the whole-source disposition has a victim at
> all**, and it is a product question rather than a scope one. Raised by the
> Adversary, which explicitly declined to rule on it; the Steward agrees it is
> not its call. **Until it is answered, do not build either disposition** — and
> do not read this open node as an implied obligation to build one.

**`gate:` corrected `none` → `operator`, 2026-08-14.** The body has said since
2026-08-13 that neither disposition may be built until the operator answers, and
the frontmatter said the node was ungated. **That is the exact misreading this
node's own opening paragraph warns about** — an open node whose metadata implies
it is startable reads as available work, and `gate:` is the field a reader
filters on rather than the prose. The gate is not new; only its recording is.

## Not this node

- Widening the `Cc` check, revisiting `is_control()`, or touching `lexer.rs`.
  **The placement reasoning in `LANG-FOREIGN-NAME-CONTROL-CHARS` binds here
  unchanged:** `lexer.rs:229` decodes every string literal, and a check there
  would forbid these characters in ordinary string data.
- Identifier confusables or TR39 — that is `SURF-IDENT-TR39`'s lane and it is
  merged.
- Normalization of any kind, or defining a well-formed C symbol name.
- Any claim that this makes the C-ABI boundary safe.
