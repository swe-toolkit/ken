---
id: RT-PX8DS-SAME-DEPTH-P1-HANDLER
title: "px8ds_real_same_depth_path_runs_exact_edges is red at base: native refuses at ObjectEmission because the relay P1 sources of px8ta owner 0 have no admitted handler, while the interpreter exits 0. Give those sources a settling route and un-ignore the row"
status: ready
owner: runtime
size: M
tier: T1
gate: architect
depends_on: [RT-NESTED-RELAY-VIS-NATIVE-SETTLEMENT]
blocks: []
github: null
origin: "Finding outside RT-NESTED-RELAY-VIS-NATIVE-SETTLEMENT (its frame, 'Finding outside this WP'; expected refusal evt_62wab4peq1htm). Serves the L1 objective, clearing the ignored runtime rows (operator 2026-09-17). Steward-filed per COORDINATION section 2."
---

# The same-depth px8ds path runs natively

## Objective

`px8ds_real_same_depth_path_runs_exact_edges`
(`crates/ken-cli/tests/px8ta_oriented_subcontinuation.rs` ~:453) is
un-ignored and passes: one Console observation and one release on the
false branch, as on the interpreter.

## Settled inputs (RELAY frame and `evt_62wab4peq1htm`)

- px8ta owner 0 (base `Spec(0)`, relay Vis 370) has P1 sources 647 and
  600 with no admitted handler. At base the row fails at `ObjectEmission`
  with "a specialized response owner did not materialize an exact
  HostResult"; the interpreter exits 0.
- After RELAY it reaches that WP's typed planning refusal instead, as an
  expected refusal.
- Its entry in `.github/ignored-test-exemptions.toml` classes it
  `policy-cost` and calls it runnable, which is false while it is red.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

D0 first: the Architect measures why sources 647 and 600 have no
admitted handler under RELAY's provenance rules and rules the settling
route. Then the route, the row un-ignored and its exemption entry
corrected to match.

## Acceptance

- **AC-0 (D0, measure).** The P1 provenance of owner 0 at the landed
  base, and which RELAY clause would settle it.
- **AC-1.** The row passes un-ignored, on its large-stack thread.
- **AC-2.** RELAY's 36-row roster and controls are unchanged; targeted
  runs only.
- **AC-3 (mutation, QA).** Removing the new route returns the row to the
  typed planning refusal, not a run-time trap.

## Stop conditions

- The route needs a kernel, trust or spec change.
- Any RELAY roster row changes outcome.
