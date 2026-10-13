---
id: RT-PENDING-CALL-ERR-PAYLOAD-ADMISSION
title: "Pending-call admission refuses the px7m dynamic-error route at (J): J-a's accounting is deferred, and response-owner emissions lower their own copy of the selected leaf's joins with no closeout. Close out joins in response-owner emissions and account R by both closeouts so px7m err runs natively with interpreter parity"
status: merged
owner: runtime
size: M
tier: T1
gate: architect
depends_on: [RT-SELECTED-PENDING-CALL-BUILD]
blocks: []
github: null
origin: "RT-SELECTED-PENDING-CALL-BUILD residual: Architect evt_2spyd3965e84m names the response-owner environment extension with the J-a join accounting as the future WP that positive native px7m err depends on. Serves the L1 objective (operator 2026-09-17). Steward-filed per COORDINATION section 2."
---

# Native px7m dynamic-error route

## Objective

`dynamic_err_payload_selects_a_multistep_tree_across_real_executors`
(`px7m_hostresult_computational_match.rs:390`) runs natively with
interpreter parity and is un-ignored.

## Settled inputs (on `44443145c`)

- **The refusal is (J).** `RT-NESTED-RESPONSE-OWNER-CALLER` (`2260b91cc`)
  re-pinned the row. Its ignore reason, and the sentinel
  `dynamic_err_pending_route_refuses_unaccounted_relocated_joins` (:408),
  pin exactly one refusal,
  `PendingRefusal::SelectedPendingLeafRelocatesUnaccountedJoins`, with no
  owner-validated route. Admission (E) no longer refuses this producer.
- **The locus.** In
  `ken-runtime/src/cranelift_backend/planning/static_transition/selected_pending_calls.rs`,
  (E) returns at :770 and (J) at :773. The comment at :766 says J-a's
  accounting is deferred, because no admitted row had a nonempty `R`. The
  complement refuses rather than making a selected join look unselected.
- **The capability** (Architect `evt_2spyd3965e84m`): the J-a join
  accounting for a selected leaf whose relocated joins are nonempty.
- **Controls already native.** Both px7l rows and px7m ok run natively with
  zero packages. An OrdinarySpecialization-fed leaf is refused at admission
  (C3). The two-print arm-tail row
  `selected_pending_two_print_route_has_native_interpreter_parity` is a
  native parity row on main. It was a witness of this WP's first frame, and
  it is now a control.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **D0 (measure only, at the start of the repair).** For px7m err, name
   the leaf's relocated join set `R`, each join's origin and the emission
   that would consume it, and why no accounting covers it today. Report
   whether the leaf is StaticResponseOwner-fed or OrdinarySpecialization-fed
   under C1, since C3 refuses the second. State whether (E) still admits
   the producer once `R` is accounted. The Architect rules the repair shape.
2. **The ruled repair (Architect `evt_7859dqm72wayz`, on `44443145c`).**
   The D0 found that `R` is duplicated, not relocated. The issuer emission
   `Predeclared(5)` lowers root 400 ordinarily, because the placeholder is
   licensed only in `Continuation` scopes. Its unit closeout consumes 393,
   396 and 404 and passes. `StaticResponseOwnerId(1)` lowers its own copy
   and consumes `{393, 396}`, and no closeout checks it. Bypassing (J) alone
   makes the row pass at parity. No environment extension is needed, and
   (E) is untouched.
   - **A response-owner join closeout.** Add
     `validate_response_join_plan_consumption` in `joins.rs` and make
     `validate_materialized_dead_join_cfg_for` `pub(super)`. In
     `define_static_response_owner_bodies`, derive `required` from the same
     `OwnedResponseEmission` the body lowers. Call the closeout after the
     last `lower_expr`, and the dead-join CFG check next to
     `verify_cranelift_function`. The ruling has the code.
   - **J-a accounting at admission.** It replaces the blanket refusal at
     `selected_pending_calls.rs:714-719`. `R` is accounted when it is a
     subset of both the response owner's copied operation-argument joins
     and the issuer's `required_join_origins`. That uses a new planner
     helper, `unique_emission_containing`, in `joins_traps.rs`. The
     comments at :656 and :765-766 are updated.

## Acceptance

- **AC-1.** The row is un-ignored and agrees across real executors on
  stdout (`missing.binnot-found\n`) and the host-op vector `[FsReadFile,
  ConsoleWrite, ConsoleWrite]`.
- **AC-2 (sentinel).** The (J) sentinel is retargeted. The err producer is
  admitted with a validated response owner. Each join in `R` is consumed
  exactly once by the issuer emission and exactly once by the
  response-owner emission, each under its own closeout.
- **AC-3 (controls).** Every existing response-owner row keeps its result:
  both px7l rows, px7m ok, the two-print parity row, and the rt suites with
  response owners. So do the C3 classifier pair and `rt_parity_native` at 4
  threads.
- **AC-4 (mutation, QA).** AC-1 alone cannot discriminate, because the row
  is green with (J) simply deleted. These two observations carry the gate:
  - (a) restoring `unaccounted_joins |= !relocated_joins.is_empty()`
    returns err to the (J) refusal;
  - (b) passing an empty `required` to the response closeout refuses err
    with "source join StaticOriginId(393) was classified outside its
    owning function".

## Stop conditions

- The repair needs a kernel, trust or spec change.
- A join is consumed twice within one emission, an emission lowers a
  planned join with no closeout, or a `Deferred` row lowers outside its
  handler owner.
- The response closeout reddens an AC-3 row, or `unique_emission_containing`
  returns `None` for err's root 400: stop to the Architect.
- A runtime trap or a dropped effect on any row after the repair.
