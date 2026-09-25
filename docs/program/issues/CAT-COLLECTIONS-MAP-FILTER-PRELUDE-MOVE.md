---
id: CAT-COLLECTIONS-MAP-FILTER-PRELUDE-MOVE
title: "Move the prelude's map and filter into Data.Collections.Derived as its structural ops, delete the two prelude declarations, and re-key every consumer and pin by exact identity; third L3 slice of the minimal-prelude program (Prod, zip, fold, And/and_*/is_sorted excluded)"
status: active
owner: foundation
size: M
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Operator rulings 2026-09-25: the prelude is the minimal set required; convenience names not required by the prelude rules are technical debt and move to packages. Move units ruled by Architect evt_222zfr1tdg94h (L3 collections split, measured at 2596150ae) on the program design evt_4s5he6tnf3xs4. Steward-filed per COORDINATION section 2."
---

# Move map and filter into Data.Collections.Derived

## Objective

`map` and `filter` over `List` are declared once, in
`Data.Collections.Derived`, and the prelude no longer declares them. Every
consumer reaches them by import.

## Settled inputs -- Architect `evt_222zfr1tdg94h`, measured at `2596150ae`

- **U1 `map`** (`prelude.rs:527-531`) becomes `pub fn map (a : Type) (b :
  Type) (f : a → b) (xs : List a) : List b` in `Derived`, with the same
  `Nil`/`Cons` recursion, among its structural ops (§4.1, `:125`). Under
  §2a it does not lead the module. Direct consumers add `map` to their
  existing selective `import Data.Collections.Derived (...)`:
  `Application.CommandLine.ArgParse`, `Capability.Filesystem.Path.Posix`,
  `Data.Binary.BytesPrimitiveContracts`, `Data.Collections.NonEmpty`,
  `Tooling.Testing.Property`. `Derived` uses it locally (`map_length`).
- **U2 `filter`** (`prelude.rs:542-546`) becomes `pub fn filter (a : Type)
  (p : a → Bool) (xs : List a) : List a` beside `map`. Its only consumer is
  `Derived`.
- **Not consumers:** the `map` in `EffectfulClasses`, `LawfulFunctors` and
  `Validation` is the `Functor` class field. Leave it untouched.
- **No prelude-internal reader** of either name, and no `map_id`/`filter_id`
  field. The `combinator_trusted_before`/`after` bracket stays; its comment
  now names only `fold`/`zip`.
- **Out of scope:** `Prod` and prelude `zip` (keyed by runtime and native
  ABI machinery; a floor question for L2), prelude `fold` (no catalog
  consumer; a separate removal), and `And`/`and_*`/`is_sorted` (held for
  `LANG-EXPRESSION-SIGMA`). The `list_map` duplication in
  `Core.Classes.LawfulFunctors` is the next slice, not this one.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

`map` and `filter` are declared in `Derived` with the prelude's bodies and
removed from `prelude.rs`. Every consumer imports them, and every pin that
named the prelude identity is re-keyed to `Data.Collections.Derived.map` or
`.filter`. The `crates/` changes are the two deletions, the bracket comment
and test expectations. No kernel or `trusted_base()` change.

## Acceptance

- **AC-0 (behavioral, before re-keying).** Delete the two prelude
  declarations first, then record which targeted suites and loads go red.
  The Architect's prediction, to reconcile and not to copy:
  - pins that invert to "`Derived` owns it; `ElabEnv::new()` has no
    `map`/`filter`": `cat3_collections_package.rs:232-246`,
    `cat_derived_filter_membership_law.rs:22-37`,
    `cat_property_acceptance.rs:162-198`;
  - suites using the prelude combinators directly:
    `lang_prelude_collections.rs`, `l3a_acceptance.rs`;
  - one non-catalog source consumer: `examples/rosetta/closures/closures.ken:55`.

  Each is re-keyed by exact identity, not by spelling. A red outside this
  list is a measurement, reported in the handoff with its disposition.
- **AC-1 (census, deletion-only).** In
  `catalog_ambient_passthrough_migration_census`, 22 of the 44 rows lose
  exactly `{filter, map}`: 340 pairs become 296. No row empties or is added;
  the other 22 are byte-unchanged. `vector_strict_floor_provider_transition_sentinel`
  stays empty. Any added pair or unpredicted deletion is a STOP.
- **AC-2 (identity and allocator).** Removing the two declarations shifts
  `fold`/`zip` down by 1 and every prelude id after `filter` down by 2, and
  lowers `ElabEnv::new()`'s `declarations().len()` and `next_global_id()` by
  2. Each numeric pin that moves is re-predicted by that rule, never
  blanket-updated. `trusted_base()` is unchanged.
- **AC-3 (collisions).** `Application.Configuration.Decoder` imports
  `Derived` wholesale and loads at the candidate. Vector's local `map` does
  not clash.
- **AC-4.** Targeted builds only, through `scripts/ken-cargo`. No-regression
  means green in CI.

## Stop conditions

- Any `prelude.rs` change beyond the two deletions and the bracket comment,
  or any kernel or `trusted_base()` change.
- A census change outside AC-1, or a moved statement that is not
  byte-identical to the prelude's.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.
