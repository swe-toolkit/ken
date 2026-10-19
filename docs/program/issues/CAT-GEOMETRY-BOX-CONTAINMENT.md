---
id: CAT-GEOMETRY-BOX-CONTAINMENT
title: "Algorithm.Geometry.NatBox exports spec 67 section 3: the corner-wise box test inside and its seven laws (soundness, reflexivity, intersect inside each input, each input inside the hull, and the witnessed converse). Proved as checked terms at zero trust, from the Architect's D0 development, with the containment seed run by the package test"
status: ready
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-GEOMETRY-BOX-CONTAINMENT]
blocks: []
github: null
origin: "Operator 2026-09-13: every catalog package fully proved. SPEC-GEOMETRY-BOX-CONTAINMENT states section 3 and records its laws as checked in the Architect's D0 scratch development (evt_117jpaxkqd01k), not yet exported. Same shape as CAT-SYMBOLIC-ALGEBRA-SUBSTITUTION. Steward-filed per COORDINATION section 2."
---

# NatBox exports box containment and its laws

## Objective

`Algorithm.Geometry.NatBox` exports `inside` and its seven laws with
exactly spec 67 §3's public types, each law a checked proof.

## Settled inputs (spec 67 §3; Architect D0 `evt_117jpaxkqd01k`)

- **The surface.** `pub fn inside (b : Box) (c : Box) : Bool` decides
  corner-wise through a private `axis_within`, by `leq_nat`, with no
  inverted-box case. The laws are `inside_sound`, `inside_refl`,
  `intersect_inside_left`, `intersect_inside_right`,
  `inside_hull_left`, `inside_hull_right` and `inside_complete`, whose
  premise is a point `q` of `b`.
- **The proofs.** The intersect and hull laws use `left_leq`/`right_leq`
  for `max` and `leq_left`/`leq_right` for `min` from
  `Data.Numeric.Nat.Order`. `inside_refl` uses `proof refl for
  leq_nat`. The D0 package delta is `local/arch-box-d0-package.diff.txt`.
  It checked with no Axiom, `trusted_base()` 114 before and after, and
  the ambient census at residuals `[]`. Everything it uses is already
  imported.
- **Arrangement.** `inside` and the seven laws lead their block, and the
  private helpers follow.
- **Seed.** Spec 67 §3's containment seed under
  `conformance/stdlib/geometry/`, from SPEC-GEOMETRY-BOX-CONTAINMENT
  AC-2.
- **Package.** `catalog/packages/Algorithm/Geometry/NatBox.ken.md`,
  tested by `crates/ken-elaborator/tests/cat_geometry_nat_box.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

`inside`, `axis_within` and the seven laws in `NatBox.ken.md`, and test
rows that root-load each new public name and run the containment seed.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass. The test root-loads
  `inside` and the seven laws as package-owned and transparent, keeps
  `axis_within` private, and runs the seed's positive cases. Its refused
  block is pinned to the `inside` proposition between the seed's named
  inner box and the strictly smaller box, so an unrelated false claim
  does not satisfy it.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom or
  primitive. `lang_mod_strict_resolution_d0`'s
  `catalog_ambient_passthrough_migration_census` row for the package is
  unchanged, measured on the candidate.
- **AC-3 (mutation, QA).** Each mutant fails, and is restored:
  - swapping one `leq_nat` operand pair in `axis_within` fails
    `inside_sound` or `inside_refl`;
  - dropping the `y` axis from `inside` fails `inside_sound`;
  - removing `inside_complete`'s witness premise fails to check.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a public type
  other than §3's.
- A census row, or a consumer of `NatBox` outside this package, changes.
