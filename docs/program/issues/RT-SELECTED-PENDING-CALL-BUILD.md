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
- **AC-1a (admission witness; Architect `evt_46vm0djbs20k5`, clauses per
  `evt_2nn9ta5ywrkyh`).**
  - Admission returns a `SelectedPendingRouteWitness`, constructible only in
    `planning/static_transition/selected_pending_calls.rs`. The emitter reads
    its facts from the witness and does not re-derive them. Its one
    ownership check is an internal assertion at package entry, over the
    package owner only.
  - **Census first, clauses second.** Every witness clause is the
    planner-side evaluation of one emitter condition in the census below. No
    clause is written from a model of the route.
  - **Measure first** (a probe, removed after), on px7l and px7m err:
    - M1: `R(leaf)`, the planned joins in the operation subtree of each
      `Specialized` `Vis` the package lowers but does not own;
    - M2: for each join in `R`, the emission that consumes it, or none;
    - M3: for rows 341 and 378, the owner of the emission that lowers them,
      compared with their handler owners.
  - **M4, static only** (Architect `evt_2spyd3965e84m`; no suppression, no
    compile past a refusal): for each relocated unit on px7l and px7m err
    (the operation subtree of a `Specialized` `Vis` the package does not
    own, and the drive subtree of each `Deferred` row), list each free
    variable, its binder origin, and whether the response owner's frame
    receives it.
  - **(E) environment closure**, checked first. Every relocated unit's free
    variables are among the bindings its owner emission receives (its
    `StaticResponseEnvironmentBinding` frame sources plus its K ABI
    parameters and captures). Otherwise the route is refused with
    "relocated work references a binder its response owner does not
    receive". If (E) holds everywhere, STOP and report the Var(1) cause.
  - **(J) join partition**, checked after (E). A leaf with a non-empty `R`
    that no witnessed accounting covers is refused with "selected pending
    leaf relocates planned joins no emission lowers". The J-a accounting is
    not built while no Planned row has a non-empty `R`; it is deferred with
    the response-owner environment extension below. Never read "consumed
    nowhere" off a compile that stopped before the owner emission ran.
  - **(D) Deferred drive.** For each `Deferred` row on the route, its
    handler owner equals the owner of the emission that lowers it.
    Otherwise it is refused with "Deferred response lowered outside its
    handler owner".
  - **Closure census.** Every `unsupported(...)` and `backend_module(...)`
    refusal reachable from the pending-route emission path, including the
    `ObjectEmission` join closeout (`joins.rs:2234`), is classified in the
    handoff as a witness clause, an asserted invariant, or unreachable on a
    witnessed route, with the reason. An unclassified row is a finding.
  - **Pins.** The pair control is px7l (Planned, native, (E) holds, `R`
    empty) against px7m err (Refused by (E)). Each of px7l ×2, px7m ok and
    px7m err is reported Planned or Refused, a refusal asserted by reason.
    Mutations: dropping (E) refuses err by (J); dropping (E) and (J) returns
    it to the join-393 closeout. (D) is reported exercised only by a
    measured row; otherwise unexercised, not claimed. No Planned row fails
    at emission.
  - **Stop** if `R` is not empty on px7l, or if one join is consumed by two
    emissions.
  - Forbidden: dispositioning join 393 as unselected, forcing the
    placeholder, or claiming px7m err native without the (J-a) accounting.
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

## Shared predicate

Pending-call admission decides Planned from facts the emitter re-derives
elsewhere. AC-1a closes the class: the emitter consumes the admission
witness, and each clause evaluates an emitter condition the census names.
Relocating a `Specialized` response to its owner is a lawful handoff that
px7l already uses; it is not a future capability.

The next capability, a future WP (Architect `evt_2spyd3965e84m`): the
response-owner environment extension, which passes binders from the pending
package's scope into a response owner's frame, with the J-a join accounting.
Positive native px7m err depends on it.

## SYMPTOM INVENTORY (append one line per hard-stop; never rewrite history)

1. AC-1's per-member backing refusal has no planner-plane authority:
   `AbiSlot` is uniformly `ValueWord`/`OwnedByFrame` and the plane is
   names-only (`abi.rs:55-60`) -- keyed on the per-value pointee type the
   plane deliberately does not carry. Discharged by route confinement (no
   F5/F6 crossing) for increments 1-2 (Architect `evt_327ykvk4cv8bq`).
2. The pending IH's runtime position has two derivations that disagree at
   px7l origin 47 (IR binding structure 4, erasure-minted morphism and
   callee `Var` 3): one `Match` case binder on the host-response dispatch
   path has no group in erasure's `BranchBinderRemap` -- keyed on a runtime
   binder that erasure's coordinate map does not record (Architect
   `evt_3yce3vzbddvk0`).
3. A selected pending arm's operation field is `StaticResponseDeferred`, so
   its argument join (px7m dynamic-err origin 393, owner
   `PredeclaredFunctionId(5)`) has no evaluating seat on the pending route
   -- keyed on which emission owns relocated deferred-response work, which
   pending-call admission never consults (Architect `evt_3engggxek84h8`).
4. HS3 ownership clause refused px7l's lawful Specialized relocation,
   because it was keyed on owner identity, which the emitter never tests
   (Architect `evt_2nn9ta5ywrkyh`).
