---
id: RT-UNIT-FAILURE-STATUS-PROVENANCE
title: "Reporter honesty successor — an internal generated-unit failure status (e.g. emit_carrier_dynamic_constructor's residual MALFORMED_DYNAMIC_CONSTRUCTOR_STATUS = -3) reaches the process reporter, which globally classifies bare -3 as 'malformed ExitCode::Failure payload'; the unit failure must carry an origin/kind the reporter can classify rather than borrowing a globally interpreted scalar"
status: draft
owner: runtime
size: S
gate: none
depends_on: [RT-DYNAMIC-CONSTRUCTOR-DISPATCH-PROVENANCE]
blocks: []
github: null
origin: "Steward, 2026-08-25, from the Architect hard-stop #3 ruling (evt_1vhmndq7fscd1, thr_305pn5gzx37h). Split OUT of the causal dynamic-constructor dispatch object as an independently proven honesty defect: the process reporter's 'malformed ExitCode::Failure payload' label is a sentinel alias for a bare -3 returned by a generated unit, not an actual ExitCode failure. Sequenced AFTER RT-DYNAMIC-CONSTRUCTOR-DISPATCH-PROVENANCE; must NOT be folded into the dispatch repair or widen it. New node, Steward framing call per COORDINATION section 2."
---

> # Reporter honesty successor — DRAFT stub, held behind the causal dispatch object
>
> Split from the dynamic-dispatch investigation by the Architect (hard-stop #3,
> evt_1vhmndq7fscd1). The `-3` reporter alias is an independently proven honesty
> defect, but it is NOT the causal dispatch defect and must NOT be folded into
> [[RT-DYNAMIC-CONSTRUCTOR-DISPATCH-PROVENANCE]] nor widen it: that object does
> not touch the residual classification, and this object does not touch dispatch
> provenance. Frame this after the causal dispatch object lands.

## Objective (Architect ruling evt_1vhmndq7fscd1)

`MALFORMED_DYNAMIC_CONSTRUCTOR_STATUS == -3` currently reaches the process
reporter, which classifies bare `-3` as "malformed ExitCode::Failure payload."
That classification is a sentinel alias: the selected path never produced an
ExitCode failure — a generated unit returned an internal dynamic-constructor
mismatch scalar directly, forwarded unchanged by `call_declared_unit_target`. An
internal generated-unit failure must carry an origin/kind the reporter can
classify, rather than borrowing a globally interpreted scalar.

## Design boundary (fixed input; do not pre-authorize)

The future design must use the existing typed trap/failure authority (or one
subsuming envelope), NOT allocate another uncoordinated sentinel. This is the same
predicate that produced the three ExitCode hard stops — a downstream semantic
classification (the reporter's `-3` label) standing in for upstream producer
identity — so the fix is provenance-carrying, not a new magic number.

## Sequencing

Draft. `depends_on` [[RT-DYNAMIC-CONSTRUCTOR-DISPATCH-PROVENANCE]] — it does not
block that object's D0-P0/D0-P1 probes, and it is framed + released only after the
causal dispatch object is resolved. Steward owns lane order.
