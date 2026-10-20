---
id: RT-BRACKET-PRODUCER-ROUTE-NESTED-BODY-ORDER
title: "HELD INPUT to RT-BRACKET-LOWERING-AND-D0-REFUTER (child 3). At px8ta depth 3, natively and under either settlement mode, the existing computational-producer route never drives the middle bracket's body: after the second acquire it reaches the outer region's settlement S524, whose ResourceBodyResult scrutinee at origin 509 is unwritten (tag 0), and it traps with planned identity 40. The interpreter opens 1, 2, 3 and releases 3, 2, 1. Make the producer route drive nested bracket bodies in order and write each body result before its settlement runs. THIS NODE DOES NOT LAND ALONE: its reviewed tip is assembled by child 3."
status: active
owner: runtime
size: L
gate: none
tier: T1
depends_on: []
blocks: [RT-BRACKET-LOWERING-AND-D0-REFUTER]
github: null
origin: "Architect scope ruling evt_4yf9jdc9veepy on child 3's 4th advancing stop (evt_x3qm5wmxcwm5) and its diagnosis (evt_4mcwhasce96f1): forced all-Deferred fails byte-identically, so the defect is in the existing producer route that child 3 exposes, not in child 3's settlement emission. Child 3's section 4 stop has fired. Operator 2026-10-17: reactivate the remaining ignored tests. Steward-filed per COORDINATION section 2."
---

> **HELD INPUT.** This node does not land on its own. Its reviewed tip
> is assembled into child 3, like 2a and 2b. No publication and no
> partial landing.

# Nested bracket bodies run in order on the producer route

## Objective

At px8ta depth 3, linked native execution acquires 1, 2 and 3, releases
3, 2 and 1, and exits normally, as the interpreter does, with each
ResourceBodyResult written before its settlement reads it.

## Fixed inputs (Architect `evt_4yf9jdc9veepy`, on `8d3731a3d`)

- **The evidence checkpoint** is child 3's WIP `8d3731a3d` (tree
  `c487089f`). It is evidence, never a candidate. R8 is retained there.
- **Measured at depth 3, native, both modes:** FsOpen 1 (source 935),
  FsOpen 2 (source 216), ResourceRelease 1 (seat 190), then a match at
  origin 509 reading tag 0 against ResourceBodyOk and ResourceBodyErr,
  and trap 40. ResourceRelease 2 has no host-call marker, so it is exit
  cleanup. Under forced Deferred there are 0 copy-entry markers, and
  child 3's Deferred arms emit only ledger claims.
- **Interpreter, same source:** opens 1, 2, 3; releases 3, 2, 1;
  NormalReturn.
- **First native run.** Depth 3 has never executed natively before. On
  main it refuses at object emission with "the claimed continuation
  target was not declared into this function" (`core.rs:11890` at
  `8d3731a3d`; `RT-DEPTH3-CONTINUATION-CLAIM-UNDECLARED`, draft). Which
  change of the assembly routes depth 3 around that refusal has been
  read, not run.
- **Child 3's boundary.** Its §5 excludes a new carrier, return
  protocol, KRET lane, host operation, dispatcher reorder, trace sort or
  fallback. That exclusion does not bind this node, but every widening
  needs an Architect ruling.

Treat anchors as perishable. If a fixed input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

**Recut at the 5th stop (Architect `evt_7knq6fn8km9ra`, on Research
`evt_2gt3nfsvmzzke`): the handler-owned drive becomes a planned
resumption call.** It is no longer a lowering-time route. Measured at
`1a4ed40ea`: the settlement plane orders nothing across regions. Mode
comes only from whether a region has a returned-exit owner, and InPlace
is LIFO only because the release runs inside K's native frames.

1. **Replace** `DrivenKResume::TransferToContext`, reached from
   `drive_handler_owned_deferred_response`. The planner emits one
   planned resumption call per handler-owned Deferred
   `DeferredResponseRow`, in `PlannedContinuationSpecializationCall` or
   in a sibling table that every consumer of it reads. The record
   carries:
   - `emission_owner`: the row's `admitted_deferred_handler_owner`;
   - `producer_result_origin`: K's worker body;
   - `producer_owner`: raw Pred4;
   - `context`: C0;
   - `inputs`: `[response] ++ claims`, the L1′ ABI;
   - the checked eliminator run starting at `route.continuation_origin`,
     as in L2′;
   - `enclosed_regions`: the bracket regions acquired in K's body, with
     their `parent` links.
2. **Every plane reads the record, and none re-derives it at lowering.**
   The readers are `intern_generated_contexts` (C0),
   `entry_frame_claim_in` and `verify_entry_frame` (claims),
   `call_declared_context` (inputs), the L2′ composition (eliminators)
   and `BracketSettlementControlView` (settlement order).
3. **Settlement contract.** Within one planned resumption, deferred
   settlement order is the reverse of acquisition order. It is derived
   from `enclosed_regions` nesting, not from emission order, and the
   planner checks it: any other order is a `planner_error`.
4. **Retain**, as consumers of the record rather than self-deriving:
   F1; P0 with its per-capture source compare; V; P with
   `raw_owner == Pred4` and C0 certification; L1′; and L2′ with its
   frame-0 identity check. The non-candidate WIP `1a4ed40ea` is the base
   to build from.

## Acceptance

- **AC-0 (D0, non-advancing; measures, edits no product).**
  - (1) Under forced-Deferred, at depth 2 and depth 3: each region's
    acquisition index and settlement index within the drive's
    resumption, and the function and iteration key whose order emits
    releases 1, 2, 3.
  - (2) A census of the unforced regions that take Deferred, by
    `BracketResponseContextKey` (Continuation or Inline) and by nesting
    depth, across px8ta and the drivers in `crates/*/tests`,
    `examples/`, `catalog/` and `conformance/`.
  - (3) The planned-resumption record written as a Rust declaration in
    the delivered vocabulary, naming its table, its id type and every
    consumer.
- **T''-1..16** stand (Architect `evt_3tma32z29wz46`, `evt_3ztrhbzpvyb0`,
  `evt_1t43pvv4apd6`).
- **R-1.** Forced-Deferred depth 3 exits 0 with terminal None, opens 1,
  2, 3 and releases 3, 2, 1. This is T''-3.
- **R-2.** The same at depth 2.
- **R-3.** As-shipped is unchanged: depth 3 exits 0 with releases 3, 2,
  1, and depth 2 and `rt_native_bracket_body_once` are unchanged.
- **R-4 (mutation).** Reversing `enclosed_regions` order on the record
  gives releases 1, 2, 3 and exit 1, or a planner refusal, never exit 0.
- **R-5 (mutation).** Deleting the record for the depth-3 row refuses at
  the planner, not at lowering.
- **R-6.** The settlement-order check refuses a hand-built plan whose
  order is not the reversal.

**Precommit.** D0 returns are non-advancing. A planner refusal of the
record on depth 3 is a query. Any other new wall is the node's 6th
advancing stop: hold and call Research, scoped to the new fork. D0 (2)
does not reclassify T''-3, whatever it shows.

## Stop conditions

- The fix needs a new carrier, return protocol, KRET lane or host
  operation: stop to the Architect and the Steward.
- (a) cannot name the skipped step from runtime markers: stop with the
  marker trace. Do not infer the step from static emission counts.
- Do not reopen `RT-PLANNER-KRET-GRAFTED-SPINE`, and do not add a census,
  carve-out, narrowed matrix or fallback.

## Hard-stop inventory (§1b)

§1a: 5, and the child 3 aggregate is 6 (Architect `evt_d6kxxtgevwbw`).
The Research advisory discharged the trigger, and the ruling is the
structural recut in the Deliverable (`evt_7knq6fn8km9ra`). The next
trigger is the node's 6th stop or child 3's 9th. WIPs `16cc5ee62` and
`1a4ed40ea` are not candidates; at `1a4ed40ea` the as-shipped depth-3
test exits 0 with releases 3,2,1. The 3rd stop is ruled as
closure (T), transfer, which is the recut in the Deliverable
(`evt_687xbfdep022z`), realized as T'' (the drive calls K883's one
planned emission C0 through a planner-recorded route; `evt_3tma32z29wz46`,
L1′ `evt_3ztrhbzpvyb0`). The F2/F3 WIP `9ecf2948d` is superseded.

1. With every bracket forced Deferred, object emission refuses the
   lexical-owner call ("claimed continuation target not declared into
   this function", token owner Predeclared(4), construct S874, defining
   Spec(3); `evt_xp7wn0apy41f`).
2. The CurrentLexical availability claim for Pred4's lexical frame is
   refused inside the generated emission context Spec(3)
   (`evt_2ydj1fayk50sc`).
3. S747's descent-owned continuation call (emission owner
   Specialization(1)) claimed inside Spec(3) by the handler-owned inline
   drive of Vis899's K body — keyed on continuation-call identity's
   emission owner (class not covered by F2's lexical-only copy rule)
4. Handler-owned drive transfers into C0, the K883 context, and then
   applies only eliminators after frame 0. Its value then reaches a
   `ResourceBodyResult` match no case accepts (trap identity 40), after
   a correct LIFO 3,2,1. Keyed on which eliminator frames the context's
   result has already passed through, versus which the drive applies.
   That split was hand-derived from `route.continuation_origin`, not
   read from a planner relation.
5. Under forced-Deferred settlement, the drive's transfer into C0
   releases the three nested regions in acquisition order 1,2,3 instead
   of LIFO, then reaches trap 40. Keyed on the deferred-settlement
   plane's ordering of releases for regions acquired inside a
   handler-resumed continuation, which the T'' route (recorded for the
   call target and capture claims only) does not carry.

**§1b: YES.** The handler-owned drive (F1,
`drive_handler_owned_deferred_response` -> `drive_response_at_source`)
lowers Vis899's K body a second time, inline in the handler owner
Spec(3)'s function. Every per-emission plane was derived under the
invariant that a construct is lowered in exactly one function, its
planned emission owner. Entries 1 and 3 hit it on the call-declaration
plane, for the lexical and descent owner classes. Entry 2 hits it on the
availability plane, for the lexical class. At entry 5 the Architect
sharpens it (`evt_d6kxxtgevwbw`): the handler-owned drive is a resumption
of K that the planner's planes do not see as a planned route, and each
entry hand-recorded one more aspect (the owner gates, the call target and
capture claims, the eliminator composition, the deferred settlement
order). The Architect ruled the structural closure
(`evt_7knq6fn8km9ra`): the drive becomes a planned resumption call that
every plane consumes.
