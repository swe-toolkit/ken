---
id: CAT-LIST-APPEND-LENGTH-CANONICAL
title: "The length-of-append law is proved twice in the catalog, once in Derived and once privately in Map. Publish Derived's as the canonical attached law and retire Map's private copy, deriving its one external use through add commutativity"
status: active
owner: foundation
size: S
tier: T1
gate: architect
depends_on: [CAT-DERIVED-STRING-VIEW-LAWS]
blocks: []
github: null
origin: "Architect rulings evt_7gvrbta87rt6z and evt_xqx26mskfhjv (the fence fold) on the CAT-DERIVED-STRING-VIEW-LAWS semantic-duplicate finding (Foundation QA evt_5eapcc68m6cac): ba4f1b5a8 stands, Derived owns the law, and Map's private copy is retired in a follow-up. Steward-filed per COORDINATION section 2."
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
- **Statement sweep.** Searching for `Equal Nat (length _ (list_append`
  over `catalog/` and `library/` finds these two laws and two
  specializations, which are not duplicates (Foundation QA
  `evt_5kdh5tsn759k0`, Steward ruling `evt_2n49gqc2vmzm4`):
  - Derived's `append_length_snoc` (`:882`), the single-element case;
  - Map's private `list_append_remove_length` (`Map.ken.md:19191`),
    `length (prefix ++ (stored :: suffix)) = Suc (length (prefix ++
    suffix))`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

- Derived's `length_append` becomes a public attached law under the
  module's law-naming convention, such as `list_append::length`, next to
  `list_append::assoc` and `list_append::left_unit`.
- Map's `list_append_length_swapped` is deleted. `raw_outer_keys_length`
  uses the Derived law composed with `add::comm`.
- **Folded: the String-law fences say what they measure** (Architect
  `evt_xqx26mskfhjv` on Adversary M8 `evt_qg7bq28ghbzc`, at `111442ba8`).
  `list_char_to_string` is conversion-opaque, so the §4.6 reject fences
  measure "not definitional", not NFC. In `Derived.ken.md`:
  - rename `derived_reject_concat_nfc_round_trip` to
    `derived_reject_round_trip_not_definitional`, and reword its prose to
    the opacity reason: `s2l (l2s cs) = cs` holds by `Refl` for no `cs`,
    ASCII and `Nil` included, and the count equations do not hold by
    `Refl`. Drop "an NFC boundary can change the character count" as the
    refusal's stated reason;
  - keep one reject fence beside it on the ASCII `"a"`/`"b"` control;
  - add an accepting fence where NFC is observable, at literal admission:
    `Equal (List Char) (string_to_list_char "e\u{301}")
    (string_to_list_char "\u{e9}")` by `Refl`;
  - keep the NFC motivation, attributed to literal admission: a decomposed
    list such as `['e', U+301]` is the view of no String literal, which is
    why the laws take the round trip as a premise;
  - add one §4.6 sentence: "No closed term currently discharges the
    round-trip premise; these laws apply to a consumer that obtains it
    (e.g., from a future conditional section certificate, an operator TCB
    decision)."

## Acceptance

- **AC-1.**
  - Map and Derived check.
  - `trusted_base()` is unchanged.
  - A typed consumer outside Derived applies the public law at its
    general proposition.
- **AC-2 (controls).**
  - The catalog census shows no verdict change.
  - The statement grep above finds exactly one general additive
    length-of-append law, plus the two specializations
    `append_length_snoc` and `list_append_remove_length`.
  - Replacing the public law with a reflexive filler reddens its consumer.
  - The ASCII control refuses with the same error as the renamed fence,
    and the literal-admission fence is accepted.

## Stop conditions

- The public name collides with a prelude or built-in name: stop to the
  Architect (operator 2026-09-25, built-ins are fixed).
- Any change to `list_append`, `length` or `add`.
- Any discharge route for the round-trip premise. It needs a new
  `trusted_base()` entry, which is an operator question, and only after
  `LANG-REFINEMENT-INTRODUCTION-OBLIGATION` lands.
