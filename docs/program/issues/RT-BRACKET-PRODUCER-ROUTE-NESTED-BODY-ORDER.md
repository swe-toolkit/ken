---
id: RT-BRACKET-PRODUCER-ROUTE-NESTED-BODY-ORDER
title: "HELD INPUT to RT-BRACKET-LOWERING-AND-D0-REFUTER (child 3). At px8ta depth 3, natively and under either settlement mode, the existing computational-producer route never drives the middle bracket's body: after the second acquire it reaches the outer region's settlement S524, whose ResourceBodyResult scrutinee at origin 509 is unwritten (tag 0), and it traps with planned identity 40. The interpreter opens 1, 2, 3 and releases 3, 2, 1. Make the producer route drive nested bracket bodies in order and write each body result before its settlement runs. THIS NODE DOES NOT LAND ALONE: its reviewed tip is assembled by child 3."
status: active
owner: runtime
size: M
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

The localization D0 is done (`evt_5hhmx816x4ac8`, D0-2
`evt_3z9erek910n8w`). F1, the route selection with owner(899) = Spec(3),
is retained from checkpoint `edc32de81` (tree `5bd10e81`).

**Recut at the 3rd stop (Architect `evt_687xbfdep022z`, on Research
`evt_57nv4616cf07w`).** Replace the inline re-lowering of a bounded P2
Deferred response's K body at the handler-owned drive with a planned
unit transfer: `DrivenKResume::TransferToUnit`, that is
`call_declared_unit(k_body, [response] ++ captures)` followed by the
composed eliminators. F2 and F3 are withdrawn, and the rebuild starts
from `edc32de81`. The InPlace bracket S at `bracket_emission.rs:68`
stays inline (`DrivenKResume::InlineInPlace`, its tail byte-identical).
A D0 of the edge, the emission and LIFO comes before any build.

## Acceptance

- **AC-T1 (D0, non-advancing).**
  - (a) Spec3's `function_local.unit_calls` has key 883, with
    `frame_owner` Unit(Pred4) and the slot run Parameter plus 7
    captures, matching M2. Name the observed caller.
  - (b) Under forced Deferred, Pred4 is a defined function in the
    object. List the planned emission owners of S874 and S747.
  - (c) At the 899 drive site, list the open settlements in stack
    order. Each one reached from subtree(883) either opens and closes
    inside K, or is open before the drive and still open there in stack
    order. Anything else is a LIFO violation.
- **AC-T2.** As-shipped depth 3, native: opens 1, 2, 3, strict LIFO
  releases, exit 0. Each release's handle equals that of the open it
  pairs with, and all three are distinct.
- **AC-T3.** The same under forced Deferred, which re-runs the stop-3
  counterexample.
- **AC-T4 (mutation).** Under forced Deferred, forcing the P2 caller to
  `InlineInPlace` reproduces the stop-1 refusal exactly, and is then
  byte-restored.
- **AC-T5 (carried from F1).** Depth 2 and `rt_native_bracket_body_once`
  are unchanged. The owner(899) = Spec3 planner pin reddens on an A-only
  or B-only revert. The owner readers are reconciled. The closeout shows
  no unsettled candidates in either mode.

## Stop conditions

- The fix needs a new carrier, return protocol, KRET lane or host
  operation: stop to the Architect and the Steward.
- (a) cannot name the skipped step from runtime markers: stop with the
  marker trace. Do not infer the step from static emission counts.
- Do not reopen `RT-PLANNER-KRET-GRAFTED-SPINE`, and do not add a census,
  carve-out, narrowed matrix or fallback.

## Hard-stop inventory (§1b)

§1a: 5, and the child 3 aggregate is 6 (Architect `evt_d6kxxtgevwbw`):
the Research trigger, ruling held for the advisory. WIPs `16cc5ee62` and
`1a4ed40ea` are not candidates; at `1a4ed40ea` the as-shipped depth-3
test exits 0 with releases 3,2,1. The 3rd stop is ruled as
closure (T), transfer, which is the recut in the Deliverable
(`evt_687xbfdep022z`), realized as T'' (the drive calls K883's one
planned emission C0 through a planner-recorded route; `evt_3tma32z29wz46`,
L1′ `evt_3ztrhbzpvyb0`). The F2/F3 WIP `9ecf2948d` is superseded. The
node's next Research trigger is its 6th advancing stop.

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
order). The Architect's lean, not yet a ruling, is a structural closure:
the drive becomes a first-class planned emission that every plane
consumes.
