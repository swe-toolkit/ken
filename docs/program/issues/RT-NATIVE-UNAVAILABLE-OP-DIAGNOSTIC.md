---
id: RT-NATIVE-UNAVAILABLE-OP-DIAGNOSTIC
title: "When the host refuses an operation as unavailable inside a native process, the emitted code's require_i64 turns the status into -1 and the C starter prints \"malformed borrowed process input\". No product program reaches that refusal today: emission or the Process gate refuses first (Architect evt_59z2v4tgmnvs4). It is reached on the px8-ds-test-support census route, where PX10-NATIVE-SPAWN-PARITY Increment B works and where the BRANCH-ALT stop-11 misread happened. Carry the refusal to the process boundary as its own status and diagnostic naming the operation, without changing which operations are admitted"
status: merged
owner: runtime
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect ruling evt_4qb22p4991jmm on PX10-NATIVE-BRANCH-ALTERNATIVES stop 11: the native smoke's -1 was the host's deliberate dispatch gate, read as malformed input; filed by the Steward on request. L1 objective: the Linux ABI program. Placed before PX10-NATIVE-SPAWN-PARITY Increment B, whose native parity work meets the same -1. Steward-filed per COORDINATION section 2."
---

# An unavailable host operation says so

## Objective

When the host's unavailable-operation refusal reaches a native process,
by any route, the process exits nonzero with a diagnostic naming the
operation, distinct from every other trap. No product program reaches
the refusal today, so the fixture is built on the px8-ds-test-support
census route, and the candidate says so. The admitted operation set
does not change (Steward re-scope evt_19rfcye3rj7s4).

## Settled inputs (main `80525e894`)

- **The refusal.** `require_native_operation_v1`
  (`crates/ken-host/src/abi_v1.rs:820` to `:824`) returns
  `TerminalErrorV1::OperationUnavailable(operation)` from
  `HostOpV1::availability()` alone. It is the host's single native
  authority, and this WP does not change it.
- **The collapse.** `Lowering::require_i64`
  (`cranelift_backend/lowering/mod.rs:14294`) returns `-1` on any
  mismatch. The C starter (`object_linker_packaging.rs:2573`) prints
  "ken native trap: malformed borrowed process input" for any `-1`.
- **The fan-in.** The Architect counted 18 production sites that return
  `-1` (evt_29xbp91z77nap); only the host-status checks are in scope.
  The starter already reserves `-2`, `-3`, `-4` and the `0xff` low-byte
  token; `-7` is the capacity status.
- **Measured.** On `1a756ef69` the PX10 smoke artifact exits 1 with
  that stderr line and `effect_trace=[]`, because its first operation,
  ProcessPlanOpen, is not NativeTested (evt_4qb22p4991jmm).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

One candidate in which the host-status refusal for an unavailable
operation reaches the starter as its own status and prints its own
diagnostic, with the pins below.

## Acceptance

- **AC-0 (Architect).** Rule the carrier: which host-status check sites
  propagate the host's status instead of `-1`, the status value, and
  whether the diagnostic names the operation. Rule the sweep of
  existing tests that assert the old stderr line or exit status.
- **AC-1.** A native fixture on the census route calling one
  unpromoted operation exits nonzero and prints the ruled
  unavailable-operation diagnostic, not "malformed borrowed process
  input". A mutation restoring `-1` at the ruled site reddens it.
- **AC-2.** A control whose borrowed process input is malformed still
  prints the malformed-input line. A NativeTested-only fixture still
  exits 0 with an unchanged trace.
- **AC-3.** No operation's availability changes, and `trusted_base()`
  is unchanged.

## Stop conditions

- A change to availability, dispatch or any admission route.
- A kernel or trust change.

Checks: 2 fired: the `-1` has 18 production sources and one starter
reader, so AC-0 names the in-scope sites before any edit. 8 fired: the
unpromoted-operation fixture and the malformed-input control print the
same line on the base and must print different lines after.
