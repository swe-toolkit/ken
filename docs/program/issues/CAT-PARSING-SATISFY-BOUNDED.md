---
id: CAT-PARSING-SATISFY-BOUNDED
title: "A client proving ParserLaws for many over a consuming satisfy step must rebuild the byte-cursor bound from four pieces, and the package re-derives that fact three times. Publish byte_satisfy_bounded as one generic law at zero TCB, route the existing instances through it, and delete the unused private re-derivation"
status: active
owner: foundation
size: S
tier: T1
gate: architect
depends_on: [CAT-PARSING-COMBINATOR-LAWS]
blocks: []
github: null
origin: "Architect evt_51kc72sm7ycvc on Steward evt_6d2nb743022wf: next L3 proof-backfill obligation (operator 2026-09-13: schedule the proof backfill before extending the catalog). Steward-filed per COORDINATION section 2."
---

# One public bound for satisfy over the byte cursor

## Objective

A client gets `ParserLaws` for `many (satisfy accept)` over Parsing's byte
cursor, at a free `accept`, from one public theorem and
`byte_many_parser_laws`, without importing the cursor proofs.

## Settled inputs (Architect `evt_51kc72sm7ycvc`, read at `a559a6e2b`)

- **Delivered public names.** In Parsing: `DecoderPreservesBounded`
  (`:430`), `ByteCursorBounded` (`:423`), `byte_cursor_bounded_locate`
  (`:615`), `byte_cursor_bounded_after_peek` (`:1856`),
  `byte_satisfy_parser_laws` (`:1904`) and `byte_many_parser_laws`
  (`:1926`). In Decoder, already imported at Parsing `:80`:
  `decoder_satisfy_preserves` (`:2097`).
- **The fact is proved three times.**
  - Inline in `byte_satisfy_parser_laws` (`:1909-1925`);
  - in `byte_code_decoder_public_bounded` (`:1950`);
  - privately, by hand, in `byte_code_decoder_preserves` (`:1745`) and
    `byte_code_decoder_bounded` (`:1797`). These two have 0 consumers
    outside their own definitions in `catalog/`, `crates/`, `conformance/`
    and `examples/`.
- **The new theorem's body is already kernel-checked.** It is the subterm at
  `:1909-1925`.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **Add** `pub theorem byte_satisfy_bounded (accept : UInt8 → Bool) :
   DecoderPreservesBounded UInt8 (decoder_satisfy ByteCursor UInt8 Span
   byte_cursor_ops accept)`. Its body is `λs. λstart.
   decoder_satisfy_preserves … (byte_cursor_bounded_locate s start)
   (byte_cursor_bounded_after_peek s start)`, as in the ruling. Place it
   directly before `byte_satisfy_parser_laws`.
2. **Route through it.**
   - `byte_satisfy_parser_laws accept` becomes `parser_from_decoder_laws`
     applied to `byte_satisfy_bounded accept`.
   - `byte_code_decoder_public_bounded code` becomes `byte_satisfy_bounded
     (λbyte. eq_int (uint8_to_int byte) code)`. Its signature and six
     in-package uses stay unchanged.
3. **Delete** `byte_code_decoder_preserves` and `byte_code_decoder_bounded`,
   then only the private helpers whose reference count is measured at 0
   afterwards. The candidates are `byte_code_decoder_outcome_equation`,
   `option_prop_elim` and `bool_prop_elim`.
4. **Name lists.** `byte_satisfy_bounded` joins the registered-name lists at
   `cat5_parsing_package.rs:367` and
   `r_layer_tests/cat_tier_d_parsing_group_import.rs` (~`:496`).

Scope:

- `catalog/packages/Capability/Parsing/Parsing.ken.md`;
- `crates/ken-elaborator/tests/cat5_parsing_package.rs`;
- `crates/ken-elaborator/src/r_layer_tests/cat_tier_d_parsing_group_import.rs`.

## Acceptance

- **AC-1.** `ken check` passes on Parsing, and `byte_satisfy_bounded` is a
  public theorem.
- **AC-2 (falsifier).** Extend
  `cat5_parser_laws_are_publicly_instantiable_without_new_trust` (`:1344`)
  with `client_many_satisfy_parser_laws (accept : UInt8 → Bool)`, generic in
  `accept`. It imports neither `decoder_satisfy_preserves` nor the two cursor
  theorems, and closes by `byte_many_parser_laws UInt8 (decoder_satisfy …
  accept) (byte_satisfy_bounded accept)`.
  - On `a559a6e2b` it fails on name resolution.
  - The same statement at a fixed predicate, with the generic law absent,
    fails with a typed `TypeMismatch`.
  - The trusted-base equality assertion in that test stays.
- **AC-3.**
  - `trusted_base()` is unchanged.
  - The catalog census is byte-identical except for `Parsing.ken.md`.
  - The Parsing and Decoder acceptance targets and the r_layer Parsing group
    stay green.

## Stop conditions

- Any new import, primitive, postulate or axiom, or a `Decoder.ken.md` or
  `Derived.ken.md` change.
- A deleted theorem or helper has a consumer that was not counted: stop and
  name it.
