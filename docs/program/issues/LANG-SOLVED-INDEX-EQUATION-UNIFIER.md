---
id: LANG-SOLVED-INDEX-EQUATION-UNIFIER
title: "Stage 2 of solved-index elimination: LANG-SOLVED-INDEX-SUBSTITUTION eliminates only all-variable methods whose premise slots each project to one leaf, and mixed methods and multi-leaf slots keep the alias path. Replace the per-leaf plan with a per-equation unifier (an equation telescope, injectivity decomposition, per-equation rule dispatch, dependency-ordered transport) and lift the gate"
status: active
owner: language
size: L
tier: T1
gate: architect
depends_on: [LANG-SOLVED-INDEX-SUBSTITUTION]
blocks: []
github: null
origin: "Architect staged route evt_6htgrnanmjnm4 on the §1a 6 hold of LANG-SOLVED-INDEX-SUBSTITUTION, after the Research advisory evt_34qvnmp4g573c (the standard unifier: Agda's eqTel and reorderTel, Lean's unifyEqs). Within the operator's 2026-10-17 restructuring authorization of that WP. One unit with the no-trial regime per Steward evt_2t9d4ea4vn9yq, after the Architect measured cut (i) inert with the trial kept (evt_2qef3qpjvtaf6). Absorbs LANG-SOLVED-INDEX-ELIMINATE-ALWAYS and the retry cost of LANG-DEPENDENT-BRANCH-RETRY-NESTED-COST. Steward-filed per COORDINATION section 2."
---

# Solved indices by elimination, with no regime trial

## Objective

Every variable-side method is eliminated directly, including mixed
methods and multi-leaf slots, with no retained-first attempt and no
rollback. A refused nested peel costs one arm-body entry per level.

## Settled inputs (Architect AC-0, at `d366662a6`)

- **2(a) to 2(c).** The ruled code is in `evt_42gkba2758n1v`:
  - (a) remove both gates in `method_has_variable_side_leaves`;
  - (b) in `refine_branch_goal`, reassemble a solved premise from its
    observational reduct (`reassemble_solved_premise`, checking the raw
    `Eq` before whnf) instead of refusing a non-reflexive one;
  - (c) fix the stage-1 defect in `check_generalized_branch_goal`, whose
    no-binder path also caught a solved restoration with an empty
    dependency telescope.
- **They are one unit with 2(d).** With the trial kept, (a) to (c) are
  inert: 842 newly admitted visits, 0 reassembly firings, and verdicts
  identical to base on all 230 population tests and on m1 to m4
  (`evt_2qef3qpjvtaf6`). Under eliminate-always, reassembly fires on 6A
  with 0 failures and m3 pins (c).
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

2(a) to 2(d), plus fixes for D-1 and D-2, in one landing unit or in
straight-ancestor increments the Architect cuts. An increment lands only
with its own reaching witness.

## Acceptance

- **AC-0a and AC-0b (Architect).** Rule D-1 and D-2 before any code. The
  Architect may scope each as a diagnostic measurement the ring runs.
- **AC-1.** 6A (86 rows, all from `fok_invert_atomlike`) and 6C (4 rows)
  eliminate and pass. The hand rows m1 to m4 give the right verdicts, and
  m3 pins (c). `trusted_base()` is unchanged.
- **AC-2.** The D-1 and D-2 rows are green, and there are zero main-green
  reds across every ken-elaborator integration binary, the catalog
  loaders and the mod-strict census.
- **AC-3.** A test pins deepD at body entries <= d for d = 3, 5 and 7,
  for the refusal and for the accepted twins, with the diagnostic
  unchanged. Time is measured locally and is not asserted in CI.
- **AC-4.** A deep-nest row whose leaves are all parameter-side, so the
  unrefined path and the refined fallback still run, measures F's entry
  count. If it grows as 2^d, F needs its own no-trial rule here.
- **AC-5 (mutation, QA).** Restoring the gate leaves AC-1's rows on the
  fallback path, pinned by the eliminated binder's coordinate. Restoring
  the retained-first trial fails AC-3's count.

## Stop conditions

- Any kernel or `trusted_base()` change.
- An accepted verdict changes outside the D-1 and D-2 rows.
- A rule outside deletion, solution, injectivity and conflict, unless
  the Architect rules it.

Checks: 8 fired: cut (i) alone gives base's observations, so AC-1 is
stated where elimination is the accepting path; a 2^d configuration
passes "not 3x per level", so AC-3 pins entries <= d. 9 fired: F is
unmeasured on non-variable-side nests, so AC-4 measures it.
