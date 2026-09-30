---
id: LANG-SIBLING-GOAL-REFINEMENT
title: "A goal that mentions an indexed sibling binder refined by a match on a different scrutinee fails (BadEliminator, 'could not classify the branch goal'); record each failing leaf's proof, its inferred type and its installing match before any fix"
status: active
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-INFER-MATCH-INDEX-COVERAGE]
blocks: []
github: null
origin: "Architect AC-0 disposition evt_6e58trrr1bnfx on LANG-REFINED-SIBLING-MATCH-TAIL (component 3, mechanism not yet known, so it keeps an AC-0). Inherits the operator's 2026-09-30 Language scheduling ('concur with recs'). Steward-filed per COORDINATION section 2."
---

# Goal refinement over a refined sibling

## Objective

A goal that mentions a sibling binder, whose index was fixed by matching a
different scrutinee, is refined consistently. So f7, f4, f5 and f6 check.

## Settled inputs (Architect `evt_6e58trrr1bnfx`)

- **The witnesses.** f7 is the minimal `BadEliminator("refl a does not
  match Eq A x y")`. f4 and f5 raise `Internal("index refinement: could not
  classify the branch goal")`. On `ba2cd314c`, f6 is the same `Internal`
  classification error, not a `BadEliminator`. All are in
  `LANG-REFINED-SIBLING-MATCH-TAIL.md` and `evt_3tsanywqzghge`.
- **The compared indices are distinct binders:** the inner VCons field
  index `_` and the outer Fin-branch `m`, related only by generated
  premises.
- **The predecessor landed** as `ba2cd314c`
  (`LANG-INFER-MATCH-INDEX-COVERAGE`). There, `lookup_zip_with` gets past
  inferred-match coverage and fails at sibling-goal refinement.

## Deliverable

Two increments on this thread, each its own candidate (Architect AC-0b
ruling `evt_bw82kr4k5pm5`; Steward resize):
1. **Class B (SUBST, 10 rows): f5, f6 and f4's outer `xs`** (M, T1;
   Architect B ruling `evt_71bay9b5yf04r`).
   - The defect: the re-typed context set is computed on the ambient
     `cx.ctx`, but the candidate goal is classified on the expanded kernel
     view (`active_premise_kernel_view_for_context`). The binders it
     references live only on the second plane.
   - The repair, in `refine_branch_goal`: take the binders free in the
     candidate goal, on the expanded view, whose types mention
     `leaf.scrutinee`, closed outer-first. Abstract the goal over them,
     rewrite with `subst_term_generalize`, and restore through the existing
     `try_reindex_cast` or Ω transport on the same leaf proof.
   - Invariant: no goal rewrite without a rewrite of every goal-referenced
     dependent, in the same step and on the same plane.
2. **Class A (5 rows): the J-base `Refl` inside a `Cast`, f7 and f4's
   inner `ys`.** This is a kernel reducer defect (Architect
   `evt_449gyxrrejte1`): the reducer emits ill-typed `Cast` reducts.
   Increment 2 waits for `KERNEL-OBS-REDUCT-WITNESS-TYPING`, and the five
   rows may then go green. A B row that involves a reducer-synthesized
   reduct moves here too.

## Acceptance

- **AC-0 (no fix).** For each failing J base and each classification
  attempt, including f6, record:
  - the resolved `leaf.proof`: sentinel region, slot, and the match that
    owns the region;
  - its kernel-inferred type, against
    `Eq(leaf.index_ty, leaf.target, leaf.scrutinee)`;
  - which match installed the leaf that the goal rewrite consumed.

  It is one mechanism if and only if every failure is a leaf whose proof's
  endpoints are not its recorded endpoints (a wrong region, orientation or
  slot). Stop to the Architect with the rows.
- **AC-0b (done).** The measurements refute one mechanism: Class A is
  OTHER and Class B is SUBST, and the kernel is correct on all 15 rows.
- **B-AC0 (done).** 10 attempts and 48 dependent binders measured: two
  exclusion mechanisms, so a structural closure. No B row fires one of the
  four kernel witness sites, so all 10 stay in B.
- **B-D0 (design only, Architect gate).** On a base other than
  `ba2cd314c`, first re-count the 10 rows and 48 binders. Then: the
  representation of a generalized premise binder and the step that resolves
  it; the sweep of goal-rewriting `subst_term_generalize` callers, each in
  or out with a reason; the double-refinement control.
- **A measurement.** Paused until the kernel WP lands.
- **AC-1.** Increment 1: all 10 B rows classify. Each of f4, f5 and f6
  checks or fails at a named later site. The 38 unreferenced dependents
  stay untouched. Increment 2: f7 and f4 check. `lookup_zip_with` checks
  unchanged, together with `LANG-INFER-MATCH-INDEX-COVERAGE`.
- **AC-2.** The controls e2, e3, e6, f1-f3 and f8 are unchanged. A
  committed test pins that a binder already refined by its own enclosing
  match is transported once, not twice. A's five rows keep their verdicts
  under increment 1.
- **AC-2.** The controls e2, e3, e6, f1-f3 and f8 are unchanged, plus a
  mutation named in the ruling.

## Stop conditions

- Any kernel conversion change, trust change, or change to `zip_with`.
- g1 fails at parse (`expected a type, found Lambda`). That is the
  surface-grammar gap (`evt_7aem5zqk3dqm8`) and out of scope.
