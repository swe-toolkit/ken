---
id: VERIFY-SPACE-OBLIGATION-HOLE-REGISTRATION
title: "Obligation holes minted inside a space declaration are never recorded in the hole registry, so a claim that reaches one through a transparent space operation or cell still reads proved. Register them at the space seam"
status: ready
owner: verify
size: S
tier: T1
gate: architect
depends_on: [VERIFY-TRANSITIVE-HONESTY]
blocks: []
github: null
origin: "Adversary M8 finding evt_5yk799ke10462 on 874e9f7bb (VERIFY-TRANSITIVE-HONESTY): an honesty over-claim against 21 section 5.4 and that WP's AC-1. Steward-filed per COORDINATION section 2."
---

# Space obligation holes reach the registry

## Objective

A claim reads `proved` only if no open obligation hole is reachable from
it, including holes minted inside a `space` declaration.

## Settled inputs (Adversary `evt_5yk799ke10462`, at `874e9f7bb`)

- **The seam.** `register_obligation_holes` (modules.rs:3300) runs at
  modules.rs:3655, :3696 and :5099. The space path at :4756-4760
  (`elaborate_resolved_space` then `ids.extend(space_results)`) does not
  call it.
- **The unrecorded holes:** cell-initializer `requires` holes
  (elab.rs ~:16879), operation-body call-site holes, and operation
  `ensures` holes. Space state and operation bodies are transparent defs.
- **Repro.** `fn d (x : Int) (y : Int) : Int requires Not (Equal Int x y)
  = x`, then `space S { mut cell : Int = 0; proc call () : Int visits [S]
  = d 0 0 }` and `space T { mut cell : Int = d 0 0 }`.
  - `S.call`: hole g652 is open in `trusted_base()` but not recorded, so
    `claim_proved(S.call)` is true and `attempt_with_cert` gives Proved.
  - `theorem ts1 : Top = let u = S.call in Proved` reads proved while
    `postulates_reachable(ts1)` is {g652}.
  - `T`'s cell hole g654 is likewise unrecorded.
  - Control: `const k : Int = d 0 0` records g648, and a theorem reaching
    `k` reads not proved.
- **The AC-2 census is not evidence here.** It reads the same registry,
  so a space-sourced hole reads untainted either way.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

Every obligation hole a space declaration mints is recorded at minting,
by the same registration the other three seams use. Sweep every other
path that feeds `ids` for the same gap.

## Acceptance

- **AC-1.** `S.call`, `ts1` and a claim reaching `T`'s cell read not
  proved, and each names its reached hole.
- **AC-2 (control).** The `k` row and VERIFY-TRANSITIVE-HONESTY's suite
  and conformance row are unchanged. The catalog census is re-run, with
  any newly tainted root listed.
- **AC-3 (mutation, QA).** Removing the space-seam registration reddens
  AC-1.

## Stop conditions

- Any kernel or `trusted_base()` change.
- A catalog claim newly reads not proved: stop with the list, since that
  is a status change the Architect rules on.
