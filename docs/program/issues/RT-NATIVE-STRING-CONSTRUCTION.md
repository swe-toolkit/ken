---
id: RT-NATIVE-STRING-CONSTRUCTION
title: "Native String construction disagrees with the interpreter: Cranelift list_char_to_string treats each Char as a UTF-8 byte (a silent wrong value for scalars 128-255, refusals above), and native and runtime-IR bytes_decode skip NFC; fix every native String ingress together so the trusted byte contracts stay true on every engine"
status: merged
owner: runtime
size: M
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Adversary M8 report on BYTES D2 0a6aa5987 (evt_cr9t8xh9hpb7), findings F-A (MEDIUM, silent miscompile) and F-B (LOW-MEDIUM, NFC skipped), both measured and pre-existing. Serves 37 §2.1/§2.3 and 41 §3a and keeps the operator-admitted F3 axiom true natively. Steward-filed per COORDINATION section 2."
---

# Native String construction agrees with the interpreter

## Objective

Every native path that constructs a `String` (Cranelift and the runtime-IR
evaluator) produces the same NFC value as the interpreter, or refuses closed.
None returns a different string.

## Settled inputs -- Adversary `evt_cr9t8xh9hpb7`, measured at `0a6aa5987`

- **F-A, silent wrong value.** `cranelift_backend/lowering/core/primitive.rs`
  `lowered_char_list` narrows each Char with `u8::try_from` (`:34`), then
  `list_char_to_string` runs `String::from_utf8` over those bytes (`:262`).
  - Chars [195, 169] return "é" (U+00E9); the interpreter (`eval.rs:2410`,
    `char::from_u32`) returns "Ã©".
  - [233] refuses "non-UTF-8 Char values".
  - Any scalar at or above 256 refuses "not a closed List Char".
  - No native test calls `list_char_to_string`.
- **F-B, NFC skipped.** Native `bytes_decode` (`primitive.rs:774`) and the
  runtime-IR evaluator (`runtime_ir_evaluator.rs:1816`) use a bare
  `String::from_utf8`. The interpreter wraps the same step in `NfcString::new`
  (`eval.rs:2140`). Bytes [65 CC 81] decode to "e\u{301}" natively and "é" in
  the interpreter.
- **Coupling.** The trusted F3 axiom (BYTES D2) holds today only because no
  native ingress yields non-NFC text that decode would then normalize. So F-A
  and F-B must land together: a fixed `list_char_to_string` that can build
  "e" followed by U+0301 must normalize as the interpreter does.
- `STR-NFC-CONSTRUCTION` fixed the interpreter's ingresses only. Its ruling
  (NFC at construction is normative) governs here.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

Native `list_char_to_string` builds the string from Unicode scalars and
normalizes it to NFC. Native and runtime-IR `bytes_decode` normalize to NFC on
success. Any other native `String` ingress that AC-0 finds gets the same
treatment. No change to the interpreter or to the four BYTES axioms.

## Acceptance

- **AC-0 (post to the WP thread, then proceed).** Classify every native and
  runtime-IR site that constructs a `String` value by a predicate (every
  producer of the native String representation), not by a list. Mark each
  site as normalizing or not.
- **AC-1 (three-engine agreement).** One fixture per ingress runs on the
  interpreter, runtime-IR and Cranelift and asserts the same value:
  - Chars [195, 169] give "Ã©";
  - [233] gives "é";
  - a scalar at or above 256 succeeds;
  - Chars ['e', U+0301] give NFC "é";
  - bytes [65 CC 81] decode to `Ok "é"`.

  Each fixture is red on base in at least one native engine.
- **AC-2 (controls).** Reverting only the NFC step reddens the [65 CC 81] and
  ['e', U+0301] rows. Reverting only the scalar fix reddens the [195, 169]
  row. The BYTES axiom test is extended to check F3 against both native
  engines. Targeted builds only, through `scripts/ken-cargo`; no-regression
  means green in CI.

## Stop conditions

- A native ingress cannot normalize without a new runtime resource or a new
  trusted entry.
- Any change to the interpreter's behavior or to a BYTES axiom statement.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.
