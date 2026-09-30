---
id: LANG-GENERATED-J-PROOF-ASCRIPTION
title: "An elaborator-built J carries its equality proof bare, so once the index premise is substituted by refl the kernel must infer a bare Refl and cannot; ascribe every generated J proof argument in the three index builders so generated terms stay inferable under substitution"
status: merged
owner: language
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect AC-0 disposition evt_6e58trrr1bnfx on LANG-REFINED-SIBLING-MATCH-TAIL (split into three mechanisms; this is component 1, mechanism measured). Inherits the operator's 2026-09-30 scheduling of LANG-REFINED ('concur with recs'): next on the Language ring. Steward-filed per COORDINATION section 2."
---

# Ascribe the generated J proof argument

## Objective

A generated `J` stays inferable after its index premise is substituted, so
`zip_with_vcons` closes at an open index.

## Settled inputs (Architect `evt_6e58trrr1bnfx`)

- `build_index_type_cong` (`elab.rs:6796`), `build_sym` and
  `build_index_omega_transport` build a `J` whose proof `h` is bare.
- At an instance the premise becomes `refl n`. `J(motive, base, refl n)`
  sits inside a witness the kernel re-checks, and `infer_j` cannot infer a
  bare `Refl`.
- e2 passes only because its cast is at the head, where whnf's
  `cast_reduce` erases it. In e5 the cast is under the neutral `g n (...)`
  and survives.
- The witnesses (e1-e6, f1-f8) are in
  `LANG-REFINED-SIBLING-MATCH-TAIL.md` and the Architect's rows
  `evt_3tsanywqzghge`.

## Deliverable

Every generated `J` proof argument in the three builders is
`Ascript(h, Eq idx_ty old_idx new_idx)`. Elaborator only.

## Acceptance

- **AC-1.**
  - e5, e1 (`zip_with_vcons`, generic) and e4 check.
  - e2, e3, e6, f1-f3 and f8 are unchanged.
- **AC-2 (controls).**
  - Removing the ascription in `build_index_type_cong` alone makes e5
    re-fail with the same intro-form error.
  - The f7, f4 and f5 verdicts are recorded before and after. A changed
    failure is expected, because they belong to
    `LANG-SIBLING-GOAL-REFINEMENT`; record the new text.
  - The catalog census shows no verdict change, or each change is named.

## Stop conditions

- Any kernel change. The kernel `infer_j` stability question is a separate
  carry.
- Any trust change, or any change to the landed `zip_with`.

## Closeout

Merged as `b40f28977` (PR #4395).
- **AC-1.** All four generated-J builders ascribe their equality argument
  as `Eq index_ty old_index new_index`: `build_index_type_cong`,
  `build_sym`, `build_index_omega_transport` and
  `build_result_index_type_cong`. The fourth came from the Architect's
  review of `e2755b545`. Fixtures `e5`, `e1` (`zip_with_vcons`, generic)
  and `e4` check in `lang_generated_j_proof_ascription.rs`.
- **AC-2.** Removing the `build_index_type_cong` ascription alone makes
  `e5` fail again. The 61-file catalog census is byte-identical, and the
  targeted tests pass 51/51.
- **Carried.** `f6` belongs to `LANG-SIBLING-GOAL-REFINEMENT`. The
  fourth site shows use, not necessity. The rustfmt drift in `elab.rs` is
  inherited from main (141 hunks both before and after).
