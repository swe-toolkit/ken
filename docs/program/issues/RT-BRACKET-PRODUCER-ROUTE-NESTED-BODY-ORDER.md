---
id: RT-BRACKET-PRODUCER-ROUTE-NESTED-BODY-ORDER
title: "HELD INPUT to RT-BRACKET-LOWERING-AND-D0-REFUTER (child 3). At px8ta depth 3, natively and under either settlement mode, the existing computational-producer route never drives the middle bracket's body: after the second acquire it reaches the outer region's settlement S524, whose ResourceBodyResult scrutinee at origin 509 is unwritten (tag 0), and it traps with planned identity 40. The interpreter opens 1, 2, 3 and releases 3, 2, 1. Make the producer route drive nested bracket bodies in order and write each body result before its settlement runs. THIS NODE DOES NOT LAND ALONE: its reviewed tip is assembled by child 3."
status: ready
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

The producer-route repair that makes depth 3 run as the objective
states, on its own branch, whose base the Architect rules with the fix
design.

Its first step is a localization D0, in scratch, run in separate
processes. It is non-advancing.
- **(a)** At depth 3 under forced Deferred on `8d3731a3d`, emit runtime
  markers at each producer-trampoline step and each body-continuation
  entry. Name the step where the middle bracket's body (region 1, S747,
  the second held acquire) is skipped, and the write that should have
  filled the origin 509 scrutinee.
- **(b)** The same markers at depth 2 on main. This tests whether
  main's depth-2 release of 1 before 2 is the same mechanism
  (`RT-SUBCONTINUATION-LIFO-RELEASE-ORDER`, merged).

The Architect rules the fix design on that measurement. If (a) places
the defect in child 3's planning (`bracket_rows`, seat-owner source
(f)), the work returns to child 3 and this node closes.

## Acceptance

- **AC-1.** px8ta depth 3, natively, in its own process, gives the
  interpreter's sequence: opens 1, 2, 3, releases 3, 2, 1, exit 0.
  Paired: reverting the fix gives back the measured trap 40 at origin
  509 with tag 0.
- **AC-2.** Depth 2 keeps its current native observation, and the five
  `rt_native_bracket_body_once` tests pass, under both settlement modes.
- **AC-3.** The fix names the producer-route write that fills each body
  result before its settlement, with no second derivation of the body
  order. A mutation that drops that write reddens AC-1 on the origin
  509 match.

## Stop conditions

- The fix needs a new carrier, return protocol, KRET lane or host
  operation: stop to the Architect and the Steward.
- (a) cannot name the skipped step from runtime markers: stop with the
  marker trace. Do not infer the step from static emission counts.
- Do not reopen `RT-PLANNER-KRET-GRAFTED-SPINE`, and do not add a census,
  carve-out, narrowed matrix or fallback.
