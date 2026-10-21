---
id: CAT-GEOMETRY-BOX-AREA
title: "Algorithm.Geometry.NatBox exports spec 67 section 4: the natural-valued area of a box and its three laws (area_inside, area_inverted_x, area_inverted_y), proved at zero trust from the Architect's D0 development, with the section 4 seed run by a package test"
status: ready
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-GEOMETRY-BOX-AREA]
blocks: []
github: null
origin: "Operator 2026-09-13: every catalog package fully proved. SPEC-GEOMETRY-BOX-AREA states section 4 (5efe44c95) from the Architect's checked D0 scratch development, not yet exported. Same shape as CAT-OPTIMIZATION-ARGMIN-OPTION. Steward-filed per COORDINATION section 2."
---

# NatBox exports box area

## Objective

`Algorithm.Geometry.NatBox` exports `area`, `area_inside`,
`area_inverted_x` and `area_inverted_y` with exactly spec 67 §4's
public types, each law a checked proof.

## Settled inputs (spec 67 §4, on `main`; Architect D0)

- **The surface.** `area` is the product of two inclusive spans,
  `axis_span lo hi = sub (Suc hi) lo`, with `sub` saturating. An
  inverted axis gives area zero. The intersection bounds are
  corollaries of `area_inside` and §3.2, not new public names (§4.2).
- **The D0.** Scratch commit `2645b9830`, on base `0fd75f551`:
  `catalog/packages/Algorithm/Geometry/NatBoxAreaD0.ken.md`, with private
  helpers for `sub` and `mul` monotonicity and the span lemmas, and
  `spec_geometry_area_d0_trust.rs`. Checked rc 0, trust identities
  unchanged (`/workspaces/ken/local/arch-area-d0/`).
- **Seed.** The §4 seed under `conformance/stdlib/geometry/`, from
  SPEC-GEOMETRY-BOX-AREA AC-2. It must be on `main` before this WP
  starts.
- **Package.** `catalog/packages/Algorithm/Geometry/NatBox.ken.md`.
  §1 to §3 are tested by
  `crates/ken-elaborator/tests/cat_geometry_nat_box.rs` and
  `cat_geometry_manhattan_metric.rs`. The package prose near line 1295
  says area remains outside the module (Spec leader
  `evt_1q9caz2wtbx0r`).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The four exports and their private helpers in `NatBox.ken.md`, with the
package prose that defers area corrected; the spec 67 §4 status lines
updated to say the exports are delivered; and a new
`crates/ken-elaborator/tests/cat_geometry_box_area.rs` that root-loads
each new public name and runs the seed. Every seed execution-status
statement changes in the same candidate to say the package test
executes it, with no conformance-runner, CI or landing claim, and no
fence or fixture change.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass. The test root-loads
  the four names as package-owned and transparent, keeps the helpers
  private, and runs the seed's positive cases. Its refused block is
  pinned to the seed's strictly-larger-inner-area proposition. The §1
  to §3 suites still pass. The Spec leader reviews the spec/ status
  lines, and CV the seed's.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom or
  primitive. The census row for `Algorithm.Geometry.NatBox` in
  `lang_mod_strict_resolution_d0.rs` is unchanged, measured on the
  candidate.
- **AC-3 (mutation, QA).** Each mutant fails, and is restored:
  - `axis_span lo hi = sub hi lo` (an exclusive span) fails the seed's
    point-box case;
  - `area` as the sum of the spans fails the seed's proper-box case;
  - an unrelated false closed proposition in the refused block fails
    the pin.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a public type
  other than §4's.
- A new public `Nat` lemma, or any change to `Data.Numeric.Nat`: keep
  the helpers private in `NatBox`, or import an existing public lemma.
- A change to a §1 to §3 operation or law, a census row change, or a
  change to a consumer of `NatBox` outside this package.

Checks: 4 fired: `2645b9830`, `local/arch-area-d0/` and both geometry
suites exist, and §4 is on the approved candidate `5efe44c95`. 7 fired:
AC-3 mutates the span and the product, which a weak seed would miss.
