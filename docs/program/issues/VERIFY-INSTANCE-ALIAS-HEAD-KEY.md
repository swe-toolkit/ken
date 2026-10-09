---
id: VERIFY-INSTANCE-ALIAS-HEAD-KEY
title: "An instance declared at a transparent def alias (def N2 = Nat; instance E N2) is keyed on the alias's own global id, not on head(A) after unfolding, so instance E N2 and instance E Nat are both admitted with no OverlappingInstances, and where E Nat misses the alias instance with NoInstance. Key instance registration, overlap and resolution on the unfolded head"
status: merged
owner: verify
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary M8 finding evt_2em0e8ma3sgwb on b5ba6619c (VERIFY-STRUCTURAL-HEAD-INSTANCE-IDENTITY); pre-existing, not introduced by that merge. Spec 39 sections 6.1-6.2 key the instance registry on head(A), the outermost type constructor. Steward-filed per COORDINATION section 2."
---

# An instance at an alias is keyed on the head it unfolds to

## Objective

Instance registration, the overlap check and resolution agree on `head(A)`
after transparent `def` aliases unfold. One class at one head has at most
one instance, however its head is spelled.

## Settled inputs (Adversary `evt_2em0e8ma3sgwb`, at `b5ba6619c`)

- **The witness.** `class E carrier { label : String }`, `def N2 = Nat`,
  `instance E N2 { label = "a" }` and `instance E Nat { label = "b" }`
  are both admitted, as `E_instance_N2` and `E_instance_Nat`, with no
  `OverlappingInstances`. `fn up (h : Nat) : N2 = h` and its reverse both
  check, so `N2 ≡ Nat`.
- **Resolution follows the spelling.** With only `instance E N2`
  registered, `const lb : String where E Nat = d.label` fails with
  `NoInstance { ty: "Nat" }`.
- **Other shapes** that admit both instances: `def F = Nat -> Nat`
  against `E (Nat -> Nat)`; `E (N2 -> Nat)` against `E (Nat -> Nat)`;
  across modules, `module B { import A (E, T) def T2 = T instance E T2 }`
  beside A's `E T`.
- **The site.** `instance_head_key` (`crates/ken-elaborator/src/elab.rs:15382`)
  resolves the alias to `Global(<def id>)` from the checked global, or
  `core_type_head_id` on the core before whnf. The overlap test at
  `:15457-15458` therefore never compares the alias with its target.
- **The rule.** Spec 39 §6.1 and §6.2: the registry is keyed on `head(A)`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (measure only).** Done: `evt_53ymgk8e23mra`, at `85e651a44`. The
   catalog has one alias-headed instance family, Char's.
2. **The ruled repair** (Architect `evt_2e19wfx773mxv`, key mechanism
   corrected by `evt_4w9d7wbnr4gw9`). The rule (39 §6.1):
   - A bare nullary alias names its whole body.
   - An applied head keeps its own outermost constructor, following only
     renaming aliases (`alias_head_root`). `(Ord, Pair)` keys on `Pair`;
     a type function is never reduced.
   - A named refinement stops the unfolding and keys on its
     `RefinementFacts::refinement_root`, since 18a §5.9.1 makes `Char`
     distinct from `Int`.
   - A structural key is normalized.
   - Dictionary names are unchanged: the spelled head, as in
     `E_instance_N2` and `Ord_instance_Char`.

   One `instance_head_key` (or its id-only form) serves all six
   head-identity sites:
   - registration and overlap;
   - resolution;
   - sub-constraint resolution, keyed on the constraint's own unreduced
     head (`InstanceConstraintInfo::head_core`), not the class-applied
     carrier;
   - the orphan check, which tests the owner module of the key's Global
     id;
   - the projection-purity lookup `constraint_instance_id`;
   - fixed-head matching in `match_instance_head_core`: equal keys, then
     an alias's body compared by conversion, and any other fixed head
     requested bare, so `List` never matches `List Nat`.

## Acceptance

- **AC-1.** Each row is red at base and green after the repair:
  - (a) For the witnesses N2/Nat, F/(Nat -> Nat) and
    (N2 -> Nat)/(Nat -> Nat), the second instance refuses with
    `OverlappingInstances`. With only the alias instance registered,
    `where E Nat` resolves to it.
  - (b) `instance E Char` and `instance E Int` both admit. Then
    `def C2 = Char` plus `instance E C2` refuses as overlapping `E Char`.
  - (c) Module A has `class E`, `data T`, `def T2 = T` and
    `instance E T2`. In module B, which imports `(E, T)` from A, a
    `where E T` use resolves to A's dictionary.
  - (d) The owner spells its own instance through its own alias: module C
    has `pub class E`, and module A, which imports `E` from C, has
    `pub data T = MkT`, `def T2 = T` and `instance E T2`. A admits.
    Base refuses it with `OrphanInstance` (Architect `evt_5endccp1f2j94`).
- **AC-2 (control).** Instances at distinct heads, the structural-head and
  named-head rows, and every catalog package keep their results; any moved
  dictionary name, owner or hash is a stop to the Architect. Also:
  - es4 `char_ord_laws_reject_missing_law_field` still refuses for the
    missing field;
  - lang_refinement_introduction_coverage `Pick Char` and `Pick Int` both
    still admit;
  - C1 `sound-deceq-char` is unaffected;
  - (d') `imported_alias_cannot_evade_orphan_rule`:
    `module B { import A (E, T) def T2 = T instance E T2 }`, where A has
    no `E T`, still refuses with `OrphanInstance`. An alias registers no
    owner, so this is green at base too.
- **AC-3 (mutations, QA).** Each reddens and is restored:
  - keying on the head before whnf reddens AC-1 (a);
  - deleting the `refinement_root` stop in both `instance_head_key` and
    `alias_head_root` refuses row (b) and the catalog's
    `Ord Char` and `DecEq Char` against `Ord Int` and `DecEq Int`.

## Stop conditions

- A catalog or corpus package is newly refused: stop with the list.
- Any kernel, `trusted_base()` or spec change.

## Carry

To the W5 Σ demote, not built here:
- normalizing a structural key collapses a nested refinement, so
  `E (Char -> Int)` overlaps `E (Int -> Int)`, which refuses rather than
  admits;
- two refinement aliases with identical predicates key separately;
- a transparent type function whose result is a refinement keys by the
  rule above, not by the refinement root;
- `where Pick PIB` (`def PIB = Pair Int Bool`) against
  `instance Pick (Pair a b)` refuses with `NoInstance`, because the
  surface matcher does not see through the alias spelling; base refuses
  it too;
- `resolve_instance_dictionary_by_head_id` (the comparison operators and
  the core-path sub-constraint) keys on the head of `whnf(carrier)`. It
  agrees for plain aliases, and for Char it selects `Ord Int` as base
  does. Re-keying it moves catalog hashes;
- a named refinement's root has no owner, because the TypeAlias arm
  registers none, so `def C2 = {x:Int | …}` with `instance E C2` for an
  imported `E` refuses with `OrphanInstance`. Base refuses it too, and
  the catalog's `Ord Char` and `DecEq Char` admit on the class-owner arm.

These disappear once Char's core is the subset Σ, and then the
refinement stop can be deleted.

## Closeout

Merged `6d5eeb9b7` from exact `72c634a40` (PR #4629). QA
`evt_2pdyvbm6wve4x`, Architect `evt_1aa1026q3ys29`, Decision
`dec_471mf1khxa8sm`. Instance registration, the overlap check and
resolution key on the semantic head after transparent aliases unfold, so
`instance E N2` beside `instance E Nat` is refused as overlapping and `E
Nat` resolves the alias instance. A head that does not elaborate keeps
base's `named_head_id` orphan refusal ahead of the elaboration error: the
first candidate inverted that order, and the Architect's change request
`evt_735y1br6ga2sv` restored it. AC-1 7/7 against 1/7 at base; elaborator
`--lib` 613/613. §1a: 1. The Carry list above stays with the W5 Σ demote.
