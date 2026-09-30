---
id: RT-ROOT-EXIT-PROJECTION-KEYED-ON-JOIN
title: "Decode the root exit status once, at the root result boundary, from the checked answer type (ExitCode), whatever join representation delivers it; joins stop projecting, so an Option arm no longer collapses to -2 and an ExitCode constructor carried through a CarrierWord merge no longer fails the root guard"
status: active
owner: runtime
size: M
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Architect carry 2026-09-28 in the RT-NATIVE-TREE-MATCH-RUNTIME-SCRUTINEE repair ruling (evt_4zd7rsxhmnr4j), on the runtime-implementer's join probe (evt_6pdpb04atj0cc). Recut 2026-09-29 on the Architect's RT-FRAME-MARKER-ONCE B1 ruling (evt_17dvdhb4k3c57): the carrier-merge under-projection is the mirror of the scalar-merge over-projection. Steward-filed per COORDINATION section 2."
---

# Root-exit status decoded at the root boundary

## Objective

The root exit status is decoded at the root result boundary from the checked
answer type, ExitCode, whatever join representation delivers it. Joins do not
project into an exit status.

## Settled inputs

- **Over-projection at a scalar merge** (Architect `evt_4zd7rsxhmnr4j`, read
  at `ce2276225`).
  - `crates/ken-runtime/src/cranelift_backend/lowering/joins.rs:2647`
    (`lowered if checked_root_exit_representation`) projects any value that
    is not already a `ProcessExitStatus`, Int or Bool into
    `ScalarMergeKind::ExitCode`. It is keyed on a lowering-wide flag, not on
    the join's answer.
  - `emit_process_exit_status` (`calls.rs:2698`) returns `-2` for anything
    other than a well-formed `exit_success` or `exit_failure`.
  - The `rt_escape` inner joins (origins 1003 and 1429) collapsed Option
    identity to `-2` this way (probe `evt_6pdpb04atj0cc`). After
    `RT-NATIVE-TREE-MATCH-RUNTIME-SCRUTINEE` those rows compose instead, so
    they cannot witness this repair.
- **Under-projection at a carrier merge** (Architect `evt_17dvdhb4k3c57`,
  trace `evt_47d1vyr09mvnt`, on `RT-FRAME-MARKER-ONCE` WIP `5dc0e3e4a`).
  - In px7n Ok, both successors of a class merge deliver carrier words
    (`joins.rs:775-797`) to one root result-slot store (`units.rs:9471-9480`).
  - The word is the ExitCode constructor, materialized as a
    `PersistentGround` Constructor at `aggregates.rs:1885`.
  - The root guard at `units.rs:9389` demands `ImmediateExitStatus` (tag 2),
    sees tag 5, and returns -1. px7n Err fails the same way
    (`evt_2j3r1qnvjq052`).
  - The interpreter gives exit 0 for Ok and exit 7 for Err, each with two
    writes and a flush.
- **The shared predicate:** the projection is keyed on the join's
  representation, not on the root boundary.
- **Unmeasured:** which currently green consumer, if any, passes on the `-2`
  sentinel.

## Deliverable

One decode at the root result boundary, keyed on the checked answer type
ExitCode. It accepts an immediate status or a carried exit constructor and
maps each to its exit code. The join-level projection at `joins.rs:2647` is
removed, so a non-exit value at a scalar merge reaches the existing "dynamic
arms must produce scalar Int or Bool values" refusal.

## Acceptance

- **AC-0 (census, then ruling; no build).**
  - Over the targeted runtime and ken-cli suites, count the `:2647`
    projections by input class and join answer, and the root-guard
    decodes by tag. Name each test that projects a non-exit value.
  - The Architect rules the repair against that census before any edit.
- **AC-1.**
  - Both px7n rows pass un-ignored with native parity: stdout, exit code (0
    and 7) and effect count equal to the interpreter's.
  - An Option-armed join under the root flag now refuses with the existing
    message.
  - A green exit-status join (Success and Failure n arms) keeps its codes.
  - Third witness (Architect `evt_215bv807te16m`): the TREE-MATCH byte-1
    row, a carried ExitCode constructor delivered by a
    continuation-specialization result edge, passes un-ignored with native
    parity. It exists once TREE-MATCH lands, as an ignored row whose reason
    names this WP.
  - Closed ExitCode through a composed arm (Architect `evt_6ma0t6rfwg29s`):
    the fix covers ANY closed ExitCode (Success or `Failure n`) returned
    through a TREE-MATCH composed arm. The census adds the Adversary's (b)
    byte 1 (inner Success, the outer arm returns the closed `Failure 5`) and
    a discriminator: the same shape returning a runtime `Failure c`. If the
    discriminator still traps, stop to the Architect; it is a D1 lowering
    defect, not this WP.
- **AC-2 (control).**
  - Reverting the repair returns px7n to the `units.rs:9389` tag-5 refusal
    and re-admits the Option row as `-2`. The byte-1 row returns to the
    same tag-5 refusal at the root guard.
  - Every test the census names stays green, or it is reported with its
    verbatim refusal.

## Stop conditions

- Any kernel, `trusted_base()` or spec change (an operator question).
- A census consumer that needs the `-2` projection to stay green is a stop to
  the Architect with the test and its trace.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.
