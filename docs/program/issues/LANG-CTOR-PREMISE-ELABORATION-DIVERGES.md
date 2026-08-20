---
id: LANG-CTOR-PREMISE-ELABORATION-DIVERGES
title: "A data constructor whose premise applies a recursive function to a telescope-bound variable diverges during elaboration -- proof-carrying inductive families are unavailable in Ken"
status: active
owner: kernel
size: L
gate: none
depends_on: []
blocks: [V3-FO-CHECKER-SOUNDNESS]
github: null
origin: "Steward, 2026-08-16, on the merge of V3-FO-CHECKER-SOUNDNESS D1a (PR #2428). D1a hard-stopped with a bounded finding: FokDerivation cannot be authored because every constructor it needs diverges. Architect dec_47g7jtcrb5rhv records the architectural reading and the liveness-not-soundness severity. Steward-filed per COORDINATION section 2. Filed owner:language on the received disposition; changed to owner:kernel 2026-08-16 when the frame was located at ken-kernel/src/inductive.rs:97 (adversary evt_30vg7q07ypktj, coordinates Steward-verified). The LANG- id prefix is historical."
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

**Severity is liveness, not soundness** (Architect, `dec_47g7jtcrb5rhv`), and
**that half of the disposition survived the frame being located.** A
non-terminating positivity check admits nothing — it hangs. No wrong term is
emitted and no declaration is wrongly accepted.

⇒ **This is kernel-owned and TCB-resident, and it is still not a soundness
escalation.** Those are three separate facts and the second does not imply the
third. Route it as a liveness defect in the admission gate.

## The frame, located — and the ownership fork closed against the filing

**The ownership fork this node was filed to resolve is answered, and it inverts
the received disposition.** Adversary, `evt_30vg7q07ypktj`, hunting
`c67201f2f..8d6d7d545`. **Every coordinate below was re-verified by the Steward
against the tree at `f97ffb87a`** — see the epistemic status at the end of this
section for what is measured and what is not.

**The path, entry to suspect frame, each link confirmed:**

| site | what it does |
|---|---|
| `ken-elaborator/src/data.rs:61` | `elab_data_decl` calls `declare_inductive` — the surface `data` entry |
| `ken-kernel/src/check.rs:921` | `declare_inductive` → `check_positivity` |
| `ken-kernel/src/inductive.rs:396` | `check_positivity` → `check_positivity_inner` (`:404`) |
| `ken-kernel/src/inductive.rs:442` | **per constructor argument, UNGATED:** `check_pos_arg(env, d, Pol::Plus, a, ..)` |
| `ken-kernel/src/inductive.rs:97` | **`let normalized = normalize(env, &Context::new(), a);`** — a FULL normal form, before anything is inspected |

**`check_pos_arg_normalized` recurses back into `check_pos_arg`** at `:110`,
`:111`, `:114`, `:115`, `:118` and `:149` — **so it re-normalizes at every
node**, not once at the root.

**`normalize` is not `whnf`.** `ken-kernel/src/conv.rs:225` is whnf-then-
structural: it recurses under `Pi`, `Lam`, `Sigma`, `Pair`, `App`, `Proj`,
`Elim`, materializing the whole normal form.

### It explains all five isolating facts

| measured observation | explanation |
|---|---|
| `fn`/`theorem` parameter is instant | Pi-binder checking goes through whnf-based `classify`/`convert` and **never calls `check_pos_arg`** |
| the `data` constructor telescope diverges | **only that path runs positivity** |
| **the non-indexed, non-self-referential minimal repro diverges identically** | **`normalize` runs BEFORE any occurrence test** — there is no `occurs(d, a)` gate at `:442` |
| recursive fn required, non-recursive fine | fully normalizing a **stuck recursor** is what fails to terminate; a non-recursive definition unfolds once |
| a 1 GiB big stack does not help | **heap** growth building an ever-larger normal form, not stack depth |

⇒ **The third row is the diagnostic one, and it is the row `D1a`'s bisection
went out of its way to establish.** Excluding indexing and self-reference is
what *points at* this frame: positivity normalizes first and looks for
occurrences second, so a declaration with nothing to find still pays the full
normalization.

**The contrast is visible inside the same function.** `check_positivity_inner`
tests the index telescope and the constructor target indices with a bare
`occurs(d, ix)` and **no normalization at all** (`:435`, `:449`). Only the
constructor-argument loop normalizes.

### The disposition inverts on two of three points

| received | status |
|---|---|
| **owner: Language/elaborator track** | **REFUTED — `check_pos_arg` is `ken-kernel/src/inductive.rs`** |
| **"no TCB contact"** | **REFUTED — strict positivity is the admission gate for inductive declarations** |
| **"liveness, not soundness"** (Architect, `dec_47g7jtcrb5rhv`) | **HOLDS.** A non-terminating positivity check admits nothing; it hangs |

**The node id's `LANG-` prefix is historical**, from the filing that assumed the
elaborator. `owner:` is authoritative; the id is not being churned mid-flight.

### A second, independent observation at the same line

`normalize(env, &Context::new(), a)` normalizes under an **empty context**,
while `a` is a constructor-telescope-bound type whose free de Bruijn indices
refer to earlier telescope entries. **That is an open term normalized under an
empty context.** Whether it contributes to the divergence or merely renders
those variables neutral, it is a mismatch `D1` names separately.

### Epistemic status — read this before treating the frame as settled

**The coordinates are verified; the causal attribution is not.** The Steward
re-read every line cited above and the call chain is exactly as stated. **What
nobody has run is the diverging case with the frame instrumented** — the
Adversary explicitly did not reproduce it (`COORDINATION §12`), and neither did
the Steward.

⇒ **This is a very strong read, not a measurement.** It is recorded here so
`D1` confirms a named frame instead of re-searching for one — which is worth a
turn — and `D1` is not discharged by citing it.

## Deliverables

**`D1` — CONFIRM the located frame. Do not re-search for it.** Instrument entry
to `inductive.rs:97` with a counter and re-run the diverging case under the
external bound QA already used. **If entry count and RSS climb together, the
frame is located.** That is one counter and a run that is already tooled — QA
showed the row reaches stack overflow at the default stack and allocator failure
under a 2.5 GiB cap.

**`D1` is not discharged by citing the section above.** It carries three
outputs: the confirmation (or its refutation, which is equally a result), the
disposition of the empty-context mismatch, and **the owner, stated as a
measured fact rather than an inherited one.**

**`D2` — the repair, in the component `D1` confirms.**

> ### `D2` IS EXPECTED TO BE A KERNEL EDIT. IT IS NOT THIS RING'S TO MAKE.
>
> On the located frame, the repair is in `ken-kernel/src/inductive.rs`. **Stop
> and hand back before editing it.** That routes to the Architect and the
> operator — see Sequencing.

**The repair direction the frame suggests:** positivity asks a **syntactic**
question — does `d` occur, and at what polarity. **Full normalization is far
stronger than that needs.** whnf-on-demand while walking would answer it without
ever materializing a normal form, and the same function already answers the
index-telescope half of the question with a bare `occurs` and no reduction at
all. **This is a direction to attack, not a design ruling** — the Architect owns
the call once `D1` confirms the frame.

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

**`AC-4`. No weakening of strict positivity.** The repair must terminate the
reduction, not skip the check that performs it. **State explicitly which check
still runs on the repaired path**, with **two** negative controls:

1. **A plainly negative occurrence** — `D` to the left of an arrow in a
   constructor argument — still yields `PositivityViolation`.
2. **An occurrence that is only visible AFTER reduction** — `D` appearing in a
   constructor argument written through a definition that must be unfolded to
   see it — **still yields `PositivityViolation`.**

**The second control is the one that matters and the one an over-broad fix
breaks.** Reduction is at `:97` for a reason: a syntactic scan of the unreduced
term would miss an occurrence hidden behind a definition, and that is a
soundness hole, not a liveness one. **A repair that deletes the normalization
outright passes `AC-1` and `AC-2` and fails here.** Whatever replaces it must
still reduce enough to expose hidden occurrences — which is why the direction
under `D2` is whnf-on-demand while walking, not no-reduction.

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

**Priority against the truncation node: this one first.** It blocks a broader
capability and its reproduction already exists.

> ### THE RING QUESTION IS THE OPERATOR'S, AND IT IS OPEN. Do not dispatch `D2`.
>
> **This is lane 2's blocker by objective and kernel-owned by location, and the
> kernel ring is not one of the two lanes** (operator, 2026-08-15: no third lane
> gets a ring, however well-framed and however idle the team). **The Steward
> does not resolve that against a standing directive.** Briefed to the operator
> 2026-08-16.
>
> **`D1` does not wait on the answer.** It is instrumentation of a landed
> reproduction with no kernel edit, it fits inside the language ring, and it is
> what makes the operator's decision an informed one. **`D2` is the edit and
> `D2` is what is gated.**
>
> ⇒ **Dispatching `D1` is not a decision that the kernel ring is open.** Keep
> the two apart; conflating them is how a directive gets defeated by a chain of
> individually reasonable steps.
