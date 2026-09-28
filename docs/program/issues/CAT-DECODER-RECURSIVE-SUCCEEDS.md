---
id: CAT-DECODER-RECURSIVE-SUCCEEDS
title: "Publish a success-only recursion principle for decoder_recursive in Capability.Parsing.Decoder, proved by strong induction on the private fuel recursor, so a recursive client parser can prove that its decoder succeeds on printed input -- the enabler BYTES D3's printer round trip needs"
status: ready
owner: foundation
size: S
gate: architect
tier: T1
depends_on: []
blocks: [BYTES-CONCAT-AND-ENCODE-CONTRACTS]
github: null
origin: "Architect design check evt_67rjfsq02mtkg on BYTES-CONCAT-AND-ENCODE-CONTRACTS D3 (Foundation question evt_2r4pr9qy6mmrk): the Parsing round-trip induction cannot close because decoder_recursive's child call is the private fuel recursor, and no public Decoder law is a success law. The Architect asked the Steward for a Decoder provider increment on the CAT-PARSING-DECODER-PRESERVATION pattern. Steward-filed per COORDINATION section 2."
---

# Decoder: a success-only recursion principle

## Objective

A recursive client of `Capability.Parsing.Decoder` can prove that its
decoder succeeds on input it printed, without seeing the private fuel
recursor.

## Settled inputs (Architect `evt_67rjfsq02mtkg`, read on main `8dd81d7b6`)

- `Decoder.ken.md:206-230`: `decoder_recursive ... layer` is
  `λcur. decoder_recursive_fuel ... layer (cursor_remaining ... cur) cur`,
  and `decoder_recursive_fuel (Suc fuel2) cur` is
  `layer (decoder_recursive_fuel ... fuel2) cur`. The fuel recursor is
  private.
- Every public Decoder law is a bounds-preservation law
  (`decoder_*_preserves`, `Decoder.ken.md:1817-2480`); none states success.
- **The naive unfolding equation is false.** It is
  `decoder_recursive layer cur = layer (decoder_recursive layer) cur` at
  positive remaining. At input `(not ` then end of input, the left side
  rejects and the right side is `DecoderFuelExhausted`, which `decoder_alt`
  treats differently. Do not publish it.
- The theorem to publish is `decoder_recursive_succeeds`, with the exact
  statement given in `evt_67rjfsq02mtkg`. It is success-only, quantifies over
  no error path and exposes no fuel. The proof is by strong induction on the
  fuel, with positivity refuting fuel zero.

## Deliverable

`pub theorem decoder_recursive_succeeds` in `Decoder.ken.md` plus its
private fuel lemma. No Decoder definition or error constructor changes.

## Acceptance

- **AC-0 (probe, before building; Check 4).** Post the exact child-call term
  after one unfolding of Parsing's `bool_expression_decoder`, and confirm
  that no public Decoder law mentions it. Ground the spellings of
  `cursor_nat_lt` and positivity in Decoder's own helpers and their
  visibility. The Architect rules on the statement as checked text.
- **AC-1.** The theorem checks with no `Axiom` and no trusted item; the
  `trusted_base` delta is empty. Decoder's public surface gains exactly this
  one name.
- **AC-2 (consumer).** A test instantiates the theorem for a small
  recursive layer and derives `Decoded` on a concrete printed input.
- **AC-3 (control).** Weakening the positivity hypothesis makes the proof
  fail to check.
- **AC-4.** Targeted builds only, through `scripts/ken-cargo`.
  No-regression means green in CI.

## Stop conditions

- A change to any Decoder definition, error constructor or the `Source`
  contract, or a new trusted item, is a stop to the Architect.
- The statement needs an error-path clause to be provable: stop to the
  Architect.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.
