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
   `KERNEL-OBS-TYPE-EQ-STRUCTURAL` (site 3) merged `884f493fe`, so
   increment 2 is released on main `db19a8d0c`. Measure the five rows there
   first: a row still red names its first rejecting site before any
   repair. A B row that involves a reducer-synthesized
   reduct moves here too.
   - Measured on `f8bc5d4e1` (`evt_11byqak8y2p2v`): the five Class A rows
     pass. `lookup_zip_with` (CAT-VECTOR law 5) fails at the convoy guard
     `elab.rs:3535`, in the FZero arm at the inner `match ys`.
   - **Repair (Architect `evt_1b4t346jcdrkr`).** The entry that trips the
     guard is the index equation the enclosing `xs` arm generated. The
     arm's `match_field_regions` range, pushed at `elab.rs:6206` as
     `outer_scope_depth..ctx.len()`, covers only the constructor fields,
     so the re-bound convoy entry and the generated equation are classified
     ambient by position. When an equation-convoy arm pushes its re-bound
     convoy entries and generated index-equation premises, extend that
     arm's own range to cover them, and pop it unchanged with the arm.
     `compute_context_convoy`, the `:3535` refusal and the scrutinee
     self-skip stay as they are. The measurement commit `a64abf8ae` rides
     first in the candidate.

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
- **AC-1.** Increment 1 (the B-D0 gate `evt_1fbdcqg5127qa`, restoration
  form `evt_6pqf4vbt8n1f8`):
  - All 10 B rows classify, and the emitted method re-checks in the kernel
    at the original unrefined goal.
  - Each of f4, f5 and f6 checks or fails at a named later site. A later
    site in the obs.rs Phase 3 sub-cast is a kernel-dependency row.
  - The 38 unreferenced dependents stay untouched.
  - A committed row consumes a generalized premise through a generated
    proof (f6's shape).

  Increment 2 (Architect `evt_1b4t346jcdrkr`):
  - (a) `lookup_zip_with`, both arms including the FSuc recursive call,
    checks as a pinned fixture.
  - (b) The five Class A rows and `zip_with_map_pointwise` stay green.
  - (c) A genuine-ambient control keeps its verdict before and after: a
    user binder inside the `xs` arm whose type mentions the field index
    (`VCons _ x tail_xs ↦ let t = tail_xs in match ys {…}`) is pinned at
    its current verdict. Recorded provenance must not exempt user binders.
  - (d) Falsifier: drop the region extension, and (a) reddens at `:3535`.
- **AC-2.** The controls e2, e3, e6, f1-f3 and f8 are unchanged. A
  committed exactly-once control counts one leaf-keyed whole-Π restoration
  on the emitted term, and a duplicate-restoration mutation reddens it. A's
  five rows keep their verdicts under increment 1.

## Increments landed

- Increment 1 (Class B): merged `0ae184458` (PR #4410), exact
  `aa0c46bc1`. Language QA `evt_ca83804wqt64`, Architect
  `evt_24a17cz9c2zya`, Decision `dec_33yge2tyn50s9`.
  - The 10 rows classify, the 38 untouched dependents stay untouched, and
    one binder identity crosses the generalization boundary.
  - f4 now fails at the `obs.rs` Phase 3 kernel dependency, and f5 and f6
    at the `Refl` residual below.
  - The CI-red respin (`f0aaa79e1` to `aa0c46bc1`) returns early with zero
    equality leaves, so a malformed non-sort Pi still fails at final kernel
    admission (`ds5b` row).
  - Architect residual, diagnostic only: a no-op with nonempty leaves still
    builds the expanded view.
- Open: increment 2 (Class A), released on `db19a8d0c`. The WP stays open
  until it lands.

## Residual (carried, not closed)

Potential one-sided rewrite surfaces, unmeasured (`evt_1fbdcqg5127qa`):
`subst_term_generalize` callers at `elab.rs:3015`, `:3330`, `:3446`, `:3489`,
`:4271`, `:4461`, `:4615` and `:4931` (read at `ba2cd314c`). They rewrite a
goal, motive or IH while the context keeps its types, under independent
producers, with no measured failure. Each needs a measured failure before
it joins scope.

- B residual: surface `Refl` vs an observationally reduced `Eq` goal (f5,
  f6; Architect `evt_enxtkpcwcad3`). Measured on `f8bc5d4e1`: both goals
  are kernel-reflexive. f5 stops at the elaborator's Refl gate; f6 is a
  kernel `TypeMismatch` between `Eq Nat (Suc @9) (Suc @9)` and `Eq Nat
  (Suc @14) (Suc @14)`. Neither reaches `:3535`, so neither is increment
  2. The next f6 measurement, outside increment 2: name binders @9 and
  @14 and the producer of the `found` term and its build depth
  (`evt_1b4t346jcdrkr`).

## Hard-stop inventory (§1b)

§1a count: 2 (Architect `evt_34d247xhy4t9f`).

1. Nested eliminator method binder mismatch under generalized goal (f4,
   inner match `ys`), keyed on binder source across the generalization
   boundary (to be measured).
2. Nested equation-convoy match sees the generalized binder as an ambient
   convoy sibling of its own redirected scrutinee — keyed on scrutinee
   identity across the redirect (sentinel spelling vs pushed Var).

Candidate shared predicate (Architect `evt_1b4t346jcdrkr`): binder
provenance (generalized vs original, scrutinee vs sibling,
enclosing-generated vs ambient) is reconstructed at the consumer, by
spelling or by position, instead of being recorded where the binder is
pushed. Increment 2's convoy repair is its third consumer and not item 2:
there the scrutinee is a plain `Var` and is skipped correctly. If a next
stop lands, the §1b answer is this predicate, and the recut is one
provenance record for every binder match elaboration pushes.

## Stop conditions

- Increment 2: after the region extension, `lookup_zip_with` fails at a
  new site. Stop with that site: it would be advancing stop 3, a research
  hold under §1a.

- Any kernel conversion change, trust change, or change to `zip_with`.
- g1 fails at parse (`expected a type, found Lambda`). That is the
  surface-grammar gap (`evt_7aem5zqk3dqm8`) and out of scope.
