---
id: CAT-NUMERIC-INTERVAL-DISTRIBUTIVITY
title: "Algorithm.Numeric.RationalInterval exports spec 68 section 4: the four interval distributivity inclusions (left and right, both directions), proved at zero trust from the Architect's D0 development, with the section 4 seed run by a package test"
status: ready
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-NUMERICAL-INTERVAL-DISTRIBUTIVITY]
blocks: []
github: null
origin: "Operator 2026-09-13: every catalog package fully proved. SPEC-NUMERICAL-INTERVAL-DISTRIBUTIVITY states section 4 from the Architect's checked D0 scratch development, not yet exported. Same shape as CAT-GRAPHS-WALK-COMPOSITION-ORDER. Steward-filed per COORDINATION section 2."
---

# RationalInterval exports distributivity

## Objective

`Algorithm.Numeric.RationalInterval` exports
`interval_distrib_l_inside`, `interval_factor_l_inside`,
`interval_distrib_r_inside` and `interval_factor_r_inside` with exactly
spec 68 §4's public types, each a checked proof.

## Settled inputs (spec 68 §4; Architect D0)

- **The surface.** The four laws state both `interval_inside` directions
  between `interval_mul i (interval_add j k)` and `interval_add
  (interval_mul i j) (interval_mul i k)`, and the right-hand pair, with
  no premise. No `Equal Interval`, normalization or interval
  commutativity law is added.
- **Private stages.** `equiv_leq` and `rational_distrib_r` stay
  private. The right laws use spec 64's `comm for rational_mul` and
  `distrib_l for rational_mul`, never an interval commutativity.
- **The D0.** `/workspaces/ken/local/arch-sid-d0/IntervalD0Probe.ken.md`
  on base `826a4fed0`, with its control and refusal probes and logs:
  rc 0, no `Axiom`, no open obligation.
- **Seed.** The §4 seed under `conformance/stdlib/numerical/`, from
  SPEC-NUMERICAL-INTERVAL-DISTRIBUTIVITY AC-2. It must be on `main`
  before this WP starts.
- **Package.** `catalog/packages/Algorithm/Numeric/RationalInterval.ken.md`,
  tested by `cat_numeric_rational_interval.rs`,
  `cat_numeric_interval_hull.rs` and `cat_numeric_interval_inclusion.rs`
  under `crates/ken-elaborator/tests/`, with a census row in
  `lang_mod_strict_resolution_d0.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The four exports and two private stages in `RationalInterval.ken.md`;
the spec 68 §4 status lines updated to say the exports are delivered;
and a new `crates/ken-elaborator/tests/cat_numeric_interval_distributivity.rs`
that root-loads the four names and runs the seed. If an existing package
test pins the exact public-name set, it gains exactly these four names
and nothing else. Every seed execution-status statement changes in the
same candidate to say the package test executes it, with no
conformance-runner, CI or landing claim, and no fence or fixture change.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass. The test root-loads
  the four names as package-owned and transparent, keeps the two stages
  private, and runs the seed's positive cases. Its refused block is
  pinned to the seed's dropped-product proposition, and the same-input
  control checks. The §1 to §3 suites still pass. The Spec leader
  reviews the spec/ status lines, and CV the seed's.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom or
  primitive. The census row for `Algorithm.Numeric.RationalInterval` is
  unchanged, measured on the exact candidate.
- **AC-3 (mutation, QA).** Each mutant fails, and is restored:
  - a law stated with `Equal Interval` in place of `interval_inside`
    fails to check;
  - an unrelated false closed proposition in the refused block fails
    the pin.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a public type
  other than §4's.
- A new public Rational lemma, or any change outside `RationalInterval`
  other than the inventory pin above.
- A change to a §1 to §3 operation or law, or a census row change.

Checks: 4 fired: the D0 probe checks the four §4 bodies at rc 0 on
`826a4fed0`, and the three interval suites exist on `main`. 7 fired:
AC-3's `Equal Interval` mutant is the structural claim the D0 measured
as refused, which an inclusion-only test might not distinguish.
