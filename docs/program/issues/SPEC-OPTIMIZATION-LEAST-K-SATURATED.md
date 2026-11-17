---
id: SPEC-OPTIMIZATION-LEAST-K-SATURATED
title: "Spec 72 §4 bounds least_k's length by min k (length xs) and relates two bounds by prefix, but no law says what a bound covering the whole input returns, so a client asking for at least as many candidates as it has cannot conclude it gets all of them in stable cost order. Rule from a checked D0 at zero trust that, when length xs ≤ k, least_k cost k xs equals sort (cost_le cost) xs, and state it with a seed that separates sorted order from input order"
status: merged
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Optimization follows Symbolic algebra in the L3 layer rotation (docs/program/06-catalog-campaign.md). least_k has length, prefix and invariance laws but none for a saturating bound. Steward-filed per COORDINATION section 2."
---

# A bound covering the input keeps every candidate, in stable cost order

## Objective

`Algorithm.Optimization.Argmin` exports a public checked law that, for
every `cost`, `k` and `xs`, if `length xs ≤ k` then
`least_k a cost k xs` equals `sort a (cost_le a cost) xs` as
`Equal (List a)`. It holds at zero trust, with that one premise. Spec 72
states it by its public type.

## Settled inputs (main `8cf27bd2c`)

- **The operation.** `least_k` (`Argmin.ken.md:307`) is
  `take a k (sort a (cost_le a cost) xs)`. Its public laws run from
  `least_k_length` (`:310`) to `least_k_prefix` (`:510`), before
  `argmax` (`:524`).
- **The list floor.** `take` (`Data/Collections/Derived.ken.md:130`)
  recurses on `n`, then on `xs`. Its public attached proofs are
  `length`, `member`, `sorted`, `filter_prefix` and `prefix`
  (`:1225` to `:1294`). `(proof length for sort)` (`:1955`) gives
  `length (sort le xs) = length xs`.
- **The gap.** No law in Derived, Argmin or spec 72 says that `take n xs`
  is `xs` when `length xs ≤ n`. `least_k_length` gives only the length of
  the result, and §4.3's `k = 9` row is a closed computation, not a law.
- **Spec and tests.** Spec 72 ends with §4.5
  (`spec/50-stdlib/72-optimization.md:1651`), so the new section is §4.6.
  The seed directory is `conformance/stdlib/optimization/`. The test
  pattern is `crates/ken-elaborator/tests/cat_optimization_least_k_prefix.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 72 §4.6, the catalog law with its `take` saturation step, a seed, a
package test and the status lines. No §1 to §4.5 operation or law type
moves, and no kernel or surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-OPTIMIZATION-LEAST-K-PREFIX.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules:
  - the name and binder order;
  - the premise form (`Equal Bool (leq_nat (length a xs) k) True`, as
    `prefix for take` states its bound, or another);
  - whether the `take` step is a public Derived attached proof, as
    `prefix for take` was, or a private Argmin helper.
- **AC-1.** §4.6 states the law by its public type. The Spec Leader and CV
  review it, with a Decision.
- **AC-2.** The seed's accepted block applies the law on §4.3's input
  `[A, B, C, D]` with costs `2, 1, 2, 3`:
  - at `k = 4`, the bound equal to the length, giving `[B, A, C, D]`;
  - at `k = 9`, a bound past the length, giving the same list.

  Its refused block pins, in the same form as an accepted row and on a
  fresh accepted prefix:
  - `least_k` at `k = 4` claimed `Equal (List a)` to the input
    `[A, B, C, D]`. The law keeps every candidate but reorders them, so
    the claim fails exactly when saturation is read as returning the
    input unchanged.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise
  beyond the one stated.
- A change to an existing §1 to §4.5 operation or law type.

Checks: 3 fired. I grepped spec 72, `Argmin.ken.md` and `Derived.ken.md`
for a saturating `take` or `least_k` law and found none; §4.3's deferral
list does not exclude it. 4 fired. I wrote the obligation in the
delivered vocabulary. `take` at `Suc m` and `Cons h t` is
`Cons h (take m t)`, and `leq_nat (Suc (length t)) (Suc m)` reduces to
`leq_nat (length t) m`, so induction on `xs` with `n` cased closes by
`cong`. The `Cons` and `Zero` case refutes its false bound with `absurd`,
and `length for sort` moves the premise onto the sorted list. 8 fired.
The refused row's input has a cheaper candidate after the head, so input
order and sorted order differ, and the claim fails exactly when the law is
read as identity on the input.
