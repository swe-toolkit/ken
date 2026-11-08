---
id: LANG-PROC-TREE-ARGUMENT-AND-SCRUTINEE-SLOTS
title: "A performing proc sequences a callee's tree only at a let and a returned tail. A tree-typed result in a scrutinee is refused E0038 with a diagnostic that names ITree where the user wrote Box, and spec 36 §2.4's argument sequencing (g a1 … an) is not implemented. Sequence a tree in the scrutinee and argument slots as §2.4 rules, with the native plan for a Ret that carries a closure"
status: ready
owner: verify
size: L
tier: T1
gate: architect
depends_on: [LANG-PROC-NON-TREE-RESULT-TREE-FS-ROW-AND-ARROW]
blocks: []
github: null
origin: "Architect FSRA closeout evt_7atdxpv0zgghg (scrutinee and argument slots open, the native returned-closure plan rides with them); FSRA settled input evt_688spjn83adpq. Steward-filed per COORDINATION section 2."
---

# A tree sequences in every slot

## Objective

A performing proc whose callee returns a tree sequences that tree where
spec 36 §2.4 rules it: in each argument of `g a1 … an`, left to right,
and in a match scrutinee. The pin `r1_recursive_result` flips from
refusal to execution.

## Settled inputs (measured at `45b53ce49`, FSRA Inc 3 landed)

- **The rule.** Spec 36 §2.4 (`36-effects.md:600`): `⟦g a1 … an⟧` binds
  each `⟦ai⟧` in order, then splices `incl (g↓ caps x1 … xn)`. The
  grouped-let paragraph forbids hoisting an action across a binding, a
  lambda, a match branch or a handler.
- **Today's slots.** FSRA sequences a tree only at a let and a returned
  tail. `refuse_unsequenced_performed_let` (`elab.rs:2464`, callers
  `:2502` and `:14838`) is the §2.5 refusal for an action with no tree.
  It is not this node's target and stays.
- **The scrutinee pin.** `r1_recursive_result` and
  `r1f_recursive_result_fs_row`
  (`crates/ken-cli/tests/lang_effect_row_cell_scrutinee_carried_row.rs:442`)
  match on `pick`'s recursive `Box` result. Both refuse E0038 with
  "constructor 'MkBox' is not a constructor of type 'ITree'".
  `r1k_recursive_constructor` (`MkBox action` as scrutinee) refuses
  E0087 and is not in scope.
- **No argument pin exists.** No test applies a callee to an argument
  whose own call returns a tree.
- **Out of scope.** The transition sentinels r11, p3, r1 and r1k stay
  E0087 refusals: the spec did not rule a [Console]-latent callback's
  tree to be `IO` (Architect `evt_7atdxpv0zgghg`). The E0087 raw-term
  text (`let_inclusion_unavailable`, `elab.rs:2303`) is a separate
  diagnostic.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

Scrutinee and argument sequencing in the elaborator, the native plan
the accepted programs need, and the migrated pins, in the increments
AC-0 rules.

## Acceptance

- **AC-0.** The Architect specifies a D0 on the landed base. The Verify
  implementer runs it on scratch and reports it. The Architect rules
  from the report and does not build or test. It rules the elaboration
  per slot, the native plan for a `Ret` that carries a closure, and the
  increment split. A runtime plan change outside the elaborator is a
  stop to the Steward for placement.
- **AC-1.** `r1_recursive_result` and `r1f_recursive_result_fs_row` run
  and print the pinned output. A new pin applies a callee to two
  tree-returning arguments whose actions print distinct bytes, and its
  output order is `a1` before `a2`.
- **AC-1r.** An argument action inside a lambda or a match branch is
  not hoisted out of it. A pin whose branch is not taken prints none of
  that branch's bytes.
- **AC-2.** A mutation that restores the scrutinee refusal reddens
  AC-1. A mutation that reverses argument order reddens the order pin.

## Stop conditions

- A kernel or `trusted_base()` change: an operator question.
- A change to a declared-tree program's core type, or to §2.4's text.

Checks: 4 fired: read §2.4 at `36-effects.md:600`, the r1/r1f/r1k pins
at `:442` and the refusal at `elab.rs:2464` on `45b53ce49`. 3 fired:
grepped crates/*/src and crates/*/tests for argument-slot sequencing
pins; none exist, so AC-1 adds the order pin. 8 fired: the order pin's
two actions print distinct bytes, so a reversed or dropped argument
yields a different output.
