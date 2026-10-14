---
id: CAT-NUMERIC-RATIONAL-INTERVAL
title: "Spec 68 section 1 specifies Algorithm.Numeric.RationalInterval with six proved laws, but no catalog package delivers it and Data.Numeric.Rational lacks the two monotonicity laws it needs. Publish those laws, deliver the package from the Architect's checked D0 at zero trust, and run its seed as an acceptance target"
status: ready
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-NUMERICAL-RATIONAL-INTERVAL-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 68 section 1 from SPEC-NUMERICAL-RATIONAL-INTERVAL-CONTRACT (Architect D0 evt_5b9ngzfc62h0j, review evt_7dzj7kdk9fgq8). Same shape as CAT-GEOMETRY-NAT-BOX. Steward-filed per COORDINATION section 2."
---

# Exact intervals over rationals, delivered and proved

## Objective

`Algorithm.Numeric.RationalInterval` exists in `catalog/`, matches spec
68 section 1 token for token, and proves all six laws with no Axiom. A
client outside the module can build intervals and use every operation.

## Settled inputs (on `7b57754a4`)

- **The contract.** Spec 68 section 1 fixes:
  - the carrier `Interval` (two `Rational` endpoints, no order-proof
    field, `export MkInterval`) and `interval_contains`;
  - the point, add and multiply operations (section 1.2);
  - the six laws `interval_add_encloses`, `interval_mul_encloses`,
    `interval_point_contains`, `interval_point_sound`,
    `interval_add_point` and `interval_mul_point` (section 1.3).
- **A checked development exists.** The Architect's D0 checks all six
  laws generically, rc=0, no Axiom, against a scratch Rational with two
  added monotonicity laws.
- **Rational must gain two public laws** (section 1.4).
  `Data.Numeric.Rational` exports neither `rational_add_mono` nor
  `rational_mul_mono`; against the landed package the D0 fails
  `UnboundName rational_add_mono`. Add both as public theorems with their
  private Nat-derived helpers, signatures as in section 1.4.
- **Coupled tests.** `crates/ken-elaborator/tests/cat_numeric_rational_values.rs`
  keeps a must-resolve list for Rational's public names, and both new
  laws join it. `catalog_ambient_passthrough_migration_census` in
  `lang_mod_strict_resolution_d0.rs` needs the new package's measured
  row (CAT-SYMBOLIC's first candidate went red on CI for lacking one).
- **The seed** is `conformance/stdlib/numerical/seed-rational-interval.md`:
  24 positive assertions and the five refusals B1-B5.
- **Known limits.** Pass `map` a named helper, not a λ. A dependent
  `match` refines only its scrutinee as written (CHECKS 11).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. `rational_add_mono` and `rational_mul_mono` as public theorems in
   `Data.Numeric.Rational`, with private helpers, and both added to the
   must-resolve list in `cat_numeric_rational_values.rs`.
2. The package at `catalog/packages/Algorithm/Numeric/RationalInterval.ken.md`
   with the D0's declarations and proof bodies, `export MkInterval`, and
   the six laws as public theorems, lede first.
3. An acceptance target in `crates/ken-elaborator/tests/` that loads the
   package, runs the 24 positives as a client import, refuses B1-B5,
   restates the six law types generically, and records `trusted_base()`
   before and after loading.
4. The package's measured row in the WP-4 ambient census.

## Acceptance

- **AC-1.** The package and the extended Rational check, every law is a
  checked proof body, and `trusted_base()` does not grow on loading
  either.
- **AC-2.** The 24 positives pass from a client module and B1-B5 are
  refused. The census passes with the new row and every other row
  unchanged.
- **AC-3 (mutation, QA).**
  - Swapping the upper endpoint of `interval_add` to `lo` reddens
    `interval_add_encloses` or the seed's B1 control.
  - Removing `export MkInterval` reddens the client import.

## Stop conditions

- A law needs an Axiom, or Rational's existing public laws change.
- `trusted_base()` grows on loading either package.
- A change outside the package, Rational, the two named tests and the new
  acceptance target.
