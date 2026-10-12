---
id: VERIFY-RETIRE-BOUNDED-NAT-RECOGNIZER
title: "The static-response BoundedNat recognizer keys a declaration and a constructor family by spelling. It fires on the prelude freeze path, but the general OperationArgument path computes the same length correctly. Remove it, with a positive control that observes the length"
status: merged
owner: verify
size: S
tier: T2
gate: architect
depends_on: [VERIFY-PRELUDE-CLASSIFIERS-WHOLE-SYMBOL]
blocks: []
github: null
origin: "Architect carry evt_34jjncjpjkt6b, closing VERIFY-PRELUDE-CLASSIFIERS-WHOLE-SYMBOL by outcome (b'). Check 10 (lookup by spelling). Steward-filed per COORDINATION section 2."
---

# Retire the unreachable BoundedNat recognizer

## Objective

No native classifier keys on the spelling `::BufferSpan::` or
`::buffer_nat_to_int`, because the only one that did is gone.

## Settled inputs (Architect `evt_34jjncjpjkt6b`; D0 ruling `evt_6jz92x1j08wd4`)

- **The recognizer.** `is_exact_bounded_nat_to_int`
  (planning/static_transition/responses.rs:2291) matches a
  `contains("::BufferSpan::")` case with 3 binders, the `::Nat::Zero` and
  `::Nat::Suc` suffixes, and a recursive `::buffer_nat_to_int` callee. On a
  match, :2427 emits `StaticResponseEffectInput::BoundedNatToInt`
  (variant at :128). Its one consumer is lowering/units.rs:3109.
- **It fires, on the prelude's own freeze path.** The D0 recorded 205
  hits in `rt_escape_second_resource_native` (`evt_40ykd4k8pxm35`): the
  prelude's `freeze` -> `spanBytes` -> `buffer_span_length`. Each of the
  four R2 tests lowers the arm once. The earlier zero came from fixtures
  that never call `freeze`.
- **Deleting it is safe.** With the recognizer forced off, the suite keeps
  the same 10/10 results and every build succeeds. On a same-buffer
  freeze, the general path returns the interpreter's 6 bytes.
- **The existing suite cannot observe the value.** Every R2 row ends in
  InvalidBounds, so forcing the lowered length to 1 leaves it green. A
  same-buffer control row is needed.
- **No identity repair.** Re-keying on a prelude decl symbol would keep a
  second lowering for a computation the general path already performs.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

Add the same-buffer positive-control row, then delete the recognizer, the
variant and its lowering arm. Build in place at base `fd3b9262b`.

## Acceptance

- **AC-1 (census).** Report the recognizer's hit count per suite, with
  the same instrument, over `px8f_buffer_native`,
  `rt_escape_second_resource_native`, `rt_scalar_feedback_bound_process`
  and every suite that loads either `rt_nat_fanout_*` fixture.
- **AC-1b (positive control).** The Architect's
  `r2_same_buffer_freeze_returns_exact_span_bytes`
  (`evt_6jz92x1j08wd4`), in `rt_escape_second_resource_native.rs`,
  asserts native and interpreter both return exactly `b"held r"`. It is
  green on base and after the deletion.
- **AC-1c (mutation, QA, on base before the deletion).** Replacing
  `value` with `iconst(types::I64, 1)` in the `BoundedNatToInt` arm
  reddens AC-1b, with native returning 1 byte. Restore it byte-identically.
- **AC-2.** After the deletion, every AC-1 suite has the same per-test
  results.
- **AC-3.** `git grep` in `crates/` finds no `BoundedNatToInt`, no
  `ends_with("::buffer_nat_to_int")` and no `contains("::BufferSpan::")`.

## Stop conditions

- AC-1b is not green on base, or after the deletion its bytes differ from
  the interpreter's or from `b"held r"`.
- An AC-1 suite changes a per-test result or a build outcome after the
  deletion.
- The base mutant leaves AC-1b green.

## Hard-stop inventory

```text
1. spelling-keyed classifier ruled unreachable from a fixture population that never called the prelude op reaching it (freeze/spanBytes -> buffer_span_length) — keyed on fixture population, not on the op inventory
```

## Closeout

Merged `095aecb61` from exact `53e0c19ff` (PR #4701). Verify QA
`evt_2x7vvthkwn68e`; Architect `evt_ce0f83xfmnc2` with Decision
`dec_p79zp9wcfm5c`.

- The `BoundedNatToInt` input, `is_exact_bounded_nat_to_int` with its
  spelling probes, and its lowering arm are deleted. Every argument takes
  the general `OperationArgument` lowering. No shared helper is orphaned.
- `r2_same_buffer_freeze_returns_exact_span_bytes` asserts native and
  interpreter `b"held r"`. The base length mutant reddens it.
- The AC-1 census counted 0, 251 and 0 recognizer fires, a population and
  not a gate. Per-test results match the base.
- The verify ring moves to `LANG-ELAB-RECURSIVE-FRAME-BUDGET`.
