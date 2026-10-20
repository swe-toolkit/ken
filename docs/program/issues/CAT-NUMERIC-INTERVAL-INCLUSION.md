---
id: CAT-NUMERIC-INTERVAL-INCLUSION
title: "Algorithm.Numeric.RationalInterval exports spec 68 section 3: the endpoint-wise inclusion test interval_inside and its nine laws (soundness, reflexivity, add and mul isotonicity, each input inside the hull, the intersection inside each input, and the witnessed converse). Proved as checked terms at zero trust, from the Architect's D0 development, with the inclusion seed run by a package test"
status: ready
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-NUMERICAL-INCLUSION-ISOTONICITY]
blocks: []
github: null
origin: "Operator 2026-09-13: every catalog package fully proved. SPEC-NUMERICAL-INCLUSION-ISOTONICITY states section 3 and records its laws as checked in the Architect's D0 scratch development, not yet exported. Same shape as CAT-GEOMETRY-BOX-CONTAINMENT. Steward-filed per COORDINATION section 2."
---

# RationalInterval exports inclusion and its laws

## Objective

`Algorithm.Numeric.RationalInterval` exports `interval_inside` and its
nine laws with exactly spec 68 §3's public types, each law a checked
proof.

## Settled inputs (spec 68 §3; Architect D0)

- **The surface.** `pub fn interval_inside (i : Interval) (k :
  Interval) : Bool` is `bool_and (rational_leq c a) (rational_leq b d)`
  on `i=[a,b]`, `k=[c,d]`, with no inverted-interval case. The laws are
  `interval_inside_sound`, `interval_inside_refl`,
  `interval_add_inside`, `interval_mul_inside`,
  `interval_inside_hull_left`, `interval_inside_hull_right`,
  `interval_intersect_inside_left`, `interval_intersect_inside_right`
  and `interval_inside_complete`, whose premise is a member `q` of `i`.
- **The proofs.** Isotonicity uses `rational_add_mono` and
  `rational_mul_mono`; the hull and intersect laws use the §2.3 min/max
  proofs, all from `Data.Numeric.Rational`. The D0 package delta is
  `local/arch-interval-d0-package.diff.txt` (primary checkout). It adds
  ten Transparent declarations and no import, checked with no Axiom or
  Opaque, `trusted_base()` 114 before and after, and the ambient census
  row unchanged at `[And, and_fst, and_intro, and_snd]`.
- **Seed.** `conformance/stdlib/numerical/seed-interval-inclusion.md`,
  from SPEC-NUMERICAL-INCLUSION-ISOTONICITY AC-2.
- **Package.** `catalog/packages/Algorithm/Numeric/RationalInterval.ken.md`.
  The §1 and §2 rows are tested by
  `crates/ken-elaborator/tests/cat_numeric_rational_interval.rs` and
  `cat_numeric_interval_hull.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

`interval_inside` and the nine laws in `RationalInterval.ken.md`, the
spec 68 §3 status lines updated to say the exports are delivered, and a
new `crates/ken-elaborator/tests/cat_numeric_interval_inclusion.rs`
that root-loads each new public name and runs the inclusion seed.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass. The test root-loads
  `interval_inside` and the nine laws as package-owned and transparent,
  and runs the seed's positive cases. Each of its two refused blocks is
  pinned to the `interval_inside` proposition between the seed's named
  wide interval and its strictly smaller target, so an unrelated false
  claim does not satisfy it. The §1 and §2 suites still pass. The
  status lines change with the exports (Steward `evt_5rtkgkdf42qb`),
  and the Spec leader reviews those spec/ paths.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom or
  primitive. `lang_mod_strict_resolution_d0`'s
  `catalog_ambient_passthrough_migration_census` row for the package is
  unchanged, measured on the candidate.
- **AC-3 (mutation, QA).** Each mutant fails, and is restored:
  - swapping the `rational_leq` operands of the lower comparison in
    `interval_inside` fails `interval_inside_sound` or the lower-only
    refusal;
  - dropping the upper comparison fails `interval_inside_sound` or the
    upper-only refusal;
  - removing `interval_inside_complete`'s witness premise fails to
    check.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, a signed carrier,
  or a public type other than §3's.
- A census row, or a consumer of `RationalInterval` outside this
  package, changes.
