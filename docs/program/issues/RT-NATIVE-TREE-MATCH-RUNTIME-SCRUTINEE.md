---
id: RT-NATIVE-TREE-MATCH-RUNTIME-SCRUTINEE
title: "Clear the two rt_escape rows' next native refusal: lower a tree-producing ComputationalMatch whose scrutinee is not a specialized Bool, Nat or constructor, so the escaped-resource and nat-fanout programs build and run natively and agree with the interpreter"
status: ready
owner: runtime
size: M
gate: architect
tier: T1
depends_on: [RT-IGNORED-ROWS-NEXT-GROUP]
blocks: []
github: null
origin: "Operator L1 directive 2026-09-17 (clear the ignored tests). Successor to RT-IGNORED-ROWS-NEXT-GROUP, whose occurrence-key repair moves both rt_escape rows past the response-route collision to this refusal (runtime-implementer evt_5bgww4jqhyh6z). Steward-filed per COORDINATION section 2."
---

# Native tree-producing match on a runtime scrutinee

## Objective

Both `rt_escape` rows build natively, run, and match the interpreter.

## Settled inputs (runtime-implementer `evt_5bgww4jqhyh6z`, at `c569eef94` plus the ruled occurrence-key change)

- **The rows**, in `crates/ken-cli/tests/rt_escape_second_resource_native.rs`:
  - `escaped_resource_used_by_fanning_host_op_matches_interpreter` (`:654`);
  - `nat_fanout_escaped_resource_matches_interpreter` (`:714`).
- With the occurrence-key repair in place, both unchanged row sources exit 1
  at the same refusal:
  `ObjectEmission/checked_process_object: unsupported runtime-IR lowering:
  ComputationalMatch: tree-producing match scrutinee is not Bool or a
  constructor`.
- The refusal site is
  `crates/ken-runtime/src/cranelift_backend/lowering/core.rs:6527`. The
  lowering accepts a scrutinee operand that is a specialized Bool, a bounded
  or structural Nat, or a `Lowered::Constructor`, and refuses everything
  else.
- **Unmeasured:** which operand kind reaches the refusal, from which source
  term, and whether `RT-CLOSURE-BOUNDARY-LANE` or another blocker sits
  behind it.

## Deliverable

The two rows run green and un-ignored natively, with the interpreter
agreeing, through one Architect-ruled repair. An operand the repair does not
cover still refuses.

## Acceptance

- **AC-0 (probe, then D0; no build).**
  - Re-measure both rows on the landed NEXT-GROUP repair (Check 4).
  - At the refusal, report the operand's `LoweringOperand` kind, its
    `RuntimeExpr` origin, and the source term that produces it.
  - Propose the lowering. Name what the interpreter does on the same term,
    and what the new arm may and may not assume about the operand.
  - The Architect rules before any build.
- **AC-1.** Both rows pass un-ignored, and the differential agrees on both
  engines.
- **AC-2 (control).** Removing the new arm returns each row to the AC-0
  refusal. Rows outside the pair keep their current refusal.

## Stop conditions

- Any kernel, `trusted_base()` or spec change (an operator question).
- A row needs a second, independent repair behind this one: land what the
  one repair clears, relabel the rest, and report.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.
