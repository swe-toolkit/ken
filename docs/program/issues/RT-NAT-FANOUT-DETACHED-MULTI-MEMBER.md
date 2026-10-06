---
id: RT-NAT-FANOUT-DETACHED-MULTI-MEMBER
title: "After the scalar-admission retry forces source Match 1289 to CarrierWord, the Nat fanout escaped-resource row refuses at object emission: the detached-result seat sees 4 undischarged causal calls on one unit result, a multi-member projection it rejects by design. Find why 4 edges reach that seat, and repair the upstream discharge so the row reaches native/interpreter parity"
status: active
owner: runtime
size: M
tier: T1
gate: architect
depends_on: [RT-JOIN-SCALAR-PAIR-NONSCALAR-RESULT]
blocks: []
github: null
origin: "The successor recorded by RT-JOIN-SCALAR-PAIR-NONSCALAR-RESULT AC-1 on the ignore annotation of nat_fanout_escaped_resource_matches_interpreter (merged 3695cd16a). L1 objective: clear the selected ignored runtime rows. Steward-filed per COORDINATION section 2."
---

# The Nat fanout row reaches parity

## Objective

`nat_fanout_escaped_resource_matches_interpreter`
(`crates/ken-cli/tests/rt_escape_second_resource_native.rs:644`) runs natively
with the interpreter's result and is un-ignored.

## Settled inputs (ignore annotation `:643`, on `3695cd16a`)

- **The route.** The bounded retry from SCALAR-PAIR forces source Match 1289
  to `CarrierWord`. Object emission then refuses with
  `ContinuationSpecialization`: "the detached-result seat projected 4
  undischarged causal calls onto one unit result".
- **The refusal is correct.** It is the multi-member guard of
  `eliminate_detached_producer_continuation`
  (`lowering/core.rs`, the `residual.as_slice()` match near `:12093`). One
  result value cannot discharge two causal calls, and choosing one would make
  lowering the authority for a planner fact. The guard stays.
- **The fixture.** `rt_nat_fanout_escaped_resource.ken`: an escaped
  resource's checked frame in the shared continuation of a `match n {Zero;
  Suc}` fanout, where several arms reach `second_read`.
- **Unmeasured:** which 4 edges are residual, their construct origins, and
  why none is in `composed_discharges` or claimed.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## R7 ruling (Architect `evt_6pzevh7vt0brk`, on `4177e68de`)

The ruled designs R1-R4' are withdrawn. This WP does not share
`RT-CARRIED-CHILD-REFERENT-CONTAINMENT`'s planner rule, and neither WP
blocks the other.

- **The defect.** Lowering replaces a claimed producer Construct with the
  planned continuation call's result, a Carried word
  (`core.rs:7536-7561`, `calls.rs:2602`). The planner never sees the
  replacement: Construct1227 is summarized `SpecializedOnly`, the F819 arm
  binders (Var1187, 1180, 1175, 1173) copy that summary, and Match1176 gets
  a NativeScalarPair token while receiving a Carried operand. The sentinel
  (Construct378/349, Match391) has the same shape.
- **Fan-in.** Both replacement seats (`core.rs:7490` fused outer
  realization, `:7520` claim) select only through
  `continuation_call_binding_for` over `continuation_calls()`, which is
  planned before `build_join_result_plan`.

## Deliverable

R6 and R7 together; the ruling's code is the text.

- **R7.** `is_continuation_call_producer(origin)` beside
  `continuation_call_binding_for`, and the Construct arm of
  `summarize_result_phase` raised to `CarrierRequired` for such a producer
  under `functionized_units` when the result continues.
- **R6.** The disposable probe's carried-suffix re-entry at
  `lowering/core.rs:6571`, ungated, bounded by
  `CARRIED_SUFFIX_REENTRY_LIMIT`, with no trace.

## Acceptance

- **AC-1 (census, base+R6 against base+R6+R7).** Every source flip is a
  Construct with `is_continuation_call_producer` true, counted as distinct
  origins; every other change descends from one. Rows 1227, 1328, 1187,
  1180, 1175, 1173 and 1176 go to CarrierRequired, and join 1176 to
  CarrierWord. Aggregate parent-owner changes are reported.
- **AC-2 (rows and guards).** The Nat row
  `nat_fanout_escaped_resource_matches_interpreter` is un-ignored and
  passes with interpreter parity, and so does the sentinel row. Runtime
  lib 1014/1014, native-tree 8/8, and the second-resource active rows stay
  green. The NativeScalarPair-with-Carried refusal and the R6 re-entry
  refusal fire zero times. The D5a guard mutations stay red. The full
  `rt_parity_native` suite is CI's.
- **AC-3 (mutants, QA).** M1: deleting the R7 clause brings back the exact
  1176 refusal on the Nat row. M2: dropping `functionized_units &&` is
  measured and its AC-1 delta recorded.

## Stop conditions

- A predicted row stays `SpecializedOnly`, a CarrierWord join becomes
  NativeScalarPair, or a flipped producer's claim returns a Specialized
  `Closure` (`calls.rs:2592`): stop to the Architect.
- A new first refusal on the Nat row is an advancing stop: bring it back
  with the same evidence shape.
- A repair that weakens, bypasses or orders the multi-member guard.
- After this repair the row refuses on a different, independent mechanism:
  record that refusal on the ignore annotation as the successor, and stop.
- Any kernel, trust or spec change.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`, and never land `a7d46d6f2`.
