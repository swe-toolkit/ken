---
id: SPEC-OPTIMIZATION-BOUNDED-SEARCH
title: "Spec 72 covers choosing a best candidate from a given list, but not the search half of the optimization/search layer: finding the least natural number below a bound that satisfies a decidable predicate. Author section 2 with a find operation whose soundness, leastness and completeness are proved at zero trust"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-OPTIMIZATION-SEARCH-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. 06-catalog-campaign.md roadmap layer optimization/search; spec 72 section 1 (SPEC-OPTIMIZATION-SEARCH-CONTRACT) delivers the optimization half. The next layer, verification/model-checker interop, coordinates with Lane B and is not on the roster. Steward-filed per COORDINATION section 2."
---

# An exact bounded search contract

## Objective

Spec 72 gains a normative section 2: finding the least `n < bound` for
which a Boolean predicate on `Nat` holds, or reporting that none does.
The search catalog node builds against it.

## Settled inputs (on the SPEC-OPTIMIZATION landing)

- **Section 1 exists.** Spec 72 section 1 states `argmin` over `NonEmpty`
  under a `Nat` cost. Section 1.3 defers infinite and implicit spaces,
  heuristics, approximation and `argmax`.
- **Nat order is proved.** `Data.Numeric.Nat.Order` exports `leq_nat`,
  `lt_nat`, `min` and `max` with their order laws.
- **Search over a given list exists.** `Algorithm.Searching` provides
  membership search over a sorted list (`elem`, `search`). No spec
  section or catalog package states a search over a bounded `Nat`
  range.
- **Known elaborator limits.**
  - Pass `map` a named helper, not a λ.
  - A dependent `match` refines only its scrutinee as written (CHECKS
    11).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (Architect boundary ruling, measured).** Rule from a checked
   development (rc=0, no Axiom) on:
   - the operation, for example `find_below (p : Nat → Bool) (bound : Nat)
     : Option Nat`, and its module path;
   - the laws: at least soundness (a `Some n` result has `n < bound` and
     `p n = True`), leastness (no `m < n` has `p m = True`) and
     completeness (`None` means no `n < bound` has `p n = True`);
   - whether section 1's deferred `Option`-returning selector belongs
     here or stays deferred.
2. **Section 2 of spec 72**, stating the D0's operations and proved laws,
   with a conformance seed of closed cases.

## Acceptance

- **AC-1.** The Architect's D0 development checks generically, with no
  Axiom, and the section states exactly the laws it proved.
- **AC-2.** The seed has at least one case per operation and one per
  law, a not-found case, and a case that refuses a claimed result that
  a smaller witness beats.
- **AC-3.** `spec/50-stdlib/README.md` and `SPEC-PROGRESS.md` record the
  section.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a new primitive.
- The D0 cannot prove completeness generically. Report the measured
  blocker, and do not state that law as proved.
