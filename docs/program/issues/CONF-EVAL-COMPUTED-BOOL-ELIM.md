---
id: CONF-EVAL-COMPUTED-BOOL-ELIM
title: "The conformance matrix does not state that a closed computed Bool consumed by the Bool eliminator selects the same method as the corresponding constructor -- the two runtime representations reach the eliminator by independent index derivations and nothing ties them together"
status: merged
owner: spec-enclave
size: S
gate: none
depends_on: [CI-L1-EXECUTING-COVER]
blocks: []
github: null
origin: "Surfaced by CI-L1-EXECUTING-COVER's D5 row-claim checker on its first real-tree run (verify-implementer hard stop evt_1rg7vw502amr0, 2026-08-10): four tests in elim_bool_dispatch_acceptance.rs claimed surface/numbers/elim-reduce-computed-bool-* ids that resolve to zero conformance headings. Steward ruled the false certificates retired and forbade Verify from authoring conformance artifacts. Conformance Validator judgment evt_2ah01fn9v4ev3 ruled the family belongs in the matrix but under conformance/runtime/evaluation/, not surface/numbers/. Mechanism independently measured by the Steward at origin/main bebe1a79 in crates/ken-interp/src/eval.rs elim_reduce. Steward-filed (agents cannot create tracked work per COORDINATION §2)."
---

> ## READY as of 2026-08-10 — the dependency is MET. Frame is shovel-ready.
>
> **`CI-L1-EXECUTING-COVER` merged** at exact `bfac3f6f` (PR #1776, `main`
> `53c09f9b`), all six paths blob-verified. The edge this node waited on was
> real, not ceremonial, on two counts and **both are now discharged**:
>
> 1. **`AC-1` is discharged by a checker that node delivered.**
>    `scripts/ci-ignored-sweep.py verify-row-claims` now exists on `main` and
>    reports **29 resolved claims** on the delivered tree. That number is
>    `AC-1`'s baseline: the count must rise by exactly the number of claims
>    this node adds.
> 2. **Both nodes write
>    `crates/ken-interp/tests/elim_bool_dispatch_acceptance.rs`.** That file is
>    now settled on `main` with the four false certificate lines retired to
>    prose and every assertion untouched, so `D2`'s comment edits no longer
>    collide.
>
> **Frame:** `docs/program/wp/CONF-EVAL-COMPUTED-BOOL-ELIM.md`. Nothing further
> is owed and no additional framing pass is needed — this is releasable to the
> spec enclave on the next sequencing pass.

## Why this is not "the tests already cover it"

The four tests exist, assert, and pass. What is missing is the **matrix
statement**, and the gap is a composition seam that both existing halves stay
green across:

- Numbers coverage establishes `eq_int` / `leq_int` compute the right logical
  `Bool`.
- Iota coverage establishes a constructor `True` / `False` selects the right
  eliminator method.

Under the historical bug both halves passed while a primitive-computed `Bool`
reached the evaluator in its scalar representation and left a closed ground term
**neutral**.

## The measured fact that makes the family load-bearing

Measured at `origin/main = bebe1a79`, `crates/ken-interp/src/eval.rs`,
`elim_reduce`: the two scrutinee arms derive the method index by **independent
routes**. The `Ctor` arm looks the constructor up through
`globals.constructor(ctor_id)`; the `Bool` arm hardcodes
`let k = if b { 0 } else { 1 }`, correct only because `data Bool = True | False`
declares its constructors in that order.

A change to `Bool`'s declared constructor order would be followed by one arm and
not the other. **The computed-versus-literal agreement observation is the only
thing that could catch that**, and the matrix does not state it.

A flipped repair is also strictly worse than the original bug: it returns the
wrong branch silently, where the original produced a visible stuck term. So
`runtime/evaluation/can-no-stuck-closed-ground` would not catch it either.

## What was already settled and is not reopened here

- The four phantom `surface/numbers/elim-reduce-computed-bool-*` claim lines are
  **retired**, converted to ordinary prose with no assertion or test body
  changed.
- `surface/numbers/legacy-add-sub-mul-retired` is **closed as decorative**. It
  is not part of this node.
- Authoring `conformance/` artifacts is the spec enclave's and Conformance
  Validator's lane. That is why this node is not owned by Verify, and it is an
  ownership boundary rather than a sizing one.
