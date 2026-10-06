---
id: CAT-LIST-LENGTH-TRUST-FREE-BASE
title: "The canonical List length lives in Data.Collections.Derived, whose import closure declares five trusted axioms, so a zero-trust package such as Vector cannot import it. Move length verbatim into a trust-free base list module that Derived re-exports with the same checked identity"
status: ready
owner: foundation
size: M
tier: T2
gate: architect
depends_on: []
blocks: [CAT-VECTOR-TO-LIST-LENGTH-LAW]
github: null
origin: "Architect recut evt_17t0yjaee4atj on CAT-VECTOR-TO-LIST-ZIP-LAWS stop 2 (the frame's length enabler was false against spec 60 §7). Enables the length/to_list bridge spec 60 §5 defers. Steward-filed per COORDINATION section 2."
---

# A trust-free base module holds List length

## Objective

`length` is declared in a list module with no imports and no trust, and
`Data.Collections.Derived` re-exports it with the provider's checked
identity. A zero-trust package can import `length` without loading
Derived's closure.

## Settled inputs (Architect `evt_17t0yjaee4atj`, read at `28d937604`)

- `pub fn length` (`Derived.ken.md:241`) needs only prelude `List` and
  `Nat`. Derived imports `Core.Classes.LawfulClasses` (four `ord_int_*`
  axioms, `:215-228`) and `Data.Text.StringBijection`
  (`string_to_list_char_retraction`, `:15`). That makes 5 by source count;
  the ID attribution is not yet measured.
- Re-exports keep the provider's checked identity (`modules.rs` tests
  `file_reexports_preserve_provider_id_through_facade_and_in_scope`,
  `checked_current_local_and_its_inline_reexport_are_one_identity`). They do
  not create an `env.globals["Data.Collections.Derived.length"]` entry, so
  tests that read that spelling must re-key.
- **Consumers.** 18 catalog files import Derived and resolve through the
  re-export unchanged. Tests keyed on the global spelling include the six
  the Architect listed (`cat_tier_d_cursor_import.rs`,
  `cat_tier_e_json_import.rs`, `cat_property_acceptance.rs`,
  `cat_vector_closeout.rs`, `cc3_parsing_cursor_decoder_acceptance.rs`,
  `cat_derived_pub_export.rs`), and a literal grep also hits
  `cc5_pretty_doc_acceptance.rs`, `ds9_json_codec_acceptance.rs` and
  `lang_prelude_collections.rs`. The two lists disagree, so D0 censuses by
  mechanism.
- Derived's `length` laws (`list_append::length`, `map_length`,
  `length_take_min`, `length_drop`, `some_below_length for nth`) stay in
  Derived.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **D0.** Map Vector's cold trust delta (+5 when it imports Derived) to
   declaration names by ID. Confirm `Data.Collections.List` is free. Census
   every test that reads `length` through the Derived global spelling, by
   format string as well as literal.
2. The base module with `pub fn length` verbatim; Derived imports and
   exports it; each census test re-keys to the provider's ID and adds a
   facade selective-import assertion that the re-export carries that ID.

## Acceptance

- **AC-1.** Derived's export table has `length` with the base provider's ID,
  and the base module's cold trust equals the compiler base.
- **AC-2 (controls).** Every catalog importer of Derived checks unchanged,
  and every census test passes.
- **AC-3 (mutation, QA).** Declaring a second `length` in Derived instead of
  re-exporting reddens the identity assertion.

## Stop conditions

- A consumer whose resolution changes, not only its test key.
- Any kernel, `trusted_base()` or spec change.
