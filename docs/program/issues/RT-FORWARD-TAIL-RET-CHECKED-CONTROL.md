---
id: RT-FORWARD-TAIL-RET-CHECKED-CONTROL
title: "Lowering refuses a forward Tail producer-to-Ret edge whose Ret body carries nested checked control ('the active carried frame has no installed strict Ret sink'), while the planner certifies it; installing the sink naively traps or drops an effect. Build the capability so the reached Nat fanout and the escaped-buffer fanning row run natively with interpreter parity"
status: active
owner: runtime
size: M
tier: T1
gate: architect
depends_on: [RT-GENERATED-ENTRY-PROJECTION-INVARIANT]
blocks: []
github: null
origin: "Architect recut evt_52g57s1w4jgc1 of RT-GENERATED-ENTRY-PROJECTION-INVARIANT on hard stop evt_43ff7m0sw9g98: AC-1/AC-2 move here with the capability. Steward-filed per COORDINATION section 2."
---

# Forward Tail into a Ret body that carries checked control

## Objective

The `:616` escaped-buffer fanning row in
`rt_escape_second_resource_native.rs` runs natively with interpreter parity.
The reached Nat fanout fails closed at the carried-match class guard. Its
parity moves to `RT-CARRIED-NAT-MATCH-BOUNDED-IMMEDIATE` (Architect split
`evt_617g7acwvmk4n`).

## Settled inputs (Architect `evt_52g57s1w4jgc1`, on `575baaef4`)

- **Two predicates for one property.** The planner's
  `checked_ih_strict_ret_sink` (`aggregates.rs:6630`) admits any frame with
  one unary `Ret` case and no recursive positions. Lowering installs the
  sink (`lowering/core.rs:15212-15228`) only on exact Ret+Vis topology with
  a Ret body free of checked control markers (PX8-TR, `8d761bc5a`).
- **The failing member.** Frame 716, Ret body 1081: `after_read` inlined
  into the `read_body` bind, carrying 3 checked subcontinuation frames,
  3 IH slots and 3 IH calls.
- **Naive widening miscompiles.** Installing the sink on topology alone
  traps the Nat row (`UnclassifiedRuntimeTrap { terminal_value: -1 }`) and
  drops `FsReadAt` from `:616` (native `[FsOpen, BufferAllocate,
  ResourceRelease, ResourceRelease]` against the interpreter's
  `[FsOpen, BufferAllocate, ResourceRelease, FsReadAt, ResourceRelease]`).
- Both rows enter ignored with their sink refusal quoted, from the
  predecessor.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **D0 (measure only, at the start of the repair).**
   - (a) On `:616` under the widened sink, find the edge that leaves Ret body
     1081/766 without running its nested frames. Use a disposable
     distinct-trap-code probe for the Nat `-1`.
   - (b) Census the Tail plans whose producer frame has a marker-bearing Ret
     body across `rt_parity_native`, `rt_escape_second_resource_native` and
     the PX8 suites, and how many of them lowering actually requests.
   - (c) With (b) known, propose one planning-owned sink predicate that
     replaces both. The Architect rules the repair.
   - (d) Record the current first refusal of `:584`
     (`escaped_resource_used_by_fanning_host_op_matches_interpreter`, the
     escaped `FsHandle` sibling) on the base and under the widened sink. If
     it reaches the same sink refusal, it joins AC-1, AC-2 and AC-4.
     Otherwise report its refusal and keep it out of this repair.
2. **The ruled repair** (`evt_617g7acwvmk4n`): the planning-owned
   `StrictRetSinkAssessment`, the phase B execute-then-resume disjunct, and
   the `FormedBasePath` authority outcome, with the strict sink installed
   only for `Ready`.

## Acceptance

- **AC-1.** The `:616` row is un-ignored and passes on the default thread:
  exit, terminal, stdout, the full `EffectEvent` vector, and one read.
- **AC-2 (Nat fails closed).** The Nat row stays ignored, naming the
  successor and the measured site. One active pin over the three Nat
  variants asserts `UnclassifiedRuntimeTrap { terminal_value: -1 }`.
- **AC-3 (controls).** `rt_parity_native` passes 189/189 on the default
  thread, the read and write collapsibility control passes, and the escape
  file's six active rows pass. A population counter shows the phase B
  disjunct true only for the escape-file target programs.
- **AC-4 (mutation, QA).** The five mutants of `evt_617g7acwvmk4n`
  (M-admission-off, M-base-path-capture, M-ready-forward,
  M-assessment-split, M-marker-free-suppressed) each redden a named active
  row.

## Stop conditions

- The repair needs a kernel, trust or spec change.
- A parity red, a dropped effect, or a trap anywhere except the pinned Nat
  guard.
- The phase B disjunct true outside the target programs, or the
  transport-source population refusal (`construction.rs:1530`).
- Default-thread stack overflow on any un-ignored row.
