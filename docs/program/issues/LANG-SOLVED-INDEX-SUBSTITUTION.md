---
id: LANG-SOLVED-INDEX-SUBSTITUTION
title: "A solved match index is moved per variable by cast aliases whose proofs stay pinned to the raw index premise, so a nested match cannot abstract the index, a match on an alias takes the equation convoy at a stuck eliminator, and a written sibling type disagrees with its unmoved field. Eliminate a solved leaf with a variable side by substitution and one J transport of the reverted arm"
status: active
owner: language
size: L
tier: T1
gate: architect
depends_on: [LANG-LET-ANNOTATION-MOVED-FIELD-COORDINATE]
blocks: []
github: null
origin: "Architect D0 for LANG-THIRD-LEVEL-MATCH-MOVED-FIELD (evt_3nbk4bv8c7kpe), which also covers the Adversary finding on 8e9e2c2c7 (evt_563avf72ddez3, ruled real evt_1ndrpz0mg3fan). The ruled change outgrows that M node; restructuring returns to the operator to size (2026-09-21); operator authorized 2026-10-17. Steward-filed per COORDINATION section 2."
---

# A solved index is eliminated, not aliased

## Objective

A match arm whose index premise solves a variable is elaborated against
real binders at the solved coordinate, so later matches, written types
and the goal all see one coordinate.

## Settled inputs (Architect `evt_3nbk4bv8c7kpe`, on `8e9e2c2c7`)

- **The alias.** In the `DualCons` arm, `rest : DualEnv a k` is moved to
  the scrutinee coordinate `m` by
  `var_refinements[rest] = cast (DualEnv a k) (DualEnv a m) (J … e) rest`,
  installed by `install_index_refinements` (elab.rs:10031) through
  `try_reindex_cast` (:9595). Its proof uses the premise
  `e : Eq Nat (Suc k) (Suc m)`.
- **Three producers, one predicate.**
  - A (sec, secpp): `build_checked_dependent_motive` (:8075) rejects the
    inner `match j` motive. Generalizing `m` rewrites the alias's types
    but not its proof, which still goes through `e`.
  - B (secrest): the inner `match rest` scrutinizes the alias, so the
    guard at :4684-4708 sees a refined scrutinee and selects the equation
    convoy. Its motive is a stuck `elim_Nat` at `m`, and
    `finish_dependent_elim` (:7284) fails with NotAFunction.
  - q2/q3: producer (a) (`elab_type_named_variable`, :1148) resolves
    written `rest` through the alias, while the sibling
    `h : Equal Nat (dual_len a k rest) k` keeps its raw type. The result
    is K2 at `h`. Reverting (a) is not a repair: r1, r1pp and r2 regress.
- **Substituted arms check.** With the fields as real binders at the
  scrutinee coordinate, sub_sec, sub_secrest, s3 and s2 are Ok.
- **Population.** 59 of 59 leaf installs on 14 rows have a variable side
  (the constructor's field variable or an outer scrutinee variable).
- **Enabler.** The kernel J (check.rs:893) takes a Type-valued motive;
  the aliases on main already carry one. The arm-closing obligation for
  leaf `e' : Eq Nat k m` is
  `λ k x rest e. J (λ i _. Π (rest : DualEnv a i). G_i) (λ rest. B) (sym e') rest`.
  It is read, not built.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

For a leaf with a variable side, substitute the solved variable through
its dependents (later fields, hidden result variables, the goal),
elaborate the arm against real binders, and close the method with one J
transport of the reverted arm. This replaces, on that path:
`install_index_refinements` capability 1,
`install_hidden_result_variable_refinements` (:9880) and
`install_fields_from_goal_transport` (:8469).
`install_plain_declared_index_aliases` stays on its current path: it is not
on the variable-side-leaf path (Architect `evt_7ppfqyymvx6qn` Q3). The
Architect's design ruling there (R1: the generalized-goal telescope engine
with a dependency-closure selector and one J per leaf) is the build route.

Retained: leaf projection, injectivity and conflict leaves; the convoy
machinery; the kernel; the cast-alias path for a leaf with no variable
side. Producer (a) and `collapse_own_leaf_field_transports` become
no-ops on eliminated leaves; keep them until the acceptance rows show
no change without them.

## Acceptance

- **AC-0 (measure, before building).** Census the leaf installs across
  the LET-ANNOTATION AC-2 suites, split into field-variable side,
  scrutinee-variable side and neither. If "neither" is nonzero and has
  no retained path, stop and report.
- **AC-1.** sec, secpp, secrest, q2 and q3 are Ok, with `trusted_base()`
  unchanged.
- **AC-2.** secrestpp, r1, r1pp, r2, r2len, p8, b1 and b2 stay Ok; their
  false twins stay refused with a typed error; the LET-ANNOTATION AC-2
  suites stay green. q4 (written at `n`) is measured, not required.
- **AC-3 (mutation, QA).** Restoring the alias installer for
  variable-side leaves returns sec to its motive rejection and q3 to K2
  at `h`.

## Stop conditions

- Any kernel or `trusted_base()` change.
- A leaf class outside AC-0's census with no retained path.
- A local repair for one producer in place of the elimination: that is
  the next entry on the same predicate, not this node.
