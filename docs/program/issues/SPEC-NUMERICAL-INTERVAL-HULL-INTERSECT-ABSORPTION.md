---
id: SPEC-NUMERICAL-INTERVAL-HULL-INTERSECT-ABSORPTION
title: "Spec 68 states hull and intersection as tightest bounds under interval_inside, but no law relates the two operations. Rule from a checked D0 at zero trust that each absorbs the other as endpoint-value mutual inclusion, for every two intervals including inverted ones, and state the laws with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Numerical is next in the layer rotation after Graphs. Same shape as SPEC-GEOMETRY-BOX-INTERSECT-HULL-ABSORPTION. Steward-filed per COORDINATION section 2."
---

# Interval hull and intersection absorb each other

## Objective

Spec 68 states, as public checked laws over
`Algorithm.Numeric.RationalInterval`, that for every two intervals `i`
and `j`, including inverted ones:

- `interval_intersect i (interval_hull i j)` and `i` are each inside the
  other;
- `interval_hull i (interval_intersect i j)` and `i` are each inside the
  other.

## Settled inputs (spec 68 §1 to §7, main `944cabd38`)

- **The operations.** `interval_hull` takes `rational_min` of the lower
  endpoints and `rational_max` of the upper ones. `interval_intersect`
  takes the reverse (`RationalInterval.ken.md:322`, `:330`).
  `interval_inside i k` compares endpoint values (`:340`). Laws are
  stated as inclusion, not structural equality, because normalized
  endpoint spellings stay deferred (§1.4).
- **No mixed law.** §2, §5 and §7 each relate one operation to
  inclusion. No section states absorption; `absorb` has no hit in spec
  68 or the package.
- **Why it holds.** Two directions are already instances of public laws:
  `interval_intersect_inside_left` (`:541`) and
  `interval_inside_hull_left` (`:511`). The other two compose from public
  laws with no premise:
  - `i` inside `interval_intersect i (interval_hull i j)` is
    `interval_intersect_greatest` (`:1010`) applied to
    `interval_inside_refl i` (`:377`) and `interval_inside_hull_left`;
  - `interval_hull i (interval_intersect i j)` inside `i` is
    `interval_hull_least` (`:978`) applied to `interval_inside_refl i` and
    `interval_intersect_inside_left`.
- **Seeds.** Under `conformance/stdlib/numerical/`, next to
  `seed-interval-hull-intersect-bounds.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

An §8 in spec 68 stating the ruled laws by their public types, a closed
conformance seed under `conformance/stdlib/numerical/`, and the status
lines, all in one candidate. The catalog half travels in the candidate
when AC-0 rules it. Otherwise it is a separate Foundation WP built from
the D0.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules:
  - the names, and whether the public surface is the two new directions
    or all four (the §6 `_inside`/`_rev_inside` pairing);
  - the argument order;
  - which helpers, if any, stay private.

  It also confirms that no §1 to §7 operation or law type changes.
- **AC-1.** §8 states each ruled law by its public type. The Spec Leader
  and CV review it, with a Decision.
- **AC-2.** The seed's accepted client uses two closed intervals with
  distinct endpoints that overlap and neither contains the other.
  - It applies each law and checks each premise-free conclusion
    independently by `Proved`.
  - Its refused block pins one false mixed claim on the same intervals,
    with the shared argument moved:
    `interval_hull (interval_intersect i j) j` inside `i`. For `[0,2]`
    and `[1,3]` that is `[1,3]` inside `[0,2]`. It also carries an
    accepted same-shape control and an independent `False` value.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, a new Rational
  export, or any premise.
- A change to an existing §1 to §7 operation or law type.

Checks: 4 fired: the two compositions were read against the public types
of `interval_intersect_greatest`, `interval_hull_least`,
`interval_inside_refl`, `interval_inside_hull_left` and
`interval_intersect_inside_left` on `944cabd38`, and each argument has
the type the composition needs, with no premise left over. 8 fired: the
refused row has the accepted law's shape with the shared argument moved,
and for the seed's intervals its upper endpoint 3 exceeds 2, so a kernel
that accepted it would show the law was not checked.
