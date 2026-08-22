---
id: CAT-GCD
title: "Euclidean gcd with divides laws — Algorithm/Numeric: gcd over Nat proved to be a greatest common divisor, target 4 of the Foundation expressibility trial and a deliberate termination-presentation probe"
status: merged
owner: foundation
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: "Foundation expressibility trial (docs/program/wp/foundation-expressibility-trial.md), operator-directed 2026-08-21. Steward-filed, functional-build phase-1 scope. Target 4 of 5, non-sequential. Chosen to probe well-founded/structural recursion and the termination-checker presentation."
---

## Objective

Author `catalog/packages/Algorithm/Numeric/Gcd.ken.md` — the Euclidean algorithm
on `Nat` with its divisibility laws proved. Non-dependently-typed carrier, but
**termination-proof-heavy**: Euclid's descent is on the remainder, not a
structural sub-term, so this deliberately probes how Ken's surface presents
well-founded / structural recursion to the termination checker.

## Deliverables

- `Divides : Nat -> Nat -> Type` (or reuse a catalog carrier if present).
- `gcd : Nat -> Nat -> Nat` (Euclid), authored so its recursion is accepted by
  the termination checker — the presentation choice (structural reformulation,
  fuel, or a well-founded measure) is the implementer's call and is itself the
  probe.
- The laws below **proved**.

## Acceptance criteria (the laws)

- `AC-1` — **common divisor:** `Divides (gcd a b) a` and `Divides (gcd a b) b`.
- `AC-2` — **greatest:** for any `d`, `Divides d a -> Divides d b -> Divides d
  (gcd a b)`.
- `AC-3` — targeted validation only; no `--workspace`. Elaborates and kernel-checks.

## Trial protocol

Functional build (phase-1): real proofs, honest trusted-base; guide-quality
refinement deferred. If `gcd` **cannot be presented so its recursion is accepted**,
or a law cannot be proved, in Ken's current surface without a new surface/kernel
capability, **STOP**, file the gap as a finding node (name the termination/surface
capability missing), move to the next target. A merely-hard proof is not a stop.
Note: the language ring is concurrently working a termination-presentation issue
(SCT over a mutual group) on a different node — if this target hits the **same**
surface wall, say so in the filed gap; it corroborates rather than duplicates.
Tier: T1 (proof engineering). `git_request` direct to the lieutenant.
