---
id: LANG-MATCH-ARM-REFL-INDEXED-MOTIVE
title: "Refl in an arm of a match on an indexed family, with goal Equal (Pair Nat Nat) p p, is refused with 'Refl expects an Eq-shaped goal', while the same goal accepts Refl standalone and in List, Bool and let arms. stated_equality reads the goal's syntactic head, and on the indexed route the goal arrives as a motive redex. Recognise the stated equality up to that redex"
status: active
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-MATCH-ARM-REFL-PAIR-GOAL]
blocks: []
github: null
origin: "Adversary report evt_69f8mtptgk8a8 on 55a0bee93 (LANG-MATCH-ARM-REFL-PAIR-GOAL): the merged fix does not close the class its frame names on the indexed-family route. Anchors confirmed by the Steward on main. Steward-filed per COORDINATION section 2."
---

# Refl in an indexed-family arm

## Objective

`Refl` in a match arm has its standalone meaning on every match route,
including a match on an indexed family.

## Settled inputs (Adversary `evt_69f8mtptgk8a8`, at `55a0bee93`)

- **Repro.** A single-level `match i` on `i : DualFin n` (the
  `lang_index_convoy_impossible_arm_level.rs` fixture), goal
  `Equal (Pair Nat Nat) p p`, with `Refl` in both arms. It fails with
  `TypeMismatch "Refl expects an Eq-shaped goal"`. The same goal and leaves
  check standalone and in a match on `List Nat`, on `Bool`, and in a `let`
  arm.
- The same split holds for `Two`, `Pair (Pair Nat Nat) Nat` and
  `Pair Box Nat`. Non-Sigma carriers (`Box`, `Nat`, a Pi goal) pass on
  every route. The false control `mk_pair a b = mk_pair b a` on the indexed
  route is refused with the Eq-shaped reason, not as not convertible.
- **Mechanism.** In `simplify_branch_goal_unguarded` (elab.rs ~:4276-4285),
  the Sigma guard calls `stated_equality` (~:4336-4347) on the unreduced
  term. It accepts only an `Eq`, or a `Const` head under `peel_app`. On the
  indexed route the goal is an annotated motive lambda applied to (index,
  scrutinee), so the head is a lambda and the goal falls back to its
  Sigma reduct.
- It is distinct from `LANG-COHERENT-FRAME-SINGLE-CONSTRUCTOR-EQUAL`, a
  nested match that kernel-rejects at the inner `DualFS`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The stated equality is recognised when the goal is a motive redex whose
reduct before the Sigma step is `Equal` or `Eq`, not only by syntactic
head. The Architect confirms the reduction boundary at review.

## Acceptance

- **AC-1.** The four true Sigma carriers check in the `DualFin` arm, as
  they do standalone.
- **AC-2 (control).** The false `mk_pair` control on the indexed route is
  refused as not convertible. The List, Bool and let rows and the landed
  `lang_match_arm_refl_pair_goal.rs` suite are unchanged.
- **AC-3 (mutation, QA).** Restoring the syntactic-head-only guard
  reddens AC-1.

## Stop conditions

- The repair needs a kernel or conversion change, or a `trusted_base()`
  change.
- A catalog or suite delta outside these rows.
