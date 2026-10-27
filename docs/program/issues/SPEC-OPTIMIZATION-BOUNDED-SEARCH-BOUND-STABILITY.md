---
id: SPEC-OPTIMIZATION-BOUNDED-SEARCH-BOUND-STABILITY
title: "Spec 72 section 2 states that find_below is sound, least and complete at one bound, but no law relates two bounds. Rule from a checked D0 at zero trust that a witness found below a bound is the one found below every larger bound, and state it with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Optimization is next in the layer rotation after Symbolic algebra. Same shape as SPEC-SYMBOLIC-ALGEBRA-DERIVATIVE-COMMUTATION. Steward-filed per COORDINATION section 2."
---

# A bounded-search witness is stable under a larger bound

## Objective

Spec 72 states, as a public checked law over
`Algorithm.Optimization.BoundedSearch`, that when `find_below p bound`
returns `Some n`, every larger bound returns the same `Some n`.

## Settled inputs (spec 72 §2, main `769b3847e`)

- **The operation.** `find_below (p : Nat → Bool) (bound : Nat) :
  Option Nat` returns the least `n < bound` with `p n = True`, else
  `None` (§2.1). The landed body recurses on the bound through
  `find_next` (`BoundedSearch.ken.md:34-37`).
- **No two-bound law.** §2.2 states `find_below_sound`,
  `find_below_least` and `find_below_complete`, each at one bound.
- **Why it holds unconditionally.** The witness is below `bound`, so it
  is below any larger bound. Every `m < n` misses, so no earlier hit
  appears when the range grows. The `None` direction is false, because
  a larger range can contain a hit: with `p n` meaning `2 < n`, bound
  `3` gives `None` and bound `10` gives `Some 3`.
- **Seeds.** Under `conformance/stdlib/optimization/`, next to
  `seed-bounded-search.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §2 subsection in spec 72 stating each ruled law by its public type,
and a closed conformance seed under `conformance/stdlib/optimization/`,
with the status lines changed in the same candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged. It rules the
  name, the public type (a larger bound written as `add bound extra`
  or as a `leq_nat` premise), and which helpers stay private.
- **AC-1.** The new subsection states each ruled law by its public
  type. The Spec leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the law to `p n` meaning
  `2 < n`, from bound `5` to bound `10`. Its refused block is pinned to
  one false proposition: the `None` direction, from bound `3` to bound
  `10`.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or a premise
  beyond the larger bound.
- A change to an existing §1 to §3 law or operation.
