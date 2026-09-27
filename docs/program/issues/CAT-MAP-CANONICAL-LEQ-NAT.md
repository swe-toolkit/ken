---
id: CAT-MAP-CANONICAL-LEQ-NAT
title: "Replace Map's private leq_nat reimplementation with the canonical Core.Classes.LawfulClasses.leq_nat, if Map can import it without changing its public surface, trust closure or pinned provider edges"
status: ready
owner: foundation
size: S
gate: architect
tier: T2
depends_on: [LANG-SESSION-SCOPE]
blocks: []
github: null
origin: "Architect ruling 2026-09-27 (evt_4xtksh29xcm9k, LANG-SESSION-SCOPE Map stop 13): Map defines its own private leq_nat alongside Core.Classes.LawfulClasses.leq_nat, a candidate redundant reimplementation under catalog factoring; the Language diagnostic surfaced it as an AmbiguousReference. Steward-filed per COORDINATION section 2."
---

# Map's canonical leq_nat

## Objective

Map uses the one canonical `leq_nat`, or the frame records why it cannot.

## Settled inputs (measured on `ab9faa5d2`)

- `catalog/packages/Data/Collections/Map.ken.md:5787` defines a private
  `fn leq_nat (m : Nat) (n : Nat) : Bool`. Map's prose at `:5706` uses it
  with the reflexivity, transitivity, antisymmetry and totality laws.
- `catalog/packages/Core/Classes/LawfulClasses.ken.md:489` defines
  `pub fn leq_nat` with the same signature.
- Importing the canonical one into a Map `ken example` fence fails with
  `AmbiguousReference { name: "leq_nat" }` (Language, `evt_6rnzmys0s2nff`).
- **Sequencing.** `LANG-SESSION-SCOPE` is editing `Map.ken.md` now, so this
  waits for that WP's Map increment to land.

## Deliverable

Either Map imports the canonical `leq_nat` and deletes its own, with its
private laws re-proved against the canonical definition or reused from
LawfulClasses, or a recorded reason it cannot, for example a provider-edge or
import-cycle constraint.

## Acceptance

- **AC-0.** The frame is re-measured against its settled inputs:
  - Map's existing provider edges;
  - whether `LawfulClasses` is already in Map's import closure;
  - every Map consumer of `leq_nat` and of its laws (Check 3).

  The Architect rules on the route.
- **AC-1.** Map's public nine-name surface, exports, loader-visible
  inventory and trust closure are byte-identical. A new provider edge is
  added only if the Architect accepts it at AC-0.
- **AC-2.** The Map and catalog suites stay green (Full CI).

## Stop conditions

- The change would add a Map public name or widen its interface.
- Any kernel, `trusted_base()` or spec change.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.
