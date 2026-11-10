---
id: SPEC-NUMERICAL-INTERVAL-WIDTH
title: "Spec 68 defers interval width (68-numerical.md:245, :405), so a client using RationalInterval as an error bound cannot state how wide an enclosure is or how addition grows it, and Rational has no subtraction to define it with. Add a truncated rational subtraction, an interval width, and checked laws at zero trust: a point has width zero, and the width of an ordered sum equals the sum of the widths by value, with a conformance seed"
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

# Interval width

## Objective

`Algorithm.Numeric.RationalInterval` exports `interval_width`, and
`Data.Numeric.Rational` exports the truncated subtraction it is built
from. Two public checked laws hold, both stated by value
(`rational_equiv`). **Point:** the width of a point interval is zero.
**Sum:** for two ordered intervals, the width of their sum equals the
sum of their widths. Spec 68 states the operations and laws by their
public types.

## Settled inputs (main `8815527de`)

- **The carrier.** `Interval` (`RationalInterval.ken.md:76`) has one
  constructor over two `Rational` endpoints. `interval_add` (`:268`)
  adds endpoints pointwise, and `interval_point` (`:284`) repeats one
  endpoint.
- **No subtraction.** `Data.Numeric.Rational` (`Rational`,
  `MkRational num den_tail` at `:42`) exports `rational_add`,
  `rational_mul`, `rational_leq` (`:162`), `rational_min`, `rational_max`
  and `rational_equiv` (`:64`, a `Prop`), but no subtraction of any kind.
- **The Nat piece.** Truncated Nat subtraction is public as `sub`
  (`Data/Numeric/Nat/Order.ken.md:322`).
- **The deferral.** Spec 68 lists "width and diameter" as outside
  §§1–3 (`68-numerical.md:245`) and still deferred (`:405`). The spec
  ends at §14 (`:1328`), so the new section is §15.
- **Order and ordered intervals.** An interval is ordered when
  `rational_leq lo hi` is True. `interval_inverted_empty` and
  `interval_ordered_contains_lo`/`_hi` already use that premise form.
- **Seeds and tests.** Seeds go under `conformance/stdlib/numerical/`;
  tests are named `cat_numeric_interval_<slug>.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

One candidate:
- the truncated rational subtraction, with the laws the width proof
  needs, in `Data.Numeric.Rational`;
- `interval_width` and the two laws, with names and placement as AC-0
  rules them;
- a §15 in spec 68, and the deferral lines at `:245` and `:405`
  narrowed to what is still deferred;
- a closed conformance seed;
- a package test;
- the status lines.

## Acceptance

- **AC-0 (D0, Architect specifies, Foundation implementer runs on
  scratch).** A checked development on the landed base, with rc 0, no
  `Axiom`, and `trusted_base()` unchanged as an identity set. It rules:
  - the subtraction's definition and name, and which of its laws are
    public;
  - whether the sum law needs both intervals ordered, or only states
    them as premises;
  - that no §1 to §14 operation or law type changes.
- **AC-1.** §15 states the subtraction, `interval_width` and both laws
  by their public types. The Spec Leader and CV review it.
- **AC-2.** The seed's accepted client computes a closed width of a
  sum of two ordered intervals and checks it through the sum law, and
  a point's width through the point law. Its refused block pins, in
  the same form as an accepted control, the sum law claimed for an
  inverted operand, or a wrong width value, as AC-0 rules. A kernel
  that accepts it would show the law was not checked.
- **AC-3.** Every RationalInterval and Rational package test that
  checked before still checks. The new package test fails if
  `interval_width` is redefined to ignore the lower endpoint.

## Stop conditions

- A law needs an Axiom, or a kernel or surface change.
- A change to an existing §1 to §14 operation or law type.
- Signed values become necessary: that is a separate, larger node.

Checks: 4 fired: `Interval`, `interval_add`, `interval_point`,
`rational_leq`, `rational_equiv` and Nat `sub` were read at the cited
lines on `8815527de`. 3 fired: `sub`, `monus`, `minus` and `diff` were
grepped over `Data/Numeric`; the only hit is Nat's `sub`, so Rational has
no subtraction to reuse. `width` occurs in spec 68 only at the two
deferral lines.
