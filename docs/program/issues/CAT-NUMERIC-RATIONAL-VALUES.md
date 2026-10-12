---
id: CAT-NUMERIC-RATIONAL-VALUES
title: "No catalog package carries an exact rational value, so value-typed probability and means have nothing to land on. Deliver sections 1, 2 and 2.1 of spec/50-stdlib/64-rational.md as Data.Numeric.Rational: the positive-denominator carrier, equivalence, total arithmetic, respect and the algebra laws, fully proved at zero trust"
status: merged
owner: foundation
size: M
tier: T1
gate: architect
depends_on: [CAT-NAT-RATIONAL-PREREQUISITES]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. spec/50-stdlib/64-rational.md (SPEC-NUMERIC-RATIONAL-CONTRACT, merged 2263461c4). Second of three CAT nodes: section 4 Nat prerequisites first, the section 2.2 order laws after this. Steward-filed per COORDINATION section 2."
---

# Exact rational values, proved

## Objective

`Data.Numeric.Rational` lands as sections 1, 2 and 2.1 of
`spec/50-stdlib/64-rational.md` specify, with every law proved and no new
trust.

## Settled inputs (spec merged at `2263461c4`)

- **Section 1.** `Rational` has the single constructor `MkRational num
  den_tail`, read as `num/(Suc den_tail)`. It comes with `numerator`,
  `denominator`, `rational_zero`, `rational_one`, `rational_of_nat` and
  `rational_equiv` (equality of cross products, an `Equal Nat`
  proposition). The laws are `positive for denominator`, and `refl`, `sym`
  and `trans` for `rational_equiv`. There is no normalization and no
  `Equal Rational` between spellings.
- **Section 2.**
  - `rational_add`, `rational_mul` and `rational_leq` (the cross products
    compared by canonical `leq_nat`).
  - A package-local `positive_product_tail` keeps denominators positive.
  - `denominator_product` for add and for mul, and `respect` for add, mul
    and leq.
- **Section 2.1.** `comm`, `assoc`, `zero_l` and `zero_r` for add; `comm`,
  `assoc`, `one_l`, `one_r` and `zero_l` for mul; `distrib_l`. Every
  equation is up to `rational_equiv`.
- **Feasibility.** The spec's checked development covers the carrier,
  `trans` and add `respect` only. Every other law is an obligation here,
  not a claim.
- **Providers.** The section 4 Nat laws come from
  `CAT-NAT-RATIONAL-PREREQUISITES`. Use them; never re-prove or assume
  them locally. `Algorithm.Numeric.Gcd` stays private.
- **The seed** is `conformance/stdlib/numeric/seed-rational.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The package, reading top-down in the spec's order, with its own acceptance
target running the seed cases and a strict-resolution ambient-census row.
The section 2.2 order laws (`refl`, `trans`, `antisym` and `total` for
`rational_leq`) are the next node.

## Acceptance

- **AC-1.** `ken fmt --check` and `ken check` pass. The acceptance target
  root-loads every public name and runs the seed's cases within this scope
  with their listed results.
- **AC-2.** `trusted_base()` is equal before and after. No Axiom,
  postulate, primitive or `Float`.
- **AC-3 (consumers).** No existing public surface changes, and the Nat
  importers stay green.
- **AC-4 (mutation, QA).** Each of these makes a proof or seed oracle
  kernel-reject, and is restored: `trans` with a dropped premise; add
  `respect` with the second hypothesis weakened; `distrib_l` with its
  sides swapped across a product.

## Stop conditions

- A law needs an Axiom, a quotient or setoid facility, a kernel or surface
  change, or a Nat law outside section 4.
- Reuse needs a public-surface change to a provider.

## Closeout

Merged `557b9daea` from exact `7f86db5cc` (PR #4682). Foundation QA
`evt_1awnzdejggz7n`, Architect `evt_25x1rh9wx6tft`, Decision
`dec_203195shy59b4`.

`Data.Numeric.Rational` carries the section 1, 2 and 2.1 surface with
every law proved, using the section 4 Nat laws from
`CAT-NAT-RATIONAL-PREREQUISITES`. Its private helpers read top-down as
ruled. The acceptance target `cat_numeric_rational_values.rs` runs the
seed cases, and the strict-resolution census gains its row.
`trusted_base()` is unchanged. Three paths, catalog plus crates. The
section 2.2 order laws are `CAT-NUMERIC-RATIONAL-ORDER`.
