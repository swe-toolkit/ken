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

## Deliverable (W2a, Architect recut `evt_7729d1mezkcxk`)

- **Convoy.** At every `if`, literal matrix, and `match` on a family
  neither indexed nor recursive (kernel `recursive_shapes`), bind the
  equation as a hidden Γ entry pushed before the branch is elaborated:
  motive `λ y. Eq S s y → T`, applied to `refl s`. Those conditions are Γ
  binders and are not pushed to `path_conditions`.
- **`let` evidence.** At both `let` writers, push a hidden
  `e : Eq A x rhs↑` directly after `x`, elaborate the body under both
  binders, and emit `Let { ty: A, val: rhs, body: Let { ty: Eq A x rhs↑,
  val: refl rhs↑, body } }` (`evt_dha43qwwnp4g`). On the inferred route,
  strengthen the body type out of `e` with `lower_by` before the existing
  `subst0`. When `let_equation` returns `None`, behaviour is unchanged.
- **Indexed and recursive families.** No convoy; the base path condition
  and the base `admissible_path_conditions` filter, unchanged. A recursive
  family's induction hypothesis shares the motive, so a convoy there is
  `LANG-RECURSIVE-BRANCH-EVIDENCE-BELOW`'s work. `path_conditions`
  survives, holding exactly these constructor equations.
- **Certificate search.** In `emit_refinement_predicate_with`, record the
  ctx depth `d` of each convoy binder and add the candidate
  `Var(hyps + conds + ctx.len() - 1 - d)`, where `conds` is the surviving
  admissible conditions.
- **Hole application builder.** `refinement_hole_application(cx, hole,
  evidence)` returns `Option<Term>`, called in
  `emit_refinement_predicate_with` right after `declare_obligation_hole`
  with the hole's own level args. It returns `None` when any admissible
  condition remains; otherwise `Some(h v_{n-1} .. v_0 e_1 .. e_k)`,
  kernel-checked in the zonked `cx.ctx` against `Π hyps. φ`, refusing with
  `ElabError::Internal("path-condition evidence does not instantiate its
  obligation hole")`. Nothing new is emitted into the program.
- Representation, obligation ids and discharge are otherwise unchanged.

## Acceptance

- **AC-0 (D0, measure only).** Done at `c8e59b77c` (`evt_cphrr7fbt6gm`,
  ruled `evt_31p5m7pnr13fv`) and re-measured at `49dcfafc2`
  (`evt_2d0yxbbwg3251`).
- **AC-1.** One row per convoy site kind: `let`, `check_if`, `infer_if`,
  literal true, prior-false and residual, and a non-recursive constructor
  family at each caller that reaches one. Each shows the binder in the
  emitted core and a `Some` that passes its kernel check.
- **AC-1 controls.** Recursive: `pce_rec`'s `Zero` arm stays discharged
  with its goal byte-identical to base, the builder returns `None`, and an
  IH-using `Suc` method checks with the base motive. Indexed: the goal is
  byte-identical to base and the builder returns `None`.
- **AC-2.** Catalog and corpus obligation counts and outcomes identical.
- **AC-3.** Runtime parity is unchanged, which shows the convoy erases
  (through `LANG-REFINEMENT-PROOF-ERASURE`).
- **AC-4 (mutation).** Dropping one evidence argument in the hole
  application, including `e` at a `let` site, reddens that row and is
  refused by the builder's kernel check.

## Stop conditions

- A convoy site whose condition has no term in Γ: stop to the Architect.
- Any AC-2 delta: stop to the Architect with the named population.
- The change needs any file other than `elab.rs` and tests.
- Any kernel, `trusted_base()` or spec change.
