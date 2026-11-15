---
id: SPEC-OPTIMIZATION-LEAST-K-PREFIX
title: "Spec 72 §4 gives least_k eight laws and cost-order invariance but no law relating different k, so a client cannot conclude that the k cheapest candidates are the first k of the k+m cheapest. Rule from a checked D0 at zero trust that least_k at k is the k-prefix of least_k at add k m, and state it with a conformance seed"
status: merged
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Optimization follows Symbolic algebra in the L3 layer rotation (docs/program/06-catalog-campaign.md:376); least_k (spec 72 §4) has no law across selection sizes. Steward-filed per COORDINATION section 2."
---

# The k cheapest are a prefix of the k+m cheapest

## Objective

`Algorithm.Optimization.Argmin` exports a public checked law that
`least_k a cost k xs` equals `take a k (least_k a cost (add k m) xs)`
as `Equal (List a)`, for every `k`, `m`, `cost` and `xs`. It holds at
zero trust, and spec 72 states it by its public type.

## Settled inputs (main `fd523a842`)

- **The operation.** `least_k` (`Argmin.ken.md:305`) is
  `take a k (sort a (cost_le a cost) xs)`. Its public laws run from
  `least_k_length` (`:308`) to `least_k_cost_order_invariant` (`:487`).
  None relates two selection sizes.
- **The list floor.** `take` (`Data/Collections/Derived.ken.md:130`)
  recurses on `n`, then on the list. No public `take` composition law
  exists in `catalog/packages`. The law reduces to
  `take k (take (add k m) ys) = take k ys` for the one sorted list `ys`.
- **Spec and tests.** Spec 72 §4 ends at §4.4
  (`72-optimization.md:1595`), so the new subsection is §4.5. The test
  pattern is `crates/ken-elaborator/tests/cat_optimization_least_k_cost_order.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 72 §4.5, the catalog law with any helper, a seed, a package test
and the status lines. No §1 to §4.4 operation or law type moves, and no
kernel or surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-OPTIMIZATION-LEAST-K-COST-ORDER-INVARIANCE.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules:
  - the name and binder order;
  - where the `take` composition step lives: private in `Argmin`, or
    public in `Data.Collections.Derived` with that module named as its
    owner;
  - the proof route.
- **AC-1.** §4.5 states the ruled law by its public type. The Spec
  Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted block applies the law at closed
  arguments with `k = 2` and `m = 1`, over at least four candidates
  whose costs include a tie at the cut.

  Its refused block pins, in the same form as an accepted row and on a
  fresh accepted prefix:
  - a claim that `least_k` at `k = 2` equals `least_k` at `k = 3` on the
    same list;
  - a claim that it equals the last two of `least_k` at `k = 3`.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise.
- A change to an existing §1 to §4.4 operation or law type.

Checks: 4 fired. Both sides take from the same `sort a (cost_le a cost)
xs`, so the obligation is the `take` composition alone. Its `Suc` step
on `k` is the same `Cons` on both sides once the list is split. 3 fired:
the grep for `least_k` and for `take` theorems covered
`catalog/packages`, `spec/`, `conformance/` and `crates/`. 8 fired: with
four candidates, `k = 3` gives three elements where `k = 2` gives two,
and with distinct candidates the last two differ from the first two.
So each refused row is false on a closed list. The tie at the cut makes
the accepted row depend on the stable order, which a law that sorted
each side independently would not have to keep.
