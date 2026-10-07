---
id: RT-FORWARD-TAIL-RET-CHECKED-CONTROL
title: "Lowering refuses a forward Tail producer-to-Ret edge whose Ret body carries nested checked control ('the active carried frame has no installed strict Ret sink'), while the planner certifies it; installing the sink naively traps or drops an effect. Build the capability so the reached Nat fanout and the escaped-buffer fanning row run natively with interpreter parity"
status: ready
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

The reached Nat fanout and the `:616` escaped-buffer fanning row in
`rt_escape_second_resource_native.rs` run natively with interpreter parity.
The NAT-FANOUT parity claim becomes measured rather than vacuous.

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
2. **The ruled repair.**

## Acceptance

- **AC-1.** Both rows are un-ignored and pass native-versus-interpreter
  parity on stdout and the full `EffectEvent` vector.
- **AC-2 (discriminates arms).** A variant whose Suc arm reads once more
  records one more `FsReadAt` in both engines (3 against 2), and the row
  tells it apart from the as-written arms.
- **AC-3 (controls).** The other rows in that file, `rt_parity_native` at
  4 threads, and the D0 (b) population keep their results, unless the
  ruling names a change.
- **AC-4 (mutation, QA).** Restoring the marker gate on the sink returns
  both rows to the sink refusal.

## Stop conditions

- The repair needs a kernel, trust or spec change.
- A runtime trap or a dropped effect on any row after the repair.
- Default-thread stack overflow on any un-ignored row.
