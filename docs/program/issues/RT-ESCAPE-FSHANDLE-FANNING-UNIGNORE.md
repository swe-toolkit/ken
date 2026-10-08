---
id: RT-ESCAPE-FSHANDLE-FANNING-UNIGNORE
title: "The ignored escaped-FsHandle fanning row (rt_escape_second_resource_native.rs:584) no longer refuses: on landed main it passes native-versus-interpreter parity under a 256 MiB stack, so its BoundaryCarrier ignore is stale. Un-ignore it with a measured stack provision and full parity"
status: active
owner: runtime
size: S
tier: T2
gate: architect
depends_on: []
blocks: []
github: null
origin: "RT-FORWARD-TAIL-RET-CHECKED-CONTROL D0(d) (report 18a514459, runtime-implementer evt_1zrecsttqb4sm): :584 has no first refusal on uninstrumented 61661201b and forms no Tail, so it stays out of that WP (Architect evt_5zrkbed31xn9j). Serves the L1 objective (operator 2026-09-17). Steward-filed per COORDINATION section 2."
---

# Un-ignore the escaped-FsHandle fanning row

## Objective

`escaped_resource_used_by_fanning_host_op_matches_interpreter`
(`crates/ken-cli/tests/rt_escape_second_resource_native.rs:584`) runs and
passes, and its ignore is gone.

## Settled inputs (D0(d) of `RT-FORWARD-TAIL-RET-CHECKED-CONTROL`, on `61661201b`)

- The row's ignore quotes a `BoundaryCarrier` refusal at continuation origin
  329. On landed main the row reaches no refusal. With a disposable 256 MiB
  builder thread it passes the existing differential, and a separate check
  matched stdout and the full `EffectEvent` vector. It forms and requests no
  Tail plan.
- Its comment records an overflow of the libtest default thread at
  `RT-NATIVE-TREE-MATCH` D1. D0(d) claims no default-stack adequacy.
- Sibling rows in the same file run in `in_large_stack_thread` (256 MiB),
  with the stated "no default-stack claim" precedent at `:470`.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

Remove the ignore and its stale refusal comment. Run the row on the default
test thread first. If it overflows, run it in the existing
`in_large_stack_thread` helper, and record the measured result that
justifies that provision in the row's comment. No production change.

## Acceptance

- **AC-1.** The row passes native-versus-interpreter parity on stdout and
  the full `EffectEvent` vector, on whichever thread the deliverable
  measured.
- **AC-2 (controls).** The other rows in that file keep their results.
- **AC-3 (mutation, QA).** A scratch edit that drops the last native
  `EffectEvent` before the comparison reddens the row, so its pass compares
  the full vector and is not a vacuous run. Restore byte-identically.

## Stop conditions

- Any refusal or trap on landed main, which contradicts D0(d).
- An overflow at 256 MiB, or any need for a production change.
