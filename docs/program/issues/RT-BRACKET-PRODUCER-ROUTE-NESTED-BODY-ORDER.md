---
id: RT-BRACKET-PRODUCER-ROUTE-NESTED-BODY-ORDER
title: "HELD INPUT to RT-BRACKET-LOWERING-AND-D0-REFUTER (child 3). At px8ta depth 3, natively and under either settlement mode, the existing computational-producer route never drives the middle bracket's body: after the second acquire it reaches the outer region's settlement S524, whose ResourceBodyResult scrutinee at origin 509 is unwritten (tag 0), and it traps with planned identity 40. The interpreter opens 1, 2, 3 and releases 3, 2, 1. Keep the as-shipped InPlace route releasing 3, 2, 1, and refuse nested Deferred settlement at planning instead of ordering it (Architect evt_3d42r13m743t). THIS NODE DOES NOT LAND ALONE: its reviewed tip is assembled by child 3."
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

> **Amended 2026-10-23 (Architect `evt_28qc7qjzewbhd`, Steward
> `evt_1ptt5bsccyk0`).** This node is the last piece of the single
> link 3 + child 3 candidate, built fresh on that branch's tip. Its held
> checkpoints (`1a4ed40ea` and earlier) are references only. The
> Architect rules its port once BoundaryCarrier (cause D) is closed.
>
> **HELD INPUT.** This node does not land on its own. Its reviewed tip
> is assembled into child 3, like 2a and 2b. No publication and no
> partial landing.

# Nested bracket bodies run in order on the producer route

**Recut 2026-10-24 (Architect `evt_2j4fxp3ekyr03`, on Research
`evt_4a2ee3es6nvdw`).** The combined-tree D0 (child 3 §2) found a
main-green regression from the T'' port: the two drive-seat tests in
`abi_s6_mapping_surface_native.rs` exit 1 natively where the interpreter
exits 0 (red from `e31be7b05`, green at `2012431be`). Item 2 of the
Deliverable is replaced by R-a and R-b, and R-10 to R-12 join the
Acceptance. Build on the combined branch
`wp/RT-BRACKET-PRODUCER-ROUTE-NESTED-BODY-ORDER-combined` at
`c68c77a3e` (tree `532654446`, fresh on `d26c7964e`, with the
pre-registration `c22f4861d` cherry-picked). Held refs are never moved,
rebased or landed.

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

**Closure at the β′ ruling (Architect `evt_3d42r13m743t`, on the M
return `evt_35kdvqg0h8p04`): nested Deferred settlement is refused at
planning, not ordered.** Measured at `1a4ed40ea`: a Deferred settlement
lowers as an ordinary Construct and does not own its release, so nested
Deferred regions release FIFO at depth 2 and 3. No unforced corpus row
has a Deferred region with a parent; as shipped, depth 3 is InPlace and
releases 3, 2, 1.

1. **The refusal.** In `BracketSettlementControlView::from_plan`
   (`brackets.rs` ~171), after `mode` is computed and after the
   test-support override, so that forced runs observe it:

   ```rust
   if mode == BracketSettlementMode::Deferred && region.parent.is_some() {
       return Err(planner_error(
           "a nested bracket region has no in-place settlement owner; \
            deferred settlement order is not planned for nested regions",
       ));
   }
   ```

   Depth-0 Deferred is untouched.
2. **Retain** F1, P0 and V, the T'' route with L1′ and L2′, and the
   published `PlannedHandlerResumptionCall`'s `route` and `parameters`.
3. **Drop** the `PlannedResumptionSettlement` list and its validator
   clauses. With nested Deferred refused they have no consumer.
4. **R-a. Do not record the eliminator run.** Remove
   `PlannedHandlerResumptionCall.eliminators`, the lexical
   source-parent walk that derives it, the validator's eliminator clause,
   and the lowering's `frame_origins != record.eliminators` check. The
   frame run is the live stack the lowering already threads through named
   calls. Keep the head check against `route.continuation_origin`.
5. **R-b. Record where K executes, keyed on response ownership.** Over
   every handler-owned Deferred row whose Vis K's body reaches without
   crossing a closure: **Transfer** when every such row is admitted to the
   target context's enclosing specialization (vacuously, when there are
   none); **AtHandler** when every such row is admitted to the handler, in
   which case K is lowered inline under the live stack; otherwise refuse
   at planning. Route, parameters, claims, P0, V, L1′ and L2′ are
   unchanged for Transfer records.

The Architect's verified patch for 4 and 5 is
`local/architect-shared/rt-combined-d0/s9/fix-c68.patch` (sha256
`2bbcc828…16463e2`, four files, +67/-31; hunks verbatim in
`evt_2j4fxp3ekyr03`).

## Acceptance

- **AC-0, AC-0b and M (D0s, non-advancing)** are measured and ruled
  (`evt_2nwrvm9038k7t`, `evt_7w488rtt4yj9j`, `evt_3d42r13m743t`).
- **T''-1..16** stand (Architect `evt_3tma32z29wz46`, `evt_3ztrhbzpvyb0`,
  `evt_1t43pvv4apd6`).
- **R-1′.** Forced-Deferred depth 3 gives the planner refusal, naming the
  nested region. Today it exits 1 at trap 40.
- **R-2′.** Forced-Deferred depth 2 gives the same refusal. Today it
  exits 0 releasing 1, 2.
- **R-3′.** The eight unforced depth-0 Deferred sources keep their exact
  observed behaviour: multileaf, transport_mapping,
  one_buffer_residual_layout, per_emitter_one_bracket,
  two_bracket_distinguishable, ignored_two_buffer_witness,
  px8x_single_schema_observation and rt_parity_native
  buffer_allocate_malformed_capacity.
- **R-7.** As shipped, depth 3 exits 0 with terminal None and releases 3,
  2, 1.
- **R-8.** The unforced census re-run is byte-identical (InPlace 30,
  Deferred 12, Inline 0), with no new refusal.
- **R-9 (mutation).** Deleting the check makes R-1′ fail again, back to
  the trap-40 or FIFO observation.
- **R-10.** Both drive-seat tests in `abi_s6_mapping_surface_native.rs`
  pass, and the whole file passes 12/12.
- **R-11 (mutation).** Forcing Transfer reddens R-10 with native exit 1,
  (1,0) against (0,0). Restore it byte-identically.
- **R-12.** The planner's coverage on the candidate is: abi_s6 record 580
  AtHandler and 557 Transfer, px8ta record 899 Transfer, body-once 0
  records.
- **Reruns on the exact candidate** (ancestor approvals do not carry):
  the `e1d947edd` partition rows and the `a408136b9` P2 rows in
  `rt_parity_native`, β′'s forced controls 2/2, T''-15 and T''-16.
- **Stated gap, not a criterion.** Forcing AtHandler on px8ta's record
  899 stays green, so no measured consumer needs the Transfer arm. Its
  necessity rests on T''-4, T''-15 and T''-16.

The child-3 D0 (§2 of `RT-BRACKET-LOWERING-AND-D0-REFUTER`) and closeout
then resume on this node's tip.

**Precommit.** If R-8 shows an unforced row refused, there is a nested
Deferred region in the corpus: a query, which the Architect rules. The
next advancing stop is the node's 9th, a Research trigger.

## Stop conditions

- The fix needs a new carrier, return protocol, KRET lane or host
  operation: stop to the Architect and the Steward.
- Do not order nested Deferred settlement, and do not add a fallback that
  lowers it. The refusal is the ruled closure.
- Do not reopen `RT-PLANNER-KRET-GRAFTED-SPINE`.

## Hard-stop inventory (§1b)

§1a: 13, child 3 aggregate 14 (Architect `evt_3kd7n1m3611zg`: class C,
the handler resumption route required for every admitted handler-owned
Deferred row; advancing, next Research trigger at 15). Before it: 12,
aggregate 13 (Architect `evt_4hendk9snt9em`, Research `evt_6xjyeh42hvapg`;
stands as published; its diagnosis was refuted and the rescope withdrawn
by the non-advancing ruling `evt_7pa9km38hak7s`). Before that: 11,
aggregate 12 (Architect `evt_5aw3s64w0zan7`: the capsule footprint
declared from a filtered projection). Before that: 10, aggregate 11
(`evt_2rp8t2pz1rz4f`: the respin's attribution baseline failed on a
mis-specified criterion). Before that: 9, aggregate 10
(`evt_61nznjb9jvcer`: the df26d3135 CI red; Research `evt_yxd12w9d32sx`,
same-WP respin on `76fc5d236`). Before that: 8, aggregate 9 (`evt_62kr796gv7yjf`;
Research `evt_4a2ee3es6nvdw`, ruling `evt_2j4fxp3ekyr03`). The 7th stop is the unforced binary after the β′
closure (runtime-leader `evt_3k3pjar47z8zj`). It advances with no hold.
Only the non-advancing
baseline attribution B (B0 `0919ab3d6`, B1 `1a4ed40ea`, B2 the stop
configuration) is authorized. The next Research trigger is the node's
9th stop or child 3's 9th. The 6th was at R-1 (runtime-leader
`evt_1h4jhr9pnqr8j`, implementer `evt_70paqtnpebmqz`); Research advised
on it (`evt_7svg4vbndmhqx`, `evt_5qx5b0c76anpn`), and the closure is the
β′ ruling in the Deliverable (`evt_3d42r13m743t`). At
the 5th stop the aggregate was 6 (`evt_d6kxxtgevwbw`), and that advisory
gave the structural recut in the Deliverable (`evt_7knq6fn8km9ra`).
WIPs `16cc5ee62` and `1a4ed40ea` are not candidates; at `1a4ed40ea`
the as-shipped depth-3 test exits 0 with releases 3,2,1. The 3rd stop
is ruled as closure (T), transfer, which is the recut in the Deliverable
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
6. With the corrected β record published and every validator clause
   passing, forced-Deferred depth 3 still releases handles 1, 2, 3 and
   exits 1 at trap 40. The reader executes the record's route,
   parameters and eliminators, but "does not enter region2's settlement
   continuation first: it still calls the route C0 once"
   (`evt_70paqtnpebmqz`). Keyed on how the planned resumption enters the
   deepest checked settlement continuation (region2, K Spec0) before its
   parent chain, versus the current single C0 call. Evidence:
   `local/rt-bracket-producer-authenticity-d0/child3-assembly/recut-r1-node6/stop.txt`;
   the WIP stays `1a4ed40ea`.
7. After the β′ closure (nested-Deferred refusal, R-1′, R-2′, R-7, R-8
   and R-9 met), the unforced `px8ta_oriented_subcontinuation` binary has
   3 failures. `px8ds_real_same_depth_path_runs_exact_edges` and
   `px8ds_retired_flat_order_does_not_gain_m4_representation` fail
   ObjectEmission with `PlannerInvariant: body tail leaves the region
   unit` (bracket `body_walk`, `brackets.rs` ~407/~419).
   `public_one_level_bracket_finishes_and_releases` sees 0 versus 2
   source captures. Keyed on: unattributed, baseline owed
   (`evt_3k3pjar47z8zj`).
8. Combined D0, read-write-read: the handler drive's eliminator run
   is derived by a lexical source-parent walk from the Vis to its
   closure (empty here), while the route's continuation (CM 350) is the
   specialization's continuation reached through a named-proc call; T''
   executes wrong (native exit 1), beta-prime refuses. Keyed on:
   lexical source ancestry of the Vis, versus the specialization
   plane's continuation relation.
8 (correction). The e31 wrong execution is owner escape, not frames:
   T'' transfers K into a context whose enclosing specialization (Spec0)
   does not own a handler-owned response K performs (Vis 557, Spec2),
   so that response is never driven. The lexical eliminator walk
   explains only c68's refusal. Keyed on: target context selected by
   source position (unique selecting unit at the Vis), versus ownership
   of K's nested responses.
9. T'' handler-drive route keyed on a unique continuation-unit lookup
   at (K body, Vis); unit-less P1 rows lost their established in-place
   drive. Keyed on: specialization-table lookup by source position.
   Shares entry 8's predicate (Architect `evt_61nznjb9jvcer`).
10. The respin's footprint pins attributed each member by immediate
   adjacency in a unified recorder log; the branched Route1 row follows
   a ResidualArrival, not the Consumed event. Keyed on: emission
   position in the log. Shares the entry 8 and 9 family (Architect
   `evt_2rp8t2pz1rz4f`); segment attribution with a counterfactual run
   replaces it (`evt_4mm2j7qj6tkb4`).
11. The capsule footprint F was enumerated by hand from the filtered
   site-A projection; the complete recorder log also carries
   Route1(1286,true) x8, which vanishes under M-2'. Keyed on: a
   vocabulary subset of the recorder log. Shares entry 10's predicate,
   a population declared from a partial projection; F is now the
   measured complete-log delta against M-2' (Architect
   `evt_5aw3s64w0zan7`).
12. Bracket settlement membership was decided per static Vis origin;
   Vis 537's ordinary owner-4 instance left ordinary ownership although
   only 975 is Consumed. Keyed on: static source origin, versus the exact
   settlement context set from `marked_bracket_response_contexts`
   (Architect `evt_4hendk9snt9em`).
   12-corr: M-3 refuted the static-origin key as the cause; the three
   reds have three causes (R2, px8f, px4b below). R2 is a pin keyed on
   main's representation of an intended ownership move (migrated); px8f
   is a mutation hook and a suppressed plane on a consumer the WP no
   longer goes through; px4b is `lower_expr` stack-frame growth, closed
   by an `#[inline(never)]` hoist. They share no predicate. Carry:
   per-instance settlement keying once a witness exists (Architect
   `evt_7pa9km38hak7s`).
13. Handler resumption route required for every admitted handler-owned
   Deferred row, keyed on admitted ownership; transfer feasibility (K's
   captures held by the handler frame) not consulted. Plus control hook
   keyed on the pre-move Deferred plane (same ownership move as R2)
   (Architect `evt_3kd7n1m3611zg`). Not a third-entry checkpoint; the
   predicate question is next due at 15.

**Shared predicate, entries 4, 8 and 8 (correction)** (Architect
`evt_2j4fxp3ekyr03`): the drive's composition, both its frames and its
execution site, was selected by source position, not read from the plane
that owns K's continuation. R-a and R-b close it: the frames are the
threaded continuation and are never re-derived, and the execution site
is keyed on response ownership.

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
