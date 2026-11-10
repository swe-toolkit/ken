---
id: LANG-ARGUMENT-SLOT-SCAN-DEPENDENT-AND-SUBSET-HEADS
title: "SLOTS's argument-slot scan (application_slot_candidate) has two residuals on main: at a parameter whose codomain depends on it, the scan checks the argument and the real path checks it again, so a nest of held trees under a dependent head elaborates exponentially (3x per level, 1390 s at depth 14, against 0.009 s at depth 10 before SLOTS); and the scan reads the head's Pi without forgetting a subset, so a subset-typed head is refused NotAFunction where its let twin checks. Make the scan linear on dependent heads and agree with the real path on subset heads"
status: merged
owner: verify
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary findings on SLOTS Inc3 squash 4b6b3aabe: evt_6mdk3vhtan87z (dependent-codomain cost) and evt_5ejskbgzjbyr2 (subset head). Measured product defects in the code LANG-PROC-TREE-ARGUMENT-AND-SCRUTINEE-SLOTS landed. Steward-filed per COORDINATION section 2."
---

# The argument-slot scan is linear and sees through subsets

## Objective

A held tree passed to a dependent parameter is elaborated once, by the
path that keeps it, so the dependent nest elaborates in time linear in
its depth. A call whose head has a subset type over a Pi carrier
classifies its slots exactly as the real application path does, so the
direct form checks wherever its explicit-let twin checks.

## Settled inputs (main `4b6b3aabe`)

- **The scan.** `application_slot_candidate` (`elab.rs:14880`) walks the
  head's Pi telescope once. At a dependent codomain (`lower_by` fails,
  `:14893`), it runs `check(cx, argument, &domain)` (`:14894`) inside the
  throwaway snapshot that `infer_application_slot` takes and rolls back
  (`:14916`). The kept path then checks the same argument again.
- **The cost.** Adversary rows, one `ElabEnv` per row: `seqd`, whose
  result type mentions its first argument, takes 1.95 s at depth 8,
  17.3 s at 10, 156.8 s at 12 and 1390.8 s at 14. The non-dependent
  `seq2` takes 0.014 s at depth 14. On the pre-SLOTS base `2345d4178`,
  `seqd` takes 0.0091 s at depth 10.
- **The subset head.** The scan requires `whnf` of the head's inferred
  type to be a `Pi` (`:14883`-`14887`) and propagates the error through
  `scanned?` (`:14918`). The real path calls
  `forget_subset_at_elimination_head` first (`:14968`). With
  `def Fn2 = { f : Unit -> Unit -> Unit | ... }`,
  `g MkUnit (emit second)` and `g (emit second) MkUnit` are refused
  `NotAFunction`. The let twin and the plain-Pi control check.
- **The existing pin.** `held_tree_arguments_elaborate_in_linear_time`
  (`crates/ken-elaborator/tests/lang_proc_tree_argument_slot_core.rs:143`)
  covers only non-dependent heads. Its THE GAP line names the dependent
  scan.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

One candidate. It repairs the scan where the Architect rules and adds
both rows to the slot core tests. Each slot's core still equals its let
twin's.

## Acceptance

- **AC-0 (Architect).** Rules the repair: how the dependent arm learns
  the argument's core without a second elaboration, and where the
  subset is forgotten, so the scan and the real path read one head
  type.
- **AC-1.** A depth-14 `seqd` nest elaborates within the existing
  pin's bound. A mutation restoring the scan's check at the dependent
  arm reddens the pin.
- **AC-2.** `direct` and `direct0` over a `Fn2` head check, and their
  cores equal their let twins'. A mutation removing the subset forget
  from the scan reddens both, and the plain-Pi control stays green
  under it.
- **AC-3.** Every slot test that checked before still checks.

## Stop conditions

- A kernel or `trusted_base()` change.
- A change to what the real application path accepts.

Checks: 2 fired: read the scan at `elab.rs:14880`-`14897`, its caller
at `:14906`-`14918` and the real path at `:14955`-`14972` on
`4b6b3aabe`; only the scan reads the head's Pi without the subset
forget. 8 fired: the pin's bound already fails at depth 14 on `seqd`
(1390 s), so AC-1 can fail; AC-2's control separates the subset head
from slot sequencing.
