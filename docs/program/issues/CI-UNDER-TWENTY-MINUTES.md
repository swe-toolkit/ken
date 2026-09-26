---
id: CI-UNDER-TWENTY-MINUTES
title: "Bring Full CI wall time under 20 minutes by splitting the longest multi-case rt_parity_native tests and balancing the native-parity runners by measured duration, with every test and assertion preserved"
status: ready
owner: verify
size: M
gate: architect
tier: T2
depends_on: []
blocks: []
github: null
origin: "Operator 2026-09-26: 'CI is now at 28 minutes. schedule work to bring that down under 20.' Supersedes the active CI-NATIVE-PARITY-DURATION frame, whose measurements predate the current suite. Steward-filed per COORDINATION section 2."
---

# CI under 20 minutes

## Objective

Every Full CI run on main finishes in under 20 minutes of wall time.

## Settled inputs -- measured on main run `36231472089` (`90e53a629`)

- **Wall time 28.4 min.** The critical path is `native-slow
  (rt_parity_native) 4/6` at 28.1 min. Every other job finishes by 21.2
  min, and the eight workspace test shards by 16.9 min.
- **The six parity runners are unbalanced.** nextest summaries: 1/6 775 s,
  2/6 1060 s, 3/6 718 s, 4/6 1534 s, 5/6 1048 s, 6/6 1106 s, for 6,241 s in
  total. Each runner also spends about 1 min compiling. A perfect six-way
  balance is about 1,040 s plus overhead, which is 19-20 min: not enough by
  itself.
- **A few single tests dominate** (each runs many mutations serially):
  - `static_response_context_demand_controls_reach_and_restore` 834 s
  - `composed_return_forward_ret_authority_population_is_exact` 626 s
  - `checked_ih_fresh_result_route_observation_is_forward_and_paired` 602 s
  - `composed_return_forward_ret_authority_controls_refuse` 453 s

  The first three and the fourth share runners 4/6 and 6/6.
- **The partition is by count** (`--partition count:N/6` in
  `.github/workflows/ci.yml`), so it cannot see duration.

Re-measure on a current main run before building. If a settled input is
false, stop and report.

## Deliverable

A CI change, plus behaviour-preserving test splits, that brings the
critical path under 20 minutes with margin.

## Acceptance

- **AC-0.** Post the job timeline and the per-test durations of the
  slowest runner from a current main run. Propose the levers: split the
  dominant tests into per-case tests that nextest can schedule, partition
  by measured duration (or add runners), or both. The Architect rules on
  the test splits.
- **AC-1.** Each split test keeps the same cases, assertions and outcomes;
  list old test to new tests. No test is deleted, ignored or weakened, and
  no mutation case is dropped.
- **AC-2.** The PR's own Full CI run, and the first main run after it
  lands, each finish under 20 minutes; post both timelines. A run within a
  minute of the limit is a stop to re-balance, not a pass.
- **AC-3.** Targeted local checks only, through `scripts/ken-cargo`; CI is
  the measurement.

## Stop conditions

- Any change to what a test asserts, or any kernel, `trusted_base()` or
  spec change.
- A split that needs production code changes.
- **Contention:** the runtime ring is on `RT-COMPMATCH-TREE-SCRUTINEE`. If
  it touches `rt_parity_native.rs`, coordinate through the Steward.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.
