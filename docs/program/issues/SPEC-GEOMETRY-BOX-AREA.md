---
id: SPEC-GEOMETRY-BOX-AREA
title: "Spec 67 defers box area: a client of NatBox can compare boxes by containment but cannot measure one, so nothing states that a box inside another has no greater area, or that an intersection is no larger than either input. Rule a natural-valued area and its laws from a checked D0 at zero trust, and state them as a section 4 with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 67 section 1.4 defers box area. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md). Same shape as SPEC-GEOMETRY-BOX-CONTAINMENT. Steward-filed per COORDINATION section 2."
---

# The area of a natural box

## Objective

Spec 67 states a public natural-valued area for an
`Algorithm.Geometry.NatBox` box, with checked laws that tie it to the
containment test of §3 and to `intersect`.

## Settled inputs (spec 67 §1 to §3, on `main` at `438a18b97`)

- **Carriers (§1.1).** `Point = MkPoint Nat Nat` and `Box = MkBox Point
  Point`, with inclusive bounds. A box inverted on either axis contains
  no point and is still a valid `Box`; `Eq Box` is structural.
- **The deferral (§1.4).** "box area" is listed among the deferred
  operations, beside a canonical empty box and point-set equality.
- **Containment (§3).** `pub fn inside (b c : Box) : Bool` compares
  corners with `leq_nat` even when `b` is inverted, with the seven
  §3.2 laws, including `intersect_inside_left` and `_right`.
- **Package.** `catalog/packages/Algorithm/Geometry/NatBox.ken.md`.
  `Data.Numeric.Nat` supplies `add`, `mul` and the checked `leq_nat`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

A new spec 67 section stating the area operation and each ruled law by
its public type, the §1.4 deferral sentence corrected, and a closed
conformance seed under `conformance/stdlib/geometry/`, with the status
lines changed in the same candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0 and no `Axiom`. It rules:
  - the operation's name and definition, including the inclusive span
    of an axis and the area of a box inverted on either axis;
  - monotonicity: `inside b c` gives area `b` at most area `c`, with
    whatever premise an inverted `c` needs stated or excluded;
  - the area of `intersect b c` against each input, and whether this
    follows from monotonicity and §3.2 without a new statement;
  - the private helpers the proof needs.
- **AC-1.** The section states each ruled operation and law by its
  public type, and the deferral sentence is corrected. The Spec leader
  and CV review it, with a Decision.
- **AC-2.** The seed checks the area of a closed single-point box, a
  closed proper box and an inverted box, and monotonicity on a closed
  nested pair. It has a refused block that claims a strictly larger
  area for the inner box of that pair.

## Stop conditions

- A law that needs signed coordinates, a canonical empty box or
  point-set equality of boxes, unless the D0 rules it as the stated
  premise.
- Any new primitive, kernel or surface feature, or a change to a §1 to
  §3 operation or law.
- An ambient name in the package beyond its current census row
  (`lang_mod_strict_resolution_d0`): import it explicitly.

Checks: 4 fired: `inside` and the §3.2 laws are `pub` in `NatBox.ken.md`
at `438a18b97`, and §1.4 lists box area as deferred. 7 fired: the seed
covers an inverted box, where a naive span difference would underflow
to a wrong positive area.
