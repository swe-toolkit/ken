---
id: CAT-DEQUE
title: "Two-list functional deque — Data/Collections: a persistent double-ended queue with amortized front/back ops and a proved sequence-abstraction law, target 2 of the Foundation expressibility trial"
status: ready
owner: foundation
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: "Foundation expressibility trial (docs/program/wp/foundation-expressibility-trial.md), operator-directed 2026-08-21. Steward-filed, functional-build phase-1 scope. Target 2 of 5, non-sequential."
---

## Objective

Author `catalog/packages/Data/Collections/Deque.ken.md` — a persistent
double-ended queue held as a front list and a reversed back list (the classic
functional-DS translation, formulated from general knowledge, clean-room). Ops
that add/remove at both ends, with the abstraction law proved. Non-dependently
typed.

## Deliverables

- `Deque a` carrier (front + reversed-back lists), `empty`, `pushFront`/`pushBack`,
  `popFront`/`popBack` (returning `Option (a, Deque a)`), and `toList : Deque a ->
  List a` giving the abstract front-to-back sequence.
- The laws below **proved**.

## Acceptance criteria (the laws)

- `AC-1` — **abstraction homomorphism:** `toList (pushFront x q) = x :: toList q`
  and `toList (pushBack x q) = toList q ++ [x]`.
- `AC-2` — **pop inverts push:** `popFront (pushFront x q)` yields `Some (x, q')`
  with `toList q' = toList q`; symmetrically for `popBack`/`pushBack`.
- `AC-3` — targeted validation only; no `--workspace`. Elaborates and kernel-checks.

## Trial protocol

Functional build (phase-1): real proofs, honest trusted-base; guide-quality
refinement deferred. If a required law **cannot be expressed or proved in Ken's
current surface** without a new surface/kernel capability, **STOP**, file the gap
as a finding node, move to the next target. A merely-hard proof or a missing
lemma-below is not a stop. Tier: T1 (proof engineering). `git_request` direct to
the lieutenant.
