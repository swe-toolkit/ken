---
id: RT-NATIVE-SETTLEMENT-PLACEMENT-KEY
title: "PX10 B1a's direct-style bind moves a recursive worker's consumer across the worker boundary, so one static continuation recurs once per specialization clone, and the native planner refuses the repeat: the detached exit chain's (origin, frame) key is shared by clones 10, 8, 6, 4 and 1 of one worker, and the bracket plane admits one returned-exit owner per static region. Key both settlement planes on (origin, placement), with one shared walk deriving the frame list and the required consumer, so activations of one placement are planned once and kept apart at runtime by their frame"
status: ready
owner: runtime
size: M
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
reaches native parity on the three F2 rows.

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
  same predicate. It is in scope only if AC-0 shows the F2 rows reach it.
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
the frame list and the consumer. The F2 rows are committed, and the
mutations below are recorded. All of it goes in one candidate, stacked
under B1a.

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
- **AC-1.** The three F2 rows are committed as named rows. Under B1a's
  Let form each is native-Ok, with native exit and stdout equal to the
  interpreter's.
- **AC-2.** These stay green:
  - the smoke;
  - the green-three row;
  - entry 18's row;
  - B-closure's census pin and six-case matrix.

  They are regression controls, because green-three reaches none of
  these sites.
- **AC-3.** Each mutation compiles and reddens the F2 rows:
  - M-A, which restores the (origin, frame) exit key, at `:7043`;
  - M-B, which restores one owner per region, at `:263`;
  - M-C, which restores the separate consumer search past the
    boundary, with "no exact incoming edge".
- **AC-4.** Full-workspace CI is green on the exact candidate. Edge
  equality is not evidence; native parity is the gate.

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
