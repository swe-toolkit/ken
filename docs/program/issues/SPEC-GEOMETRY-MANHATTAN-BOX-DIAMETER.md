---
id: SPEC-GEOMETRY-MANHATTAN-BOX-DIAMETER
title: "Spec 67 gives boxes point membership and points a Manhattan metric, but no law connects them: a client cannot bound the distance between two points it knows lie in a box. Rule from a checked D0 at zero trust that two points contained in MkBox lo hi are at most manhattan lo hi apart, and state the law with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Geometry is next in the layer rotation after Optimization. Steward-filed per COORDINATION section 2."
---

# Points in a box are no farther apart than its corners

## Objective

Spec 67 states, as a public checked law over `Algorithm.Geometry.NatBox`,
that for every `lo`, `hi`, `p` and `q` with
`IsTrue (contains (MkBox lo hi) p)` and
`IsTrue (contains (MkBox lo hi) q)`,
`IsTrue (leq_nat (manhattan p q) (manhattan lo hi))`.

## Settled inputs (spec 67 §1 to §9, main `bf9afd79f`)

- **The operations.** `contains` is the conjunction of `axis_contains lx
  hx px` and `axis_contains ly hy py`, each `leq_nat lo x` and `leq_nat x
  hi` (`NatBox.ken.md:66`, `:69`). `manhattan` is the sum of the private
  `axis_distance` on each axis (`:129`).
- **The metric half exists.** §2.1 states `manhattan_triangle` and
  `manhattan_zero_equal` (`67-geometry.md:189`), exported at
  `NatBox.ken.md:951` and `:991`.
  Their per-axis lemmas are private (§2.2), and `axis_distance` stays
  private.
- **Not yet stated.** No §1 to §9 law relates `contains` to
  `manhattan`. The law needs, per axis, that two naturals between `lo`
  and `hi` are at most `axis_distance lo hi` apart; that helper is new
  private proof structure.
- **Inverted boxes.** An inverted box contains no point (§3.2), so the
  law is vacuous there; no validity premise is added.
- **Seeds.** Under `conformance/stdlib/geometry/`, next to
  `seed-manhattan-metric.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §10 in spec 67 stating the ruled law by its public type, the catalog
theorem in `NatBox`, a closed conformance seed under
`conformance/stdlib/geometry/`, and the status lines, all in one
candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules:
  - the name, argument order, premise form and placement;
  - whether the attainment companion (the corners of a non-inverted box
    are contained in it) belongs in this §10 or stays out;
  - which helpers stay private.

  It also confirms that no §1 to §9 operation or law type changes.
- **AC-1.** §10 states each ruled law by its public type. The Spec
  Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client uses the box `(1,1)–(4,3)`, whose
  corners are `5` apart.
  - It applies the law to `p = (1,3)` and `q = (4,1)` and checks
    independently by `Proved` that their distance is `5`, so the bound
    is tight.
  - Its refused block pins the bound for `p = (0,3)`, outside the box,
    and `q = (4,1)`: their distance is `6`, so the containment premise
    is load-bearing.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, a validity
  premise, or a premise that names a private helper.
- A change to an existing §1 to §9 operation or law type.

Checks: 4 fired: `contains`, `axis_contains`, `manhattan`,
`manhattan_triangle` and `manhattan_zero_equal` were read at their lines
on `bf9afd79f`; the public theorem list has no law over both `contains`
and `manhattan`. 8 fired: the accepted pair attains the corner distance,
so a law with a strictly smaller bound fails the seed, and the refused
pair differs only by moving `p` one unit outside the box, so a law
without the containment premise accepts it.
