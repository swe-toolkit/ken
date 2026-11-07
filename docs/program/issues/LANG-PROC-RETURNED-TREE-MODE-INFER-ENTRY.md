---
id: LANG-PROC-RETURNED-TREE-MODE-INFER-ENTRY
title: "LANG-PROC-RETURNED-TREE-MODE-RESULT-POSITION hides returned-tree mode only on entry to `check`, but an unannotated let right-hand side, an if condition and a match scrutinee enter through `infer`, which leaves the bit set. A match inside them checks its arms as result positions, so inside a changed proc a plain arm at a written tree type is Ret-wrapped: `let r = match True { True ↦ (… : IO Int); False ↦ 4 } in 1` is accepted where the same body in a const refuses K0004, and `ken run` executes it. Hide the mode on every non-result entry"
status: active
owner: verify
size: S
tier: T1
gate: architect
depends_on: [LANG-PROC-RETURNED-TREE-MODE-RESULT-POSITION]
blocks: []
github: null
origin: "Adversary report evt_gf8mhwnbj9ep on c5c5e0900 (LANG-PROC-RETURNED-TREE-MODE-RESULT-POSITION), measured as 4 accepted forms against their refused const controls, one of them executed. A measured product defect; the fix direction is the owner's. Steward-filed per COORDINATION section 2."
---

# Returned-tree mode stays at the result, through infer too

## Objective

Inside a changed proc, a subterm that is not one of the proc's own
result positions never sees returned-tree mode, whether it is reached
through `check` or through `infer`. Every form below refuses as its
const sibling does.

## Settled inputs (main `eb65971bf`)

- **The fix that landed.** `check` (`elab.rs:3442`) diverts to
  `check_outside_result_position` (`:3432`) when the bit is set, and
  that function clears the bit for the call. `check_returned_tail`
  (`:3298`) clears it for the tail application (`:3314`).
- **The entries it misses.** These reach a subterm through `infer`,
  which does not clear the bit:
  - an unannotated let right-hand side, `prepare_let_rhs`'s
    `None => infer(cx, rhs)` (`:2023`);
  - an if condition, `infer(cx, condition)` (`:2581`);
  - a match scrutinee, through `infer_match_with_predicates`
    (`:28446`).
  A match in those positions checks its arms through
  `check_match_arm_result` (`:3367`), which calls
  `check_result_position` (`:3256`), and that function reads the bit.
- **The measured forms** (Adversary `evt_gf8mhwnbj9ep`, `ken check`
  at `eb65971bf`). Each is inside `proc p : Int visits [Console]`, with
  the match `match True { True ↦ (ITree.Ret … 3 : IO Int); False ↦ 4 }`:
  - accepted: as an unannotated let RHS; nested inside a let in that
    RHS; inside a let in a match scrutinee; inside a let in an if
    condition;
  - refused: the same bodies in a `const` refuse K0004. The condition
    form's const control refuses with IfConditionNotBool instead.
- **Run.** With `p (b : Bool) : String` and an unannotated let RHS
  `match b { True ↦ (print_line "t" : IO Unit); False ↦ MkUnit }`,
  `ken run` accepts `MkUnit` as `Ret MkUnit` and prints `done`.
- **The pin gap.** The landed WP's AC-1 forms all enter through
  `check`, so no row reaches an `infer` entry.

## Deliverable

The mode confined on every non-result entry, the measured forms pinned
as refusals beside their const controls, in one candidate.

## Acceptance

- **AC-0.** The Architect rules the fix site from the fan-in. The
  options are clearing the bit on the `infer` entries, or carrying
  result position explicitly rather than as a context-wide flag. The
  ruling enumerates every caller of `check_result_position`, and every
  path into it that is not a result position.
- **AC-1.** The four accepted forms refuse inside a changed proc, each
  with its const sibling's error. The run form no longer executes.
- **AC-2.** A changed proc's own returned if, match and let tails still
  sequence. The LANG-PROC-RETURNED-TREE-MODE-RESULT-POSITION suites stay
  green.
- **AC-3.** A mutation that leaves the bit set through the unannotated
  let RHS reddens AC-1.

## Stop conditions

- A kernel or `trusted_base()` change: an operator question.
- A change to an accepted program's core type outside the measured
  forms.

Checks: 4 fired: `check`, `check_outside_result_position`,
`check_returned_tail`, `prepare_let_rhs`'s `infer` arm, the condition
`infer`, `check_match_arm_result` and `check_result_position` were read
at `eb65971bf` at their lines. 2 fired: `check_result_position`'s
fan-in includes `check_match_arm_result`, which `infer_match` reaches,
so a guard in `check` alone does not cover it. 8 fired: each accepted
form has a refused control that is the same expression in a `const`.
