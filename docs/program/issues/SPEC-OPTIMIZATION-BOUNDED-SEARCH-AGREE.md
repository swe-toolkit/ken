---
id: SPEC-OPTIMIZATION-BOUNDED-SEARCH-AGREE
title: "Spec 72 §2 says the bounded-search contract is extensional and that its laws determine the result, but every BoundedSearch law speaks of one predicate, so no checked law says two predicates that agree below the bound give the same search result. Rule from a checked D0 at zero trust a public agreement law for find_below, with a conformance seed"
status: active
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-SYMBOLIC-ALGEBRA-DERIVATIVE-ABSENT-VARIABLE]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Optimization follows Symbolic algebra in the layer rotation. Steward-filed per COORDINATION section 2."
---

# Agreeing predicates search alike

## Objective

`Algorithm.Optimization.BoundedSearch` exports a public checked law:
for every `p`, `q : Nat → Bool` and `bound : Nat`, if `p m` equals
`q m` for every `m` below `bound`, then:

```ken
Equal (Option Nat) (find_below p bound) (find_below q bound)
```

Spec 72 states the law by its public type.

## Settled inputs (main `145405e2f`)

- **The search.** `find_below` (`BoundedSearch.ken.md:38`) recurses on
  the bound through the private `find_next` (`:130`) and `found_at`
  (`:136`).
- **The laws.** The package exports 6 public theorems: `sound` (`:44`),
  `least` (`:55`), `complete` (`:68`), `refuted` (`:76`), `least_hit`
  (`:82`) and `stable` (`:90`). Each speaks of one predicate.
  `find_below_stable` is the pattern for an induction on the bound.
- **The claim's gap.** Spec 72 §2 (`72-optimization.md:533`) says at
  `:577` that the contract is extensional, and at `:610` to `:611` that
  its laws determine the returned value or its absence. No law compares
  two predicates. Spec 72 runs §1 to §3 (§2.1 to §2.6; file end
  `:1243`).
- **The enablers.** `(proof self_suc for lt_nat)` and `(proof trans for
  lt_nat)` (`Data/Numeric/Nat/Order.ken.md:584`, `:590`) restrict the
  agreement premise from `Suc below` to `below`.
- **Seeds.** Under `conformance/stdlib/optimization/`, next to
  `seed-bounded-search-least-hit.md`, which defines `above_two`, `three`
  and `ten`. The package test pattern is
  `crates/ken-elaborator/tests/cat_optimization_bounded_search_least_hit.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

The ruled law in BoundedSearch, a spec 72 section stating it by its
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
  - the law's name, argument order and placement;
  - the section number;
  - the premise's form: pointwise below the bound with `lt_nat`, or an
    equivalent form.

  It confirms that no §1 to §3 operation or law type changes.
- **AC-1.** The section states the ruled law by its public type. The
  Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client uses the law to show
  `find_below above_two three` equals `find_below never three`, where
  `never` is false everywhere and the two agree below three.
  - Its refused block claims the same equation at bound `ten`, where
    the two disagree at three.
  - Its accepted same-form control is the bound-three claim, so the
    refusal is the disagreement below the bound, not the form.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or a premise
  beyond agreement below the bound.
- A change to an existing §1 to §3 operation or law type.

Checks: 4 fired: the law is written from `find_below` at `:38`,
`find_next` at `:130`, `find_below_stable`'s induction at `:90` and the
`lt_nat` proofs at `Order.ken.md:584` and `:590`, each read at its line.
3 fired: BoundedSearch's 6 public theorems and spec 72 §2 were read for
a two-predicate law; none exists. 8 fired: the refused row and its
control share the predicates and the claim, and differ only in the
bound.
