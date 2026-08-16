---
id: LANG-TRUNC-INTRO-DIAGNOSTIC-REMEDIES
title: "The `trunc_intro` infer-position diagnostic advises two remedies that both require an annotation-position `‖A‖` spelling the surface does not have, and omits `elim_trunc`'s motive, which is the one position that works"
status: draft
owner: language
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: Adversary hunt evt_1m95xs8h72pbm on the merged LANG-TRUNCATION-SURFACE-SYNTAX range 392f228b8..4b2d4cd9a, measured 2026-08-16. Filed draft and NOT released -- the second lane is quiet by operator instruction 2026-08-16. Also carries the Architect's two deferred follow-ups from evt_5f2dy109fjsct, which were routed to the Steward and had no node.
---

## The gap

`LANG-TRUNCATION-SURFACE-SYNTAX` shipped `trunc_intro` as checked-position-only
sugar. Its infer-position arm emits an actionable-looking diagnostic:

```
trunc_intro (‖A‖ introduction) cannot be inferred — it needs an expected type;
add an ascription `(trunc_intro a : ‖A‖)` or place it where the expected type
is already known (e.g. a declaration's declared type)
```

**Both advised remedies are unwritable in the surface that ships them.**
Measured by the Adversary on the merged range:

| advised remedy | measured |
|---|---|
| `(trunc_intro a : ‖A‖)` | parse error: expected a type, found `TruncBar` |
| a declaration's declared type | `fn … : ‖A‖` and `let y : ‖A‖` both parse error |

Both remedies need a **type-annotation-position** spelling of `‖A‖`. That is
exactly what the WP deliberately did not add — and its absence is the
load-bearing premise behind `elim_trunc`'s literal-`trunc_intro` special case.
**The diagnostic assumes the surface whose non-existence the premise asserts.**

**The route that does work is not named.** The candidate's own
`d2_trunc_intro_checks_against_a_trunc_shaped_goal` demonstrates it:

```
elim_trunc (‖ Nat ‖) (\x . trunc_intro Zero) (trunc_intro True)
```

`elim_trunc`'s motive is an **expression** position, so `‖A‖` is writable
there, and it supplies the expected type for `trunc_intro` in the method.

**Severity is user-facing and immediate.** Anyone writing `trunc_intro` outside
`elim_trunc` gets a diagnostic whose primary advice produces a second parse
error, and whose fallback advice produces the same one.

## The prior question, which decides the shape of the fix

The remedy text is downstream of a design call the Architect deferred at
`evt_5f2dy109fjsct` and routed to the Steward with no node: **is the
expression-position-only spelling of `‖A‖` the intended end state, or a
staging decision?**

- If **annotation position is to be added**, the diagnostic is already correct
  and the work is the grammar, not the string.
- If **expression-position-only is the end state**, the diagnostic is wrong on
  both clauses and must name `elim_trunc`'s motive instead.

**Do not fix the string before this is answered** — a rewritten remedy that
names `elim_trunc` becomes wrong again the moment annotation position lands.
The Architect's second deferred follow-up, `elim_trunc` reservation/shadow
policy, belongs to the same decision and is folded here rather than filed
separately.

## Deliverables

**`D0` — the spelling ruling.** Architect answers whether
expression-position-only is the end state or staging, and rules the
`elim_trunc` reservation/shadow policy alongside it. No code.

**`D1` — the diagnostic, made true under `D0`'s answer.** Either the
annotation-position production, or a remedy naming the working position. One
test asserting the emitted remedy is a program that parses.

## Acceptance criteria

**`AC-1`.** Every remedy the diagnostic names is exercised by a test that
compiles the suggested program to at least a successful parse. **The control is
the suggestion itself being run**, not a string comparison against expected
text — a string assertion is what let two unwritable remedies ship.

**`AC-2`.** If `D0` rules expression-position-only as the end state, the
premise sentence behind `elim_trunc`'s literal-`trunc_intro` special case is
restated in the code comment as a **ruled** invariant rather than a measured
one, so a later grammar change has a named thing to break.

**`AC-3`.** No `ken-kernel`, `spec/`, or `catalog/` path is touched.

## What is already established — do not re-measure

The Adversary attacked the load-bearing premise on five surface paths and
**it holds at the grammar level**: the type parser has no `TruncBar` production
at all, so this is a structural fact rather than five failed attempts. The
unannotated route is independently closed by `trunc_intro` refusing inference.

`canonical_token_spelling(TruncBar) = "‖"` matches the existing `∨`/`⊑`/`×`
policy and is deliberate, not a round-trip defect. The `lossless.rs` `ETrunc`
arm was probed with comments inside the delimiters in three positions and both
spellings — all parse, all attach, and `canonical_unicode` preserves them.
**No defect there; do not re-run it.**
