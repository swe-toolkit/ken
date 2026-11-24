---
id: RT-NATIVE-INDEXED-CONVOY-LOWERING
title: "A dependent match on two same-indexed vectors elaborates to a convoy, a recursive eliminator whose motive is a Π telescope over an erased equation, so its result and each computational IH answer are functions. Native lowering compiles recursors for first-order answers only, and the native runtime has no durable closure lane, so a Vec zip refuses at the motive guard. Raise each method's arity by the motive telescope's runtime-visible binders in erasure, so each saturated IH use is a direct call, and refuse a partially applied IH at admission"
status: ready
owner: runtime
size: L
tier: T1
gate: architect
depends_on: [RT-NATIVE-INDEXED-FAMILY-LOWERING]
blocks: []
github: null
origin: "Architect ruling evt_6a134t5e4mfkr on INDEXED-FAMILY's Y4-d stop (third advancing, evt_tp1rehxgarss), after research advisory evt_7t7gydw2s34m5; Steward recut scope evt_3xmyjt1heygej. The convoy rows W1 and W2' leave RT-NATIVE-INDEXED-FAMILY-LOWERING and refuse at its pinned motive guard; their lowering is this node. L1 Linux ABI program, after PX11-NATIVE-STREAM-SOCKET-PARITY, which does not consume a convoy. Steward order."
---

# Native lowering of indexed convoys

## Objective

A natively built program that zips two runtime `Vec` values of the same
index by a dependent match builds and runs, and agrees with the
interpreter. No closure crosses the native boundary to do it.

## Settled inputs (main after INDEXED lands; Architect `evt_6a134t5e4mfkr`)

- **The rows.** W1 and W2' from INDEXED's D0 (`thr_5bct5cf5t57aj`), the
  Int and String `zipPrimitive` convoys. On INDEXED's landed scope they
  refuse E0075 `unsupported_dependent_motive` at
  `decl:native-program::zipPrimitive`.
- **The motive.** `λk. λv. Π(e : Eq Nat k n). Π(w : Vec A k). Vec A n`.
  `e` is Ω-sorted and erased, and `w` is runtime-visible.
- **The walls past the guard.** With the motive's Π arm admitted, W1 and
  W2' reach object emission. There, a closure has no durable lane
  (`cranelift_backend/lowering/boundary.rs`), and a case closure of arity
  0 meets 3 constructor fields (`lowering/core.rs` ~`:17276`).
- **The fact's home.** `CheckedCoreMatchView` (`checked_core.rs` ~`:314`)
  carries the motive. `RuntimeComputationalMatchCase` (`ir.rs` ~`:817`)
  carries no motive arity, so the raising is decided in erasure.
- **Starting code.** `index_only_type` and `is_equality_type` at WIP
  `f3958b11e`, reachable from `wip/RT-NATIVE-INDEXED-CONVOY-LOWERING-y4`.
- **Design direction (ii).** Each method takes the telescope's
  runtime-visible binders as parameters, and erased `Eq` binders add none.
  Each IH use saturated against them is a direct call of the raised arity.
  A partially applied IH refuses at admission under a named lane. A
  durable closure lane (i) is not adopted.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The motive admission for convoys, the arity raising in erasure, the
partial-IH refusal, and the rows below. No kernel, surface, trust or
prelude change, no durable closure lane, and no fixture rewritten to avoid
the convoy.

## Acceptance

- **AC-0 (D0, then Architect, before code).** With the Π arm restored on
  the landed base, count over W1 and W2':
  - IH uses saturated against the telescope's runtime binders;
  - partial IH uses;
  - slots that use their IH more than once.

  Attribute each emission refusal to its match or IH. The Architect then
  rules the raising, the refusal lane and the pins.
- **AC-1.** W1 and W2' build natively and agree with `run_program` on
  exit and stdout. INDEXED's W3 and W4' rows stay green, and the motive
  guard's refusal pins are restated to what the ruling still refuses.
- **AC-2.** Revert the raising: W1 and W2' refuse again at their named
  lane. Restore it byte-identically.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- AC-0 finds a partial IH use in W1 or W2'. Report it; the row then needs
  (i), which is an operator-visible representation change.
- The raising needs a kernel, surface, trust or closure-lane change.
- A row's fixture trips the provider safety flag while it is being
  written. Stop to the Steward. No seat retries it in other wording.

Checks: 6 fired. None of the three counts has been taken, and AC-0 takes
them before any ruling. 5 fired. The motive is on the checked plane and
is dropped before the backend, so the decision sits in erasure. 8 fired.
The revert makes the convoy rows refuse at their lane, so a fix that
bypasses the raising is caught.
