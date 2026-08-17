---
id: V3-Z3-EMISSION-CONTROL
title: "Put a control on the SMT-LIB query generator that does not need an installed solver, so the emission path stops being witnessed only by a fleet-wide required CI job"
status: merged
owner: verify
size: S
gate: none
depends_on: [V3-Z3-PROCESS-ADAPTER]
blocks: []
github: https://github.com/swe-toolkit/ken/pull/2326
origin: "Steward, 2026-08-15. The gap is recorded in the merged [[V3-Z3-PROCESS-ADAPTER]] node's live-hazard block, from Adversary evt_1cg4kd7edak6c: the stub harness discards stdin, so no stub test can see the emission at all. Steward-filed per COORDINATION section 2."
---

## The gap, measured and already in the tree

**`stub(..)` writes `#!/bin/sh\ncat >/dev/null\n{body}`.** It **discards
stdin** and prints canned output.

⇒ Every stub-driven test exercises the **parser** and the **failure taxonomy**
and **cannot see the SMT-LIB emission at all.** A query generator producing
garbage still gets the canned model back, and
`parsed_model_is_candidate_not_verdict` still passes.

**Exactly one test drives a real binary** —
`installed_z3_round_trip_reaches_kernel_checked_refutation` — and it does not
skip when `z3` is absent; it asserts `Disproved` unconditionally. Its own doc
comment says the claim is *"CI installs a working process adapter, not merely
stub coverage."*

⇒ **The emission path's only witness is the `z3-process-adapter` CI job**, which
`apt-get install`s `z3` and is wired into the required `build + test` aggregate
at `.github/workflows/ci.yml:354`.

**This is a coverage hole, not an availability complaint.** A completely broken
`emit_int_expr` is caught by nothing that runs on a developer's machine, and by
nothing at all if the solver were ever absent from CI.

## The a priori best guess — build this

> **Assert the emitted SMT-LIB directly.** Drive the query builder on a fixed
> goal and check the text it produces, with no child process and no installed
> binary. The emission is a pure function from a Ken term to a string; it does
> not need a solver to be checked, and the only reason it is currently checked
> by one is that the stub harness throws stdin away.

**Attack the obvious objection before writing the test.** A golden-string
assertion over solver input is brittle against harmless formatting drift. If
that is the shape you land on, say why it is acceptable here — or assert
structure rather than bytes. **Do not make the test so loose that a garbage
generator passes it, which is the state you are repairing.**

## The second item, and it is CONDITIONAL on the first

The merged node records these two as pulling against each other and says to
decide them together:

- **This node's control** removes the emission path's dependence on an installed
  binary for its coverage.
- **Only once that control exists** does dropping the `z3-process-adapter` job
  from the required aggregate's `needs:` and pass/fail loop become available —
  a default-off feature currently holds a blocking edge on every PR in the
  fleet, including runtime's `RecursiveDescent` lane.

**The dependence is one-directional and the order is not negotiable.** Dropping
the job first leaves a stub suite that a broken query generator passes with
everything green. The merged node's own words: the one-line remedy stated there
first was **wrong**, and this is why.

**`D1` was not held to bundle `D2`, and it landed on its own — squash
`ad2c5bf12`, 2026-08-15.** That part stands: splitting them was right.

**What did not survive contact is "`D2` is optional."** This section originally
said so. Architect Finding A shows `D1` alone is executed by no CI job, so
stopping after `D1` leaves the node's own gap open with a control in the tree
that reads as coverage. **`D2a` is now required**; see Deliverables.

## Deliverables

**`D1` — the binary-independent emission control.** A test that fails when the
query generator produces something wrong, and that runs with the feature enabled
but no solver installed.

**`D2` — no longer optional, and its first step is NOT the one this frame
originally named.** Corrected by the Steward 2026-08-15 on Architect Finding A
(`evt_1esqmscbshp0f`, restated `evt_4d3hbvtjwk968`), verified against the tree.

> **The original text said to drop the `z3-process-adapter` job "because the job
> stops being the witness and `D1` starts being it." That premise is false, and
> acting on it would have removed the last emission coverage in the tree.**
>
> `mod z3_process` is `#[cfg(feature = "z3-process")]` and the feature is
> default-off. The only feature-on job runs exactly
> `cargo test --locked -p ken-elaborator --features z3-process --test
> v3_z3_process_adapter`, and `--test <name>` selects **only that
> integration-test target**. `D1` is a unit test in the **lib** target. **No CI
> job executes it.** QA's green came from a local `--lib` run, which CI does not
> reproduce.
>
> ⇒ **`D1` landing does not make `D1` the witness.** Until the run below exists,
> `D1` is an artifact that reads as coverage and supplies none — which is worse
> than no control, because it is the thing a future reader will point at when
> deciding the solver job is redundant. `main` already carries `fa18caec0`
> (*"dropping the job removes the only emission control"*), so this exact job has
> had one near-miss already, and the frame was steering into the second.

**`D2a` — make `D1` actually run, and this is the whole point.** Add a
feature-on `--lib` execution for `ken-elaborator`, as **its own job, wired into
the required `build + test` aggregate through `needs:` and the pass/fail loop.**
`D1` was demonstrated to pass with `PATH=/definitely/absent`, so it is
binary-independent and has no reason to hang off the job whose sole purpose is
installing the solver it does not need.

> ### AMENDED 2026-08-16. A CANDIDATE WAS BUILT ON THE EARLIER WORDING.
>
> **This sentence previously read *"Put it in the ordinary required `build +
> test` job, not the optional solver job."*** `190f705bb` did exactly that —
> added a checkout and a `cargo test` step **inside** `build-test` — and it is
> a correct implementation of the sentence as written. **The defect is the
> frame's, not the ring's** (Architect `evt_2q76aqesx3hnj`; `COORDINATION §14a`
> puts the amendment here rather than in the ring's fold-in).
>
> **`build-test` is a pure aggregate gate.** `ci.yml:379` says so in the block
> already carrying a do-not-touch warning: *"It is intentionally a pure gate —
> no checkout, no build, a few seconds."* Steward-verified: the job has
> `needs:` and a single `echo`/exit step, and **no `actions/checkout` and no
> `cargo` anywhere in it.**
>
> **Three things break if a build step goes inside it**, none of them a
> soundness failure — the gate stays fail-closed either way, and a new step can
> only add a failure mode:
>
> 1. **The warning comment becomes false**, in the one job the file flags as
>    load-bearing for branch protection.
> 2. **The diagnosis regresses.** Steps abort the job on failure and `All test
>    jobs passed` carries no `if:`, so a failing new step means the six
>    `::error::$name did not pass` lines never emit. **The case that costs you
>    is a shard failing *and* the new test failing** — shard attribution is
>    exactly what is lost, on the single check branch protection and the
>    publisher both read.
> 3. **`if: always()` makes it compile on every run**, including runs where
>    every test job already failed. A job documented as "a few seconds" would
>    pay a cold `ken-elaborator` build before reporting that things are broken.
>
> **The contrast the sentence was actually drawing is with the solver job**, and
> a sibling job satisfies it. The frame's own vocabulary already agreed: two
> sections above, it describes `z3-process-adapter` — **a separate job** — as
> *"wired into the required `build + test` aggregate"*. `ci.yml:381-384` gives
> the procedure: *"EVERY TEST-RUNNING JOB MUST BE LISTED IN `needs` AND CHECKED
> BELOW. Add a job above, add it in both places here."*
>
> **This is the cheap remedy, not the expensive one.** It needs no
> repository-settings change — which is the aggregator's stated reason for
> existing. `build + test` keeps its name and its contract; one new job name
> appears in the PR checks.

**The shape, so the recut is mechanical.** A sibling job mirroring
`z3-process-adapter` minus the Z3 install, then three edits in `build-test`:
add it to `needs:`, add its result binding and `echo` line, and add it to the
failure loop. **The four-places sharded-matrix discipline at `ci.yml:192-205`
does not apply** — the matrix runs this crate's lib tests with default features,
so a feature-on run is a distinct configuration, not a binary exclusion.

> ### `D2a` HAS LANDED, and its M7 was never recorded
>
> **So the gate below reads as unmet when it is not.**
>
> **Verified by blob, not by ancestry:** `.github/workflows/ci.yml` at candidate
> `17dd097b5812646f94a16b0e4e0e4229db93ff58` is **identical** to the file on
> `origin/main`, and `z3-emission-control` appears there. Decision
> `dec_4y9mg19kem5mr` is `resolved`, APPROVED by the Architect at that exact
> SHA. Shape matched its declaration: one non-merge commit, `ci.yml` only,
> `+13/-0`, purely additive.
>
> **What this does NOT say:** that `AC-7` is discharged. `AC-7` asks for a CI
> run on the PR showing `D1` actually running, which is evidence the verify ring
> supplies — landing the job and the job selecting a new target are different
> facts, and conflating them is the exact failure `D2a` exists to prevent.
>
> **What it does say:** the *"only after `D2a` is green on `main`"* condition
> below is no longer waiting on a publish. **A seat status line reading
> "awaiting publish" for this candidate is stale.**
>
> This node stays `active` with `D2b` undelivered, which is the correct record
> for a quiet lane — not a stall.

**`D2b` — only after `D2a` is green on `main`.** Make the round-trip test skip
when `z3` is absent rather than asserting unconditionally, and take the
`z3-process-adapter` job out of the required aggregate. **State in the commit
what coverage moved where.** The ordering is not stylistic: `D2b` before `D2a`
is the coverage hole this node exists to close.

> ### `D2b` LANDED 2026-08-17 AS `4011e58be`. THE NODE IS COMPLETE.
>
> Exact `fc3c6f7e49d5d0ef30db679f772b2a09dce70207`, direct base `5bac56000`, one
> non-merge commit, two paths, `+12/-5`, both verified byte-identical on
> `origin/main`. Decision `dec_2t672rb1fpt7` resolved APPROVE, QA
> `evt_39n30q555j5z`, Architect `evt_7k7rb416pv408` with amendment
> `evt_758x87jj5zn9y`. PR #2578.
>
> **The reverse-`D2a` trap was checked, not assumed.** Removing a job from the
> required set can silently deselect the emission control, and reading the YAML
> cannot catch it. Discharged from the candidate's own PR run — job
> `z3 emission control` (`95494411825`), command `cargo test --locked -p
> ken-elaborator --features z3-process --lib`, log line
> `test z3_process::tests::fixed_goal_emits_complete_smtlib_structure_without_a_solver ... ok`,
> 137 passed / 0 failed / 1 ignored. **Required selection is preserved.**
>
> **That check is one observation on one SHA, not a property.** Nothing in the
> tree fails if a later aggregate edit stops selecting `D1` without removing any
> job. The node closes with that stated rather than implied.
>
> ### CORRECTED SAME DAY: THE DISCHARGE INSTRUMENT IS BLIND, NOT MERELY NARROW
>
> **The sentence above understates the defect and it was the Steward's.** It says
> the check is one observation short of a property. Adversary
> `evt_526chtkpem6gp` measured the actual limit: **the instrument cannot
> distinguish the failure it was chosen to detect.**
>
> The mutation, run on the landed tree — insert the same probe-and-return idiom
> at the head of `fixed_goal_emits_complete_smtlib_structure_without_a_solver`
> (`z3_process.rs:194`) and re-run `--lib`:
>
> | | `D1` log line | totals |
> |---|---|---|
> | landed | `... ok` | `137 passed; 0 failed; 1 ignored` |
> | **body never executed** | `... ok` | `137 passed; 0 failed; 1 ignored` |
>
> Identical on both axes. `ci.yml:376` runs a bare
> `cargo test --locked -p ken-elaborator --features z3-process --lib` with **no
> `--exact`, no count assertion, no membership assertion** — Steward-verified.
>
> ⇒ **"Read the log, confirm it names `fixed_goal_…`" passes on a `D1` that does
> nothing.** A `... ok` row is evidence the row was *selected*, never that its
> body *ran*. The two readings differ only in a per-test duration libtest does
> not print.
>
> **The route this answers is the one the closeout asked for.** The deselection
> risk is not the YAML edit — `z3-emission-control` survives at all four sites
> (`ci.yml:402/415/421/428`), Steward-verified. It is the idiom this candidate
> introduces one file over.

> ### THE PUBLICATION CONDITION WAS UNDISCHARGEABLE AS FIRST WRITTEN
>
> `dec_2t672rb1fpt7` originally required *"fresh candidate PR CI must show `D1`
> running in the required `z3 emission control` job"* as a **pre-merge gate**.
> It could not be one, for two independent reasons:
>
> - the candidate SHA did not exist on GitHub until publication pushed it —
>   `commits/<sha>/check-runs` returned **422 no-commit-found** — so the evidence
>   the gate demanded is *created by* the act it was gating;
> - the publisher reads check-run **status**, never job-log **content**, so it
>   would have merged on green without ever looking for the name.
>
> ⇒ Amended in place to a **Steward-owned post-merge verification**, which is
> what the block above records. **This is the same shape as `AC-7` on this node**
> — a criterion naming an instrument structurally incapable of producing what it
> asks for. Two instances on one node is the tell worth carrying: when a
> criterion names a job, ask what that job *prints*, not what it *runs*.

> ### TWO RESIDUALS FILED AGAINST THIS CLOSED NODE. BOTH QUEUE — LANE 2 IS RETIRED.
>
> Adversary `evt_526chtkpem6gp`, every coordinate Steward-verified against
> `4011e58be`. Neither is soundness and neither is reachable in CI today. They
> are recorded here rather than as new nodes because the operator retired lane 2
> on 2026-08-17 and a filed node would be dead on arrival. **If verify reopens,
> start here.**
>
> **1. A RUNTIME SELF-SKIP IS OUTSIDE THE REPO'S NOT-RUN NET BY CONSTRUCTION.**
> `v3_z3_process_adapter.rs:176-183` probes `z3 -version` and `return`s on
> `NotFound`. Steward-verified as **the only runtime self-skip in `crates/`**.
> With `z3` present and with `PATH` masked, the harness prints byte-identical
> lines — the `eprintln!` at `:178` surfaces only under `--nocapture`, and
> libtest discards captured output for passing rows, so it is absent from
> exactly the artifact CI keeps.
>
> **This repo already treats a not-run row as a first-class hazard**, and that
> is what makes this a defect rather than a preference. `ignored-row-sweep` sits
> in `build-test`'s `needs` (`ci.yml:403`) and `scripts/ci-ignored-sweep.py`
> derives its population from a source scan for `#[ignore = "…"]`, cross-checked
> against `cargo nextest list --run-ignored=only`. **A runtime `return` carries
> no attribute, so it is outside that population by construction** — the ignored
> count stayed `1` across the mutation. ⇒ The repo runs a required job enforcing
> *"every not-run row is enumerated and claimed"*, and this candidate adds a
> not-run row through the one door that job has no lock on.
>
> Remedy is the owning ring's and either half is cheap: make absence an
> `#[ignore]`-shaped row the sweep can enumerate, **or** leave the skip and say
> in the test's own doc block what `ok` no longer entails. That block
> (`:168-173`) currently reads *"MEASURED: the configured external binary can
> propose a model that the kernel accepts as a refutation."* **After this change
> that sentence is conditional and nothing in the file says on what.**
>
> **2. `ci.yml`'s OWN `needs` RULE IS NOW FALSE IN THE FILE THAT ASSERTS IT.**
> `ci.yml:390-393`: *"EVERY TEST-RUNNING JOB MUST BE LISTED IN `needs` AND
> CHECKED BELOW. This job is what branch protection actually reads, so a test job
> missing from here reports GREEN no matter how it failed."* Steward-verified:
> `z3-process-adapter` is **defined and running tests at `:354`** and appears in
> **no `needs` list**, while `z3-emission-control` appears at `402/415/421/428`.
>
> **The repo's established shape for a non-blocking test job is the opposite
> one:** `ignored-row-sweep` stays in `needs` and makes its findings exit zero
> (`ci.yml:130-132`). `D2b` needed the adapter job to stop blocking and took the
> route the file's own rule forbids, when a non-forbidden route with in-tree
> precedent existed. The job still burns CI time and now reports green however
> it fails.
>
> The Adversary flagged this as outside `COORDINATION §10⁻a` and did not pursue
> it. **It is the Steward's, and it is recorded rather than dropped.**

## The de Bruijn fixture: what to build, and the one-binder trap

Architect Finding B (optional, additive) says the landed `Π a b : Int. a = b`
fixture cannot pin the de Bruijn mapping, because `=` is symmetric and an
inverted `emit_int_expr` still yields an equivalent query. **True, and the
proposed replacement is right — but its one-line description is not, and
implementing the description rather than the fixture produces a test that pins
nothing.** Adversary `evt_4r7epys21b76a`, re-verified against `origin/main` by
the Steward before recording.

**The trap.** `emit_int_expr` maps `Var(index) → k{binders - 1 - index}`
(`z3_process.rs:63`). Finding B describes the repair as *"a second fixture with
one variable and one literal."* Implemented literally as **one binder**,
`Π a : Int. a = 0`, the arithmetic is `1 - 1 - 0 = 0` and the identity mapping
`k{index}` also gives `k0`.

⇒ **At one binder the inversion and the identity are the same function.** That
fixture emits `(= k0 0)` under both and reds on neither — strictly weaker than
the fixture it replaces, which at least reds on argument order.

**Build the fixture Finding B actually names: two binders, only one used.**

| mapping | `Π a b : Int. a = 0` emits |
|---|---|
| current (inversion) | `(assert (not (= k0 0)))` |
| identity (the bug) | `(assert (not (= k1 0)))` |

Different queries, and **semantically** different — one asks for a counterexample
in the outer variable, the other in the inner.

**And Finding B's characterization understates the gap.** It calls
`Π a b : Int. a = b` *"the one shape in the supported fragment where the mapping
is invisible."* `emit_query` accepts only `Term::Eq` (`:36`) and `emit_int_expr`
only `Var` and `IntLit` (`:63-64`), so **no goal in the fragment has an
asymmetric relation** — the mapping is invisible in essentially *every*
two-variable shape. **The escape is dropping a variable, not changing the
operator.**

**Severity, and it bounds how hard to argue this in `D2b`:** a wrong mapping
costs **completeness, never soundness**. `specialize_int_goal` re-substitutes and
re-checks, so a mis-mapped assignment specializes to a goal that does not refute
and the seam returns `Unknown`.

**One gap recorded and not scheduled:** `emit_int_expr`, `parse_assignment`, and
`specialize_int_goal` do agree on outermost-first ordering — checked, no defect —
but **nothing in the tree pins that agreement**, and the halves live in different
files. It is true by inspection only. Not this node's scope; do not let a future
reader believe the emission control covers it.

## Acceptance criteria

**`AC-1`.** `D1` fails on a deliberately broken generator, demonstrated by a run
— mutate the emission, watch it red, revert. Not asserted.

**`AC-2`.** `D1` passes with the `z3-process` feature enabled and **no `z3` on
`PATH`.** This is the whole point of the node; demonstrate it.

**`AC-3`.** The set of goals `is_linear_int_expr` accepts is unchanged. Its body
is not touched. Narrowing it drops goals the seam currently serves and is not
authorized here.

**`AC-4`.** No kernel change, no new postulate, no new registrant, and
`trusted_base()` is unchanged. The solver remains an oracle and never an
authority.

**`AC-5`.** With the feature disabled the tree behaves exactly as it does today.

**`AC-6`.** No-regression, in CI (`COORDINATION §12`). **If `D2` lands, say
explicitly in the handback which required checks changed**, since `D2` edits the
gate that this criterion is measured by. **Under the amended `D2a` the expected
answer is: one new job name appears in the PR checks, `build + test` keeps its
name and its contract, and branch protection needs no edit.** An answer that
reports a change to `build + test` itself is the signal the recut went back into
the aggregate.

**`AC-7`.** `D2a` is demonstrated by **a CI run on the PR that shows `D1`
executing** — the test name in a job's output, not the workflow diff. A workflow
edit that looks correct and selects no new target is the exact failure `D2a`
repairs, and it cannot be caught by reading the YAML. **`D2b` may not land until
this is shown green.**

> **"A job's output" means the new job's log — NOT `build + test`'s.** This
> criterion needed no amendment, but the `190f705bb` review request paraphrased
> it as *"exact PR `build + test` output naming ..."*, and that narrowing is
> part of what made the aggregator look like the only place the step could go
> (Architect `evt_2q76aqesx3hnj`). **The aggregate runs no tests and never
> prints a test name**; requiring one from it is unsatisfiable under any correct
> implementation. A dedicated job's log satisfies `AC-7` exactly as written.

## Banned scope

- **The bare `PATH` resolution of `z3`.** Confirmed real and dispositioned in
  [[V3-Z3-PROCESS-ADAPTER]] as a **graduation gate**, owned by whoever proposes
  making the feature developer-facing or default-on. Its severity is local code
  execution, not soundness, and it is bounded by the feature being off by
  default. **Do not "fix" it here by deleting the default** — off-by-default with
  a `PATH`-resolved default is the correct posture for a feature nobody ships.
- **The FO direction of D-route displacement.** Closed structurally in the
  merged predecessor. **It must not be re-filed**, and the corpus control once
  offered for it is withdrawn: it would replace a total argument with a sampled
  one.
- **Throughput characterization.** `docs/program/wp/V3-z3-throughput-evaluation.md`
  step 2 needs a catalog-scale proof-heavy corpus that does not exist. Not here,
  and do not recommend for or against expanding solver use.
- **cvc5, and proof reconstruction.** Separate designs, separate nodes.

## Provenance

The gap and both carries are recorded in the merged [[V3-Z3-PROCESS-ADAPTER]]
node, from Adversary `evt_1cg4kd7edak6c`. Nothing above is this frame's own
measurement; the ring should re-derive the stub body and the CI wiring against
current `main` before writing, since both are cited from a node written at merge
time.
