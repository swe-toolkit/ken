---
id: CAT-DECODER-RECURSIVE-FUEL-SEED
title: "decoder_recursive must not report fuel exhaustion on legal input: a layer that consumes one unit per recursion reaches fuel zero at end of input, so standard right-recursive repetition over public combinators fails on every input while the package claims exhaustion is impossible"
status: ready
owner: foundation
size: S
gate: architect
tier: T1
depends_on: [CAT-DECODER-RECURSIVE-SUCCEEDS]
blocks: []
github: null
origin: "Adversary finding 2026-09-28 on M8 00ffedcfd (evt_1ay9njh53gkr2), a measured defect in a public combinator that predates that merge. Steward-filed per COORDINATION section 2."
---

# Decoder: fuel seed for decoder_recursive

## Objective

A recursive layer that consumes at least one unit before each recursion
never reaches `DecoderFuelExhausted` on legal input. Decoder's docs state
the recursive contract truthfully.

## Settled inputs (Adversary `evt_1ay9njh53gkr2`, at `00ffedcfd`; not re-run by the Steward)

- `decoder_recursive` seeds its fuel at exactly `cursor_remaining cur`
  (`Decoder.ken.md:227`; the zero arm is at `:215`). `decoder_alt`
  propagates `DecoderFuelExhausted` without backtracking (`:90`).
- **Repro.** The cursor is `List Bool` with `remaining = length`, which
  satisfies all three Cursor laws. The layer is
  `alt (seq (satisfy any) recur) (pure True)`. It fails on Nil, `[True]`
  and `[True, False]`. The control, `remaining = Suc (length cur)`, succeeds
  on all three.
- **False claims.** `Decoder.ken.md:10` calls exhaustion impossible, and
  `:2773` says legal repeated input cannot reach it.
- **Why the landed test misses it.** The `CAT-DECODER-RECURSIVE-SUCCEEDS`
  AC-2 client (`cat_decoder_recursive_succeeds_client.rs:40`) uses the
  `Suc (length cur)` cursor, the one under which the defect disappears.
- **Consumers.** `Parsing.ken.md:750` is not exposed: every BoolExpr
  recursion consumes 5 or more bytes first. `ds9_json_codec_acceptance.rs:718`
  and DS-9's depth argument (`ds-9-json-codec.md:120`) rely on the same seed.
  JSON was not probed.

## Deliverable

One of the following, as ruled at AC-0:
- the repaired seed, with `decoder_recursive_succeeds` and the preservation
  laws re-proved;
- or the corrected claims, with the one-unit-per-recursion limit stated as
  part of the recursive contract.

## Acceptance

- **AC-0 (probe, then ruling).**
  - Re-run the repro and its control at the landed base.
  - Probe the JSON consumer.
  - Report whether a reseed changes any public statement, including
    `decoder_recursive_succeeds`'s `positive` premise.
  - The Architect rules the route.
- **AC-1.** The repro's layer decodes Nil, `[True]` and `[True, False]`
  under an honest `length` cursor. If the ruled route is documentation only,
  a pinned test states the limit instead.
- **AC-2 (control).** Reverting the repair returns the repro to failure.
- **AC-3.** No trust change. Decoder's public surface is unchanged unless
  the ruling says otherwise. Targeted builds only.

## Stop conditions

- Any kernel, `trusted_base()` or spec change (an operator question).
- **Sequencing:** do not start while `wp/BYTES-CONCAT-AND-ENCODE-CONTRACTS-D3`
  is in flight if the repair touches a statement D3 consumes.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.
