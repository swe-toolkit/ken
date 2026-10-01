---
id: CAT-DERIVED-MAP-APPEND
title: "Derived owns map and list_append but does not publish that map distributes over list_append, so Parsing and EffectfulClasses each re-prove it privately. Publish proof append for map in Derived at zero TCB and retire both private copies"
status: active
owner: foundation
size: S
tier: T1
gate: architect
depends_on: [CAT-PARSING-SATISFY-BOUNDED]
blocks: []
github: null
origin: "Architect evt_69s0mbck17pz on Steward evt_59pc4v3ftaxt5: next L3 proof-backfill obligation (operator 2026-09-13), a slice of the named follow-on CAT-DERIVED-COLLECTIONS-LAWS. Steward-filed per COORDINATION section 2."
---

# Map distributes over append, published once

## Objective

A client proves `map f (xs ++ ys) = map f xs ++ map f ys` from one public
Derived law, and no package keeps a private copy.

## Settled inputs (Architect `evt_69s0mbck17pz`, read at `a559a6e2b`)

- **Delivered.** In `catalog/packages/Data/Collections/Derived.ken.md`:
  `pub fn map` (`:175`), `pub fn list_append` (`:89`), `proof id for map`
  (`:181`) and `proof fusion for map` (`:187`). `cong` is imported at `:87`.
  The name `append` is free on `map`.
- **Two private copies of the same proof:**
  - `Capability/Parsing/Parsing.ken.md:1286` `theorem map_appends`, used at
    `:1343`;
  - `Core/Classes/EffectfulClasses.ken.md:674` `theorem
    list_map_append_distrib`, used at `:730` and `:888`, and named in prose
    at `:775` and `:1173`.

  No test pins either name, across `catalog/`, `crates/`, `examples/`,
  `conformance/`, `docs/` and `spec/`.
- **The qualified spelling is delivered:** `(proof fusion for DC.map)` at
  EffectfulClasses `:869`, `(proof id for DC.map)` at `:591`.
- **The body is already kernel-checked** at Parsing `:1286-1302` and
  EffectfulClasses `:674-690`. Its `Nil` case is `Refl`.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **Add** `pub proof append for map (a) (b) (f) (xs) (ys) : Equal (List b)
   (map a b f (list_append a xs ys)) (list_append b (map a b f xs) (map a b
   f ys))`, by induction on `xs` with `cong` on `Cons`, as in the ruling.
   Place it directly after `proof fusion for map`.
2. **Retire the copies.**
   - Parsing: delete `map_appends`. At its use, call `(proof append for
     map) UInt8 Int uint8_to_int (bytes_to_list a) (bytes_to_list b)`.
   - EffectfulClasses: delete `list_map_append_distrib`. At its two uses,
     call `(proof append for DC.map)`. The prose at `:775` and `:1173` names
     the Derived law.

Do not widen to `concat_map_append`, which Derived keeps private on purpose
(`:2595`).

Scope:

- `catalog/packages/Data/Collections/Derived.ken.md`;
- `catalog/packages/Capability/Parsing/Parsing.ken.md`;
- `catalog/packages/Core/Classes/EffectfulClasses.ken.md`;
- `crates/ken-elaborator/src/r_layer_tests/cat_lawful_functors_pub_export.rs`;
- `crates/ken-elaborator/src/r_layer_tests/cat_tier_d_parsing_group_import.rs`:
  only the `map::append` entry in the exact expected Derived provider set of
  `parsing_module_provider_closure_is_exact_and_sibling_disjoint` (~`:660`),
  since Parsing now reuses the Derived law (implementer `evt_1ap6mqgabpdhs`).

## Acceptance

- **AC-1.** `ken check` passes on all three packages, and `proof append for
  map` is public.
- **AC-2 (falsifiers).** In `cat_lawful_functors_pub_export.rs`, next to the
  `(proof fusion for map)` client (`:608`):
  - a clean-environment client importing only `Data.Collections.Derived
    (list_append, map)` states the law at free `a b f xs ys` and closes it by
    `(proof append for map) a b f xs ys`. On `a559a6e2b` it fails because the
    name is absent;
  - a mutant with the right-hand operands swapped is rejected with a typed
    `TypeMismatch`, not by name resolution;
  - `trusted_base()` equality is asserted.
- **AC-3.**
  - `trusted_base()` is unchanged.
  - The catalog census is byte-identical except for the three packages.
  - The Derived, Parsing and EffectfulClasses acceptance targets,
    `crates/ken-cli/tests/rosetta.rs`, and the r_layer lawful-functors group
    stay green.

## Stop conditions

- Any new import, primitive, postulate or axiom.
- A consumer of either deleted name that was not counted: stop and name it.
