---
id: CAT-BSEARCH
title: "Decidable ordered search — Algorithm/Searching: membership over a sorted List returning a Dec proof, target 3 of the Foundation expressibility trial"
status: ready
owner: foundation
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: "Foundation expressibility trial (docs/program/wp/foundation-expressibility-trial.md), operator-directed 2026-08-21. Steward-filed, functional-build phase-1 scope. Target 3 of 5, non-sequential. Independent of CAT-SORT — sortedness enters as a hypothesis, not a build dependency."
---

## Objective

Author `catalog/packages/Algorithm/Searching/OrderedSearch.ken.md` — a search
over a sorted `List` that returns a **decision with evidence**, exercising the
landed `Dec`/`Empty` (`Core/Logic/EmptyDec.ken.md`) and `Ord`. Mildly dependent:
the result carries a proof, and sortedness is a refinement hypothesis.

## Deliverables

- `Elem : a -> List a -> Type` (membership) if no catalog carrier exists.
- `search : Ord a => (x : a) -> (xs : List a) -> Sorted xs -> Dec (Elem x xs)`
  (or an index-plus-proof form) — a total decision that uses the sortedness
  hypothesis to prune. `Sorted` may be shared with / defined as in CAT-SORT's
  formulation, but this node does **not** depend on CAT-SORT.
- The correctness of both `Dec` arms **proved**.

## Acceptance criteria (the laws)

- `AC-1` — **sound-yes:** the `yes` arm carries a real `Elem x xs` witness.
- `AC-2` — **sound-no:** the `no` arm carries `Elem x xs -> Empty` (a real
  refutation), discharged using sortedness.
- `AC-3` — targeted validation only; no `--workspace`. Elaborates and kernel-checks.

## Trial protocol

Functional build (phase-1): real proofs, honest trusted-base; guide-quality
refinement deferred. If the decision or either arm's proof **cannot be expressed
in Ken's current surface** without a new surface/kernel capability, **STOP**, file
the gap as a finding node, move to the next target. A merely-hard proof or a
missing lemma-below is not a stop. Tier: T1 (proof engineering). `git_request`
direct to the lieutenant.
