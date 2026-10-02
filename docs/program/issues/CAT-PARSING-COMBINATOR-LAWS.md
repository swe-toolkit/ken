---
id: CAT-PARSING-COMBINATOR-LAWS
title: "parser_from_decoder_laws needs DecoderPreservesBounded, but that premise is private and duplicates Decoder's public DecoderPreserves, so no client can discharge it for any combinator decoder. Export the byte-cursor bound as DecoderPreserves, export the existing cursor locate and advance proofs, and instance satisfy and many, at zero TCB"
status: merged
owner: foundation
size: S
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
- **The private abbreviation.** Parsing's private `DecoderOutcomeBounded`
  (`Parsing.ken.md:410`) matches Decoder's `DecoderResultPreserved` arm for
  arm, at `good_cursor = ByteCursorBounded s start` and
  `good_location = ValidSpan s`. That Decoder predicate is private
  (`Decoder.ken.md:1788`), so Parsing cannot spell it (Architect
  `evt_3mttfykt8802n`).
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

Export, subsume and instance: no re-derivation, and keep the existing names
(Architect `evt_1x2anyf7dzsqy`, which corrects `evt_688y060mgq32r`; the
first nomination missed the private obligations already in Parsing).

1. **Publish the bound over `DecoderPreserves`.**
   - `pub fn DecoderPreservesBounded (a) (decoder) : Prop` becomes
     `(s : Source) → (start : Nat) → DecoderPreserves ByteCursor Span a
     (ByteCursorBounded s start) (ValidSpan s) decoder`.
   - `ByteCursorBounded` becomes `pub`.
   - `fn DecoderOutcomeBounded` stays private and textually unchanged, as
     the helper types' abbreviation; do not inline a `match` into a
     proposition type. A helper given the new premise converts arm for arm
     to it, as the λ-identity bridges `decoder_bounded_as_public` (`:1860`)
     and `decoder_public_as_bounded` (`:1870`) show.
   - Delete those bridges if nothing still calls them; otherwise keep them
     private. If conversion fails at a site, stop and post the site; add no
     alias.
2. **Make public, unchanged:** `byte_cursor_bounded_locate` (`:614`) and
   `byte_cursor_bounded_after_peek` (`:1812`, the advance-sound type, which
   already composes Derived's `some_below_length for nth`).
3. **New public instances**, each a composition:
   - `byte_satisfy_parser_laws (accept)`, through `decoder_satisfy_preserves`
     at a free `accept`. `byte_code_decoder_public_preserves` (`:1884`)
     becomes a one-line instance of it or is deleted.
   - `byte_many_parser_laws (a) (step) (step_safe : DecoderPreservesBounded
     a step)`, through `decoder_many_preserves`.

Scope:

- `catalog/packages/Capability/Parsing/Parsing.ken.md`;
- the Parsing acceptance tests and the public-inventory pin;
- `crates/ken-elaborator/src/r_layer_tests/cat_tier_d_decoder_import.rs`
  (`:294`). That fixture records the private predicate as a gap. It imports
  the exported names, or its gap note is updated.

## Acceptance

- **AC-1.** All three items check by `ken check`. The two instances are stated
  as public theorems that a client can apply.
- **AC-2 (falsifiers).**
  - Twin cursor ops whose advance is `Suc (Suc position)`: the
    advance-sound proof fails at the end-position conjunct.
  - A strengthened `byte_cursor_bounded_after_peek` end bound,
    `LessEqNat (Suc (Suc position)) …`, fails for the real cursor ops.
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
  - The catalog census is byte-identical except for `Parsing.ken.md`.
  - The Parsing, Decoder and Derived acceptance targets stay green, as does
    the serial `lang_mod_strict_resolution_d0`.

## Stop conditions

- Any new import, primitive, postulate or axiom.
- A `Decoder.ken.md` or `Derived.ken.md` change.
- Item 1 needs two public spellings of the bound: stop to the Architect.

## Closeout

Merged `a559a6e2b` (PR #4432), exact `abe38b1b9`: Foundation QA
`evt_4m4s097q89bs`, Architect `evt_6w5s72w7axyaf`, Decision
`dec_4dp5g3axy1t99`.

- `DecoderPreservesBounded` is public over Decoder's `DecoderPreserves`, and
  `ByteCursorBounded`, `byte_cursor_bounded_locate` and
  `byte_cursor_bounded_after_peek` are public. The two λ-identity bridges
  are deleted.
- `byte_satisfy_parser_laws` and `byte_many_parser_laws` are public
  compositions that the CAT5 client applies.
- The PARSER-LAWS false-twin fence closes by `refl` with an accepted true
  twin, pinned structurally.
- Zero TCB; only `Parsing.ken.md` changed in the catalog census.
