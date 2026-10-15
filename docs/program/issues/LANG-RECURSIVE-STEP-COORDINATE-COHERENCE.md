---
id: LANG-RECURSIVE-STEP-COORDINATE-COHERENCE
title: "After a nested index match splits, the goal is refined from the scrutinee coordinate to the target coordinate while the fields are cast the other way, so the recursive step of a true Equal proof compares terms neutral at different index variables and is refused. A step-reachable control with no embedded eliminator ends in an Internal error. Put the goal, the fields and the recursive call at one coordinate, with the IH at that coordinate"
status: active
owner: language
size: L
tier: T1
gate: architect
depends_on: [LANG-EMBEDDED-METHOD-EQUATION-CONVOY, LANG-CONVOY-AMBIENT-BINDER-COEXISTENCE]
blocks: []
github: null
origin: "Architect fork evt_3bfdj7qm0azt4, option (i), taken by the Steward (evt_33zvpm72tmtwn): T2, T3 and the step-reachable control move here from LANG-EMBEDDED-METHOD-EQUATION-CONVOY, which lands at T1. Steward-filed per COORDINATION section 2."
---

# One coordinate for the recursive step

## Objective

The recursive step of a nested indexed match sees one index coordinate
for its goal, its fields and its recursive call, so true rows check and
user-reachable failures are typed.

## Settled inputs (Architect `evt_3bfdj7qm0azt4`, WIP `1048192afa`)

- **T2 is green on `033c19fa7`.** `LANG-NESTED-INDEX-MATCH-PROPOSITION-GOAL`
  admits it through site 3's checked-`Omega` arm and pins it as a positive
  row with exact `trusted_base()` identity (Architect `evt_hvk99ype841v`,
  Steward fork (i) `evt_6267xckj3mzbk`). The step-reachable control stays
  `Internal` there.
- **T3 is green at `44e4087d1`** (Architect `evt_6c47png8ynswh`): Ok with
  `trusted_base()` unchanged, as is its sibling `dual_option`, and its
  false twin is still refused. Some landing after WIP `1048192afa` turned
  it green.
- **Two producers act in opposite directions; the live anchor is H1.**
  H1 is KernelRejected TypeMismatch at its `DualFS` leaf. On the found
  side, the unfolded `dual_lookup` recursive step casts `rest` from
  `DualEnv a k` to `DualEnv a m` through `Eq Nat k m`. The expected side's
  `DualFin` eliminator is stuck on the variable. K2 is KernelRejected
  TypeMismatch: expected `Eq Nat @5 @9`, found `Eq Nat (Suc @5) (Suc @5)`.
- **The wall predates the convoy.** The step-reachable control, with
  `Proved` at DualFZ and no embedded xs-eliminator, fails identically on
  base `f7a9a2989` and on the WIP. The error is `Internal("large index
  convoy could not construct its recursive goal transport")`, now at
  elab.rs:7815 in `build_large_convoy_recursive_method` (:7780). Its step
  calls `cong`, which is not in the prelude
  (`Core/Logic/Transport.ken.md:56`).
- **Prior art to pull at kickoff.** The Coq `depelim` solution and
  simplification step, and Lean `cases` substituting index equations.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

D0 first: the Architect rules the design. The research pull is done
(`evt_5fcb7m8pk78nx`). Its invariant is a D0 input:
- a solved index variable is eliminated exactly once, by one transport;
- that transport's motive abstracts the goal together with every
  dependent of the variable: convoyed hypotheses, fields and the
  recursive call's arguments;
- no dependent is cast separately, in either direction.

Then one coordinate for goal, fields and recursive call after the split,
with the IH at that coordinate.

**Carried from the convoy WP (Architect `evt_1kc148cfjm929`).** It changes
no behaviour and needs no gate of its own. In `elab.rs`, move
`mentions_enclosing_refinement` above `compute_context_convoy`'s doc block,
and move `convoy_entry_order` out of the middle of `convoy_binder_types`'s
doc comment.

## Acceptance

- **AC-1.** The rows from `LANG-CONVOY-AMBIENT-BINDER-COEXISTENCE` check,
  with no Axiom and no trust change (Architect `evt_40eh6614dch8`; exact
  sources in thread `thr_5007397vycb5t`):
  - H1, whose `DualFS` leaf uses the convoyed `h`;
  - K2, `pp (dual_lookup a n xs i)` with the recursive call
    `hp (dual_lookup a m rest j)`.

  CONVOY's typed refusals of the false twins stay refusals.
- **AC-2.** The step-reachable control, spelled with `cong` in scope (a
  fixture declaration or an import of `Core.Logic.Transport`), succeeds or
  returns a typed refusal. No `Internal` reaches the user from
  `build_large_convoy_recursive_method`.
- **AC-3 (control).** These stay green with `trusted_base()` identity
  pinned: T2, T3 and `dual_option`. The convoy WP's T1, named-row and F4
  rows, its 61-row suite set and the catalog census are unchanged.
- **AC-4 (mutation, QA).** Restoring either producer's direction reddens
  H1 or K2.

## Stop conditions

- Any kernel rule, conversion change or `trusted_base()` change.
- A suite or census delta outside the named rows: stop to the Architect.
