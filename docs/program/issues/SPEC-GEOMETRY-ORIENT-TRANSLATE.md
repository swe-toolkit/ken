---
id: SPEC-GEOMETRY-ORIENT-TRANSLATE
title: "Spec 67 §18 fixes orient by its natural cross sums and states swap, rotation and repeated-point laws, but no law says the turn is unchanged when all three points move by the same vector, so a client cannot carry an orientation result across a shift of its coordinate origin. Rule from a checked D0 at zero trust that translating all three points by one (dx, dy) leaves orient unchanged, and state it with a seed that separates a shared translation from moving one point alone"
status: ready
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Geometry follows Optimization in the L3 layer rotation (docs/program/06-catalog-campaign.md). §18 and §19 give orient no invariance under a change of origin. Steward-filed per COORDINATION section 2."
---

# Orientation is unchanged by a shared translation

## Objective

`Algorithm.Geometry.NatOrientation` exports a public checked law. For
every set of coordinates and every `dx` and `dy`, moving all three points
by `(dx, dy)` leaves `orient` unchanged, as `Equal OrdResult`. It holds
at zero trust, and spec 67 states it by its public type.

## Settled inputs (main `9286f8fec`)

- **The operation.** `orient` (`NatOrientation.ken.md` ~`:25`) is
  `compare L R`, where `L` and `R` are the two natural cross sums of spec
  67 §18.1.
- **Why it holds.** Moving every point by `(dx, dy)` adds the same natural
  term to `L` and to `R`. That term is
  `dy(xa + xb + xc) + dx(ya + yb + yc) + 3 dx dy`. So the law is
  `compare (L + T) (R + T) = compare L R`, after the two sums are
  rearranged.
- **The vocabulary.**
  - NatOrientation imports `add` and `mul` from `Data.Numeric.Nat.Arithmetic`
    and `compare` from `Data.Numeric.Nat.Order`.
  - Arithmetic exports `add` `assoc` and `comm`, `mul` `comm` and `assoc`,
    and both `mul_add_distrib` laws (`:46` to `:213`).
  - Order exports no law that cancels a shared addend inside `compare`.
    `compare` goes through `leq_nat` (`Order.ken.md` ~`:506`).
- **The neighbours.** The public laws are `orient_cross`, `_swap`,
  `_rotation`, `_repeated_point` (§18), `_shared_x` and `_shared_y` (§19).
  None relates two triples whose coordinates differ.
- **Spec and tests.** Spec 67 ends at §20
  (`spec/50-stdlib/67-geometry.md` ~`:1614`), so the new section is §21.
  The seed directory is `conformance/stdlib/geometry/`. The test pattern
  is `crates/ken-elaborator/tests/cat_geometry_nat_orientation.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 67 §21, the catalog law with any private helper, a seed, a package
test and the status lines. No §1 to §20 operation or law type moves, and
there is no kernel or surface change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-GEOMETRY-ORIENT-AXIS-COLLINEAR.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules:
  - the name and binder order, stated at coordinates as `orient_cross`
    is, with no new translation operation;
  - where the shared-addend cancellation for `compare` lives: a private
    helper in NatOrientation, or a public law in `Data.Numeric.Nat.Order`.
    If it is public, name the second package and sweep its consumers.

  Sweep the consumers of the package's public roster for the sibling
  laws.
- **AC-1.** §21 states the law by its public type. The Spec Leader and
  CV review it, with a Decision.
- **AC-2.** The seed's accepted block applies the law to the
  counter-clockwise triple `(0,0), (1,0), (0,1)` moved by `(2,3)` to
  `(2,3), (3,3), (2,4)`. It concludes that the moved triple's turn equals
  the original's, `Gt`. On the moved triple, `L = 24` and `R = 23`.

  The refused block is on a fresh accepted prefix, in the same claim
  form. It moves only the first point: `(2,3), (1,0), (0,1)`. Its claim
  that this triple's turn equals the original's is refused. On it,
  `L = 1` and `R = 5`, so the turn is `Lt`. The two rows differ only in
  which points move.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise.
- A change to an existing §1 to §20 operation or law type.
- The rearrangement of `L` and `R` needs a normalization procedure the
  catalog does not have. Report the size of the hand proof instead of
  adding a tactic.

Checks: 3 fired. I listed every public NatOrientation law on `9286f8fec`
(six), and none relates two coordinate triples. I also checked
`Data.Numeric.Nat.Order` for a cancellation of a shared addend under
`compare`, and it has none, so AC-0 rules where that helper lives.
4 fired. I read `orient`'s body and expanded both cross sums under the
shift by hand. The added term is the same in `L` and `R`. 8 fired. In the
refused row only one point moves, and the turn flips from `Gt` to `Lt`.
A law that held for any per-point shift would wrongly accept it.
