---
id: CAT-NUMERIC-INTERVAL-HULL
title: "Spec 68 section 2 specifies interval hull and intersection with four containment laws over six new Rational min/max order proofs, but no catalog package delivers them. Export rational_min and rational_max with their proofs from Data.Numeric.Rational, extend Algorithm.Numeric.RationalInterval from the Architect's checked D0 at zero trust, and run the section 2 seed as an acceptance target"
status: ready
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-NUMERICAL-INTERVAL-HULL]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 68 section 2 from SPEC-NUMERICAL-INTERVAL-HULL (Architect D0 on e07a73535, Decision dec_ebytsfhh39qf). Same shape as CAT-NUMERIC-RATIONAL-INTERVAL. Steward-filed per COORDINATION section 2."
---

# Interval hull and intersection, delivered and proved

## Objective

`Algorithm.Numeric.RationalInterval` exports `interval_hull` and
`interval_intersect` with the four spec 68 §2.2 laws proved, and the §2
seed checks against the shipped package.

## Settled inputs (spec 68 §2, SPEC-NUMERICAL-INTERVAL-HULL)

- **Operations (§2.1).** Hull is `MkInterval (rational_min a c)
  (rational_max b d)`; intersection is `MkInterval (rational_max a c)
  (rational_min b d)`, returning an `Interval`, not an `Option`. Min and
  max select stored values by `rational_leq`, without normalizing.
- **Laws (§2.2).** `interval_hull_contains_left`, `_right`,
  `interval_intersect_sound` (into the ambient Ω-valued `And`, not
  `Core.Logic.And`), and `interval_intersect_complete`. No ordered-endpoint
  premise. No structural commutativity, idempotence or associativity law.
- **Rational prerequisites (§2.3).** `Data.Numeric.Rational` exports
  `rational_min`, `rational_max` and six proofs: `leq_left`, `leq_right`
  and `greatest for rational_min`; `left_leq`, `right_leq` and `least for
  rational_max`. The D0's `rational_pick` and `pick_*` helpers stay
  private. The interval module adds only the two names to its Rational
  import list and does not clone ordering functions or proofs.
- **Checked in scratch (Architect D0, on `e07a73535`).** Both modules
  `ken check` rc 0, with the four laws, a 16-assertion closed client, and
  four negative clients rejected. No new `Axiom`, primitive, kernel or
  surface change.
- **Seed.** `conformance/stdlib/numerical/seed-interval-hull-intersection.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The Rational exports in `catalog/packages/Data/Numeric/Rational.ken.md`
and the extension in
`catalog/packages/Algorithm/Numeric/RationalInterval.ken.md`, with the §2
seed run from an acceptance test beside
`crates/ken-elaborator/tests/cat_numeric_rational_interval.rs`.

## Acceptance

- **AC-1.** Both packages check with the four laws and six proofs as
  public, proved theorems. `trusted_base()` identity count is unchanged.
- **AC-2.** The §2 seed's positive cases check and each of its three
  negative blocks is refused, run one at a time against the shipped
  package.
- **AC-3 (mutation, QA).** Swapping min for max in `interval_intersect`'s
  lower bound makes `interval_intersect_sound` or the seed fail.
- The §1 interval suite stays green.

## Stop conditions

- Any `Axiom`, admitted body, primitive, or kernel or surface change.
- A §2.2 law that needs an ordered-endpoint premise or a structural
  equality.
