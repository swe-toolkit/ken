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

## Deliverable

1. **AC-0, at the start of the repair.** On main, run the row under
   `RT_TREE_DETACHED_CENSUS` and trace the 4 residual edges: identity,
   construct origin, owning arm, and the upstream site that should have
   discharged or claimed each. The Architect rules the repair.
2. **The ruled repair**, upstream of the detached-result seat. The planner
   stays the single authority on representation and discharge.

## Acceptance

- **AC-1.** The row is un-ignored and passes: native matches the
  interpreter. Its failure on main before the repair is the current
  refusal.
- **AC-2 (controls).**
  - The five D5a guard mutations
    (`d5a_the_detached_result_seats_five_guards_are_each_reached_by_a_real_mutation`)
    each stay red. `DuplicateResidualEdge` still refuses.
  - The scalar-join feedback tests (`with_scalar_join_feedback_attempts`,
    `cranelift_backend/artifact/tests.rs`) keep their attempt counts.
  - `rt_native_tree_match_case_of_case` and `rt_escape_second_resource_native`
    keep their default results. The full `rt_parity_native` suite is CI's
    and is not run locally; touched parity rows are run by name.
- **AC-3 (falsifier).** Reverting the repair brings back the verbatim
  refusal on the row.

## Stop conditions

- A repair that weakens, bypasses or orders the multi-member guard.
- After this repair the row refuses on a different, independent mechanism:
  record that refusal on the ignore annotation as the successor, and stop.
- Any kernel, trust or spec change.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`, and never land `a7d46d6f2`.
