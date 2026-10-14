---
id: SPEC-GEOMETRY-EXACT-CONTRACT
title: "No geometry contract exists in spec/50-stdlib, so the catalog's next roadmap layer after symbolic algebra has nothing to build against. Author its chapter with a first exact section: points and axis-aligned boxes over natural coordinates, containment, intersection and hull, with their laws proved at zero trust"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: [CAT-SYMBOLIC-ALGEBRA-NAT-POLYNOMIAL]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. 06-catalog-campaign.md:375-376 places geometry (exact-before-float) after symbolic algebra. Same shape as SPEC-SYMBOLIC-ALGEBRA-CONTRACT. Steward-filed per COORDINATION section 2."
---

# An exact geometry contract

## Objective

A new `spec/50-stdlib` chapter exists, and its first section is the
normative contract for exact geometry over natural coordinates. The
first geometry catalog node builds against it.

## Settled inputs (on `c50808381`)

- **Natural order is proved.** `Data.Numeric.Nat.Order` exports `min`,
  `max`, saturating `sub`, `leq_nat`, `lt_nat` and `compare`. Its public
  laws include `leq_left` and `leq_right` for `min`, `left_leq` and
  `right_leq` for `max`, and `leq_nat_mul_right_mono`. The bound laws
  `greatest` for `min` and `least` for `max` exist but are private.
- **Natural arithmetic is proved.** `Data.Numeric.Nat.Arithmetic`
  exports `add` and `mul` with their associativity, commutativity, unit
  and distributivity laws.
- **No geometry declaration exists.** Nothing in `catalog/`,
  `spec/50-stdlib` or `conformance/` declares a point, a box, an
  interval or a distance.
- **No signed carrier has proved laws.** There is no `Int` catalog
  package. Spec 64's `Rational` is non-negative and has no subtraction.
  So an orientation or cross-product predicate over signed coordinates
  has nothing proved to stand on.
- **Known elaborator limits.** A λ passed to `map` hits
  `LANG-ELABORATED-BETA-REDEX-AT-KERNEL-INFERENCE`; use a named helper,
  as spec 65 does. A dependent `match` refines only its scrutinee as
  written (CHECKS 11).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (Architect boundary ruling, measured).** Rule from a checked
   development (rc=0, no Axiom) on:
   - the chapter number, the package's Section and its module path;
   - the carriers: for example a point of two `Nat` coordinates and a
     closed axis-aligned box given by two corners, and how an empty
     intersection is represented;
   - the operations: point-in-box membership, box intersection, the
     bounding hull of two boxes, and one exact measure such as
     Manhattan distance or box area;
   - the laws: at least that a point lies in the intersection of two
     boxes exactly when it lies in both, and that the hull contains
     both boxes. Distance laws (symmetry, the triangle inequality) are
     in scope only if the D0 measures them;
   - whether the private `greatest` and `least` bounds must become
     public, which is a change to `Data.Numeric.Nat.Order` the catalog
     node then owns;
   - what stays deferred: signed coordinates, orientation, rotation,
     Euclidean distance and floating point.
2. **The chapter, section 1**, stating the D0's operations and proved
   laws, with a conformance seed of closed cases.

## Acceptance

- **AC-1.** The Architect's D0 development checks generically, with no
  Axiom, and the section states exactly the laws it proved.
- **AC-2.** The seed has at least one case per operation and one per
  law, and a case that refuses an intersection which keeps a point
  lying in only one box.
- **AC-3.** `spec/50-stdlib/README.md` and `SPEC-PROGRESS.md` record the
  chapter.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a new primitive.
- The D0 cannot check a membership law generically. Report the measured
  blocker, and do not state that law as proved.
