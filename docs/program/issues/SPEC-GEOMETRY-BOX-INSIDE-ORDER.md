---
id: SPEC-GEOMETRY-BOX-INSIDE-ORDER
title: "Spec 67 section 3 puts an intersection inside each operand and each operand inside the hull, but states neither that inside is transitive nor that intersect and hull are the greatest and least such boxes. A client cannot chain two containments or conclude that a box inside both operands is inside their intersection. Rule those laws from a checked D0 at zero trust and state them as a section 5 with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 67 section 3.2 exports only the one-sided intersect and hull containments. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md). Same shape as SPEC-GEOMETRY-BOX-AREA. Steward-filed per COORDINATION section 2."
---

# Containment orders boxes, with intersect and hull as its bounds

## Objective

Spec 67 states, as public checked laws over
`Algorithm.Geometry.NatBox`, that `inside` is transitive and that
`intersect b c` is the greatest box inside both `b` and `c` and
`hull b c` the least box containing both.

## Settled inputs (spec 67 §3 and §4, main `c0ff598df`)

- **`inside` (§3.1).** The corner-wise Boolean
  `axis_within blx bhx clx chx` and the same on y, with
  `axis_within l1 h1 l2 h2 = bool_and (leq_nat l2 l1) (leq_nat h1 h2)`.
  No validity premise and no empty-box branch, so inverted boxes compare
  by their corners. It is `NatBox.ken.md:819`.
- **The one-sided laws (§3.2).** `inside_refl`, `intersect_inside_left`
  and `_right`, `inside_hull_left` and `_right`, `inside_sound` and the
  witnessed `inside_complete` are public at `NatBox.ken.md:840-1046`.
  `area_inside` (§4.2) consumes `inside`.
- **The gap.** No public law chains `inside` or bounds `intersect` from
  below and `hull` from above. `inside_trans`, a greatest-intersection
  law and a least-hull law are absent from `spec/` and `catalog/`.
- **Why they can be unconditional.** Each is a per-axis composition
  of the checked Nat order with `max` and `min` (the `greatest for min`
  and `least for max` bounds §1.4 already published), so no inverted-box
  case, canonical empty box or point-set equality is needed.
- **Seeds.** Under `conformance/stdlib/geometry/`, next to
  `seed-box-containment.md`.

Treat anchors as perishable. If a fixed input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §5 in spec 67 stating each ruled law by its public type, the §3.3
scope sentence updated to point at it, and a closed conformance seed
under `conformance/stdlib/geometry/`, with the status lines changed in
the same candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged. It rules the
  names and public types of transitivity and of the greatest and least
  laws, and which per-axis helpers stay private.
- **AC-1.** The §5 states each ruled law by its public type, and §3.3
  points to it. The Spec leader and CV review it, with a Decision.
- **AC-2.** The seed checks each law on closed boxes, including an
  inverted box on at least one side of the premise, and has refused
  blocks that claim `inside (hull b c) b` where `c` is not inside `b`,
  and `inside b (intersect b c)` where `b` is not inside `c`. The
  candidate's handoff shows a `ken check` rc on its blocks.

## Stop conditions

- A law that needs a validity premise, a canonical empty box, point-set
  equality, or a change to `inside`, `intersect`, `hull`, `area` or
  their existing laws, unless the D0 rules the premise.
- Any new primitive, kernel or surface feature, or a public Nat order
  lemma beyond those §1.4 already published.

Checks: 4 fired: `inside` and the seven §3.2 laws are `pub` at
`NatBox.ken.md:819-1046` on `c0ff598df`; `grep -rn inside_trans spec
catalog` is empty. 8 fired: AC-2's refused blocks are the reverse
inclusions, which hold only under the new laws' premises, so a §5 law
stated without its premise would admit them.
