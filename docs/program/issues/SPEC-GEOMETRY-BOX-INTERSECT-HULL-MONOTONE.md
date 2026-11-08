---
id: SPEC-GEOMETRY-BOX-INTERSECT-HULL-MONOTONE
title: "Spec 67 bounds box intersection and hull by their arguments (§3, §5) but states no law that either operation is monotone in containment: a client that shrinks both arguments must rebuild the bound by hand from four public laws. Rule from a checked D0 at zero trust that intersect and hull are each monotone under corner-wise inside, and state both laws with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Geometry follows Optimization in the L3 layer rotation (docs/program/06-catalog-campaign.md). Steward-filed per COORDINATION section 2."
---

# Box intersection and hull are monotone in containment

## Objective

`Algorithm.Geometry.NatBox` exports two public checked laws:
if `a` is inside `a2` and `b` is inside `b2`, then `intersect a b` is
inside `intersect a2 b2`, and `hull a b` is inside `hull a2 b2`. Spec 67
states them by their public types.

## Settled inputs (main `ddb2b24f8`)

- **The bound laws.** In `NatBox.ken.md`: `intersect_inside_left`
  (`:1597`), `intersect_inside_right` (`:1630`), `inside_hull_left`
  (`:1663`), `inside_hull_right` (`:1696`), `inside_trans` (`:1829`),
  `intersect_greatest a b c : inside a b → inside a c → inside a
  (intersect b c)` (`:1986`), and `hull_least b c d : inside b d →
  inside c d → inside (hull b c) d` (`:2052`). §16's two laws sit after
  `hull_least`, at `:2118` and `:2128`, and before `pub fn area` (`:2133`).
- **No monotone law.** A grep for `intersect_mono`, `hull_mono`,
  `intersect_monotone` and `hull_monotone` over `catalog/`, `spec/`,
  `conformance/` and `crates/` returns no file. Spec 67 ends at §16
  (`67-geometry.md:1338`).
- **Why it holds.** `intersect_greatest (intersect a b) a2 b2` takes two
  premises. Each is an `inside_trans` of an intersect bound with a given
  premise. `hull_least a b (hull a2 b2)` is the dual, built from the hull
  bounds. Neither needs a match, a Nat lemma or an import. Both hold for
  inverted boxes, because `inside` is corner-wise.
- **Seeds.** Under `conformance/stdlib/geometry/`, next to
  `seed-box-inside-intersect-hull.md`. The test pattern is
  `crates/ken-elaborator/tests/cat_geometry_box_inside_intersect_hull.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §17 in spec 67 stating both laws by their public types, a closed
conformance seed under `conformance/stdlib/geometry/`, and the status
lines, all in one candidate. The catalog half travels in the candidate
when AC-0 rules it, as it did for §16.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules the names and the placement of the two laws, and
  confirms that no §1 to §16 operation or law type changes.
- **AC-1.** §17 states both laws by their public types. The Spec Leader
  and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies both laws to closed boxes
  with both premises `Proved`. Its refused block pins, by the same form
  as an accepted control, one closed conclusion that fails because one
  premise fails. A kernel that accepts the refused row would show the
  premise was not checked. Each refusal sits on a fresh accepted prefix.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a premise beyond
  the two `inside` premises.
- A change to an existing §1 to §16 operation or law type.

Checks: 4 fired: the signatures of `intersect_greatest`, `hull_least`,
`inside_trans` and the four bound laws were read on `ddb2b24f8` at the
cited lines, and the composition type-checks on paper in the order they
take. 3 fired: the name grep covered `catalog/`, `spec/`, `conformance/`
and `crates/`, and found no file.
