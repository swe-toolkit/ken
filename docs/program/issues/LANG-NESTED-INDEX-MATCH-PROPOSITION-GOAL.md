---
id: LANG-NESTED-INDEX-MATCH-PROPOSITION-GOAL
title: "A dependent match on a second vector at a constructor-refined index checks only when the goal is syntactically Term::Eq. The coherent-frame convoy gates on goal syntax, so a goal that is a variable application or Equal at a data type leaves the inner scrutinee unrefined. Close the predicate to any proposition"
status: merged
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

D0 ruled (Architect `evt_3yey05qyvbds0`; Steward fork (i) `evt_jqa7eq0hnx8w`).
The predicate is "the goal's checked sort is `Omega`", at three sites:

- a helper `goal_is_proposition`, next to `states_equality`;
- `has_nontrivial_coupled_sides` in `plan_coherent_frame_motive`, keeping
  both of its uses;
- the Eq destructure in `check_large_convoy_recursive_arm`. Its Eq arm
  stays byte-identical, and a non-Eq proposition reads its level from
  its own `Omega` sort.

The repair also deletes the planner's unused `defer_coupled_expansion`
parameter (Architect `evt_23ne4fxf29f0x`). `left_head` is a different
defect, a λ-headed redex reaching kernel inference, and moves to
`LANG-ELABORATED-BETA-REDEX-AT-KERNEL-INFERENCE`.

## Acceptance

- **AC-1.** `vec_pair_ind` checks in both forms, with no Axiom and no
  trust change.
- **AC-2 (control).** The `lang_embedded_context_convoy` and
  `lang_embedded_method_equation_convoy` suites, the 61-row AC-3 set of
  `LANG-EMBEDDED-CONVOY-WITH-CONTEXT-CONVOY` and the 74-file catalog
  census are unchanged. Report the census per file, with rc before and
  after.
- **AC-3.** A false twin of `vec_pair_ind`, whose `on_cons` step has the
  wrong motive index, is still refused.
- **AC-4 (mutation, QA).** Each revert alone reddens AC-1: the Eq-only
  flag, and the Eq-only site 3 (`_ => return Ok(None)`).

## Stop conditions

- Any kernel, conversion or `trusted_base()` change.
- Restated by the Architect (`evt_hvk99ype841v`), replacing "an Eq-goal
  row changes", which was keyed on surface spelling rather than the
  planner's goal. Each is advancing:
  - a row accepted on base is refused, or changes diagnostic class, other
    than the authorized `dual_false` re-pin;
  - a new `Internal` appears on any row or census file;
  - a newly accepted row adds a `trusted_base()` entry, or the false twin
    checks.

  A row going from refused to accepted with no trust delta is recorded,
  not a stop.
- The embedded planner raises "changed the embedded-eliminator
  population" or an ill-typed-ambient Internal on any census file.

## Closeout

Merged `033c19fa7` from exact `0f2247b04` (PR #4710). Language QA
`evt_268d05xtfzqgr`; Architect `evt_ygrc5ga8cvwn` with Decision
`dec_2wk69frs7wkw1`. The first route, `d555d6ba3` (PR #4707), failed CI:
an in-crate `#[cfg(test)]` caller still passed the deleted argument
(E0061). The respin deleted that one argument. QA on `d555d6ba3`
(`evt_2a0df0pkwjz4t`) carries over for unchanged content.

- `goal_is_proposition` reads the goal's checked sort as `Omega`. Both
  planner entries share it, and site 3 builds the recursive transport from
  `Omega(l)` when the goal is not a syntactic `Eq`. The kernel stays the
  arbiter.
- `defer_coupled_expansion` is deleted from `plan_coherent_frame_motive`,
  closing the carry from `evt_23ne4fxf29f0x`.
- Pinned rows: `vec_pair_ind` (direct and result-binder), the wrong-index
  twin at kernel `TypeMismatch`, and RECURSIVE-STEP's T2, each with exact
  `trusted_base()` identity. The false `Refl` row pins its own shape
  error.
- **Carries:**
  - `LANG-RECURSIVE-STEP-COORDINATE-COHERENCE` records T2 green and
    narrows AC-1 to T3;
  - the λ spelling of `left_head` stays with
    `LANG-ELABORATED-BETA-REDEX-AT-KERNEL-INFERENCE`.
