---
id: RT-NATIVE-EXITCODE-FAILURE-OUT-OF-RANGE
title: "For ExitCode.Failure with a runtime payload that is negative or at least 2^64, the interpreter reports terminal_error MalformedHostAbiField. The native exit decoder instead tests the payload for zero without gating on whether narrowing succeeded, so the result depends on an output word the helper never wrote. Main reports the wrong trap line or none, and after RT-CLOCK it reports none. Report the malformed-Failure-payload trap natively, matching the interpreter"
status: ready
owner: runtime
size: M
tier: T1
gate: architect
depends_on: [RT-CLOCK-OP-NATIVE-PROMOTION]
blocks: []
github: null
origin: "Architect evt_1cwe6bhb979j2 (RT-CLOCK CI-respin review, 2026-11-26): pre-existing native defect measured by scratch D0 on b8bc69953 and 385e84aa5, handed to the Steward as a separate WP. L1 Linux ABI program, native parity. Steward-filed per COORDINATION section 2."
---

# Native exit reports an out-of-range Failure payload

## Objective

A native program that exits with `ExitCode.Failure n` behaves as the
interpreter does for every runtime `n`:

- When `n` is negative or at least 2^64, it exits with status 1 and its
  stderr carries the malformed-Failure-payload trap line, which the
  interpreter reports as `MalformedHostAbiField`.
- In-range payloads behave exactly as they do today.

## Settled inputs (Architect `evt_1cwe6bhb979j2`; anchors re-checked at release)

- **The decoder.** `lowering/calls.rs:3089-3113` narrows the payload with
  `narrow_native_int_u64`, then computes `is_zero = (value == 0)` without
  `valid_int`, and selects `1` on zero. The `-3` malformed result is
  gated on `valid_int`, but the zero arm is not, so a failed narrowing
  reads the output word.
- **The fan-in.** `narrow_native_int_u64` has 8 call sites. The other 7
  gate on `valid`: ConsoleRead limit, FsChangeMode mode, the capacity
  sites, MappingAllocate length, and the positioned operands.
- **RT-CLOCK's zero-init.** `lowering/effects.rs:1491-1494` zeroes the
  output word before the helper call. So after CLOCK, a failed narrowing
  reads 0, and the program exits with status 1 and empty stderr.
- **The measurement.** The payloads were `0 - |argv0|`, `2^64 + |argv0|`,
  a runtime `0` and a runtime `3`.

  | Build | Negative | 2^64 | 0 | 3 |
  |---|---|---|---|---|
  | main `b8bc69953` | status 1, "malformed borrowed process input" | status 1, empty stderr | 1 | 3 |
  | CLOCK `385e84aa5` | status 1, empty stderr | status 1, empty stderr | 1 | 3 |
- **The gate is not enough.** Adding `is_zero = band(valid_int, is_zero)`
  gives the `-1` "malformed borrowed process input" line, not `-3`. The
  decoder's word passes through a further helper call (`fn24` in
  `funcid45`) before the status is reported, and that helper produces
  `-1`. The repair needs its own diagnosis.
- **The IR pin.** `crates/ken-cli/tests/rt_plain_match_layout.region-capacity.clif`
  is the only `.clif` snapshot. It moves when this repair lands.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The native exit decoder and the status path after it, as far as the
reported trap, the paired rows, and the `.clif` snapshot. Out of scope:
the interpreter, the trap catalogue's existing codes, and the 7 gated
narrowing sites.

## Acceptance

- **AC-0 (Architect, before code).** Rule the repair seat: where the `-1`
  arises after the gate, and whether the malformed payload reaches the
  starter as `-3` or as a dedicated status.
- **AC-1.** Four native rows, built and run on real roots, each paired
  with the interpreter. The negative and 2^64 rows exit with status 1
  and the malformed-Failure-payload line. The runtime `0` row exits 1 and
  the runtime `3` row exits 3, as controls. The rows differ only in the
  payload.
- **AC-2.** One mutation: drop the `valid_int` gate. The negative row
  reddens.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The repair needs a new trap code or a change to the starter's status
  contract that AC-0 did not rule.
- A change to the interpreter's reported error.

Checks: 4 fired. I read the decoder on main and the Architect's measured
table across both bases and the gated build. 2 fired. The Architect
enumerated all 8 narrowing callers, and only this one is ungated. 8
fired. The negative row exits 1 in every build, so status alone cannot
discriminate; the trap line does, and the `0` and `3` controls tell a
fix apart from a decoder that always traps.
