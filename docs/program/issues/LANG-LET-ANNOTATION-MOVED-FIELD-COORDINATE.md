---
id: LANG-LET-ANNOTATION-MOVED-FIELD-COORDINATE
title: "In a nested recursive step, a let annotation that mentions a moved field is typed at a different coordinate from the body, so a true proof returns Internal (R1) or K2's pre-fix refusal (R2). Type positions ignore var_refinements, and refine_let_rhs then rewrites the field's transport. Repair both as one atomic pair"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-RECURSIVE-STEP-COORDINATE-COHERENCE]
blocks: []
github: null
origin: "Adversary hunt of 45c3bd6db (evt_7qdsskjwg0mwh). Architect scoping ruling evt_2qa859pwa5sfq, on 17bf5738e: R1 and R2 are one atomic WP; the third-level match is LANG-THIRD-LEVEL-MATCH-MOVED-FIELD. Steward-filed per COORDINATION section 2."
---

# Let annotations see the moved field at the body coordinate

## Objective

A true proof whose recursive step binds a let with an annotation that
mentions a moved field elaborates, and no such row reaches the user as
`Internal`.

## Settled inputs (Architect `evt_2qa859pwa5sfq`, on `17bf5738e`)

- **Rows.** The Architect's probe extends the RSC test file's FIXTURES
  with `dual_len`. In the DualFS/DualCons arm:
  - r1, r1pp: `let ih : <goal at rest j> = <rec> … in ih` return
    `Internal` at elab.rs:10756.
  - r2, r2pi: `let ih : pp (dual_lookup a m rest j) = hp … in ih` and the
    Π-typed form are KernelRejected with K2's signature.
  - r2len (`let u : Equal Nat (dual_len a m rest) … = Refl`) is Ok.
  - The false twins r1f and r2f are refused, and the controls are Ok.
- **Producer (a).** `elab_type_named_variable` (:1148) is the only
  `RVarTy` resolver (:1263), and it returns the raw field. The
  expression-side resolvers (:11553, :1815) return fix 2's component. The
  ruling gives a prototype that consults `var_refinements`.
- **Producer (b).** `refine_let_rhs` (:1371) applies the
  scrutinee-to-target substitution inside the field's transport, which
  turns `cast (D k) (D m)` into `cast (D k) (D k)`.
- **(c).** K2's signature is the dependent-refined fallback's error,
  which masks a different error from the first attempt.
- **Atomic pair.** (a) alone makes r2len refused. Disabling (b) outright
  breaks ds5b `intervening_let_fresh_binder_inherits_branch_index_refinement`,
  which needs the substitution on an outer variable. So (b)'s repair is
  conditional.
- **The :10756 guard stays N0003.** It detects an inconsistent earlier
  stage. Close the producer, not the guard.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

D0 first: the Architect rules (b)'s conditional form, keeping the branch
index substitution for outer variables while never rewriting inside a
field's transport. Measure it against ds5b and the R rows. The D0 census
names r2pi's third mechanism, where the first attempt fails with "cannot
infer an introduction form", and rules it in or routes it out. Then
implement (a) and (b) together.

## Acceptance

- **AC-1.** r1, r1pp, r2 and r2len are Ok with `trusted_base()`
  unchanged, and no row returns `Internal`.
- **AC-2.** r1f and r2f stay refused and typed, and the controls stay Ok.
  These stay green: ds5b_dependent_match_refinement_acceptance,
  let5_checking_mode_let, lang_recursive_step_coordinate_coherence,
  lang_convoy_ambient_binder_coexistence,
  lang_convoy_embedded_sentinel_under_binder,
  dependent_match_coherent_frame_acceptance, lang_embedded_context_convoy
  and elab_recursive_frame_budget.
- **AC-3 (mutation, QA).** Reverting (a) returns r1 to `Internal`.
  Reverting (b) returns r2 to K2's signature.

## Stop conditions

- Any kernel, conversion or `trusted_base()` change.
- The repair needs the :10756 guard retyped.
