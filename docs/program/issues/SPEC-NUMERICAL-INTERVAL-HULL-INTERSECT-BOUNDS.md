---
id: SPEC-NUMERICAL-INTERVAL-HULL-INTERSECT-BOUNDS
title: "Spec 68 states that hull contains and intersection lies inside each operand, but no law says either is the tightest such interval. Rule from a checked D0 at zero trust that hull is the least upper bound and intersection the greatest lower bound under interval_inside, and state them with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Numerical is next in the layer rotation after Graphs. Same shape as SPEC-NUMERICAL-INTERVAL-ASSOCIATIVITY-INSIDE. Steward-filed per COORDINATION section 2."
---

# Hull and intersection are the tightest bounds under inclusion

## Objective

Spec 68 states, as public checked laws over
`Algorithm.Numeric.RationalInterval`, that for every interval `k`,
including inverted ones:

- if `i` and `j` are both inside `k`, then `interval_hull i j` is
  inside `k`;
- if `k` is inside both `i` and `j`, then `k` is inside
  `interval_intersect i j`.

## Settled inputs (spec 68 §1 to §6, main `64e6bf67e`)

- **The operations.** `interval_hull` takes `rational_min` of the lower
  endpoints and `rational_max` of the upper ones, and `interval_intersect`
  takes the reverse (`RationalInterval.ken.md:320`, `:328`).
  `interval_inside i k` compares endpoint values:
  `rational_leq c a` and `rational_leq b d` (`:338`).
- **Only the bound half is stated.** §2.2 and §3.2 give the four
  one-sided laws: `interval_inside_hull_left`/`_right` and
  `interval_intersect_inside_left`/`_right` (`:509` to `:567`). §5 gives
  the operand swaps. No section states that hull is the least interval
  containing both operands, or that intersection is the greatest
  interval inside both.
- **Why they should hold.** `Data.Numeric.Rational` exports
  `proof greatest for rational_min` and `proof least for rational_max`.
  `interval_hull_comm_inside` (`:790`) already uses both. For `k` with
  endpoints `e` and `f`, the hull law needs `e ≤ min a c` and
  `max b d ≤ f` from the four endpoint premises, and the intersection
  law needs the mirrored pair. The D0 measures whether a new private
  helper is needed.
- **The premises are necessary.** Without them both claims are false.
  `interval_hull [0,1] [2,3]` is `[0,3]`, which is not inside `[0,1]`.
  `[0,3]` is not inside `interval_intersect [0,3] [1,2]`, which is
  `[1,2]`.
- **Seeds.** Under `conformance/stdlib/numerical/`, next to
  `seed-interval-hull-intersection.md`.
- **Status.** Spec 68's SPEC-PROGRESS row still says the §6 exports are
  absent. They landed with `CAT-NUMERIC-INTERVAL-ASSOCIATIVITY-INSIDE`.
  The status change below corrects that sentence in the same row.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §7 in spec 68 stating each ruled law by its public type, and a closed
conformance seed under `conformance/stdlib/numerical/`. The same candidate
changes the status lines, including spec 68's SPEC-PROGRESS row.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged. It rules:
  - the names;
  - the public types and argument order, including how the two
    premises are stated;
  - which helpers stay private.
- **AC-1.** The new section states each ruled law by its public type.
  The Spec leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies both laws to closed
  intervals with distinct endpoints, with `k` strictly larger than the
  hull or strictly smaller than the intersection. It also computes each
  premise and conclusion independently by `Proved`. Its refused block is
  pinned to one false proposition of the right type: the hull of two
  operands inside one of them, when the other operand is not inside it.
  It also carries an accepted same-shape control and an independent
  `False` value.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, a new Rational
  export, or a premise beyond the two inclusions.
- A change to an existing §1 to §6 law or operation.

Checks: 4 fired: `proof greatest for rational_min` and `proof least for
rational_max` are applied with exactly this shape in
`interval_hull_comm_inside` and `interval_intersect_comm_inside`
(`RationalInterval.ken.md:790`, `:815`) on `64e6bf67e`. 6 fired: the
claim that no law states either bound was measured by grepping spec 68
and the package for "least" and "greatest", outside the `proof least`
and `proof greatest` applications. The only hits are §2.1's endpoint
prose and a note in the package's trust and derivation section. The
32 `pub theorem`s were listed, and none takes an inclusion premise into
a hull or intersection conclusion.
