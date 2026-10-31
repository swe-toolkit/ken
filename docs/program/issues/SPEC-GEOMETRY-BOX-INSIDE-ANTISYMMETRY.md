---
id: SPEC-GEOMETRY-BOX-INSIDE-ANTISYMMETRY
title: "Spec 67 makes corner-wise inside reflexive (§3) and transitive (§5), but no public law makes it antisymmetric, so a client holding inside a b and inside b a cannot conclude Equal Box a b, and inside is not yet a partial order. Rule from a checked D0 at zero trust that mutual inside gives structural box equality, for every Box including inverted ones, and state the law with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-GEOMETRY-BOX-CORNER-MEMBERSHIP]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Successor to SPEC-GEOMETRY-BOX-CORNER-MEMBERSHIP. Steward-filed per COORDINATION section 2."
---

# Corner-wise inclusion is antisymmetric

## Objective

Spec 67 states, as a public checked law over `Algorithm.Geometry.NatBox`,
that for every `a` and `b` of type `Box`, `IsTrue (inside a b)` and
`IsTrue (inside b a)` give `Equal Box a b`.

## Settled inputs (spec 67 §1 to §11, main `dd1e0d820`)

- **The relation is corner-wise.** `inside` (`NatBox.ken.md:1430`) is
  `bool_and` of the private `axis_within` per axis (`:2044`), which is
  `leq_nat` of the lower bounds and of the upper bounds. Mutual `inside`
  therefore gives `leq_nat` in both directions on all four coordinates.
- **The pieces exist.** `inside_refl` (`:1509`) and `inside_trans`
  (`:1757`) are public. `proof antisym for leq_nat` is in
  `Data.Numeric.Nat.Order` (used at `Order.ken.md:471`). The private
  `box_equal` (`:1334`) assembles `Equal Box` from two `Equal Point`.
- **The scope note.** §5.1 ends by saying its laws are not "an
  additional public antisymmetry law". That records §5's scope; it is not
  a ruling against the law. This WP replaces that sentence.
- **Inverted boxes keep their corners.** The law holds without a validity
  premise, and it does not identify distinct empty boxes: `(3,3)–(1,1)`
  and `(2,2)–(0,0)` contain no point, but neither is inside the other.
- **Seeds.** Under `conformance/stdlib/geometry/`, next to
  `seed-box-inside-order.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §12 in spec 67 stating the ruled law by its public type, the catalog
theorem in `NatBox`, a closed conformance seed under
`conformance/stdlib/geometry/`, the §5.1 sentence replaced, and the status
lines, all in one candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules the name, argument order and placement, and which
  helpers stay private. It confirms that no §1 to §11 operation or law
  type changes.
- **AC-1.** §12 states the ruled law by its public type. The Spec Leader
  and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the law generically and to
  the closed box `(1,1)–(2,2)`. Its refused blocks pin:
  - `IsTrue (inside (MkBox (MkPoint 0 0) (MkPoint 3 3)) (MkBox (MkPoint
    1 1) (MkPoint 2 2)))`, the missing direction for `(1,1)–(2,2)` inside
    `(0,0)–(3,3)`, so a one-premise law is refused;
  - `Equal Box` of the two inverted boxes `(3,3)–(1,1)` and
    `(2,2)–(0,0)`, so the law does not collapse empty boxes.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, a validity premise,
  or a premise that names a private helper.
- A change to an existing §1 to §11 operation or law type.

Checks: 4 fired: `inside`, `axis_within`, `inside_refl`, `inside_trans`
and `box_equal` were read at their lines on `dd1e0d820`, and
`leq_nat::antisym` at `Order.ken.md:471`; no public antisymmetry law
exists in NatBox. 8 fired: `(1,1)–(2,2)` is inside `(0,0)–(3,3)` but not
conversely, and the boxes are not equal, so a law with only the first
premise is false and the seed's first refused block fails it.
