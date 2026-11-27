---
id: RT-FS-SET-LENGTH-HOST-WIDTH-ADMISSION
title: "Spec 38 §1.7.1 says setLength refuses a host-width-unrepresentable length with InvalidOffset before dispatch, but both lanes admit any length that fits u64, while the Linux host's off_t is signed 64-bit: a length in [2^63, 2^64) is dispatched, and Rust std refuses it inside File::set_len, so the program sees ResourceHostIO instead of InvalidOffset. Bound admission at the host width in both lanes and pin the range"
status: merged
owner: language
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary evt_2qd9sk1n35tvh on RT-FS-DESCRIPTOR-SEEK-LENGTH-SYNC r4 (44364bd79): a measured spec-implementation mismatch at the error boundary, fail-closed (file bytes unchanged). Steward-filed per COORDINATION section 2 as the product repair."
---

# setLength admits only host-representable lengths

## Objective

For every `length` that the host's length type cannot represent, `setLength`
returns `InvalidOffset` with no `FsSetLength` dispatch, in the interpreter and
natively, as spec 38 §1.7.1 states. A representable length still dispatches,
and a host refusal of it, such as `EFBIG`, stays `ResourceHostIO`.

## Settled inputs (main `44364bd79`; Adversary measurement `evt_2qd9sk1n35tvh`)

- **The claim.** `spec/30-surface/38-ffi-io.md:602`: "rejects a negative or
  host-width-unrepresentable length with `InvalidOffset` before dispatch".
- **The admission seats.**
  - Interpreter: `ken-interp/src/eval.rs:6738` narrows through
    `narrow_host_u64` (`:6360`), which is `to_u64` only.
  - Native: `ken-runtime/.../lowering/effects.rs:3850` narrows through
    `narrow_positioned_int_seat`, also u64.
- **The host.** `ken-host/src/lib.rs:438` calls `File::set_len(u64)`. Rust
  std refuses a value above `i64::MAX` during the `off_t` conversion,
  without calling the kernel, as `InvalidInput`.
- **Measured** (interpreter lane, the shipped `fs_descriptor.ken`
  invalid-length body, only the length changed):
  - `2^64`: no event, `InvalidOffset`. This is the shipped control.
  - `2^63` and `2^64 - 1`: dispatched, `Io(InvalidInput)`, exit 73.
  - `2^63 - 1`: dispatched, `Io(Other(27))`, a genuine `EFBIG`. This is
    correct behaviour.
- **The pins that miss the range.**
  `negative_length_refuses_before_dispatch` and
  `positive_u64_overflow_refuses_before_dispatch_in_interpreter`
  (`ken-verify/src/scenario/tests/fs_descriptor.rs:279`, `:299`), and the
  conformance case `file-io/set-length-invalid-length-refuses-before-host`
  (`seed-descriptor-operations.md:41`), sample only `-1` and `2^64`.
- **The native boundary.** DESCRIPTOR's native build of the `2^64` literal
  is a named boundary owned by `RT-STATIC-UNSELECTED-CALL-DISPOSITION`.
  The values in this range fit u64, so they are not that boundary.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The two admission seats, one bound shared by both if AC-0 rules one, the
named rows and the conformance case. No change to the host's
`resource_set_length` signature, to the wire request, or to any other
positioned operation unless AC-0 finds it shares the defect.

## Acceptance

- **AC-0 (Architect, before code).** Rule:
  - the bound: `i64::MAX` as the Linux `off_t` width, or a host-declared
    width, and where that width is stated once for both lanes;
  - whether `readAt` and `writeAt` offsets share the defect (same narrowing,
    same host conversion), and so join this WP or are recorded clear;
  - whether spec 38 §1.7.1 needs a sentence naming the width.
- **AC-1.** Interpreter and native, both real roots: `2^63` and `2^64 - 1`
  return `InvalidOffset` with no `FsSetLength` event; `2^63 - 1` dispatches.
  Paired rows that differ only in the length.
- **AC-2.** One mutation per lane: restoring the u64-only bound in that
  lane's seat reddens its `2^63` row.
- **AC-3.** The conformance case gains the `2^63` row beside `2^64`, as a
  black-box recording-host case. CV reviews it.
- **AC-4.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The bound needs a wire or host ABI change.
- Native admission of `2^63` meets the unselected-call boundary rather than
  the seat.
- AC-0 finds the defect wider than the positioned operations.

Checks: 4 fired. I read the spec line, both narrowing seats and the host
call on `44364bd79`. 6 fired. The Adversary measured four lengths across
the range, and the shipped pins sample two values outside it. 8 fired. The
`2^63 - 1` row dispatches and the `2^63` row must not, so the pair tells the
bound apart from an always-refuse.
