---
id: CAT-LAWFULFUNCTORS-LIST-MAP-SUBSUMPTION
title: "Subsume Core.Classes.LawfulFunctors.list_map into Data.Collections.Derived.map: move its attached id and fusion proofs onto Derived.map, retarget the Functor List instance and EffectfulClasses, delete list_map, and re-key its pins by exact identity; fourth L3 slice of the minimal-prelude program"
status: merged
owner: foundation
size: M
gate: architect
tier: T1
depends_on: [CAT-COLLECTIONS-MAP-FILTER-PRELUDE-MOVE]
blocks: []
github: null
origin: "Architect section 2a factoring carry in evt_222zfr1tdg94h: list_map 'is the same tool as map' and 'must not survive the program'; subsuming it is a public-API change with proof movement, so it is its own slice. Serves the operator's 2026-09-25 minimal-prelude program. Steward-filed per COORDINATION section 2."
---

# Subsume list_map into Derived.map

## Objective

The catalog has one list map. `Data.Collections.Derived.map` carries the
`id` and `fusion` laws, the `Functor List` instance uses it, and
`Core.Classes.LawfulFunctors.list_map` no longer exists.

## Settled inputs -- Architect `evt_222zfr1tdg94h`; to measure at the base

- **Base.** Cut from the `main` that holds
  `CAT-COLLECTIONS-MAP-FILTER-PRELUDE-MOVE`, which puts `pub fn map` in
  `Derived`.
- **The duplicate.** `LawfulFunctors.ken.md:163` declares `pub fn list_map`
  with the same `Nil`/`Cons` match tree as `Derived.map`. It carries `pub
  proof id for list_map` and `pub proof fusion for list_map`, and the
  `Functor List` instance (`:219-221`) uses all three.
- **Consumers.** `EffectfulClasses` imports `list_map` (`:60`) and uses it
  and `proof id for list_map` (`:491`, `:586-607`, `:620-649`). Pins:
  `cat_lawful_functors_pub_export.rs:347-349`,
  `cat_effectful_classes_import.rs:83, :247-258`,
  `seal2_tests/producer_closure.rs:360`, and the `kenfmt_b3_layout.rs:104-105`
  import fixture.
- **Imports.** `Derived` already imports `Core.Logic.Transport (cong, sym,
  trans)`. `Core.Function.Combinators` (`comp`, `idf`) has no imports, so
  `Derived` can import it without a cycle.
- **Name collision to resolve, not to assume away.** `LawfulFunctors`
  declares the `Functor` class, whose field is spelled `map`, and
  `EffectfulClasses` imports that class. A bare selective import of
  `Derived.map` into either module may collide with the field. Reach it by
  its qualified identity, through a module alias as `LawfulFunctors` already
  does for `LawfulClasses as LC`, unless AC-0 shows the bare spelling
  resolves to the exact `Derived` identity.
- **Out of scope.** `option_map`, the local `list_map_coh` theorem name in
  `EffectfulClasses`, `fold`, `Prod`/`zip`, and `And`/`and_*`/`is_sorted`.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

`pub proof id for map` and `pub proof fusion for map` are declared in
`Derived`, beside `map`, with the moved proof bodies. `list_map` and its two
proofs are deleted from `LawfulFunctors`. The `Functor List` instance, the
`EffectfulClasses` uses and every pin name `Derived.map`, `map::id` and
`map::fusion` by exact identity. No `crates/` change beyond test
expectations. No prelude, kernel or `trusted_base()` change.

## Acceptance

- **AC-0 (behavioral, before re-keying).** Delete `list_map` and its two
  proofs first, then record which targeted suites and `ken check` loads go
  red. The list above is the prediction, to reconcile and not to copy. A red
  outside it is a measurement, reported with its disposition. Record whether
  bare `map` in `LawfulFunctors` and `EffectfulClasses` resolves to the class
  field or to `Derived.map`.
- **AC-1 (identity).** `Functor_instance_List`'s `map`, `id_law` and
  `fusion_law` fields are, by checked identity, `Derived.map`,
  `Derived.map::id` and `Derived.map::fusion`. No global spelled `list_map`
  remains in either module. The `Functor Option` instance is byte-unchanged.
- **AC-2 (laws are real).** The moved proofs check in `Derived` against its
  own imports. A control that breaks one `Cons` arm of the moved `fusion`
  proof reddens it, and restoring it goes green.
- **AC-3.** `ken check` and `ken fmt --check` pass on every edited catalog
  file. Targeted builds only, through `scripts/ken-cargo`; no-regression
  means green in CI.

## Stop conditions

- The subsumption needs a prelude, kernel or `trusted_base()` change, or an
  edit to the `Functor` class declaration.
- The `Derived` import of `Core.Function.Combinators` would form a cycle.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.
