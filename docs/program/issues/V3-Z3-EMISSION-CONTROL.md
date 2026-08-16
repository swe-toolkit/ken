---
id: V3-Z3-EMISSION-CONTROL
title: "Put a control on the SMT-LIB query generator that does not need an installed solver, so the emission path stops being witnessed only by a fleet-wide required CI job"
status: active
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

**`D2b` — only after `D2a` is green on `main`.** Make the round-trip test skip
when `z3` is absent rather than asserting unconditionally, and take the
`z3-process-adapter` job out of the required aggregate. **State in the commit
what coverage moved where.** The ordering is not stylistic: `D2b` before `D2a`
is the coverage hole this node exists to close.

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
