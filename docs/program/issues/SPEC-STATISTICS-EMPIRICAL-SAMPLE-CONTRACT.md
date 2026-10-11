---
id: SPEC-STATISTICS-EMPIRICAL-SAMPLE-CONTRACT
title: "spec/50-stdlib/63-statistics.md stops at the exact tier, so the catalog's empirical statistics tier has nothing to build against. Author section 2: a non-empty sample of observations, its empirical distribution, and its counts and sums, each law proved at zero trust"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-STATISTICS-EXACT-DISTRIBUTION-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\". 06-catalog-campaign.md:375 names the statistics layer's exact, empirical and approximate tiers in that order; spec 63 section 1.4 defers the empirical tier. Same shape as SPEC-STATISTICS-EXACT-DISTRIBUTION-CONTRACT: an Architect boundary ruling from a checked development, then the contract, then the CAT node. Steward-filed per COORDINATION section 2."
---

# An empirical sample contract

## Objective

Section 2 of `spec/50-stdlib/63-statistics.md` is the normative contract
for the empirical tier: the observations of a sample, read as an exact
distribution, with counts and sums the next catalog node delivers.

## Settled inputs (on `8a97906bc`)

- **Section 1 is normative** (`9355e98c8`): `ExactDistribution q`, `total`,
  `event_mass`, `expectation_weight`, five builders and nine laws.
  `CAT-STATISTICS-EXACT-DISTRIBUTION` delivers it in
  `Algorithm.Statistics.Exact` (exact candidate `921969a7c` at QA).
- **A non-empty list exists:** `Data.Collections.NonEmpty` exports
  `NonEmpty a`, `nonempty_to_list` and `nonempty_append`, and
  `Data.Collections.List` exports `length`.
- **Section 1.4 defers** the empirical and approximate tiers, `uniform`,
  `pure`/`bind`, conditional probability, variance, `Float` and a
  rational carrier.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (Architect boundary ruling, measured).** Rule from a checked
   development (rc=0, no Axiom) on:
   - the module path, and whether the tier extends
     `Algorithm.Statistics.Exact` or is a sibling module importing it;
   - the sample carrier (a `NonEmpty q` is the default) and the empirical
     distribution built from it, one unit weight per observation, so that
     its `total` is the sample size by a checked proof;
   - the operations and their laws: at least sample size, the count of
     observations satisfying a Boolean event, and the sum of a
     natural-valued score, each equal to the section 1 quantity on the
     empirical distribution; and how appending two samples adds sizes,
     counts and sums;
   - what stays deferred: the approximate tier, means as values,
     variance, sampling from a distribution and any `Float` result.
2. **Section 2, normative** for every ruled declaration and law, each
   stated as a proved theorem, with the same trust boundary as section 1.
   Update section 1.4's deferred list and `spec/SPEC-PROGRESS.md`.
3. **A conformance seed** under `conformance/stdlib/statistics/` in the
   existing seed shape: a one-observation sample, a sample with repeated
   observations, an event no observation satisfies, an appended pair,
   and one score sum.

## Acceptance

- **AC-1.** Every declaration and law the CAT node must deliver is named
  with its type, and nothing required is outside the Architect's ruling.
- **AC-2.** Repeated observations count separately: the seed's
  repeated-observation case has a count above one for a single value.
- **AC-3.** Conformance-validator vote on the exact SHA of the assembled
  scope: the spec paths plus the seed.

## Stop conditions

- The contract needs a kernel, trust, `Float` or surface-syntax change.
- A law cannot be stated without a type that does not exist and that the
  D0 does not place in this scope.
- The tier needs a section 1 declaration changed rather than reused.
