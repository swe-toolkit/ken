---
id: CAT-GEOMETRY-MANHATTAN-METRIC
title: "Spec 67 section 2 states that Manhattan distance on natural points is a metric, with a proved triangle inequality and zero-distance identity, but Algorithm.Geometry.NatBox delivers only section 1. Deliver section 2 from the Architect's checked D0 at zero trust, with its seed as an acceptance target"
status: active
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-GEOMETRY-MANHATTAN-METRIC]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 67 section 2 from SPEC-GEOMETRY-MANHATTAN-METRIC (Architect D0 evt_1nzebfnfzwq0). Steward-filed per COORDINATION section 2."
---

# Manhattan distance is a metric, delivered and proved

## Objective

`Algorithm.Geometry.NatBox` gains spec 67 section 2 token for token: the
public theorems `manhattan_triangle` and `manhattan_zero_equal`, proved with
no Axiom. A client outside the module bounds a detour and concludes point
equality from zero distance.

## Settled inputs (Architect `evt_1nzebfnfzwq0`, on `3573ec369`)

- **The contract.** Spec 67 section 2, as landed by
  SPEC-GEOMETRY-MANHATTAN-METRIC, consists of:
  - `manhattan_triangle (p q r : Point) : IsTrue (leq_nat (manhattan p r)
    (add (manhattan p q) (manhattan q r)))`;
  - `manhattan_zero_equal (p q : Point) : Equal Nat (manhattan p q) Zero →
    Equal Point p q`.
- **A checked development exists.** The D0 appended a 253-line second fence
  to a scratch copy of `NatBox.ken.md`:
  - `ken check` returns rc 0, with no Axiom, primitive or postulate;
  - `cat_geometry_nat_box` passes 3/3 with `trusted_base()` 114 before and
    after.

  The source is `notes/manhattan-metric-d0-3573ec369.ken` on
  `architect/work`, and the seed client is
  `notes/manhattan-metric-d0-seed-pos-3573ec369.ken`. The Architect posts
  the full body inline in this WP's thread.
- **Imports.** Section 1's imports become
  `Data.Numeric.Nat.Order (min, max, sub, leq_nat_add_left_bound,
  leq_nat_weaken_right)` and `Core.Logic.Transport (cong, sym, trans)`.
- **Private helpers stay private.** The per-axis lemmas
  `axis_distance_triangle` and `axis_distance_zero`, `sub_triangle` and the
  addition and point-equality lemmas are private, and so is section 1's
  `axis_distance`. A client reference to `axis_distance` fails with
  `UnresolvedCon`.
- **Encoding note.** An `Equal Nat Zero Zero` arm must be `Proved`. `Refl`
  fails with "Refl expects an `Eq`-shaped goal".
- **The seed** is the CV's section 2 seed,
  `conformance/stdlib/geometry/seed-manhattan-metric.md`, landed with the
  spec.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. Section 2 in `catalog/packages/Algorithm/Geometry/NatBox.ken.md`: the
   D0's declarations and proof bodies, the two import changes, and the two
   laws as public theorems. Do not change another package.
2. An acceptance target in `crates/ken-elaborator/tests/` that loads the
   package, runs the section 2 seed as a client import, refuses its
   negatives, restates both law types generically, and records
   `trusted_base()` before and after loading.

## Acceptance

- **AC-1.** The package checks, both section 2 laws are checked proof
  bodies, and `trusted_base()` does not grow on loading it.
  `cat_geometry_nat_box` stays green.
- **AC-2.** The seed's positive cases pass from a client module, including
  the strict-slack detour `(0,0)→(3,2)→(1,1)`. The distinct-points-at-zero,
  reverse-detour and identity-on-distinct negatives are refused with K0004,
  as in the D0.
- **AC-3 (mutation, QA).** Swapping the `sub_triangle` arguments in
  `axis_distance_triangle` (the D0's M1) reddens a load-bearing proof or
  seed row. QA names the row and the assertion that fires.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a public signature
  that differs from spec 67.
- A section 1 declaration, a census row, or a catalog package other than
  this one changes.
