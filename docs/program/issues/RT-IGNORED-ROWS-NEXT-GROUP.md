---
id: RT-IGNORED-ROWS-NEXT-GROUP
title: "Clear the next ignored L1 rows: re-measure the first refusal of every remaining unowned ignored runtime row once the owner return protocol lands, then repair the refusal the most rows share"
status: ready
owner: runtime
size: M
gate: architect
tier: T1
depends_on: [RT-OWNER-VIS-RETURN-PROTOCOL]
blocks: []
github: null
origin: "Operator L1 directive 2026-09-17 (clear the ignored tests; top priority). Successor to RT-COMPMATCH-TREE-SCRUTINEE, closed at e41f7589e, for the rows neither it nor RT-OWNER-VIS-RETURN-PROTOCOL owns. Steward-filed per COORDINATION section 2."
---

# Clear the next ignored L1 rows

## Objective

Un-ignore the largest group of the remaining runtime rows that one repair
can clear.

## Settled inputs -- to re-measure at the landed owner return protocol

- **The rows** (labels from `RT-COMPMATCH-TREE-SCRUTINEE` AC-0 and its
  inventory line 1; predictions only):
  - `crates/ken-cli/tests/px7n_nested_computational_eliminator.rs`:
    `nested_ok_payload_reaches_both_real_executors`,
    `nested_err_payload_reaches_both_real_executors` (duplicated host
    response block);
  - `rt_escape_second_resource_native.rs`:
    `escaped_resource_used_by_fanning_host_op_matches_interpreter`,
    `escaped_buffer_used_by_fanning_host_op_matches_interpreter`,
    `nat_fanout_escaped_resource_matches_interpreter` (duplicated response
    block, carried site operand, process exit status);
  - `rt_span_prov_native.rs:356` `sp_a_foreign_span_freeze_...` (the
    response-owner coverage gate, `units.rs:6823`: the selected caller has
    no candidate disposition).
  - `crates/ken-elaborator/src/compiler_driver.rs:5409`
    `gate_4a_preparation_and_full_build_are_one_transaction` ("no green
    fixture: recursive source stops at RT-CLOSURE-BOUNDARY-LANE"). Its named
    node is merged, so the row has no other owner; re-measure it here.
- **Out of scope:** the three Ret-tag rows (`RT-OWNER-VIS-RETURN-PROTOCOL`),
  `px7m` dynamic err (Architect `evt_2spyd3965e84m`), the parked `px8ta`
  bracket row, and `px8ds`, which is ignored by design.
- The owner return protocol changes the machinery these labels name, so
  every label is re-measured on its landed tree before a group is chosen.

## Deliverable

One repair, ruled by the Architect, that un-ignores every row sharing the
chosen first refusal, with no other row changing colour.

## Acceptance

- **AC-0 (D0, before any repair).** Run each row on the landed protocol and
  post the verbatim first refusal and whether its label agrees. Group by
  first refusal; the leader proposes the largest group, and the Architect
  rules on it and on the repair's shape.
- **AC-1.** Every row in the group runs green, un-ignored, on the engines it
  asserts; a differential row needs both engines to agree.
- **AC-2 (control).** Removing the repair re-reddens each un-ignored row with
  its AC-0 refusal, then it is restored.
- **AC-3.** Rows outside the group keep their AC-0 refusal. Targeted suites
  only, through `scripts/ken-cargo`; no-regression means green in CI.

## Stop conditions

- Any kernel, `trusted_base()` or spec change (an operator question).
- A row in the group needs a second, independent repair: land the rows the
  one repair clears.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.
