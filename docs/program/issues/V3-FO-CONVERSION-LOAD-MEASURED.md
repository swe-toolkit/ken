---
id: V3-FO-CONVERSION-LOAD-MEASURED
title: "Measure what kernel conversion actually costs when it runs check_cert, on real source programs, before any of it is argued about"
status: ready
owner: language
size: M
gate: none
depends_on: [V3-FO-OBLIGATION-SIGNATURE-DISCOVERY]
blocks: []
github: null
origin: "Steward, 2026-08-15, on the operator's posture ruling: 'Nothing ventured, nothing gained. We will only know the cost if we build it and test it on real programs, so we should do that.' Settles the evaluator/TCB posture half of spec/20-verification/23-prover.md section 4.4, which that section assigned to the Architect and operator. Steward-filed per COORDINATION section 2."
---

## The operator's ruling, and what it settled

> Nothing ventured, nothing gained. We will only know the cost if we build it
> and test it on real programs, so we should do that.

**This settles the evaluator/TCB posture** — the half `23 §4.4` left open after
the artifact home was ruled in `docs/design/fo-route-theorem-home.md`.

**We are NOT requiring `18 §6`'s subject reduction and confluence to be
mechanized before route FO is built and exercised.** The new load class is
accepted as something to **measure**, not to pre-empt.

> ### WHAT THIS DOES NOT CLEAR, because it will otherwise read as cleared
>
> `23 §4.4` still forbids `proved` **"until both theorems are kernel-checked in
> an approved home."** That is a precondition, not a decision, and this ruling
> does not touch it. `embedding_adequacy` and `checker_soundness` are unproved,
> unstarted, and have no node. **Steward framing debt, recorded here so the
> posture ruling is not misread as authorizing `proved`.**

## What is actually being measured, and why it is cheap

The expensive computation is one equation, from `23 §4.4`:

```
ok : check_cert (embed Sigma f) pi = True
```

**`refl True` at that type forces exactly the conversion work in question** —
kernel conversion must evaluate `embed Sigma f` and then run `check_cert` over
the whole derivation tree.

⇒ **It requires NEITHER theorem.** `embedding_adequacy` and `checker_soundness`
turn that computation into a *discharge*; they are not what makes it expensive.
**So the cost number is obtainable long before the metatheory is proved**, and
this node exists to take it at the earliest point it is real rather than at the
end.

## The hazard this node exists to avoid, stated as a constraint

**"Real programs" means SOURCE-LEVEL programs.**

Measured 2026-08-15 on `RT-SYNTHESIZED-ENV-RECORD-OCCURRENCE` `D3`: an entire
product fork rested on rows that turned out to have **no source-level witness
at all** — the governed shape was hand-authored `RuntimeExpr` reachable by no
program a user could write. **A measurement taken on hand-constructed inputs
would have answered confidently and meant nothing.**

⇒ **Every obligation measured here must arise from compiling a Ken source
program.** A hand-built `IForm`, a synthesized `Cert`, or an obligation
constructed in Rust is **not** a real program and its timing is not this node's
deliverable.

## A SECOND HAZARD, AND IT IS ABOUT WHETHER THE WORKLOAD IS THE RIGHT ONE

**Architect `evt_6psj9y9e0ah41`, on approving
[[V3-FO-OBLIGATION-SIGNATURE-DISCOVERY]] `D1`-`D3`. He raised it as
non-blocking there and named this node as its home.**

Discovery is deliberately a candidate generator that may be wrong — a missed
occurrence costs completeness, never soundness. **The entire safety weight sits
on conjunct 3:** the quoted `f` must `denote` back to the original obligation
under the kernel's own `convert`. That check is what stops an arbitrarily wrong
signature from being adopted.

**Nothing currently observes conjunct 3 refusing.** The predecessor's two
refusal controls are both caught by conjuncts 1 and 2 — ambiguity and
declaration shape — which run *before* preservation. So the check carrying the
safety argument may never have been seen returning `false`.

**Why that is this node's problem rather than a general wish.** `convert` is
type-directed and carries an unconditional proof-irrelevance shortcut at
`conv.rs:341`: if `ty : Ω`, any two terms are definitionally equal. Conjunct 3
passes `phi_ty = infer(phi_closed)` as that `ty`. The shortcut cannot fire
today, because `check.rs:296` gives `Ω_ℓ : Type (suc ℓ)`, which is not
`Omega(_)`. **That is a property of the kernel, not of the elaborator.** If `Ω`
ever became self-typed, or the guard widened, conjunct 3 would start returning
`true` for everything — the mechanism would keep compiling and the suite would
stay green.

⇒ **A vacuous conjunct 3 makes every obligation this node measures the wrong
obligation.** Discovery would adopt signatures that do not denote back, so the
`check_cert (embed Sigma f) pi` the measurement times would not be the one the
source program stated. `AC-1` demands provenance from a named Ken source
program; a silently-vacuous preservation check breaks that provenance without
breaking a test.

## Deliverables

**`D0` — pin conjunct 3's discriminating power, before measuring anything.**
An obligation where discovery **succeeds** and returns a signature whose
`denote` does **not** convert back to `phi_closed`, asserted to fall through to
ordinary IPC. **It must reach conjunct 3** — a case refused by conjuncts 1 or 2
does not test this and is the failure mode to avoid. The control goes red the
day the proof-irrelevance shortcut starts firing here, which no existing test
would notice.

**`D1` — the corpus.** A set of Ken source programs whose compilation produces
first-order obligations route FO can quote. **State how many were found and
whether the corpus had to be written**; if real programs producing such
obligations are scarce, that is a finding about the route's reach and is worth
more than a timing table.

**`D2` — the measurement.** For each, the wall-clock and, where obtainable, the
reduction-step count that conversion spends on
`check_cert (embed Sigma f) pi = True`. **Report the distribution, not an
average** — the interesting number is the worst case, and a mean hides it.

**`D3` — the shape of the growth.** How the cost scales with certificate size
and formula depth. **A blowup on small input is a checker bug, not a budget
problem**, and this deliverable is what tells the two apart.

**`D4` — the honest report.** Whether conversion terminated on every case, and
any case where it did not. **A non-terminating or pathological case is the most
valuable result this node can produce** and must be reported as a result rather
than worked around.

## Acceptance criteria

**`AC-1`.** Every measured obligation traces to a named Ken source program.
**Demonstrate the provenance**, do not assert it.

**`AC-1a`.** `D0`'s control **demonstrably reaches conjunct 3** — show that the
input it uses passes conjuncts 1 and 2, so the refusal it observes is
preservation refusing and not declaration validation refusing. **Demonstrate it
against the mechanism, not by assertion.** A control that never reaches the
check it names is the failure this deliverable exists to prevent, and it looks
identical to a control that works.

**`AC-2`.** The measurement is of kernel conversion, not of the solver. z3's
wall-clock is out of scope — it runs outside the kernel and carries no
authority (`23 §4.3`). **If a number in the report includes solver time, it is
the wrong number.**

**`AC-3`.** No `proved` verdict is produced for FO. `23 §4.4`'s reservation is
untouched by this node, which measures the computation rather than trusting it.

**`AC-4`.** No new kernel primitive and no trusted axiom.

**`AC-5`.** No-regression, in CI (`COORDINATION §12`).

## Banned scope

- **Proving either theorem.** That is the unfiled successor and it is not this.
- **Optimizing what is measured.** Measure first. A repair chosen before the
  distribution is known is a guess with a number attached.
- **Widening the slice.** `§4.5`'s bounds are unchanged.

## Sequencing

**Depends on `V3-FO-OBLIGATION-SIGNATURE-DISCOVERY`** — until `D1`-`D3` there,
no real obligation reaches route FO at all, so there is nothing to measure.
**Do not start this before that lands**, and do not substitute helper-level
inputs to start earlier; that substitution is exactly what `AC-1` forbids.

**`D0` runs first and does not wait on the corpus.** It is a control on the
predecessor's mechanism, not a measurement, and it needs no source program. If
it cannot be made to fail against a signature that does not denote back, that is
a finding about conjunct 3 and it is worth more than the timing table — **report
it and stop rather than proceeding to measure a workload whose provenance the
control could not establish.**

## Provenance

Operator ruling, 2026-08-15, on the Steward's brief of the `23 §4.4` posture.
`OQ-12` recorded the residual as a **feasibility** risk to be retired by a thin
front-loaded slice; `V3-FO-KRIPKE-SLICE` is that slice and merged at
`55783edf0`. The Architect's `docs/design/fo-route-theorem-home.md` §4 named
this the one thing worth measuring and declined to settle it, which is the
question the operator has now answered.
