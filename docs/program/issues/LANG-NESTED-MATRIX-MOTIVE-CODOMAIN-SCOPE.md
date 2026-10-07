---
id: LANG-NESTED-MATRIX-MOTIVE-CODOMAIN-SCOPE
title: "nested_matrix_motive's non-reverting branch infers the codomain's sort in a context missing the split binder, and a guarded arm swallows the resulting VarOutOfScope into Level::Zero, so every such split stores its motive at Type 0 and an or-pattern match whose result lives at Type 1 is rejected. Infer under the binder and stop swallowing the error"
status: ready
owner: language
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary finding F2 evt_2akanmfydmqgz on 440b216f1: correctness, false reject, fails closed, pre-existing, reachable from well-formed source with no level metas. Steward-filed per COORDINATION section 2."
---

# A nested-matrix motive is sorted under its own binder

## Objective

An or-pattern or nested-pattern match elaborates at its result type's
level. The motive's sort is inferred in the context it lives in.

## Settled inputs (Adversary, measured at `440b216f1`)

- `nested_matrix_motive`'s non-reverting branch (elab.rs:20599): `codomain`
  sits under the split binder but is inferred in `cx.ctx` without it. The
  kernel returns `VarOutOfScope { index: 3, depth: 3 }`.
- elab.rs:20605-20606, `Err(Kernel(_)) if !needs_reverting &&
  active_index_premise_frames.is_empty() => Level::Zero`, swallows that
  error. The `(a : Type)` rows pass only through this fallback.
- **Repro**, prelude only, each `KernelRejected TypeMismatch { expected:
  Type 0, found: Type suc 0 }`:
  `fn m1 (a : Type 1) (b : Bool) (x : a) : a = match b { True | False ↦ x }`;
  the same over `n : Nat` with `Zero | Suc _ ↦ x`; and
  `match n { Zero | Suc Zero ↦ x; Suc (Suc k) ↦ x }`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The codomain's sort is inferred with the split binder pushed. A kernel error
from that query is not mapped to `Level::Zero`. The Architect rules at AC-0
what remains of the guarded arm.

## Acceptance

- **AC-0.** The Architect's ruling on the guarded arm: delete it, or name
  the population it still serves, counted.
- **AC-1.** The three repro rows check at `(a : Type 1)`, and their
  `(a : Type)` twins still check.
- **AC-2 (controls).** The nested non-or pattern at `(a : Type 1)` and
  `fn m1 (n : Nat) : Type 1 = match n { Zero | Suc Zero ↦ Type; Suc (Suc k)
  ↦ Type }` stay Ok. The catalog census has no verdict change.
- **AC-3 (mutation).** Restoring the binder-less query reddens AC-1, and
  restoring the swallow arm alone makes the query error invisible: name the
  row that observes it.

## Stop conditions

- A population that needs the `Level::Zero` fallback: stop to the
  Architect with it.
- Any kernel, `trusted_base()` or spec change.
