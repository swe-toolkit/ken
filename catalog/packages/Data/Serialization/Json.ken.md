# `Json` — JSON values

`Json` is the ordinary six-constructor value model used by Ken's JSON codec.
Arrays and objects preserve their source order as lists.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws & proofs](#4-laws--proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust & derivation](#7-trust--derivation)

## 1. Motivation

A JSON codec needs a value carrier that represents every structural case
directly. `Json` keeps null, Boolean, number, string, array, and object values
distinct, so later encoders and decoders can cover the format by ordinary
structural recursion.

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
```

## 3. Using it

`JsonNull` is null. `JsonBool`, `JsonNumber`, and `JsonString` introduce the
three scalar leaves. `JsonArray` introduces a `List Json`, while `JsonObject`
introduces a `List (Pair String Json)`.

This first increment defines only the value carrier. The `List Char` encoder,
decoder, cursor instance, and round-trip proof are later DS-9 increments.

## 4. Laws & proofs

The value carrier introduces no laws. The codec increment will state and prove
round-trip behavior for each constructor separately.

## 5. Design notes

**The representation is the ordinary nested inductive.** Arrays and objects
contain recursive `Json` values through the strictly positive `List` and
`Pair` paths. There is no finite-function, flattened, Church-encoded, or
postulated substitute.

**The current number domain is integral JSON numbers.** `JsonNumber` stores an
arbitrary-precision `Int`, matching the landed numeric parser. JSON numbers
with a fractional part or exponent are outside this increment's accepted
domain and remain an explicit DS-9 residual rather than a silent widening or
narrowing of the round-trip claim.

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
`JsonString`, `JsonArray`, and `JsonObject`.

**Source map:**

| Reader task | Section |
|---|---|
| Inspect the carrier | [§2](#2-definition) |
| Understand the number domain | [§5](#5-design-notes) |
| Locate later codec work | [§3](#3-using-it), [§4](#4-laws--proofs) |

**Derivation path from built-ins.** `Json` is an ordinary strictly positive
inductive assembled from the built-in `Bool`, `Int`, `String`, `List`, and
`Pair` types. Its recursive occurrences follow `List`'s checked positive
parameter and the transparent non-dependent `Pair`/Sigma structure.

**`trusted_base()` delta: zero.** This file declares no primitive, opaque
constant, postulate, or `Axiom`; the carrier is checked by ordinary inductive
admission.

**Validation evidence.** `ken check` elaborates the tangled declaration. The
DS-9 AC-1 acceptance test additionally resolves the family and every
constructor as real registered kernel globals.
