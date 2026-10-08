---
id: RT-CARRIED-NAT-PREDECESSOR-RE-ELIMINATION
title: "In a carried bounded-immediate Nat match, a nested match on the Suc arm's predecessor reaches merge_scalar_branch on a join the planner planned as a carrier word, so the native build fails at ObjectEmission with the internal 'carrier-result join reached a native-only scalar merge consumer' instead of running at parity or refusing typed. Make the planned representation and the reached consumer agree"
status: ready
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary M8 finding evt_2rjw2bf3e4r8d on e18f9a2cc (RT-CARRIED-NAT-MATCH-BOUNDED-IMMEDIATE). Fails closed at build time; before that merge the same rows trapped -1 at run time. Steward-filed per COORDINATION section 2."
---

# A carried Nat predecessor can be matched again

## Objective

A nested match on the predecessor bound by a carried bounded-immediate
`Suc m` arm runs with interpreter parity natively, or is refused at
admission with a typed `unsupported`. It never reaches the backend's
internal planner/lowering disagreement.

## Settled inputs (Adversary `evt_2rjw2bf3e4r8d`, at `e18f9a2cc`)

- **The witness.** The shape of `rt_nat_fanout_reached_live_resource.ken`:
  one live `readAt` at window (0,6), then `ReadSome span count`, then
  `bind (match buffer_span_budget span { Zero |-> Ret ..; Suc m |-> BODY })
  (\_. Ret ..)` with BODY `match m { Zero |-> readAt ..; Suc q |-> Ret .. }`.
  The interpreter gives reads=2 at offset 12 (budget 1) and reads=1 at
  offset 0 (budget 6), both exit 0. Native `build_native_program` fails at
  ObjectEmission: "carrier-result join reached a native-only scalar merge
  consumer". A two-deep nested match on `m` then `q` fails the same way.
- **The mechanism.** `joins.rs:1709` binds the predecessor as
  `Lowered::BoundedNat`, and the arm lowers through
  `lower_computational_producer_expr` with the composed suffix
  (`:1716-1719`). The nested frame reaches `lower_bounded_nat_computational`
  (`core.rs:7636`), whose `merge_scalar_branch` (`:7747`, `:7875`) needs a
  `NativeScalarPair` join. The planner planned it as a carrier word, so
  `joins.rs:2528-2531` returns `backend_module`.
- **The control.** The single-level rows over `buffer_span_budget`,
  `transfer_count_remaining` (0 and 3) and `transfer_count_predecessor`
  (0 and 5) are at parity. The specialized route handles px8n's exact-Nat
  Suc-then-Zero (`RT-CARRIED-NAT-MATCH-BOUNDED-IMMEDIATE-D0.md`).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (measure only).** Where the planner fixes the join representation
   for the nested frame, and what it would need to see to plan a
   `NativeScalarPair` (or the composed carrier route) there. The Architect
   rules at D0: parity, or a typed admission refusal.
2. **The ruled repair.**

## Acceptance

- **AC-1.** Both nested witnesses (one-deep and two-deep) build and match
  the interpreter on exit, stdout and the full `EffectEvent` vector at
  budgets 1 and 6. If the ruling is a refusal instead, each is refused at
  admission with a typed reason naming the nested predecessor match.
- **AC-2 (control).** The seven single-level rows above and the
  `rt_escape_second_resource_native` Nat fanout row keep their results;
  `rt_parity_native` is unchanged in CI.
- **AC-3 (mutation, QA).** Reverting the repair returns the witnesses to
  the ObjectEmission failure.

## Stop conditions

- The repair needs a kernel, trust or spec change.
- A dropped or reordered effect on any row.
