---
id: LANG-INDEX-REFINEMENT-POSITIONAL
title: "Refining one index of a constructor arm's type rewrites every equal value in the family application, so a sibling index that happens to hold the same value is captured and a well-typed match on a multi-index family is falsely rejected. Rewrite only the refined index position"
status: ready
owner: language
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary report evt_4qp92zg6ngmjb (M8 on b40f28977): a false reject that the kernel catches (fail-closed), predating LANG-GENERATED-J-PROOF-ASCRIPTION. Reachable from well-formed source (operator 2026-08-29). Steward-filed per COORDINATION section 2."
---

# Index refinement rewrites one position

## Objective

A dependent `match` on a family with several indices refines only the
index it is refining. Sibling indices are preserved even when they hold
an equal value.

## Settled inputs (read at `b40f28977`)

- **The replacement is by value, not by position.** `build_index_type_cong`
  (`elab.rs:6815`) computes `subst_term_generalize(cur_ty, old_idx,
  new_idx)` over the whole family application (`:6825`, and the motive at
  `:6828`). `build_index_omega_transport` does the same (`:6877`), and
  `try_reindex_cast` pre-tests the same way (`:6916`).
- **Callers.** `try_reindex_cast` (`:6937`) and
  `classify_branch_goal_restoration` (`:4795`).
- **The positional precedent.** `build_result_index_type_cong` (`:3743`)
  already rewrites exactly one index argument (doc comment at `:3739`).
- **Repro.** A family `Mat (a : Type) : Nat → Nat → Type` with
  `MNil : Mat a Zero Zero`. Its identity match
  `MCons r1 k1 x t ↦ MCons a r1 k1 x t` fails with `KernelRejected
  TypeMismatch`: expected `Mat a Zero @3`, found `Mat a @3 @3`. With
  `MNil : Mat a Zero (Suc Zero)` it passes.
- **Population.** No test, catalog, library, example or conformance source
  declares a two-index family today.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The three sites rewrite only the refined index position, sharing one
positional helper with `build_result_index_type_cong`. There is no
second rewrite rule beside it.

## Acceptance

- **AC-1.**
  - The `Mat` identity match checks with `MNil` at `Zero Zero`.
  - It also checks at `(Suc Zero) Zero`, where the refined value occurs
    inside the sibling's value.
- **AC-2 (controls).**
  - The non-overlapping `Mat a Zero (Suc Zero)` form still checks.
  - A genuinely ill-typed arm on the same family is still rejected.
  - Reverting the helper at `:6825` alone reddens the overlap row.
  - The 61-file catalog census shows no verdict change.

## Stop conditions

- Any kernel or trust change.
- A caller that relies on the by-value rewrite of a sibling: stop to the
  Architect with the site.
