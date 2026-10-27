---
id: LANG-SOLVED-INDEX-DEPENDENT-INDEX-RESIDUAL
title: "Since LANG-SOLVED-INDEX-EQUATION-UNIFIER (b7e85023e), a match on a family with a dependent index, such as DI : (n : Nat) -> Box n -> Type against z : DI x w with w : Box x, fails with ElabError::Internal(\"solved-index dependency closure crosses outside the method\"). The parent accepts it, and the NONLINEAR gate does not see the dependent index. Restore the parent's verdict, never an internal error"
status: ready
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-SOLVED-INDEX-NONLINEAR-RESIDUAL]
blocks: []
github: null
origin: "Adversary finding on LANG-SOLVED-INDEX-NONLINEAR-RESIDUAL squash 7a5efa246 (evt_679wnydrgqbmq): a correctness regression from b7e85023e that fails closed. 5 rows are OK on 336f9d253 and Internal on 98970bde7 and 7a5efa246. It violates the unifier WP's stop condition that no accepted verdict changes outside its D-1 and D-2 rows. Serves the L2 objective. Steward-filed per COORDINATION section 2."
---

# A dependent-index method never reaches an internal error

## Objective

Every program the parent of `b7e85023e` accepts in the reported
dependent-index families is accepted again. No index shape reaches an
`ElabError::Internal` from the solved-index path.

## Settled inputs (main `7a5efa246`, Adversary `evt_679wnydrgqbmq`)

- **The rows.** With `data Box : Nat -> Type where { MkBox : (k : Nat)
  -> Box k }`, the family `DI : (n : Nat) -> Box n -> Type` (constructor
  `MkDI : (u : Nat) -> (b : Box u) -> DI u b`) and the family `DI2`
  (constructor `MkDI2 : (u : Nat) -> DI2 u (MkBox u)`), five rows are OK
  on `336f9d253` and Internal on `7a5efa246`. They are `di_value`,
  `di_box`, `di_cont`, `di2_value` and the `Refl` theorem `di2_thm` at
  `Equal (Box x) (MkBox x) w`. The control `z : DI x (MkBox x)` is OK on
  all three trees.
- **The path.**
  - The NONLINEAR gate `method_has_variable_side_leaves`
    (`crates/ken-elaborator/src/elab.rs:9297`) counts leaves from
    `method_index_premise_pairs` (`:10939`). That function drops any
    index whose domain mentions a prior index
    (`index_domain_mentions_prior_index`, `:12905`).
  - So the method takes the elimination path, and
    `solved_index_dependency_binders` (`:11255`) reaches the outer binder
    `w : Box x`, which sits before `method_start` and is not a convoy
    original. It raises Internal at `:11314`.
- **Controls already closed by NONLINEAR.** A parameterised
  `PTwo A a a` and a three-index `Three u v u` are OK on `7a5efa246`,
  and their false variants refuse with TypeMismatch.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

Before the implementer starts, the Architect rules how a method whose
index premises include a dependent index is disposed of: route it to
the alias path, as NONLINEAR does for a repeated field, or extend the
solved-index closure to the outer binder. Then the repair, with the
five rows and the control pinned in
`elab/solved_index_equation_unifier_tests.rs`.

## Acceptance

- **AC-0 (Architect).** The ruled disposition, on the landed base, with
  a census of the other `Internal` exits on the solved-index path that
  the same plane omission reaches.
- **AC-1.** All five rows elaborate as on the parent, and `di2_thm`
  checks. `trusted_base()` is unchanged.
- **AC-2 (control).** The NONLINEAR rows, the unifier WP's D-1 and D-2
  rows, and its depth-seven and ParamNest rows are unchanged. CI is
  green.
- **AC-3 (mutation, QA).** Removing the repair makes `di2_value`
  Internal again.

## Stop conditions

- The disposition changes an accepted verdict outside these families.
- Any kernel or trust change.
