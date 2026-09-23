---
id: CAT-CONFIGURATION-DECODER-IMPORT-EDGES
title: "give Decoder.ken.md a declared import edge for every qualified reference it makes -- 30 references to Data.Collections.Derived.nth and 7 qualified Application.Input.Schema names resolve today only because a sibling module's private import loaded their module, which spec 33-declarations section 3.2 does not grant -- and make the Decoder ledger test red on an undeclared provider"
status: merged
owner: foundation
size: S
gate: none
tier: T1
depends_on: [CAT-CONFIGURATION-DECODER-LAWS]
blocks: []
github: null
origin: "Adversary post-merge finding evt_3spntbt1amt0p on squash 59bf793d4 (CAT-CONFIGURATION-DECODER-LAWS), severity LEAK/GAP, not soundness. Steward re-measured the eight uncovered references at 59bf793d4 and confirmed the section 3.2 text before filing. The resolver half (resolve_ref accepts M.leaf for any loaded M) is language surface and is NOT this node; it waits on an operator lane decision because L2 is idle. Steward-filed per COORDINATION section 2."
---

# Decoder references modules it never imports

## Settled inputs -- measured at `59bf793d4`. Re-ground before acting.

- `spec/30-surface/33-declarations.md §3.2`: qualified `M.foo` access comes
  only from `import M` (or `import M as N`). A selective `import M (foo, Bar)`
  brings exactly those names **unqualified** and "nothing else of `M`".
- `catalog/packages/Application/Configuration/Decoder.ken.md` makes eight
  references §3.2 does not cover:
  - `Data.Collections.Derived.nth`, 30 uses, first at `:214`; Decoder has no
    import of `Data.Collections.Derived` at all.
  - `Application.Input.Schema.` `SchemaPresence`, `SchemaRequired`,
    `SchemaOptional`, `SchemaValueShape`, `MkSchemaField`,
    `SchemaFieldAccepted`, `schema_validate_fields`, qualified, while
    Decoder's only Schema import is selective and omits all seven.
- They resolve because `resolve_ref` (`crates/ken-elaborator/src/modules.rs`)
  resolves `M.leaf` against any already-loaded module's exports, and Schema,
  Doc and NonEmpty privately import `Derived`. The resolver is **not** this
  node's to change.
- `crates/ken-elaborator/src/r_layer_tests/cat_tier_e_decoder_import.rs`:
  `decoder_selective_import_ledger_is_exact` states "CLAIMED: Decoder declares
  every provider dependency", yet it compares only parsed import lists, so it
  cannot see a provider reached without an import. It also requires every
  Decoder import to be selective.
  `decoder_checked_provider_and_schema_closure_is_exact` already asserts
  `Data.Collections.Derived.nth` in the checked closure.

## Deliverable

1. **Declared edges.** Every qualified reference in `Decoder.ken.md` is covered
   under §3.2. Either bring the name in selectively and write it unqualified,
   or add a qualified `import M` and relax the all-selective rule in the ledger
   test to match. Choose per module and say which. The laws, the published
   population and every proof term are unchanged apart from reference
   spelling.
2. **A ledger that can fail.** The ledger test asserts that every module
   owning a global in Decoder's checked closure is Decoder itself, a prelude
   module, or a module Decoder imports. Correct its `CLAIMED` sentence to what
   is then measured.

## Acceptance criteria

- **AC-1 (falsifier, run first).** The new closure-module assertion, applied
  to `Decoder.ken.md` as landed at `59bf793d4`, goes RED and names
  `Data.Collections.Derived`. If it stays green there, the assertion is wrong,
  not the Decoder.
- **AC-2.** After deliverable 1 it is green. A scan of qualified references in
  `Decoder.ken.md` against its import forms finds no reference outside §3.2,
  and the handback shows that scan.
- **AC-3.** `catalog_ambient_passthrough_migration_census` stays clean.
  Editing that sentinel is not authorized.
- **AC-4.** Targeted tests only (`scripts/ken-cargo`, the Decoder test module
  and the census test). Whole-repo green means CI.

## Stop condition

If an added selective import clashes with an existing unqualified binding, use
a qualified import for that module instead. If neither form checks, stop and
report the refusal verbatim to the foundation leader. Do not rename proofs or
add aliases.

## Not this node

- The resolver import-scope check in `resolve_ref`. That is language work and
  needs an operator lane decision.
- `CAT-CONFIGURATION-DECODER-PRESENCE-CARRIER`'s result-type change.
- Any other catalog module. The Adversary's scan over 53 modules found Decoder
  to be the only one relying on this route.
