---
id: SPEC-RESERVED-INFIX-NAMES
title: "bounded spec amendment so the six reserved glyph tokens (≤/<=, ≥/>=, ≠//=, ∧//\\, ∨/\\/, ∈) may be ordinary symbolic GLOBAL names and infix/fixity targets: amend 31/32's operator-name and fixity grammar to admit them, and correct 31's `∈` 'no operator semantics' sentence to permit a client-defined ordinary function while keeping any STANDARD membership binding deferred; NO `!=`/`in` aliases and NO standard Membership class in this node; plus a reaching seed"
status: ready
owner: spec
size: S
gate: none
depends_on: []
blocks: [LANG-RESERVED-INFIX-NAMES]
github: null
tier: T1
origin: "Steward cut 2026-09-13 from the Architect final-A decomposition (evt_784ge2nq65dfy, grounding corrections evt_7z907rns84e6n), grounded at main 4fdd4f0ad; Architect probe SHA e0434d58. Operator objective (Pat, 2026-09-12, this session): full reserved infix glyphs. The Architect ruled A0 (LANG-RESERVED-INFIX-NAMES) needs a bounded Spec amendment to 31/32's operator-name/fixity grammar and the `∈` status FIRST, with a seed. IN-LANE SCOPE CALL (Steward, on the operator objective + the standing ruling that language/library surface of this kind is fleet-owned not operator-gated): bounded normative surface, no new language/kernel mechanism, no new trust-root, no TCB. Analogous to SPEC-PRIORITY-QUEUE-CONTRACT -> CAT-PRIORITY-QUEUE. Re-measure spec anchors at the cut."
---

> # SPEC PREREQUISITE for A0 [[LANG-RESERVED-INFIX-NAMES]] (Architect ruling).
>
> A0 admits the six reserved glyph tokens as ordinary symbolic global names +
> infix/fixity targets. The grammar and the `∈` status must say so first. This
> node states what the amendment settles and the reaching seed; the enclave
> authors the normative text. Grounded at main `4fdd4f0ad`. Re-measure at the cut.

## What this is

Ken lexes `≤ ≥ ≠ ∧ ∨ ∈` (and the ASCII twins `<= >= /= /\ \/`) to dedicated
tokens `Le/Ge/Ne/And/Or/Member` that the parser never consumes as operator
names — they tokenize and dead-end (used only in the formatter). Before A0 can
route them into the ordinary symbolic-name and infix/fixity machinery, 31/32's
operator-name and fixity-target grammar must admit them, and 31's `∈` status
sentence must be corrected. This is the spec half; the elaborator wiring is A0.

## What the amendment must settle

1. **The six tokens are admissible operator-name spellings.** Amend
   `spec/30-surface/31-lexical.md` and `spec/30-surface/32-grammar.md` so that
   each of the six glyph tokens and its ASCII twin may serve as an ordinary
   symbolic GLOBAL name and as an infix/fixity target, entering the same
   neutral-spine + fixity-reassociation path as any user symbolic operator.
   The admitted alias pairs, EXACTLY:
   - `≤` / `<=`  (`Le`)
   - `≥` / `>=`  (`Ge`)
   - `≠` / `/=`  (`Ne`)
   - `∧` / `/\`  (`And`)
   - `∨` / `\/`  (`Or`)
   - `∈`         (`Member`; glyph-only, no ASCII twin)

2. **No new aliases.** `!=` is NOT admitted (it currently fails lexing at `!`;
   adding it is a separate additive-alias choice, not this node). `in` is NOT
   an alias for `∈` (it stays the `let … in` keyword). `/=` keeps its existing
   token/formatter identity and gains the newly specified name/application use;
   it had no prior accepted ordinary infix consumer to preserve.

3. **Correct the `∈` status sentence.** `31-lexical.md:135-137` currently states
   `∈` has no operator semantics and none is planned. Amend it to distinguish:
   `∈` may name a client-defined ORDINARY function (so the token is admissible as
   a name), while any STANDARD/library membership binding remains DEFERRED (its
   class/carrier design is a separate track, not settled here). `∈` stays
   glyph-only with no ASCII form (the existing §1c glyph-only exception holds).

4. **No standard semantics in this node.** This amendment admits NAMES and
   fixity eligibility only. It does NOT specify what `≤ ≥ ≠ ∧ ∨` mean as
   standard operators (that is the A1 contract, [[SPEC-STANDARD-INFIX-BINDING]]),
   does NOT introduce a Membership class or default membership meaning, and does
   NOT assign precedence from glyph spelling (an undeclared binding defaults to
   `infixl 9` like any symbolic operator).

## Deliverables

1. The bounded 31/32 amendment settling items 1-4, placed without a broad
   grammar rewrite or unrelated cleanup.
2. A reaching conformance seed: a discriminating implementation admits each of
   the six names in a declaration, a prefix application, an infix application,
   and a fixity declaration; a non-conforming one (a token that still dead-ends,
   or an admitted `!=`/`in` alias) reds. The seed states expected observations
   without computing them from the implementation under test.

## Acceptance criteria

- The amendment admits exactly the six alias pairs above as symbolic global
  names + infix/fixity targets; no `!=`, no `in` alias, no precedence-from-glyph.
- The `∈` status sentence distinguishes a client-defined ordinary function
  (admitted) from the deferred standard binding; `∈` stays glyph-only.
- No standard operator meaning, Membership class, or default binding is
  introduced; existing arithmetic/`==`, `< > / %`, `let … in`, lambda/arrow,
  truncation bars, annotation and reserved punctuation are unchanged in the spec.
- A reaching seed exists whose expected values are independent of the
  implementation under test.

## Not this node

- The elaborator/parser wiring — [[LANG-RESERVED-INFIX-NAMES]] (A0, draft).
- The standard operator meanings + completion contract —
  [[SPEC-STANDARD-INFIX-BINDING]] (A1's spec prerequisite, draft).
- Membership's class/carrier design — the parallel B track.

## Sizing / tier

**Size S, tier T1.** A short grammar/status amendment, but T1: it fixes which
tokens are admissible operator names and the `∈` status distinction that A0 and
A1 depend on. Spec-enclave-owned; the Architect is the design authority and a
reviewer.

## Contention

Spec enclave, `spec/30-surface/`. No cross-lane contention (L1 runtime on
`crates/`; L3 foundation on `catalog/`; this is `spec/` only). Re-measure any
cited `spec/` anchor at the cut.
