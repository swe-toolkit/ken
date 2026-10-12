---
id: LANG-NESTED-INDEX-MATCH-PROPOSITION-GOAL
title: "A dependent match on a second vector at a constructor-refined index checks only when the goal is syntactically Term::Eq. The coherent-frame convoy gates on goal syntax, so a goal that is a variable application or Equal at a data type leaves the inner scrutinee unrefined. Close the predicate to any proposition"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-EMBEDDED-CONVOY-WITH-CONTEXT-CONVOY]
blocks: []
github: null
origin: "Architect ruling evt_1ze7b0na77885 in SPEC-LINEAR-ALGEBRA-DIMENSION-SAFE-CONTRACT: the frame's Language-prerequisite stop fired on left matrix identity. Steward scope choice (ii): chapter 65 section 1 lands with five proved laws, and left identity and associativity are gated on this WP. Check 2 (fan-in) and check 11 (dependent match refines only its scrutinee). Steward-filed per COORDINATION section 2."
---

# A nested index match checks at any proposition goal

## Objective

A dependent match on a second vector, at an index refined by matching
the first, checks whatever proposition the goal is, as it already does
when the goal is a syntactic equality.

## Settled inputs (Architect `evt_1ze7b0na77885`, at `c826ab48e`)

- **Minimal reproduction, Vec only.** `vec_pair_ind`, a two-vector
  induction principle whose motive is a variable, matches `xs` and then
  `ys`. The inner match fails with an index mismatch: `Vec b n` expected,
  `Vec b (Suc t)` found. The variant binding `ys` in the result type
  fails identically. The source is in the ruling.
- **The catalog row.** `left_head` in the spec-author's
  `LeftHeadCombined.ken` (`evt_42cyqqmh8vnf`) is rejected on main with
  "cannot infer an introduction form ... without an expected type". Its
  empty and successor branches each check alone.
- **The contrast.** `dot_comm`, whose goal is a plain `Eq` on Nat, checks
  by the same nested match. So the gap is the goal's shape.
- **The locus.** The coherent-frame convoy gates on goal syntax in two
  places:
  - `has_nontrivial_coupled_sides` holds only for `Term::Eq`;
  - the builder's goal head admitted only `Eq` or `Pi` until
    `LANG-EMBEDDED-CONVOY-WITH-CONTEXT-CONVOY` widened it to any
    proposition.
  A scratch probe widening the first gate moved the failure one step, to
  `Internal("large index convoy could not construct its recursive goal
  transport")`.
- **A second symptom.** On the convoy candidate `4361cc6bc`,
  `LeftHeadCombined.ken` fails earlier with `Internal("coherent-frame
  convoy found an ill-typed ambient method: ...")`. The embedded planner
  tries to infer a bare-λ method of an eliminator in the goal.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

D0 first. The Architect rules the predicate that replaces the two
syntactic gates, and whether closing it also removes the ambient-method
symptom. Then the repair: one predicate, not one more case.

## Acceptance

- **AC-1.** `vec_pair_ind`, in both forms, and `left_head` check, with no
  Axiom and no trust change.
- **AC-2 (control).** The `lang_embedded_context_convoy` and
  `lang_embedded_method_equation_convoy` suites and the 61-row AC-3 set
  of `LANG-EMBEDDED-CONVOY-WITH-CONTEXT-CONVOY` are unchanged, as is the
  74-file catalog census.
- **AC-3.** A false twin of `vec_pair_ind`, whose `on_cons` step has the
  wrong motive index, is still refused.
- **AC-4 (mutation, QA).** Restoring either syntactic gate reddens
  AC-1.

## Stop conditions

- Any kernel, conversion or `trusted_base()` change.
- A catalog census row changes status, or the false twin checks.
- The repair overlaps `LANG-RECURSIVE-STEP-COORDINATE-COHERENCE`'s
  recursive-coordinate wall. Stop to the Architect to rule the order.
