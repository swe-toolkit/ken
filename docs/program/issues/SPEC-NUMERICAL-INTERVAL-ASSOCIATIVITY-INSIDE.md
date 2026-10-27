---
id: SPEC-NUMERICAL-INTERVAL-ASSOCIATIVITY-INSIDE
title: "Spec 68 states interval add and mul commute by endpoint-value inclusion, but no law says either associates. Rule from a checked D0 at zero trust that both regroupings of interval_add and of interval_mul are mutually included, and state them with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Numerical is next in the layer rotation after Graphs. Same shape as SPEC-GRAPHS-REACHABILITY-ALONG-WALK. Steward-filed per COORDINATION section 2."
---

# Interval addition and multiplication associate by inclusion

## Objective

Spec 68 states, as public checked laws over
`Algorithm.Numeric.RationalInterval`, that regrouping three operands of
`interval_add`, and of `interval_mul`, gives mutually included results
by `interval_inside`, for every interval, including inverted ones.

## Settled inputs (spec 68 §1 to §5, main `454adf947`)

- **The operations.** `interval_add` and `interval_mul` act on
  corresponding endpoints through `rational_add` and `rational_mul`
  (§1.2).
- **No associativity law.** §1.3 to §5 state enclosure, point laws,
  hull and intersection containment, inclusion, distributivity and the
  operand swaps. `68-numerical.md:290` and the package prose at
  `RationalInterval.ken.md:967` say associativity is not claimed as
  structural `Equal Interval`, and no section states it by inclusion.
- **Why inclusion, not equality.** The package states arithmetic laws
  by endpoint-value inclusion, and a structural `Equal Interval` law
  would need a new `Data.Numeric.Rational` export, which the stop
  conditions exclude. Closed regroupings coincide structurally, since
  closed Nat arithmetic associates exactly; open ones are not
  convertible, which is why the laws are needed (amended per Architect
  AC-0 `evt_1yd2w28b5exdv`, Steward `evt_4b8jdb01qbht0`).
  `Data.Numeric.Rational` exports `proof assoc for rational_add` and
  `for rational_mul` as `rational_equiv` (`Rational.ken.md:260`,
  `:403`), and the private `equiv_leq` turns those into `rational_leq`.
- **Both directions.** Unlike §5's swaps, the reverse regrouping is not
  an instance of the forward law, so mutual inclusion needs both. The
  D0 rules whether that is two laws per operation or one law with the
  other derived in the seed.
- **Seeds.** Under `conformance/stdlib/numerical/`, next to
  `seed-interval-inside-order.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §6 in spec 68 stating each ruled law by its public type, and a closed
conformance seed under `conformance/stdlib/numerical/`, with the status
lines changed in the same candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged. It rules the
  names, the public types, and which helpers stay private.
- **AC-1.** The new section states each ruled law by its public type.
  The Spec leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies every ruled law to three
  closed intervals with distinct endpoints. It also checks by `Refl`
  that the closed regroupings coincide structurally, and says so, so no
  reader takes a closed instance as evidence of the generic law. Its
  refused block is pinned to one false inclusion on the same inputs:
  the regrouped sum inside the sum of the first two operands alone,
  with an accepted same-shape control and an independent `False`
  value.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, a new Rational
  export, or a premise.
- A change to an existing §1 to §5 law or operation.

Checks: 4 fired: `proof assoc for rational_add` and `for rational_mul`
exist on `454adf947` with `rational_equiv` results, and `equiv_leq`
exists as private in RationalInterval (spec 68 §4). 3 fired: grepped
spec/ and catalog/ for `assoc`; the only hits are the non-claims at
`68-numerical.md:290` and `RationalInterval.ken.md:967`.
