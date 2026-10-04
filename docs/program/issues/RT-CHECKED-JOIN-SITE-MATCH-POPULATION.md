---
id: RT-CHECKED-JOIN-SITE-MATCH-POPULATION
title: "record_match records no source Match in the measured native corpus, so no Match with a checked scalar result is wrapped in a CheckedJoinSite. Find which branch drops them and wrap every scalar-result Match"
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

# Every scalar-result Match gets its CheckedJoinSite

## Objective

Each source `Match` whose checked result is `Int` or `Bool` is wrapped in a
`RuntimeExpr::CheckedJoinSite` whose plan site records that answer kind, with
no new runtime refusal.

## Settled inputs (Architect `evt_4mp5dtzn0f5rb`, D1 at `6b01ada58`)

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

## Deliverable

1. **AC-0, measure only, on current main.** Run the six default D1 targets
   and the Nat ignored test. Log every `record_match` call with owner, path
   and outcome (`NonConstantMotive`, `NoHead`, `HeadNotAnswer(head)`,
   `Recorded(kind)`), and count Match lowerings that reach neither call site.
   Report the distribution, and identify source Match 1289's checked Match
   (the Nat match on `buffer_span_budget`) and its outcome.
2. **The repair the dominant outcome selects** (Architect rules at AC-0):
   - (iii) with the head spelled `Bool`/`Int` but compared unequal: key the
     comparison on checked identity at every answer symbol (S);
   - (iv) a bypassing path: route it through `record_match` (M);
   - (i) dependent motives: return to the Architect for a design ruling.

## Acceptance

- **AC-1.** Re-running the D1 join shows a `CheckedJoinSite` with the
  matching answer kind around every scalar-result Match among the 268.
- **AC-2.** Full runtime `rt_*` parity at 4 threads introduces no new
  refusal against the D1 census, and `rt_parity_native` stays 186/186.
- **AC-3 (falsifier).** Reverting the repair returns the D1 outcome
  distribution.

## Stop conditions

- The dominant cause is (i), or the repair needs a kernel, trust or spec
  change.
- A consumer refuses the widened population with no repair inside the named
  consumer arms: stop to the Architect.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.
