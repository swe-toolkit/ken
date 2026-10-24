---
id: LANG-SOLVED-INDEX-ELIMINATE-ALWAYS
title: "Dependent-branch regimes are chosen by trial: a variable-side method is checked retained first and eliminated after a rollback, so a refused nested peel runs the arm body three times per level (3279 entries and 4.7 s at d=7). Eliminate every variable-side method with no trial, after fixing the two eliminated-path defects the trial masks"
status: ready
owner: language
size: L
tier: T1
gate: architect
depends_on: [LANG-SOLVED-INDEX-EQUATION-UNIFIER]
blocks: []
github: null
origin: "Cut (ii) of the Architect's AC-0 for LANG-SOLVED-INDEX-EQUATION-UNIFIER (evt_42gkba2758n1v), Steward scope call evt_2tsvy1y6tsfqj. Carries the retry cost of the closed LANG-DEPENDENT-BRANCH-RETRY-NESTED-COST (evt_1xjmh1w0g8nxa; Adversary evt_1beqd0ntawbqc). Steward-filed per COORDINATION section 2."
---

# One regime, no trial

## Objective

A variable-side method is eliminated directly, with no retained-first
attempt and no rollback. A refused nested peel costs one arm-body entry
per level.

## Settled inputs (Architect `evt_42gkba2758n1v`, at `d366662a6`)

- **The design, 2(d).** Delete `check_dependent_branch_regimes` and
  `MethodRegime`. `check_dependent_branch_body` uses `variable_side` as
  computed. Eliminate-always is the standard unifier (Research
  `evt_34qvnmp4g573c`). A selector keyed on what the body uses was
  measured and refuted: `lang_solved_index_substitution` `sec` turns red.
- **Measured cost.** deepD with a false Refl leaf, d = 1..7:
  - base: 3, 12, 39, 120, 363, 1092, 3279 body entries;
  - no trial: 1 to 7 entries, 20 ms at d=7, with the diagnostic
    unchanged. Accepted twins take d entries;
  - the refined fallback F alone, with every method forced retained:
    2^d - 1.
- **Two defects the trial masks**, red under 2(d), with no ruled fix:
  - **D-1, class D** (the embedded-eliminator planner,
    `plan_embedded_method_convoy` and `plan_index_equation_elim`). Six
    rows in three binaries, among them `lang_embedded_context_convoy`
    `r1_two_lists_at_suc_index` and `lang_branch_goal_budget_visit_charge`.
    The error is TypeMismatch `Dg648 @3` against `Dg648 (cg72 @6)`.
    Three closures were probed and refuted.
  - **D-2, NatMatrix** (`matrix_vector_distrib`,
    `catalog/packages/Algorithm/LinearAlgebra/NatMatrix.ken.md`:558).
    Eleven rows in four binaries. It is rejected only at declaration,
    because the eliminated arm in `complete_dependent_branch_body` has no
    `kernel_check_current`. It reproduces on base with only the no-trial
    switch. Its first step is ruled: give the eliminated arm the same
    arm-level kernel check as the unrefined path.
- Evidence: `/workspaces/ken/tmp/arch-unify/logs`, and the probe test
  `tests/arch_unify_probe.rs` in that tree.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

2(d), plus fixes for D-1 and D-2, in one landing unit or in
straight-ancestor increments the Architect cuts.

## Acceptance

- **AC-0a and AC-0b (D0s, Architect).** Rule D-1 and D-2 before any code.
- **AC-1.** With 2(d), the D-1 and D-2 rows are green, and there are
  zero main-green reds across every ken-elaborator integration binary,
  the catalog loaders and the mod-strict census.
- **AC-2.** A test pins deepD at body entries <= d for d = 3, 5 and 7,
  for the refusal and for the accepted twins, with the diagnostic
  unchanged. Time is measured locally and is not asserted in CI.
- **AC-3.** A deep-nest row whose leaves are all parameter-side, so the
  unrefined path and the refined fallback still run, measures F's entry
  count. If it grows as 2^d, F needs its own no-trial rule in this WP.
- **AC-4 (mutation, QA).** Restoring the retained-first trial fails
  AC-2's count.

## Stop conditions

- Any kernel or `trusted_base()` change.
- An accepted verdict changes outside the D-1 and D-2 rows.

Checks: 8 fired: a 2^d configuration passes "not 3x per level", so
AC-2 pins entries <= d. 9 fired: F is unmeasured on non-variable-side
nests, so AC-3 measures it. 7 fired: the body-use selector was re-run
against `sec` and fails it.
