---
id: LANG-L1-ACCEPTANCE-ROWS
title: "Un-ignore two l1_acceptance rows as real assertions of settled behaviour: the explicit Int to Int64 conversion (delivered as intToInt64) and Char literals excluding surrogates. The Int division row stays ignored until the operator rules on registering div_int and mod_int"
status: active
owner: language
size: S
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Operator 2026-09-26 ('concur with rec.'): the three l1_acceptance ignored rows go to L2. Operator 2026-09-27 ('concur with l1_acceptance disposition'): sequenced right after LANG-SESSION-SCOPE, ahead of LANG-EXPRESSION-SIGMA. Operator 2026-10-01 ('concur with rec.'): next on L2 after LANG-MATCH-ARM-LEVEL-META-KERNEL-CHECK. Operator 2026-09-17 L1 directive (clear the ignored tests; top priority). Steward-filed per COORDINATION section 2."
---

# The three l1_acceptance rows

## Objective

Two of the three ignored rows in `crates/ken-interp/tests/l1_acceptance.rs`
run un-ignored, each asserting the behaviour its spec section settles. The
third, `sec31_int_div_zero_emits_obligation`, stays `#[ignore]` with its
reason updated to "needs operator-approved div_int/mod_int registration (18a
GAP)" (Architect `evt_6bg0x6s5w300n`).

## Settled inputs (AC-0 at `abc35cbcf`, `evt_6f5tzw0wxcf88`)

- **The rows assert nothing.** Rows 1 and 2 fail when run; row 3 passes
  vacuously:
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

The assertions the Architect ruled in `evt_6bg0x6s5w300n`. No product change
is expected; both behaviours are delivered at zero TCB.

- **Row 1.** `fn f (x : Int) : Option Int64 = intToInt64 x` elaborates.
  Evaluation gives `Some` at `2^63 - 1` and at `-2^63`, and `None` just
  outside each. The head is compared by the env's `Option` constructor
  identity and the payload as a value. The doc comment notes that spec 35 §5
  spells it `Int.toInt64`; no alias is added.
- **Row 3.** `'a'`, `'\u{D7FF}'` and `'\u{E000}'` elaborate. `'\u{D800}'` and
  `'\u{DFFF}'` fail with `ElabError::InvalidEscape`, matched structurally.
- **Exemption registry.** In `.github/ignored-test-exemptions.toml`, delete
  exactly the `ac5_explicit_conversion_is_partial_option` and
  `sec24_char_excludes_surrogates` `placeholder-no-assertions` blocks, which
  the CI ignored-test sweep rejects once those rows run (PR #4433). The
  `sec31_int_div_zero_emits_obligation` block stays.

## Acceptance

- **AC-0 (D0).** For each row, post:
  - the checked Ken text and its current result, verbatim;
  - the assertion it will carry;
  - whether it needs a new kernel primitive or a `trusted_base()` entry. The
    `div_int` row is the likely one.

  The Architect rules on each before any build.
- **AC-1.** Rows 1 and 3 run un-ignored and green with the assertions above.
  Row 2 stays ignored with the updated reason.
- **AC-2 (controls).**
  - The old row-1 text `Int.toInt64 x` reproduces the AC-0 `UnboundName`.
  - Flipping one boundary expectation (`Some` at `2^63`) reddens row 1.
  - Disabling the scalar screen in `lexer.rs` reddens row 3; restore it
    byte-identically.
- **AC-3.** Targeted suites only, through `scripts/ken-cargo`. Full CI is the
  breadth gate.

## Stop conditions

- Any kernel, `trusted_base()` or spec change, including registering a new
  kernel primitive: stop, as an operator question.
- A row whose behaviour `/spec` does not settle: stop to the Spec leader.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.
