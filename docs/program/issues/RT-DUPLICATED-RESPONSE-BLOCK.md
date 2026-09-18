---
id: RT-DUPLICATED-RESPONSE-BLOCK
title: "Four ignored rows stop at `two host response cases claim one operation constructor` because the plan carries the same host-response block twice, not because two cases compete: across three programs the colliding constructors ALL agree on their operation and their effect-origin deltas are a single constant. Locate where the duplication enters and decide whether the planner emits it or the collision check is measuring a legitimate shape."
status: ready
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-18, third repair node from the RT-IGNORED-FAILING-ROWS-INVENTORY ledger on operator directive 2026-09-15 'The other tests should be fixed.' RT-HOST-RESPONSE-ROUTE-KEY-COLLISION (landed ef11485dd, annotation-only) ruled BOTH of its own section-3 branches out and closed with AC-4 unreached, stating in its own words: 'the right repair addresses the duplication, not the key -- so relaxing the key is the wrong unit.' It named the duplication and did not own it. Measured by the Steward at origin/main 60df2cfd2: zero occurrences of that duplication as a subject in any docs/program/issues or docs/program/wp file. This node is that owner. Steward-filed per COORDINATION section 2."
---

# The refusal, and what the predecessor already settled

Four of the fourteen failing-ignored rows on `main` stop at one byte-identical
planner message:

    two host response cases claim one operation constructor

    crates/ken-runtime/src/cranelift_backend/planning/
      static_transition/responses.rs:1281

The producer is `host_response_routes`, which inserts into a
`BTreeMap<RuntimeSymbol, HostResponseRoute>` keyed on `case.constructor` alone
and errors on any second insert at the same key.

**`RT-HOST-RESPONSE-ROUTE-KEY-COLLISION` already decided this is not a key
problem, and the decision is landed evidence, not an opinion.** It built the
key-widening repair — deferring the refusal from construction to use — ran it,
and reverted it before commit. That measurement is its section 9.5 and it is
this node's most important fixed input: the deferral **splits the population
2/2 and closes neither half.**

# What this node is NOT allowed to redo

The predecessor's section 3 had two branches and **both are refuted**. Do not
re-open either:

- **DIFFERENT (fix the key)** — refuted. The colliding constructors do not
  disagree about their operation; across all three measured programs every
  colliding constructor agrees on its `operation`. There is no key too narrow
  to distinguish cases that never differ.
- **SAME (the tests are wrong)** — refuted. The four rows assert
  native/interpreter agreement on valid Ken programs and do not mention the
  colliding construct at all.

**AC-4 of that node was recorded "not reached: no key change is landed, so
there is no widened key to justify."** That is the seam this node enters
through, and it is why the unit of repair moved.

# The evidence that names duplication rather than competition

Per-plan censuses, each run to completion and counted by the probe rather than
by grep:

| program | colliding constructors | agree on operation | effect-origin deltas |
|---|---|---|---|
| `px7n-nested-computational-eliminator` | 29 | 29 of 29 | one distinct value, 365 |
| `rt_escape_escape_file_then_readat` | 29 | 29 of 29 | one distinct value, 317 |
| `rt_escape_nat_fanout_escaped` | 29 | 29 of 29 | one distinct value, 317 |

**A constant offset across an entire set is a duplicated block, not 29
competing claims.** Twenty-nine independent collisions would not produce one
delta; they would produce a spread. The two `rt_escape` programs are DIFFERENT
Ken programs measured separately, and they agree in shape without sharing a
measurement — which is corroboration, not one number counted twice.

# Why deferral does not close the rows, and the split it produces

- **`px7n:149`, `px7n:170` — the collision clears and a SECOND refusal is
  underneath.** They then fail with `OrientedSubcontinuationPlanV1: checked
  Runtime frame marker was consumed more than once`, which is
  `[[RT-FRAME-MARKER-ONCE]]` (`draft`). Their original labels were never stale;
  they were SHADOWED by a newer refusal stacked in front.
- **`rt_escape:653`, `rt_escape:713` — unchanged under deferral.** The
  constructor each row SELECTS is `FSOp::ctor_543` qualified by its own
  program, and that selected constructor is itself one of that plan's 29
  counted collisions. So the collision is not merely adjacent to these rows'
  refusal; it is the thing selecting.

⇒ **Resolving the duplication is necessary for all four and sufficient for
none.** This node owns the duplication. It does not own
`[[RT-FRAME-MARKER-ONCE]]`, and it must not absorb it.

# The discriminating control that already exists

`S6` (`escaped_buffer_used_by_fanning_host_op_matches_interpreter`) lives in
the same file as two of these rows, **gets PAST response-route construction**,
and fails elsewhere — on source-specific inheritances disagreeing on their
typed consumer projection. `S8` likewise emits no collision at all.

**Co-location is not co-causation, and `S6` is the sharper control because it
shares a file with the population and does not share the cause.** Any account
of the duplication that would also predict a collision in `S6` is refuted on
arrival.

# The open question

Two readings survive the evidence, and this node exists to decide between them
rather than to assume one:

1. **The planner emits the block twice.** The same host-response block is
   planned or walked twice, at a fixed origin offset, so `source_occurrences`
   presents one program's response cases to `host_response_routes` more than
   once. The collision check is then reporting a real planner defect
   faithfully, and the repair is upstream of the check.
2. **The duplication is legitimate and the check is the wrong instrument.**
   The shape is intended — the same constructor genuinely appears under two
   occurrences that both belong in the plan — and a construction-time
   uniqueness assertion over `case.constructor` is simply not a property the
   plan was ever required to have.

**These are not ranked here, and the frame does not prefer one.** Reading (1)
is where the phrase "duplicated block" points, and that phrasing came from a
census of deltas rather than from reading the producer — so it is a
description of the symptom, not yet a diagnosis of the cause. Establishing
which reading holds is deliverable D0, and it is the whole of the node's first
turn.
