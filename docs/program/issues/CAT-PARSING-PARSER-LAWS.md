---
id: CAT-PARSING-PARSER-LAWS
title: "Parsing proves ParserLaws only for the one Boolean parser, so the card's stated behaviour of parser_pure and parser_fail and of every parser_from_decoder parser is unproved prose. Prove a generic ParserLaws for parser_from_decoder under DecoderPreservesBounded, with parser_pure and parser_fail as instances, at zero TCB"
status: merged
owner: foundation
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect evt_2a42pws33685v on Steward evt_43sd50trxm9wa: next L3 proof-backfill obligation (operator 2026-09-13). Law index row Capability/Parsing/Parsing. Steward-filed per COORDINATION section 2."
---

# ParserLaws for every bounded decoder parser

## Objective

Every parser built by `parser_from_decoder` from a bounded decoder satisfies
`ParserLaws`, by a checked theorem. `parser_pure` and `parser_fail` are
instances, and the Boolean parser's laws are re-derived from the generic
theorem.

## Settled inputs (Architect `evt_2a42pws33685v`, read at `c7f572dda`)

- **Delivered names** (`Parsing.ken.md`): `parser_from_decoder` (`:336`),
  `ParserLaws` (`:397`, the conjunction of `ParserValid`, `ParserTotal` and
  `ParserSourceLocal`), `parser_pure` and `parser_fail` (`:400-404`),
  `DecoderPreservesBounded` (`:1394`), the private
  `parser_from_decoder_valid_if_bounded` (`:1649`), and the only current
  `ParserLaws` inhabitant `parse_bool_expr_laws` (`:2819`), built through
  `parse_bool_expr_laws_if_decoder_bounded`.
- **Reachable at zero TCB.**
  - Valid is the existing `parser_from_decoder_valid_if_bounded`.
  - Total is `Top` in both arms.
  - SourceLocal follows from Valid's `ValidSpan` and source-id conjunct.
  - `decoder_pure` is bounded (end = start). `decoder_fail` has no `Decoded`
    outcome, so its bound holds vacuously.
  - The import closure is unchanged; no trusted declaration is added.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

In `catalog/packages/Capability/Parsing/Parsing.ken.md`:

```
pub theorem parser_from_decoder_laws
      (a : Type) (decoder : Decoder ByteCursor Span a)
      (bounded : DecoderPreservesBounded a decoder)
    : ParserLaws a (parser_from_decoder a decoder)
pub theorem parser_pure_laws (a : Type) (value : a)
    : ParserLaws a (parser_pure a value)
pub theorem parser_fail_laws (a : Type) : ParserLaws a (parser_fail a)
```

`parse_bool_expr_laws` becomes an instance of `parser_from_decoder_laws`, and
the bespoke `parse_bool_expr_laws_if_decoder_bounded` composition is retired.
The public theorems lead their section and the helpers follow.

Scope: `Parsing.ken.md`, its acceptance test
(`crates/ken-elaborator/tests/cat5_parsing_package.rs`), and in
`crates/ken-elaborator/src/r_layer_tests/cat_tier_d_parsing_group_import.rs`
only the three new names in the expected public inventory (Steward
`evt_2mjp2qrv96thc`). No `Decoder.ken.md` change.

## Acceptance

- **AC-1.** The three theorems check by `ken check`, and
  `parse_bool_expr_laws` keeps its statement while its body is the generic
  instance.
- **AC-2 (fence).** A false twin `ParserLaws a (parser_from_decoder a d)`
  with no `bounded` hypothesis is rejected, over a closed decoder that
  reports an end position past `source_length`. The bounded theorem is its
  true twin.
- **AC-3.** `trusted_base()` is unchanged, the 61-file catalog census is
  byte-identical except `Parsing.ken.md`, and the Parsing acceptance targets
  stay green, including `ken-elaborator --lib
  parsing_module_loader_visible_inventory_is_exact_and_coherent`.

## Stop conditions

- Any new import, primitive, postulate or axiom.
- A `Decoder.ken.md` change, or a needed bound for `decoder_alt`,
  `decoder_bind` or `decoder_many` (a follow-on; only `decoder_seq` exists).

## Closeout

Merged `5e0a97be5` (PR #4428), exact `a9ef373d7`: Foundation QA
`evt_7fdrmm9v9nh8`, Architect `evt_6am04tvz26vhx`, Decision
`dec_1469akzkrpamj`.

- `parser_from_decoder_laws` proves `ParserLaws` for every bounded decoder
  parser. `parser_pure_laws` and `parser_fail_laws` are instances, and
  `parse_bool_expr_laws` is re-derived from the generic law.
- The public laws lead §4.3, followed by an 8-helper closure, in the same
  definition run (`evt_4js9vdbbcgmb2`).
- The public parsing inventory pin lists the three new names
  (`evt_2mjp2qrv96thc`).
- `trusted_base()` is unchanged.
- Follow-on: `LANG-FORWARD-REFERENCE-ACROSS-DATA-EXPORT`. Spec 33 §8.4
  forward references are cut off by an intervening `export` or `data`.

§1a count: 1 (arrangement). The placement assumed module-wide forward
references, which the loader does not deliver (`evt_4js9vdbbcgmb2`).

