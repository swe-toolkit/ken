---
id: CAT-MAP-CANONICAL-LEQ-NAT
title: "Replace Map's private leq_nat reimplementation with the canonical Core.Classes.LawfulClasses.leq_nat, if Map can import it without changing its public surface, trust closure or pinned provider edges"
status: ready
owner: foundation
size: S
gate: architect
tier: T2
depends_on: []
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
- **Sequencing.** `LANG-SESSION-SCOPE`'s Map increment landed at
  `b6402c71c`. Its Map-owned example fence (`Map.ken.md:15546`) is
  import-free because of this collision and does not use `leq_nat`; the
  private definition is still at `:5787`.

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

  The Architect rules on the route. **Ruled** (`evt_78t51c0gc83vv`): delete
  Map's `leq_nat` and its `refl`/`trans`/`antisym` proofs, import `leq_nat`
  selectively from `LawfulClasses` (already a direct provider), and keep
  Map's private `total_leq_nat` byte-unchanged over the imported comparator.
  **Why the one local remains:** the provider's `total_leq_nat` is private
  and pinned non-importable (`cat_order_pub_export.rs`,
  `ds2_ord_nat_acceptance.rs`), and the public `proof total for leq_nat`
  erases the direction that `map_total_leq_nat_preserves_proof_relevant_or_tags`
  observes. It duplicates a private helper, not a public export.
- **AC-1.** Map's public nine-name surface, export set, `trusted_base` and
  provider-edge set are byte-identical. Map's owned inventory, which
  includes private names, loses exactly `leq_nat`, `leq_nat::refl`,
  `leq_nat::trans` and `leq_nat::antisym` and gains nothing. Pins key on
  identity through `provider_owned_id`, never on a raw GlobalId that the
  deletion shifts.
- **AC-2.** The Map and catalog suites stay green (Full CI).

## Stop conditions

- The change would add a Map public name or widen its interface.
- Any kernel, `trusted_base()` or spec change.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.

## SYMPTOM INVENTORY (append one line per hard-stop; never rewrite history)

1. Anti-duplication criterion (c) was prescribed as "a well-formed local
   copy reddens the pin" without checking the equivalence plane; kernel
   conversion does not identify distinct recursive declarations, so a
   renamed self-recursive copy passes. Keyed on definitional (not
   structural) equivalence (evt_401q33apra6qw, ruled evt_ntwgkpzk0hga:
   disclosed as THE GAP in the pin; factoring review is the backstop).
   Next trigger 3.
