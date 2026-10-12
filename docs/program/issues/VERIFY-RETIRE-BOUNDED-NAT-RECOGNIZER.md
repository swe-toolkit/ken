---
id: VERIFY-RETIRE-BOUNDED-NAT-RECOGNIZER
title: "The static-response BoundedNat recognizer keys a declaration and a constructor family by spelling and never fires: no prelude fixture reaches it and no test names it. Remove it so every candidate takes the general OperationArgument path"
status: ready
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

## Settled inputs (Architect `evt_34jjncjpjkt6b`, at `874e9f7bb`)

- **The recognizer.** `is_exact_bounded_nat_to_int`
  (planning/static_transition/responses.rs:2291) matches a
  `contains("::BufferSpan::")` case with 3 binders, the `::Nat::Zero` and
  `::Nat::Suc` suffixes, and a recursive `::buffer_nat_to_int` callee. On a
  match, :2427 emits `StaticResponseEffectInput::BoundedNatToInt`
  (variant at :128). Its one consumer is lowering/units.rs:3109.
- **It never fires.** A scratch instrument in the true branch at :2427 gave
  zero fires on six checked prelude fixtures, including the green
  `px8f_buffer_native
  write_all_classifies_mixed_specialized_and_deferred_responses`. The
  prelude's closest candidates match the span on `Var(4)` or `Var(2)`,
  and the recognizer requires `Var(0)`.
- **Nothing tests it.** `git grep` finds no test naming the recognizer or
  `BoundedNatToInt`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

D0 first: the same instrument, run over the native suites that reach
`static_response_effect_environment`, confirms zero fires on the current
base. Then delete the recognizer, the variant and its lowering arm.

## Acceptance

- **AC-1.** The D0 names each suite run and its fire count, all zero.
- **AC-2.** After the deletion, every suite the D0 ran is green with the
  same per-test results. That includes `px8f_buffer_native` and the
  suites that load the two `rt_nat_fanout_*` resource fixtures.
- **AC-3.** `git grep` in `crates/` finds no `BoundedNatToInt`, no
  `ends_with("::buffer_nat_to_int")` and no `contains("::BufferSpan::")`.

## Stop conditions

- The D0 records a fire: stop with the fixture. The recognizer is then
  reachable, and the Architect rules the identity repair instead.
- Removal changes a native result or a build outcome in any run suite.
