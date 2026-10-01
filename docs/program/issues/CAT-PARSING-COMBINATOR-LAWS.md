---
id: CAT-PARSING-COMBINATOR-LAWS
title: "parser_from_decoder_laws needs DecoderPreservesBounded, but that premise is private and duplicates Decoder's public DecoderPreserves, so no client can discharge it for any combinator decoder. Export the byte-cursor bound as DecoderPreserves, prove the cursor's locate and advance soundness, and instance satisfy and many, at zero TCB"
status: active
owner: foundation
size: M
tier: T1
gate: architect
depends_on: [CAT-PARSING-PARSER-LAWS]
blocks: []
github: null
origin: "Architect evt_688y060mgq32r on Steward evt_5edwj5k9vsrbw: next L3 proof-backfill obligation, the follow-on from CAT-PARSING-PARSER-LAWS (operator 2026-09-13). Law index row Capability/Parsing: parser laws remain explicit predicates. Steward-filed per COORDINATION section 2."
---

# ParserLaws for every combinator decoder over the byte cursor

## Objective

A client can prove `ParserLaws` for a parser built from any Decoder
combinator over Parsing's byte cursor, by composing Decoder's public
`*_preserves` theorems with `parser_from_decoder_laws`.

## Settled inputs (Architect `evt_688y060mgq32r`, read at `5e0a97be5`)

- **The gap.** `parser_from_decoder_laws` takes
  `bounded : DecoderPreservesBounded a decoder`, which is private.
- **The duplicate.** Parsing's private `DecoderOutcomeBounded`
  (`Parsing.ken.md:410`) matches Decoder's public `DecoderResultPreserved`
  arm for arm, at `good_cursor = ByteCursorBounded s start` and
  `good_location = ValidSpan s`.
- **Delivered names.**
  - Decoder, all `pub`: `DecoderPreserves` and
    `decoder_{pure,fail,bind,seq,alt,satisfy,many,recursive}_preserves`.
    `satisfy` and `many` take `locate_sound`; `satisfy` also takes
    `advance_sound`.
  - Parsing, `pub`: `parser_from_decoder_laws`, `byte_cursor_ops`,
    `ValidSpan` and `LessEqNat`.
  - Parsing, private: `ByteCursorBounded`, `DecoderPreservesBounded`,
    `DecoderOutcomeBounded` and `byte_cursor_bounded_locate` (`:614`).
  - Order and Derived, `pub` and already imported by Parsing:
    `leq_nat_successor_bound`, `leq_nat_weaken_right`, `nth`, `length` and
    `bytes_nat_length`.
- **Reachable at zero TCB.** Every step is structural or reuses a checked
  theorem. Peek is `nth` over `bytes_to_list`, so no primitive contract is
  needed.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **Subsume the duplicate.**
   - `pub fn DecoderPreservesBounded (a) (decoder) : Prop` becomes
     `(s : Source) → (start : Nat) → DecoderPreserves ByteCursor Span a
     (ByteCursorBounded s start) (ValidSpan s) decoder`.
   - `ByteCursorBounded` becomes `pub`, and `DecoderOutcomeBounded` is
     deleted.
   - The private helpers are re-proved against the new definition.
   - Fallback, only if conversion of the two stuck matches obstructs: keep
     both definitions with one private bridge theorem. Never two public
     spellings.
2. **`pub theorem byte_cursor_locate_sound`.** This is
   `byte_cursor_bounded_locate`, made public.
3. **`pub theorem byte_cursor_advance_sound`.** A successful peek and a
   bounded cursor give a bounded advanced cursor. It matches on a computed
   `nth` result (check 11): bind it first, and carry the other side with
   `cong`.
4. **`pub theorem nth_some_index_bound`** in `Data/Collections/Derived`:
   `Equal (Option a) (nth a n xs) (Some a v) → Equal Bool (leq_nat (Suc n)
   (length a xs)) True`.
5. **Public instances:** `byte_satisfy_parser_laws (accept)` and
   `byte_many_parser_laws (a) (step) (h : DecoderPreservesBounded a step)`.
   Each is a composition of the theorems above.

Scope:

- `catalog/packages/Capability/Parsing/Parsing.ken.md`;
- `catalog/packages/Data/Collections/Derived.ken.md`, for item 4 only;
- the Parsing acceptance tests and the public-inventory pin;
- `crates/ken-elaborator/src/r_layer_tests/cat_tier_d_decoder_import.rs`
  (`:294`). That fixture records the private predicate as a gap. It imports
  the exported names, or its gap note is updated.

## Acceptance

- **AC-1.** All five items check by `ken check`. The two instances are stated
  as public theorems that a client can apply.
- **AC-2 (falsifiers).**
  - Twin cursor ops whose advance is `Suc (Suc position)`: its
    `advance_sound` fails at the end-position conjunct.
  - A weakened `nth_some_index_bound`, with `leq_nat n (length xs)`, still
    checks, and a strengthened one, with `leq_nat (Suc (Suc n)) …`, fails.
- **AC-2b (repair the PARSER-LAWS fence).** `unbounded_parser_laws_false_twin`
  (`Parsing.ken.md:647-698`) closes its end bound with `Proved` on a stuck
  goal (`:674`), so its same-shape true twin is rejected too (Adversary
  `evt_16nvfbkpgkvt3`).
  - Close the bound with `(proof refl for LessEqNat) (source_length s)`.
  - Add the true twin, with the decoder ending at `source_length s`, as a
    checked block that is accepted.
  - The false twin must be rejected with `expected LessEqNat (Suc n) n`,
    found `LessEqNat n n`.
- **AC-3.**
  - `trusted_base()` is unchanged.
  - The catalog census is byte-identical except for the two named packages.
  - The Parsing, Decoder and Derived acceptance targets stay green, as does
    the serial `lang_mod_strict_resolution_d0`.

## Stop conditions

- Any new module import, primitive, postulate or axiom. Adding
  `nth_some_index_bound` to Parsing's existing `Data.Collections.Derived`
  symbol list is in scope.
- A `Decoder.ken.md` change.
- Item 1 needs two public spellings of the bound: stop to the Architect.
