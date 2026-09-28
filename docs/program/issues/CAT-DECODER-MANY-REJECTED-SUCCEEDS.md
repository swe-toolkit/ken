---
id: CAT-DECODER-MANY-REJECTED-SUCCEEDS
title: "Publish decoder_many_rejected_succeeds in Capability.Parsing.Decoder: when the step rejects at the current cursor, decoder_many decodes Nil there without consuming, so BYTES D3 can prove spaces_decoder succeeds on printed input through the sealed many fuel"
status: active
owner: foundation
size: S
gate: architect
tier: T1
depends_on: [CAT-DECODER-RECURSIVE-SUCCEEDS]
blocks: [BYTES-CONCAT-AND-ENCODE-CONTRACTS]
github: null
origin: "Architect ruling 2026-09-28 (evt_5ttkqzp913xat) on BYTES D3 question evt_4008cqx96rm6e: no proof-only route exists through Decoder's public API because decoder_many's fuel recursor is private. Same provider-first pattern as CAT-DECODER-RECURSIVE-SUCCEEDS. Steward-filed per COORDINATION section 2."
---

# Decoder: success law for decoder_many on rejection

## Objective

A client of `Capability.Parsing.Decoder` can prove that `decoder_many step`
decodes `Nil` at a cursor where `step` rejects, without seeing the private
fuel recursor.

## Settled inputs (Architect `evt_5ttkqzp913xat`, read at `22f0f2e44`)

- `decoder_many` (`Decoder.ken.md:165`) is `decoder_many_fuel` (`:117`,
  private) seeded at `cursor_remaining cur`. Its only public law,
  `decoder_many_preserves` (`:2375`), is bounds-only.
- **The statement and proof are given verbatim in the ruling.** The proof
  eliminates a Nat variable together with its equation, not the computed
  remaining, and uses `decoder_many_zero_result_matches` (`:673`) and
  `decoder_many_fuel_outcome_matches` (`:620`). The Architect's scratch
  `ken check` exited 0 on a binary built 09-21, so it is a probe, not a
  candidate.
- The scope check is done: `decoder_many` is the last fuel-sealed combinator
  D3 uses without a success law.

## Deliverable

`pub theorem decoder_many_rejected_succeeds` in `Decoder.ken.md`, placed
after `decoder_recursive_succeeds` and listed with the section's public
laws. No Decoder definition changes.

## Acceptance

- **AC-1.** `ken check` passes on a current build with no `Axiom` and no
  trusted item. Decoder's public surface gains exactly this one name, and the
  Decoder suite is green.
- **AC-2 (controls).** Both of the Architect's mutations are refused on the
  current build: `Refl` in place of `rejected`, and a `DecoderFailed`
  conclusion.
- **AC-3 (consumer).** D3's `spaces_decoder` success step instantiates the
  theorem. This may land with D3 rather than here.
- Targeted builds only, through `scripts/ken-cargo`.

## Stop conditions

- A change to any Decoder definition, or a new trusted item: stop to the
  Architect.
- The verbatim proof fails on the current build: stop to the Architect with
  the verbatim error.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.
