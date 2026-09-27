---
id: RT-SOURCE-IH-RELAY-K-VALUE
title: "Give the source machine a value for a relay Vis whose K is a zero-argument functional-IH reference, so the owner return protocol can carry it and clear r2; first settle whether ConstructArgument's backedge propagation is sound for such a constructor field"
status: ready
owner: runtime
size: M
gate: architect
tier: T1
depends_on: [RT-OWNER-VIS-RETURN-PROTOCOL]
blocks: []
github: null
origin: "Architect 2026-09-27 (evt_1j5qaw2d9sqe7): r2 descoped from RT-OWNER-VIS-RETURN-PROTOCOL under its second-independent-repair clause; successor requested with this evidence. Operator L1 directive 2026-09-17 (clear the ignored tests). Steward-filed per COORDINATION section 2."
---

# A K value for the source-machine IH relay

## Objective

Clear `rt_escape_second_resource_native.rs`
`r2_cross_buffer_freeze_fails_closed_with_invalid_bounds` (owner Vis 1298)
by giving its relay K a value the owner return protocol can carry.

## Settled inputs (Architect `evt_1j5qaw2d9sqe7`; probe `3d2647e6c` on `b1b358bb8`)

- **The wall.** In the source machine, the relay Vis577's K operand is
  `CheckedComputationalIHInvocation { body: Call { callee: Var(0), args: [] } }`
  (template 3, `OrdinaryApplication`; construct 577 in
  funcid61/ContinuationContext(0)/Specialization(3)). It lowers to
  `Specialized(RecursiveBackedge)`, because `ConstructArgument`
  (`source.rs:1643-1655`) propagates a backedge past the constructor. No K
  value exists to put in a pending-Vis record.
- **Three IH marker arms:** `core.rs:3574`, `core.rs:15382`, `source.rs:829`.
- **The protocol excludes 1298 today.** An owner whose fixpoint contains a
  relay member fails closed to the Ret-tag trap (the owner protocol's AC-3
  pin), so this node starts from that exact failure.

## Deliverable

A representation for a functional-IH reference in a non-tail constructor
field on the source-machine route, designed at D0 and ruled by the
Architect, that lets owner 1298 join the return protocol and un-ignores r2.

## Acceptance

- **AC-0a (soundness first; read plus a disposable probe).** Is
  `ConstructArgument`'s backedge propagation sound when the operand is a
  functional-IH reference in a constructor field, which denotes a function
  under λ rather than a call executed now? Measure whether that construct is
  reachable at runtime on this route. If it is, show whether the jump runs
  the recursion without the response `r`. If it is unsound and reachable, stop
  to the Architect: that is a correctness repair ahead of the representation.
  **Answered, then withdrawn by measurement.** The Architect first ruled a
  soundness stop from syntax position (`evt_2b0dwgwyb0tvc`). The census
  under `RT-IH-BACKEDGE-FAIL-CLOSED` then placed r2 inside 313 lawful
  arrivals of the identical tuple, and that node closed with no change
  (`evt_6x4nk9x3pe0r`). No correctness repair precedes this node; r2 is a
  representation question only.
- **AC-0b (D0).** The ring proposes the relay K value. The Architect rules
  before any build.
- **AC-1.** r2 runs green, un-ignored, with the full 42 §6.4 envelope and the
  terminal result agreeing on both engines.
- **AC-2 (control).** Removing the representation returns r2 to its
  exclusion trap.
- **AC-3.** No other row changes colour. Targeted suites only, through
  `scripts/ken-cargo`.

## Stop conditions

- Any kernel, `trusted_base()` or spec change (an operator question).
- A design that reopens the withheld closure lane (`dec_21aa95jbsznfh`,
  `dec_6xffebwj4s347`) is an Architect stop.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.
