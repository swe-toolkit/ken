---
id: RT-PX8DS-SAME-DEPTH-P1-HANDLER
title: "px8ds_real_same_depth_path_runs_exact_edges is red at base: native refuses at ObjectEmission because the response-owner effect gate admits only a HostResult, while ConsoleIsTerminal answers a bare Bool. Key the gate on the operation's response shape and un-ignore the row"
status: active
owner: runtime
size: S
tier: T1
gate: architect
depends_on: [RT-NESTED-RELAY-VIS-NATIVE-SETTLEMENT]
blocks: []
github: null
origin: "Finding outside RT-NESTED-RELAY-VIS-NATIVE-SETTLEMENT (its frame, 'Finding outside this WP'; expected refusal evt_62wab4peq1htm). Serves the L1 objective, clearing the ignored runtime rows (operator 2026-09-17). Steward-filed per COORDINATION section 2."
---

# The same-depth px8ds path runs natively

## Objective

`px8ds_real_same_depth_path_runs_exact_edges`
(`crates/ken-cli/tests/px8ta_oriented_subcontinuation.rs` ~:453) is
un-ignored and passes on the false branch, as on the interpreter. The
response-owner effect gate admits each operation's own response shape.

## Settled inputs (Architect D0 `evt_1jp26fw1z8cw9` on `da506265f`)

- At base the row fails at `ObjectEmission` with "a specialized response
  owner did not materialize an exact HostResult". The interpreter exits
  0 with trace `[BufferAllocate, ConsoleIsTerminal, ResourceRelease]`.
- **The failing site is owner 0's own effect**, not relay 370's P1
  sources. `lower_static_response_effect` (`lowering/units.rs`) lowers
  row 0 (Vis 676, `ConsoleIsTerminal`) to `Specialized(Bool)`, because
  `lower_process_host_effect` (`lowering/effects.rs` ~:3946) answers an
  infallible ConsoleIsTerminal with a bare Bool. The gate admits only
  `Lowered::HostResult`. Force-admitting owner 0 leaves the failure
  byte-identical.
- **No planning refusal exists.** `relay_settlement.rs` only computes the
  admitted set, and an unsettled relay traps at its lowering consumer.
- **The route is measured.** The D0 probe changes two spots: the gate
  becomes `static_response_shape_matches(row.operation(), &response)`
  (ConsoleIsTerminal admits Bool, ClockWallNow stays refused, every
  other operation admits HostResult), and the ConsoleIsTerminal arm
  emits an endpoint instruction before returning Bool. With it the row
  passes with its existing assertions, both with owner 0
  `Excluded{Relay}` and force-admitted. Without the endpoint line the
  next refusal is "host response validation emitted no finished
  instruction". Probe: `notes/px8ds-p1-handler-probe-da506265f.patch`
  on `architect/work`.
- The row's entry in `.github/ignored-test-exemptions.toml` classes it
  `policy-cost` and calls it runnable, which is false while it is red.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The D0's two changes in `lowering/units.rs` and `lowering/effects.rs`,
the row un-ignored, and its exemption entry deleted.

## Acceptance

- **AC-1.** The row is un-ignored and passes on its 256 MiB thread, and
  its exemption entry is gone.
- **AC-2.** Controls, targeted:
  - `px8ta_oriented_subcontinuation`: the retired-flat and one-level
    controls pass; `public_two_three_level_brackets_finish_and_release_lifo`
    keeps its labelled depth-2 failure;
  - `px7l_checked_host_recursive_bind` 3/3;
  - `rt_selected_pending_call_admission` 7/7;
  - RELAY's 36-row roster unchanged.
- **AC-3 (mutation, QA).** Both are typed compile refusals, never a
  run-time trap:
  - (a) the ConsoleIsTerminal arm of `static_response_shape_matches`
    back to HostResult returns the row to the ObjectEmission refusal;
  - (b) deleting the endpoint instruction yields "host response
    validation emitted no finished instruction".

## Stop conditions

- The route needs a kernel, trust or spec change, or touches relay
  settlement, the planner or the verifier.
- Any RELAY roster row changes outcome.
- `ClockWallNow` or the Carried-HostResult arm changes behaviour.

## Finding outside this WP

AC-1 observes only the false branch, since stdout is not a terminal
under the harness. On the true branch, relay 370's P1 sources 647 and
600 have K bodies that exit only by static tail calls, so the
`ret_exits > 0` fallback in `deferred_response_handler_owner` excludes
them and owner 0 stays `Excluded{Relay}`. That is
`RT-P1-TAIL-CALL-EXIT-HANDLER`.
