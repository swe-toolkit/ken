---
id: LANG-PATH-CONDITION-EVIDENCE
title: "Path conditions are obligation-only (elab.rs: never unchecked evidence in an emitted program), so a proof stored in a refinement pair has no term for the conditions its obligation is closed over. Bind each pushed path condition as a context variable through a convoy, with the carrier encoding otherwise unchanged"
status: active
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-REFINEMENT-PROOF-ERASURE]
blocks: [LANG-REFINEMENT-SUBSET-SIGMA]
github: null
origin: "Operator 2026-10-07: \"agreed, refinements should be real kernel types\" (OQ-refinement-representation DECIDED for the core subset Σ). Architect design and cut evt_30frdrmj45ehg. Steward-filed per COORDINATION section 2. W2. Recut by Architect AC-0 ruling evt_31p5m7pnr13fv: after LANG-REFINEMENT-PROOF-ERASURE."
---

# Path conditions carry evidence terms

## Objective

Every pushed path condition is a term in Γ, so the subset-Σ introduction
(`LANG-REFINEMENT-SUBSET-SIGMA`) can apply the obligation hole to it.

## Settled inputs (Architect, measured at `c8e59b77c`, `evt_31p5m7pnr13fv`)

- `elab.rs:11662`: path conditions are obligation-only, "never unchecked
  evidence in an emitted program".
- `close_refinement_goal_with` (elab.rs:11666) builds `Π Γ. Π conds.
  Π hyps. φ`, and `emit_refinement_predicate_with` (:11779) declares the
  hole and admits a found certificate as its body. **No term applies a
  refinement hole today.** The application is born with the subset-Σ
  introduction (`LANG-REFINEMENT-SUBSET-SIGMA`).
- Kernel `Let` substitutes; `let` evidence is an inner Ω `Let` binder
  with value `refl rhs` (Architect ruling `evt_dha43qwwnp4g`). The kernel
  `Context` carries no let values, so `refl rhs` does not check in the
  bare Γ.
- At an indexed-family scrutinee (`D : Π ī. Type`) the plain convoy
  motive is ill-typed in every branch: the kernel is non-cumulative and
  `ī'` is bound.
- `admissible_path_conditions` (:11643) is the only consumer.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

- **Convoy.** At every `if` and non-indexed `match` push site, bind the
  equation as a Γ entry: motive `λ y. Eq S s y → T`, applied to
  `refl s`. A condition with a Γ binder is closed by Γ and is **not**
  also pushed to `path_conditions`. `path_conditions` is removed; every
  condition is a Γ binder or, at an indexed scrutinee, not a hypothesis.
- **`let` evidence.** At both `let` writers, push a hidden
  `e : Eq A x rhs↑` directly after `x`, elaborate the body under both
  binders, and emit `Let { ty: A, val: rhs, body: Let { ty: Eq A x rhs↑,
  val: refl rhs↑, body } }`. On the inferred route, strengthen the body
  type out of `e` with `lower_by` before the existing `subst0`. When
  `let_equation` returns `None`, behaviour is unchanged.
- **Indexed families.** No convoy at an indexed-family scrutinee, and its
  condition is not a hypothesis: widen the drop in
  `admissible_path_conditions` from "indexed and ill-typed" to
  `is_indexed_family_equation`.
- **Certificate search.** In `emit_refinement_predicate_with`, record the
  ctx depth `d` of each convoy binder and add the candidate
  `Var(hyps + ctx.len() - 1 - d)`, so a discharge by a condition
  survives the move into Γ.
- **Hole application builder.** `refinement_hole_application(cx, hole,
  evidence)` builds `h v_{n-1} .. v_0 e_1 .. e_k : Π hyps. φ` in
  `cx.ctx`. Call it in `emit_refinement_predicate_with` right after
  `declare_obligation_hole`, with the hole's own level args. Kernel-check
  the result in the zonked `cx.ctx` against `Π hyps. φ` (the hypothesis
  Pis over `goal`, built as `close_refinement_goal_with` builds them,
  without the conds and Γ loops). Refuse with `ElabError::Internal("path-
  condition evidence does not instantiate its obligation hole")`. Return
  the term (`Option<Term>`) for `LANG-REFINEMENT-SUBSET-SIGMA`. Nothing
  new is emitted into the program from this builder.
- Representation, obligation ids and discharge are otherwise unchanged.

## Acceptance

- **AC-0 (D0, measure only).** Done at `c8e59b77c` (`evt_cphrr7fbt6gm`,
  ruled `evt_31p5m7pnr13fv`). Re-measure the site list at the rebased
  base before editing.
- **AC-1.** Each `if` and non-indexed `match` site binds a convoy, observed
  in the emitted core; each `let` site shows the inner `Let` with
  `val = refl rhs` at `ty = Eq A x rhs` and applies the hole to `Var(e)`.
  The hole application is observed through the builder's return and passes its
  kernel check, one row per site kind.
- **AC-2.** Population delta 0: per-package obligation counts and
  outcomes are unchanged across the catalog and the corpus. If any
  obligation's outcome changes (the indexed-family drop or the move into
  Γ), stop to the Architect with that named population.
- **AC-3.** Runtime parity is unchanged, which shows the convoy erases
  (through `LANG-REFINEMENT-PROOF-ERASURE`).
- **AC-4 (mutation).** Dropping one evidence argument in the hole
  application, including `e` at a `let` site, reddens that site's AC-1 row
  and is refused by the builder's kernel check.

## Stop conditions

- A push site whose condition has no term in Γ: stop to the Architect.
- An AC-2 outcome change: stop to the Architect with the population.
- Removing `path_conditions` needs any file other than `elab.rs` and tests.
- Any kernel, `trusted_base()` or spec change.
