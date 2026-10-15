---
id: LANG-RECURSIVE-STEP-COORDINATE-COHERENCE
title: "After a nested index match splits, the goal is refined from the scrutinee coordinate to the target coordinate while the fields are cast the other way, so the recursive step of a true Equal proof compares terms neutral at different index variables and is refused. A step-reachable control with no embedded eliminator ends in an Internal error. Put the goal, the fields and the recursive call at one coordinate, with the IH at that coordinate"
status: merged
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
- **The D0 is ruled** (Architect `evt_5hzbjtwpgm827`, at `44e4087d1`):
  one coordinate, the scrutinee index m. The "two producers in opposite
  directions" premise is measured false. The rows fail for three unrelated
  defects:
  - H1: `simplify_branch_goal_unguarded` `whnf`s through an `Ascript`, so
    the field cast's J loses its motive ascription and stays stuck.
  - K2: `install_index_refinements` casts `rest` a second time, beside the
    goal's single transport.
  - The control: `build_large_convoy_recursive_method` reports an empty
    case (`None`) as `Internal`.
- **The wall predates the convoy.** The step-reachable control, with
  `Proved` at DualFZ and no embedded xs-eliminator, fails identically on
  base `f7a9a2989` and on the WIP. The error is `Internal("large index
  convoy could not construct its recursive goal transport")`, now at
  elab.rs:7815 in `build_large_convoy_recursive_method` (:7780). Its step
  calls `cong`, which is not in the prelude
  (`Core/Logic/Transport.ken.md:56`).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The D0's exact `elab.rs` diff (`evt_5hzbjtwpgm827`, +200/-27): keep the
`Ascript` node and simplify under it; in an equation-convoy branch, rebind
each moved field to its component of the goal's transport; let `None` fall
through to the ordinary branch check. Keep the recursive-cycle frames at or
below base (`MATCH_STACK_BYTES` stays 3072 KiB; do not re-record it).

**Carried from the convoy WP (Architect `evt_1kc148cfjm929`).** It changes
no behaviour and needs no gate of its own. In `elab.rs`, move
`mentions_enclosing_refinement` above `compute_context_convoy`'s doc block,
and move `convoy_entry_order` out of the middle of `convoy_binder_types`'s
doc comment.

## Acceptance

- **AC-1.** H1 and K2 check with `trusted_base()` unchanged and no Axiom.
  The false twins H1F, K2F, G2F, H9F and `h9_false` stay refused.
- **AC-2.** The step-reachable control, spelled with `cong` in scope, is
  Ok. No `Internal` reaches the user from
  `build_large_convoy_recursive_method`.
- **AC-3 (control).** T2, T3 and `dual_option` stay Ok with
  `trusted_base()` identity pinned, and the catalog census is unchanged.
  The only test deltas are the two named positive ones:
  - `lang_convoy_ambient_binder_coexistence`: H1 and K2 move from typed
    refusals to checks;
  - `lang_convoy_embedded_sentinel_under_binder`: the true goals i6g and
    i6gb move to a checking test, and i6bf stays a refusal.
- **AC-4 (mutation, QA).** Reverting each fix singly reddens its row:
  fix 1 reddens H1, fix 2 reddens K2, fix 3 returns the control to
  `Internal`. T2, T3 and `dual_option` stay Ok under each.

## Stop conditions

- Any kernel rule, conversion change or `trusted_base()` change.
- A suite or census delta outside the named rows: stop to the Architect.
