---
id: LANG-SOLVED-INDEX-EQUATION-UNIFIER
title: "Stage 2 of solved-index elimination: LANG-SOLVED-INDEX-SUBSTITUTION eliminates only all-variable methods whose premise slots each project to one leaf, and mixed methods and multi-leaf slots keep the alias path. Replace the per-leaf plan with a per-equation unifier (an equation telescope, injectivity decomposition, per-equation rule dispatch, dependency-ordered transport) and lift the gate"
status: draft
owner: language
size: L
tier: T1
gate: architect
depends_on: [LANG-SOLVED-INDEX-SUBSTITUTION]
blocks: []
github: null
origin: "Architect staged route evt_6htgrnanmjnm4 on the §1a 6 hold of LANG-SOLVED-INDEX-SUBSTITUTION, after the Research advisory evt_34qvnmp4g573c (the standard unifier: Agda's eqTel and reorderTel, Lean's unifyEqs). Within the operator's 2026-10-17 restructuring authorization of that WP. Steward-filed per COORDINATION section 2."
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

## Deliverable

The Architect's D0 first rules the representation and the rule set in
Ken's vocabulary (the observational reduct for injectivity, a single J
per solution). Then one landing unit replaces the per-leaf plan and
removes the gate.

## Acceptance

- **AC-0 (D0, Architect).** The equation telescope and rules are
  measured on the stage-1 fallback population.
- **AC-1.** The gate is gone, the 6A and 6C rows eliminate and pass, and
  `trusted_base()` is unchanged.
- **AC-2.** Zero failures that are green on main, across every
  ken-elaborator integration binary plus the catalog loaders.
- **AC-3 (mutation, QA).** Restoring the gate leaves AC-1's rows on the
  fallback path, which a test pins by the eliminated binder's coordinate.

## Stop conditions

- Any kernel or `trusted_base()` change.
- A rule outside deletion, solution, injectivity and conflict, unless
  the D0 rules it.
