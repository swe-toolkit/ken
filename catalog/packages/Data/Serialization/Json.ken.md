# `Json` — integral-number JSON values

`Json` is the ordinary six-constructor value model for DS-9's accepted
integral-number JSON subset. Arrays and objects preserve their source order as
lists.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws & proofs](#4-laws--proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust & derivation](#7-trust--derivation)

## 1. Motivation

A codec for the accepted integral-number JSON subset needs a value carrier that
represents every structural case directly. `Json` keeps null, Boolean, number,
string, array, and object values distinct, so later encoders and decoders can
cover that subset by ordinary structural recursion.

## 2. Definition

Arrays contain JSON values directly. Objects are ordered lists of string/value
pairs; duplicate-key rejection belongs to decoding rather than to the carrier.

```ken
data Json : Type where {
  JsonNull : Json;
  JsonBool : Bool → Json;
  JsonNumber : Int → Json;
  JsonString : String → Json;
  JsonArray : List Json → Json;
  JsonObject : List (Pair String Json) → Json
}

fn char_cursor_remaining (cur : List Char) : Nat = cursor_list_length Char cur

fn char_cursor_peek (cur : List Char) : Option Char =
  match cur {
    Nil ↦ None Char;
    Cons head tail ↦ Some Char head
  }

fn char_cursor_advance (cur : List Char) : List Char =
  match cur {
    Nil ↦ Nil Char;
    Cons head tail ↦ tail
  }

fn char_cursor_locate (cur : List Char) : Nat = char_cursor_remaining cur

const char_cursor_ops : CursorOps (List Char) Char Nat =
  MkCursorOps
    (List Char)
    Char
    Nat
    char_cursor_remaining
    char_cursor_peek
    char_cursor_advance
    char_cursor_locate
```

## 3. Using it

`JsonNull` is null. `JsonBool`, `JsonNumber`, and `JsonString` introduce the
three scalar leaves. `JsonArray` introduces a `List Json`, while `JsonObject`
introduces a `List (Pair String Json)`.

The character cursor treats the unconsumed suffix as its carrier. Its location
is the remaining character count, so a parser can report the exact suffix
position without routing the proof-bearing core through a byte cursor.

## 4. Laws & proofs

The cursor laws follow by case analysis on the unconsumed list. A successful
peek exposes a `Cons`; advancing that branch removes exactly one constructor.
The empty branch is the only one with zero remaining input.

```ken
theorem char_cursor_lt_suc (n : Nat) : Equal Bool (cursor_nat_lt n (Suc n)) True =
  match n {
    Zero ↦ Proved;
    Suc rest ↦ char_cursor_lt_suc rest
  }

theorem char_cursor_peek_has_remaining
    : CursorPeekHasRemaining (List Char) Char Nat char_cursor_ops =
  λcur.
    match cur {
      Nil ↦ λvalue. λpeeked. absurd peeked;
      Cons head tail ↦ λvalue. λpeeked. Proved
    }

theorem char_cursor_advance_progress
    : CursorAdvanceProgress (List Char) Char Nat char_cursor_ops =
  λcur.
    match cur {
      Nil ↦ λvalue. λpeeked. absurd peeked;
      Cons head tail ↦ λvalue. λpeeked. char_cursor_lt_suc (char_cursor_remaining tail)
    }

theorem char_cursor_end_valid : CursorEndValid (List Char) Char Nat char_cursor_ops =
  λcur.
    match cur {
      Nil ↦ λempty. Proved;
      Cons head tail ↦ λempty. absurd empty
    }

theorem char_cursor_laws : CursorLaws (List Char) Char Nat char_cursor_ops =
  and_intro
    (CursorPeekHasRemaining (List Char) Char Nat char_cursor_ops)
    (And
      (CursorAdvanceProgress (List Char) Char Nat char_cursor_ops)
      (CursorEndValid (List Char) Char Nat char_cursor_ops))
    char_cursor_peek_has_remaining
    (and_intro
      (CursorAdvanceProgress (List Char) Char Nat char_cursor_ops)
      (CursorEndValid (List Char) Char Nat char_cursor_ops)
      char_cursor_advance_progress
      char_cursor_end_valid)
```

## 5. Design notes

**The representation is the ordinary nested inductive.** Arrays and objects
contain recursive `Json` values through the strictly positive `List` and
`Pair` paths. There is no finite-function, flattened, Church-encoded, or
postulated substitute.

**The current number domain is integral JSON numbers.** `JsonNumber` stores an
arbitrary-precision `Int`, matching the landed numeric parser. JSON numbers
with a fractional part or exponent are outside the accepted domain rather than
a silent widening or narrowing of the round-trip claim.

**Objects preserve order and duplicates structurally.** The carrier can
represent any ordered member list. The decoder's typed boundary will reject
duplicate keys; the carrier itself does not add a malformed internal state.

## 6. References

- **RFC 8259, The JavaScript Object Notation Data Interchange Format** — the
  external syntax and six value categories represented by this carrier.
- **Ken nested inductives** — `spec/10-kernel/14-inductive.md` §8.5, which
  admits recursive occurrences through checked strictly positive parameter
  paths.

## 7. Trust & derivation

**Public API (stable names):** `Json`, `JsonNull`, `JsonBool`, `JsonNumber`,
`JsonString`, `JsonArray`, `JsonObject`, `char_cursor_ops`, and the
`char_cursor_peek_has_remaining`, `char_cursor_advance_progress`,
`char_cursor_end_valid`, and `char_cursor_laws` proofs.

**Source map:**

| Reader task | Section |
|---|---|
| Inspect the carrier | [§2](#2-definition) |
| Inspect the character cursor | [§2](#2-definition) |
| Review its laws | [§4](#4-laws--proofs) |
| Understand the number domain | [§5](#5-design-notes) |
| Locate later codec work | [§3](#3-using-it) |

**Derivation path from built-ins.** `Json` is an ordinary strictly positive
inductive assembled from the built-in `Bool`, `Int`, `String`, `List`, and
`Pair` types. Its recursive occurrences follow `List`'s checked positive
parameter and the transparent non-dependent `Pair`/Sigma structure. The cursor
dictionary and its laws instantiate the carrier-neutral parsing abstraction
using transparent structural recursion on `List Char`.

**`trusted_base()` delta: zero.** This file declares no primitive, opaque
constant, postulate, or `Axiom`; the carrier is checked by ordinary inductive
admission.

**Validation evidence.** The ordered package check elaborates the parsing
dependency before this tangled source. Focused acceptance resolves the family,
every constructor, the cursor dictionary, and all four proof witnesses as real
registered kernel globals, and checks concrete cursor behavior.
