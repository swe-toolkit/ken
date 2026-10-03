---
id: LANG-SIBLING-GOAL-REFINEMENT
title: "A goal that mentions an indexed sibling binder refined by a match on a different scrutinee fails (BadEliminator, 'could not classify the branch goal'); record each failing leaf's proof, its inferred type and its installing match before any fix"
status: active
owner: language
size: L
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
   - The region-extension repair (`evt_1b4t346jcdrkr`) moved the first
     failure to the FSuc arm (`InferredMatchResultEscapesPattern {
     tail_ys }`, `elab.rs:18107`; stop 3, `evt_6gsvxjqx8wqm2`).
   - **Recut (Architect `evt_4yzxxbzka2z3c` on Research
     `evt_20w8wrae0fye3`): one frame record replaces per-consumer
     reconstruction.**
     1. A `MatchFrame`, pushed and popped by each dependent-match arm
        wherever `match_field_regions` is pushed today (`elab.rs:6206` and
        siblings), carries: (i) the origin of every binder the arm pushes,
        written at the push (Field, IH, Scrutinee, ConvoyRebound{original},
        GeneralizedDependent{original}, GeneratedEquation), keyed by de
        Bruijn level, never by index or type shape; origin is total, and a
        binder under an active frame with none is `Internal` at the
        consumer; (ii) the arm's refined target, so a nested match with a
        known expected type elaborates in check mode, and the
        inferred-matrix path (`:18107`) is the no-expected-type fallback
        only; (iii) the map from the arm's telescope to the enclosing one,
        absorbing increment 1's generalized-binder redirect.
     2. Consumers read the record and their local reconstructions are
        deleted: `compute_context_convoy`'s scrutinee self-skip and
        field-region skip, increment 1's premise-sentinel redirect, the
        nested method binder-source lookup (entry 1), and the route that
        sends a nested match to inferred-matrix projection (entry 3).
     3. Unchanged: increment 1's behaviour (`aa0c46bc1`), the Class A
        fixes, LEAF-PARITY's `lower_binders`, and the `:3535` guard and
        `lower_by` escape refusal, which stay as fail-closed backstops.
        `a64abf8ae` rides first.
   - **D0 (design only, Architect gate, non-advancing).** (a) Every site
     that pushes a binder under an active match frame, with its origin,
     counted at a named base, including the convoy re-bind and
     equation-premise pushes in `check_generalized_branch_goal`. (b) Every
     reader of `match_field_regions`, of the scrutinee `Term::Var`
     spelling, of `derived_depth` and of the increment-1 redirect, each
     migrate or out with a reason. (c) The entry-3 routing trace on the
     disposable first patch: `infer` or `check` for the FSuc inner `match
     ys`; at `:18107`, `derived_depth`, context length, `tail_ys`'s level
     and the frame ranges. The owed f6 trace rides D0 if cheap.
   - **D0 gated (Architect `evt_3v5en59224w18` on `evt_6qs6wc9m6mef7`):
     36 push sites, 21 under an active frame, 15 fresh contexts; 28
     consumer dispositions. Entry 3 is the dropped goal:
     `check_large_convoy_recursive_arm` calls `infer(cx, &arm.body)` at
     `elab.rs:5564` though `expected_here` is computed. The build follows
     three rulings:**
     - **R1.** The origin set adds `UserLocal` for every binder user source
       or term/type formation introduces under a frame (`let`, lambda, Π/Σ/J
       motive locals, inferred let, literal/tuple/record binders).
       `UserLocal` is ambient by definition. Each of the 21 active-frame
       sites pushes through one API that takes the origin as an argument,
       with no default; never derived from position or type.
     - **R2.** Every arm producer pushes its `MatchFrame` before its own
       constructor fields, so each field is written as Field into its own
       frame: `check_dependent_branch_body`,
       `check_large_convoy_recursive_arm`, the lifted and structured methods,
       and the indexed-matrix leaves (`:18352`). D0's arm-producer census is
       the population.
     - **R3.** At `:5564`, `check(cx, &arm.body, &expected_here)` replaces
       `infer`. A later use of the inferred type reads the expected type or
       the checked term's kernel type, never a re-inference.

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

  Increment 2 (the recut, Architect `evt_4yzxxbzka2z3c`):
  - `lookup_zip_with`, both arms including the FSuc recursive call,
    checks as a pinned fixture, and L3 consumes it.
  - The 10 Class B rows, the 5 Class A rows, `zip_with_map_pointwise`,
    the controls below and the exactly-once restoration control are
    unchanged. The genuine-ambient control (a user `let t = tail_xs`
    inside the `xs` arm, typed at the field index) keeps its verdict.
  - Falsifiers: F1, drop one origin write, and the totality `Internal`
    fires (not a downstream symptom); F2, force the FSuc inner match to
    inference, and `InferredMatchResultEscapesPattern { tail_ys }`
    returns; F3, classify by the fields-only region, and `:3535` returns
    on the FZero arm; F4, remove the frame push from
    `check_large_convoy_recursive_arm`, and the FSuc fields land in the
    enclosing frame and the classification test reddens. F1 may instead
    fail to compile. The ambient control is classified `UserLocal` and
    still refused at `:3535`.
  - f5 and f6 stay out of scope and are re-measured on the recut.
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

3. Inner `match ys` in the FSuc arm infers its result over its own
   pattern binder `tail_ys` (`InferredMatchResultEscapesPattern`,
   `elab.rs:18107`) despite an expected goal, keyed on the inferred-matrix
   path's `derived_depth` (`evt_6gsvxjqx8wqm2`).

§1a count: 3 (`evt_5y97zdd2prmsg`). §1b: one predicate (Architect
`evt_4yzxxbzka2z3c`): a nested match frame does not receive the state its
enclosing frame holds (binder provenance, the refined goal, the map to the
enclosing telescope), and each consumer reconstructs it by spelling, by
position or by falling back to inference. The recut above is its closure.
The next research re-trigger is the 6th stop.

## Stop conditions

- Increment 2 build: R3's check-mode change exposes a new site (advancing
  stop 4, ruled inside the recut), or an arm producer cannot open its frame
  before its fields without reordering kernel-visible binders: stop to the
  Architect with the site.
- Any kernel conversion change, trust change, or change to `zip_with`.
- g1 fails at parse (`expected a type, found Lambda`). That is the
  surface-grammar gap (`evt_7aem5zqk3dqm8`) and out of scope.
