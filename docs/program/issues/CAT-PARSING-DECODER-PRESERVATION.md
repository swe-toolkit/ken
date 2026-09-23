---
id: CAT-PARSING-DECODER-PRESERVATION
title: "Publish checked, reusable invariant-preservation laws beside the combinators in Capability.Parsing.Decoder, parametric in client-supplied good-cursor and good-location predicates, so a client parser can prove its span/source laws without seeing Decoder's private error cases or fuel recursors -- the public proof boundary CAT-PARSING-LAWS stopped on"
status: merged
owner: foundation
size: M
gate: none
tier: T1
depends_on: [CAT-PARSING-DECODER-LAWS]
blocks: [CAT-PARSING-LAWS]
github: null
origin: "Architect design/scope ruling evt_12832hyyk1mrs on held CAT-PARSING-LAWS candidate 1f5f99eb5 (base aacfc618e): Decoder preservation is absent at its public proof boundary, so a consumer cannot discharge the Parsing.ken.md:892-917 decoder obligation. Architect asked the Steward for a distinct provider WP. Steward-filed per COORDINATION section 2."
---

# Decoder's laws are private, so no client can use them

## Settled inputs -- Architect `evt_12832hyyk1mrs`, at `aacfc618e`

- `catalog/packages/Capability/Parsing/Decoder.ken.md` exports only
  `DecoderRejected` of `DecoderError` (`:22`). `:80-91` separates rejection,
  which falls back, from zero-progress and fuel exhaustion, which propagate.
  The fuel recursors (`:117-173`, `:204-227`) are private, and every law at
  `:370-1760` is a private `theorem`. `33-declarations §4.1-4.2` makes
  private names invisible to a client.
- `Parsing.ken.md:892-917` states the decoder outcome/cursor boundedness
  obligation. `:1239-1254` builds `ParserLaws` only given it, and `:716-763`
  uses `alt`, `many` and `recursive`, so the premise is live. The existing
  `DecoderManyConsumesAllLaw` is about whole-input success, not span/source
  preservation.

## Deliverable

Public preservation laws in `Decoder.ken.md`, beside the combinators:

- **Parametric** in a client good-cursor predicate and a good-location
  predicate. Premise: `cursor_locate` is sound on good cursors.
- **Invariant:** a successful `Decoded` keeps the cursor invariant.
  `DecoderFailed err` establishes the location invariant through the public
  `decoder_error_location`.
- **`alt`:** from first/second preservation. The law splits the three error
  cases internally and falls back only on ordinary rejection.
- **`many`:** assumes the step preserves the invariant. It proves the
  zero-progress and fuel-exhaustion locations from the cursor premise and
  induction on the private fuel.
- **`recursive`:** assumes the layer preserves the invariant for any
  invariant-preserving recursive argument. It discharges the zero-fuel
  location and the successor induction internally.
- Also `pure`, `fail`, `bind`, `seq` and `satisfy`, as far as `Parsing.ken.md`
  uses them.

The ring fixes the exact statements.

## Acceptance criteria

- **AC-1.** `DecoderZeroProgress`, `DecoderFuelExhausted` and both fuel
  recursors stay private. No new constructor, `Axiom`, postulate, primitive
  or second decoder. Shipped declarations are unchanged apart from added
  proofs and exports.
- **AC-2.** Decoder imports nothing from a higher tier (`Parsing`, `Source`).
  A consumer-view check instantiates the laws with Parsing's landed public
  `ByteCursor` / `byte_cursor_ops` and `ValidSpan`, a test-local
  good-cursor predicate, and a checked `cursor_locate` premise, and it
  elaborates. `ByteCursorBounded` exists only on the held Parsing
  candidate, so wiring it in belongs to `CAT-PARSING-LAWS` after this node
  lands (Steward `evt_2wrezd0sxmba8` answer). The check lives in a test,
  not in `Parsing.ken.md`.
- **AC-3 (falsifier).** Name one one-line change to `decoder_alt` that
  breaks the `alt` law, for example falling back on `DecoderZeroProgress`.
  No law may be vacuous.
- **AC-4.** Under `crates/`, only consumer-view harnesses change, such as
  Decoder's export inventory. Use Cursor's `AC-3c` rule for red
  assertions. Targeted `scripts/ken-cargo` only; no-regression means green
  in CI.

## Stop conditions

- If a law needs a fuel helper or a hidden error constructor to be public,
  STOP and return to the Architect. Do not publish it.
- Any `crates/**/src/**` path other than a consumer-view harness is a hard
  stop.
