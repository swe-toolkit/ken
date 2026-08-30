---
id: CI-SHARD-DURATION-BALANCE
title: "Partition the test shards by measured DURATION rather than by test count, and choose the shard count from the same measurement — count partitioning produced a 3.45x spread across 8 shards, so one shard ran 18m43s while another ran 5m25s and the run paid the maximum, not the mean."
status: merged
owner: verify
size: S
gate: none
tier: T2
depends_on: [CI-NATIVE-PARITY-DURATION]
blocks: []
github: https://github.com/swe-toolkit/ken/pull/3158
origin: "Steward, 2026-08-29, from the first full-matrix run after CI-NATIVE-PARITY-DURATION D1/D2 landed (c555f843a). Measured on GitHub Actions run 33230600665, PR #3079, the CAT-DERIVED-PUB-EXPORT merge candidate. Serves the operator's standing priority-1 directive on CI duration (ideal under 10 minutes, under 20 acceptable). Steward-filed per COORDINATION section 2."
---

> # TWICE CI-RED ON THE EMPTY-SHARD CLASS — new AC added, 2026-08-30. `active`.
> #
> # The candidate reddened in CI on the SAME empty-shard predicate twice: PR
> # #3120 shard 2 (Decision `dec_49ga0n9q9ax99` spent), then respin PR #3133 /
> # `f3685b50f` shard 1 (Decision `dec_5sdq3ev4vfxc0` spent, CI run
> # `33286648107`). Both times a shard with zero tests makes `nextest` exit "no
> # non-empty testcases", the job never writes `realized-shard-<k>/*`, and the
> # union's `if-no-files-found: error` fires. Both are candidate-owned and
> # deterministic (main green at base), not flakes — the lieutenant correctly did
> # not rerun, run M8, or merge.
> #
> # The frame gap: no AC required the empty-shard CI fan-in to be handled
> # end-to-end, so both local gates (QA + Architect) approved the selector's
> # empty-partition handling and CI reddened anyway — the real fan-in runs only in
> # CI. `AC-EMPTY-SHARD-FAN-IN` now pins it (see Acceptance criteria). Same
> # predicate twice is a frame gap, NOT a recut: fix at a NEW SHA with fresh QA +
> # Architect + a new Decision. Do not reuse #3120/#3133 or a spent Decision.

> # SUPERSEDED (historical) — filed as a draft successor; released since.
> #
> # `draft`, filed so lane 2 has a successor when `CI-NATIVE-PARITY-DURATION`
> # closes rather than going idle at D5. **A landing discharges a dependency; only
> # an explicit Steward release starts a turn.** Flip `draft` -> `ready` ->
> # `active` on release, because a dispatched node left at `ready` is invisible to
> # the per-node watchdog sweep.

## The defect

`.github/workflows/ci.yml:32` states the partitioning choice as a virtue:

> `--partition count:N/8` ignores crate boundaries and balances by count.

**Balancing by count is the defect.** Test cases do not have uniform duration,
so an equal-count partition is only an equal-duration partition by accident. A
sharded run's wall clock is the MAXIMUM shard, never the mean, so all of the
imbalance lands on the critical path and none of the surplus capacity on the
fast shards can be spent.

## Fixed inputs, measured at run `33230600665` (PR #3079)

Run created `03:07:12Z`, final job completed `03:38:34Z` — **31m22s total.**

| shard | duration | | shard | duration |
|---|---|---|---|---|
| 1/8 | **18m43s** | | 5/8 | 6m20s |
| 2/8 | 10m33s | | 6/8 | 6m46s |
| 3/8 | 8m17s | | 7/8 | 6m28s |
| 4/8 | 10m41s | | 8/8 | 5m25s |

Sum 73m13s, mean 9m09s. **Slowest is 2.05x the mean and 3.45x the fastest.**
Shard 1 alone carries 9m34s of excess over the mean.

**The critical path decomposes cleanly, and it is two separate problems:**

| term | cost |
|---|---|
| shard 1/8 waiting for a runner (started `+12m34s`) | 12m34s |
| shard 1/8 executing | 18m43s |
| final aggregation job | ~5s |
| **total** | **31m22s** |

First job started `+64s`, last started `+13m26s` — a **12m22s stagger** across 21
jobs contending for runners.

⇒ **Perfect duration balance alone takes the run to about 22m.** Removing the
queue stagger as well would take it under 10m. **This node owns the first term
only.** The second is runner concurrency, which costs money and is the
operator's call — see "What this node does NOT do".

## Deliverables

- **D0 — measure per-test duration and attribute it.** `cargo nextest` already
  reports per-test timing (`ci.yml:86` says so). Produce the per-test duration
  table for the sharded lane at a named SHA, and show which tests landed in
  shard 1/8 under `count:1/8`. **Name the specific tests responsible for the
  18m43s**; a shard is slow because of its members, and the repair cannot be
  judged without knowing them.
- **D1 — choose the shard count N and the assignment TOGETHER, from D0.**
  **Do not presume N=8.** The objective is to minimise `queue_position(N) +
  shard_duration(N)`, not `shard_duration` alone: every added shard is another
  job contending for a runner, so a duration-balanced 5 may beat a
  duration-balanced 8. Report the tradeoff you measured, then implement the
  winner.
- **D2 — report the residual.** After D1, restate the critical-path
  decomposition above at the new N. This is the operator-facing number that says
  what a runner-concurrency decision would still buy.

## Acceptance criteria, each with its control

- **AC-SPREAD-MEASURED.** The post-change slowest-to-mean ratio is stated as a
  measurement on a real full-matrix run, not predicted from the D0 table.
  Control: cite the run id and the per-shard durations. **A predicted balance is
  not a balance** — test duration varies with runner and cache state.
- **AC-NO-TEST-LOST.** The union of all shards is exactly the set the current
  partition covers, with the three excluded native binaries still excluded and
  still running in their own jobs. Control: compare the full test-id union
  before and after; **the counts must be equal AND the sets identical.**
  `ci.yml:99-101` states why this matters — the exclusion here and the
  inclusions there must stay complementary, "or a test is silently duplicated or
  silently dropped, and a dropped test still shows a green gate."
- **AC-DETERMINISTIC.** Two runs at the same SHA assign the same tests to the
  same shards. Control: run twice, diff the assignments. **A partition that
  drifts run-to-run makes a shard failure unreproducible.**
- **AC-DEGRADES-SAFELY.** A test present in the tree but absent from the
  duration table (newly added since the measurement) is still executed, in some
  shard, without a manual edit. Control: add a test not in the table, observe it
  run. **This is the arm that decides whether the mechanism rots** — a
  hand-maintained assignment list would fail it, which is why the assignment
  must be computed rather than checked in as a literal roster.
- **AC-EMPTY-SHARD-FAN-IN.** Every shard job 1..N produces its
  `realized-shard-<k>` artifact so the fan-in union step succeeds — INCLUDING any
  shard the partition leaves empty. The failure mode, red TWICE (PR #3120 shard
  2, PR #3133 shard 1, both CI run-confirmed): a partition assigns zero tests to
  a shard, `nextest` exits with "no non-empty testcases", the job exits before
  writing `realized-shard-<k>/*`, and the union's `if-no-files-found: error`
  fires. The mechanism must EITHER guarantee no shard is empty (choose N and the
  assignment so every shard is non-empty) OR make an empty shard emit a valid
  empty `realized-shard-<k>` artifact the union tolerates. Control/proof: a GREEN
  fan-in in CI with all N shards realized. **This AC is provable ONLY in CI** — a
  local selector-level empty-partition test does NOT satisfy it (both local gates
  approved exactly that and CI still reddened, twice). Do not rerun a red SHA; fix
  at a new SHA.
- **AC-NO-REGRESSION.** Whole-suite green in CI.

## What this node does NOT do

**It does not add runners, and it must not be recut to.** The 12m22s queue
stagger is real and is the larger of the two terms, but buying concurrency
spends money and is the operator's decision, not the Steward's and not the
ring's. D2 exists to hand the operator a current number for that decision.
**Do not treat the stagger as a defect to engineer around** — reducing N is in
scope as a balance tradeoff, but adding runners, self-hosting, or restructuring
the matrix to dodge scheduling is not.

## Capability

**T2, size S.** The measurement is mechanical and the data source already
exists. The one piece of judgment is the N-versus-balance tradeoff in D1, and it
is decided by the numbers D0 produces rather than by argument.

## Contention check

Touches `.github/workflows/ci.yml` and whatever partitioning helper D1 adds.
[[CI-NATIVE-PARITY-DURATION]] is `active` over the same file and **must land
first** — that is why it is `depends_on` and not a parallel node.
