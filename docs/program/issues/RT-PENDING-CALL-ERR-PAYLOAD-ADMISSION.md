---
id: RT-PENDING-CALL-ERR-PAYLOAD-ADMISSION
title: "Pending-call admission refuses the px7m dynamic-error route at (J): the selected ERR leaf relocates joins that no accounting covers, because J-a's accounting is deferred. Account for the leaf's relocated joins so px7m err runs natively with interpreter parity"
status: active
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
  accounting for a selected leaf whose relocated joins are nonempty. The
  response-owner environment extension the same ruling named is needed only
  if the D0 shows (E) would refuse once (J) admits.
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
2. **The ruled repair:** J-a accounting for the leaf's `R`, plus the
   environment extension only if the D0 shows it is needed.

## Acceptance

- **AC-1.** The row is un-ignored and agrees across real executors on
  stdout (`missing.binnot-found\n`) and the host-op vector `[FsReadFile,
  ConsoleWrite, ConsoleWrite]`.
- **AC-2 (sentinel).** The (J) sentinel is retargeted, as its comment
  anticipates. The err producer is admitted with a validated response
  owner, and each join in `R` is consumed by exactly one emission.
- **AC-3 (controls).** Both px7l rows, px7m ok, the C3 classifier pair and
  the two-print parity row keep their results, and `rt_parity_native` at 4
  threads is unchanged.
- **AC-4 (mutation, QA).** Removing the J-a accounting returns err to the
  (J) refusal.

## Stop conditions

- The repair needs a kernel, trust or spec change.
- One join is consumed by two emissions, or a `Deferred` row lowers outside
  its handler owner.
- A runtime trap or a dropped effect on any row after the repair.
