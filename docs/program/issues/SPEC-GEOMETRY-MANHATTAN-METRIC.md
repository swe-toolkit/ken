---
id: SPEC-GEOMETRY-MANHATTAN-METRIC
title: "Spec 67 defines Manhattan distance on natural points and proves symmetry and self-zero, but defers the triangle inequality, so manhattan is not yet a metric and no client can bound a detour. Author section 2: the triangle inequality and zero-distance identity of indiscernibles, proved generically at zero trust"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: [CAT-GEOMETRY-NAT-BOX]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 67 section 1.4 defers the Manhattan triangle inequality; it needs no signed or Float carrier. Same shape as SPEC-SYMBOLIC-ALGEBRA-CANONICAL-EQUALITY. Steward-filed per COORDINATION section 2."
---

# Manhattan distance is a metric

## Objective

Spec 67 gains a normative section 2 in which `manhattan` is a metric on
`Point`: zero distance holds exactly for equal points, and the triangle
inequality holds. Both laws are proved generically with no Axiom, and the
catalog node builds against them.

## Settled inputs (on `134fd39b3`)

- **Section 1 is delivered.** `Algorithm.Geometry.NatBox`
  (`catalog/packages/Algorithm/Geometry/NatBox.ken.md`) defines
  `manhattan` as the sum of two saturating-subtraction absolute differences
  (spec 67 section 1.2). It proves `manhattan_symm` and `manhattan_self`.
- **The deferral.** Section 1.4 defers "the Manhattan triangle
  inequality". Signed coordinates, Euclidean distance, box area and other
  dimensions stay deferred.
- **Nat order is proved.** `Data.Numeric.Nat.Order` exports `leq_nat`,
  `sub`, `leq_nat_add_left_bound` and `leq_nat_add_right_bound`. Import
  `leq_nat` from `Core.Classes.LawfulClasses`, because the `Order` facade
  fails with `UnboundName ...leq_nat::trans`
  (CAT-SYMBOLIC-ALGEBRA-CANONICAL-EQUALITY D0).
- **Known elaborator limits.** A dependent `match` refines only its
  scrutinee as written (CHECKS 11).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (Architect boundary ruling, measured).** From a checked
   development (rc 0, no Axiom), rule:
   - the triangle law's statement, for example
     `IsTrue (leq_nat (manhattan p r) (add (manhattan p q) (manhattan q
     r)))`, and its per-axis lemma;
   - the identity law, `Equal Nat (manhattan p q) Zero` implies
     `Equal Point p q`, or a measured blocker;
   - whether any section 1 helper must become public.
2. **Section 2 of spec 67**, stating exactly the D0's proved laws, with a
   conformance seed of closed cases.

## Acceptance

- **AC-1.** The D0 development checks generically with no Axiom, and the
  section states exactly the laws it proved.
- **AC-2.** The seed has a case per law, including a closed detour
  `manhattan p r <= manhattan p q + manhattan q r` with strict slack, and
  a refused claim that two distinct points are at distance zero.
- **AC-3.** `spec/50-stdlib/README.md` and `SPEC-PROGRESS.md` record the
  section.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a new primitive.
- The identity law cannot be proved generically: report the measured
  blocker, and do not state it as proved.
