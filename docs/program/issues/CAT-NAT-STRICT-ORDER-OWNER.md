---
id: CAT-NAT-STRICT-ORDER-OWNER
title: "Nat's strict order is defined twice, as Cursor's public cursor_nat_lt and Property's private property_nat_lt, and its laws are re-derived package by package across Cursor, Parsing, Decoder and Json. Make Data.Numeric.Nat.Order the owner of lt_nat and its laws, and retire Property's re-derived byte equality"
status: active
owner: foundation
size: M
tier: T2
gate: architect
depends_on: [CAT-INSERTIONSORT-DERIVED-SORT-IMPORT]
blocks: []
github: null
origin: "Architect nomination evt_1re37hce0twe5 on Steward evt_2fm966b2j2my4: next L3 proof-backfill slice (operator 2026-09-13), catalog §2a factoring. Carries the Parsing Decoder prose nit. Steward-filed per COORDINATION section 2."
---

# Nat.Order owns the strict order

## Objective

`Data.Numeric.Nat.Order` owns `lt_nat` and its laws, beside `leq_nat`,
`min`, `max`, `sub` and `compare`. No package re-derives Nat's strict order
or LawfulClasses' byte equality.

## Settled inputs (Architect `evt_1re37hce0twe5`, probed at `ed112ae4f`)

- **The duplication.**
  - `Capability.Parsing.Cursor` has `pub fn cursor_nat_lt`, and
    `Tooling.Testing.Property` has a private `property_nat_lt` with a
    byte-identical body.
  - Their laws are spread across Cursor, Parsing, Decoder and Json.
    `decoder_lt_self_suc` and `char_cursor_lt_suc` are the same lemma twice.
  - The ruling's table maps each of the 18 retired names to its Order name.
- **Property's census rows** (cat-reuse-census row 48). `property_uint8_eq`,
  `property_list_uint8_eq` and `property_bytes_eq` re-derive LawfulClasses'
  `uint8_deceq_eq`, `list_eq UInt8 uint8_deceq_eq` and `bytes_deceq_eq`.
  They are used only in two Bool counterexample witnesses.
- **Scope predicate.** In Cursor, Parsing, Decoder and Json, every
  declaration whose statement mentions the strict order or `leq_nat`, and
  otherwise only `Nat` and `Bool`, moves to Order. On main that population
  is 15.
- **Direction.**
  - Order imports only LawfulClasses, Transport and Arithmetic, so there is
    no cycle.
  - There is no re-export from Cursor. Every attached-proof consumer
    imports its subject from the owner. A proof reached through a
    re-exported subject is not delivered.
- **The probe.** The Architect applied exactly the migration in the ruling to
  an archive of `ed112ae4f`. The six packages check, and the catalog is 57/57
  exit 0, the same as baseline.
- **Not in scope:**
  - FoKripke's `fok_nat_lt`, which splits on the left argument (M3 shows
    the reduction order is load-bearing);
  - Json's `json_nat_add`, a [higher] row;
  - Property's `ByteCursor`, which is a different carrier.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

The ruling carries the code and the exact import lines. Bodies are moves,
renamed only.

1. **Order.**
   - Widen the Transport import to `(cong, sym, trans)`.
   - Add `pub fn lt_nat` beside `compare`, with its 12 attached laws after
     `compare`'s.
   - Add `pub theorem leq_nat_suc_add_right` with the other `leq_nat_*`
     bounds.
   - Update the §1/§2 prose.
2. **Consumers.**
   - Delete the retired declarations and rename every use per the table.
   - Cursor's law predicates are stated over `lt_nat`, and its prose at
     about `:224` names the owner.
   - Decoder, Json and Property each gain one selective Order import.
   - Property imports `bytes_deceq_eq` from LawfulClasses.
   - The Parsing Decoder prose nit rides here.
3. **Pins.** These are the measured consumers, and there are no hits in
   `examples/`, `conformance/`, `library/` or `spec/`:
   - `cat_tier_d_cursor_import.rs:297`, `cat_tier_d_decoder_import.rs:236`
     (plus an Order-owned `GlobalId` set of exactly `lt_nat`,
     `lt_nat::self_suc`, `lt_nat::shrink_suc`, `lt_nat::zero_right_absurd`),
     `cat_tier_d_parsing_group_import.rs:632`, and
     `cat_tier_e_json_import.rs:163/:232`;
   - the client fixtures `cat_decoder_recursive_fuel_seed.rs:58` and
     `cat_decoder_recursive_succeeds_client.rs:35`;
   - `ds9_json_codec_acceptance.rs:406`;
   - `cat_order_pub_export.rs` gains a selected-import client with zero
     trust delta;
   - `cat_property_acceptance.rs`: both witnesses are headed through
     `Core.Classes.LawfulClasses.bytes_deceq_eq`, and the four Property
     locals are absent.
   - If Property's ambient list in `lang_mod_strict_resolution_d0.rs:1105`
     shrinks, update it and report it.
4. **A vacuous pin is fixed.** `assert_private` (decoder_import `:149`,
   json_import `:289`) passes for a name that does not exist. The four
   retired Decoder and Json names move to an absence pin on the loaded env:
   `!env.globals.contains_key("<module>.<name>")`.

## Acceptance

- **AC-0 (probe; no edit).** The Order block elaborates on current main,
  and the scope predicate's population is 15.
- **AC-1.** `ken check` and `ken fmt --check` pass on the six packages, the
  catalog is 57/57 as on baseline, the named suites are green, and
  `trusted_base()` has no delta.
- **AC-2 (falsifiers; each must redden).**
  - M1: restoring `pub fn cursor_nat_lt` reddens the Cursor inventory pin.
  - M2: a local `decoder_lt_self_suc` used in Decoder reddens the Order-owned
    identity set and the absence pin.
  - M3: a left-first `lt_nat` stops Order from loading (`TypeMismatch` in
    `leq_suc`).
  - M4: withdrawing `pub` from `proof self_suc for lt_nat` stops Decoder and
    Json from loading.
  - M5: restoring `property_bytes_eq` in a witness reddens the Property
    provider pin.
- **AC-3.** Foundation QA runs the Architect's §2a review: the factoring,
  the placement in Order, and the prose.

## Stop conditions (to the Architect)

- S1: the Order block does not elaborate on current main.
- S2: a moved body needs more than the renames.
- S3: a catalog verdict changes.
- S4: the population is not 15, or the sweep finds a consumer not listed.
