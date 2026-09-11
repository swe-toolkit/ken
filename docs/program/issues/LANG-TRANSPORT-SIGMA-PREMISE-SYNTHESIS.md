---
id: LANG-TRANSPORT-SIGMA-PREMISE-SYNTHESIS
title: "transport_recursive_group_call_result over-rejects a compound result indexed by BOTH the whole matched record AND a bare component of it: it builds a single raw Eq over the user data record, which obs::eq_reduce will not decompose into per-index Sigma leaves, so the component index goes unrefined and a well-typed program is KernelRejected (TypeMismatch). Fix = route (a): synthesize transport's premise as the Sigma-shaped result-family-index equality and feed the existing walker (REUSE). DEFERRED completeness gap."
status: draft
owner: language
size: S
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Carved by the Steward 2026-09-11 from the discharged confirm-or-refute of [[LANG-RESULT-TRANSPORT-SIGMA-SWEEP]] (closed). Architect design ruling evt_79xy16y7fv5sx (re-grounded at b148a96b): the sweep's confirm-or-refute is DISCHARGED — the compound case is REACHABLE and the framed walker-application fix is REFUTED with grounds — and the correct fix (route (a)) is a real bounded refactor of transport's premise construction that EXCEEDS the confirm-or-refute the sweep was sized for, so it is a separate node (Architect ground iii). Architect RECOMMENDED deferral (completeness-only, kernel-backstopped false-reject = safe; trigger narrow/exotic); the Steward placed it here DEFERRED. NOT released — awaiting an operator L2 priority direction (L2's releasable queue is otherwise near-exhausted; item 8 LANG-TRUNC-INTRO-DIAGNOSTIC-REMEDIES is blocked on a deferred Architect design call). Re-measure the elab.rs anchors at release — they drift."
---

> # DEFERRED, NOT RELEASED. Completeness-only, kernel-backstopped (an over-strict
> # false-reject, never an over-accept), NOT a soundness hole, NOT a blocker. The
> # Architect recommended deferral; released only on an operator L2 priority call.
> # Re-measure ALL anchors at pickup — they drift (the coordinates below are from
> # the Architect's b148a96b re-grounding and elab.rs moves often).

## The finding (Architect ruling evt_79xy16y7fv5sx; confirm-or-refute discharged)

`transport_recursive_group_call_result` over-rejects a well-typed program whose
recursive-group-call result family is indexed by BOTH the whole matched record
AND a bare component of that record, co-occurring in one result. It refines the
whole-record index but leaves the bare-component index unrefined, and the
assembled generated-elim term is then `KernelRejected` with `TypeMismatch`.

**Direction: COMPLETENESS, not soundness.** The failure is a refusal of a good
program (over-strict); nothing ill-typed is ever accepted, and the kernel/TCB is
untouched. That is why deferral is legitimate and this node is safe to hold.

## Why the framed walker fix does not apply (the refuted hypothesis)

Every working index-refinement consumer builds its premise as a **Sigma-telescope
of per-index equalities** via `synth_generated_index_evidence` (elab.rs ~:1584)
and decomposes it with the walker `project_generated_index_equality_leaves`
(~:1672), whose Sigma case recurses through `proj1`/`proj2` into per-index leaves
(call sites ~:4497/:4810/:6352/:6436/:6568, incl. `install_index_refinements`
~:6540).

`transport` instead builds a **single raw `Eq(scrut_ty, concrete, scrut_core)`**
over the matched **user `data` record** (~:4881-4890). `obs::eq_reduce` reduces
`Eq` to a Sigma only for a structural `Term::Sigma`, **never for a user `data`
IndFormer** — so the walker receives exactly one whole-record leaf and refines
only the whole-record index. `transport` is the lone outlier consumer.

## The fix — route (a) (Architect-ruled; route (b)-standalone REJECTED)

Build `transport`'s premise as the **Sigma-shaped equality over the RESULT
FAMILY's index positions** — the same shape `install_index_refinements` already
consumes for ordinary dependent matches on multi-index families — and drive it
through `synth_generated_index_evidence` + the existing walker. This removes
`transport`'s outlier raw-record-`Eq` and lets the already-proven multi-index
Sigma-refinement machinery refine BOTH the whole-record index and the
bare-component index. **This is REUSE, not new capability** (subsume-don't-
proliferate); the multi-index refinement path exists and works today — transport
must simply feed it the right premise shape.

**FIRST STEP (grounding, before any fix):** determine whether the per-result-index
equalities — specifically the bare-component one (`Eq(Nat, local, index)` in the
upstream fixture) — are **already available** from the match's refinement
evidence, or **must be projected** from the record equality. That decides whether
route (a) is pure reuse or reuse + a scoped single-constructor injectivity
projection.

**On the injectivity content (route (b), folded not standalone):** the
bare-component equality is single-constructor CONSTRUCTOR INJECTIVITY
(`Eq(D, C a.., C b..)` -> per-field `Eq`s), which is SOUND in ITT (data
constructors are injective). If synthesizing the per-result-index equalities
requires projecting components out of the record equality, implement that
projection **SCOPED to single-constructor, non-indexed records and shaped to EMIT
the existing `IndexEqualityLeaf`** (i.e., as part of route (a)'s synthesis feeding
the walker) — **never** as a novel free-standing observational decomposition step
or cast alongside the walker (reflect-don't-extend). Standalone (b) is off the
table.

## Symptom inventory (Architect §1a/§1b; hard-stop #1, no research pull until #3)

1. transport's premise is a whole-record raw `Eq` over a user `data` IndFormer,
   not a per-index Sigma — the walker cannot decompose it into component-index
   leaves — keyed on premise SHAPE (raw data `Eq` vs synthesized Sigma).

## Acceptance (to be finalized at release)

The reaching fixture (the compound-indexed recursive-group-call result that
currently false-rejects) compiles; a control keeps the whole-record-indexed case
green; a genuine differential on the synthesized-premise/walker path (not a
vacuous green-vs-green). No kernel/TCB/spec change. Reviewers: **Architect
(required soundness reviewer for this class, as on the parent)** + Language QA;
standing Adversary hunt independent -> Steward M1-M4 -> lieutenant.
