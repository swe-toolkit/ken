---
id: RT-NATIVE-VOCAB-STRUCTURAL-COMPLETENESS
title: "Conjunct-(2) completeness in dead-arm effect lowering rests on NativeProcessSymbols mirroring the runtime's minting sites BY CONVENTION, not by structure -- make it structural so a newly-minted runtime constructor cannot silently create a wrongly-trapped live arm"
status: draft
owner: runtime
size: S
gate: none
depends_on: [RT-DEAD-ARM-EFFECT-LOWERING]
blocks: []
github: null
origin: "Architect recommendation on the RT-DEAD-ARM-EFFECT-LOWERING D1 soundness review (evt_1qnc66xke540m, carry) and ownership clarification (evt_11jf2ywp5drpy, thr_3r6wv5net6s61, 2026-08-22): the follow-up is a node, sizing and cut are the Steward's. Steward-filed per COORDINATION section 2."
---

# STATUS: QUEUED, NON-PRIORITY, NOT RELEASED

Fail-closed-sound today. The Architect explicitly ruled this can queue
behind lane-1 priority work indefinitely. It is captured here so the
recommendation is durable (an in-thread ruling is not); it is NOT next, and it
is not released.

# WHAT THIS NODE IS

A hardening of the [[RT-DEAD-ARM-EFFECT-LOWERING]] deadness predicate's conjunct
`(2)` (not-runtime-producible). Conjunct `(2)` is grounded on
`NativeProcessSymbols`, which is destructured with NO rest pattern -- so a
symbol that IS a field but is left unclassified is a COMPILE error. That
guarantees you cannot forget to CLASSIFY a field; it does NOT guarantee the
field SET equals the runtime's actually-minted constructor set.

The minting sites (`native_process_entrypoint.rs`, `platform_runtime_support.rs`)
build values from module-level constructor consts directly, and
`legacy_prelude()` populates the struct from those same consts -- but nothing
STRUCTURALLY forces a newly-minted runtime constructor to become a
`NativeProcessSymbols` field. Completeness holds today by CONVENTION only.

# WHY IT IS SOUND TODAY (so it can wait)

Fail-closed per the RT-DEAD-ARM ruling property (ii): a runtime constructor
missing from the struct is NOT unioned in as LIVE, so its arm could be proven
dead and trapped -- but the substitute is a TRAP, so such a program HALTS, it
never miscompiles and never relaxes a capability gate. The cost of the gap is
a wrongly-trapped LIVE arm (a broken working program that halts), which is a
liveness/regression risk, not a soundness hole. That is why it is a follow-up,
not a blocker on RT-DEAD-ARM.

# THE RULED DESIGN (Architect recommendation) -- pick one at framing

Make runtime-vocabulary completeness STRUCTURAL, so a future runtime
constructor cannot silently create a wrongly-trapped live arm. Two shapes the
Architect named; the node picks one:

- Route the runtime minting sites THROUGH `NativeProcessSymbols` (the struct
  becomes the single source of the minted set), or
- Add a test asserting every module-level constructor const appears as a
  `NativeProcessSymbols` field (a cheaper structural pin that reds when the sets
  diverge).

# ACCEPTANCE

- **AC-1 (structural completeness).** A newly-minted runtime constructor cannot
  be absent from the conjunct-`(2)` LIVE set without a compile error or a red
  test -- state which mechanism and why it is exhaustive.
- **AC-2 (no behaviour change).** No currently-compiling lowering changes; this
  only closes the convention gap. Workspace-green in CI.
- **Required reviewer:** the Architect (soundness-adjacent completeness of the
  dead-arm predicate).

# CONTENTION

`ken-runtime` (`NativeProcessSymbols` and the minting sites, or a new test).
Depends on [[RT-DEAD-ARM-EFFECT-LOWERING]] (the predicate it hardens), which is
merged. No urgency; queues behind all lane-1 priority work.

# CAPABILITY TIER

T2-leaning: the design is front-loaded to two named shapes and the property is
a structural completeness check, not a novel soundness argument. The Architect
reviews. Size S.
