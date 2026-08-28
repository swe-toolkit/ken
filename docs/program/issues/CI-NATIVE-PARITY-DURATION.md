---
id: CI-NATIVE-PARITY-DURATION
title: "Rework the CI test suite to run faster, under the operator's 20-minute ceiling and toward the 10-minute target, by removing three measured serial floors: split checked_ih_generated_entry_confluence_and_route_mutations_reject (39 sequential subprocess mutations, 1299.983s in ONE scheduling unit) and its five siblings into per-case tests so nextest can schedule them; then partition native-rt-parity across runners; then rebalance the workspace shards. Splitting alone is necessary but NOT sufficient -- it takes the job from 25m to about 18m, because 4171.65 CPU-seconds on a 4-vCPU runner floors at 17.4m -- and partitioning is INERT until the split lands, because --partition cannot subdivide a single 1300s test. Separately, the ignored-row sweep EXECUTES 33 ignored rows for 10m and treats their failures as non-blocking findings, so rows blocked on a named unbuilt capability get a registered short-circuit that the sweep enforces rather than silently passes. Behaviour-preserving throughout: the same 90 mutation cases with the same assertions and outcomes."
status: active
owner: verify
size: M
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "Operator request 2026-08-28: 'diagnose the native-slow test and brief me on what we can do about it? ideally CI should be less than 10 minutes, but less than 20 is acceptable.' Then the operator's directing message the same day: 'split up checked_ih_generated_entry_confluence_and_route_mutations_reject so that it can be parallelized and/or sharded into separate jobs. Are ignored tests run, but not considered failures if they do fail? If so, add a short circuit quick end to the tests with a comment so that they are properly re-enabled when they are rearmed. let lane 2 finish its current wp, then bring up verify on lane 2 to rework CI tests to make them run faster.' That last clause is a ROSTER RULING and is recorded in steward/lanes.md. Steward diagnosis measured against completed main run 33192361977 at bb33dfb71e302a68377ffde8038f7dc8bd2c82ac -- the first fully completed main run since 31258f403. Steward-filed per COORDINATION section 2."
---

## Lane and release condition

**This node is lane 2, owned by verify.** The operator ruled: *"let lane 2
finish its current wp, then bring up verify on lane 2 to rework CI tests to make
them run faster."*

**THE RELEASE CONDITION IS MET (2026-08-28).**
`V3-FO-SORTED-EIGENPARAMETER-DERIVATION` LANDED at `114a6c105` — all eleven
candidate paths blob-verified against `origin/main` by the Steward, CI run
`33207199378` green, tracker closeout at `5fe12514b`. Lane 2's ring therefore
changes from language to **verify**, and this node is flipped `active` and
RELEASED.

**This is a ring change WITHIN lane 2, not a fourth lane.** The three-lane
roster — runtime / language(->verify) / foundation — is unchanged, and the z3
integration campaign queues behind this node.

Landing alone still releases nothing; the Steward's explicit release does, and
it accompanies this flip.

> **CONTENTION RE-MEASURED AT RELEASE, not trusted from this frame's earlier
> reading.** This node edits `crates/ken-cli/tests/rt_parity_native.rs`. Lane
> 1's D3 node (`RT-RESULT-CONTINUATION-BINDING-PROVENANCE`) adds observations to
> the SAME file — but D3 is FROZEN at HS13, and its only authorized next turn is
> a D0 measurement that lands NO production. **There is no live writer on that
> file today.** If D0 returns YES and a later D3 release issues while this node
> is still in flight, re-measure and hard-stop rather than merging across the
> two.

## Model-capability estimate (steward.md §4h): T2 — mechanical

The diagnosis is settled below and the arithmetic is measured, not argued. What
remains is a behaviour-preserving restructure of a test file, two workflow matrix
edits, and a registry-driven short-circuit. The review turns on differential
faithfulness — same cases, same assertions, same outcomes — not on an argument.

## Fixed inputs — measured at `bb33dfb71`, run `33192361977`

Do not re-derive these; cite them. Every number is from the completed main run's
own logs.

Run wall-clock `16:56:49Z` to `17:25:44Z` = **28m55s**. Critical path:

| job | Build | Test |
|---|---|---|
| `native-slow (rt_parity_native)` | 1m | **24m** |
| `test shard 1/4` | 1m | **18m** |
| `test shard 4/4` | — | 12m |
| `test shard 2/4` | — | 12m |
| `test shard 3/4` | — | 11m |
| `ignored-row sweep` (non-blocking findings) | — | 10m |
| everything else | — | 6m or less |

**The Build step is 1m. The whole problem is the Test step.** A faster runner or
a better dependency cache buys nothing here.

`rt_parity_native` nextest summary: `Summary [1435.857s] 15 tests run: 15 passed
(12 slow), 6 skipped`. Per-test, the top of the distribution:

| seconds | test |
|---|---|
| **1299.983** | `checked_ih_generated_entry_confluence_and_route_mutations_reject` |
| 656.759 | `checked_ih_continuation_inheritance_mutations_bite_their_own_arms` |
| 623.268 | `checked_ih_generated_entry_capsule_mutations_reject` |
| 339.805 | `checked_ih_generated_entry_admission_population_mutations_reject` |
| 270.219 | `d1_route_control_full_program_mutations_are_fail_closed` |
| 258.421 | `checked_ih_generated_entry_per_arrival_operation_mutations_break_equality` |
| 136.884 and below | the remaining nine tests |

Total CPU across all 15: **4171.65s**. Wall: **1435.86s**. Effective
parallelism **2.9x** on a 4-vCPU `ubuntu-latest` runner.

## The diagnosis: three independent floors

**Floor 1 — the longest single test, which is the one the operator named.**
`nextest` schedules at `#[test]` granularity. The six slowest tests are each a
`for` loop over a case array that spawns one isolated child process per case via
`std::env::current_exe()`:

| line | test | cases |
|---|---|---|
| 1370 | `..._per_arrival_operation_mutations_break_equality` | 6 |
| 1445 | `..._admission_population_mutations_reject` | 8 |
| **1588** | **`checked_ih_generated_entry_confluence_and_route_mutations_reject`** | **39** |
| 1748 | `..._capsule_mutations_reject` | 15 |
| 1990 | `..._continuation_inheritance_mutations_bite_their_own_arms` | 15 |
| 2151 | `d1_route_control_full_program_mutations_are_fail_closed` | 7 |

**90 subprocess children, and each loop is ONE scheduling unit on ONE core.**
The named 39-case loop costs 1299.983s — 33.3s per child — and no other test can
use the cores it leaves idle. **Wall time cannot go below 1300s while that test
exists**, whatever else is done.

**Floor 2 — total CPU against runner width, and this is what makes splitting
insufficient on its own.** 4171.65s of work on 4 vCPUs floors at **1042.9s =
17.4m**, plus the 1m Build. So:

- Split alone: 25m job → **about 18m**. Under the 20m ceiling, not near 10m.
- **`--partition` is what reaches the target — and it is INERT before the
  split.** Partitioning 15 tests cannot subdivide a 1300s test; that test lands
  whole on one shard and re-floors the job at ~22m. **After** the split the
  population is ~99 tests of roughly even cost, and 3 partitions give
  ~1043/3 ≈ 350s ≈ 6m + 1m Build = **about 7m**.

**⇒ D1 is the enabler and D2 is the payoff. Neither delivers the target alone,
and D2 measured before D1 reads as no improvement.** Do not conclude from a flat
D2 measurement that partitioning does not work.

> ## PACKAGING: D1 + D2 (+ D3 if clean) LAND AS **ONE** CANDIDATE
>
> **Steward sequencing ruling, 2026-08-28, after the operator made this node the
> fleet's first priority.** The split-before-partition constraint above is a
> **MEASUREMENT** ordering and it is unchanged: perform the split, measure it,
> then apply the partition and measure again, and report BOTH numbers.
>
> **It is not a packaging instruction, and reading it as one is expensive.**
> Every publish cycle costs a full CI run — about 29 minutes today — so shipping
> D1 and D2 as separate candidates spends roughly an hour of pure latency to
> honour an ordering that is already satisfied inside a single turn. The
> measurement discipline lives in the turn; the packaging does not have to.
>
> **D3 belongs in the same candidate when it is clean, and it is not optional
> polish.** `native-slow` is 24m and `test shard 1/4` is 18m, so **D1 alone
> lands the run at roughly 19m — inside the operator's 20m ceiling by about one
> minute, with no margin**, because shard 1/4 simply becomes the new critical
> path. D1+D2+D3 is what reaches the 10m target.
>
> **This does NOT relax the behaviour-preserving requirement, which is the whole
> review.** Same 90 mutation cases, same assertions, same outcomes. A larger
> candidate makes that review bigger, not weaker. **A split that turns out not
> to be behaviour-preserving is a HARD STOP to the Steward, never a scope
> widening**, and a candidate that cannot show differential faithfulness does
> not land regardless of what it does to the clock.

**Floor 3 — shard imbalance, independent of the above.** `test shard 1/4` is 18m
against 11m for shard 3/4. `--partition count:N/4` assigns by test, not by
duration, so the split is uneven by construction. The aggregate `build + test`
gate reads `needs.test-shard.result` for the whole matrix, so the shard count can
change without touching branch protection — the stated design intent of the
comment at `.github/workflows/ci.yml:383`.

## The ignored-row question, answered

**The operator asked whether ignored tests run but are not treated as failures.
They do, and they are not.** Measured, not inferred:

- `.github/workflows/ci.yml:157` runs `cargo nextest run --workspace --locked
  --run-ignored=only --no-fail-fast`, so ignored rows **are executed**.
- It is wrapped in `set +e` and its exit status is passed to
  `scripts/ci-ignored-sweep.py report`, which routes findings and exits zero.
  The job name says so: `ignored-row sweep (findings non-blocking)`.
- On run `33192361977` that sweep ran **33 rows**, most of which FAILED, at
  roughly 40-75s each. That is the 10m job.

So the operator's premise holds and the remedy applies. **One consequence has to
be stated before it is built:** a bare short-circuit makes the sweep report a
row as passing, and the sweep's whole purpose is to notice when an ignored row
*starts* passing so it can be re-armed. Short-circuiting rows without telling the
sweep would convert a live instrument into one that always reads green — the
failure mode this program keeps paying for.

**So the short-circuit is registered, not silent.** The mechanism already
exists: `.github/ignored-test-exemptions.toml` carries a `class` and a
`readmission` field per row, and `ci-ignored-sweep.py verify-row-claims` already
enforces the registry against the tree. That `readmission` string is exactly the
"properly re-enabled when they are rearmed" hook the operator asked for.

## Deliverables

**D1 — split the mutation loops into per-case tests.**
`checked_ih_generated_entry_confluence_and_route_mutations_reject` is REQUIRED
and is the operator's named target; its five siblings are the same defect and
are in scope. Each of the 90 cases becomes its own `#[test]`, preserving the
existing parent/child subprocess isolation unchanged. The child-side dispatch
(`assert_*_child`, the `KEN_RT_*_CHILD` environment variables, the `--exact`
re-invocation) is NOT redesigned; only the parent side stops looping.

**D2 — partition `native-rt-parity` across runners.** Add a shard matrix to the
job at `.github/workflows/ci.yml:325` on the same `--partition count:N/M` form
the workspace lane already uses, with `fail-fast: false` for the reason the
existing matrix comment gives. Pick N from the post-D1 measurement, not in
advance.

**D3 — rebalance the workspace shard count.** Raise the `shard:` matrix at
`.github/workflows/ci.yml:46` so no shard's Test step exceeds the target. Keep
the `Doctests` step conditional on a single shard.

**D4 — registered short-circuit for permanently-blocked ignored rows.** For each
ignored row whose readmission condition is a NAMED unbuilt capability, return
early at the top of the test body with a comment naming that condition, and
carry a registry entry whose `readmission` field names the same condition. The
sweep must **enforce** the pairing — a short-circuited row without a registry
entry, or a registry entry whose named condition has since been built, is a
sweep FAILURE, not a pass. Rows that are plausibly close to passing keep running:
those are the ones the sweep exists to catch.

**PACKAGING (superseding the earlier "D1 may land alone" split, and stated here
because this is where the packaging instruction actually lives): D1 and D2 are
ONE candidate, and D3 joins them when it is clean.** See the packaging ruling
under "The diagnosis" for the reasoning — each publish costs a full CI run, and
D1 alone leaves only a one-minute margin under the 20m ceiling.

**D2 must still never be MEASURED before D1** — that constraint is about
measurement order inside the turn and is untouched by the packaging change.

**D4 remains independently packageable** and may land in either order relative
to D1-D3; it is the ignored-row sweep work and shares no file with them.

## Acceptance criteria

- **`AC-CASE-FAITHFUL`.** Every one of the 90 cases survives with its mode
  string, its expected outcome, and its assertions byte-faithful to the loop body
  it came from. **No case is dropped, merged, renamed in a way that changes what
  it selects, or given a weaker assertion.** A case count of 90 before and after
  is necessary and NOT sufficient — the pairing must be exhibited. An increment
  that speeds CI up and loses a mutation case has made the suite worse, not
  faster.
- **`AC-CHILD-MECHANISM-UNCHANGED`.** The `assert_*_child` functions and the
  `KEN_RT_*_CHILD` environment protocol are preserved. The child half of each
  test is not restructured.
- **`AC-NO-NEW-SKIPS`.** The `6 tests skipped` population in `rt_parity_native`
  is unchanged by D1-D3. A test that stops running also stops failing, so a
  faster job with more skips is a regression measured as an improvement.
- **`AC-SHORTCIRCUIT-ENFORCED`.** For D4: prove BY MUTATION that the sweep fails
  on (a) a short-circuited row with no registry entry, and (b) a registry entry
  whose named readmission condition is satisfied. **A short-circuit the sweep
  cannot catch is not a cheaper instrument; it is no instrument** — and a control
  that cannot fail is the defect this program has paid for repeatedly.
- **`AC-DURATION-MEASURED`.** Report the post-increment `native-slow
  (rt_parity_native)` Test-step duration, the `ignored-row sweep` duration, and
  the run wall-clock **from a completed CI run**, against the 24m / 10m / 28m55s
  baselines above. A local timing is not evidence — the target is a property of
  the CI runner. **Report the number you get, including if it misses the
  target**; the arithmetic predicts about 18m after D1 alone, and that prediction
  being met is the deliverable, not a shortfall.
- **`AC-AFFECTED-CLOSURE`.** Cover every target that loads any module whose
  CLOSURE this increment changes, diff-touched or not. This is not a relaxation
  of the targeted-build rule: what changes is which targets count as affected,
  never how many crates build at once. This criterion has now cost three lanes a
  red merge.

## Banned scope

- **Do not delete, `#[ignore]`, or conditionally skip any mutation case** to hit
  a duration number. The cases are the suite's discriminating power.
- **Do not weaken an assertion** because a split test makes it awkward to reach.
- **Do not restructure the child-side dispatch**, and do not replace the
  subprocess isolation with in-process execution. That isolation is why these
  mutations are observable at all.
- **Do not short-circuit an ignored row whose readmission condition is
  unnamed**, and do not short-circuit one merely because it currently fails.
- **Do not touch `concurrency:` / `cancel-in-progress`** at
  `.github/workflows/ci.yml:17-19`. Separate operator decision (below), not in
  scope.
- **Do not change what the sharded lane excludes** at `ci.yml:123`. The three
  native binaries stay in their own jobs.

## Contention — real, and the reason the release condition matters

`crates/ken-cli/tests/rt_parity_native.rs` is the file the FROZEN
`RT-RESULT-CONTINUATION-BINDING-PROVENANCE` chain will eventually add
observations to, and `RT-FRESH-RESULT-ROUTE-PAIRING-LEG-CONTROLS` (`draft`)
cites `rt_parity_native.rs:1149` directly.

At filing time lane 1 was stopped at HARD STOP 12 with the Architect holding for
a Research advisory, and `runtime-implementer` reported the branch free and the
tree unchanged at `bb33dfb71` with no commit, candidate, or QA — so the file was
uncontended then. That window was not guaranteed to survive to release.

**MEASURED AT RELEASE (2026-08-28): still uncontended, and the stop count has
moved on — lane 1 is now at HARD STOP 13** (`evt_59t7b49m41z8m`), which froze
D3 to a D0-only return-boundary measurement that lands NO production. The
release-condition block at the top of this node carries the live statement; the
paragraph above is filing-time history.

**Re-measure the contention at release time, not from this paragraph.** If the D3
chain has resumed and is editing this file, hard-stop to the Steward rather than
resolving a merge across the two; the sequencing call is the Steward's.

## Reviewers

**Verify QA AND the Architect, both on the exact implementation candidate SHA.**

**The Architect IS a required reviewer.** An earlier version of this section said
otherwise on the grounds that "this is not the M-series" — that is not the gate
predicate, and the claim conflicted with federation law: **a merge Decision
requires the Architect always.** The `docs/program/` editorial exception covers
this Steward-owned FRAME route; it does not cover the implementation candidate,
which touches `crates/` and `.github/workflows/`. Corrected at the Architect's
block `evt_60hd0s0sn3kxw`; the defect was the Steward's.

The review turns on **differential faithfulness** — the same 90 mutation cases,
the same assertions, the same outcomes — and on the workflow changes not
weakening what `main` is gated on.

A finding that a mutation case cannot be split without changing what it observes
is a **HARD STOP to the Steward**, never something to resolve by weakening the
case. Larger packaging does not relax this: one candidate makes the differential
review bigger, not looser.

## Out of scope, and recorded here anyway

**`cancel-in-progress` blinds `main`, and it is an operator decision.**
`.github/workflows/ci.yml:17-19` groups on `github.ref` with
`cancel-in-progress: true`, so every push to `main` kills `main`'s previous
in-flight run. **A cancelled run did not fail and did not pass.** Combined with a
28-minute CI, any landing cadence faster than about 28 minutes leaves `main`
permanently unverified — which is what happened between `31258f403` and
`bb33dfb71`, several of those cancellations caused by the Steward's own doc
routes.

Shortening CI narrows that window, which is why it is recorded alongside this
work. **It does not close it**, and whether `main` should carry
`cancel-in-progress` at all is the operator's call, not this node's.
