---
id: DIAG-N-CAUSE-SITE-RECLASSIFICATION
title: "Some native guards that a correct compiler hits on a large but well-formed program are coded N0003 (planner invariant, compiler defect) instead of N0002 (capacity), against spec 70 section 1.2. A 65-deep Let reaches one of them. Census every planner_error and PlannerInvariant site by the Architect's cause predicate, and retag the capacity guards to N0002"
status: merged
owner: runtime
size: M
tier: T2
gate: architect
depends_on: [DIAG-STABLE-ERROR-CODES]
blocks: []
github: null
origin: "Adversary hunt of 967c82d77 (evt_1s53n3xms4g8d). Architect ruling evt_1n4jqfdp6ecd4: a separate follow-up WP, not part of DIAG increment 3. Steward-filed per COORDINATION section 2."
---

# Native refusal codes follow the cause

## Objective

Each native guard carries the code its cause names: N0002 for capacity,
and N0003 only where the guard can fire on an inconsistent earlier stage.

## Settled inputs (Architect `evt_1n4jqfdp6ecd4`, at `967c82d77`)

- **Two funnels.** `planner_error` (`planning/static_transition.rs:765`)
  raises `BackendFailure::PlannerInvariant`, which is N0003. Next to it,
  `planner_capacity_error` (:769) raises `LoweringCause::Capacity`, which
  is N0002. There are 1258 non-test `planner_error` calls and 235
  `planner_capacity_error` calls. Lowering sites tag a `LoweringCause` per
  site. The funnel cannot see the cause, so the fix is at each site.
- **The predicate.**
  - N0002: a correct compiler hits the guard on a well-formed program that
    is large enough. This covers a chosen depth or size bound, and
    overflow of a planned size or offset.
  - N0003: the guard can fire only if an earlier stage produced an
    inconsistent state, such as a missing row, the wrong protocol kind, or
    a value that exceeds a frame the planner itself sized.
- **Named sites.** These become N0002:
  - `abi.rs:2423` (`MAX_PRODUCER_DEPTH` = 64). Measured: a 65-deep `Let`
    gives N0003, and 64 gives Ok.
  - `lowering/core.rs:4723` and `:6591` (`CARRIED_SUFFIX_REENTRY_LIMIT`).
    Increment 2 tagged them `PlannerInvariant`.
  - The pending-Vis offset and discriminant arithmetic at
    `returned_vis.rs:129-138`, `:234`, `:237` and `:241`, which
    `units.rs:3348-3349` already codes as Capacity.

  These stay N0003: `units.rs:3350` (exceeds its activation frame) and
  `returned_vis.rs:229` and `:232`.
- **The population is unmeasured.** A same-line wording grep finds at
  least 20 `planner_error` sites in 6 files. It misses every multi-line
  message, including all three named measured sites. Wording seeds the
  census; it does not decide it.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The Architect's D0 census (`evt_6tm3jf3y8s1f2`, on `e17020310`) is
`architect/work` `16633e5e2:notes/diag-ncause-census-e17020310.tsv`, with
2504 rows. It rules three kinds of change, each limited to the cause or
carrier:

- **18 retags to N0002,** each through `planner_capacity_error`,
  `LoweringCause::Capacity`, or, for `grafted_spine_control_graph.rs`, a
  sibling helper that keeps the `BackendFailure::Module` carrier.
- **31 Fragment arity sites to N0003**
  (`LoweringCause::Fragment` → `LoweringCause::PlannerInvariant`).
- **Kept sites:** the other 118 Fragment sites stay N0001, and 2327 sites
  stay N0003.

The returned_vis lines are corrected there: :125, :128, :130, :132, :134,
:235, :238 and :241, with :229 and :232 kept.

## Acceptance

- **AC-1.** Evidence is per site, and depends on reach.
  - **A reachable site:** a program, or a unit fixture where no program
    reaches the guard, produces the target code where `967c82d77`
    produces the base code. For a retag to N0002 the target is N0002 and
    the base N0003. For a Fragment row the target is N0003 and the base
    N0001. The reachable N0002 sites are abi.rs:2424 (a program: a
    65-deep `Let`), core.rs:4723 and :6591 (fixtures in
    `lowering/core/tests/control.rs`, plus a cause assert), and
    returned_vis.rs:125-134. A Fragment row is reachable when a
    test-only, hand-built inconsistent `RuntimeExpr` reaches its guard.
  - **A no-witness site:** the census row plus a diff limited to the
    cause or carrier substitution, which QA checks against the census.
    These are the 9 N0002 sites the census names: returned_vis.rs:235,
    :238 and :241, grafted_spine_control_graph.rs:432, :806 and :810,
    selected_pending_calls.rs:831, :832 and :870, and units.rs:2312.
    The Fragment rows the census marks `n/a (invariant)` are no-witness
    sites too.
  - **Test-only fixtures are in scope.** Malformed-plan fixtures in test
    code do not breach the cause-only scope. Do not widen a production
    constructor's visibility to build one: a site that needs it, such as
    returned_vis.rs:235, is a no-witness site.
- **AC-2.** Each site kept at N0003 has a one-line invariant argument in
  the census. The `codes.tsv` ledger is unchanged.
- **AC-3 (mutation, QA).** Reverting one retag reddens its AC-1 row. This
  does not apply to the no-witness sites.

## Stop conditions

- A code is added, removed or given a new meaning in the ledger.
- A retag changes control flow, not only the cause carried.
