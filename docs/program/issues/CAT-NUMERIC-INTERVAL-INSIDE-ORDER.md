---
id: CAT-NUMERIC-INTERVAL-INSIDE-ORDER
title: "Algorithm.Numeric.RationalInterval exports spec 68 section 5: interval_inside_trans and the four operand-swap inclusions, proved at zero trust from the Architect's D0, with the section 5 seed run by a package test"
status: merged
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-NUMERICAL-INTERVAL-INSIDE-ORDER]
blocks: []
github: null
origin: "Catalog half of SPEC-NUMERICAL-INTERVAL-INSIDE-ORDER (D0 evt_1r48zppc7y8q1, spec merged 66eec2457). Same shape as CAT-GRAPHS-WALK-APPEND-UNIT-ASSOC. Steward-filed per COORDINATION section 2."
---

# RationalInterval exports the inclusion-order laws

## Objective

`Algorithm.Numeric.RationalInterval` exports `interval_inside_trans`,
`interval_add_comm_inside`, `interval_mul_comm_inside`,
`interval_hull_comm_inside` and `interval_intersect_comm_inside`, with
exactly spec 68 §5's public types, each a checked proof.

## Settled inputs (spec 68 §5 and its seed, main `a4958ae83`)

- **The surface.** §5 states the five laws by type, with no validity or
  membership premise. The swaps are inclusions of endpoint values, not
  `Equal Interval`.
- **The D0.** The Architect's checked D0 (`evt_1r48zppc7y8q1`, on
  `d6c05223e`) is saved as
  `local/architect-shared/interval-inside-order-d0/interval-inside-order-d0.patch`
  (sha256 `1e84fec6…`). It has two hunks in `RationalInterval.ken.md`:
  after `interval_inside_refl`, and after `interval_factor_r_inside`.
  `trusted_base()` is 114 before and after, with no `Axiom`.
- **No new helpers.** The private `equiv_leq` and `rational_distrib_r`
  stay private and unchanged. No Rational declaration, helper or import
  is added.
- **The consumers.** The package prose counts its checked laws
  (`RationalInterval.ken.md:35-37` and the §4 law list). The export
  checks in `cat_numeric_interval_inclusion.rs` and
  `cat_numeric_interval_distributivity.rs` are the nearest pins. The
  implementer sweeps for any other export-set pin before editing.
- **Seed.** `conformance/stdlib/numerical/seed-interval-inside-order.md`
  is on `main` and has not been run against shipped exports.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The five exports in `RationalInterval.ken.md` with its law count
corrected, and a new
`crates/ken-elaborator/tests/cat_numeric_interval_inside_order.rs` that
root-loads all five and runs the seed. In the same candidate, update the
spec 68 §5 status lines and the seed's execution-status statements to
say the package test checks them. Make no conformance-runner, CI or
landing claim.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass. The test root-loads
  the five names as package-owned and transparent, and runs the seed's
  accepted client. Its refused block is pinned to the seed's
  reversed-chain proposition, and the same-input control checks. The
  three existing `cat_numeric_interval_*` binaries still pass.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom or
  primitive. The census row for `Algorithm.Numeric.RationalInterval` is
  unchanged, measured on the candidate.
- **AC-3 (mutation, QA).** Each mutant fails and is restored:
  - `interval_inside_trans` with its two premises swapped fails check or
    the seed's chain application;
  - the refused block's chain put back in the true order fails the pin.

## Stop conditions

- A law needs an Axiom, a helper, a new import, a kernel or surface
  change, or a public type other than §5's.
- A change to an existing operation or law, or a census row change.

Checks: 4 fired: the §5 types and the D0's base, trust count and hunk
placement are read from spec 68 on `a4958ae83` and from the patch. The
probe log ends `2 passed`; its `rc.txt` 101 is from an earlier run. 8
fired: the second AC-3 mutant turns the refusal into an accepted chain,
so a pin that only checks that some block was refused stays green on it.
