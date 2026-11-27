---
id: SPEC-OPTIMIZATION-FIND-BELOW-NONE-SHRINK
title: "Spec 72 §2.4 proves that a witness found below a bound is still found below any larger bound (find_below_stable), but no law states the absence half: a client that knows a search below a large bound found nothing cannot conclude that a search below a smaller bound finds nothing without re-deriving it from completeness and refutation. Rule from a checked D0 at zero trust that find_below p larger = None and bound <= larger give find_below p bound = None, and state it with a seed whose accepted row is open in p"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Optimization follows Symbolic algebra in the L3 layer rotation (docs/program/06-catalog-campaign.md). Bound monotonicity is stated for a found witness only. Steward-filed per COORDINATION section 2."
---

# Nothing found below a larger bound means nothing below a smaller one

## Objective

`Algorithm.Optimization.BoundedSearch` exports a public checked law. For
every `p`, `bound` and `larger`, from `Equal Bool (leq_nat bound larger)
True` and `Equal (Option Nat) (find_below p larger) (None Nat)`, it states

`Equal (Option Nat) (find_below p bound) (None Nat)`.

It holds at zero trust, and spec 72 states it by its public type.

## Settled inputs (main `e71c0c52a`; anchors re-checked at release)

- **The operation.** `find_below` (`BoundedSearch.ken.md:40`) scans upward
  from zero below `bound`.
- **The argument, for AC-0 to check.**
  - `find_below_complete` (`:70`) at `larger` refutes every `m` with `m <
    larger`.
  - For `m < bound`, `bound <= larger` gives `m < larger`. AC-0 picks the
    order step. BoundedSearch imports only `leq_nat` and `lt_nat`, so AC-0
    rules whether this needs a private helper or an existing public order
    law. `find_below_stable` (`:92`) already crosses the same pair of
    bounds.
  - `find_below_refuted` (`:78`) at `bound` closes the law.
- **No such law exists.** BoundedSearch has 10 public names. The only
  bound-monotonicity law, `find_below_stable`, preserves `Some` upward.
  Nothing preserves `None` downward.
- **Spec and tests.**
  - Spec 72 §2 ends with §2.8 (`72-optimization.md:1006`), so the new
    section is §2.9.
  - The seed directory is `conformance/stdlib/optimization/`.
  - The test pattern is
    `crates/ken-elaborator/tests/cat_optimization_bounded_search_bound_stability.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 72 §2.9, the catalog law with any private helper, a seed, the package
test rows and the status lines. No other operation or law type moves, and
there is no kernel or surface change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-OPTIMIZATION-BOUNDED-SEARCH-OFFSET-SPLIT.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules:
  - the name, the binder order and the placement;
  - the order step, and whether it stays private;
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`, including any exact
    export roster and count-bearing sentence.
- **AC-1.** §2.9 states the law by its public type. CV and the Architect
  review it, with a Decision.
- **AC-2.** The seed's accepted block must not close by computation alone.
  - Its accepted row is open in `p`. For every `p` with `find_below p 5 =
    None`, it concludes `find_below p 3 = None`.
  - Its refused twin, on a fresh accepted prefix and in the same form,
    claims the converse: from `find_below p 3 = None`, it concludes
    `find_below p 5 = None`. That claim must refuse.
  - AC-0 picks the closing terms and may strengthen the seed.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Each touched package test target passes on the exact
  candidate with no name filter. Full-workspace CI is green on the exact
  candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise beyond
  the two stated.
- The proof needs a new public export outside BoundedSearch; report it
  rather than widening another package.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing operation or law type.

Checks: 4 fired. I read `find_below`, `find_below_complete`,
`find_below_refuted` and `find_below_stable` on `e71c0c52a`. The law
composes completeness at the larger bound with refutation at the smaller
one. 3 fired. I listed BoundedSearch's 10 public names and found
monotonicity only for `Some`. 8 fired. A closed seed row computes without
the law (ADD-POINT-EXACT AC-0, `evt_1b1n2fsh3bvws`), so the witness row is
open in `p`, and the refused twin reverses the direction.
