---
id: LANG-INDEX-CONVOY-IMPOSSIBLE-ARM-LEVEL
title: "The index-equation convoy types a conflicting-equation branch as bare Top : Omega 0, so a dependent match whose goal sort is Omega l with l >= 1 builds an ill-typed index method and is refused with Internal. Make the impossible arm live at the goal's level"
status: active
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-LATE-LEVEL-SOLVE-SIBLING-SITES]
blocks: []
github: null
origin: "Adversary M8 finding F1 evt_5n6tdjd05bm7t on 49dcfafc2 (pre-existing since 5e30b8f62). Correctness, false reject, fails closed. Spec 16 section 2.1 puts Eq A u v in Omega l for A : Type l; section 8.4 says the bare Top : Omega 0 cannot stand at a higher level. Steward-filed per COORDINATION section 2."
---

# An impossible arm lives at the goal's level

## Objective

A well-typed dependent match over an indexed family elaborates at every
goal level, as spec 34 sections 3.2 and 3.2.1 promise, instead of being
refused by an ill-typed generated index method.

## Settled inputs (Adversary `evt_5n6tdjd05bm7t`, at `49dcfafc2`)

- `build_index_equation_convoy_body` (`elab.rs` ~:3683-3692) returns
  `Term::Const { id: env.top_id(), level_args: [] }`, which is
  `Top : Ω_0`, for a conflicting-equation branch that does not return a
  telescope. The selector motive's sort is `Omega(goal_level)` (~:3530),
  and the method is checked against it at ~:3816.
- **Repro.** Prelude plus `data DualFin : Nat -> Type where { DualFZ :
  (n : Nat) -> DualFin (Suc n); DualFS : (n : Nat) -> DualFin n ->
  DualFin (Suc n) }`, then `theorem t1 (X : Type) (n : Nat) (i : DualFin
  n) (j0 : DualFin n) : Eq Type X X = match i { DualFZ m ↦ match j0 {
  DualFZ q ↦ Refl; DualFS q r ↦ Refl }; DualFS m j ↦ Refl }` gives
  `Internal("large index convoy constructed an ill-typed index method:
  TypeMismatch { expected: Ωsuc 0, found: Ω0 }")`. The same shape with
  goal `Eq a x x` at `(a : Type 1)`, as a theorem and as a `let` in a
  `fn`, fails the same way.
- **Controls.** Each row at `(a : Type)` is Ok, and a forced-index
  telescope goal at `Type 1` is Ok because that arm returns
  `expected_at_depth`.
- The only other production `top_id()` with empty level args is
  `top_intro_body` (~:18143), a `Top` goal's own intro: not a sibling.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The conflicting-equation arm's result inhabits `Ω_goal_level`. The
Architect rules at D0 which proposition stands there, using what the
kernel already provides.

## Acceptance

- **AC-0 (D0).** The Architect names the term and confirms it adds no
  kernel or trust surface.
- **AC-1.** The three repro rows check, their `Type 0` twins and the
  telescope-arm control stay Ok, and the late-solved variant (`a : Type`
  solved to 1 by a later argument) is re-run and its result recorded.
- **AC-2.** The existing dependent-match and index-convoy suites stay
  green, and the catalog census has no verdict change. Targeted runs only.
- **AC-3 (mutation, QA).** Restoring `Top` at `Ω_0` reddens AC-1.

## Stop conditions

- The arm needs a new kernel constant, a universe-lifting primitive, or a
  spec change.
