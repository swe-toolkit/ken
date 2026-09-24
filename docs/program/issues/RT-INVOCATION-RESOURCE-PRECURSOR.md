---
id: RT-INVOCATION-RESOURCE-PRECURSOR
title: "Pending-call precursor: named finite invocation-profile resources reserved at BoundaryActivationV1::begin, a runtime-wide no-wrap epoch, an activation-owned bounded slot and generation issuer behind a checked services ABI, and a typed CapacityExhausted fault carried end to end, so a selected call can be authenticated exactly once; not the pending-call build"
status: merged
owner: runtime
size: L
gate: architect
tier: T1
depends_on: [RT-BRACKET-SOURCE-EDGE]
blocks: [RT-SELECTED-PENDING-CALL-PACKAGE]
github: null
origin: "Operator 2026-09-24 ~04:05Z: 'authorize option (a) for pending-call. it has to be addressed.' Resource fork: Architect evt_6dgk3tmqpbqbm, evt_1fft79t02kw31. Frame basis: Architect evt_2373feaep6zh9. Sequenced after RT-BRACKET-SOURCE-EDGE's D0 on the runtime ring (Steward). Steward-filed per COORDINATION section 2."
---

# A selected call needs an identity that is never reused

## Objective

Native code can issue and check a one-use ticket for a runtime-selected
call, and every declared resource limit it relies on fails as a typed,
named fault rather than an unclassified trap. This unblocks
`RT-SELECTED-PENDING-CALL-PACKAGE` (the px7l/px7m rows: branching on a host
result into multi-step effects, and effectful callbacks that capture data).

Scheduling gate met: `RT-BRACKET-SOURCE-EDGE` D0 closed on its STOP
(2026-09-24). That gate was scheduling only; it certifies no bracket
implementation.

## Fixed inputs -- Architect `evt_2373feaep6zh9` at `1a4495376`

- `BoundaryResourceProfileV1` has only invocation/persistent × Nodes, Words,
  DataBytes and NativeIntLimbs.
- `BoundaryActivationV1::begin` reserves, then publishes the two-field
  `GeneratedActivationServicesV1` (`native_int_arena`, `boundary_arena`).
- `require_i64` and `BOUNDARY_ERR_CAPACITY=-6` do not preserve a resource
  name. `TerminalErrorV1` has no CapacityExhausted variant. Linked
  `effect_wire` terminal errors encode only root-denial and home-root
  failures. `object_linker_packaging::decode_signed_root_trap` reads negative
  values against the planner trap catalog. An exhausted ticket is not a
  planner trap and may not become `-1` / RuntimeTrap.
- Pending values can be copied and re-entered, so a site key, fixed slot,
  spent bit or pointer aliases events (Architect `evt_6dgk3tmqpbqbm`).

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverables -- one node, in order

- **D0, before implementing.** Enumerate every profile constructor and
  packager, activation caller, services field/offset reader, status producer,
  terminal consumer, linked-wire encoder/decoder, and one real selected-call
  path. For each proposed resource, name its owner, finite unit and bound,
  reservation point, counter, and at-limit/one-past observation. Establish
  one runtime-wide nonreused epoch authority across sequential activations,
  the activation-owned backing's fixed address after `begin`, and the checked
  services ABI route to it. Record which faults occur at `begin` and which in
  emitted code.
- **D1, typed fault and profile (internal checkpoint).** Named
  invocation-profile entries with explicit scope and finite bounds for
  epochs, event generations, live pending slots and each S/C backing family
  used. Deployment supplies the profile; zero is an explicit bound; no
  implicit default. `begin` reserves everything before publication and
  returns a typed failure otherwise. A distinct typed `CapacityExhausted`
  fault (scope, resource, limit, request) has two sources: `begin` and
  epoch failures are a pre-launch typed terminal projection (generated code
  never runs); in-flight slot and generation exhaustion goes through
  generated status. Both keep the exact resource through the terminal
  boundary, `ken-host::TerminalErrorV1`, the linked trace wire in both
  directions, and packaging, on a reserved non-colliding tag. Unknown
  tags are rejected. It may land alone only if it has a real
  resource-consuming owner and at-limit/one-past controls; enums and wire
  tags alone are inert scaffolding.
- **D2, checked issuance.** An activation-owned bounded live-slot and
  monotonic-generation issuer with fixed backing. A ticket holds epoch,
  generation, slot and the exact selected body/call target. The consuming
  gate checks all four and unspent state **before** reading borrowed operands
  or calling. Duplicate, wrong arm, stale-after-slot-reuse and
  stale-after-activation-reuse are integrity faults; overflow and exhaustion
  refuse before issuing, never wrap.

The Architect's basis is the full contract; read it before D0.

## Acceptance

- **AC-1.** For each newly advertised finite resource: exact at-limit
  success and one-past typed refusal naming that resource end to end.
- **AC-2.** A below-limit selected-call positive, and cross-activation ABA,
  duplicate and wrong-target negatives at the real gate. No selected effect
  and no half-ticket before issuance succeeds.
- **AC-3.** The typed fault round-trips the linked wire; a malformed or
  unknown tag is rejected; no control goes green on an arbitrary negative
  root token.
- **AC-4.** Targeted builds only, through `scripts/ken-cargo`. No-regression
  means green in CI.

This node does not build the pending-call package. After it lands, a
separate retry captures body 322's own six S/C members (343's frame is no
surrogate) and reruns `RT-SELECTED-PENDING-CALL-PACKAGE` D1.

## Stop conditions

Stop if any new resource stays an unprofiled allocation or default; the
epoch can repeat while a ticket is live; the ABI reaches an unowned or
dangling pointer; the fault cannot name the exact resource end to end; or a
control passes on an arbitrary negative root token. True OS OOM may be a
loud fatal under spec `44 §2`; a declared limit may not. Held pending-call
and bracket refs stay unmoved.
