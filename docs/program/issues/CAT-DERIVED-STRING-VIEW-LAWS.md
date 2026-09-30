---
id: CAT-DERIVED-STRING-VIEW-LAWS
title: "Prove the Derived String concat and slice laws with no new trust: each law either is stated on the List Char side or carries the round trip for its specific list as an explicit premise, because the unconditional String-level equations are false under NFC"
status: merged
owner: foundation
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-09-30 ('concur with recs') on the Architect's ruling evt_h489cs74b8j: the section postulate string_to_list_char (list_char_to_string cs) = cs is false by NFC and is not admitted; the concat and slice laws return for restatement at zero TCB. Foundation leader evt_2ch4k71vyf8aq: the laws belong to catalog/packages/Data/Collections/Derived.ken.md section 4.6 and no active WP owns them. L3 proof backfill under the operator's 2026-09-13 rulings. Steward-filed per COORDINATION section 2."
---

# Derived String concat and slice laws, at zero trust

## Objective

`concat` and `slice` in `Derived.ken.md` §4.6 carry checked laws that are
true under the spec's NFC conversion. They add no `trusted_base()` entry.

## Settled inputs (Architect `evt_h489cs74b8j`)

- **`concat a b = l2s (list_append (s2l a) (s2l b))`** and `slice` is
  `l2s` of a `take`/`drop` window (spec 37 §2.5; `Derived.ken.md` §4.6).
- **`List Char -> String` is "encode UTF-8, then NFC-normalize"** (spec 37
  line 88). Native and runtime-IR both normalize; the interpreter does not.
- **The unconditional laws are false.** With `a = "e"` and
  `b = "\u{301}"`, `s2l (concat a b) = [U+00E9]`, not `s2l a ++ s2l b`, and
  `char_length` additivity fails, 1 against 2.
- **The only landed axiom** is `string_to_list_char_retraction`,
  `l2s (s2l s) = s` (`StringBijection.ken.md:15`). Its converse is not
  true and is not admitted.

## Deliverable

Private theorems beside `concat` and `slice`. Each one takes one of the two
honest routes:
- **(a)** a law stated on the `List Char` side, over the list the String op
  converts; or
- **(b)** a String-level law whose premise is the round trip for its
  specific list, `Equal (List Char) (s2l (l2s cs)) cs`.

## Acceptance

- **AC-0 (statement set; no build).** Propose the exact law list and each
  law's route. The Architect rules it before any proof.
- **AC-1.** Every ruled law checks. A typed consumer applies each at its
  general proposition. Trust is unchanged.
- **AC-2 (controls).**
  - The `"e"`/`"\u{301}"` pair satisfies no route-(b) premise, so a proof
    with its premise deleted does not check.
  - Replacing a law with a reflexive filler reddens the consumer.

## Stop conditions

- Any law that needs a new postulate, axiom or `trusted_base()` entry: an
  operator question. The NFC-absorption postulate is not authorized.
- Any change to `concat`, `slice` or the conversion primitives.

## Closeout

Merged as `111442ba8` (PR #4394).
- **AC-0.** The Architect ruled six private laws (`evt_27cp20nn1twrx`):
  `length_append`, `length_drop`, `concat_char_count`,
  `slice_char_count`, `concat_view` and `slice_view`. The four String
  laws take route (b), each with the round-trip premise for its list.
- **AC-1.** Each law checks and has an owner-local generic consumer.
  `trusted_base()` is unchanged when Derived loads over its providers.
  `cat_derived_string_view_laws.rs` pins both.
- **AC-2.** Paired reject fences refuse the unconditional forms. Because
  `list_char_to_string` is conversion-opaque, they show the equations are
  not definitional; they do not observe NFC absorption (Adversary M8
  `evt_qg7bq28ghbzc`, with an ASCII control that refuses identically).
- **Harness edge.** The first candidate `ba4f1b5a8` went red on four
  Rosetta examples: `rosetta.rs` `collections_prelude` lacked Derived's new
  `Data.Numeric.Nat.Arithmetic (add)` provider. The repair carries it.
- **Carried.** `length_append` becomes the public `list_append::length`
  in `CAT-LIST-APPEND-LENGTH-CANONICAL`.
