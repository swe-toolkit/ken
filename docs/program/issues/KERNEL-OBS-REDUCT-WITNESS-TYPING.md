---
id: KERNEL-OBS-REDUCT-WITNESS-TYPING
title: "The observational reducer synthesizes Refl as the proof witness in four reducts where the stated Eq relates two different types, so each reduct fails to typecheck (subject reduction fails) and a J over such a reduct goes stuck. Derive each witness from the reduction's own evidence, as spec 16 §3.2 and §4.1 specify"
status: ready
owner: kernel
size: M
tier: T1
gate: architect
depends_on: []
blocks: [KERNEL-OBS-NESTED-CAST-LINEAR]
github: null
origin: "Architect stop ruling evt_449gyxrrejte1 on LANG-SIBLING-GOAL-REFINEMENT Class A: a kernel reducer defect, not an elaborator one. Trust-root work. Steward-filed per COORDINATION section 2."
---

# Well-typed witnesses in observational reducts

## Objective

Every reduct the observational reducer produces typechecks at the type of
its redex, with each synthesized proof witness derived from the reduction's
own evidence.

## Settled inputs (Architect `evt_449gyxrrejte1`, read at `3cdf24654`)

- **The defect.** `cast_at_inductive` Phase 3 (`obs.rs:604-615`) builds
  each dependent sub-cast as `Cast(a_ty_j, b_ty_j, Refl(a_ty_j), v)` with
  `a_ty_j ≢ b_ty_j`, and discards `e` (`let _ = e;`, `:450`). `check.rs:331`
  types a `Cast` by checking `e : Eq Type A B`, so the reduct fails
  `infer`.
- **The spec requires the well-typed form.** `16-observational.md §3.2`
  gives the sub-cast evidence as `cong (Vec A) eq'`, where `eq'` decomposes
  `e`'s index equality injectively. §4.1 says the kernel synthesizes
  `pair-eq` by the singleton schema.
- **Four sites, one predicate:** a reducer-synthesized witness is ill-typed
  at the `Eq` it stands for.
  1. `eq_at_sigma` `:153`: `Refl(b1_p1)` for `Eq Type (B1 p.1) (B1 q.1)`.
  2. The `Eq`-at-inductive dependent-telescope conjunct `:299`:
     `Refl(b_ty_j)` for `Eq Type a_ty_j b_ty_j`.
  3. `cast_at_inductive` Phase 3 `:614`.
  4. `j_reduce` `:697`: `pair_eq = Refl(P a refl)` for
     `Eq Type (P a refl) (P b e)`.
- **Reach.**
  - Conversion never inspects Ω proofs, so this is a completeness defect,
    not a soundness hole.
  - Every consumer that re-types a reduct rejects it. That includes
    `j_reduce` itself, which runs `check::infer` on its `eq` (`:686`,
    `.ok()?`), so such a `J` goes stuck instead of computing.
  - The five `LANG-SIBLING-GOAL-REFINEMENT` Class A rows (f7 at contexts 12
    and 13, f4 at context 19) are rejected this way.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

All four sites build their witness from the reduction's own `e`: the
decomposition projections, then `cong` via `J` along `eq'`, accumulated
along the telescope per §3.2, or §4.1's singleton schema for `pair-eq`.

## Acceptance

- **AC-0 (no fix).**
  - For each site, a subject-reduction row: a well-typed redex whose
    reduct fails `infer` today.
  - Name the consumer that re-types the reduct on the five Class A rows.
  - Measure the witness-construction cost on the nested-`Cast` shape,
    because the reducer is hot.
  - The Kernel leader asks the Spec leader to confirm that the four witness
    schemas are the normative ones. The Architect then rules the repair.
- **AC-1.**
  - `infer(reduct)` is convertible to `infer(redex)` at each site.
  - The five Class A kernel pairs are committed as fixtures.
  - A `j_reduce` row whose proof carries a Phase 3 reduct now computes.
- **AC-2 (controls).**
  - Reverting any one site reddens its subject-reduction row.
  - The 57-package census shows no conversion verdict change.
  - `trusted_base()` is unchanged.

## Stop conditions

- A kernel-internal "irrelevant witness" term former typed by its stated
  `Eq`. It would assert an equality without evidence, which is an axiom in
  the TCB. An operator question.
- Any change to what the kernel accepts, beyond reducts that now compute:
  stop to the Architect.
- A spec change: an operator question.
