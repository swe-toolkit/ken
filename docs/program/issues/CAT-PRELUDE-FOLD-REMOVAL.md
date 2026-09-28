---
id: CAT-PRELUDE-FOLD-REMOVAL
title: "Delete the prelude's list fold, a convenience name with no catalog consumer that collides by spelling with Data.Collections.Map's own fold; move it into Data.Collections.Derived only if the consumer census finds a list-fold user; fourth L3 slice of the minimal-prelude program"
status: ready
owner: foundation
size: S
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Operator rulings 2026-09-25 (minimal fixed prelude; convenience names are technical debt moved to packages; a prelude name cannot be rebound). CAT-COLLECTIONS-MAP-FILTER-PRELUDE-MOVE left prelude fold out of scope as 'no catalog consumer; a separate removal'. Steward-filed per COORDINATION section 2."
---

# Remove the prelude's list fold

## Objective

The prelude no longer declares `fold`, and every former reader resolves a
catalog definition or has none.

## Settled inputs (read at `b63b22ea7`)

- **The declaration.** `crates/ken-elaborator/src/prelude.rs:519` elaborates
  `fn fold (a b : Type) (f : a → b → b) (z : b) (xs : List a) : b`. It sits
  inside the `combinator_trusted_before`/`after` bracket (`:516`, `:528`),
  whose population is positional and is today exactly `fold` and `zip`.
- **The collision.** `catalog/packages/Data/Collections/Map.ken.md:120`
  declares its own `fold` over `Tree k v`, a different function under the
  same spelling. Under the operator's no-rebinding rule, which the
  LANG-SESSION-SCOPE flip enforces, that declaration is an error while the
  prelude keeps `fold`.
- **Consumers.** `CAT-COLLECTIONS-MAP-FILTER-PRELUDE-MOVE` recorded no catalog
  consumer. Test, r-layer, example and conformance consumers are unmeasured.
  The word `fold` appears widely in prose, so the census is by resolved
  identity, not by grep.
- **Out of scope:** `zip` and `Prod` (runtime and ABI keyed; an L2 floor
  question), and `And`/`and_*`/`is_sorted` (`CAT-AND-SORTED-PRELUDE-MOVE`).

## Deliverable

The prelude `fold` declaration is deleted. The combinator bracket keeps `zip`
and its zero-trust assertion. Map's `fold` stays as written. If the census
finds a reader of the list fold, `Data.Collections.Derived` gains it as a
structural op, and that reader imports it.

## Acceptance

- **AC-0 (census, then ruling; no edit).** List every reader of the prelude
  `fold` by resolved `GlobalId` across `catalog/`, `crates/*/tests`,
  `r_layer_tests`, `examples/`, `conformance/` and the CLI fixtures (Check 3).
  Say whether a Derived home is needed. The Architect rules before any edit.
- **AC-1.** `ElabEnv::new()` registers no `fold`. Map's `fold` elaborates and
  resolves as Map's own inside Map and in its importers. The combinator
  bracket's delta assertion still holds over `zip`. Any census reader is
  green on its catalog import.
- **AC-2 (control).** Restoring the prelude declaration turns the AC-1 "no
  `fold` registered" row red, and a Map-importing client that names `fold`
  resolves it to one identity either way. Prelude ids after the deleted one
  shift by one, and every pin is re-keyed by identity, not by number.

## Stop conditions

- Any kernel, `trusted_base()` or spec change (an operator question).
- A reader that needs the list fold in the prelude, such as a runtime or ABI
  key, is a stop to the Architect with its site.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.
