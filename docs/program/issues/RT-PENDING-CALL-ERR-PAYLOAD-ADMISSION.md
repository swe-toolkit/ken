---
id: RT-PENDING-CALL-ERR-PAYLOAD-ADMISSION
title: "Pending-call admission refuses the px7m dynamic-error route at (E): the deferred Effect has free Var(1), but its selected operation supplies one field, so the response owner's frame never receives that binder. Build the response-owner environment extension with the J-a join accounting so px7m err runs natively with interpreter parity"
status: ready
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

## Settled inputs (on `26e7dce3f`)

- **The refusal.** The row's ignore reason, and the sentinel
  `dynamic_err_pending_route_refuses_missing_effect_binding_before_join_accounting`,
  pin one pending producer refused first by admission (E):
  `PendingRefusal::RelocatedWorkMissingLoweringBinding`. The deferred
  Effect has free `Var(1)`, but its selected operation supplies one field.
- **The checks in order** (`RT-SELECTED-PENDING-CALL-BUILD` AC-1a). (E)
  requires every relocated unit's free variables to be among the bindings
  its owner emission receives. (J) refuses a leaf whose planned joins no
  accounting covers. (D) requires each `Deferred` row to lower inside its
  handler owner. Dropping (E) refuses err at (J), and dropping both returns
  it to the join-393 closeout (owner `PredeclaredFunctionId(5)`).
- **The named capability** (Architect `evt_2spyd3965e84m`). The extension
  passes binders from the pending package's scope into a response owner's
  frame, with the J-a join accounting. Relocating a `Specialized` response
  to its owner is already lawful and px7l uses it.
- **Controls already native.** Both px7l rows and px7m ok run natively with
  zero packages (increment 2, C2 and S1'). An OrdinarySpecialization-fed
  leaf is refused at admission (C3).

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **D0 (measure only, at the start of the repair).** For px7m err, name
   `Var(1)`'s binder and its origin in the pending package's scope, the
   owner emission that lacks it, and the join set `R` that (J) would then
   see. Report whether the leaf is StaticResponseOwner-fed or
   OrdinarySpecialization-fed under C1, since C3 refuses the second. The
   Architect rules the repair shape.
2. **The ruled repair:** the response-owner environment extension and the
   J-a accounting, scoped to what D0 shows px7m err needs.

## Acceptance

- **AC-1.** The row is un-ignored and agrees across real executors on
  stdout (`missing.binnot-found\n`) and the host-op vector `[FsReadFile,
  ConsoleWrite, ConsoleWrite]`.
- **AC-2 (sentinel).** The (E) sentinel is retargeted as its comment
  anticipates: the err producer is admitted with a validated response
  owner, and each of its joins is consumed by exactly one emission.
- **AC-3 (controls).** Both px7l rows, px7m ok and the C3 classifier pair
  keep their results, and `rt_parity_native` at 4 threads is unchanged.
- **AC-4 (mutation, QA).** Withholding the extended binder returns err to
  the (E) refusal. Removing the J-a accounting returns it to the (J)
  refusal.

## Stop conditions

- The repair needs a kernel, trust or spec change.
- One join is consumed by two emissions, or a `Deferred` row lowers outside
  its handler owner.
- A runtime trap or a dropped effect on any row after the repair.
