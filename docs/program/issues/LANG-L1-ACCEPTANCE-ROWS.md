---
id: LANG-L1-ACCEPTANCE-ROWS
title: "Un-ignore the three l1_acceptance rows (explicit Int.toInt64 conversion, the Int division-by-zero obligation, and Char literals excluding surrogates) as real assertions of the behaviour /spec already settles"
status: active
owner: language
size: M
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Operator 2026-09-26 ('concur with rec.'): the three l1_acceptance ignored rows go to L2. Operator 2026-09-27 ('concur with l1_acceptance disposition'): sequenced right after LANG-SESSION-SCOPE, ahead of LANG-EXPRESSION-SIGMA. Operator 2026-10-01 ('concur with rec.'): next on L2 after LANG-MATCH-ARM-LEVEL-META-KERNEL-CHECK. Operator 2026-09-17 L1 directive (clear the ignored tests; top priority). Steward-filed per COORDINATION section 2."
---

# The three l1_acceptance rows

## Objective

The three ignored rows in `crates/ken-interp/tests/l1_acceptance.rs` run
un-ignored, each asserting the behaviour its spec section settles.

## Settled inputs (measured on `077be5457`)

- **The rows are stubs, not failing tests.** None asserts anything:
  - `ac5_explicit_conversion_is_partial_option` (`:288`) only elaborates
    `Int.toInt64 x`;
  - `sec31_int_div_zero_emits_obligation` (`:331`) only elaborates `a / b`;
  - `sec24_char_excludes_surrogates` (`:428`) has an empty body.

  Un-ignoring one as it stands proves nothing (Check 8).
- **The spec settles each behaviour:**
  - `35-numbers.md §5`: `Int.toInt64 : Int → Option Int64`, partial; there
    are no implicit coercions;
  - `18a-primitive-registry.md` `div_int`/`mod_int` row, marked GAP: `div x 0`
    yields an unsatisfiable `NonZeroDivisor` obligation and degrades to a
    panic or `Unknown`, never a value. Negative `mod` is pinned truncated;
  - `31-lexical.md`: a Unicode escape must be in U+0000–U+10FFFF, excluding
    U+D800–U+DFFF, and `35-numbers.md` gives `Char` as a Unicode scalar value
    (`'a'`).
- The ignore reasons cite "L-classes" and "not yet in scope for L1". Both
  predate the spec text above. Re-measure what builds today.

## Deliverable

For each row, an assertion that discriminates the settled behaviour, plus the
smallest elaborator or lowering change that makes it hold.

## Acceptance

- **AC-0 (D0).** For each row, post:
  - the checked Ken text and its current result, verbatim;
  - the assertion it will carry;
  - whether it needs a new kernel primitive or a `trusted_base()` entry. The
    `div_int` row is the likely one.

  The Architect rules on each before any build.
- **AC-1.** Each row runs un-ignored and green, with a discriminating pair:
  - `Int.toInt64` returns `Some` in range and `None` out of range;
  - `div` by a non-zero value computes and satisfies the div-mod identity, and
    by zero it yields the obligation and never a value;
  - a valid `Char` literal is accepted and a surrogate escape is rejected.
- **AC-2 (controls).** Removing each change re-reddens its row with the AC-0
  observation.
- **AC-3.** Targeted suites only, through `scripts/ken-cargo`. Full CI is the
  breadth gate.

## Stop conditions

- Any kernel, `trusted_base()` or spec change, including registering a new
  kernel primitive: stop, as an operator question.
- A row whose behaviour `/spec` does not settle: stop to the Spec leader.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.
