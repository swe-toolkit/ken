---
id: V3-Z3-PROCESS-ADAPTER
title: "The z3 half of the round-trip: an off-by-default external solver that proposes candidate assignments into the kernel-gated witness seam, adding a dependency and zero trusted base"
status: merged
owner: verify
size: M
gate: none
depends_on: [V3-D-OPEN-GOAL-WITNESS-ROUTE]
blocks: [V3-Z3-EMISSION-CONTROL]
github: null
origin: "Steward, 2026-08-15, framing the operator's directed z3 round-trip lane. Split from V3-D-OPEN-GOAL-WITNESS-ROUTE so the routing gap and the soundness seam land before any dependency decision. The deferred docs/program/wp/V3-z3-throughput-evaluation.md frame supplies the guardrails; its throughput-measurement half is NOT this node. Steward-filed per COORDINATION section 2."
---

> # MERGED 2026-08-15 at `9bc035710` (PR #2288), exact `b22e2dff0`.
>
> **The z3 half of the operator's directed round-trip is in the tree.** Six
> paths, `+455/-6`, declared base `46a8ba199` which was also current
> `origin/main`. Blob identity MATCH on all six. Decision `dec_7jyr8h41epg66`
> read `resolved` from the object; QA `evt_6b2fqsw5wdj4q`, Architect
> `evt_5qx7285r9mgdp`.
>
> **Verified independently rather than taken from the request:** `Cargo.toml`
> adds only the feature `z3-process = []` with no dependency, and `Cargo.lock`
> is untouched — so the zero-supply-chain-delta claim holds. **The solver is an
> oracle and never an authority**; ingestion parses a candidate assignment that
> the existing kernel-gated seam disposes of.
>
> **Read the SUCCESSOR LEDGER below before acting on anything here** — both
> Architect carries are dispositioned there, and the `PATH`-resolution one is a
> live graduation gate, not a closed item.
>
> > **A Steward failure worth the line, because it cost the ring a lawful
> > start.** The kickoff announced the `draft → ready` flip while the flip was
> > still an unpublished commit on `steward/work`. `origin/main` said `draft`,
> > and `verify-leader` correctly refused to cut a branch on a post that
> > disagreed with the tracked node (`evt_712tyaew4ynmw`).
> >
> > ⇒ **A kickoff may not announce a state change that has not published.**
> > Publish the flip, then kick — or name the commit and say plainly that it is
> > queued. The ring is right to treat the artifact as authoritative over
> > anything the Steward says about it.

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

## The predicate that selects your input is MIS-NAMED. Measured, not read.

**`is_linear_int_expr` is the gate deciding which goals reach the seam you are
attaching to, and of the three words in its name only `expr` survives.**
Architect finding 1 on `0a45f717` said it checks neither linearity nor
arithmetic; the Adversary then measured what it *does* check (`evt_7468zj89pdryh`,
grounded at `origin/main`):

```rust
Term::Var(index)  => *index < binders,
Term::IntLit(_)   => true,
Term::App(partial, right) => {
    let Term::App(operation, left) = partial.as_ref() else { return false };
    matches!(operation.as_ref(), Term::Const { .. })
        && is_linear_int_expr(left, binders) && is_linear_int_expr(right, binders)
}
_ => false,
```

| axis | what it actually enforces |
|---|---|
| arity | **binary constant application only** — a 3-ary application destructures to `operation = App(..)`, not `Const`, and is rejected |
| leaves | **bound variables and integer literals only** — a free constant as a leaf is rejected |
| the operation | **nothing.** `bytes_concat`, or any user binary function, passes |
| linearity | **nothing.** `mul_int x x` passes |
| typing | **nothing.** No term is required to be `Int` |

⇒ The honest contract is *"a binary-constant tree over bound variables and
integer literals."*

**Do not carry forward the earlier one-line summary — *"accepts any `Const`
application"*.** It states the unbounded axis and omits the bounded one, so it
reads as unbounded overall. **The population reaching you is bounded by the
leaf rule** even though the operator rule is not, and an adapter sized against
the looser sentence assumes more reach than exists.

**The widening is verdict-neutral today, and that is checkable rather than
hopeful.** A non-arithmetic term captured by the predicate can only reach a
**candidate** refutation, which `attempt_with_refutation` puts through the
kernel; a bogus one is rejected and the obligation lands on the same `Unknown`
it would otherwise have had. So the mis-naming costs exactly one thing: **a
false contract for whoever writes against the name.** That is you.

**`D5` below is the one-line fix**, and it is the Architect's own remedy —
rename to what it verifies, or state the non-guarantee in its doc comment.

## WHY THE TRUST CLAIM HOLDS — it is carried by a TYPE, not by discipline

Verified on the landed candidate (Adversary `evt_1cg4kd7edak6c`). Record the
**reason**, not just the conclusion, because the reason is what a successor must
not break.

**`attempt_d_with_z3_process` obtains a `Vec<BigInt>` from
`candidate_assignment` — or `Unknown` — and hands it to the pre-existing
`attempt_d_with_int_assignment`.** ⇒ **Z3's output cannot carry a verdict, a
certificate, or a term. It can only propose numbers.** The
oracle-not-authority property is **structural in the ingestion type**, not
maintained by care at the call site. That is the strongest available form.

**And the placement composes with the D-route containment argument.** The z3
call is the **final** statement of `attempt_d`, after IPC and the ground
refutation floor have both failed ⇒ enabling the feature can only turn
`Unknown` → `Disproved` and **can never displace a `Proved`.** Feature-on
cannot lose a verdict, by the same argument shape as the D reroute.

**Feature gating checked:** `default = []`, every reaching site is
`#[cfg(feature = "z3-process")]` with no `any(test, …)`, so a default
`cargo test` does not invoke a process either.

> **One bounded limit worth a clause, because two different events read as one
> in the failure taxonomy.** There is a real timeout — a `try_wait` poll loop
> with `kill()`/`wait()` on expiry, a kill on write failure, `stderr` nulled.
> **But the poll loop never drains stdout**, so an answer exceeding the OS pipe
> buffer blocks the child, which then cannot exit, so **the timeout fires and
> yields `Unknown`.** Fail-safe, and not live for a handful of `Int` bindings.
> **The drain and the timeout are mutually exclusive in this structure and the
> code correctly chose the timeout** — but *"timeout on a hung solver"* and
> *"timeout on an oversized answer"* are not the same event.

## SUCCESSOR LEDGER — the two Architect carries, dispositioned. Steward, 2026-08-15.

`verify-leader` asked for the authoritative disposition on both so neither is
silently dropped nor silently re-filed (`evt_g6tsbz6944c3`). **These are the
answers. Do not re-open either on the ring's own motion.**

### (a) Bare `PATH` resolution of `z3` — CONFIRMED, and it is a GRADUATION GATE

**Real, and it is in the landed candidate.** `Z3ProcessConfig::default()` sets
`program: "z3".into()` (`prover.rs:466`), reached from `prover.rs:396`, and
`Command::new(&config.program)` (`z3_process.rs:70`) resolves a bare relative
name through `PATH`.

**Its severity is code execution, NOT soundness, and the distinction is what
makes it a gate rather than a blocker.** A hostile `z3` on `PATH` cannot forge a
verdict — it proposes a **candidate assignment**, and `attempt_with_refutation`
puts it through the kernel, so a bogus one is rejected and the obligation lands
on the same `Unknown`. What a planted binary gets is **arbitrary execution as
the build or test user.** That is a genuine local supply-chain vector and it is
bounded by the feature being off by default.

⇒ **Before this feature is developer-facing or default-on, `program` must come
from explicit absolute-path configuration, and a bare relative default is not
permitted at that point.** The carrier already exists — `program` is a
`PathBuf`, not a hardcoded string — so this is a default-and-policy change, not
a redesign. **Whoever proposes graduation owns discharging it**, and graduation
is an operator call, not the ring's.

> **Do not "fix" it now by deleting the default.** Off-by-default with a
> `PATH`-resolved default is the correct posture for a feature nobody ships;
> forcing an absolute path today buys nothing and makes the disabled baseline
> harder to run.

### (b) The FO direction of D-route displacement — CLOSED STRUCTURALLY

**Not owed, and it must not be re-filed.** The argument is total over the code
path and is written out in full in the merged predecessor
[[V3-D-OPEN-GOAL-WITNESS-ROUTE]] — read it there. In short: `ctx` is built
before the match so all three arms take identical arguments; `attempt_fo` and
`attempt_ho` are `attempt_ipc` verbatim; `attempt_ipc`'s only `Proved` is
`try_ipc_cert(..) == Some`; and `attempt_d` opens with that identical call.
⇒ FO/HO `Proved` ⟺ D `Proved`, for every obligation.

**The one gap in the Steward's original version — that a dispatcher doing
something extra on one path would break step 1 — was closed by reading**
(Adversary `evt_7468zj89pdryh`). Nothing about the argument is now inferential.

**The corpus control the Steward offered is WITHDRAWN, and withdrawing it is the
point.** It would be evidence over whatever obligations the V3 corpus happens to
contain, where the argument is a proof over all of them. **Requiring it would
replace a total argument with a sampled one and produce a green that reads as
stronger evidence than the thing it replaced.** Cheapness is not the criterion.

> **If the Architect disagrees, that is his call and I will re-open on his
> ruling** — but **the ring may not spend a turn on the measurement in the
> meantime.** Raising it is one line; running it is a turn on the priority lane.

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

**`D5` — make `is_linear_int_expr`'s name match what it checks.** Rename it to
the recognized shape, or leave the name and state the non-guarantee in its doc
comment. **One line either way, and it is not optional** — the section above is
the measurement, and this node is the one that writes against the contract.
**Do not "fix" the predicate to enforce linearity or arithmetic**: narrowing it
drops goals the seam currently serves, which is a behaviour change nobody has
authorized and is not in this node's scope.

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

> ### LIVE HAZARD ON THE LANDED CANDIDATE — the CI job is a FLEET-WIDE gate
>
> **Recorded, not blocked.** `b22e2dff0` adds a `z3-process-adapter` job that
> `apt-get install`s `z3`, and wires it into the **required `build + test`
> aggregate** — it is listed in `needs:` and in the pass/fail loop.
>
> ⇒ **An apt mirror failure or a distro `z3` change reds every PR in the fleet**,
> including runtime's `RecursiveDescent` lane, which has nothing to do with z3.
> A default-off feature has acquired a blocking edge on everyone's merges.
>
> **This was merged deliberately** — both required gates were in, and a
> hypothetical infrastructure flake is not a sequencing constraint
> (playbook §4c, the safety-of-`main` trap).
>
> ### THE ONE-LINE REMEDY I FIRST NAMED IS WRONG. DO NOT APPLY IT AS STATED.
>
> **Adversary `evt_1cg4kd7edak6c`, and it is the reason this block was
> rewritten.** I wrote *"drop the job from the aggregate's `needs:` and pass/fail
> loop, leaving it advisory."* **That would remove the only control on the query
> generator**, and neither of this node's two recorded items mentions that
> component.
>
> **`stub(..)` writes `#!/bin/sh\ncat >/dev/null\n{body}` — it DISCARDS STDIN
> and prints canned output.** So every stub test exercises the **parser** and the
> **failure taxonomy** and **cannot see the SMT-LIB emission at all**: a query
> generator producing garbage still gets the canned model back, and
> `parsed_model_is_candidate_not_verdict` still passes.
>
> **Only `installed_z3_round_trip_reaches_kernel_checked_refutation` drives a
> real binary**, and it does not skip when `z3` is absent — it asserts
> `Disproved` unconditionally. Its own doc says the claim is *"CI installs a
> working process adapter, not merely stub coverage."*
>
> ⇒ **That claim is made true BY the required CI job.** Making the job advisory
> leaves a stub suite that a **completely broken query generator passes with
> everything green.**
>
> ⇒ **The two items are individually right and pull against each other, so
> decide them together.** Keeping the job required is what keeps the emission
> path witnessed. **Dropping it first requires a replacement control over
> `emit_int_expr` / the query builder that does not depend on an installed
> binary.** Pinning or vendoring the solver is the heavier alternative.

**`AC-7`.** `D5` landed, and **the set of goals `is_linear_int_expr` accepts is
unchanged** — demonstrated by the predicate's body being untouched, or by a
control if it was touched. A rename or a doc comment satisfies this; a narrowing
does not.

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
