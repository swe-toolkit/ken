---
id: LANG-CONVOY-AMBIENT-BINDER-COEXISTENCE
title: "A true dependent-match proof fails when one more ambient binder is in scope: an unused binder typed at the index hits an internal assertion that the index-equation convoy overlaps an ambient context convoy, and an unused hypothesis carrying the goal's embedded eliminator makes the elaborator emit ill-typed core. Let both convoys coexist"
status: active
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-EMBEDDED-CONVOY-WITH-CONTEXT-CONVOY]
blocks: [LANG-RECURSIVE-STEP-COORDINATE-COHERENCE]
github: null
origin: "Adversary M8 finding evt_2qd35nyer850 on 66cb5e9b7 (LANG-EMBEDDED-CONVOY-WITH-CONTEXT-CONVOY): two sibling shapes of the class, both failing on fd3b9262b too, fail-closed. Check 7 (a fix must hold on its siblings). Steward-filed per COORDINATION section 2."
---

# An ambient binder does not break a convoyed match

## Objective

A true dependent-match proof checks whether or not an unrelated ambient
binder or hypothesis is in scope. Well-formed source never reaches an
internal assertion, and the elaborator never emits core the kernel
rejects.

## Settled inputs (Adversary `evt_2qd35nyer850`, at `66cb5e9b7`)

The fixture is `env()` from
`crates/ken-elaborator/tests/lang_embedded_context_convoy.rs`. The rows'
source is in the finding.

- **Shape 1, internal error.** E8, an `Equal a (g ...) (g ...)` goal
  under an outer `match c` with an inner `match i`, is Ok. E9 adds an
  unused `(xs : DualEnv a n)` and fails with `Internal("index-equation
  convoy unexpectedly overlaps an ambient context convoy")`. E11, with
  `xs` after `i`, fails the same way. E10, which is E9 without the outer
  match, is Ok.
- **Its locus.** In `plan_coherent_frame_motive`, the `equation_convoy`
  branch (elab.rs:4486-4490) returns that error when
  `probe_context_convoy` is non-empty. The merged repair changed only the
  embedded-method gate below it.
- **Shape 2, ill-typed core.** H9 is AC-1 row R1 plus an unused
  hypothesis `(h : Equal a (dual_lookup ...) y)`. It fails with
  `KernelRejected TypeMismatch` at the `match xs`, the signature R1 had
  before the fix. H5 (leaf `h`) fails identically, and H1, the nested
  `match i` form using `h`, fails with a `J`-cast mismatch. The controls
  H6, H7 and H8 are Ok. The embedded eliminator is not what blocks H1.
  At the inner frame the convoy tests each binder at its raw context type,
  which does not mention the inner index, so `h` is never convoyed
  (Architect trace `evt_3p22pk3q6f7fb`).
- **Shape 3, an untransported hypothesis about the inner scrutinee**
  (Adversary `evt_1n4z88r4r064e`; pre-existing on `4ee16ba51`). G1 has a
  Type goal, `h : pe n xs`, nested `match i` then `match xs`, and
  `f k x rest h` at the leaf. It is `KernelRejected TypeMismatch`. G2 has
  the goal `pe n xs` with `h` at the leaf, and is a `J`-cast mismatch. The
  Vec form fails the same way. The single-level `match xs` control is Ok.
  - **The mechanism.** At the inner `match xs`, `scrutinee_level` is
    `None` and the seed is `{m}`. `compute_context_convoy` (~:4615) tests
    raw types. `cx.var_refinements` carries the refined types
    (`h : pe (Suc m) S1`, where S1 is the inner `scrut_core`).
- **Not a regression.** All the failing rows fail identically on
  `fd3b9262b`. No row has a recursive step.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The D0 is ruled (Architect `evt_mgkvvs0f31b7`, part 2): one composition
rule. The motive is one telescope. Every rewrite applied to the goal is
applied at the same depth to each premise type its subject occurs in, and
premises are ordered so each follows everything its rewritten type
mentions. The ruling gives the order at the four sites (index premises,
C1, companions, C2, goal). A companion that depends on a C2 binder is
refused as a typed error.

**S6 ruling (`evt_469ezp3by3mj6`): the selector carries no context
convoy.**
- D0's hoist of the convoy block into `build_index_equation_convoy_body`
  is withdrawn, and the selector is left untouched.
- A frame whose ambient binders depend on the index takes the coupled
  route: `equation_convoy` also requires `probe_context_convoy.is_empty()`,
  apart from `forced_telescope_convoy`.
- The overlap `Internal` stays as it is. It now states the invariant this
  gate enforces.
- **Build order:**
  1. the S1 gate;
  2. the refined plane;
  - 2b. this gate;
  3. D0 part 2's ordering at the motive, the arm,
     `build_convoy_refined_type` and `finish_dependent_elim`.

**S1 ruling (`evt_2qrmtt7t9pfkv`): E9 and E11 never take the selector.**
The index-equation selector is entered only when an enclosing frame
refines the scrutinee or its index (`enclosing_refines`, with
`mentions_enclosing_refinement` reading `var_refinements` and the
sentinels). It is no longer entered merely because some frame encloses
it. The selector method itself is not revised. Build this gate first, then
the amendment below with D0 part 2's composition. A `NotAFunction` at a
selector use with a constructor-headed index is a stop under S1.

**D0 amendment (`evt_3p22pk3q6f7fb`): the convoy reads the arm-refined
plane.**
- When `scrutinee_level` is `None`, the referent is the binder whose active
  refinement value is `scrut_core`, matched by term identity. It is
  skipped.
- A refined binder is tested and carried at its refined type, weakened to
  the ambient depth.
- `ConvoyEntry` gains an `occurrence` term, and every `Term::var(entry.var)`
  consumer uses it. The ruling lists the nine sites and the two test
  callers.
- The seed and the premise order are unchanged. G1/G2's `h` is C1, and
  H1's `h` is C2.

## Acceptance

- **AC-1.** E9, E11, H9, H5, G1, G2 and G2's Vec form check, with no
  Axiom and no trust change. H1 and K2 move to
  `LANG-RECURSIVE-STEP-COORDINATE-COHERENCE` (Architect order ruling
  `evt_40eh6614dch8`, after research `evt_5fcb7m8pk78nx`). Both fail at a
  recursive-step position, where a solved index is transported twice.
- **AC-2 (control).** E8, E10, H6, H7 and H8 stay Ok, and so does the
  single-level `match xs` control. The
  `lang_embedded_context_convoy` suite, including its false twin, is
  unchanged, and so is the catalog census.
- **AC-3.** A false twin of H9, whose hypothesis disagrees with the goal,
  is still refused. So is G2's false twin `h : pe n ys`, with a typed
  error. H1 and K2 are typed refusals (`KernelRejected` or
  `TypeMismatch`), never `Internal`, with trust identical.
- **AC-3b.** The And-over-`dual_lookup` pair from the Architect's
  `evt_26r9qax9s5ze` never reaches the user as an `Internal`. That pair is
  A1, `lookup_and`, a true theorem, and A1F, its false twin with `hp x` in
  the `qq` slot. Both were kernel refusals on `f1ae73cc5` and are
  `Internal("coherent-frame convoy found an ill-typed ambient method")`
  on `033c19fa7`. A1 checks, and A1F is refused with a typed error
  (Architect `evt_6y37qbrjket3t`, which corrects `evt_40eh6614dch8`).
  - **The source of the `Internal`:** `plan_embedded_method_convoy`
    (elab.rs ~:3062) kernel-checks the original method in an
    `ambient_ctx` whose binder positions disagree with it.
  - **The fix:** make that check well-scoped. Relabelling the `Internal`
    as a typed error is not a fix. The ruled design is a scope-carrying
    `collect_outer_embedded_elims`. An eliminator under binders that is
    independent of them is lowered to the planner's depth. Otherwise it
    is left in place.
- **AC-4 (mutation, QA).** Restoring `!cx.match_frames.is_empty()` in
  place of `enclosing_refines` returns E9 to its failure.
- **AC-4b (mutation, QA).** Removing `&& probe_context_convoy.is_empty()`
  sends H7 back to an `Internal`. It discriminates only once step 3 is
  in.

## Stop conditions

- Any kernel, conversion or `trusted_base()` change.
- The repair overlaps `LANG-NESTED-INDEX-MATCH-PROPOSITION-GOAL`'s gates
  or `LANG-RECURSIVE-STEP-COORDINATE-COHERENCE`'s wall: stop to the
  Architect to rule the order.
- **S6.** Evaluated at the end of step 3, not between steps. Stop to the
  Architect if any of these differs from Ok: H7 or another AC-2 control, a
  `lang_embedded_context_convoy` row, the 19 suite tests, or a
  `LANG-PROPOSITION-GOAL-SCRUTINEE-FREE-RECURSION` row. Also stop if step 3
  makes H9 and H5 Ok but leaves H7 failing.
