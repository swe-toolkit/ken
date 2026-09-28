---
id: RT-ROOT-EXIT-PROJECTION-KEYED-ON-JOIN
title: "The native scalar merge projects a dynamic join arm into a process exit status only when that join's own answer is ExitCode and the arm is exit_success or exit_failure; any other value falls to the existing refusal instead of collapsing to the -2 sentinel under the lowering-wide root-exit flag"
status: ready
owner: runtime
size: S
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Architect carry 2026-09-28 in the RT-NATIVE-TREE-MATCH-RUNTIME-SCRUTINEE repair ruling (evt_4zd7rsxhmnr4j), on the runtime-implementer's join probe (evt_6pdpb04atj0cc). Steward-filed per COORDINATION section 2."
---

# Root-exit projection keyed on the join

## Objective

A native join projects its arms into a process exit status only when it is an
exit-status join, and only for the two exit constructors.

## Settled inputs (Architect `evt_4zd7rsxhmnr4j`, read at `ce2276225`)

- **The arm.** `crates/ken-runtime/src/cranelift_backend/lowering/joins.rs:2647`
  (`lowered if checked_root_exit_representation`) projects any value that is
  not already a `ProcessExitStatus`, Int or Bool into `ScalarMergeKind::ExitCode`.
  It is keyed on a lowering-wide flag, not on the join's answer.
- **The sentinel.** `emit_process_exit_status` (`calls.rs:2698`) returns `-2`
  for a non-constructor, for `exit_success` with arguments, and for any
  constructor other than `exit_success` and `exit_failure`.
- **Measured reach.** The two `rt_escape` rows' inner joins (origins 1003 and
  1429) have `Option::Some` and `Option::None` arms. They reached this arm with
  `root_exit_flag=true` and `active_plan_answer=None`, so Option identity
  collapsed to `-2` (probe `evt_6pdpb04atj0cc`). After
  `RT-NATIVE-TREE-MATCH-RUNTIME-SCRUTINEE` those rows compose instead, so they
  cannot witness this repair.
- **Unmeasured:** which currently green consumer, if any, passes on the `-2`
  sentinel.

## Deliverable

The `:2647` arm fires only when the join's own planned answer is ExitCode and
the arm is `exit_success` or `exit_failure`. Every other value reaches the
existing "dynamic arms must produce scalar Int or Bool values" refusal.

## Acceptance

- **AC-0 (census, then ruling; no build).** Over the targeted runtime and
  ken-cli suites, count the `:2647` projections by input class and join answer,
  and name each test that projects a non-exit value. The Architect rules the
  repair against that census before any edit.
- **AC-1.** A planner or lowering unit row where an Option-armed join under the
  root flag reaches the projection today now refuses with the existing message.
  A green exit-status join (Success and Failure n arms) keeps its exit codes.
- **AC-2 (control).** Reverting the repair re-admits the AC-1 row as `-2`. Every
  test the census names stays green, or it is reported with its verbatim
  refusal.

## Stop conditions

- Any kernel, `trusted_base()` or spec change (an operator question).
- A census consumer that needs the `-2` projection to stay green is a stop to
  the Architect with the test and its trace.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.
