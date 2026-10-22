---
id: SPEC-NUMERICAL-INTERVAL-DISTRIBUTIVITY
title: "Spec 68 defers sub-distributivity: interval_mul and interval_add have enclosure laws but nothing relates i*(j+k) to i*j+i*k. Over non-negative rationals the endpoint products may make it an equality. Rule the law from a checked D0 at zero trust, and state it as a section 4 with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 68 section 1.4 lists sub-distributivity as outside sections 1 to 3. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md). Same shape as SPEC-LINEAR-ALGEBRA-MATRIX-SUM-DISTRIBUTIVITY. Steward-filed per COORDINATION section 2."
---

# Interval distributivity

## Objective

Spec 68 states how `interval_mul` distributes over `interval_add` in
the exact rational interval package, on each side, as a checked law:
an equality if the non-negative carrier gives one, otherwise the
inclusion through §3's `interval_inside`.

## Settled inputs (spec 68 §1 to §3, on `main` at `cb48b8ddb`)

- **Operations (§1.2).** `interval_add`, `interval_mul` and
  `interval_point` over intervals with non-negative rational endpoints.
  §1.4 lists sub-distributivity as outside §§1–3.
- **Inclusion (§3).** `interval_inside i k` and its isotonicity laws.
- **Rational (spec 64).** `distrib_l for rational_mul` is a public
  checked proof; right distributivity follows from commutativity.
- **Package.** `catalog/packages/Algorithm/Numeric/RationalInterval.ken.md`,
  tested by `crates/ken-elaborator/tests/cat_numeric_rational_interval.rs`,
  `cat_numeric_interval_hull.rs` and `cat_numeric_interval_inclusion.rs`,
  with a census row in `lang_mod_strict_resolution_d0.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

A new spec 68 section stating each ruled law by its public type, and a
closed conformance seed under `conformance/stdlib/numerical/`, with
the status lines changed in the same candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0 and no `Axiom`. It rules:
  - whether `interval_mul i (interval_add j k)` equals
    `interval_add (interval_mul i j) (interval_mul i k)` as an
    `Interval`, or only lies inside it;
  - the right-hand orientation, and whether it follows from the left
    law and a commutativity the package already has;
  - the private helpers the proofs need, reusing spec 64's
    distributivity rather than restating it.
- **AC-1.** The section states each ruled law by its public type. The
  Spec leader and CV review it, with a Decision.
- **AC-2.** The seed checks the law on closed intervals with distinct
  endpoints. It has a refused block that drops one product from the
  right-hand side, beside a same-input accepting control.

## Stop conditions

- A law that needs signed endpoints, subtraction, division or a new
  carrier.
- Any new primitive, kernel or surface feature, or a change to a §1 to
  §3 operation or law, or to the Rational package's public surface.
- An ambient name in the package beyond its current census row: import
  it explicitly.

Checks: 4 fired: §1.4 of spec 68 lists sub-distributivity as outside
§§1–3, `distrib_l for rational_mul` is public in spec 64, and the
three interval suites exist. 7 fired: the refused block drops a
product, which a seed checking only that both sides are intervals
would accept.
