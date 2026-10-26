---
id: LANG-SOLVED-INDEX-NONLINEAR-RESIDUAL
title: "Since LANG-SOLVED-INDEX-EQUATION-UNIFIER (b7e85023e), a match on a family whose constructor repeats a field across index leaves, such as MkTwo : (u : Nat) -> Two u u against z : Two x y, fails with ElabError::Internal(\"solved premise does not reassemble\"). The parent accepts it. Restore the parent's verdict on these programs through the unifier's rules, never an internal error"
status: ready
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-SOLVED-INDEX-EQUATION-UNIFIER]
blocks: []
github: null
origin: "Adversary finding on LANG-SOLVED-INDEX-EQUATION-UNIFIER squash b7e85023e (evt_431nmj385s97y): a correctness regression, 15 of 15 probe rows OK on the parent and Internal on the merge. It violates that WP's stop condition that no accepted verdict changes outside the D-1 and D-2 rows. Serves the L2 objective. Steward-filed per COORDINATION section 2."
---

# A non-linear index residual is handled by the unifier

## Objective

Every program the parent of `b7e85023e` accepts in the reported families
is accepted again. No index shape reaches an `ElabError::Internal` from
the solved-premise path.

## Settled inputs (main `b7e85023e`, Adversary `evt_431nmj385s97y`)

- **The row.** With `data Two : Nat -> Nat -> Type where { MkTwo :
  (u : Nat) -> Two u u }`, the program
  `fn p (x : Nat) (y : Nat) (z : Two x y) : Nat = match z { MkTwo u ↦ u }`
  is OK on `b7e85023e^` and Internal on `b7e85023e`.
- **The population.** 15 rows in five families: `Two u u`,
  `TwoS u (Suc u)`, `Diag (Pt u u)`, `SucD (Pt u (Suc u))` and
  `Tri (Pt u Zero) u`. The bodies are a plain `u`, a continuation at goal
  `P x`, and an `Equal Nat x y` or `Equal Nat (Suc x) y` theorem by
  `Refl`.
- **The path.**
  - `method_has_variable_side_leaves` (`elab.rs:9297`) admits these
    methods now that gate (a) is gone, so they eliminate in
    `prepare_dependent_branch_body` (`:9448`).
  - Solving `u := x` leaves the residual `Eq Nat x y` (or `Suc x = y`).
    `reassemble_solved_premise` (`:11362`) returns None for it, because
    the original leaf mentions `u`.
  - `refine_branch_goal` then raises Internal (`:11584`). The deleted
    retained-first trial used to accept these on the alias path.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

Before the implementer starts, the Architect rules how the unifier
disposes of a residual non-reflexive equation between outer variables:
solve it, keep it as a retained premise, or route the method back to the
alias path. Then the repair, with the 15 rows pinned in
`elab/solved_index_equation_unifier_tests.rs`.

## Acceptance

- **AC-0 (Architect).** The ruled disposition, on the landed base.
- **AC-1.** All 15 rows elaborate as on the parent, and the `Refl`
  theorem rows check. `trusted_base()` is unchanged.
- **AC-2 (control).** The unifier WP's D-1 and D-2 rows and its
  depth-seven and ParamNest rows are unchanged. CI is green.
- **AC-3 (mutation, QA).** Restoring the None arm for a residual that
  mentions the eliminated variable makes the `Two u u` row Internal
  again.

## Stop conditions

- The disposition changes an accepted verdict outside these families.
- Any kernel or trust change.
