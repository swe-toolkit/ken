---
id: RT-PLANNER-PER-EMITTER-AVAILABILITY
title: "The continuation planner finalizes a specialization's capture availability claims only under its interning owner, but the same construct is materialized under other owners (Vis735 under Specialization(2) on all 15 measured emissions), so no lawful capture source exists there. Finalize the W and C claims per emitting owner at every materialization point, recording each unfinalizable ordinal"
status: merged
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
point, or it records that ordinal as unfinalizable with the owner and reason.

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

## AC-0 (Architect `evt_yajvrmbnkkzb`)

Record, do not refuse. Each ordinal yields `Finalized(claim)` or
`Unfinalizable{ordinal, owner, reason}`; the fail-closed refusal belongs to
ENV-CARRIAGE's residual builder at the point of use. One read-only accessor
exposes the results, with no lowering call sites. D0 first: explain the W8
discrepancy, and name the claim-discovery function behind spec 1's
`Predeclared(4)` claims and show it runs for a given owner and origin, or stop
where it is hard-keyed to the interned owner.

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

- **AC-1.** Read through the accessor, spec 1 at Vis735 under
  `Specialization(2)` yields 14 per-ordinal results (W8 + C6).
- **AC-2 (pins).** Per specialization, the number of materialization points.
  Per owner, the finalized and unfinalizable counts. Positional reads and
  record959 reads: 0.
- **AC-3 (control).** Delete one owner's finalization: the census pin goes red.
- **AC-4.** Every interned `ContinuationSpecializationKey`, including its
  `continuation_inputs` and availability drafts, is identical before and
  after on the native census (Architect `evt_ew3tp9b71yaa`: the frame comes
  from a factored `emitter_frame_for_owner`, and interning is
  behavior-identical). No native census verdict changes. `rt_parity_native` and the
  one-bracket and two-bracket native controls stay green, and
  `trusted_base()` is unchanged.

## Stop conditions

- Claim discovery is hard-keyed to the interned owner (D0): stop to the
  Architect, naming where.
- The work needs a residual layout or gate change.

## Closeout

Merged `c95c6a556` (PR #4431), exact `d099701de`: Runtime QA
`evt_7n8bfrbew5rw8`, Architect `evt_6ccq90vgyzbeb`, Decision
`dec_1xh94n58cx7mg`.

- The planner censuses each specialization's materialization points by
  planner-owned owner relations and records, per W and C ordinal, a finalized
  claim or `Unfinalizable { ordinal, owner, reason }`, read through
  `per_emitter_materializations()`.
- Spec 1 at Vis735 under `Specialization(2)` records 14 results (W8 + C6),
  all NoClaim. Interned keys are byte-identical to the base.
- **Carry to `RT-NATIVE-CONTINUATION-ENV-CARRIAGE`** (Architect
  `evt_6ccq90vgyzbeb`): its residual builder consumes these results and owns
  the fail-closed refusal; the 14/14 NoClaim at Vis735 needs its design
  answer.
