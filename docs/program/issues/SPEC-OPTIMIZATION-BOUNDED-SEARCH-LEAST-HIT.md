---
id: SPEC-OPTIMIZATION-BOUNDED-SEARCH-LEAST-HIT
title: "Spec 72 §2 states that a Some n from find_below is an in-range hit with no earlier hit, and §2.5 gave None its converse, but Some has none: a client that holds an in-range hit n with every earlier m refuted cannot conclude that find_below p bound is Some n without casing on the result itself. Rule from a checked D0 at zero trust a public least-hit law, with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-SYMBOLIC-ALGEBRA-SUBSTITUTION-POINTWISE-CONGRUENCE]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Optimization follows Symbolic algebra in the layer rotation. Steward-filed per COORDINATION section 2."
---

# The least in-range hit is the search result

## Objective

`Algorithm.Optimization.BoundedSearch` exports a public checked law
that, for every `p : Nat → Bool` and `bound`, `n : Nat`:

```ken
Equal Bool (lt_nat n bound) True
  → Equal Bool (p n) True
  → ((m : Nat) → Equal Bool (lt_nat m n) True → Equal Bool (p m) False)
  → Equal (Option Nat) (find_below p bound) (Some Nat n)
```

It is the converse of `find_below_sound` with `find_below_least`, as
§2.5's `find_below_refuted` is of `find_below_complete`. Spec 72 states
it by its public type.

## Settled inputs (main `554523368`)

- **The forward laws.** In `BoundedSearch.ken.md`, `find_below_sound`
  (`:42`) turns `Some n` into `n < bound` and `p n = True`;
  `find_below_least` (`:53`) refutes every `m < n`;
  `find_below_complete` (`:66`) refutes the whole range on `None`.
- **The precedent route.** `find_below_refuted` (`:74`) is
  `refuted_outcome_none p bound (find_below p bound) Refl`. The private
  helper (`:174`) binds the result as `outcome` with its equation,
  because `match ... eqn:` takes only nullary enums (AC-0
  evt_6bgw9h9v84f0p).
- **The missing step.** In the `Some k` case, `k < n` is refuted by
  the premise against `find_below_sound`'s `p k = True`, and `n < k`
  by `find_below_least` against `p n = True`. Concluding `k = n` needs
  Nat trichotomy. `Data.Numeric.Nat.Order` has 18 theorems, and none
  concludes `Equal Nat` from two false `lt_nat` results. The `None`
  case is absurd by `find_below_complete` at `n`.
- **Not yet stated.** Spec 72 §2 runs §2.1 to §2.5
  (`72-optimization.md:532` to `:762`). No section and no package law
  concludes `Some`.
- **Seeds.** Under `conformance/stdlib/optimization/`, next to
  `seed-bounded-search-refuted-range.md`, whose `two`, `three`, `ten`
  and `above_two` can be reused; `four` is new. The package test is
  `crates/ken-elaborator/tests/cat_optimization_bounded_search.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

The ruled law in BoundedSearch, a spec 72 §2 section stating it by its
public type, a closed conformance seed under
`conformance/stdlib/optimization/`, a named package test, and spec 72's
status lines, all in one candidate.

## Acceptance

- **AC-0 (D0).** The Architect specifies a development on the landed
  base: the source, and the named-binary commands with a bounded
  timeout. The Foundation implementer runs it on scratch and reports
  rc, the `Axiom` grep and `trusted_base()` as an identity set before
  and after. The Architect rules from that report, and does not build
  or test. It rules:
  - the law's name, argument order, premise form and placement;
  - the section number;
  - whether the trichotomy step is a public Nat.Order law or a
    private BoundedSearch helper.

  It confirms that no §1 to §3 operation or law type changes.
- **AC-1.** The section states each ruled law by its public type. The
  Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the law to `above_two`
  at bound `ten` and `n = three`, concluding `Some three`.
  - Its refused block claims `Equal (Option Nat) (find_below above_two
    ten) (Some Nat four)`. That is false, because `three` is the least
    hit.
  - Its accepted same-form control claims the same `Some four` for the
    predicate true exactly above three, so the refusal is the earlier
    hit, not the form.
  - The D0 rules the proof form that separates the two rows; per
    evt_6bgw9h9v84f0p, `Refl` does not discriminate an `Equal` goal.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or any premise
  beyond the hit, its bound and the refuted prefix.
- A change to an existing §1 to §3 operation or law type.

Checks: 4 fired: the law is written from public `find_below_sound`,
`find_below_least`, `find_below_complete` and the `refuted_outcome_none`
route, each read at its line, and the one missing step, Nat trichotomy,
is named after grepping Nat.Order's 18 theorems. 3 fired: spec 72 §2
and the package were searched for a law concluding `Some`; none exists.
8 fired: the refused row and its control share the bound, the claimed
result and the claim form, and differ only in the predicate's
threshold, two against three.
