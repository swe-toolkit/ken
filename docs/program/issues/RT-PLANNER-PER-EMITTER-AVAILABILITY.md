---
id: RT-PLANNER-PER-EMITTER-AVAILABILITY
title: "The continuation planner finalizes a specialization's capture availability claims only under its interning owner, but the same construct is materialized under other owners (Vis735 under Specialization(2) on all 15 measured emissions), so no lawful capture source exists there. Finalize the W and C claims per emitting owner at every materialization point, fail-closed"
status: active
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: [RT-NATIVE-CONTINUATION-ENV-CARRIAGE]
github: null
origin: "Architect evt_3yp3tea99hfta on RT-NATIVE-CONTINUATION-ENV-CARRIAGE rule-4 D0 evt_3m7c6g5hrkxwd (§1a 4): the WP is mis-sized, not off-design; this planner capability is its precursor. Steward-filed per COORDINATION section 2."
---

# Capture availability is finalized per emitter

## Objective

At every point where a continuation specialization's construct is
materialized, the planner holds one finalized availability claim for each of
its W worker and C context capture ordinals, under the owner that emits that
point, or it refuses naming the ordinal and owner.

## Settled inputs (Architect `evt_3yp3tea99hfta`, measured at WIP `86b0ebb9b`)

- **The only lawful capture authority** at a materialization point is a
  finalized availability claim for that ordinal under that point's emitting
  owner. Physical `producer_env` positions and the W-only force environment
  (record959, `CheckedIhCapturedEnvironment` under `Specialization(1)`) are
  not claims.
- **Today** `finalize_continuation_availability_plan` finalizes claims only
  under the interned owner. Spec 1's are `EntryFrame(Predeclared(4), 0..5)` and
  `CurrentLexical(Predeclared(4), env1004, 2..7)`. None exist for
  `Specialization(2)`, where Vis735 is materialized.
- **Measured materialization points:** Vis1014 under `Predeclared(5)`, Vis515
  under `Specialization(2)`, and the Vis735 checked-IH transport destination
  under `Specialization(2)` (15 emissions).

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## AC-0 (Architect, at kickoff)

No residual consumes these claims until ENV-CARRIAGE resumes. The Architect
rules whether an unfinalizable ordinal refuses at planning in this WP, or is
recorded for the consumer to refuse, so that no program accepted today is
newly refused here.

## Deliverable

1. **Census.** For each specialization S, enumerate its materialization
   points as (emitting owner, kind), where kind is a construct emission or a
   checked-IH transport destination.
2. **Finalization.** For each point, finalize one claim per W and C ordinal
   of S under that owner, by the existing mechanism: that owner's own entry
   or context slot, else the single creator-local `direct_emission` at that
   emission origin under that owner.
3. **No positional mapping.** No claim is derived from an env position or a
   closure's field position.

No residual layout, gate or lowering consumer changes.

## Acceptance

- **AC-1.** The direct-application fixture finalizes 14 claims (W8 + C6) for
  spec 1 under `Specialization(2)`, or the planner refuses naming the ordinal
  and owner. The D0 first explains the measured "W8 (seven carried
  captures)" discrepancy at the Vis735 destination.
- **AC-2 (pins).** Per specialization, the number of materialization points.
  Per owner, the number of finalized claims. The number of positional reads is
  0.
- **AC-3 (control).** Delete one owner's finalization: the census pin goes red
  or the planner refuses naming that owner.
- **AC-4.** No native census verdict changes. `rt_parity_native` and the
  one-bracket and two-bracket native controls stay green, and
  `trusted_base()` is unchanged.

## Stop conditions

- An ordinal cannot be finalized under some emitting owner on the census by
  either existing mechanism: stop to the Architect, naming it.
- The work needs a residual layout or gate change.
