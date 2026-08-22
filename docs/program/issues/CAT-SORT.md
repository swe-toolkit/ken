---
id: CAT-SORT
title: "Verified insertion sort — Algorithm/Sorting: sort an Ord-ordered List with the Sorted and Permutation laws proved, the simplest-first anchor of the Foundation expressibility trial"
status: active
owner: foundation
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: "Foundation expressibility trial (docs/program/wp/foundation-expressibility-trial.md), operator-directed 2026-08-21. Steward-filed, functional-build phase-1 scope. Target 1 of 5, non-sequential."
---

## Objective

Author `catalog/packages/Algorithm/Sorting/InsertionSort.ken.md` — the Algorithm
Section's first entry. Insertion sort over `Ord a => List a -> List a`, with the
two correctness laws proved. Classic, non-dependently-typed; the trial's
simplest-first anchor.

## Deliverables

- `insert : Ord a => a -> List a -> List a` and `sort : Ord a => List a -> List a`
  (insertion sort), in a compilable `.ken.md` entry with manifest, derivation
  path, and honest `trusted_base()` delta.
- The laws below **proved**, not postulated.

## Acceptance criteria (the laws)

- `AC-1` — **sortedness:** `Sorted (sort xs)` for all `xs`, where `Sorted` is the
  standard pairwise-ordered predicate (define it in-entry if no catalog carrier
  exists — that is demand-pull, build it).
- `AC-2` — **permutation:** `Permutation xs (sort xs)` (the multiset of elements
  is preserved; define `Permutation`/count-based equivalence in-entry if absent).
- `AC-3` — targeted validation only; no `--workspace`. Elaborates and kernel-checks.

## Trial protocol

Functional build (phase-1): real proofs, honest trusted-base; guide-quality
refinement deferred. If a required law **cannot be expressed or proved in Ken's
current surface** without a new surface/kernel capability, **STOP**, file the gap
as a finding node (name the missing capability and this blocked law), and the
lieutenant/Steward move to the next target. A merely-hard proof or a missing
lemma-below is not a stop. Tier: T1 (proof engineering). `git_request` direct to
the lieutenant.
