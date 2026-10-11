---
id: SPEC-NUMERIC-RATIONAL-CONTRACT
title: "spec/50-stdlib/63-statistics.md defers every value-typed probability, mean and the approximate tier to a Data.Numeric.Rational contract that does not exist, so the statistics layer has nothing further to build against. Author that contract: an exact non-negative rational carrier, its arithmetic and equivalence, each law proved at zero trust"
status: merged
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\". 06-catalog-campaign.md:375 orders the statistics tiers exact, empirical, approximate; spec 63 sections 1.4 and 2.3 defer a value-level rational carrier, its arithmetic and normalization, and value-typed probability and expectation to a separate Data.Numeric.Rational contract. Same shape as SPEC-STATISTICS-EMPIRICAL-SAMPLE-CONTRACT: an Architect boundary ruling from a checked development, then the contract, then the CAT node. Steward-filed per COORDINATION section 2."
---

# An exact rational contract

## Objective

A normative `Data.Numeric.Rational` contract in `spec/50-stdlib/` that the
next catalog node delivers, so a probability or mean can be a Ken value
rather than a numerator/denominator interpretation.

## Settled inputs (on `4a46c6163`)

- **Spec 63 sections 1.4 and 2.3** defer, explicitly to a
  `Data.Numeric.Rational` contract: a value-level rational carrier, its
  arithmetic and normalization, and value-typed probability and
  expectation. `score_sum` over `sample_size` is today an exact
  numerator/denominator reading, not a Ken division.
- **Natural arithmetic exists:** `Data.Numeric.Nat.Arithmetic` exports
  `add`, `mul` and `mul_add_distrib_r`; `Data.Numeric.Nat.Order` carries
  the order.
- **A verified gcd exists but is not public:** `Algorithm.Numeric.Gcd`
  proves `gcd_spec`, `gcd_spec_greatest` and `divides_gcd`, and declares no
  `pub` name. Using it from another package is a public-surface change,
  which the ruling must name.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (Architect boundary ruling, measured).** Rule from a checked
   development (rc=0, no Axiom) on:
   - the module path, and the carrier: a normalized pair (coprime, with a
     positive denominator) or a pair with a positive denominator under
     cross-multiplication equivalence;
   - the operations: at least construction from naturals, `add`, `mul`,
     the equivalence or equality, and an order, each with its laws
     (commutativity, associativity, identities, distributivity, and the
     respect laws if the carrier is not normalized);
   - whether `Algorithm.Numeric.Gcd` gains a public surface, and which
     names;
   - what stays deferred: signed rationals, subtraction and division if
     not ruled in, `Float`, and the statistics re-statements that consume
     the carrier.
2. **A new spec section, normative** for every ruled declaration and law,
   each a proved theorem with the trust boundary of spec 63 section 1.
   Point spec 63's deferred lists at it and update
   `spec/SPEC-PROGRESS.md`.
3. **A conformance seed** under `conformance/stdlib/` in the existing seed
   shape: two equivalent spellings of one value, a sum, a product, an
   order comparison, and the zero and one identities.

## Acceptance

- **AC-1.** Every declaration and law the CAT node must deliver is named
  with its type, and nothing required is outside the Architect's ruling.
- **AC-2.** The seed distinguishes value from spelling: two equivalent
  spellings are equal (or equivalent) by a closed theorem, and one
  inequivalent pair is not.
- **AC-3.** Conformance-validator vote on the exact SHA of the assembled
  section and seed, then Architect review.

## Stop conditions

- A law needs an Axiom, a kernel or surface-syntax change, a primitive or
  `Float`.
- The ruled carrier needs a quotient type or setoid machinery the catalog
  does not deliver: stop with the measurement.

## Closeout

Merged `2263461c4` from exact `1a944dd98` (PR #4673). Conformance
validator `evt_7jk8rj2zt32t6`, Architect `evt_5k6fz5fqxnqgm`, Decision
`dec_67zmm64jbp7dg`. `spec/50-stdlib/64-rational.md` specifies
`Data.Numeric.Rational`: a positive-denominator carrier with no
normalization, value equality by cross-product equivalence, total
arithmetic, exact order, and their laws. Section 4 names the Nat
prerequisites, and the seed is `conformance/stdlib/numeric/seed-rational.md`.
The development checks feasibility for a subset only; the CAT nodes
deliver every law. §1a: 0.
