---
id: RT-NATIVE-SELF-EMITTER-RESPONSE-ELIGIBILITY
title: "F's checked-IH caller of owner2 is a transport source transported to its own emitter, so the detached seat consumes it as payload and no Function emits the call, while the response plane still gives the identity a forward-declared static response owner that waits for it; the lowering audit refuses F before object emission. Make the two planes agree on that identity, so F's three rows emit an object under the C-28 overlay with no PlannerInvariant and every authority control stays green"
status: ready
owner: runtime
size: L
tier: T1
gate: architect
depends_on: []
blocks: [RT-NATIVE-FORWARDED-BOUNDARY-TRAP]
github: null
origin: "Architect ruling evt_57f2n3vsn34qm on the boundary-trap AC-0 continuation (evt_1q0s7y4sjp735): Part B is NOT-EMITTED, and the plane is response-owner eligibility, not the IH-edge lowering. Steward recut evt_1dy9v5gtmhrr6: F-to-object-emission is its own node ahead of the trap. L1 objective: the Linux ABI program. Steward-filed per COORDINATION section 2."
---

# One answer for the self-emitter caller

## Objective

F's three rows (argv, live, null-stdout) reach object emission under
the C-28 overlay with no PlannerInvariant. The transport plane and the
response plane give F's owner2 caller one consistent treatment, as
AC-0 rules, and every control of the placement authority stays green.

## Settled inputs (main `f32af2a0b`; measured `evt_1q0s7y4sjp735`)

- **The refusal.** All three F rows fail at `units.rs:9807`
  (`validate_response_owner_call_coverage`), "a forward-declared
  response owner has no verified selected incoming call". The owner is
  `StaticResponseOwnerId(2)`, context 0. Its selected caller is
  Pred8/Spec8 to Spec10 (result 1507, construct 1500) in argv and null,
  and Pred6/Spec9 to Spec11 (1026/1019) in live.
- **The transport plane.** That caller is a transport source
  transported to its own emitter (Spec8, Spec9):
  `checked_ih_environment_transport_source` is Some, and
  `checked_ih_self_emitter_source` is true. The detached seat
  (`core.rs:13195`-`13204`) returns the carried word on membership and
  emits no call.
- **No emitter.** Across 49 argv and 47 live Functions there is no
  identity record and no CLIF call to the selected target. None of
  the three `verified_response_owner_calls` feeds (`units.rs:7808`,
  `:8475`, `:11774`) receives either identity.
- **The response plane.** It still installs owner2. The seed in
  `escaped_checked_ih_callers` (`responses.rs:4113`-`4120`) would route a
  self-emitter caller to the deferred `EscapedCheckedIhCaller` row, but
  its conjunct `!transport_sources.contains(identity)` (`:4116`) excludes
  this one. The seed's producer-result count of 1 is unmeasured.
- **The history.** This is RNSPK's F-residual on the same identities
  (C-7 to C-18). Patching one owner2 edge moved the wall: C-12 to owner2
  context 1, C-17 to a new owner at context 8. The §1b predicate named
  then: per-owner static-response eligibility by escape closure, with
  an ineligible owner taking the dynamic resume path. Research
  advisory: `evt_7yds2s5j9q732`.
- **The gate.** The production Process gate (`compiler_driver.rs:3001`-
  `3024`) refuses F before object emission on every production base,
  so F is measured under the uncommitted C-28 overlay (sha256
  `4365de7f...`). F emitting there exposes nothing to users.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

One candidate on `f32af2a0b` or its successor: the transport and
response planes agree on every self-emitter transport-source caller,
in the direction AC-0 rules. The same candidate closes the carries
from the Architect's authority review (`evt_kac366kxewtv`), moved here
from the trap node (`evt_57f2n3vsn34qm`):
- **C1.** `record_relocated_exit` (`units.rs:9315`) has no caller.
  Delete its ledger, or give it a caller and a mutation that reddens it.
- **C2.** The E3 seed in `escaped_checked_ih_callers` (`responses.rs:4102`)
  is this node's candidate locus. Resolve it with the repair; do not
  delete or empty-pin it first.
- **C3.** Correct the prose for these planes: the
  `checked_ih_self_emitter_source` doc (`aggregates.rs:6308`, "cannot
  mint a relocation", false since self-emitter transports were restored),
  the `EscapedCheckedIhCaller` doc (`responses.rs:803`) and the trap
  comment at `construction.rs:1663`-`1665`.

## Acceptance

- **AC-0 (D0, Architect, before product code).** Count the population
  first: on F's three rows and every authority control, every response
  owner whose selected caller is a self-emitter transport source, with
  its owner, context and seat outcome. Then rule the direction: the
  seat emits the call, or the identity gets no static owner and takes
  the dynamic resume path. Also name the committed observation that
  fails on `f32af2a0b`.
- **AC-1 (measured under the overlay).** F's three rows reach object
  emission with no PlannerInvariant. F's native outcome is recorded:
  the ordered markers, whether Buffer1 is released early, and the
  fallback count. That is D2 re-measured on the landed authority.
- **AC-2.** The authority's AC-1 controls stay green on the committed
  product, without the overlay, including entry 18 at parity and
  ANCHOR-PARITY C's 16 generated-entry arrivals on each checked_ih row.
- **AC-3.** Reverting the repair returns F to the `units.rs:9807`
  refusal, and the AC-0 committed observation reddens.
- **AC-4.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The refusal moves to another owner or context. Post it as a stop with
  its log; do not patch the next edge.
- A kernel or trust change, or a change to the Process gate,
  `HostOpV1::availability` or AC-2g (B2's).

Checks: 2 fired: the fan-in to owner2's call was measured at
`f32af2a0b`: the seat's membership branch, the three verified feeds
and all 49/47 emitted Functions, with zero calls and zero feeds.
6 fired: AC-0 counts every self-emitter transport-source owner before a
direction is chosen, because one-edge patches moved the wall at C-12
and C-17. 8 fired: AC-3 is the overlay refusal returning, the failing
configuration itself, since the production gate refuses F with or
without the repair.
