---
id: SPEC-NUMERICAL-INTERVAL-WIDTH-MUL
title: "Spec 68 §15 says how width propagates through interval addition but not through multiplication, so a client that multiplies two enclosures cannot bound the error of the product. Rule from a checked D0 at zero trust that, for ordered operands, the width of the product is equivalent to hi(i)·width(j) + lo(j)·width(i), with any Rational subtraction-distribution law it needs, and state it with a conformance seed"
status: merged
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Numerical follows Graphs in the L3 layer rotation (docs/program/06-catalog-campaign.md, whose Numerical entry is error-bound refinements). Steward-filed per COORDINATION section 2."
---

# Width propagates through interval multiplication

## Objective

`Algorithm.Numeric.RationalInterval` exports a public checked law
that, when both operands have ordered endpoints, gives the width of
`interval_mul i j` as a `rational_equiv` to an expression in the
operands' endpoints and widths. It holds at zero trust, and spec 68
states it by its public type.

## Settled inputs (main `a931dcb40`)

- **The gap.** §15 (`spec/50-stdlib/68-numerical.md:1401`) gives
  `interval_width_point` and `interval_width_add`
  (`RationalInterval.ken.md:267`, `:272`). §16 (`:1497`) relates width
  to inclusion. No law gives the width of a product. Spec 68 ends at
  §16, so the new section is §17.
- **The operations.** `interval_mul` multiplies corresponding
  endpoints (`RationalInterval.ken.md:296`; spec §1.2 `:126`).
  `interval_width i` is `rational_sub hi lo` (`:306`), which truncates
  at zero.
- **The arithmetic.** For `i = [a,b]` and `j = [c,d]` with `a ≤ b`
  and `c ≤ d`, the width of the product is `b·d ∸ a·c`. It equals
  `b·(d ∸ c) + c·(b ∸ a)`, and also `d·(b ∸ a) + a·(d ∸ c)`. AC-0
  picks one form.
- **The bridges.** `rational_mul` has `distrib_l` over `rational_add`
  (`Rational.ken.md:508`). `rational_sub` has `add_distrib` (`:616`),
  and `rational_sub_mono` exists (`:665`). Nat has
  `mul_sub_distrib_r` and `_l` (`Nat/Order.ken.md:451`, `:475`). No
  `rational_mul` law distributes over `rational_sub`.
- **Seeds and tests.** The seed goes beside
  `conformance/stdlib/numerical/seed-interval-width.md`. The test
  pattern is `crates/ken-elaborator/tests/cat_numeric_interval_width.rs`.
- **Catalog style.** The catalog half follows
  `docs/program/07-catalog-style-guide.md` and
  `agent/playbooks/tools/write-ken.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 68 §17, the catalog law with any Rational provider law it needs,
a seed, a package test and the status lines. No §1 to §16 operation
or law type moves, and there is no kernel or surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-NUMERICAL-INTERVAL-WIDTH-INSIDE-MONO.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules:
  - the law's name, binder order and which of the two forms it
    states, following `interval_width_add`;
  - its placement in the export order;
  - whether `rational_mul` distribution over `rational_sub` is a
    public Rational law or a private stage, and its owner;
  - whether the ordered-endpoint premises are both necessary.
- **AC-1.** §17 states the ruled law by its public type. The Spec
  Leader and the CV review it, with a Decision.
- **AC-2.** The seed works on closed intervals.
  - **Accepted block.** For `[1,2]·[1,2] = [1,4]`, it applies the law
    and concludes width 3, as `2·1 + 1·1` in the first form.
  - **Refused block 1.** It is on a fresh accepted prefix and in the
    same form as the accepted row. It claims the product's width is
    the product of the widths, which gives 1, not 3.
  - **Refused block 2.** It drops the premises. For the inverted
    `[3,1]·[1,2] = [3,2]`, the width is 0, while the first form gives
    `1·1 + 1·0 = 1`.
- **AC-3.** Readability. The catalog proof and any provider law read
  as the style guide requires: local names state a proof stage or
  endpoint, and the candidate passes the independent local-naming
  and formatter-layout review.
- **AC-4.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise
  beyond the two ordered-endpoint hypotheses.
- A change to an existing §1 to §16 operation or law type.

Checks:
- **4 fired.** The obligation is written out. With `a ≤ b` and
  `c ≤ d`, monotonicity gives `a·c ≤ b·d`, so the truncation is exact.
  Cross-multiplying the Rational endpoints turns the equivalence into
  a Nat identity. `mul_sub_distrib_l` and `_r` with `add_distrib`
  for `rational_sub` close it. No public Rational law distributes
  `rational_mul` over `rational_sub`, so AC-0 rules that stage.
- **3 fired.** No `width_mul` occurs in `catalog/`, `spec/`,
  `conformance/` or the elaborator tests.
- **8 fired.** A law using the product of the widths accepts refused
  block 1. A law reusing the add-law shape `w(i) + w(j)` gives 2 for
  the `[1,4]` row, so it refuses the accepted block. A law without
  premises accepts refused block 2. Only the ordered law accepts the
  `[1,4]` row and refuses both blocks.
