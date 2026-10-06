---
id: LANG-OBLIGATION-HOLE-LEVEL-META-CLOSURE
title: "A refinement obligation emitted while an unannotated Type parameter's level metavariable is unsolved closes over LevelVar(0) and is declared with no level parameters, so fn k (A : Type) (n : Int) : Int = use5 n is rejected by the kernel. Declare the hole over a level-closed context"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-REFINEMENT-TYPE-POSITION-INTRODUCTION]
blocks: []
github: null
origin: "F-F: Architect evt_71vryd6g641y3 found the leak while ruling TYPE-POSITION stop 5; the language implementer measured it pre-existing on c49297983 (evt_5qm7aksdjemxm). Fails closed: a valid program is rejected. Steward-filed per COORDINATION section 2."
---

# Obligation holes close over level-solved contexts

## Objective

A refinement obligation emitted inside a declaration with an unannotated
`Type` binder is declared as a well-formed hole, so the declaration checks
and reports the obligation, as it does without that binder.

## Settled inputs (Architect `evt_71vryd6g641y3`; measured `evt_5qm7aksdjemxm` on `c49297983`)

- **The row.** `fn use5 (x : {v : Int | Equal Int v 5}) : Int = x;
  fn k (A : Type) (n : Int) : Int = use5 n` exits 1 with `KernelRejected
  IllFormedDecl("undeclared level variable LevelVar(0)")` at the call. The
  same source without `(A : Type)` exits 0 with one open `Equal Int n 5`.
- **Mechanism.** Unannotated `Type` elaborates to a level metavariable
  (`elab.rs:1154`, represented at `:160-164`). `zonk_level` (`:166`)
  defaults an unsolved one to `Zero`, but `close_refinement_goal_with`
  (`:11494`) closes the goal over the raw `cx.ctx.types`, and
  `declare_obligation_hole` (`:4830`) declares it with no level parameters.
  Neither zonks.
- **Fan-in.** Every caller of `close_refinement_goal_with` and
  `close_goal` (`:11266`), not one emitter.
- **Not a drive-by.** Zonking the hole early is unsafe: a meta defaulted to
  `Zero` inside the hole, then solved to another level by the declaration,
  would make the hole's binder types disagree with the place it is applied.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **D0 (measure only).** Where holes are declared relative to the
   declaration's level solving and generalization, for each caller in the
   fan-in. Count the corpus declarations that emit an obligation under an
   unannotated `Type` binder. The Architect rules the closure point.
2. **The ruled closure.**

## Acceptance

- **AC-1.** The row checks with exactly one open `Equal Int n 5`, closed
  over `k`'s parameters; the twin without `(A : Type)` is unchanged.
- **AC-2 (controls).** A row whose level meta is solved to a non-zero level
  by the declaration also checks, with its hole well-typed at that level.
  TYPE-POSITION's fail-closed `Five → Type` pin turns into its one-obligation
  form.
- **AC-3 (mutation, QA).** Removing the closure returns the row to
  `KernelRejected`.

## Stop conditions

- Any kernel, `trusted_base()` or spec change.
- A closure that changes an existing obligation's goal text in the corpus.
