---
id: CAT-VEC
title: "Length-indexed Vector — Data/Vector: Vec n a with total head/index/zip/map and the length laws, the deliberate fully-dependent probe of the Foundation expressibility trial"
status: ready
owner: foundation
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: "Foundation expressibility trial (docs/program/wp/foundation-expressibility-trial.md), operator-directed 2026-08-21. Steward-filed, functional-build phase-1 scope. Target 5 of 5, non-sequential. The one fully-dependently-typed target and the highest expressibility risk (Fin is absent today); deliberately included per the operator's some-dependently-typed framing."
---

## Objective

Author `catalog/packages/Data/Vector/Vector.ken.md` — the length-indexed vector
`Vec n a`, the canonical dependent-types showcase, with total (exhaustive-by-type)
operations and the length laws. This is the trial's fully-dependent probe.

## Deliverables

- `Vec : Nat -> Type -> Type` (indexed inductive: `vnil : Vec 0 a`, `vcons : a ->
  Vec n a -> Vec (n+1) a`).
- Total `head : Vec (n+1) a -> a`, `tail : Vec (n+1) a -> Vec n a`, `map`, and
  `zipWith : (a -> b -> c) -> Vec n a -> Vec n b -> Vec n c`.
- A total element accessor. **`Fin` is absent from the catalog and prelude today** —
  the accessor may take a `Fin n` (define `Fin` in-entry) or a `(i : Nat) -> Lt i
  n -> a` bounded form; the encoding is the implementer's call and is part of the
  probe.
- The laws below **proved**.

## Acceptance criteria (the laws)

- `AC-1` — **length preservation:** `map`/`zipWith` return a `Vec` at the input
  length by construction (the index makes this typecheck, not a separate proof).
- `AC-2` — **totality by type:** `head`/`tail`/the accessor are total — no
  runtime bounds failure is representable; the empty case is excluded by the index.
- `AC-3` — targeted validation only; no `--workspace`. Elaborates and kernel-checks.

## Trial protocol

Functional build (phase-1): real proofs, honest trusted-base; guide-quality
refinement deferred. This is the **highest-risk** target: if the indexed
inductive, `Fin`, or total length-indexed elimination **cannot be expressed in
Ken's current surface** without a new surface/kernel capability, **STOP**, file
the gap as a finding node (name exactly what indexed-inductive/eliminator surface
is missing and which operation it blocks), move on. A cleanly-encoded `Vec` is a
strong positive signal for the lane; a blocked one is a designated, expected
finding — either way, report it. Tier: T1 (dependent-type proof engineering).
`git_request` direct to the lieutenant.
