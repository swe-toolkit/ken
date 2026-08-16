---
id: LANG-CTOR-PREMISE-ELABORATION-DIVERGES
title: "A data constructor whose premise applies a recursive function to a telescope-bound variable diverges during elaboration -- proof-carrying inductive families are unavailable in Ken"
status: ready
owner: language
size: L
gate: none
depends_on: []
blocks: [V3-FO-CHECKER-SOUNDNESS]
github: null
origin: "Steward, 2026-08-16, on the merge of V3-FO-CHECKER-SOUNDNESS D1a (PR #2428). D1a hard-stopped with a bounded finding: FokDerivation cannot be authored because every constructor it needs diverges the elaborator. Architect dec_47g7jtcrb5rhv records the architectural reading and the liveness-not-soundness severity. Steward-filed per COORDINATION section 2."
---

## What is broken

**A `data` declaration whose constructor telescope carries a premise applying a
recursive user-defined function to a telescope-bound variable does not
elaborate.** It diverges — unbounded, still-growing allocation, measured above
10 GiB RSS and climbing after roughly 100 seconds.

**It is not a deep-but-finite computation.** A 1 GiB `run_with_big_stack`
thread, the remedy that has worked for every prior deep-computation problem in
this family, runs away identically.

## The axis, isolated by elaborating `.ken` rather than by reading source

Measured at `V3-FO-CHECKER-SOUNDNESS` `D1a` (`8d6d7d545`), whose test file is
the reproduction and is on `main`.

| shape | result |
|---|---|
| `data` constructor telescope + recursive function on an abstract binder | **diverges** |
| `fn`/`theorem` parameter + the same recursive application | instant |
| `data` constructor telescope + a **non**-recursive function | instant |

**Both single-factor moves away from the diverging cell fix it**, which isolates
the conjunction rather than either factor.

**Excluded, and this is what makes the finding usable:** indexing and
self-reference are not implicated — a trivial non-indexed, non-self-referential
`data` with one `Equal Bool (fok_nat_eq a b) True` premise diverges identically,
and `fok_nat_eq` is the smallest recursive function in `FoKripke.ken`, two
non-nested match arms. `fok_nth_form`, `List` and `Option` complexity are
excluded by that same minimal repro.

⇒ **The reproduction is already minimal.** Do not re-derive it.

## Why this is worth an `L` rather than a workaround

**Proof-carrying inductive families are effectively unavailable in Ken** —
constructors carrying equality premises about a recursive function. That is the
standard idiom for derivations, well-typed syntax, and inductively defined
relations with computed side conditions.

**The corpus systematically avoids the shape**, so the defect was invisible to
the entire suite by construction: every existing inductive family is either
premise-free or carries a nullary proof marker, never a computed `Equal`
hypothesis. `FokDerivation` is the first program anyone tried to write this way.

**Severity is liveness, not soundness** (Architect, `dec_47g7jtcrb5rhv`). The
elaborator fails to terminate; it does not emit a wrong term, and the kernel
re-checks whatever it does produce. **Do not escalate this to the Kernel ring as
a soundness matter** — but see the ownership fork below, which is a different
question.

## The guess to attack, and the fork it turns on

Per the framing rule, this is the a-priori best guess and discovery is expected
inside the attempt. **If it is wrong, that is a result — report what you found.**

**Guess: something on the constructor path reduces a term containing a recursive
function applied to a stuck variable, and unfolds the recursive definition
without a guard for the stuck match.** A recursive function applied to a
telescope-bound variable cannot reduce — its match scrutinee is stuck — so a
reducer that unfolds the definition anyway re-enters itself forever. That
matches every measured cell: the `fn`/`theorem` path does not perform this
reduction, and a non-recursive function unfolds once and terminates.

**Candidate sites, each an anchor to re-find rather than a value to check:**

- `ken-elaborator/src/data.rs`, `elab_data_decl` — the constructor-telescope
  entry point, and the one `whnf(env, ctx, &infer(...))` call in that file;
- `ken-kernel/src/inductive.rs`, `check_positivity` /
  `derive_parameter_polarities_inner`, reached through `declare_inductive`;
- the constructor-universe check, which `check.rs` documents as re-checking
  signatures, strict positivity, and constructor universes.

> ### THE OWNERSHIP FORK. Attribute before you assume the owner.
>
> **The received disposition is "Language/elaborator track, no TCB contact."
> That is a conclusion about the owner drawn before the diverging frame was
> located.** Two of the three candidate sites above are in `ken-kernel`.
>
> ⇒ **If the divergence is in `check_positivity` or the constructor-universe
> check, this is kernel code and the owner and the TCB question both change.**
> Establish which frame actually spins — a stack sample or an instrumented
> unfold counter — **before** deciding whose repair it is. Report the answer
> either way; it is a deliverable, not a preliminary.
>
> This node exists because `D0` on the predecessor recorded a conclusion whose
> stated method could not reach it. **The same shape is available here**: "the
> elaborator diverges" is a true statement about an observed symptom and not yet
> a statement about which component contains the loop.

## Deliverables

**`D1` — locate the diverging frame and name the owner.** Which component, which
function, what is unbounded. The reproduction exists; this is instrumentation,
not re-derivation. **A stack sample under an external bound is sufficient and is
the cheap instrument** — QA already showed the row reaches stack overflow at the
default stack and allocator failure under a 2.5 GiB cap.

**`D2` — the repair, in the component `D1` named.** The expected shape is a
stuck-recursion guard on whatever reduces the constructor premise. **If the
repair lands in `ken-kernel`, stop and hand back before editing** — that routes
to the Architect and the operator, not to this ring's discretion.

**`D3` — the capability, demonstrated on the shape that motivated it.** A
`data` declaration with a constructor carrying `Equal (Option FokForm)
(fok_nth_form gamma left) (Some FokForm g)` elaborates and kernel-checks.
**That is `FokDerivInit`'s actual premise**, not an analogue.

## Acceptance criteria

**`AC-1`. The minimal repro is un-`#[ignore]`d and runs in ordinary CI.** The
two hazardous rows in `v3_fo_checker_soundness_d1a_elaborator_divergence_finding.rs`
are ignored today with the resource hazard in the reason string. **A repair that
leaves them ignored has not been shown to work** — the un-ignoring is the proof.

**`AC-2`. The three-cell control table still holds after the repair**, with the
diverging cell now terminating. The two control cells must not regress; they are
what attribute the defect and they are equally what would detect an
over-broad fix that disables the reduction entirely.

**`AC-3`. Zero new `trusted_base()` entries**, pinned before and after. A
liveness repair adds no trust surface. **If the fix requires one, that is the
signal it is the wrong fix.**

**`AC-4`. No weakening of strict positivity or the constructor-universe check.**
The repair must terminate the reduction, not skip the check that performs it.
**State explicitly which check still runs on the repaired path**, with a
negative control: a genuinely positivity-violating declaration must still be
rejected.

**`AC-5`. A hard stop is a complete result**, as it was at `D0` and `D1a`.

**`AC-6`. No-regression, in CI** (`COORDINATION §12`). Targeted local validation
only. **Never run the diverging repro unbounded on the shared box** — that is
what `§12` forbids and it is why the Architect declined to reproduce it.

## Banned scope

- **Editing `ken-kernel` without handing back first.** See `D2`.
- **Restructuring `FokDerivation` to avoid the shape.** `AC-2` of
  [[V3-FO-CHECKER-SOUNDNESS]] requires its premises to be the same checks
  `fok_check_rule` performs, and there is no alternative signature — unlike
  `D0`'s Bool-inversion restructuring, which had one.
- **Truncation surface syntax.** That is
  [[LANG-TRUNCATION-SURFACE-SYNTAX]], a separate and independent blocker.
- **Proving anything, emitting FO `proved`, or touching `attempt_fo`,
  `fok_check_cert`, or the Rust reference checker.**

## Sequencing

**`ready` at filing, `depends_on: []`.** The reproduction is landed and minimal.

**This and [[LANG-TRUNCATION-SURFACE-SYNTAX]] are TWO INDEPENDENT blockers on
[[V3-FO-CHECKER-SOUNDNESS]], and neither unblocks the other** — `D1a` on this,
`D1b` on truncation. **That node must not be read as one blocker away.**

**`D2` of [[V3-FO-CHECKER-SOUNDNESS]] is unaffected and remains dispatchable.**
It is ordinary `fn`/`theorem` work, which the control cells show elaborates
fine, and `D0` fixed the signature shape its lemmas must take.

**Lane 2 under the operator's 2026-08-15 directive.** It is the FO Kripke
embedding's blocker, not a third lane.

**Priority against the truncation node: this one first.** It blocks a broader
capability, its reproduction already exists, and the truncation node's premise
is settled while this one's owner is not.
