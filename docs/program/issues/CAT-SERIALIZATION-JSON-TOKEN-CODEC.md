---
id: CAT-SERIALIZATION-JSON-TOKEN-CODEC
title: "Spec 71 section 1 specifies a JSON token codec over the existing Json carrier, with four proved laws, but no catalog package delivers it. Deliver Data.Serialization.JsonCodec from the Architect's checked D0 at zero trust, and correct the Json design note that promises duplicate-key rejection"
status: ready
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-SERIALIZATION-ROUND-TRIP-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 71 section 1 from SPEC-SERIALIZATION-ROUND-TRIP-CONTRACT (Architect D0 evt_yttamsdpp9nh, review evt_2sy6g7srx6e4a). Same shape as CAT-TIME-EVENT-TRACE. Steward-filed per COORDINATION section 2."
---

# The JSON token codec, delivered and proved

## Objective

`Data.Serialization.JsonCodec` exists in `catalog/`, matches spec 71
section 1 token for token, and proves its four laws with no Axiom. A
client outside the module can encode, decode and match every error.

## Settled inputs (on the SPEC-SERIALIZATION landing)

- **The contract.** Spec 71 section 1 fixes:
  - the module path `catalog/packages/Data/Serialization/JsonCodec.ken.md`,
    over the existing `Data.Serialization.Json.Json` carrier, which it
    does not redeclare (section 1.1);
  - the 9-token alphabet, the 4-constructor error type and the decoder's
    error account, branch by branch (section 1.2);
  - the laws `json_decode_encode`, `json_decode_trailing`,
    `json_decode_empty` and `json_decode_stray_close` (section 1.3).
- **The proof route is stated.** Section 1.3 gives it: bounded induction
  on the encoded token list's length, reusing Derived's `list_append`
  associativity and right identity and `Core.Logic.Transport`'s `cong`,
  `sym` and `trans`. The opaque `String` and `Int` payloads are never
  inspected.
- **The D0 development is not persisted.** No codec file is on
  `architect/work` under `notes/`. Section 1.4 records the D0 as
  Architect-reported scratch results.
- **The design-note obligation.** `Json.ken.md` section 5 says the
  decoder will reject duplicate keys. Spec 71 section 1.4 defers that
  rejection and preserves repeated keys, so the note must be corrected.
- **Carry from the Architect's review** (`evt_2sy6g7srx6e4a`): the
  module is lede first.
- **Coupled test.** `catalog_ambient_passthrough_migration_census` in
  `lang_mod_strict_resolution_d0.rs` needs the new package's measured row.
- **Known limits.** Closed goals with `String` or `Int` leaves cannot be
  discharged in-language on this base. The seed labels those token
  vectors as harness expectations, not theorems.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

0. Before implementation, the leader asks the Architect to commit the D0
   development to `architect/work` under `notes/`, unless it has already
   been committed.
1. The package at `catalog/packages/Data/Serialization/JsonCodec.ken.md`,
   with the D0's declarations and proof bodies and the four laws as
   public theorems, lede first.
2. `Json.ken.md` section 5's duplicate-key sentence corrected to spec 71
   section 1.4: the token decoder preserves repeated keys, and rejection
   is deferred.
3. An acceptance target in `crates/ken-elaborator/tests/` that:
   - loads the package;
   - runs the seed `conformance/stdlib/serialization/seed-json-token-codec.md`
     from a client import, refusing each of its negatives at its own
     theorem;
   - restates the four law types generically;
   - records `trusted_base()` before and after loading.
4. The package's measured row in the ambient census.

## Acceptance

- **AC-1.** The package checks, every law is a checked proof body, and
  `trusted_base()` does not grow on loading it.
- **AC-2.** The seed's positives pass from a client module, and each
  negative is refused at its own theorem. The census passes with the new
  row, and every other row is unchanged.
- **AC-3 (mutation, QA).**
  - Swapping two array items in the encoder reddens `json_decode_encode`
    or its seed case.
  - Accepting trailing tokens in the decoder reddens the trailing case.
  - Mapping a stray close token to `UnexpectedEnd` reddens the
    stray-close case.

## Stop conditions

- A law needs an Axiom, or the proof needs to inspect a `String` or
  `Int` payload.
- `trusted_base()` grows on loading the package.
- A change outside the new package, `Json.ken.md`'s design note, the
  census test and the new acceptance target.
