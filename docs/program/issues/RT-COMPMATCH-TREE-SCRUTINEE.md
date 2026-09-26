---
id: RT-COMPMATCH-TREE-SCRUTINEE
title: "Clear the next ignored L1 rows: re-measure the first refusal of every remaining unowned ignored runtime row at the increment-2 base, then repair the refusal the most rows share"
status: ready
owner: runtime
size: M
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Operator L1 directive 2026-09-17 (clear the ignored tests; top priority) and ruling 2026-09-26 (ds5b to L1). Reframed by the Steward 2026-09-26 after RT-SELECTED-PENDING-CALL-BUILD increment 2 landed at 912c44cf4, which changed the response-owner machinery several row labels name. The prior D0-D2 for rt_span_prov row 14 landed at 03976d2ac and cleared no row; that analysis is superseded and lives in git history. The id is kept because other artifacts cite it. Steward-filed per COORDINATION section 2."
---

# Clear the next ignored L1 rows

## Objective

Un-ignore the largest group of runtime rows that one repair can clear.

## Settled inputs -- to re-measure at `912c44cf4`

- **The rows.** Every owning node below is merged or closed, so these rows
  have no live owner. Their labels were measured before increment 2 and
  may be stale:
  - `crates/ken-cli/tests/px7f_resource_native.rs`:
    `linked_public_right_denial_preserves_exact_masks`,
    `linked_public_second_release_is_closed_and_the_handle_closes_once`
    (label: a static response owner's `require_i64(ret_tag, ...)`).
  - `px7n_nested_computational_eliminator.rs`:
    `nested_ok_payload_reaches_both_real_executors`,
    `nested_err_payload_reaches_both_real_executors` (label: duplicated
    host response block).
  - `rt_escape_second_resource_native.rs`:
    `escaped_resource_used_by_fanning_host_op_matches_interpreter`,
    `escaped_buffer_used_by_fanning_host_op_matches_interpreter`,
    `nat_fanout_escaped_resource_matches_interpreter`,
    `r2_cross_buffer_freeze_fails_closed_with_invalid_bounds` (labels:
    duplicated response block, carried site operand, process exit status).
  - `rt_span_prov_native.rs:356` `sp_a_foreign_span_freeze_...` (label:
    `StaticResponseDeferred` enters only its exact response owner).
  - `crates/ken-elaborator/tests/ds5b_dependent_match_refinement_acceptance.rs:376`
    (ken-interp has no `Term::J` reduction arm; operator 2026-09-26).
- **Out of scope:** `px7m` dynamic err (it needs the future response-owner
  environment extension, Architect `evt_2spyd3965e84m`), the parked
  `px8ta` bracket row, and the `px8ds` focused-cost row, which is ignored
  by design.

Treat every label as a prediction. If the landed code contradicts a settled
input, stop and report the mismatch.

## Deliverable

One repair, ruled by the Architect, that un-ignores every row sharing the
chosen first refusal, with no other row changing colour.

## Acceptance

- **AC-0 (D0, before any repair).** Run each row above at `912c44cf4` and
  post, in the WP thread, the verbatim first refusal and whether its label
  agrees. Group the rows by first refusal. The leader proposes the group
  to repair, preferring the one with the most rows, and the Architect rules
  on it and on the repair's shape. The table goes in the handoff, not in a
  document.
- **AC-1.** Every row in the chosen group runs green, un-ignored, on the
  engines it asserts. A differential row needs both engines to agree.
- **AC-2 (control).** A mutation that removes the repair re-reddens each
  un-ignored row with its AC-0 refusal, then is restored.
- **AC-3.** Rows outside the group keep their AC-0 refusal; relabel any
  whose label AC-0 showed to be stale, with the base SHA.
- **AC-4.** Targeted builds and suites only, through `scripts/ken-cargo`:
  the rows' suites and the unit tests of the module the repair touches.
  No-regression means green in CI (operator 2026-09-26: no full local
  runs).

## Stop conditions

- Any kernel, `trusted_base()` or spec change (an operator question).
- A row in the chosen group needs a second, independent repair: report it,
  and land the group that one repair clears.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.
