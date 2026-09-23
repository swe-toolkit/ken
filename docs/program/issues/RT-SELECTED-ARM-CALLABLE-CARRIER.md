---
id: RT-SELECTED-ARM-CALLABLE-CARRIER
title: "build a checked cross-boundary callable-and-operand carrier that binds a runtime-selected source arm to its exact recursive-unit/context call target and its own activation-local worker and continuation-capture runs, validated at consumption, so the four px7l/px7m rows can lower natively without admitting a nonselected sibling, inventing entry-ABI membership, or moving effects"
status: ready
owner: runtime
size: L
gate: none
tier: T1
depends_on: [RT-ARM-SCOPED-CONSTRUCTED-FRAME]
blocks: []
github: null
origin: "Operator authorized 2026-09-23 (concurred with Steward recommendation evt_46d36ha5k2943) after RT-ARM-SCOPED-CONSTRUCTED-FRAME closed on its D1 STOP. Contract from Architect ruling evt_52fhyanq4p78r. Steward-filed per COORDINATION section 2."
---

# The selected arm's call target and captures must cross the join

## Settled inputs -- Architect `evt_52fhyanq4p78r`, D0 at `bbe57c25f`

- Rows: the two in `crates/ken-cli/tests/px7l_checked_host_recursive_bind.rs`
  and the two in `crates/ken-cli/tests/px7m_hostresult_computational_match.rs`.
  All four still refuse at `L1`
  (`recursive_position_captures_all_planner_recoverable`), with zero
  retargets. The per-alternative D0 record is in the runtime thread for
  `RT-ARM-SCOPED-CONSTRUCTED-FRAME`.
- Each fixture's one materialized frame is keyed to a sibling body
  (343/362/332/341), while the reachable `L1` query needs 322/347/369/378.
  In dynamic-err the two bodies build different pending `Vis` values.
- `core.rs::resolve_recursive_unit_body` (`~13633-13706`) runs `L2` over all
  matching alternatives before the dynamic choice. The carried consumer
  (`core.rs:14370-14510`) stores one `recursive_unit_body` per recursor, and
  `:3103-3128` / `:16121-16175` emit the call only when that one body is
  `Some`.
- The producer arms (`core.rs:15070-15360`) join through
  `joins.rs::jump_planned_join_arm` (`~350-395`), which carries a word, not a
  callable plus operand runs. `FunctionLocalRefs::constructed_context_frame`
  is one `Option`, consumed at `calls.rs:988-1050`. `StaticWorker` cannot
  cross the join.
- **`L2` stays.** `agreeing_recursive_body_unit` (`core.rs:1230`) is correct
  for one-unit paths. On the new boundary the check is instead
  `selected arm <-> exact unit <-> matching frame`.

## Deliverables

- **D1, sketch to the Architect before code.** Name the carrier and where it
  is produced, crosses the join and is consumed. Show how it:
  - binds the selected source arm or construct and the exact unit/context
    target to that arm's own separately ordered worker-capture and
    continuation-capture runs;
  - validates source -> planner -> call identity and the declared header at
    consumption;
  - preserves per-activation freshness, re-entrancy and effect timing;
  - refuses absent, duplicate, nonselected or ambiguous entries.
- **D2, only after the Architect approves D1.** Build it. Un-ignore each row
  it clears; relabel each row it does not clear to its measured stop. Correct
  the stale owner comment on `predeclared_entry_frame_slot`
  (`planning/static_transition/continuations.rs:~4334`, "`D4b` owns making
  such a value capturable") to name this node.

## Acceptance criteria

- **AC-1.** `L1`, `L2`, `L3` and `calls.rs::gather_cannot_serve` are
  unchanged for every path that does not use the carrier. Controls show a
  nonselected sibling's entry, a duplicate and a missing entry each refused
  at the selected call.
- **AC-2.** Every un-ignored row passes differentially
  (`assert_native_matches_interpreter` plus its own assertions), including
  effect order. Native-only green does not count.
- **AC-3.** Targeted builds only, through `scripts/ken-cargo`. No-regression
  means green in CI.

## Stop conditions

- **Not authorized:** a shared mutable `Option`, a map of origins,
  rendered-body equality, a fabricated `ProducerLocal` `EntryAbi` claim,
  moving the call before the join (that needs its own effect-order and
  one-call proof), bypassing `L1`/`L2`/`L3`, or repurposing the bracket
  control-region IR.
- If the carrier cannot preserve effect timing or call multiplicity, STOP and
  return the measurement to the Steward.
- **Held work:** never move `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the
  child-2 checkpoint, and do not edit `planning/static_transition/responses.rs`.
