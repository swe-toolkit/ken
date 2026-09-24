---
id: RT-SELECTED-PENDING-CALL-PACKAGE
title: "design, before any code, a typed activation-owned selected pending-call package carried beside the pending ITree word from the executed source arm through the six transport families to a one-event consuming call gate, with a runtime call-event authority, so the four px7l/px7m rows have a legitimate package to check; bounded D0 census of the present substrate first, then a D1 pre-code specification; STOP and park only if D1 cannot specify owner, extent, identity or a complete route"
status: closed
owner: runtime
size: L
gate: architect
tier: T1
depends_on: [RT-INVOCATION-RESOURCE-PRECURSOR]
blocks: []
github: null
origin: "Operator 2026-09-23 ~15:00Z: 'concur with rec on L1 carrier', authorizing the new runtime representation the Architect named in evt_7v8he9tyxftp6 after the RT-SELECTED-ARM-CALLABLE-CARRIER D2 STOP (evt_6zf0n10f0djzs). Steward-filed per COORDINATION section 2."
---

# The selected arm needs a package the call can check

## Settled inputs -- Architect `evt_7v8he9tyxftp6`, at `a64aa66e8`

- No carrier design exists on the present substrate. The executed arm is not
  represented as an activation-owned call authority across the value-only
  boundary. The planner's `producer_alternative=1` is shared by both arms.
  `ContinuationActivationId(0)` is compiler-local. The join, bind/project,
  recursor/resume and generated return lose the source ticket.
- Rows, fixtures and the six transport families are as recorded in
  `RT-SELECTED-ARM-CALLABLE-CARRIER` and `evt_4j4rakqyr5g2w`. The
  borrowed-span tuple (`evt_51cgf00j10a9p`) is a direction, not a design.
- An allocation failure today travels `BOUNDARY_ERR_CAPACITY=-6 -> -1 ->
  UnclassifiedRuntimeTrap`, which is not a typed fault.

> **Resumes after `RT-INVOCATION-RESOURCE-PRECURSOR` lands (operator
> 2026-09-24).** The first D0 census below was attempted and ended in the D1
> STOP (`evt_4rgbwg2vgk6kc`). On resumption, D0 is a capture of **body 322's
> own six S/C members** on current main; sibling 343's 3+3 frame is no
> substitute. D1 then proceeds on the precursor's resources.

## Carried from the precursor D2 (`6653de61d`; Architect `evt_gawqf96vgdzw`)

- The call-event authority now exists. D2 added an activation-owned,
  bounded, fixed-backing issuer. Tickets carry {epoch, generation, slot,
  exact selected body/call target}. The consuming gate is at
  `calls.rs::call_declared_unit_target`, before frame packing and operand
  loads. D1 **uses** this issuer and gate. It does not specify a second
  authority. Its event generations and live slots are the profiled
  resources; any other new resource is still a STOP.
- **Borrowed operands load after the consume check.** D2's operands are
  computed locally before issuance. Once a copied pending value carries
  borrowed operands, D1 must emit their loads **after** the same consume
  check, never while building the call inputs.
- **Nested-unit gate witness.** The later implementation adds a witness
  that the gate holds for a selected call inside a nested unit, not only at
  top level.

## Deliverables

- **D0, bounded census of the present substrate** (Architect
  `evt_4chkm2xxyfang`; the first D0 demanded a package that does not yet
  exist). Scratch instrumentation only. Identify the executed arm versus the
  sibling, the producer, `L1` and word-only loss points, the available
  operands and their owners, and the first refusal. Report the positive
  package and its mutations as **not yet runnable**, not as green or red.
  The baseline is `evt_30dybw9824mt`: the only frame is sibling 343's, and
  body 322's S/C values are never materialized.
- **D1, pre-code specification to the Architect.** Specify the package:
  - **Placement.** It travels beside the pending `ITree` word. Not inside
    `CarriedBoundaryWord` and not in the frozen `LoweringOperand` sum.
  - **Contents.** The executed-arm construction site for body 322's
    **own** ordered S/C values; relabeling 343's frame does not count. Name
    each member's owner, representation, extent, same-activation lifetime
    and backing. Owning the package does not establish who owns each
    borrowed span.
  - **Transport.** Its route through all six families, ending at a
    one-event consuming gate that re-declares and authenticates the target.
  - **Call-event authority.** Use D2's issuer and gate; do not specify a
    second authority. Say where the selected arm's ticket is issued (after
    selection executes, so nothing is issued for the sibling), how the
    ticket travels with the package through all six families, and that
    the one consume is D2's gate in `call_declared_unit_target`. Map the
    refusals onto D2's typed faults: a duplicate is `Spent`, an expired
    ticket is `StaleGeneration` or `WrongActivation`, and a wrong arm is
    `WrongTarget`.
  - **Live-slot lifetime.** In D2 a slot is live only between issue and
    the adjacent consume. Here it stays live from selection to the call,
    and D2's issuer frees a slot only by consuming it. Bound how many
    tickets can be live at once in one activation against
    `live_pending_slots`, and show that every issued ticket is consumed
    on every path or the activation terminates. If a path needs to give
    up a ticket without calling, specify a release on D2's issuer that
    frees only a live slot of the same epoch and never returns a
    generation. That release is D1 content, not a new resource and not a
    STOP.
  - **Effects.** Nothing takes effect before selection, and there is
    exactly one call.
  - **Later proof.** Say how the implementation will produce a positive
    selected-arm witness and independently turn the sibling, duplicate and
    missing mutations red at the real call gate.
  - **Lifetime.** A proof that it cannot outlive its borrowed bytes or carry
    foreign SSA handles.
  - **Allocation.** Say whether it needs a metered allocation.

## Acceptance criteria

- **AC-1.** The D0 census is posted in the WP thread. The positive witness
  and the mutation reds are obligations of the later implementation frame.
  This node does not claim them.
- **AC-2.** The D1 design keeps `L1`/`L2`/`L3`, one-unit `L2` and
  `gather_cannot_serve`. It does not force `Vis` or move the context call.
- **AC-3.** No production code lands from this node. A later build node is
  framed only after the Architect approves D1.

## Stop conditions

- If D1 cannot specify a representable owner, extent, identity and
  complete route, that is the §5 STOP. The Steward parks the attempt. A
  record name with no production site or transport is not D1.
- Operator 2026-09-24 authorized the resource fork as
  `RT-INVOCATION-RESOURCE-PRECURSOR`. Once it lands, this node consumes its
  bounded resources and typed terminal path. A genuinely new, unprofiled
  resource is still a STOP.
- **Held work:** never move `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the
  child-2 checkpoint.
