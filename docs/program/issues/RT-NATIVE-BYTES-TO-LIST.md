---
id: RT-NATIVE-BYTES-TO-LIST
title: "bytes_to_list is outside the native primitive set, and uint8_to_int refuses a UInt8 that arrives as a List element, so every byte-literal pattern and the prelude's socket_unix_path build under the interpreter but refuse at native-build. Rule from an AC-0 how bytes_to_list lowers over every Bytes representation and where a list element's UInt8 is projected to Int, and land both with interpreter-against-native parity rows"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: []
blocks: [PX11-NATIVE-STREAM-SOCKET-PARITY]
github: null
origin: "Architect ruling evt_773gmfx4vmmab (U1) on PX11-NATIVE Inc2's missing-listener stop (runtime-implementer evt_3h2ca7fh2v0vx), measured on main with no Net operation. Steward decision evt_29axys7grks27: a separate prerequisite, run on the Language ring (L1b; operator 2026-11-22: \"Approved. Use the language ring.\") ahead of ABI-S2. L1 Linux ABI program, Steward order."
---

# Native bytes_to_list and list-element UInt8

## Objective

A Ken program that turns `Bytes` into a `List UInt8` and reads its
elements as `Int` builds natively and agrees with the interpreter. That
covers `bytes_to_list` used directly, every byte-literal pattern, and the
prelude's `socket_unix_path`. No prelude rewrite and no new primitive
are involved: the backend lowers the prelude as written.

## Settled inputs (main `9716dd74f`; Architect probes under `/workspaces/ken/tmp/arch-subst-coinc-wt/zz-arch-u8probe/`)

- **G2, bytes_to_list.** `ken-runtime/src` has no `bytes_to_list`
  anywhere, including `runtime_ir_evaluator.rs`. Native lowering refuses
  it as "not in the supported native set" (`lowering/core/primitive.rs`
  ~`:278`). The native byte set is `bytes_at` (static index only,
  ~`:617`), `bytes_concat`, `bytes_decode`, `bytes_encode`,
  `bytes_length` and `bytes_slice`.
- **G1, a list element's UInt8.** `uint8_to_int` (~`:216`) needs a
  `Lowered::Int` operand. The scalar projection (~`:95`) covers only a
  `Carried` operand. A literal one-element `List UInt8` consumed by
  `uint8_to_int` refuses with "uint8_to_int expects an Int-represented
  value" (probe p5). `bytes_at`'s element is the small-int form that
  works (~`:655`, probe p3).
- **Consumers.**
  - `socket_unix_path` and `private_socket_has_nul`
    (`ken-elaborator/src/prelude/net.rs` ~`:40` to ~`:76`) use both
    sites.
  - Byte-literal patterns elaborate through `bytes_to_list` and
    `uint8_to_int` (`elab.rs` ~`:26272` to ~`:26300`, ~`:26557`). Probe
    p7 (`match path { b"ab" ↦ True; _ ↦ False }`) runs on the
    interpreter and refuses natively.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Native lowering of `bytes_to_list`, the list-element `UInt8` projection,
the runtime-IR evaluator if it is a lane that runs this code, the arena
cost accounting for the new list, and the parity rows. No prelude change,
no new primitive, no change to `bytes_at`'s static-index rule, and no
`argv[0]` contract change (carried to Spec, `evt_5bp7x43zy0bvz`).

## Acceptance

- **AC-0 (Architect, before code).** Rule:
  - G2's lowering: a `List UInt8` whose elements take `bytes_at`'s
    small-int form, over every `Bytes` representation the
    `bytes_length` path admits (specialized response bytes, a borrowed
    native value, the ordinary value, and `Carried`);
  - its arena cost in the invocation profile;
  - whether the runtime-IR evaluator needs the same arm.
  For G1, the implementer first measures and names the `Lowered` variant
  a list element reaches `uint8_to_int` as. The fix goes in the operand
  projection keyed on the primitive's static symbol, not as a
  `uint8_to_int` special case at the call site.
- **AC-1.** Parity rows, interpreter against native, each with a known
  answer:
  - p5, a literal `List UInt8` element through `uint8_to_int`;
  - p6, `bytes_to_list` over a runtime argument;
  - p7, a byte-literal pattern that matches on one input and fails on
    another;
  - `socket_unix_path` on an absolute, a relative, an over-length and a
    NUL-bearing path, each giving its ruled result;
  - one `bytes_to_list` row per `Bytes` representation AC-0 names. The
    representation cannot be seen from the source, so each row states
    which one it reaches.
- **AC-2.** Mutations, restored byte-identically:
  - drop one representation's arm from G2: its row fails and the others
    pass;
  - revert the G1 projection: p5 fails at native-build.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The lowering needs a new primitive, a prelude change, or a TCB change.
- A `Bytes` representation that G2 cannot read without copying through a
  path the arena profile does not account for.

Checks: 4 fired. The Architect's probes p2, p3, p5, p6 and p7 measured
each gap on main source with no `Net` operation. I read the refusal
sites and the native byte set at the cited lines. 2 fired. The guarded
property is that a primitive the prelude uses lowers natively. Its
fan-in is every `bytes_to_list` caller and every list-element
`uint8_to_int`. Probe p7 shows the byte-literal pattern is a second
consumer path, and AC-1 covers it. 9 fired. G2 dispatches on the `Bytes`
representation, so AC-1 has one row per representation, and AC-2's drop
mutation proves each row reaches its own arm.
