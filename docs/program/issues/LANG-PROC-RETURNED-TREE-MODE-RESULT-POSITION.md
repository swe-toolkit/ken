---
id: LANG-PROC-RETURNED-TREE-MODE-RESULT-POSITION
title: "LANG-PROC-NON-TREE-RESULT-TREE's returned-tree mode is documented as read only at result positions, but the bit stays set through a let right-hand side and into lambda bodies, so inside a changed proc a plain value checked at a written tree type is wrapped in Ret: `let r : IO Int = if True then 3 else 4 in r` is accepted where the plain `let r : IO Int = 3 in r` and the same if-form in a const or a tree-result proc refuse K0004. Confine the mode to the proc's own result positions"
status: active
owner: verify
size: S
tier: T1
gate: architect
depends_on: [LANG-PROC-NON-TREE-RESULT-TREE]
blocks: []
github: null
origin: "Adversary report evt_6arvq7ax81ybr on f00ff23bb (LANG-PROC-NON-TREE-RESULT-TREE), measured as 4 accepted leak forms against 6 refused same-expression controls. A measured product defect; the fix direction is the owner's. Steward-filed per COORDINATION section 2."
---

# The returned tree stays at the result

## Objective

Inside a changed proc, whether `let r : T = e` checks does not depend on
the enclosing proc's row or on whether `e` is written as an if, a match
or a lambda. Returned-tree mode is consulted only at the proc's own
result positions, as `elab.rs:937` to `:938` states.

## Settled inputs (main `f00ff23bb`)

- **The stated rule.** `returned_tree_mode` (`elab.rs:939`) is
  documented: "Only result positions consult this bit; argument, RHS and
  type subchecks do not."
- **The leak.** `check_result_position` (`:3251`) dispatches to
  `check_returned_position` whenever the bit is set. That function routes
  `RLet` to `check_let` (`:3292`) with the bit still set, so the let's
  RHS, and the if and match branches inside it, reach a result-position
  check in tree mode. A lambda body is checked by
  `check_result_position` (`:3588`), so it inherits the bit too. The
  tail application path (`:3303`) clears the bit correctly.
- **The measured forms** (Adversary `evt_6arvq7ax81ybr`, `ken check` at
  `f00ff23bb`), each inside `proc p : Int visits [Console]`:
  - accepted: `let r : IO Int = if True then 3 else 4 in r`, the same
    with a match, `let k : Unit -> IO Int = \_. 3 in 1`, and `let r =
    apply (\_. 3) in r` with `fn apply (k : Unit -> IO Int) : IO Int`;
  - refused K0004: the plain `let r : IO Int = 3 in r`, and the if and
    lambda forms in a `const` and in a tree-result proc.
- **Run.** `ken run` of the String if-form prints `three`; the plain
  sibling is refused.
- **The spec.** Spec 36 §2.4's `⟦v⟧ = Ret ⟦v⟧` denotes the proc's own
  effectful expression, not a coercion at a written annotation.

## Deliverable

The mode confined to result positions, the four measured forms pinned
as refusals with their accepted controls, in one candidate.

## Acceptance

- **AC-0.** The Architect rules the fix site from the measured fan-in:
  clearing the bit on entry to a let RHS and a lambda body, or carrying
  result position by position rather than as a context-wide flag.
- **AC-1.** The four accepted forms refuse inside a changed proc, with
  the same error their const sibling gives.
- **AC-2.** A changed proc's own returned if, match and let tails still
  sequence, and the LANG-PROC-NON-TREE-RESULT-TREE suites stay green.
- **AC-3.** A mutation that leaves the bit set through the let RHS
  reddens AC-1.

## Stop conditions

- A kernel or `trusted_base()` change: an operator question.
- A change to an accepted program's core type outside the four forms.

Checks: 4 fired: `returned_tree_mode`, `check_result_position`,
`check_returned_position`'s `RLet` arm, the lambda body check and the
tail-application clear were read at `f00ff23bb` at their lines. 8 fired:
each accepted form has a refused control that is the same expression
with the mode off.
