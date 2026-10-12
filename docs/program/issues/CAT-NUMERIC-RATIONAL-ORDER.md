---
id: CAT-NUMERIC-RATIONAL-ORDER
title: "Data.Numeric.Rational has rational_leq and its respect law but no order laws, so no consumer can sort, bound or compare rationals with a proof. Deliver section 2.2 of spec/50-stdlib/64-rational.md: refl, trans, antisym up to value equivalence, and total, fully proved at zero trust"
status: active
owner: foundation
size: M
tier: T1
gate: architect
depends_on: [CAT-NUMERIC-RATIONAL-VALUES]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. spec/50-stdlib/64-rational.md section 2.2 (SPEC-NUMERIC-RATIONAL-CONTRACT, merged 2263461c4). Third of three CAT nodes, named as the next node by CAT-NUMERIC-RATIONAL-VALUES. Steward-filed per COORDINATION section 2."
---

# Exact rational order, proved

## Objective

`Data.Numeric.Rational` carries the section 2.2 order laws, so
`rational_leq` is a proved total preorder that is antisymmetric up to
`rational_equiv`.

## Settled inputs (spec merged at `2263461c4`)

- **The four laws**, stated over the Boolean relation:
  - `refl`: `Equal Bool (rational_leq x x) True`.
  - `trans`: from `x ≤ y` and `y ≤ z`, `x ≤ z`.
  - `antisym`: from `x ≤ y` and `y ≤ x`, `rational_equiv x y`, never
    `Equal Rational x y`. `1/2` and `2/4` order each way and stay distinct
    pair terms.
  - `total`: from `Equal Bool (rational_leq x y) False`, `y ≤ x`.
- **Base.** `rational_leq` compares cross products by canonical
  `leq_nat`. Its `respect` law lands with `CAT-NUMERIC-RATIONAL-VALUES`.
- **Feasibility.** The spec's D0 checked `refl` and closed unequal
  outcomes only. Generic `trans`, `antisym` and `total` are obligations
  here, not claims.
- **Providers.** The canonical `leq_nat` laws (`refl`, `trans`,
  `antisym`, `total`, used by `Data.Collections.PriorityQueue`) and the
  section 4 laws from `CAT-NAT-RATIONAL-PREREQUISITES`
  (`leq_nat_mul_right_mono`, `leq_nat_mul_right_cancel`, the public `mul`
  laws). Use them; never re-prove or assume them locally.
- **The seed** is `conformance/stdlib/numeric/seed-rational.md`, section
  "Order compares values".

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The four laws in `Data.Numeric.Rational`, in the spec's order after the
section 2.1 laws, with the package's acceptance target extended to
root-load them and run the seed's order cases.

## Acceptance

- **AC-1.** `ken fmt --check` and `ken check` pass. The acceptance target
  root-loads the four laws and runs the seed's order cases with their
  listed results.
- **AC-2.** `trusted_base()` is equal before and after. No Axiom,
  postulate, primitive or `Float`.
- **AC-3 (consumers).** No existing public surface changes. The
  `CAT-NUMERIC-RATIONAL-VALUES` rows and the Nat importers stay green.
- **AC-4 (mutation, QA).** Each of these kernel-rejects, and is restored:
  `trans` with its second premise dropped; `antisym` concluding
  `Equal Rational x y`; `total` concluding `x ≤ y` from its `False`
  premise.

## Stop conditions

- A law needs an Axiom, a quotient or setoid facility, a kernel or surface
  change, or a Nat law outside section 4 and the canonical `leq_nat` laws.
- Reuse needs a public-surface change to a provider.
