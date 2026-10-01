---
id: KERNEL-OBS-TYPE-EQ-STRUCTURAL
title: "Since the P0 fix, Eq Type between two same-former compound types (Π/Π, Σ/Σ, D/D, Quot/Quot) stays neutral, so every cast at a compound former projects witnesses (e.1, e.2, index equalities) that have no type, and the reducer's Phase 3 sub-cast synthesizes an ill-typed Refl. Add spec 16 §2.2's structural Eq Type arms, with Refl checked by conversion, behind one TCB Decision"
status: merged
owner: kernel
size: M
tier: T1
gate: architect
depends_on: [KERNEL-OBS-EQ-AT-TYPE-RIGID-BOTTOM, KERNEL-OBS-REDUCT-WITNESS-TYPING]
blocks: [LANG-SIBLING-GOAL-REFINEMENT]
github: null
origin: "Architect recuts evt_229qe9tgfetw1 (site 3 and the Class A fixtures) and evt_4sj0kg0kd2qbz (§1b entry 2 of KERNEL-OBS-REDUCT-WITNESS-TYPING: one defect across every compound-former cast). Kernel TCB change, approved by the operator 2026-10-01. Steward-filed per COORDINATION section 2."
---

# Eq Type decomposes structurally

## Objective

`Eq (Type l) A B` between two rigid heads of the same former reduces
structurally, as in spec 16 §2.2. Every cast at a compound former then
takes its sub-evidence from `e` and produces a reduct that typechecks.

## Settled inputs (Architect `evt_4sj0kg0kd2qbz`, read at `65ab19be8`)

- **One defect, four consumers.** Every cast at a compound former consumes
  a decomposition of `e : Eq Type A B`:
  - `cast_at_pi` and `cast_at_sigma` project `e.1` and `e.2`;
  - `cast_at_quot` projects `e.1`;
  - `cast_at_inductive` Phase 3 (`obs.rs:614`) needs the index equalities.
    Today it discards `e` (`:450`) and emits
    `Cast(a_ty_j, b_ty_j, Refl(a_ty_j), v)`.
- **Why they are untyped.** After P0, `eq_at_type` leaves every
  same-former compound pair neutral, so no projection of `e` has a type.
  The enabler `e.1 : Eq Type A1 A2` needs
  `whnf(Eq Type ((x:A1)→B1) ((x:A2)→B2))` to be a Σ.
- **Subject reduction, not soundness.** Conversion never inspects Ω
  proofs, and the reducts' computational content comes from the endpoints.
- **The `Refl` rule must move with the arms.** `check`'s whnf-first `Refl`
  (`check.rs:483`) would reject `refl` at every compound type equality
  once it reduces to a conjunction. That includes the elaborator's `J`
  bases, which `refl_base_arg` rescues only in the single-conjunct case.
  The declarative rule of spec 15 §2 is `refl a : T` iff
  `T ≡ Eq (infer a) a a`. It is sound because P0 removed the
  reflexive-false arm from `eq_reduce`.
- **Consumers.** The five `LANG-SIBLING-GOAL-REFINEMENT` Class A rows (f7
  at contexts 12 and 13, f4 at context 19), and the Π-motive
  `j_dependent_motive_fires` row moved here from
  `KERNEL-OBS-REDUCT-WITNESS-TYPING`.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

Under one TCB Decision:

1. `eq_at_type` structural arms per spec 16 §2.2 for Π/Π, Σ/Σ, D/D with
   the same inductive id, and Quot/Quot, keyed on the P0 rigid-former
   classifier (`rigid_type_former`).
2. `Refl` checked by conversion.
3. `cast_at_pi`, `cast_at_sigma`, `cast_at_quot` and `cast_at_inductive`
   consume the now-typed projections. Site 3 derives each sub-cast witness
   from `e`.
4. The five Class A fixtures and the Π-motive row are committed.

## Acceptance

The operator authorized the TCB change (2026-10-01). The Architect sets AC-0
at kickoff, before any edit. It includes the Π-motive row and one typed-whnf
subject-reduction row per former.

- **AC-0: ruled** (`evt_71pe5ka6mwg8r`, base `a02cecfed`). §1a count 0.
- **Re-pin after CI red on `16ecf06b2`** (Architect `evt_39snx0g9hygtg`).
  Refl now fails at conversion as `TypeMismatch` (spec 18), which is lawful.
  The scope adds exactly three paths:
  - `crates/ken-elaborator/tests/ds6a_int_deceq_acceptance.rs`: pin the
    payload, and add the accepting `Refl x : x = x` twin.
  - `crates/ken-elaborator/tests/sec4_acceptance.rs`: pin the payload and
    update the doc comments.
  - The `abstract-distinct-index-certificate-rejected` row of
    `conformance/security/trust-model/seed-trust-model.md`. This takes the
    Spec vote (conformance-validator) on the exact SHA.

## Stop conditions

- Any trusted-base change beyond the four deliverables: stop to the
  Architect.

## Closeout

Merged `884f493fe` (PR #4427), exact `668e1304b`: Kernel QA
`evt_20jb7d9qv120w`, Architect `evt_1ybskzzw7fgt0`, conformance-validator
Spec vote `evt_7pkc0k1s7b24a`, Decision `dec_7f4v538pk5wnz`. The TCB change
was approved by the operator on 2026-10-01; `trusted_base()` is unchanged.

- `Eq Type` reduces structurally on Π, Σ, Quot and same-id inductive
  families. `Refl` checks by conversion, and every new failure path stays
  neutral.
- `type_eq_sym` and `cast_at_pi` build their witnesses from checked
  evidence, and phase 3 checks the chained index witness before using it.
- Refl now fails at conversion with `TypeMismatch`. The ds6a and sec4 rows
  and the conformance row `abstract-distinct-index-certificate-rejected` pin
  that payload (`evt_39snx0g9hygtg`), after CI reddened on `16ecf06b2`.
- Cost: nested-Π conversion grows quadratically in Π depth, accepted with a
  recorded bound.
