---
id: CAT-MAP-CANONICAL-LEQ-NAT
title: "Replace Map's private leq_nat reimplementation with the canonical Core.Classes.LawfulClasses.leq_nat, if Map can import it without changing its public surface, trust closure or pinned provider edges"
status: merged
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
  Map's `leq_nat` and its `refl`/`trans`/`antisym` proofs and import
  `leq_nat` selectively from `LawfulClasses` (already a direct provider);
  landed `c3896b72e`. **Re-ruled post-merge** (`evt_1r2xkx93bgysg`, on
  Adversary `evt_7yrgfmy1wwdce`): the retention of Map's private
  `total_leq_nat` does not stand. Nothing in Map uses it, and
  `map_total_leq_nat_preserves_proof_relevant_or_tags` resolved the flat
  name to LawfulClasses' private copy, so it never measured Map's.
  Increment 2 deletes `total_leq_nat`, the now-unused `leq_nat` import line
  and that test; the `(bool_and)` and `(Ord)` import lines stay
  byte-unchanged.
- **AC-1.** Map's public nine-name surface, export set, `trusted_base` and
  provider-edge set are byte-identical; LawfulClasses stays a direct
  provider through `bool_and` and `Ord`. Map's owned inventory, which
  includes private names, loses exactly `leq_nat`, `leq_nat::refl`,
  `leq_nat::trans`, `leq_nat::antisym` and `total_leq_nat` and gains
  nothing. Pins key on identity through `provider_owned_id`, never on a raw
  GlobalId that the deletion shifts.
- **AC-2.** The Map and catalog suites stay green (Full CI).

- **AC-3 (recut after stop 3, Steward evt_8entn03svwk2).** Main is out of
  conformance with §58 §§2 and 8 since increment 1. The repair is a Spec
  amendment naming LawfulClasses as the Nat-order provider, not a revert,
  unless the census below finds a plane where the owning module is
  load-bearing. Before any further route ruling, a plane-complete ownership
  census of `leq_nat`, its three proofs and `total_leq_nat` is posted: `spec/`
  and `conformance/` (Spec leader), and `catalog/`, `docs/program` and
  `crates/` by name, source-text mechanism and identity (Foundation). The
  Architect then rules the route once against the census and the Research
  advisory. Candidate `7c0d3875` is held.

## Stop conditions

- The change would add a Map public name or widen its interface.
- Any kernel, `trusted_base()` or spec change.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.

## Closeout

Landed in three commits. Increment 1 (`c3896b72e`, respin `19dfa5e7d` after
the CI red on `a1a538d85`) imports the canonical `leq_nat` and deletes Map's
duplicate and its three proofs. The Spec amendment (`a215e140d`) names
LawfulClasses as the Nat-order provider in §58 §§2 and 8 and the CAT-4 seed.
Increment 2 (`8dd81d7b6`) deletes Map's private `total_leq_nat`, the unused
import and the or-tags test that never measured Map's copy. Map now defines no
Nat order; main conforms to §58 again. Carried to the Architect: the first
client that needs a public Or-valued Nat totality (`evt_7zwcp751eb3zd`).

## SYMPTOM INVENTORY (append one line per hard-stop; never rewrite history)

1. Anti-duplication criterion (c) was prescribed as "a well-formed local
   copy reddens the pin" without checking the equivalence plane; kernel
   conversion does not identify distinct recursive declarations, so a
   renamed self-recursive copy passes. Keyed on definitional (not
   structural) equivalence (evt_401q33apra6qw, ruled evt_ntwgkpzk0hga:
   disclosed as THE GAP in the pin; factoring review is the backstop).
2. A source-text consumer (an exact-line import mutation in
   `cat_map_bool_and_owner`) was missed because the AC-0 consumer sweep was
   by name, not by mechanism; the combined import line removed the line the
   control replaces. Keyed on exact source-text spelling of an edited line
   (CI red run 36351135377, ruled evt_69sc92fjt908e: split the import, test
   unchanged).
3. Increment 2's route (delete Map's `total_leq_nat`) was ruled without
   reading `spec/50-stdlib/58-maps-sets-relations.md` §§2 and 8, which name
   Map as the provider of `leq_nat` and its four order results; increment 1
   had already removed four of them. Keyed on the owning module named in a
   normative spec, not on code-name consumers (QA evt_2ztee6q31p0vk, Spec
   evt_5my9xhynmc1fe; Architect evt_2cnccr7d2t7cb: §1b predicate is an
   ownership move scoped from one plane; Research called). Next trigger 4.
