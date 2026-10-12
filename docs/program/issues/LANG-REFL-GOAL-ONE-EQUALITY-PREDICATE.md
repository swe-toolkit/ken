---
id: LANG-REFL-GOAL-ONE-EQUALITY-PREDICATE
title: "Refl decides that its goal is an equality by two different keys: the standalone gate looks up the spelling Equal in globals, the match-arm branch simplifier reads a transparent body that is exactly Eq. A user alias of Eq is refused standalone and accepted in every arm. Decide it once, by checked identity, for both paths"
status: ready
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-MATCH-ARM-REFL-INDEXED-MOTIVE]
blocks: []
github: null
origin: "Adversary report evt_6nqn8v9x79mrg on f7a9a2989 (LANG-MATCH-ARM-REFL-INDEXED-MOTIVE): the split between standalone and arm Refl now runs the other way. Anchors confirmed by the Steward on main. Steward-filed per COORDINATION section 2."
---

# One equality predicate for Refl

## Objective

`Refl` has the same meaning standalone and in every match arm, decided by
one predicate that does not depend on a spelling.

## Settled inputs (Adversary `evt_6nqn8v9x79mrg`, at `f7a9a2989`)

- **Repro.** `const MyEq : Pair Nat Nat -> Pair Nat Nat -> Prop =
  λp. λq. Eq (Pair Nat Nat) p q`. Standalone `MyEq p p = Refl` fails with
  "Refl expects an `Eq`-shaped goal". The same goal checks in List, Bool,
  let and `DualFin` arms. The false twin is refused on every route, and
  the kernel re-checks every emitted proof, so this is a correctness gap,
  not a soundness one.
- **Two keys.**
  - Standalone: `refl_goal_originates_in_equality` (elab.rs ~:2340, gate
    ~:2135) accepts a literal `Term::Eq`, or a head equal to
    `cx.globals.get("Equal")`. That is a lookup by spelling (Check 10).
  - Arms: `stated_equality` (~:4337) accepts a `Const` whose fully
    applied transparent body is exactly `Eq`, plus the motive redex
    `LANG-MATCH-ARM-REFL-INDEXED-MOTIVE` added.
- **Agreement elsewhere.** An alias whose body is `Equal`, closed-Top
  carriers, and index-dependent goals give one answer on all five routes.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

D0: the Architect rules whether a transparent alias of `Eq` is an
equality goal for `Refl`. Then one predicate, keyed on checked identity,
serves both the standalone gate and the branch simplifier.

## Acceptance

- **AC-1.** `MyEq p p` gives the same verdict on all five routes, the one
  D0 rules.
- **AC-2 (control).** The false twin is refused on every route. The
  `lang_match_arm_refl_pair_goal.rs` and
  `lang_match_arm_refl_indexed_motive.rs` suites are unchanged.
- **AC-3 (mutation, QA).** Reverting either path to its own key reddens
  AC-1.

## Stop conditions

- A kernel, conversion or `trusted_base()` change.
- A catalog or suite delta outside these rows.
