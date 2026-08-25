---
id: RT-EXITCODE-FAILURE-PAYLOAD-TRANSPORT
title: "Execution-parity successor — give the ExitCode::Failure result payload a durable native transport so a checked program that crosses M3's effect seat does not trap `malformed ExitCode::Failure payload` at process exit (object_linker_packaging.rs:2223, native stub value -3)"
status: draft
owner: runtime
size: M
gate: none
depends_on: [RT-CARRIED-IH-DISPATCH-SITEOP]
blocks: []
github: null
origin: "Steward, 2026-08-25, from the Architect object-distinctness ruling (evt_317adj9ebfw86) on M3 WIP 3e9821c5d. M3's ConstructorTag/CarriedWord effect-seat crossing succeeds; px8ta HALF B then executes and traps at process exit on a malformed ExitCode::Failure payload — a DISTINCT execution-parity object in the process-exit/ExitCode native-trap family (value -3, beside RT-BORROWED-INPUT-CARRIER-DURABILITY value -1 and the closed RT-ENTRY-TRAP-254 value 254/-2). New sibling node, not a fold: RT-ENTRY-TRAP-254 is closed and specific to a different value, so it cannot own this live successor. Steward framing call per COORDINATION section 2; the dispatcher-bypass diagnostic (identical trap/trace with the entire M3 dispatcher removed) confirms M3's guard is off the causal path."
---

> # Execution-parity successor — gated behind M3's crossing (DRAFT stub)
>
> Node minted on the M3 accept-COMPLETE-for-object disposition
> (Steward evt_3v7t4qcp9m8gt). Full WP frame + release queue behind M3's landing;
> this stub records the grounded object so M3's re-pointed row has a real owner.

## Objective

A checked program that crosses M3's effect-seat boundary runs to process exit and
then TRAPS at the native C stub with `ken native trap: malformed ExitCode::Failure
payload` (object_linker_packaging.rs:2223, value -3). The `ExitCode::Failure`
result payload has no durable native transport across the process-exit boundary.
Give it one so the program's exit result is carried faithfully and the row runs
end-to-end.

This is the same execution-parity native-trap family as
[[RT-BORROWED-INPUT-CARRIER-DURABILITY]] (value -1) and the closed
[[RT-ENTRY-TRAP-254]] (value 254/-2), but a DISTINCT object: the ExitCode::Failure
payload transport, not borrowed-input durability and not the entry ExitCode trap.
The M4->M3 pattern again — the crossing succeeds and exposes a deeper, distinct
seam, not a defect of our own making.

## Fixed inputs (Architect evt_317adj9ebfw86, object DB `f0292222`)

- The trap is EXECUTION-layer, native C stub at
  `crates/ken-runtime/src/object_linker_packaging.rs:2223`:
  `value == -3 -> "ken native trap: malformed ExitCode::Failure payload"`, sitting
  beside `value == -1` (borrowed input) and `value == -2` (malformed entry
  ExitCode).
- Trigger row: `crates/ken-cli/tests/px8ta_oriented_subcontinuation.rs:372`
  (`px8ds_real_same_depth_path_runs_exact_edges`, HALF B). Trace: BufferAllocate +
  ResourceRelease, ZERO ConsoleIsTerminal — the program runs, then malforms its
  ExitCode::Failure payload at exit. Re-pointed to this node by M3's finalization.
- Dispatcher-bypass diagnostic (decisive): removing the entire M3
  carried-constructor dispatcher yields the IDENTICAL trap/trace, so M3's guard is
  not on the causal path — this is a distinct object, not unfinished M3.

## Sequencing

Draft, execution-parity family. Gated behind M3's crossing (depends_on). Whether it
later joins an execution-parity umbrella with
[[RT-BORROWED-INPUT-CARRIER-DURABILITY]] is a post-framing call. Full WP frame +
release queue behind M3's landing; the Architect reviews the WP at release.
