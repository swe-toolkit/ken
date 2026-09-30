---
id: CAT-LIST-APPEND-LENGTH-CANONICAL
title: "The length-of-append law is proved twice in the catalog, once in Derived and once privately in Map. Publish Derived's as the canonical attached law and retire Map's private copy, deriving its one external use through add commutativity"
status: ready
owner: foundation
size: S
tier: T1
gate: architect
depends_on: [CAT-DERIVED-STRING-VIEW-LAWS]
blocks: []
github: null
origin: "Architect ruling evt_7gvrbta87rt6z on the CAT-DERIVED-STRING-VIEW-LAWS semantic-duplicate finding (Foundation QA evt_5eapcc68m6cac): ba4f1b5a8 stands, Derived owns the law, and Map's private copy is retired in a follow-up. Steward-filed per COORDINATION section 2."
---

# One length-of-append law, owned by Derived

## Objective

The catalog states the length of a `list_append` once, as a public law
attached to Derived's operations.

## Settled inputs (Architect `evt_7gvrbta87rt6z`, measured on `fe3f08860`)

- **Derived owns the operations.** Map imports `list_append` and `length`
  from `Data.Collections.Derived` (`Map.ken.md:102`).
- **The two copies.** Derived's private `length_append` proves
  `add (length xs) (length ys)`. It lands with
  `CAT-DERIVED-STRING-VIEW-LAWS`. Map's private
  `list_append_length_swapped` (`Map.ken.md:18470`) proves
  `add (length right) (length left)`. They are the same law up to
  `add::comm` (`Nat/Arithmetic.ken.md:55`).
- **Map's uses.** `:18482`, the recursive call inside itself, and `:18502`
  in `raw_outer_keys_length`.
- **Statement sweep, done.** Searching for `Equal Nat (length _ (list_append`
  over `catalog/` and `library/` finds only these two laws and Derived's
  `append_length_snoc` (`:882`). The last is the single-element special
  case, not a duplicate.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

- Derived's `length_append` becomes a public attached law under the
  module's law-naming convention, such as `list_append::length`, next to
  `list_append::assoc` and `list_append::left_unit`.
- Map's `list_append_length_swapped` is deleted. `raw_outer_keys_length`
  uses the Derived law composed with `add::comm`.

## Acceptance

- **AC-1.**
  - Map and Derived check.
  - `trusted_base()` is unchanged.
  - A typed consumer outside Derived applies the public law at its
    general proposition.
- **AC-2 (controls).**
  - The catalog census shows no verdict change.
  - The statement grep above finds exactly one length-of-append law, plus
    `append_length_snoc`.
  - Replacing the public law with a reflexive filler reddens its consumer.

## Stop conditions

- The public name collides with a prelude or built-in name: stop to the
  Architect (operator 2026-09-25, built-ins are fixed).
- Any change to `list_append`, `length` or `add`.
