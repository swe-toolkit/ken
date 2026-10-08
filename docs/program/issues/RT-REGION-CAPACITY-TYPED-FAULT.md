---
id: RT-REGION-CAPACITY-TYPED-FAULT
title: "Emitted code that exhausts a declared invocation-region limit returns UnclassifiedRuntimeTrap { terminal_value: -1 }: require_i64 turns the allocator's BOUNDARY_ERR_CAPACITY into the same bare -1 a miscompile guard returns. Route every capacity status from emitted code to the typed CapacityExhausted terminal, naming the profile and the limit"
status: ready
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary M8 finding evt_4dmtyxqj0a7pa on e7260a1cd (spec/40-runtime/44-capacity.md section 2). A capacity trap that reads as a miscompile also blinds L1's own native-row triage. Steward-filed per COORDINATION section 2."
---

# Region capacity exhaustion is a typed fault

## Objective

Exhausting a declared region limit in emitted code ends in the typed
`CapacityExhausted` terminal with the profile and limit, never in the bare
`-1` trap.

## Settled inputs (Adversary `evt_4dmtyxqj0a7pa`, re-measure on the base)

- **The witness.** `ESCAPE_FILE_THEN_READAT`
  (`crates/ken-cli/tests/rt_escape_second_resource_native.rs:245`) with one
  more constant `readAt` before `Ret`, built with `starter_smoke_profile`
  (invocation node limit 64). The interpreter exits 0 with two reads. Native
  returns `UnclassifiedRuntimeTrap { terminal_value: -1 }`. At limit 65 it
  matches the interpreter on exit, stdout and the full effect-event vector.
- **The site.** The `refs.alloc` status check in `emit_carrier_alloc`
  (`lowering/aggregates.rs:2555`; tag `InvocationAggregate`, class
  `Constructor`) receives `BOUNDARY_ERR_CAPACITY` (-6). `require_i64`
  (`lowering/mod.rs:14146`) returns `-1` for any mismatch.
- **The typed terminal exists.** `ken_host::TerminalErrorV1::CapacityExhausted`
  with `CapacityExhaustedV1` (`object_linker_packaging.rs:394`, `:1568`).
  `RT-INVOCATION-RESOURCE-PRECURSOR` typed only epochs, event generations
  and live pending slots.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (measure only).** The population of `require_i64` and equivalent
   status checks in `lowering/` whose callee can return
   `BOUNDARY_ERR_CAPACITY`, by resource. The Architect rules the route at
   D0.
2. **The ruled repair.** Each capacity status in that population reaches
   the typed terminal with its resource and limit. Other mismatches keep
   their current trap.

## Acceptance

- **AC-1.** The witness at limit 64 ends natively in `CapacityExhausted`
  naming the invocation node resource and limit 64, with the interpreter's
  effect prefix up to the refusal. At limit 65 it keeps full parity.
- **AC-2.** One row per other resource in the D0 population that a checked
  program can reach, each driven to its limit.
- **AC-3 (control).** A non-capacity status mismatch still returns the
  current trap, and `rt_parity_native` is unchanged.
- **AC-4 (mutation, QA).** Mapping the capacity status back to `-1` at the
  witness site reddens AC-1.

## Stop conditions

- The typed terminal needs a host ABI version change.
- Any kernel, trust or spec change.
