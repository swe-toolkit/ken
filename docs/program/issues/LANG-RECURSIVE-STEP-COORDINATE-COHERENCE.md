---
id: LANG-RECURSIVE-STEP-COORDINATE-COHERENCE
title: "After a nested index match splits, the goal is refined from the scrutinee coordinate to the target coordinate while the fields are cast the other way, so the recursive step of a true Equal proof compares terms neutral at different index variables and is refused. A step-reachable control with no embedded eliminator ends in an Internal error. Put the goal, the fields and the recursive call at one coordinate, with the IH at that coordinate"
status: ready
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
- **T3 is red** in the corrected spelling, in the convoy WP's entry-1
  shape. The found side casts `rest` to `DualEnv a m` through the
  predecessor `Eq Nat k m` binder. The expected side keeps the target
  coordinate `k`.
- **Two producers act in opposite directions.** At the T3 leaf (Nat,
  target @2 = k, scrutinee @4 = m), `refine_branch_goal` moves the goal
  from m to k, while `install_index_refinements` casts `rest` from k to m.
  Both Σ components then compare terms neutral at different index
  variables, related only by an equation. No convoy companion supplies
  that.
- **The wall predates the convoy.** The step-reachable control, with
  `Proved` at DualFZ and no embedded xs-eliminator, fails identically on
  base `f7a9a2989` and on the WIP. The error is `Internal("large index
  convoy could not construct its recursive goal transport")` at elab.rs
  ~:6886 in `build_large_convoy_recursive_method`, where
  `check_large_convoy_recursive_arm` returns `None`.
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

- **AC-1.** T3 elaborates and checks, and the pinned T2 row stays green.
  The rows from `LANG-CONVOY-AMBIENT-BINDER-COEXISTENCE` also check, with
  no Axiom and no trust change (Architect `evt_40eh6614dch8`; exact sources
  in thread `thr_5007397vycb5t`):
  - H1, whose `DualFS` leaf uses the convoyed `h`;
  - K2, `pp (dual_lookup a n xs i)` with the recursive call
    `hp (dual_lookup a m rest j)`.

  CONVOY's typed refusals of the false twins stay refusals.
- **AC-2.** The step-reachable control succeeds or returns a typed
  refusal. No `Internal` reaches the user from elab.rs ~:6886.
- **AC-3 (control).** The convoy WP's T1, named-row and F4 rows, and its
  61-row suite set and 72-file catalog census, are unchanged.
- **AC-4 (mutation, QA).** Restoring either producer's direction reddens
  AC-1.

## Stop conditions

- Any kernel rule, conversion change or `trusted_base()` change.
- A suite or census delta outside the named rows: stop to the Architect.
