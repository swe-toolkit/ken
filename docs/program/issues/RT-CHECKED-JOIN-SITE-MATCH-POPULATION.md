---
id: RT-CHECKED-JOIN-SITE-MATCH-POPULATION
title: "The NativeJoinPlanV1 Match-site plane is dormant in production: its producer, planner reach and consumer were exercised only by fixtures. Measure every remaining layer and whether demoting every SpecializedOnly Match to CarrierWord lowers correctly, so one ruling chooses activation or demotion"
status: active
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: [RT-JOIN-SCALAR-PAIR-NONSCALAR-RESULT]
github: null
origin: "Architect ruling evt_4mp5dtzn0f5rb on the RT-JOIN-SCALAR-PAIR-NONSCALAR-RESULT D1 frame stop (runtime-leader evt_74k9ej0zqr505): the scalar-pair repair needs a CheckedJoinSite around each scalar-result Match, and D1 measured none. Separate node because it changes the population of an erasure-produced artifact that every runtime plane consumes. Steward-filed per COORDINATION section 2."
---

# Activate or demote the dormant Match-site plane

## Objective

One measured pass shows whether the 268 scalar merges need the checked
Match-site plane (activation, α) or lower correctly as `CarrierWord`
(demotion, β), so SCALAR-PAIR is repaired under one ruling and not stop by
stop.

## Original settled inputs (Architect `evt_4mp5dtzn0f5rb`, D1 at `6b01ada58`)

- **The gap.** D1 (`/workspaces/ken/local/rt-scalar-d1/`) saw 252 compiles
  and 252 ANSWER sites, one per compile, all `record_root_exit_answer`
  (`erasure.rs:1233`, `ExitCode`). There were zero Match sites and zero
  WRAP visits. The 268 Ok scalar merges are all unwrapped source Matches.
  Only erasure's unit tests (`test_answer_symbols`, `erasure.rs:7156`) reach
  the wrapped path.
- **The producer.** `record_match` (`erasure.rs:1258`) returns `None` on
  three branches: (i) `checked_constant_motive_result_type` is `None`; (ii)
  `checked_type_head_symbol` is `None`; (iii) the head is not one of
  `answer_symbols.{int, bool_, structural_nat, exit_code}`. A fourth cause
  is possible: (iv) a Match lowered through neither call site (`:2950`
  `lower_body_term`, `:3249` root path).
- **The consumers** of a wider `CheckedJoinSite` population:
  `aggregates.rs:8124`, `selected_pending_calls.rs:867`, `continuations.rs`,
  `closure.rs`, `responses.rs`, and the evaluator, validator and
  differential arms.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Recut (Architect frame stop `evt_73mrdr69bpqgk`)

AC-0 (`evt_37bbprs1gwajn`) found cause (i-a): elaborated motives are a bare
`lam`, and the result-type helper accepted only the test-minted ascribed
shape. The ruled helper fix is built at `a7d46d6f2` on this WP branch (base
`e771f7e7d`), with a unit pin and an elaborated Bool pipeline pin. AC-1 then
stopped (`evt_7qxp95yyc824w`).

- `rt_nested_ih_native_realization`, green on `e771f7e7d`, refuses with
  `checked join occurrence marker was not consumed`. Three new WRAPs, all
  `ExitCode`.
- Nat's first failing origin, now 1293, has no WRAP. The AC-0 attribution
  of Nat to origin 1289 is refuted. No checked identity links a source
  Match to that runtime join.

Measured cause, on main `14fcb44b2`:

- The `CheckedJoinSite` arm (`core.rs:16207`) refuses an unconsumed marker.
- Only `planned_join_site_for_frame` (`mod.rs:13848`) consumes it, and its
  sole production caller is the active scalar cut (`source.rs:6190`,
  `:6206`).
- Neither the ordinary route (`lower_runtime_match_expr`) nor the
  computational route consumes it.

`a7d46d6f2` stays as the measurement base. No repair is authorized on it.

## Deliverable

**AC-0, measure only, one pass**: the six default D1 targets plus the Nat
test, at 4 threads where applicable. Restore all probes byte-identically.

1. **M1, plane inventory, on `a7d46d6f2`.** For every WRAP site, log:
   - its `site_id` and `answer_kind`;
   - its body's lowering route: ordinary, computational, consumed by the
     active cut, or body not a Match;
   - whether the recorded `runtime_frame_fingerprint` equals the one
     computed at lowering time from the body's cases and default;
   - whether it was consumed.

   Disable only the two not-consumed refusals (`core.rs:16216` and
   `require_complete_join_plan_consumption`, `mod.rs:13937`) so the run
   reaches every site, and log what they would have refused. Report counts
   per route and per fingerprint outcome.
2. **M2, D1 re-join.** Of the 268 D1 Ok scalar merges, report how many sit
   under a WRAP with an `Int` or `Bool` answer kind, and by which route.
   Also give the WRAP state and checked result head of Nat's first failing
   origin, if one is found.
3. **M3, demote-all, on main.** Change only `joins_traps.rs:533`, so that
   every SpecializedOnly Match plans `CarrierWord`. Run the six targets,
   `rt_parity_native` and Nat. Report every refusal with its message, and
   whether Nat's dynamic-Match refusal is gone.

The Architect then rules:

- **(β)** M3 is green apart from performance: the repair is SCALAR-PAIR at
  size S with no checked plane. This node closes, or becomes a separate
  plane-activation or performance WP.
- **(α)** M3 refuses somewhere: the ordinary and computational routes
  consume the marker through `planned_join_site_for_frame`, and the merge
  kind comes from `site.answer_kind`. That repair is sized from M1's counts.

## Acceptance

- AC-0 is reported in the WP thread with the M1 counts, the M2 join and the
  M3 refusal list. Each count names the log it came from.
- The repair's acceptance is set by the ruling.
  - Under (β), it is SCALAR-PAIR's: the 268 merges stay Ok, Nat passes its
    256 MiB test, and `rt_parity_native` is 186/186.
  - Under (α), it is this node's former AC-1 to AC-3, with no new refusal.

## Stop conditions

- The measurement needs a product change beyond the probes and the two
  disabled refusals: stop to the Architect.
- Any repair needs a kernel, trust or spec change.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.

## Hard-stop inventory (§1b)

§1a count: 1 (Architect `evt_73mrdr69bpqgk` on stop `evt_7qxp95yyc824w`).
The shared predicate is that the NativeJoinPlanV1 Match-site plane is dormant
in production and every layer was exercised only by fixtures.

1. CheckedJoinSite Match marker not consumed by ordinary Match lowering —
   keyed on the Match lowering route.
