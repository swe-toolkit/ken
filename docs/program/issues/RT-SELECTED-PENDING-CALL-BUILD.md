---
id: RT-SELECTED-PENDING-CALL-BUILD
title: "Build the selected pending-call package designed in RT-SELECTED-PENDING-CALL-PACKAGE D1: a companion package beside the pending ITree word, built in the executed arm from its own ordered S/C members, carried through the six transport families to D2's one-event consuming gate, so the px7l/px7m rows can lower natively; first checkpoint measures backing classes, arm-edge dominance and traversed families before the build proceeds"
status: draft
owner: runtime
size: L
gate: architect
tier: T1
depends_on: [RT-SELECTED-PENDING-CALL-PACKAGE, RT-PLANNER-SEED-BINDING-ORDER]
blocks: []
github: null
origin: "RT-SELECTED-PENDING-CALL-PACKAGE AC-3: Architect D1 disposition 'REPRESENTABLE D1, no §5 STOP' (evt_3y5xkyf1v02dj) on main 1303e5cd4 and D0 evt_3ywcecyzbyqkh; runtime-leader request evt_2vx0eh0v312rb. Serves the L1 objective under operator 2026-09-23 'concur with rec on L1 carrier'. Steward-filed per COORDINATION section 2."
---

# Build the selected pending-call package

**Status: parked at AC-0 on `RT-PLANNER-SEED-BINDING-ORDER`**, under
`evt_6yjef2cy4nv1e`'s e2 ≠ e3 outcome (Architect `evt_2eqe033bxff6f`). On
resume, rebase onto it and re-run AC-0(e).

## Settled inputs -- at `1303e5cd4`

- **The design is D1**, recorded verbatim in
  `RT-SELECTED-PENDING-CALL-PACKAGE.md`, section `D1 disposition (Architect
  evt_3y5xkyf1v02dj, at 1303e5cd4)`. Build items 1-5 from there: the
  placement, the contents, F1-F6 transport, the static-candidate gate over
  `call_declared_unit_target`, D2's issuer as the only call-event authority,
  and exactly one call. This frame does not restate it.
- **D1 amendment 1** (Architect `evt_54vq22cwfnp9k`), recorded verbatim in
  the same file, governs this build: backing class (f), the F4 gate at every
  carried-residual consumer, nested arms, C0 and the dominance test. D1
  amendment 2 (Architect `evt_6yjef2cy4nv1e`) replaces its AC-0(e).
- **D0 census** `evt_3ywcecyzbyqkh`. Body 322's six members [S0, S1, S2,
  C0, C1, C2] are planner declarations only on main. The first refusal is
  `reject_carried_residual_arguments`, and it is not a D2 typed fault.
- **Rows.** The target rows are ignored in `crates/ken-cli/tests/`:
  - `px7l_checked_host_recursive_bind.rs`:
    `delayed_capturing_generic_bind_agrees_across_real_executors` (`:164`)
    and `runtime_selected_non_unit_response_is_consumed_across_real_executors`;
  - `px7m_hostresult_computational_match.rs`: the `dynamic_ok_payload_...`
    and `dynamic_err_payload_...` rows.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

Implement D1 items 1-5 in `ken-runtime`, under the acceptance criteria
below. Un-ignore each target row that the positive witness makes pass
natively. Any row that stays ignored gets a measured reason in its
`#[ignore]` text.

## Acceptance

- **AC-0 (first checkpoint, before the build proceeds).** Post to the WP
  thread, measured on the fixture, and stop for Architect review:
  - (a) each of the six members' backing class, (a) to (e) of D1 item 2;
  - (b) that closure 326's package construction site is dominated by the
    selected arm edge of the dynamic Bool `brif`;
  - (c) which of F1-F6 the fixture actually traverses;
  - (d) for each of the four target rows, its first refusal on the base and
    whether it is in the AC-1 admission population. A row outside the
    population stays ignored with that measured reason; it is not a new
    admission.
  - (e) the C1/C2 operand order: the static three-way check of D1
    amendment 2 (source truth, planner, emitter, source-to-ABI mapping),
    before AC-1. All agree continues into AC-1 with a static pin against the
    source declaration; planner ≠ emitter or source ≠ planner is a STOP.
- **AC-1 (admission, D1 item 0).**
  - A package is planned only for a pending producer `Match` whose arms
    declare different recursive body units: the population
    `agreeing_recursive_body_unit` refuses today. That predicate is called
    read-only and L2 is not edited.
  - Every producer that L1/L2/L3 admits today keeps its current route,
    including 343's constructed-frame route, `gather_cannot_serve` and the
    retarget route.
  - The planner proves the route linear and must-reach. Each of these is
    refused at compile time with today's BoundaryCarrier refusal, never a
    runtime `Spent`:
    - a pending value read twice;
    - a route that reaches the generated root;
    - a member with an unclassifiable backing.
- **AC-2 (proof, D1 item 6).**
  - **Positive.** Run the px7l fixture natively and assert:
    - the selected arm's host effect happens once;
    - there is one issue and one consume for the selected target;
    - there are zero issues for the sibling target.

    Also:
    - add a sibling variant (`match (not terminal)`), so the other
      candidate's positive path is also observed;
    - add the nested-unit gate witness.
  - **Independent mutations at the real gate.** Run each alone, with the
    others off and the same-harness positive control green:
    - sibling target with 322's members → `WrongTarget`;
    - `DuplicateConsume` → `Spent`;
    - suppressed issue with a zero ticket → `WrongActivation`;
    - flipped `target.body` → `WrongTarget`.

    Each red shows `selected_call_integrity: Some(..)` and no host effect
    after selection. A compile-time refusal is not a red for these.
- **AC-3 (lifetime and allocation, D1 item 7).**
  - **No foreign SSA.** Every use site asserts
    `package.function == defining_function_id` as a fail-closed backend
    error. Words cross functions only through declared frame slots (F6).
  - **Class (e)** (native stack) is admitted only when no return leaves
    its owning activation.
  - **No metered allocation and no release entry point** on D2's issuer.
- **AC-4 (preserved).** L1/L2/L3, one-unit L2 and `gather_cannot_serve`
  are unchanged. No forced `Vis`, and the context call does not move.
  Targeted builds only, through `scripts/ken-cargo`. No-regression means
  green in CI.

## Stop conditions

- A genuinely new, unprofiled runtime resource, or any need for a release
  on D2's issuer, is a STOP. That release lands only through a D1
  amendment reviewed by the Architect.
- A member whose measured backing is outside classes (a) to (e) is a STOP
  for a ruling, not a new class.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.

## SYMPTOM INVENTORY (append one line per hard-stop; never rewrite history)
