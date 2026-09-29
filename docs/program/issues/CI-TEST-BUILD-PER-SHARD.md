---
id: CI-TEST-BUILD-PER-SHARD
title: "Cut the per-shard test-build phase of Full CI: each workspace test shard builds only the test binaries it runs (whole binaries assigned to shards, balanced by build plus run time), with a first-party dependency cache only if measured to help; measured first, stop if the predicted saving is under two minutes"
status: ready
owner: verify
size: M
gate: verify-qa
tier: T2
depends_on: [V3-Z3-STUB-EXEC-RACE]
blocks: []
github: null
origin: "Operator 2026-09-29: \"yes, that sounds like a good plan, L2 after the z3 fix.\" Residual of CI-UNDER-TWENTY-MINUTES (closed at 83b20f189): the test build takes 178-276 s per shard because every shard builds the full workspace inventory. Steward-filed per COORDINATION section 2."
---

# Each shard builds only what it runs

## Objective

Full CI wall time drops by at least two minutes. Every test identity still
runs exactly once, and the 20-job cap holds (operator 2026-09-27).

## Fixed inputs (read at `4902a2869`)

- **Walls.** PR run `36439436031` took 1,110 s and post-landing run
  `36442072417` took 1,107 s (CI-UNDER-TWENTY-MINUTES closeout).
- **The phase.** In `.github/workflows/ci.yml`, each of the seven
  workspace test shards runs `cargo nextest list --workspace` (`:182`),
  which builds every workspace test binary before the shard selects its
  share. That phase took 178-276 s per shard. The ten `rt_parity_native`
  shards and the px8f job also each run a workspace-wide `Build` step.
- **Population.** There are about 343 integration-test binaries: 240 in
  `ken-elaborator`, 51 in `ken-cli`, 26 in `ken-kernel`, 23 in
  `ken-interp`, and 3 in the other crates. Each is compiled and linked
  separately, and none depends on another. The eight workspace crates form
  a dependency chain that every shard needs.
- **No cache, by ruling.** `Swatinem/rust-cache` was removed (item C8): it
  never measurably helped, and it is third-party code with access to the
  build. The workflow comment at `:117-123` requires a first-party
  `actions/cache` and measured numbers for any return.
- **A shared compile cache cannot carry the binaries.** sccache does not
  cache linked outputs, and every test binary is linked.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

1. **Measure, in one CI run, before any change.** Record each shard's build
   time, split into three parts: third-party dependencies, the
   workspace-crate chain, and the test binaries (`cargo build --timings`).
   Also record each shard's peak disk use and peak memory. From that,
   predict the wall saving of (a) per-shard binary ownership, (b) a
   first-party dependency cache, and (c) both.
2. **If the best prediction saves at least two minutes of wall time,**
   build it:
   - the planner assigns whole test binaries to shards, balanced by
     measured build plus run time;
   - each shard builds only its own binaries;
   - add the dependency cache only if step 1 measured it to help.

3. **Resource recording, kept whatever step 2 decides** (operator
   2026-09-29). Every build and test job logs its peak disk and memory, and
   a failed job logs a `df -h` and `free -m` snapshot at failure. This
   diagnoses the runner SIGBUS on post-landing run `36495621224`
   (`native-slow (rt_parity_native) 2/10`, 09-28 23:04Z): three minutes of
   silence during `cargo build --workspace`, then several `rustc` processes
   and `cargo` died together with exit 135.

## Acceptance

- **AC-1 (population).** On the candidate's PR run and its first
  post-landing run, every live test identity runs exactly once. The
  existing shard-union check (`scripts/check-ci-shard-union.py`) passes
  unmodified in meaning.
- **AC-2 (wall).** Both runs are at least 120 s faster than the 1,110 s
  baseline. The 20-job first wave is unchanged.
- **AC-3 (control).** A binary removed from every shard's assignment turns
  the union check red.
- **AC-4 (recording).** Both runs show per-job peak disk and memory in the
  logs. A deliberately failed step on a scratch branch shows the failure
  snapshot.

## Scope

- Expected: `.github/workflows/ci.yml` and the shard planner and selection
  scripts it calls.
- Must not move: the test population, any test body, and the `--locked`
  workspace compile check (it may move to one job, but it must stay).

## Gate

Verify QA only. Workflow and CI paths are outside the Architect's domain
(COORDINATION §8a, §14a). If a change reaches `crates/`, the Architect gate
returns (Steward kickoff `evt_70t1502evc5w6`).

## Stop conditions

- Step 1 predicts under 120 s of saving. Stop with the measurements, land
  only the recording (step 3), and the Steward returns the rest to the
  operator.
- The chosen design needs a 21st job or a third-party action. Stop to the
  Steward.
