---
id: VERIFY-NATIVE-STABLE-SYMBOL-PRELUDE-IDENTITY
title: "Native stable symbols are named by reverse-mapping the live globals table, so a top-level const reusing a prelude constructor's spelling (const Stdout) takes its name, the prelude constructor falls back to ctor_{index}, and ken-runtime's suffix classifiers refuse a valid program. Name prelude ids from their checked identity"
status: active
owner: verify
size: M
tier: T1
gate: architect
depends_on: [VERIFY-ENTRY-ABI-CHECKED-IDENTITY]
blocks: []
github: null
origin: "Architect D0 ruling evt_45kr33z11554r on VERIFY-ENTRY-ABI-CHECKED-IDENTITY, point 4: a mechanism the AC-0 sweep missed, measured at 3ab95b8a7 and routed to the Steward as its own node. Steward-filed per COORDINATION section 2."
---

# Native symbols follow prelude identity

## Objective

A program's native build names every prelude declaration as spec 46
section 3.2 says, whatever spelling a user declaration takes.

## Settled inputs (Architect `evt_45kr33z11554r`, at `3ab95b8a7`)

- `stable_symbols_for_env` (compiler_driver.rs ~:4367-4412) names each id
  by reverse-mapping the live `globals`
  (`names_by_id.entry(*id).or_insert_with(|| name.clone())`). Once
  `const Stdout` takes the spelling, the prelude's `Stream` constructor
  falls back to `format!("ctor_{index}")`.
- `console_stream_tag` (ken-runtime lowering/mod.rs ~:9782) checks
  `ends_with("::Stdout")`, gets `None`, and refuses with `ObjectEmission
  ... Console operation has a malformed Stream operand`. There are about
  45 production `ends_with("::…")` classifiers: lowering/mod.rs 14,
  core.rs 9, calls.rs 1, source.rs 4, units.rs 2, primitive.rs 2,
  planning/static_transition responses.rs 13, aggregates.rs 3.
- The `ctor_{index}` fallback departs from spec 46 section 3.2:
  constructors use the parent family stable symbol plus the constructor
  name.
- Rows: `print-example-Stdout` and `print-plain-Stdout` fail native
  emission; `VERIFY-ENTRY-ABI-CHECKED-IDENTITY` pins them unchanged.
- Same class, interpreter route (Architect carry, `evt_3rsmr33aw58s2`):
  primitive reduction resolves `bytes_at`, `bytes_slice` and
  `bytes_decode` with `elab.globals.get(symbol)` (ken-interp eval.rs
  ~:2897). It moves to checked identity here too.
- The L2 strict-resolution flip (operator 2026-09-25) will refuse binding
  a prelude name. This node is the native route's identity repair and
  does not wait for it.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

AC-0 first. Then stable symbols for prelude ids come from checked
identity, and a same-spelled user declaration cannot take or collide with
them; the Architect rules the naming at D0.

## Acceptance

- **AC-0 (measure).** The two rows and a `zz_unrelated` control on native
  build. Every stable-symbol producer that reads live `globals`, and the
  classifier population above, swept by mechanism. The collision case:
  a prelude declaration and a user declaration of one spelling under
  `declaration_symbol(package, name)`.
- **AC-1 (D0 `evt_x572bpfnsyj1`).**
  - Both Stdout rows build and run as their `zz_unrelated` control (exit
    0, `entered main\n`).
  - The `const Stream` row builds and runs. A producer unit test asserts
    the prelude family is `Declaration [native-program, Stream]` and the
    user const is `[native-program, Stream#k]`, with `k` measured.
  - A unit test elaborates `const bytes_at : Nat = Zero` and calls
    `prim_reduce_elaborated("bytes_at", [Bytes [7], Int 0])`. It returns
    the `Some` constructor, against `Neutral` on base, measured first.
- **AC-2 (control).** A temporary census, removed before the candidate,
  shows pre-source and live names differ only in rows that declare a
  prelude spelling. The existing hash pins, trusted-base roster pins and
  `zz_unrelated` symbol tables are unchanged.
- **AC-3 (mutation, QA).** `name_of` reading live first reddens the Stdout
  rows. Removing the reserved substitution makes the `const Stream` row
  return `DuplicateStableSymbol`.

## Stop conditions

- The repair changes the stable-symbol contract in spec 46, or any
  kernel or trust surface: stop to the operator with the measurement.
- The repair needs a prelude rebinding to become a refusal: that is the
  L2 flip.
