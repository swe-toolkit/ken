---
id: CI-GATE-TIME-REDUCTION
title: "Temporary operator-directed detour (2026-09-03): the PR-gate CI wall-clock regressed from ~19m to ~60m when RT-COMPOSED-RETURN-SSA-SPECIALIZATION landed (ad9905a7e). Attribute the 3x regression across the gate's jobs and reduce the PR-gate wall-clock back to under 20m WITHOUT reducing gate coverage or soundness (no gate removed, no suite dropped, no --locked/conformance weakening). Reduce wall-clock by parallelism, sharding, caching, and per-test cost, not by gating less."
status: merged
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
status_note: "D1 COMPLETE (foundation, 2026-09-03, evt_27h2v7c4ny07f + evt_4b0zpkf3gyg6t): regression is 100% RT-SSA's rt_parity_native.rs test additions, not workflow shape. D2 COMPLETE (runtime, 2026-09-03, landed 4b9408b25 / PR #3263): rt_parity_native.rs decomposed 37-arm loop into per-arm tests, native-rt-parity resharded 3->6, union-pin added. Measured on the candidate SHA: native-rt-parity 6-shard max 17m19s, overall PR-gate wall-clock 17m41s, both under the AC-UNDER-20M target; all 26 checks pass. Coverage-preserving (no gate/suite/--locked/conformance change). Detour complete; foundation Tier-B and runtime D3-RECUT proceed unpaused."
origin: "Operator directive 2026-09-03 (Steward session): 'RT-COMPOSED-RETURN-SPECIALIZATION merged and bumped CI time from ~19m to almost 60m. At the next convenient seam of any team, temporarily redirect them to reducing the CI time to less than 20m.' Steward first considered verify (natural CI-tooling owner) but its ring is NOT seated this session (no moot-verify-* tmux session; convo participant records are stale). Reassigned to foundation: lane 3 (lowest priority) and between deliverables (Tier-A landed, Tier-B not yet cut) = least-disruptive convenient seam, keeping the priority runtime lane free for the L1 D3-RECUT closeout. Temporary detour: foundation's Tier-B (scaffold-retirement) pauses and resumes on completion."
---

> # Temporary operator-directed detour. Faster gate, SAME gate. D1 was
> # foundation's (complete); D2 is runtime-owned (it edits ken-cli's parity tests).
>
> The PR-gate CI now costs ~60m per merge — a fleet-wide tax on every lane's
> throughput (three lanes each pay it). The operator directed a temporary redirect
> of the next available team to bring it back under 20m. This is a WALL-CLOCK
> reduction, not a COVERAGE reduction: the full-workspace build, the `--locked`
> gate, and the conformance suite must all still run and still gate. Getting under
> 20m by gating less is the one disallowed outcome.

## Objective (operator, 2026-09-03)

Return the PR-gate CI wall-clock to **< 20m** (from the ~60m post-RT-SSA
regression, baseline ~19m pre-RT-SSA). Coverage-preserving: no gate deleted, no
suite skipped, no soundness/`--locked`/conformance weakening. Speed comes from
HOW the same checks run (parallelism, sharding, caching, per-test cost), never
from WHICH checks run.

## The regression, and the guardrail

- **Named cause:** RT-COMPOSED-RETURN-SSA-SPECIALIZATION merged at `ad9905a7e`
  (PR #3250) tripled the gate wall-clock ~19m -> ~60m. The regression's shape is
  unmeasured — attribute it first (D1), do not assume it is test time vs build
  time.
- **Guardrail (hard):** any change to WHAT the gate covers — dropping a suite,
  narrowing the `--workspace` build, relaxing `--locked`, skipping conformance,
  or making a check non-blocking — is OUT OF SCOPE and escalates to Steward
  (then operator). It is not a lever available to this WP. A green under-20m gate
  that no longer runs a check it ran at 60m is a FAILED deliverable, not a met
  one.

## D1 — COMPLETE (foundation, 2026-09-03)

Read-only profiling (evidence `evt_27h2v7c4ny07f` full job/time partition +
`evt_4b0zpkf3gyg6t` attribution; raw under the foundation seat's
`/tmp/ci-gate-time-d1/`). Comparator pre-RT-SSA run = PR #3249 (`33687061800`,
wall 19:46); RT-SSA PR #3250 (`33694424301`, 55:49); current PR #3255
(`33704847269`, 54:40).

FINDING: the regression is entirely **nextest execution inside the
`native-slow (rt_parity_native)` job**, NOT compile/build (critical-path compile
~2:13 both pre and post, unchanged), NOT conformance (zero-second placeholder),
NOT queue/poll overhead (dominant shard starts at run+0:14). Critical-shard
nextest grew 17:01 -> 52:00 (+34:58). Cause = the 8 tests RT-SSA added to
`crates/ken-cli/tests/rt_parity_native.rs` (+1272/-65 from base `e485a696c`; net
+7, one row retired). The three new shard-1 tests are 92% of the 3120s nextest
step. The single dominant test
`static_response_full_demand_population_controls_reach_red_and_restore` runs **42
serial full native builds in one indivisible `#[test]` = 33:57**, already 14m
over the whole 20m target; `static_response_owner_body_...` = 24 builds,
`checked_ih_direct_application_population_...` = 11 builds, same monolithic shape.
`nextest --partition count:N/3` can move a test between shards but cannot
subdivide one test. Workflow-only caching saves at most ~2:13; isolating the
33:57 test to its own job still leaves a hard floor above 20m. => Not fixable by
a workflow-only, Foundation-owned change. HARD STOP, correctly routed to runtime.

## D2 — RUNTIME-OWNED (the substantive remaining work)

Decompose the three monolithic RT-SSA mutation grids in
`crates/ken-cli/tests/rt_parity_native.rs` into independently-named,
independently-schedulable test cases — **one mutation plus its exact
baseline/restoration obligation per `#[test]`** — so nextest can parallelize
them across threads and shards; then apply **duration-aware** sharding (or more
rt-parity jobs) sized so the native builds do not OOM a runner. This is a
**behavior-preserving** decomposition: same builds, same mutation arms, same
per-arm exact diagnostic/reach observations, same read/write populations, same
restoration hash/byte comparisons — only the `#[test]` granularity changes. Do
NOT reduce the number of native builds, merge arms, or drop an observation; if a
real speedup requires changing what is observed (foundation's candidate 2:
reusing immutable baseline artifacts / planner-only assertions), that is a
separate design fork and a HARD STOP to Steward (pulls in the Architect, as the
rt-parity corpus is the RT-SSA differential oracle).

Also **strengthen the realized-union pin** from a test-name roster to
test-name-plus-required-control-arm coverage, so a decomposition that silently
drops an arm REDS the pin. Land the change; re-measure the PR-gate on a
representative PR and show < 20m with every prior check still present and
blocking.

## Acceptance criteria

- **AC-UNDER-20M.** A representative PR's gate completes in < 20m, measured on the
  actual GitHub-Actions PR-gate (not a local run), shown against the ~60m
  baseline.
- **AC-COVERAGE-PRESERVED.** Enumerate the gate's checks before and after; the
  after-set is a superset-or-equal of the before-set — the full `--workspace`
  build, `--locked`, every test suite, and conformance all still run and still
  block. A diff of the workflow's job/step list evidences nothing was dropped or
  made non-blocking. This is the load-bearing AC.
- **AC-ATTRIBUTION-CITED.** The D2 change targets the job D1 identified as the
  regression's dominant cost, with the before/after time for that job shown — not
  a blind speedup that happens to help.
- **AC-NO-FALSE-GREEN.** The faster gate still REDS on a genuine regression:
  demonstrate the reshaped gate fails on an injected build break and on an
  injected test failure in the sharded suite (a shard that silently runs nothing
  is a coverage hole, not a speedup).
- **AC-NO-KEN-TCB-TOUCH.** No change to `crates/ken-kernel` or elaborator
  soundness surface; this is CI-config/test-harness work. If a fix requires a
  lane crate's test change, that change routes through the owning lane's QA.

## Gate, reviewer, sequencing

- **gate: none** (no Ken TCB touch — this is `ken-cli` test-harness structure +
  a CI-shard change, not kernel/elaborator soundness). Reviewer: **runtime-qa**
  on the mechanics (re-measure < 20m; per-arm/observation/restoration preserved;
  the strengthened realized-union pin reds on a dropped arm) + **Steward** on
  AC-COVERAGE-PRESERVED (the coverage-superset check is mine before merge).
  **Architect** enters only if the decomposition changes what is observed
  (foundation's candidate 2) — a design fork, hard-stop to Steward first.
- **Capability tier T1.** The decomposition is behavior-preserving but the
  coverage bookkeeping (every arm/observation/restoration retained across the
  split, the pin strengthened so a drop reds) and the OOM-safe duration-aware
  sharding design are reasoning-careful. Runtime is Sonnet 5 — matched.
- **Temporary detour, sequenced before D3-RECUT.** This is the operator's
  explicit CI-reduction priority and it is runtime-owned; runtime does it first,
  then returns to the lane-1 D3-RECUT closeout. Landing it makes every later code
  merge (D3-RECUT included) pay ~20m instead of ~55m, so CI-first is also the
  faster path to the L1 closeout.
- **This is a CODE candidate**, not doc-only: it runs the full PR-gate (itself
  the ~55m gate being fixed — the last slow run before the fix lands). Standard
  path: runtime builds the branch, runtime-qa reviews the exact SHA, Steward
  M1-M4, lieutenant M5-M9.
- **Sequencing vs the other lanes.** D1 was foundation's (done); foundation now
  resumes Tier-B (lane 3 un-paused). Language and the runtime D3-RECUT proceed on
  their own gates. This node accelerates all of them.
