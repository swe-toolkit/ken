---
id: RT-NATIVE-SETTLEMENT-PLACEMENT-KEY
title: "PX10 B1a's direct-style bind moves a recursive worker's consumer across the worker boundary, so one static continuation recurs once per specialization clone, and the native planner refuses the repeat: the detached exit chain's (origin, frame) key is shared by clones 10, 8, 6, 4 and 1 of one worker, and the bracket plane admits one returned-exit owner per static region. Key both settlement planes on (origin, placement), with one shared walk deriving the frame list and the required consumer, so activations of one placement are planned once and kept apart at runtime by their frame"
status: active
owner: runtime
size: L
tier: T1
gate: architect
depends_on: []
blocks: [PX10-NATIVE-SPAWN-PARITY]
github: null
origin: "Architect ruling (i) evt_5b1gt2a3cz0f0 on PX10 B1a's F-NEXT call (evt_a0asf7f4xt0w), with Research evt_1kywnb81qgrqm and addendum evt_3xevbv79j2pad; Steward scope read evt_h4eybf0c1ydg recut it into its own node, landing before B1a resumes. L1 objective: the Linux ABI program. Steward-filed per COORDINATION section 2."
---

# Settle by placement, not by label

## Objective

Every static settlement point in the PX10 native planner is keyed on
(origin, placement). Activations of the same placement share that key,
are planned once, and are kept apart at runtime by their frame, never by
a second static key. With this in place, B1a's direct-style bind
reaches the boundary arm on the three F2 rows and refuses there by name.

This is the standard discipline: a return point is a static label, a
specialization is a static placement, and an activation is a runtime
frame. Each plane keys on exactly one of these (Research
`evt_1kywnb81qgrqm`).

## Settled inputs (B1a base `a966f0ee5` plus wip2 `723942de`)

- **Plane A, the detached exit chain**
  (`planning/static_transition/responses.rs`).
  - `detached_post_call_consumer_frames` (`:7056`) pushes the worker
    fold's `target.key.continuation_origin` once per worker boundary.
  - `first_computational_consumer`
    (`planning/static_transition/aggregates.rs:448`) skips that origin
    at the same boundary. The two derivations disagree there (D0-6,
    `evt_3tb2baxz49kea`).
  - Each frame becomes a step keyed (eliminator origin, checked frame
    id). Three live gates refuse a repeated key:
    - `:495`, "repeats its selected-caller exit";
    - `:530`, "a caller-completed required consumer has more than one
      exact incoming edge";
    - `:7043`, "a self-defining required consumer has more than one
      exact incoming edge".

    `:6798` is vacuous, because `position()` returns the first match.
  - D0-7 (`evt_38w050r0b331w`) measured the repeats as distinct
    selecting specializations of one recursive worker: targets 10, 8,
    6, 4 and 1, all with continuation origin 1165. It found zero
    same-target repeats in 16 lists across 3 rows.
- **Plane B, the bracket returned-exit owner.** `brackets.rs:263`
  refuses "a bracket region has two returned-exit owners". It walks
  `pending_vis_settlements.bracket_exits`, built in `returned_vis.rs`
  (`:257`), and admits at most one owner per static region id. The
  tree-form lowering hit this same gate on these rows (§1b entry 19),
  so keeping recursive workers tree-form is not a closure (Architect
  `evt_5b1gt2a3cz0f0`).
- **Plane C, the relay and K key** (§1b entries 17 and 18) carries the
  same predicate. It is in scope: after planes A and B, the F2 rows
  reach its call-coverage gate (`units.rs:9779`; Steward
  `evt_1kd86qmt5zw1t`).
  Entry 18's committed row is a must-not-regress control.
- **Retained, not product here:**
  - B1a's wip2 (three uncommitted files, the Let form) and
    DeclarationCallResult, at `a966f0ee5`;
  - the D0-6 and D0-7 env trials, which are D0-only.
- **The rows.**
  - The F2 rows are `d0_argv_env_pipe`, `d0_live_child` and
    `d0_v_argv_stdout_null`.
  - They exist today only in the D0SITE scratch fixture, so they are
    not committed evidence.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

Planes A and B keyed on (origin, placement), with one shared walk for
the frame list and the consumer, and plane C repaired as the Architect
rules. The F2 rows are committed, and the
mutations below are recorded. All of it goes in one candidate, stacked
under B1a.

The candidate also carries nested Deferred settlement (Steward
`evt_5xqm5nyd0ah7n`; no gated-off partial landing):

- **Plane C's closure edge is E3**, emission containment. A row whose
  selected caller is emitted in a specialization that is the
  selected-call target, or the k-context unit, of an R member joins R
  (Architect `evt_202qekyxt9rdw`).
- **A nested region settles at its own S.** A region with a parent
  whose mode is Deferred settles through the InPlace path when it has a
  validated row matching its checked region. Without that row, it keeps
  the `brackets.rs:281` refusal. Root regions still defer to the driver
  (Architect `evt_500h9ww4pam4h`).
- **The product shape** follows the C-20 D0's LIFO-PARITY branch, as
  the Architect rules it.

The C-28 recut (Steward `evt_527p7m4kfdbc0`) located F's defect (D2,
`evt_6t5mkjt1y7znx`). F's body-tail request enters a pure-forward carried
clause whose IH lowers in place: it jumps the header with k under the
CHECKED control, both tags miss, and the checked-answer fallback releases
Buffer1 early. Step 0 found a genuine handler between the clause and the
host (HANDLERS-PRESENT, `evt_4n6yrejbn6f1a`), so the direct forward was
withdrawn. D0 S0b returned RE-ENTRY-ABSENT: the caller's handler is
reachable, but no re-entry unit exists, and the chain is two clauses
(S1165, then C986 in argv; S606, then C427 in live). The lawful forward is
`RT-NATIVE-FORWARDED-CONTINUATION-REENTRY`. This candidate lands
fail-closed (Steward `evt_6rkw3hdx6m2jn` and the 14:50Z recut):

- **The trap.** The boundary arm's in-place IH edge, jump header(k, 1),
  becomes a named trap. The boundary arm is the pure-forward arm
  (`brackets.rs:429` `forwarding_vis_arm`) whose clause is the outermost
  carried ComputationalMatch of its emitting function: C986 in argv, C427
  in live. Inner pure-forward clauses keep their by-value forward. The
  checked-answer fallback stays untrapped.
- **F's result.** Each F row refuses natively by the trap's name at the
  boundary clause, after the inner S1165 or S606 Vis case has executed, in
  S0b M4's order, shown by ordered markers. The fallback count is 0.

- **Residual, not in scope.** A recursive non-Unit Nat result used
  after a later Console effect fails native-build closed with N0003
  (`BoundaryCarrier`; Architect probe `natrec-inc2/after-effect.ken`,
  `evt_5b1bpcemby1jm`). It belongs to B1a's runtime half.
- **WIP2's erasure half** (`erasure.rs`,
  `erasure/performed_call_scrutinee_tests.rs`) stays with B1a's WIP2,
  because `RT-NATIVE-NAT-RECURSION-HOSTIO-COPRODUCT` is parked behind B1a
  (`evt_50705xx614069`).

## Acceptance

- **AC-0 (D0, before any build).** At the `:263` gate on the 3 F2 rows,
  print each colliding owner's `k_specialization()`.
  - If the two owners share a placement, plane B's rekey is wrong, and
    the work returns to the Architect.
  - The Architect also rules:
    - plane A's placement for a step inside a context (its emission
      owner);
    - whether `:6798` is rekeyed or deleted;
    - whether plane C is in scope.
- **AC-1 (measured, not committed).** On any production base the Process
  gate refuses F before object emission
  (`compiler_driver.rs:3001`-`3024`, `host_effect_lowering_unavailable`),
  so the trap is observed under the C-28 diagnostic overlay: the gate
  condition and the stream and Process availability, uncommitted, named
  by sha256 in the report. Under it, each F2 row refuses natively by the
  trap's name at the boundary clause, as stated above, with the fallback
  count 0. This is B1's posture (`evt_5vsaybryes08b`; Steward
  `evt_1kpgr76vwez5g`). QA re-runs it at the exact tip. Native parity on
  these rows is `RT-NATIVE-FORWARDED-CONTINUATION-REENTRY`'s.
- **AC-1c (committed).** The F2 fixtures, with a committed native test
  whose expectation is today's production refusal naming the seven
  unavailable operations, for B2 to promote.
- **AC-2.** These stay green:
  - the smoke 9/9, both G controls, px8ta 3/3 and the 9 checked_ih rows;
  - the two-print parity row (`rt_selected_pending_call_admission.rs:303`)
    and span freeze 2/2;
  - the green-three row;
  - entry 18's row;
  - B-closure's census pin and six-case matrix.

  They are regression controls, because green-three reaches none of
  these sites.
- **AC-3.** Each mutation compiles and, under the AC-1 overlay, moves the
  F2 rows' refusal off the trap's name:
  - M-A, which restores the (origin, frame) exit key, at `:7043`;
  - M-B, which restores one owner per region, at `:263`;
  - M-C, which restores the separate consumer search past the
    boundary, with "no exact incoming edge";
  - M-T, which removes the trap and restores the early Buffer1 release;
  - M-P, which places the trap at the inner clause, and the ordered
    markers show the wrong placement.
- **AC-4.** Full-workspace CI is green on the exact candidate. Edge
  equality is not evidence; native parity is the gate.

## Inventory (node §1a)

1. Plane B's owner check keyed on `k_specialization`, the response's
   specialization, not the placement it settles (`evt_yz6rxwdke9f6`).
2. After planes A and B, F2 argv and v_null refuse at the
   response-owner call-coverage gate (`units.rs:9779`). Owner
   `StaticResponseOwnerId(2)`'s selected caller (emission
   `Specialization(8)`, target `Specialization(10)`) has no recorded
   verified incoming call and no candidate disposition, although its
   source Construct was lowered. Keyed on `ContinuationCallIdentity`,
   the selected caller (`evt_3aax4002wm3yt`).
3. The checked-IH environment transport moves a K call's emission to
   its destination owner, but three consumers read the planned emitter
   or a plan-wide fact instead. The detached-seat exit and the
   candidate close key on plan-wide "is a transport source", and plane
   B's settlement gate keys on `k_identity().emission_owner()`. On F,
   all 21 self transports (emitter == destination) are unemitted. An
   env-gated arm consult restores native parity on all four rows, but
   six producer-reached ones stay dormant. On e9fbbca83 the plane B
   gate refuses 18/19 main checked_ih parity tests. Keyed on the
   planned emitter, not the code identity that emits the call
   (`evt_662q5ektq3zcx`, Architect `evt_1dnbnhbgs176n`).
4. Detached seat pass-through keyed on owner comparison (live owner
   versus placement) instead of on this Function's emission evidence.
   Keyed on owner identity (`evt_dd551d27htbj`, Architect
   `evt_7gs4bp33yneey`).
5. Self relocation withdrawn at the post-ownership builder only, while
   the pre-schema source stratum is a parallel derivation with its own
   readers, pins and equality invariant. Keyed on transport-source
   membership, used as if it were relocation (`evt_3tfjfceyg80y9`,
   Architect `evt_6rnvdpnq9b9vk`).
6. Escaping K calls are excluded from
   `preselect_static_response_callers` per caller (SELF, CTX, SOLE,
   exact ORACLE). With exactly the measured seats excluded, the
   call-coverage gate (`units.rs:9807`) refuses owner 2's next context,
   whose selected caller is the next link of a worker-body chain (worker
   body_origin = previous producer_result_origin). Keyed on a single
   caller's seat property, used as if escape were per-caller rather
   than transitive (`evt_brqpp3zm90sp`, Architect `evt_p77sfegqy2k2`).

## Stop conditions

- A kernel or trust change.
- A change to the Process gate, `HostOpV1::availability` or AC-2g.
  Those belong to B2.
- PX10 §1a reaching 21 (it is 18 at this recut). That triggers the
  Architect's re-ask to Research.

Checks: 2 fired: the refusal sites `responses.rs:495`, `:530`, `:6798`,
`:7043`, `brackets.rs:263` and `aggregates.rs:448` were read at
`a966f0ee5`; plane A's fan-in is those gates, and plane B's is the owner
map. 6 fired: the population is the 3 F2 rows, as D0-7 measured it, and
no claim is made beyond them. 8 fired: the F2 rows are scratch-only, so
AC-1 commits them, and each mutation names the gate that must fire.
