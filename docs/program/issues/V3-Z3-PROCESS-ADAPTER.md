---
id: V3-Z3-PROCESS-ADAPTER
title: "The z3 half of the round-trip: an off-by-default external solver that proposes candidate assignments into the kernel-gated witness seam, adding a dependency and zero trusted base"
status: ready
owner: verify
size: M
gate: none
depends_on: [V3-D-OPEN-GOAL-WITNESS-ROUTE]
blocks: []
github: null
origin: "Steward, 2026-08-15, framing the operator's directed z3 round-trip lane. Split from V3-D-OPEN-GOAL-WITNESS-ROUTE so the routing gap and the soundness seam land before any dependency decision. The deferred docs/program/wp/V3-z3-throughput-evaluation.md frame supplies the guardrails; its throughput-measurement half is NOT this node. Steward-filed per COORDINATION section 2."
---

> # FRAMED AND SHOVEL-READY. `draft` IS A SCHEMA CONSEQUENCE, NOT FRAMING DEBT.
>
> `check-issue-schema.sh --strict` fails a `ready` node whose `depends_on` is
> itself `ready`. This node flips to `ready` the moment
> [[V3-D-OPEN-GOAL-WITNESS-ROUTE]] goes `active`. **Nothing here is owed by the
> Steward** — do not read the status as unstarted framing.

## What this node is

The **solver** end of the operator's directed round-trip: an obligation leaves
Ken, an external solver answers, and a verdict comes back **through the kernel**.

Its whole soundness argument is inherited, not invented. `attempt_with_refutation`
(`prover.rs:254`) checks `q : φ → Bottom` with the kernel before returning
`Disproved`, and yields `Unknown` when the check fails (`:265`). The predecessor
generalizes that seam to open goals. **This node attaches z3 to it and changes
nothing about why a verdict is believed.**

## Guardrails — verbatim from `wp/V3-z3-throughput-evaluation.md`

- **Z3 is an oracle, never an authority.**
- `proved` still requires a kernel-checked certificate.
- Solver failure, timeout, nondeterminism, or a missing certificate yields
  `unknown` or a rejected certificate — **never a false proof**.
- The disabled path remains the baseline and must keep passing.
- **No kernel trusted-base expansion is in scope.**

`23 §6` is the spec anchor: Z3 is the primary solver, **there is no external
proof-checker dependency**, and Ken's own kernel is the proof checker.

## Deliverables

**`D1` — the binding decision, costed before it is taken.** Process invocation
of a `z3` binary over SMT-LIB versus a linked crate. State for each: what enters
`Cargo.toml`, what CI must install, what happens when the binary is absent, and
whether builds stay reproducible. **The absent-solver path is the one that
matters** — it must be the disabled baseline, not a build failure.

**`D2` — the adapter, off by default.** A cargo feature that is not in the
default set. With the feature off, the tree behaves exactly as the predecessor
left it.

**`D3` — the emission and ingestion.** Ken goal to SMT-LIB, and the solver's
model back to a candidate assignment the predecessor's seam consumes.
**Ingestion parses a candidate, never a verdict.**

**`D4` — the honest-failure matrix, run.** Timeout, `unknown`, malformed output,
absent binary, and a **deliberately wrong model**. Every one yields `Unknown`.

## Acceptance criteria

**`AC-1`.** With the feature disabled, every changed crate's behaviour is
unchanged and the suites are green. **Control: the disabled run is the
baseline.**

**`AC-2` — the adversarial control, and it is the node.** A solver stub that
returns a model which does **not** refute the goal produces `Unknown`, not
`Disproved` — demonstrated by a run, not by pointing at the kernel check.

**`AC-3`.** `trusted_base()` gains nothing from any solver-assisted verdict.
Compare the base before and after on the same corpus.

**`AC-4`.** No new postulate, no new registrant, no kernel change.

**`AC-5`.** Determinism: the same input yields the same verdict across runs.
Where the solver is nondeterministic, the **verdict** must not be — a
nondeterministic search that changes only which candidate is proposed is
acceptable; one that changes the verdict is not, and is a hard stop.

**`AC-6`.** No-regression, in CI (`COORDINATION §12`).

## Banned scope

- **Throughput characterization.** `docs/program/wp/V3-z3-throughput-evaluation.md`
  step 2 needs a catalog-scale proof-heavy corpus that does not exist, and that
  frame is deferred for exactly that reason. **Do not measure throughput here
  and do not recommend for or against expanding solver use** — this node builds
  the path, it does not evaluate it.
- **The FO/Kripke route.** Spec-blocked; see [[V3-KRIPKE-THEORY-CLOSURE]].
- **cvc5.** `23 §6` names it as an optional second solver. Not now.
- **Proof reconstruction (SMTCoq-style).** `23 §3.2` offers reflection **or**
  reconstruction; the predecessor's seam takes the reflection route, which needs
  no new theorem. Reconstruction is a separate design and a separate node.

## Stop condition — return to the operator, not the Architect

**If `D1` concludes the dependency cannot be made optional** — a linked crate
that builds unconditionally, or a CI requirement that cannot be skipped — stop.
That is a dependency and build-complexity call above the ring and above me.
