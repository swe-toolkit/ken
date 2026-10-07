---
id: RT-GENERATED-ENTRY-PROJECTION-INVARIANT
title: "Native build refuses with 'source-specific inheritances at one generated entry disagree on their typed consumer projection' (aggregates.rs:8215) on the ignored escaped-buffer fanning row and on a reached Nat fanout with a pending suffix, so the NAT-FANOUT parity row only passes because its fanout never runs. Repair the projection so both execute natively with interpreter parity"
status: active
owner: runtime
size: M
tier: T1
gate: architect
depends_on: [RT-NAT-FANOUT-DETACHED-MULTI-MEMBER]
blocks: []
github: null
origin: "L1 roster item 'rt_escape :691 (generated-entry projection invariant), unframed'. Adversary finding evt_6dy48x09rfktx on 21defbec2: the un-ignored nat_fanout row never reaches its Nat fanout, and the reached shape refuses at this invariant. Steward-filed per COORDINATION section 2."
---

# Generated-entry projections agree, and the Nat fanout runs natively

## Objective

Each member of a generated-entry class reads its own forward-Ret Tail plan,
so the projection-disagreement refusal is gone. Recut by the Architect
(`evt_52g57s1w4jgc1`): the reached Nat fanout and `:616` rows stay ignored
at their next, honest refusal. `RT-FORWARD-TAIL-RET-CHECKED-CONTROL` owns
their parity.

## Settled inputs (Adversary `evt_6dy48x09rfktx`, on `21defbec2`)

- **The vacuous row.** `nat_fanout_escaped_resource_matches_interpreter`
  (`rt_escape_second_resource_native.rs:635`) reads through `file_closed`,
  the handle that escaped after its bracket released it. The interpreter
  trace is `FsOpen ok, ResourceRelease ok, BufferAllocate ok, FsReadAt
  Error(Resource(Closed)), ResourceRelease`, so `after_read` takes
  `Err` and the `match buffer_span_budget span { Zero; Suc }` fanout
  (`rt_nat_fanout_escaped_resource.ken:25`) never runs. Variants that make
  either arm read again keep `parity=true` with an identical trace.
- **The reached shape.** A `withResource` body that calls
  `withBuffer (read_body file)` on the live handle runs the fanout in the
  interpreter (2 `FsReadAt`, 3 when the Suc arm reads again). All four
  variants refuse at native build with the message above, at
  `aggregates.rs:8215`.
- **The ignored row.** `escaped_buffer_used_by_fanning_host_op_matches_interpreter`
  (`:616`) is ignored with the same refusal (AC-0 at `310bf4f21`).
- **Why R6 is involved.** R6 passes `Some(eliminators)` at
  `lowering/core.rs:6584`, and `lower_carried_match` disables the exact
  BoundedNat/StructuralNat adapter when the suffix is nonempty
  (`lowering/joins.rs:514`). A Zero/Suc producer match with a pending suffix
  therefore takes the generic constructor dispatcher. No active row
  executes that route on a Nat scrutinee.
- `RT-SITEOP-RETAINED-ROWS-ADVANCED-PAST-LABEL` and
  `RT-SITEOP-ORPHANED-ROWS-REFUSAL-CENSUS` track this refusal as their row
  10. This node owns its repair. Their remaining rows stay with them.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **D0 (measure only, at the start of the repair).** Add the reached Nat
   fanout as an ignored row, with arm-sensitive variants. For it and the
   `:616` row, record which inheritances disagree at `aggregates.rs:8215`:
   owners, generated entry, and each side's typed consumer projection.
   Census every other test that pins this refusal text
   (`rt_parity_native.rs:3815-3847`). The Architect rules the repair.
2. **The ruled repair** (`evt_25wwm2j6e35nm`, kept by `evt_52g57s1w4jgc1`):
   class-common route in the projection, a per-member Tail plan beside each
   member, re-derived from the member's own transport. Neither sink
   predicate changes.

## Acceptance

- **AC-1.** Both rows stay `#[ignore]`, each reason quoting its new first
  refusal verbatim, at `ComposedReturnRetSink` /
  `ComposedReturnForwardRetAuthority`. Any other first refusal is a stop.
- **AC-2 (pins).** Class-common pins keep the projection-disagreement
  message. Tail-member pins retarget to "a member's Tail plan disagrees with
  its transport's own derivation". `route_disagreement` becomes the
  route-kind flip.
- **AC-3 (controls).** The other rows in that file, `rt_parity_native` at
  4 threads, and the refusal pins the D0 census names keep their results,
  unless the ruling names a change.
- **AC-4 (mutation, QA).** M-class-tail restores the projection refusal
  on both rows, ahead of the sink refusal. M-wrong-member fails at the
  re-derivation check.

## Stop conditions

- The repair needs a kernel, trust or spec change.
- A sink predicate would need to change, or a refusal would move into the
  planner.
- Default-thread stack overflow on any un-ignored row.

## Symptom inventory

```text
SYMPTOM INVENTORY (append one line per hard-stop; never rewrite history)
1. planner certifies a forward-Ret Tail route into a Ret body that lowering never sinks — keyed on Ret-body checked-control freedom
```
