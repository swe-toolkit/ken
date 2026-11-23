---
id: RT-NATIVE-METADATA-WIDE-INT
title: "On the native lane, a program that reads a FileMetadata time whose nanosecond value lies outside the boundary immediate Int domain, which includes every present-day timestamp, aborts with UnclassifiedRuntimeTrap -1, while the interpreter returns the exact Instant. ABI-S4's rows used only in-domain values or None. Rule from an AC-0 where the spill of a wide Int inside lower_file_metadata_option's nested Some payload fails, repair it, and pin every wide metadata field against the interpreter"
status: active
owner: language
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary finding evt_1bv5w8aw8g0rq on ABI-S4 at 097517427, measured on both lanes: a correctness defect in the landed product, outside every merged row. Steward-filed per COORDINATION section 2. L1b on the Language ring, after RT-NATIVE-BYTES-TO-LIST (the Verify ring is unseated, Steward evt_4m67y43b306fb)."
---

# Native metadata carries wide Int values

## Objective

On the native lane, every `FileMetadata` field that ABI-S4 reports carries
its exact value when that value lies outside the boundary immediate `Int`
domain. A program that consumes such a field agrees with the interpreter.
That includes every present-day file time.

## Settled inputs (main `097517427`)

- **The trap.** The Adversary set atime with `utimensat` on tmpfs and
  matched `Some instant ↦ MkInstant n ↦ eq_int n n` on both lanes.
  - At `2^55 - 1` and `-2^55` both lanes pass.
  - At `2^55` and `-2^55 - 1` the native lane aborts with
    `NativeRun(UnclassifiedRuntimeTrap { terminal_value: -1 })`. It also
    aborts at the current time, `1e8` s, `2^31` s, `9e9` s, `-1e9` s and
    `-2^31` s.
  - `1.5e19`, on the Big arm, passes.
  - Raising every arena region does not clear it.
  - When the same time is set but its payload is not matched, it passes.
- **The mechanism, as measured.** Values between `±2^55` and `2^63` take
  the spill arm `emit_carrier_spillable_immediate`
  (`crates/ken-runtime/src/cranelift_backend/lowering/aggregates.rs`
  ~`:2714`), bounded by `BOUNDARY_IMMEDIATE_INT_MAX`
  (`crates/ken-runtime-support/src/boundary_value.rs` ~`:91`). The `Instant`
  is built inside the `Some` alternative by `lower_file_metadata_option`
  (`lowering/effects.rs` ~`:2070`). A top-level `wall_now` `Instant` of the
  same magnitude crosses correctly, so the failure is specific to the
  nested shape. The `require` that trips is not isolated.
- **Why the gates missed it.**
  `abi_s4_returned_options_match_raw_statx_for_both_operations`
  (`crates/ken-verify/src/scenario.rs` ~`:4559`) sets mtime to 15 s, which
  is inside the domain. The procfs row consumes only `born`, which is
  `None`.
- **The fan-in, unmeasured.** The same carrier serves the four time
  fields, the three optional `Int` fields (inode, links, mode), and the
  top-level size, major and minor. An inode at or above `2^55`, as overlayfs
  xino numbers can be, likely traps the same way.

The Architect's AC-0 (`evt_4dy6fnsexd6hc`) corrects three of these
inputs without changing a row. The defect is in the consumer: the carried
`Int` arm of the scalar projection (`core/primitive.rs` ~`:158`) accepts
only an immediate word. The spill arm and the payload construction are
sound. Size traps at top level, so the nested shape is not the trigger.
`1.5e19` traps on the handle lane. AC-0 also rules the field fan-in (times,
size, and inode by injection; not links, mode, major or minor) and the
AC-1 rows. D1-E is `evt_3be5px88gdnzk`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The native lowering of wide `Int` values in metadata results, the parity
rows, and any shared spill path the AC-0 names. No change to spec 38's
metadata contract, the boundary immediate bound, or the interpreter lane.

## Acceptance

- **AC-0 (Architect, before code).** Isolate the `require` that traps, and
  name whether the defect is in `lower_file_metadata_option`'s payload
  construction or in the shared spill arm. If it is in the shared arm,
  enumerate the other nested-constructor callers that reach it, and rule
  whether they are in scope here. Rule each field of the fan-in above as
  reaching the defect or not.
- **AC-1.** Parity rows, interpreter against native, with a consumed
  payload and a known value:
  - a time at `2^55` and at `-2^55 - 1`;
  - a present-day time;
  - controls at `2^55 - 1` and `-2^55`;
  - one row per other field AC-0 rules as reaching the defect, at a value
    outside the domain. Where the host cannot set that value, AC-0 names
    the injection point.
- **AC-2.** Revert the repair: the `2^55` time row traps again on the native
  lane, and the controls pass. Restore it byte-identically.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The repair needs a change to the boundary immediate bound, the boundary
  value format, or the TCB.
- A field cannot reach a value outside the domain on any host, and AC-0
  has no injection point for it. Report it instead of dropping the row.

Checks: 4 fired. On `097517427` I read the four cited anchors:
`emit_carrier_spillable_immediate`, `BOUNDARY_IMMEDIATE_INT_MAX`,
`lower_file_metadata_option` and the ABI-S4 row. The row's only time is
15 s. 8 fired. Every merged ABI-S4 row passes in the failing
configuration, because none consumes an out-of-domain value. So AC-1 and
AC-2 key on values at the domain edge, with controls just inside it.
2 fired. The fan-in is the Adversary's ten fields. Only the times were
measured, so AC-0 rules on the rest.

## Symptom inventory

1. int_view's View output carries a limb pointer into the helper's own
   stack frame for a native-arena Small. Keyed on the storage lifetime of
   the native resolver's view slot, which is helper-local.
