---
id: CAT-STATISTICS-RATIONAL-VALUES
title: "Spec 63 section 3 makes probability, expectation and empirical mean Rational values, but no statistics package exports them. Deliver the 15 ruled names in Algorithm.Statistics.Exact and Empirical, every law proved at zero trust"
status: active
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-STATISTICS-RATIONAL-VALUES-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 63 section 3 from SPEC-STATISTICS-RATIONAL-VALUES-CONTRACT (Architect D0 evt_1m3dbd68wnrcn, review evt_5175kmt2m2k68). Adversary evt_79d84krb25dj7 for the stale Rational row. Steward-filed per COORDINATION section 2."
---

# Statistics values as Rationals, delivered and proved

## Objective

`probability`, `expectation` and `empirical_mean` return `Rational`
values, as spec 63 section 3 specifies, with every law proved and no new
trust.

## Settled inputs (D0 `evt_1m3dbd68wnrcn`, on `29fe3a597`)

- **Placement.** `Algorithm.Statistics.Exact` gets `probability`,
  `expectation` and nine laws. `Algorithm.Statistics.Empirical` gets
  `empirical_mean`, defined through Exact's `expectation`, and three
  laws. There is no new package and no change to an existing declaration.
- **The 15 names and their types** are listed verbatim in the D0 and in
  spec 63 section 3. Each `den_tail` is the existing `one_less_total`,
  bridged by `total_positive`. Bridges are forward only. Agreement is
  `rational_equiv`, never `Equal Rational`.
- **Imports** are exactly the lists the D0 names.
- **The development exists.** The spec-author's scratch fragments check
  rc=0 with no Axiom. `probability_leq_one` uses the D0's inlined body,
  with no new private `leq_nat` congruence helper.
- **The seed** is `conformance/stdlib/statistics/seed-rational-values.md`:
  `1/3 ~ 2/6`, expectation `4/3`, empirical mean `5/3`, and the closed
  contradiction `1/3 ≁ 1/2`.
- **Carry (Adversary `evt_79d84krb25dj7`).** The private-import negative
  row at `cat_numeric_rational_values.rs:251` names
  `bool_or_false_right`, which the ORDER respin deleted. It passes
  without testing anything: point it at a private helper that exists.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The 15 declarations in section 3's order, reading top-down, with seed
rows in the two existing statistics acceptance targets.

## Acceptance

- **AC-1.** `ken fmt --check` and `ken check` pass. The targets
  `cat_statistics_exact_distribution` and
  `cat_statistics_empirical_sample` root-load all 15 names and run the
  four seed cases with their listed results.
- **AC-2.** `trusted_base()` is equal before and after. No Axiom,
  postulate, primitive or `Float`. The strict-resolution census rows for
  Exact and Empirical are unchanged.
- **AC-3.** The carried Rational row names an existing private helper and
  still fails with `UnboundName`.
- **AC-4 (mutation, QA).** Each mutant makes a proof or a seed oracle
  kernel-reject, and is restored:
  - `probability` built with `total` as its tail;
  - `probability_complement_one` stated over `event` twice;
  - `empirical_mean` over `sample_size` as its tail.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, a primitive or
  `Float`.
- Reuse needs a public-surface change to Rational, Nat.Order or an
  existing statistics declaration, beyond the 15 new names. Exporting
  one shared `leq_nat_cong_two` is not this WP.
