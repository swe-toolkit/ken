---
id: RT-NATIVE-CONTINUATION-DUPLICATION-BUDGET
title: "Native lowering emits the continuation after a carried match once per arm, with no join point, so a body under N enclosing match paths is emitted N times: PX11 Inc2's exchange lowers the bracket body under ComputationalMatch 706 18 times inside one function, and the copies compound with nesting. Share a continuation across carried-match arms as a join point that takes the arm-specific facts in carried form, and duplicate only below a size budget"
status: ready
owner: runtime
size: L
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect shape ruling evt_18xe7x3rjaqj5 (Ruling 4) on RT-NATIVE-AGGREGATE-DATA-NOT-CODE, after AC-0 evt_2ry7fef3x4b9r measured the 18 path copies and research advisory evt_5x0v73wpgth68 (join points). L1 Linux ABI program, native parity. Steward-filed per COORDINATION section 2."
---

# A continuation is shared across match arms

## Objective

The continuation after a carried match is emitted once, as a join point
that takes the arm-specific facts in carried form. It is duplicated into
an arm only when the duplicate is below a stated size budget. So the code
for a body under N enclosing match paths does not grow as N copies.

## Settled inputs (Architect AC-0 and shape ruling; anchors re-checked at release)

- **The measurement.** ComputationalMatch 706, the bracket in main's
  `withResource` body, is lowered 18 times inside F = `u2:59` on PX11 Inc2
  WIP `34cc2134f`, all at nesting depth 0. The 18 copies have distinct
  Rust call chains that differ only in the enclosing `lower_carried_match`,
  `lower_carried_constructor_match` and `lower_carried_bool_match` frames.
  Each copy is 53,528 to 54,028 values.
- **The seats.** `lowering/joins.rs`: `lower_carried_match` (`:511`),
  `lower_carried_bool_match` (`:1029`) and
  `lower_carried_constructor_match` (`:1143`) on main.
- **Relation to the sibling.** `RT-NATIVE-AGGREGATE-DATA-NOT-CODE` moves the
  transfers out of each copy. The Architect's arithmetic puts each copy at
  about 16k values afterwards and F at about 290k, under that WP's 400,000
  bound. Duplication still compounds with nesting, which is why this node
  exists. It does not block PX11 Inc2 unless the sibling's H-dup arm fires.
- **Prior art.** Research `evt_5x0v73wpgth68`: join points (Maurer et al.,
  PLDI 2017) share a continuation across arms, and tail duplication runs
  under a size budget.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The carried-match lowering in `joins.rs` and the continuation it emits per
arm. Out of scope: the transfer seat (the sibling WP), the function
partition, and the FNSPLIT activation-identity closure, which stays as
proved.

## Acceptance

- **AC-0 (Architect, before code).** Rule:
  - which arm-specific static facts, such as the selected constructor or a
    refined type, flow into a shared continuation, and their carried form;
  - the duplication budget and the unit it is measured in;
  - the invariant that keeps ownership and region obligations from
    crossing the join point unannounced;
  - the base: whether this waits on the sibling.
- **AC-1, size.** On the exchange's native build, ComputationalMatch 706's
  body is emitted once, or at most as many times as the budget admits, and
  the report gives the count.
- **AC-2, behavior.** Paired rows built and run natively and run in the
  interpreter, through a match whose arms carry different static facts
  into the shared continuation. The rows differ only in the selected arm,
  and the two lanes agree on each.
- **AC-3, mutation.** Force per-arm duplication. AC-1's count returns to the
  per-path figure.
- **AC-4.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A fact the continuation needs cannot be passed in carried form.
- The join point would carry an ownership or region obligation across the
  arm boundary that AC-0 did not rule.
- The repair needs a change to the function partition or the FNSPLIT
  closure.

Checks: 4 fired. I read the three carried-match lowerings on main. 6
fired. AC-0 measured 18 copies of one body in F, which is the population
this node addresses. 8 fired. AC-3 restores per-arm duplication, so AC-1's
count separates a shared continuation from the current lowering.
