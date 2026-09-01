---
id: RT-COMPOSED-RETURN-DIRECT-ROLE-SPLIT
title: "Composed-return native repair, option (a)(i) WP1 of 3 (HELD CHECKPOINT of one atomic merge unit — no independent QA/Decision/merge): split captured-environment from application-result in private compiler types, and make DirectInvocationReturn's carried-environment arm emit one declared continuation call whose Trap-checked Result is the application result (no environment-as-result). app486 and constructor materialization stay environment-only; Tail byte-identical except shared private refactoring."
status: active
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: [RT-COMPOSED-RETURN-TAIL-FORWARD-EDGE]
github: null
origin: "Architect component design for the operator-funded composed-return native repair, option (a)(i) co-located forward Ret edge (PART 1/2 evt_381dzjykr4knn, PART 2/2 evt_5963far74b735, 2026-09-01), grounded on the operator funding (evt_3met6tbk5wrnd) that followed the accepted terminal NO_UNIQUE_EDGE (evt_mx6scjje1yjp). Bound base current origin/main e6a6c5240e6d74381ffcd755c60c6705f1a99501, tree 1513c8279a669225e985ee97e746c0af786021a7. This is WP1 of a three-checkpoint ATOMIC merge unit (WP1 RT-COMPOSED-RETURN-DIRECT-ROLE-SPLIT -> WP2 RT-COMPOSED-RETURN-TAIL-FORWARD-EDGE -> WP3 RT-COMPOSED-RETURN-ATOMIC-CLOSEOUT); the three land as ONE product candidate cut only after WP3. WP1 and WP2 are HELD CHECKPOINTS: no QA, Decision, publication, or merge follows either. Steward owns framing + release sequencing; the Architect's two posts are the authoritative mechanism contract (do NOT re-derive it here)."
---

> # WP1 of 3 — HELD CHECKPOINT of the RT-COMPOSED-RETURN atomic merge unit.
> # RELEASED to the runtime ring (lane 1). Runtime is parked; this IS the release.
> #
> # **This node lands NOTHING on its own.** No QA, no Decision, no PR, no merge
> # follows WP1. The runtime ring executes WP1, holds the checkpoint locally
> # (even if green), then proceeds to WP2, then WP3 — and only WP3 cuts the SOLE
> # production candidate from WP1+WP2+WP3, which then goes to Runtime QA +
> # Architect. Intermediate commits are held checkpoints and may be amended or
> # discarded. Splitting code generation is permitted; splitting the semantic
> # landing is not.
> #
> # **The mechanism contract is the Architect's, verbatim in scope.** PART 1/2
> # (`evt_381dzjykr4knn`) fixes the mechanism and re-measured coordinates; PART
> # 2/2 (`evt_5963far74b735`) fixes this WP's deliverables and ACs. Read BOTH
> # before cutting anything. Do NOT reopen the twelve-stop D0 chain as another
> # selector/materializer/capsule recut — it is evidence/history.

## Mechanism (option (a)(i), co-located forward Ret edge) — the frame this WP sits in

Co-locate the governed producer result with the exact shared `Ret` sink on ONE
forward SSA edge BEFORE the `RoutedAnswer` / constructor collapse. Route-specific:

- **`DirectInvocationReturn`** turns its carried capture environment into one
  exact declared continuation call; that local call result IS the application
  result. **(WP1 builds this.)**
- **`TailProducerToRet`** keeps its existing governed call; its returned SSA word
  branches directly to the exact function-local shared `Ret` block under the
  existing move-only forward-Ret authority. (WP2 builds this.)

Do NOT de-quotient the generated entry and do NOT widen the two-word carried
backedge (that is (a)(ii)/(b), out of scope). No runtime callable object.

## Fixed inputs (re-measured by the Architect at base e6a6c5240)

- Base `origin/main` `e6a6c5240e6d74381ffcd755c60c6705f1a99501`, tree
  `1513c8279a669225e985ee97e746c0af786021a7`.
- Blobs: `lowering/source.rs` `c39f82e7854f626244b4398ba9941ae38b25485e`;
  `lowering/core.rs` `eea98dc6ddb0ae2f7656b16fed7ee461b24de0a1`;
  `lowering/calls.rs` `fa010fed973dfa8cb638c3a2a546594b93443efb`;
  `lowering/mod.rs` `2d461cef0b98ca8ff04e2853624b838c2e375293`;
  planner `aggregates.rs` `e7bc36287fd0557b0670a6e1ab20171be42f6dbd`;
  planner `continuations.rs` `2f7700d15dd37bb834533ea879425143e2221e90`.
- WRITE witness: app486/u0:53, template4, closure1246/body1238/arity1/captures7,
  route `GeneratedContext`, `DirectSpecializationCall`, `transport=None`. It
  materializes environment; it is NOT the new call site and stays zero-call.
- Current producer `source.rs:4491-4496`; loss through the general split at
  `source.rs:1266+`, `ConstructArgument` `:1636-1726`, replacement
  `RoutedAnswer::direct(constructed)` `:1725`.

## Deliverables (Architect PART 2/2)

- Separate captured-environment and application-result compiler types / entry
  points: `CheckedIhCapturedEnvironment` and `CheckedIhApplicationResult` may each
  carry an `i64` SSA word, but NEITHER converts generically into the other.
  Constructor materialization receives only the first; governed application only
  the second.
- Preserve app486 and constructor materialization as **environment-only**.
- Implement Direct's carried-environment projection under the exact
  `DirectInvocationReturn` projection: validate the source record, project
  captures in planner order, assemble the existing ordinary envelope plus
  continuation-input morphism, resolve ONLY
  `continuation_calls[transport.source_call_identity()]`, and emit ONE declared
  call through existing authority; its Trap-checked Result becomes
  `CheckedIhApplicationResult`. No runtime tag or worker reconstruction selects
  this arm.
- Leave Tail's producer behavior **byte-identical** except shared private
  refactoring.

## Acceptance criteria (Architect PART 2/2)

- **AC-DIRECT-EXHAUSTIVE.** Planner-variant selection is exhaustive; a
  missing/unknown variant REFUSES (fail-closed).
- **AC-DIRECT-PAIRING.** Every Direct governed arrival pairs to its exact
  transport/call identity and one emitted call/result.
- **AC-DIRECT-MUTATIONS.** Population-side, compile-preserving mutations that
  drop the call, vary transport identity, permute/drop one capture, or substitute
  environment for Result each REDDEN the exact Direct relation; each mutation
  records application provenance and restores byte-identically.
- **AC-TAIL-INERT.** Tail cannot enter the Direct lookup; app486 remains
  zero-call.
- **AC-CHECKPOINT.** Record exact executed-test counts and HOLD the checkpoint
  locally even if green (no candidate, no gate, no merge at WP1).

## Held-checkpoint discipline

WP1 produces held checkpoint commits only. Do not open a PR, route QA, propose a
Decision, or merge. On completing WP1's ACs, hold locally and proceed to WP2
(`RT-COMPOSED-RETURN-TAIL-FORWARD-EDGE`). Any outcome other than "WP1 ACs met,
Tail byte-identical, app486 zero-call" that the design does not cover is a HARD
STOP to the Architect — not a local workaround.
