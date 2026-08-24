---
id: RT-ACTIVE-EXIT-RESIDUAL-PIN
title: "Mutation-proven route/arrival coverage for the narrowed non-process `ProcessExitStatus x Active` residual arm at core.rs:4292, which M6 (RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION) narrowed to non-process callers only. M6 correctly deleted the sole prior control for this route (the sar_d3 mutation-provenance test in positional_candidate_settlement.rs) because it pinned the RETIRED SAR measurement mechanism (the sar_d2_* counters); the retained residual arm is behavioral and under the green suite but no NEW test re-establishes route/arrival mutation coverage specifically for the narrowed arm. Add one, without reopening the retired measurement mechanism."
status: draft
owner: runtime
size: S
gate: none
depends_on: [RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION]
blocks: []
github: null
origin: "Runtime-leader request evt_20fgbmv6m408g on the Architect's M6 breadth-review item (C) (evt_198g61jc9sz08, non-blocking should-consider; M6 APPROVE unchanged, routed as-is). Steward-filed per COORDINATION section 2. Post-merge follow-up: released once RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION lands, since the arm's narrowing is defined by that change."
---

> # DRAFT — post-merge follow-up; released when M6 lands.
>
> Non-blocking coverage follow-up carved off M6 at the runtime-leader's request
> (evt_20fgbmv6m408g). M6 (`e9c9f8be`) was routed as-is; this node does NOT gate
> it. The residual arm this pins is defined by M6's narrowing, so this node
> depends on M6 and is released once M6 lands.

# Objective

Give the narrowed non-process `ProcessExitStatus x Active` residual arm
(`core.rs:4292`, commented "covers non-process callers only" after M6) its own
mutation-proven route/arrival coverage, so the arm's reachability and correct
arrival are pinned by a test that bites under mutation — not merely executed
incidentally by the existing green suite.

# Background

M6 retired the `RT-SPECIALIZED-ACTIVE-RESUME` D3 measurement mechanism: the
`sar_d2_*` counters were removed, and the 118-line `sar_d3_*` mutation-provenance
control in `positional_candidate_settlement.rs` was deleted with them (it called
the now-gone `sar_d2_*` counters, i.e. it pinned the retired measurement, not the
behavior). That deletion was correct and Architect-approved. The behavioral
`ProcessExitStatus x Active` residual arm at `core.rs:4292` remains live and is
narrowed to non-process callers only (the checked process pop now reaches the
seam as `Carried`, `core.rs:4237`, covered there). The gap: no NEW test
re-establishes route/arrival mutation coverage for the narrowed arm.

# Deliverable

One runtime test that:
- routes a non-process caller through the narrowed `ProcessExitStatus x Active`
  residual arm at `core.rs:4292` and asserts correct arrival/behavior;
- is proven by a natural, compile-preserving mutation AT THE SITE that reds the
  test (route mis-directed or arrival dropped), restored byte-identically;
- does NOT reintroduce the retired SAR measurement mechanism — no `sar_d2_*` /
  `sar_d3_*` counters, no measurement-provenance instrumentation; it pins the
  BEHAVIORAL arm, not a counter.

# Acceptance criteria

- AC-1 (route/arrival pin). A test exercises the narrowed non-process arm at
  `core.rs:4292` and asserts the arm is reached and arrives correctly for a
  non-process caller. Targeted `-p` locally; whole-suite in CI.
- AC-2 (mutation bites). A compile-preserving mutation at the arm's natural site
  (mis-route the arm, or drop its arrival) reds AC-1's test; the candidate is
  green on the unmutated site; the mutation is restored byte-identically before
  commit.
- AC-3 (no retired mechanism). `git grep sar_d2` and `git grep sar_d3` at the
  candidate SHA return nothing; the test introduces no measurement counter and no
  measurement-provenance control. The pin is behavioral.
- AC-4 (cross-cutting invariant). Zero `trusted_base()` delta; no kernel/prelude
  path touched. Whole-suite green in CI; local targeted `-p` only.

# Reviewers

Runtime QA (route/arrival discriminator power; mutation bites; no retired
mechanism) + Architect only if the arm's contract needs a soundness read
(runtime-leader's call); the Steward-owned Adversary gate at handback per §10a.

# Capability tier

T2 — mechanical test authoring pinning an existing behavioral arm with a biting
mutation control; the care is in targeting the exact narrowed arm and a mutation
that discriminates route/arrival, not novel design. Size S (one test + its
mutation control).
