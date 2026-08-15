---
id: V3-Z3-EMISSION-CONTROL
title: "Put a control on the SMT-LIB query generator that does not need an installed solver, so the emission path stops being witnessed only by a fleet-wide required CI job"
status: ready
owner: verify
size: S
gate: none
depends_on: [V3-Z3-PROCESS-ADAPTER]
blocks: []
github: null
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

**`D2` is optional and non-blocking.** If the ring lands `D1` and stops, that is
a complete increment. Do not hold `D1` to bundle it.

## Deliverables

**`D1` — the binary-independent emission control.** A test that fails when the
query generator produces something wrong, and that runs with the feature enabled
but no solver installed.

**`D2` — optional, only if `D1` lands.** Make the round-trip test skip when `z3`
is absent rather than asserting unconditionally, and take the
`z3-process-adapter` job out of the required aggregate. **State in the commit
what coverage moved where**, because the job stops being the witness and `D1`
starts being it.

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
gate that this criterion is measured by.

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
