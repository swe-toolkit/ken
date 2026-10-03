---
id: KERNEL-LEVEL-CLOSURE-CHECK
title: "The kernel proves Bottom with no postulate, because no admission checks a declaration's free level variables against its level parameters, so a definition's own u escapes instantiation. Check level closure and distinct parameters at every admission, and emit Bottom for unequal universes only over closed levels"
status: active
owner: kernel
size: S
tier: T1
gate: architect
depends_on: [KERNEL-QUOT-EQ-INTERIM-CAST-LEVEL]
blocks: []
github: null
origin: "Research sweep evt_7daqm6ydmbnsr S1 (operator item (v), evt_6d6fzbqt2sa8r). Architect ruling evt_24070d0c9zgmw item 1. Kernel change; operator 2026-10-03: 'approve A and B'. Steward-filed per COORDINATION section 2."
---

# Level variables are closed at admission

## Objective

No declaration is admitted with a level variable outside its own distinct
`level_params`, and no Eq at universes reduces to Bottom over open levels.

## Settled inputs (Architect `evt_24070d0c9zgmw`)

- Repro (Research S1): `f : Type (suc u) := Type u` with `level_params =
  []`; `g {u} : Eq (Type (suc u)) f (Type u) := refl f`; `g {0} : Bottom`
  is accepted. `[u, u]` parameters are admitted too.
- Shape: one kernel function `check_level_closure(level_params, terms)`,
  refusing `IllFormedDecl` on a duplicate parameter or any `Level::Var`
  outside `level_params` in types, bodies, inductive params, indices,
  constructor argument types, the family level, and every `level_args` on
  Const, IndFormer, Constructor and Elim.
- Fan-in: `GlobalEnv::add_decl` is `pub(crate)` (`env.rs:561`). Production
  sites: `declare_inductive` (`check.rs:1052`) and its support inductive
  (`:1098`), `stage_placeholders` (`:1239`) and the body upgrade in
  `admit_pending`, `declare_postulate` (`:1449`), `declare_primitive`
  (`:1470`), `declare_deceq_certificate`; `declare_prelude_const`
  (`env.rs:505`) asserted closed. The checked-core decode
  (`checked_core.rs:3023`) reaches the env only through the public
  `declare_*` functions.
- Defence in depth: `eq_at_type`'s Type/Type and Ω/Ω arm (`obs.rs:402-408`)
  emits Bottom when levels are not `equiv`, which is sound only if level
  normalization is complete (S8 shows it is not). Bottom only when both
  levels are closed and differ; otherwise neutral.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. `check_level_closure` at every production admission and in-place `Decl`
   replacement; the narrowed universe arm.
2. Spec 12 §4 states the admission check; spec 16 §2.2's universe arm states
   the closed-level condition. Conformance rows follow.

## Acceptance

- **AC-0.** At the exact SHA, every production `add_decl` site and every
  in-place `Decl` replacement is listed with its check call, and a grep
  shows no decode-to-env path bypassing the public `declare_*` API.
- **AC-1.** `f` is refused at `f`; `[u, u]` is refused; `g {0}` cannot be
  formed; open-level `Eq (Type L) (Type u) (Type v)` stays neutral. The
  prelude, catalog parity and the kernel suite stay green.
- **AC-2 (falsifiers).** M1: skip the check in `stage_placeholders`; the
  `f` pin reddens. M2: restore the open-level Bottom; the neutral pin
  reddens.

## Stop conditions

- An admission site without the check, or a decode path that bypasses the
  public API: stop to the Architect.
