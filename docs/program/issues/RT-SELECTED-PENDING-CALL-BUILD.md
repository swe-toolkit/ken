---
id: RT-SELECTED-PENDING-CALL-BUILD
title: "Build the selected pending-call package designed in RT-SELECTED-PENDING-CALL-PACKAGE D1: a companion package beside the pending ITree word, built in the executed arm from its own ordered S/C members, carried through the six transport families to D2's one-event consuming gate, so the px7l/px7m rows can lower natively; first checkpoint measures backing classes, arm-edge dominance and traversed families before the build proceeds"
status: active
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
    - in increments 1-2, a package route that crosses F5 or F6, meaning it
      leaves its defining function (Architect `evt_327ykvk4cv8bq`: route
      confinement discharges the backing question; the AC-0 member classes
      stay evidence and nothing branches on them).
- **AC-1a (admission witness, Architect `evt_46vm0djbs20k5`).**
  - Admission returns a `SelectedPendingRouteWitness`, constructible only in
    `planning/static_transition/selected_pending_calls.rs`. It carries the
    package emission owner; per selected leaf, each `Vis`'s response
    disposition and owner (from the plan functions at `responses.rs:3389`
    and `:3548`); per-member backing; and binder coordinates.
  - A route with any `Specialized` or `Deferred` `Vis` whose owner is not
    the package emission owner is REFUSED at admission, reason "selected
    pending leaf crosses a response owner", at the granularity admission
    already refuses at. No per-leaf fallback.
  - The emitter reads owner, dispositions, backing and coordinates from the
    witness. Its one ownership check is an internal invariant assertion at
    package entry (`defining_emission_owner == witness.owner`).
  - **Closure census.** Every `unsupported(...)` and `backend_module(...)`
    refusal reachable from the pending-route emission path, including the
    `ObjectEmission` join closeout (`joins.rs:2234`), is classified in the
    handoff as a witness clause, an asserted invariant, or unreachable on a
    witnessed route, with the reason. An unclassified row is a finding.
  - **Pins.** Each of px7l ×2, px7m ok and px7m err is reported Planned or
    Refused. px7m err is Refused with the owner-crossing reason, asserted by
    reason. A leaf whose `Vis` owner equals the package owner stays Planned
    beside a crossing leaf that refuses. Dropping the ownership clause
    returns px7m err to the join-393 refusal. No Planned row fails at
    emission. If px7m ok shares px7m err's package, report both refused;
    do not split the route per leaf.
  - Forbidden: dispositioning join 393 as unselected, forcing the
    placeholder across the owner boundary, or claiming the err row native.
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
- The build gives a converting owner (`CallableDeclaration`, `ClosureBody`)
  with two or more parameters non-uniform per-slot result phases while
  `joins_traps.rs::result_phase_environment_for_owner` is still seeded in ABI
  order: STOP. Its de Bruijn reads would then take the mirror parameter's
  phase. Seed it through `source_body_binding_order` first (latent at
  `6bdd75394`, because every entry there is `ResultPhaseSummary::carrier()`).
- **The F5/F6 increment starts with a design ruling.** Carrying a package
  across a return or a generated unit call needs a per-member backing
  authority, which the names-only plane (`abi.rs:55-60`) cannot supply. That
  increment opens with an Architect ruling on a typed or provenance-tagged
  authority, and may be framed as its own successor.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.

## Shared predicate (Architect `evt_46vm0djbs20k5`, third stop)

Three advancing stops had one cause: pending-call admission decided Planned
from facts the emitter re-derives elsewhere. Those facts were per-member
backing (`evt_327ykvk4cv8bq`), the IH binder coordinate
(`evt_3yce3vzbddvk0`), and response ownership (`evt_3engggxek84h8`). AC-1a
closes the class: the emitter consumes the admission witness instead of
re-deriving.

Positive support for an owner-crossing pending leaf (retargeting the
response owner's resumption into the package, or calling the specialization
from the arm) is a new capability and a future WP, not part of this one.
