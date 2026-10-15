---
id: SPEC-OPTIMIZATION-SEARCH-CONTRACT
title: "No optimization or search contract exists in spec/50-stdlib, so the catalog's next roadmap layer after serialization has nothing to build against. Author its chapter with a first exact section: choosing a best candidate from a finite list by a natural cost, with membership and optimality proved at zero trust"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-SERIALIZATION-ROUND-TRIP-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. 06-catalog-campaign.md:377-379 places optimization/search after protocols/serialization/supply-chain; the protocol and supply-chain parts coordinate with Lane B and are deferred (SPEC-SERIALIZATION-ROUND-TRIP-CONTRACT). Same shape as SPEC-TIME-EVENT-TRACE-CONTRACT. Steward-filed per COORDINATION section 2."
---

# An exact best-candidate contract

## Objective

A new `spec/50-stdlib` chapter exists, and its first section is the
normative contract for choosing a best candidate from a finite list by
a natural-number cost. The first optimization catalog node builds
against it.

## Settled inputs (on `967c82d77`)

- **Natural order and lists are proved.** `Data.Numeric.Nat.Order`
  exports `leq_nat`, `lt_nat`, `min` and `max` with their order laws.
  `Data.Collections.Derived` exports `length` and the list operations.
- **Search exists, optimization does not.** `Algorithm.Searching`'s
  `OrderedSearch` provides decidable membership search (`elem`,
  `search`). No spec chapter or catalog package states an argmin,
  argmax or best-candidate law.
- **Chapters 61 to 71 are taken.** 70 is diagnostic codes and 71 is
  serialization.
- **Known elaborator limits.**
  - Pass `map` a named helper, not a λ.
  - A dependent `match` refines only its scrutinee as written (CHECKS
    11).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (Architect boundary ruling, measured).** Rule from a checked
   development (rc=0, no Axiom) on:
   - the chapter number, the package's Section and its module path;
   - the operation: for example a best candidate of a non-empty list
     under a cost function into `Nat`, with its tie rule;
   - the laws: at least that the result is a member of the input, and
     that no member has a strictly smaller cost;
   - what stays deferred: real-valued and continuous optimization,
     search over infinite or implicit spaces, heuristics, and
     approximation guarantees.
2. **The chapter, section 1**, stating the D0's operations and proved
   laws, with a conformance seed of closed cases.

## Acceptance

- **AC-1.** The Architect's D0 development checks generically, with no
  Axiom, and the section states exactly the laws it proved.
- **AC-2.** The seed has at least one case per operation and one per
  law, a tie case, and a case that refuses a claimed best candidate
  which some member beats.
- **AC-3.** `spec/50-stdlib/README.md` and `SPEC-PROGRESS.md` record the
  chapter.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a new primitive.
- The D0 cannot prove optimality generically. Report the measured
  blocker, and do not state that law as proved.
