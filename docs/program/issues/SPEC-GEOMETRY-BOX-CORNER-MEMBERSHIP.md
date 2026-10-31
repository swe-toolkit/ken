---
id: SPEC-GEOMETRY-BOX-CORNER-MEMBERSHIP
title: "Spec 67 §10 bounds the distance between two points in MkBox lo hi by manhattan lo hi, but no public law says the bound is reached: a client cannot conclude that a box containing some point contains its own corners. Rule from a checked D0 at zero trust that a box containing a point contains lo and hi, and state the two laws with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-GEOMETRY-MANHATTAN-BOX-DIAMETER]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Named as the possible successor by Architect AC-0 evt_3ed16jma3qjxs on SPEC-GEOMETRY-MANHATTAN-BOX-DIAMETER. Steward-filed per COORDINATION section 2."
---

# A box that contains a point contains its corners

## Objective

Spec 67 states, as public checked laws over `Algorithm.Geometry.NatBox`,
that for every `lo`, `hi` and `p` with `IsTrue (contains (MkBox lo hi)
p)`, both `IsTrue (contains (MkBox lo hi) lo)` and `IsTrue (contains
(MkBox lo hi) hi)` hold.

## Settled inputs (spec 67 §1 to §10, main `618450bf3`)

- **The proof exists privately.** `axis_corner_low` and
  `axis_corner_high` (`NatBox.ken.md:2080`, `:2095`) turn one axis
  membership into membership of that axis's lower and upper bound. They
  are private, and `inside_complete` uses them (`:1591`, `:1632`).
- **The bound exists.** §10's `manhattan_box_diameter` (`:1016`) bounds
  two contained points by `manhattan lo hi`. With these laws, the corners
  are two contained points at exactly that distance, so the bound is
  attained whenever the box contains any point.
- **The witness premise is load-bearing.** An inverted box such as
  `(3,3)–(1,1)` contains no point, and does not contain its `lo` corner.
- **Seeds.** Under `conformance/stdlib/geometry/`, next to
  `seed-manhattan-box-diameter.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §11 in spec 67 stating the ruled laws by their public types, the two
catalog theorems in `NatBox`, a closed conformance seed under
`conformance/stdlib/geometry/`, and the status lines, all in one
candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules:
  - the names, argument order, premise form and placement;
  - whether the axis helpers are promoted or wrapped, and which stay
    private.

  It also confirms that no §1 to §10 operation or law type changes.
- **AC-1.** §11 states each ruled law by its public type. The Spec
  Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client uses the box `(1,1)–(4,3)` and the
  witness `(2,2)`.
  - It applies both laws and checks independently by `Proved` that the
    box contains `(1,1)` and `(4,3)`.
  - Its refused block pins `IsTrue (contains (MkBox (MkPoint 3 3)
    (MkPoint 1 1)) (MkPoint 3 3))` for the inverted box, so a law
    without the witness premise is refused.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, a validity premise,
  or a premise that names a private helper.
- A change to an existing §1 to §10 operation or law type.

Checks: 4 fired: `axis_corner_low`, `axis_corner_high`,
`inside_complete` and `manhattan_box_diameter` were read at their lines on
`618450bf3`; neither corner law is public. 8 fired: the refused claim is
the inverted box's own `lo` corner, for which `contains` computes `False`
(`leq_nat 3 1`), so dropping the witness premise fails the seed.
