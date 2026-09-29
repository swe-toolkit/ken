---
id: CAT-ORD-INT-AXIOM-NAMING
title: "Name the four anonymous law Axioms of instance Ord Int (refl, antisym, trans, total) as named catalog axioms in LawfulClasses.ken.md, consumed by the instance fields, with trusted_base() cardinality unchanged; first L3 proof-backfill slice"
status: ready
owner: foundation
size: S
gate: architect
tier: T2
depends_on: []
blocks: []
github: null
origin: "Architect ruling PRIMITIVE-CONTRACT-AXIOM-PLACEMENT (2026-09-14, evt_45tk37v830x9b; docs/program/PRIMITIVE-CONTRACT-AXIOM-PLACEMENT.md). Architect L3 runway ruling 2026-09-29 (evt_5f1ewknxv3m6h): no definable prelude convenience slice remains; L3 turns to proof backfill, this node first. PRINCIPLES #17 (complete trust manifest). Steward-filed per COORDINATION section 2."
---

# Name the Ord Int law axioms

## Objective

The trust manifest names each assumption behind `instance Ord Int`. Today
its four law fields are anonymous term-level `Axiom`s. Each one mints a
`trusted_base()` entry labelled only by `owner_label`, so the four cannot be
told apart, cited or discharged one at a time.

## Fixed inputs (read at `e5cd36c54`)

- **The site.** `catalog/packages/Core/Classes/LawfulClasses.ken.md:215-220`:
  `instance Ord Int { leq = int_leq; refl = Axiom; antisym = Axiom;
  trans = Axiom; total = Axiom }`. Locate it by the declaration, not the
  line; the file is high-contention.
- **The field types** come from the `Ord` class in the same file, at
  `a = Int` and `leq = int_leq`.
- **The template** is `axiom string_to_list_char_retraction`
  (`catalog/packages/Data/Text/StringBijection.ken.md:15`), consumed by name.
- **Home rule** (ruling doc). The statements use catalog vocabulary
  (`IsTrue`, `bool_or`), so they are named catalog axioms, not kernel
  certificates. `IsTrue` and `bool_or` have zero occurrences in
  `crates/ken-kernel/src/`.
- **Known consumers of `Ord Int`**: ES4's Derived owner-local examples and
  the other LawfulClasses instances. This is a starting list, not the
  population. `Capability.System.Error` is a measured zero (Architect
  `evt_vqs91q7y15vz`, correcting `evt_5f1ewknxv3m6h`).

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

Four named catalog `axiom` declarations in `LawfulClasses.ken.md` (for
example `ord_int_refl`, `ord_int_antisym`, `ord_int_trans`, `ord_int_total`),
each at its field's type. The instance consumes them by name. Prose that
enumerates the anonymous fields is updated to match.

## Acceptance

- **AC-0 (census, then build).** List every consumer of the four law fields
  by resolved id across `catalog/`, `crates/*/tests`, `examples/` and
  `conformance/`, with its elaborated term.
- **AC-1.** `instance Ord Int` has no `= Axiom` field. `trusted_base()`
  cardinality is unchanged, with four anonymous entries becoming four named
  ones. Every AC-0 consumer's elaborated term is unchanged except for the
  constant it references.
- **AC-2 (controls).**
  - Changing one axiom's statement, for example dropping `antisym`'s second
    premise, makes the instance fail to check.
  - `IsTrue` and `bool_or` stay at zero occurrences in
    `crates/ken-kernel/src/`.

## Stop conditions

- Any kernel, `trusted_base()` count or spec change (an operator question).
- A consumer whose elaborated term changes beyond the referenced constant.
  Stop to the Architect with the site.
- Out of scope: `Eq Int` and `DecEq Int` (already proved),
  `string_to_list_char_retraction`, `BytesRoundTripLaw`, and
  `string_ord_leq`'s laws, which are proved and must not become axioms.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.
