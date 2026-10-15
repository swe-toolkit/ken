---
id: SPEC-SERIALIZATION-ROUND-TRIP-CONTRACT
title: "spec/50-stdlib names a serialization round-trip law (decode after encode is Ok) as a package obligation, but no chapter states it and no catalog codec proves it. Author the serialization chapter with a first exact section: one pure codec whose round trip and malformed-input refusal are proved at zero trust"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-TIME-EVENT-TRACE-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. 06-catalog-campaign.md:377-378 places protocols/serialization/supply-chain after time/events/traces. Same shape as SPEC-TIME-EVENT-TRACE-CONTRACT. Steward-filed per COORDINATION section 2."
---

# An exact serialization round-trip contract

## Objective

A new `spec/50-stdlib` chapter exists, and its first section is the
normative contract for one pure codec: an encoder, a decoder, the proved
round-trip law and a typed refusal of malformed input. The first codec
catalog node builds against it.

## Settled inputs (on `44e4087d1`)

- **The law is named, not stated.** `spec/50-stdlib/README.md` section 4
  names a derivable `Encode`/`Decode` with the round-trip law `decode ∘
  encode = Ok` as a package obligation. No chapter states it.
- **A carrier exists, with no codec.** `Data.Serialization.Json`
  (`catalog/packages/Data/Serialization/Json.ken.md`) defines the
  six-constructor `Json` over integral `Int` numbers, with a proved size
  fold and no encoder or decoder. Its design notes defer duplicate-key
  rejection to the decoder.
- **The byte round trip is trusted.** `bytes_decode_encode` in
  `Data.Binary.BytesPrimitiveContracts` is an `axiom`. A zero-trust codec
  cannot rest on it.
- **Chapters 61 to 69 are taken**, and 70 is
  `SPEC-DIAGNOSTIC-CODE-CONTRACT`'s diagnostic-codes chapter.
- **Known elaborator limits.**
  - Pass `map` a named helper, not a λ.
  - A dependent `match` refines only its scrutinee as written (CHECKS
    11).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (Architect boundary ruling, measured).** Rule from a checked
   development (rc=0, no Axiom) on:
   - the chapter number, the package's Section and its module path;
   - the first codec: its value carrier (`Json`, or a smaller one if
     `Json`'s `String` and `Int` fields need trust), its token alphabet
     (for example `List Char` or a list of tokens) and its error type;
   - the laws: at least decode after encode is `Ok` of the value, and
     decode refuses one named malformed class with a typed error;
   - what stays deferred: derivation of `Encode`/`Decode`, bytes and
     UTF-8, Merkle hashing, and the protocol and supply-chain structures
     that coordinate with Lane B.
2. **The chapter, section 1**, stating the D0's operations and proved
   laws, with a conformance seed of closed cases.

## Acceptance

- **AC-1.** The Architect's D0 development checks generically, with no
  Axiom, and the section states exactly the laws it proved.
- **AC-2.** The seed has at least one round-trip case per carrier
  constructor and a case that decodes malformed input to the named typed
  error.
- **AC-3.** `spec/50-stdlib/README.md` and `SPEC-PROGRESS.md` record the
  chapter.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a new primitive.
- The D0 cannot prove the round trip generically for any carrier. Report
  the measured blocker, and do not state that law as proved.
