---
id: LANG-EFFECT-ROW-AMBIENT-LABEL-IDENTITY
title: "Effect rows compare labels by spelling, so a user `space Console` is the ambient Console label: a top-level proc that calls `Console.call` on that space passes the row check with `visits [Console]`, where the same program with the space named Other refuses EffectEscapes. Base then fails at the kernel with K0004, and LANG-PROC-NON-TREE-RESULT-TREE fails closed at its inclusion table with E0087. Make a row label mean one checked effect, so a user space cannot stand in for an ambient label"
status: merged
owner: verify
size: M
tier: T1
gate: architect
depends_on: [LANG-PROC-NON-TREE-RESULT-TREE]
blocks: []
github: null
origin: "Architect exact-SHA review of LANG-PROC-NON-TREE-RESULT-TREE Inc 1 (evt_11k493kejypt9, carry 2): a base defect for the effects owner, either reserve the ambient labels against space names or key rows on the checked effect identity. Operator 2026-09-25, the L2 objective: binding a prelude name anywhere is an error. Steward-filed per COORDINATION section 2."
---

# A row label names one effect

## Objective

A `visits` row label denotes one checked effect. A user space whose name
spells an ambient label (`Console`, `Clock`, `Entropy`) is never charged
or discharged as that ambient label. Either the space declaration is
refused as binding a built-in name, or rows are keyed on the checked
effect identity and the two labels stay distinct.

## Settled inputs (to be re-anchored on the landed base at kickoff)

- **The measured program** (Architect `evt_11k493kejypt9`, binaries over
  the source tree, on base `8bfa0b4e1` and candidate `6f5cc13d0`):
  `space Console { proc call () : Int visits [Console] = 1 }` checks rc 0.
  A top-level `proc user_console (n : Int) : Int visits [Console] = let m
  : Int = Console.call in m` passes the effect-row check on both.
- **The control.** With the space named `Other`, the same program
  refuses `EffectEscapes ... Other`. The row check therefore matches
  labels by spelling.
- **Where it fails today.** Base fails later at the kernel with K0004.
  After Inc 1, the candidate fails closed with E0087 at
  `include_effect_tree` (`elab.rs`), because the space's tree is not in
  the closed table. Neither accepts the program, so this is fail-closed
  aliasing, not unsoundness.
- **The two sites the Architect names**: the effect checker
  (`crates/ken-elaborator/src/effects/check.rs`, `EffectEscapes`) and
  `returned_tree_signature` (`elab.rs`), which both read the label by
  spelling.
- **The ledger.** Spec 30's intrinsic ledger reserves floor names, but no
  row reserves the ambient effect labels as names. The operator's
  2026-09-25 ruling makes binding a prelude name anywhere an error.

## Deliverable

The ruled rule in the elaborator, any spec 30 or spec 36 sentence it
needs, and the measured program pinned as a sentinel, in one candidate.

## Acceptance

- **AC-0.** The Architect specifies a D0 on the landed base, and the
  Verify implementer runs it on scratch. The Architect rules from the
  report between reserving the ambient labels and keying rows on checked
  identity. It names every consumer that reads a label by spelling. A
  spec change is a Spec child, reviewed by the Spec Leader and CV.
- **AC-1.** The measured program no longer passes the row check as the
  ambient label. It refuses with the ruled named error, or it is charged
  the user space and refuses EffectEscapes.
- **AC-2.** The same-form control with the space named `Other` keeps its
  current refusal, and a program that uses the ambient `Console` with no
  user space keeps checking.
- **AC-3.** A mutation restoring spelling comparison reddens AC-1.

## Stop conditions

- A kernel or `trusted_base()` change: an operator question.
- A rule that changes an existing accepted program's row.
