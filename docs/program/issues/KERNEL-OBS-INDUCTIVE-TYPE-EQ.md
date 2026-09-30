---
id: KERNEL-OBS-INDUCTIVE-TYPE-EQ
title: "Eq Type between two applications of the same inductive family stays neutral, so the reducer's Phase 3 sub-cast has no evidence to decompose and synthesizes an ill-typed Refl. Add spec 16 §2.2's structural inductive arm to eq_at_type, together with Refl checked by conversion, behind one TCB Decision"
status: draft
owner: kernel
size: M
tier: T1
gate: architect
depends_on: [KERNEL-OBS-EQ-AT-TYPE-RIGID-BOTTOM, KERNEL-OBS-REDUCT-WITNESS-TYPING]
blocks: [LANG-SIBLING-GOAL-REFINEMENT]
github: null
origin: "Architect recut evt_229qe9tgfetw1 of KERNEL-OBS-REDUCT-WITNESS-TYPING: site 3 (cast_at_inductive Phase 3) and the five LANG-SIBLING-GOAL-REFINEMENT Class A fixtures. Kernel TCB change: held at draft for the operator. Steward-filed per COORDINATION section 2."
---

# Inductive type equality decomposes

## Objective

`Eq Type (D p̄ ī) (D p̄ j̄)` reduces structurally, as in spec 16 §2.2. A Phase 3
sub-cast in `cast_at_inductive` can then take its index evidence from `e`,
as in §3.2, instead of synthesizing `Refl`.

## Settled inputs (Architect `evt_229qe9tgfetw1`, read at `aa51bf9d7`)

- **No evidence source today.** `e : Eq Type (D p̄ ī) (D p̄ j̄)` stays
  neutral, because `eq_at_type` leaves `(App, App)` neutral. Phase 3
  (`obs.rs:614`) discards `e` (`:450`) and emits
  `Cast(a_ty_j, b_ty_j, Refl(a_ty_j), v)`.
- **Two TCB changes that must land together.**
  - First, the §2.2 inductive arm of `eq_at_type`. It is keyed on rigid
    heads, as in the P0 fix.
  - Second, `Refl` checked by conversion: `refl a : T` iff
    `Eq (infer a) a a ≡ T`, the declarative rule of spec 15 §2.
  - The second is needed because `check`'s whnf-first `Refl`
    (`check.rs:483`) would otherwise reject `refl` at every inductive type
    equality once it reduces to a conjunction. That includes the
    elaborator's `J` bases, which `refl_base_arg` rescues only in the
    single-conjunct case.
  - Checking by conversion is sound only once `eq_reduce` is sound. That is
    the P0 fix.
- **Consumers.** The five `LANG-SIBLING-GOAL-REFINEMENT` Class A rows:
  f7 at contexts 12 and 13, and f4 at context 19.

## Deliverable

- The inductive arm and `Refl` by conversion, together, under one TCB
  Decision.
- Phase 3 derives each sub-cast witness from `e`.
- The five Class A fixtures are committed.

## Acceptance

AC-0 is set by the Architect when the operator authorizes the TCB change.

## Stop conditions

- The operator does not authorize the TCB change: the node stays draft.
