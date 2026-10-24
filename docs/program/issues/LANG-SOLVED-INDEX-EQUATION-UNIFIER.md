---
id: LANG-SOLVED-INDEX-EQUATION-UNIFIER
title: "Stage 2 of solved-index elimination: LANG-SOLVED-INDEX-SUBSTITUTION eliminates only all-variable methods whose premise slots each project to one leaf, and mixed methods and multi-leaf slots keep the alias path. Replace the per-leaf plan with a per-equation unifier (an equation telescope, injectivity decomposition, per-equation rule dispatch, dependency-ordered transport) and lift the gate"
status: active
owner: language
size: L
tier: T1
gate: architect
depends_on: [LANG-SOLVED-INDEX-SUBSTITUTION]
blocks: [LANG-SOLVED-INDEX-ELIMINATE-ALWAYS]
github: null
origin: "Architect staged route evt_6htgrnanmjnm4 on the §1a 6 hold of LANG-SOLVED-INDEX-SUBSTITUTION, after the Research advisory evt_34qvnmp4g573c (the standard unifier: Agda's eqTel and reorderTel, Lean's unifyEqs). Within the operator's 2026-10-17 restructuring authorization of that WP. Cut (i) per Steward evt_2tsvy1y6tsfqj on Architect AC-0 evt_42gkba2758n1v; the no-trial regime and the retry cost moved to LANG-SOLVED-INDEX-ELIMINATE-ALWAYS. Steward-filed per COORDINATION section 2."
---

# Solved indices by a per-equation unifier

## Objective

Every method's index premises are solved one equation at a time:
deletion, solution, injectivity or conflict, with the transported context
reordered by dependency on each solution. Mixed methods and multi-leaf
slots are then eliminated too, and the stage-1 gate is removed.

## Settled inputs

- **Stage 1** (LANG-SOLVED-INDEX-SUBSTITUTION, recut by the Architect
  `evt_6htgrnanmjnm4`). It provides the gate in
  `method_has_variable_side_leaves`, dependency-ordered binding in
  `solved_index_dependency_binders`, the AC-1b name layer
  (`arm_coordinates`), the convoy dedupe, and ConvoyRebound admission.
- **The population** is stage 1's gate fallbacks, counted by equation in
  its AC-0 census. The measured fallbacks so far are classes 6A
  (multi-leaf slots, 86 rows) and 6C (mixed methods, 4 rows).
- **The design** is the Research map (`evt_34qvnmp4g573c`): a variable
  telescope plus an equation telescope; injectivity replaces an equation
  by its argument equations; each solved equation becomes Refl
  separately; the context is reordered by free-variable dependency.

- **AC-0 ruling (Architect `evt_42gkba2758n1v`, at `d366662a6`).** The
  ruled code is in that event:
  - (a) remove both gates in `method_has_variable_side_leaves`, so mixed
    methods and multi-leaf slots become variable-side;
  - (b) in `refine_branch_goal`, reassemble a solved premise from its
    observational reduct (`reassemble_solved_premise`, checking the raw
    `Eq` before whnf) instead of refusing a non-reflexive one;
  - (c) fix the stage-1 defect in `check_generalized_branch_goal`, whose
    no-binder path also caught a solved restoration with an empty
    dependency telescope and left the solved field at its old coordinate.
  Measured in the probe: 6A (86 rows, all from `fok_invert_atomlike`)
  and 6C (4 rows) close, with 0 reassembly failures across 37 binaries.
- **Out of scope.** The regime trial stays. One regime with no trial,
  and the retry cost, are LANG-SOLVED-INDEX-ELIMINATE-ALWAYS.

## Deliverable

One landing unit with (a), (b) and (c), built from the AC-0 ruling, with
the regime trial unchanged.

## Acceptance

- **AC-0 (D0, Architect).** Done: `evt_42gkba2758n1v`.
- **AC-1.** The gates are gone, the 6A and 6C rows eliminate and pass,
  and `trusted_base()` is unchanged. The hand rows m1 to m4 (multi-leaf
  and mixed, true and false) give the right verdicts, and m3 pins (c).
- **AC-2.** Zero failures that are green on main, across every
  ken-elaborator integration binary plus the catalog loaders.
- **AC-3 (mutation, QA).** Restoring the gate leaves AC-1's rows on the
  fallback path, which a test pins by the eliminated binder's coordinate.

## Stop conditions

- Any kernel or `trusted_base()` change.
- A rule outside deletion, solution, injectivity and conflict, unless
  the D0 rules it.

Checks: 4 fired: the AC-0 code compiled and ran in the probe at
`d366662a6`. 8 fired: AC-3 restores the gate, the configuration that
keeps AC-1's rows on the fallback path.
