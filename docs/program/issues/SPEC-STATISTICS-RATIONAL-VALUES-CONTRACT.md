---
id: SPEC-STATISTICS-RATIONAL-VALUES-CONTRACT
title: "Spec 63 reads every probability, expectation and empirical mean as a numerator/denominator interpretation, and spec 64 now supplies an exact Rational carrier but adds no value-returning statistics operation, so a probability still cannot be a Ken value. Author the adapter contract: probability, expectation and mean returning Rational, agreeing with spec 63's numerators, each law proved at zero trust"
status: ready
owner: spec
size: M
tier: T1
gate: architect
depends_on: [CAT-NUMERIC-RATIONAL-ORDER]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\". spec/50-stdlib/64-rational.md section 5 names this as the carrier's next consumer: value-typed probability, expectation and mean need their own checked CAT contract and conformance, including any bridge to spec 63's numerators. 06-catalog-campaign.md:375 orders the statistics tiers next. Same shape as SPEC-NUMERIC-RATIONAL-CONTRACT. Steward-filed per COORDINATION section 2."
---

# Statistics values as Rationals

## Objective

A normative contract that lets a probability, an expectation or an
empirical mean be returned as a `Rational` value, agreeing exactly with
the numerator/denominator readings spec 63 already proves.

## Settled inputs (on `96d8f0112`)

- **Spec 63 section 1.1.** The probability of an event is the pair
  `event_mass q d event` over `total q d`, and an expectation is
  `expectation_weight q d score` over `total q d`. The denominator is
  positive by the constructor's checked proof. These are interpretations,
  not exported values (sections 1.4 and 2.3 defer the adapters).
- **Spec 63 section 2.** The empirical mean is `score_sum q sample score`
  over `sample_size q sample`, with `sample_size` positive on a
  `NonEmpty` sample.
- **Spec 64.** `Rational` is `MkRational num den_tail`, read as
  `num/(Suc den_tail)`. It has `rational_equiv`, add, mul and the order,
  all proved, delivered by `CAT-NUMERIC-RATIONAL-VALUES` and
  `CAT-NUMERIC-RATIONAL-ORDER`.
- **The seam the D0 must settle.** A positive `Nat` must become the
  `den_tail` of a `Suc`, by a proof or a total predecessor. The answer has
  to agree with spec 63's cross-multiplication equality.
- **Still outside:** `pure` and `bind`, `uniform`, conditional
  probability, variance, signed values, division, normalization and
  `Float`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (Architect boundary ruling, measured).** Rule from a checked
   development (rc=0, no Axiom) on:
   - the module path, and whether the adapter lives in each statistics
     package or in a new one;
   - the operations, at least `probability`, `expectation` and
     `empirical_mean`, and how each forms `den_tail` from its positive
     total;
   - the bridge laws: each value is `rational_equiv` to the spec 63 pair
     under cross-multiplication, and complement and bounds laws such as
     probability at most one, if ruled in;
   - what stays deferred.
2. **A new spec section, normative** for every ruled declaration and law,
   each a proved theorem with the trust boundary of spec 63 section 1.
   Point spec 63 sections 1.4 and 2.3 and spec 64 section 5 at it, and
   update `spec/SPEC-PROGRESS.md`.
3. **A conformance seed** under `conformance/stdlib/` in the existing seed
   shape: an event probability, an expectation and an empirical mean, each
   agreeing with its spec 63 pair. Include one non-equivalent pair.

## Acceptance

- **AC-1.** Every declaration and law the CAT node must deliver is named
  with its type, and nothing required is outside the Architect's ruling.
- **AC-2.** The seed distinguishes value from spelling. A returned value
  and its spec 63 pair are equivalent by a closed theorem, and one
  inequivalent pair is not.
- **AC-3.** Conformance-validator vote on the exact SHA of the assembled
  section and seed, then Architect review.

## Stop conditions

- A law needs an Axiom, a kernel or surface-syntax change, a primitive or
  `Float`.
- The adapter needs a public-surface change to a statistics or Rational
  package beyond adding the new names: stop with the measurement.
