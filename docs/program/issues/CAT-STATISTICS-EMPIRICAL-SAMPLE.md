---
id: CAT-STATISTICS-EMPIRICAL-SAMPLE
title: "No catalog package reads a non-empty sample as an exact distribution with counts and sums. Deliver section 2 of spec/50-stdlib/63-statistics.md, the empirical tier over NonEmpty q, five functions and seven laws, fully proved at zero trust"
status: ready
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-STATISTICS-EMPIRICAL-SAMPLE-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. 06-catalog-campaign.md:375, the statistics layer's empirical tier after the exact tier. Spec section 2 from SPEC-STATISTICS-EMPIRICAL-SAMPLE-CONTRACT (Architect D0 evt_6xr53agbj43fr, review evt_1bsh3kwmz8gys). Steward-filed per COORDINATION section 2."
---

# Empirical samples, delivered and proved

## Objective

The empirical tier lands as section 2 of
`spec/50-stdlib/63-statistics.md` specifies, with every law proved and no
new trust.

## Settled inputs (spec at `000747633`)

- **Section 2 is normative.** Carrier `NonEmpty q`, observations
  `nonempty_to_list`, append `nonempty_append`. Five functions:
  `sample_size`, `event_count`, `score_sum`, `unit_weights`,
  `empirical_distribution`. Seven laws: `empirical_entries`,
  `empirical_total_size`, `empirical_event_count`, `empirical_score_sum`,
  `sample_size_append`, `event_count_append`, `score_sum_append`, in
  `Algorithm.Statistics.Empirical`, importing `Algorithm.Statistics.Exact`.
- **The development exists.** The spec author's scratch `Empirical.ken`
  (sha256 prefix `6fe07c50`) checks rc=0 with no Axiom against the landed
  `Algorithm.Statistics.Exact` and `Data.Collections.NonEmpty`, and a
  separate consumer checks 21 closed claims and 7 generic law clients. It
  is the starting point, not a design to revisit.
- **Provider laws.** `nonempty_to_list_shape` and `nonempty_append`'s
  `list_view` (`e49cd5b95`) are the two transports the proofs use.
- **Reuse (Architect carry, `evt_1bsh3kwmz8gys`).** Define `unit_weights
  q xs = map q (Pair q Nat) (weighted_one q) xs` with
  `Data.Collections.Derived` `map`, not hand recursion, and check the
  event count against `filter` with `length`. `sum_list` has no catalog
  counterpart.
- **The seed** is `conformance/stdlib/statistics/seed-empirical-sample.md`:
  five cases. Seed results are closed `Equal` theorems checked by kernel
  reduction; Unknown is never a pass.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The package, in section 2's order, reading top-down, with its own
acceptance test target and a strict-resolution ambient-census row.

## Acceptance

- **AC-1.** `ken fmt --check` and `ken check` pass. The acceptance
  target root-loads all public names and runs the five seed cases with
  their listed results.
- **AC-2.** `trusted_base()` is equal before and after. No Axiom,
  postulate, primitive, foreign declaration or `Float`.
- **AC-3 (consumers).** No existing package's public surface changes;
  the importers of `Exact`, `NonEmpty` and `Derived` stay green, and the
  Rosetta runner and SEAL-2 pass.
- **AC-4 (mutation, QA).** Each mutant makes a proof or seed oracle
  kernel-reject, and is restored: deduplicating repeated observations; a
  zero unit weight; an append that drops its right sample.

## Stop conditions

- A law needs an Axiom, `Finite q`, `DecEq q`, a rational type, `Float`,
  or a kernel or surface-syntax change.
- Reuse needs a public-surface change to `Exact`, `NonEmpty` or
  `Derived`.
