---
id: RT-COMPOSED-RETURN-TAIL-FORWARD-EDGE
title: "Composed-return native repair, option (a)(i) WP2 of 3 (HELD CHECKPOINT of one atomic merge unit — no independent QA/Decision/merge): add the consuming method on the move-only forward-Ret authority so TailProducerToRet branches its exact Trap-checked call-result word to the function-local shared Ret block; represent 'edge already emitted' only as compiler control; generated-entry quotient and two-word header/backedge ABI stay structurally unchanged."
status: active
owner: runtime
size: M
gate: none
tier: T1
depends_on: [RT-COMPOSED-RETURN-DIRECT-ROLE-SPLIT]
blocks: [RT-COMPOSED-RETURN-ATOMIC-CLOSEOUT]
github: null
origin: "Architect component design for the operator-funded composed-return native repair, option (a)(i) (PART 1/2 evt_381dzjykr4knn, PART 2/2 evt_5963far74b735, 2026-09-01). WP2 of the three-checkpoint ATOMIC merge unit; a HELD CHECKPOINT — no QA, Decision, publication, or merge follows it. Flips ready when WP1's checkpoint is reached; the Steward releases it (the WP1 landing does not authorize a start — the sequence is one ring executing WP1->WP2->WP3, but each checkpoint release is the Steward's). Bound base e6a6c5240; the Architect's two posts are the authoritative mechanism contract."
---

> # WP2 of 3 — HELD CHECKPOINT of the RT-COMPOSED-RETURN atomic merge unit.
> # DRAFT — reached after WP1's checkpoint. Lands NOTHING on its own (no QA,
> # Decision, PR, or merge). The sole production candidate is cut only after WP3.
> #
> # Mechanism contract: Architect PART 1/2 `evt_381dzjykr4knn` + PART 2/2
> # `evt_5963far74b735`. Read both. Do not reopen the twelve-stop D0 chain.

## What WP2 builds (Architect PART 1/2 mechanism, PART 2/2 deliverables)

`TailProducerToRet` keeps its existing governed call. After it returns, the exact
Trap-checked result SSA word branches directly to the exact function-local shared
`Ret` block, under the EXISTING move-only forward-Ret authority — a producer-order
change that co-locates result and the already-planned Ret sink BEFORE app486's
environment escape and the answer collapse. The call stays in CBV/effect order;
the branch occurs only after Trap-before-Result yields its Result.

The authority consumer is the ONLY accessor for its `Block`: it takes `self`,
accepts only `CheckedIhApplicationResult`, and exposes neither Block, operand,
proof, nor `Option`. The proof binds selected transport member, generated-entry
projection, active frame, Ret body and binder; the operand comes from the actual
call instruction. This is dynamic ownership, not endpoint co-occurrence.

Existing sink machinery (do NOT widen): move-only
`ComposedReturnForwardRetAuthority` at `mod.rs:2782`; planner proof
`checked_ih_forward_ret_plan_proof` at planner `aggregates.rs:7327`; function-local
sink resolution/authority formation at `core.rs:12215-12418`. Old carried path
untouched for other populations: active jump `core.rs:12528-12559`, two-parameter
header `:12562-12578`, ordinary Ret predecessor `:12751-12762`, checked fallback
`:12989-13063`. Generated-entry quotient `aggregates.rs:6471`, confluence `:6726`,
publication `:6988-7101` — none widened.

## Deliverables (Architect PART 2/2)

- Add the consuming method on the move-only forward-Ret authority.
- At the existing Tail producer, branch the exact Trap-checked result word to the
  shared Ret block (`authority.consume(result)` emits `jump return_body(result.word)`).
- Represent "edge already emitted" ONLY as compiler control (switch to a fresh
  unreachable block; return explicit compiler control meaning "edge already
  emitted"); `RecursiveBackedge` may propagate that outcome through
  source-continuation cleanup but is NEVER payload.
- Leave generated-entry quotient and the two-word header/backedge ABI
  structurally unchanged.

## Acceptance criteria (Architect PART 2/2)

- **AC-TAIL-LEDGER.** An identity-keyed causal ledger relates selected Tail
  source, actual call result, authority, branch argument, Ret block parameter and
  Ret body input, per arrival.
- **AC-TAIL-AT-MOST-ONCE.** Compilation makes duplicate authority/result
  consumption unrepresentable where possible; independently mutate drop and
  duplicate consumption where representable.
- **AC-TAIL-SUBSTITUTION.** Population-side substitutions of seed, operation word,
  and each neighbouring live word REDDEN while the actual call remains emitted.
- **AC-TAIL-WRONG-SOURCE.** Wrong producer call, governed invocation/call/callee,
  active frame, Ret body or binder each reaches its OWN exact refusal; reversed
  direction and non-producer-direct delivery refuse.
- **AC-TAIL-NO-ABI-CHANGE.** No app486 call, no extra header argument, no ABI
  change, no Tail-through-Direct call.

## Held-checkpoint discipline

Same as WP1: held checkpoint commits only; no PR/QA/Decision/merge. On completing
WP2's ACs, hold locally and proceed to WP3 (`RT-COMPOSED-RETURN-ATOMIC-CLOSEOUT`),
which cuts the sole candidate. Any uncovered outcome is a HARD STOP to the
Architect.
