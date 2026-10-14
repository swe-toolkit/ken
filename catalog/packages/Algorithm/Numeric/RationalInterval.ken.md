# Exact rational intervals

Closed intervals with exact non-negative rational endpoints provide total
membership, endpoint arithmetic, and checked enclosure for two member values.

## Contents

1. [Motivation](#1-motivation)
2. [Definition](#2-definition)
3. [Using it](#3-using-it)
4. [Laws & proofs](#4-laws--proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust & derivation](#7-trust--derivation)

## 1. Motivation

Rational intervals bound uncertain non-negative values without approximating
them as floating-point numbers. Each endpoint retains its rational spelling;
membership compares values through rational order. The carrier has no proof
field imposing a sorted pair, so an inverted interval represents an empty set
of members while remaining a valid input to total endpoint arithmetic.

## 2. Definition

`Interval` holds two rational values and exports `MkInterval` so clients can
construct boxes directly. The six public laws lead the checked source; the
operations they describe follow the proofs. Enclosure uses both membership
premises, one for each operand, and the two checked rational monotonicity laws.

```ken
import Core.Classes.LawfulClasses (IsTrue, bool_and)

import Data.Numeric.Rational
  (Rational,
    MkRational,
    rational_add,
    rational_mul,
    rational_leq,
    rational_equiv,
    rational_add_mono,
    rational_mul_mono)

pub data Interval : Type where {
  MkInterval : (lo : Rational) → (hi : Rational) → Interval
}

export MkInterval

pub theorem interval_add_encloses
      (i : Interval) (j : Interval) (x : Rational) (y : Rational)
    : IsTrue (interval_contains i x)
      → IsTrue (interval_contains j y)
      → IsTrue (interval_contains (interval_add i j) (rational_add x y)) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          λhx.
            λhy.
              (proof intro for bool_and)
                (rational_leq (rational_add a c) (rational_add x y))
                (rational_leq (rational_add x y) (rational_add b d))
                (rational_add_mono
                  a
                  x
                  c
                  y
                  ((proof left for bool_and) (rational_leq a x) (rational_leq x b) hx)
                  ((proof left for bool_and) (rational_leq c y) (rational_leq y d) hy))
                (rational_add_mono
                  x
                  b
                  y
                  d
                  ((proof right for bool_and) (rational_leq a x) (rational_leq x b) hx)
                  ((proof right for bool_and) (rational_leq c y) (rational_leq y d) hy))
      }
  }

pub theorem interval_mul_encloses
      (i : Interval) (j : Interval) (x : Rational) (y : Rational)
    : IsTrue (interval_contains i x)
      → IsTrue (interval_contains j y)
      → IsTrue (interval_contains (interval_mul i j) (rational_mul x y)) =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦
          λhx.
            λhy.
              (proof intro for bool_and)
                (rational_leq (rational_mul a c) (rational_mul x y))
                (rational_leq (rational_mul x y) (rational_mul b d))
                (rational_mul_mono
                  a
                  x
                  c
                  y
                  ((proof left for bool_and) (rational_leq a x) (rational_leq x b) hx)
                  ((proof left for bool_and) (rational_leq c y) (rational_leq y d) hy))
                (rational_mul_mono
                  x
                  b
                  y
                  d
                  ((proof right for bool_and) (rational_leq a x) (rational_leq x b) hx)
                  ((proof right for bool_and) (rational_leq c y) (rational_leq y d) hy))
      }
  }

pub theorem interval_point_contains
      (x : Rational)
    : IsTrue (interval_contains (interval_point x) x) =
  (proof intro for bool_and)
    (rational_leq x x)
    (rational_leq x x)
    ((proof refl for rational_leq) x)
    ((proof refl for rational_leq) x)

pub theorem interval_point_sound
      (x : Rational) (y : Rational)
    : IsTrue (interval_contains (interval_point x) y) → rational_equiv x y =
  λh.
    (proof antisym for rational_leq)
      x
      y
      ((proof left for bool_and) (rational_leq x y) (rational_leq y x) h)
      ((proof right for bool_and) (rational_leq x y) (rational_leq y x) h)

pub theorem interval_add_point
      (x : Rational) (y : Rational)
    : Equal Interval
        (interval_add (interval_point x) (interval_point y))
        (interval_point (rational_add x y)) =
  Refl

pub theorem interval_mul_point
      (x : Rational) (y : Rational)
    : Equal Interval
        (interval_mul (interval_point x) (interval_point y))
        (interval_point (rational_mul x y)) =
  Refl

pub fn interval_contains (i : Interval) (x : Rational) : Bool =
  match i {
    MkInterval lo hi ↦ bool_and (rational_leq lo x) (rational_leq x hi)
  }

pub fn interval_add (i : Interval) (j : Interval) : Interval =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦ MkInterval (rational_add a c) (rational_add b d)
      }
  }

pub fn interval_mul (i : Interval) (j : Interval) : Interval =
  match i {
    MkInterval a b ↦
      match j {
        MkInterval c d ↦ MkInterval (rational_mul a c) (rational_mul b d)
      }
  }

pub fn interval_point (x : Rational) : Interval = MkInterval x x
```

## 3. Using it

Membership includes each endpoint of a nonempty closed interval. For a point
interval, the same rational value supplies both bounds:

```ken example
const interval_unit : Rational = MkRational (Suc Zero) Zero

const interval_at_unit : Interval = MkInterval interval_unit interval_unit

theorem interval_unit_member
    : Equal Bool (interval_contains interval_at_unit interval_unit) True =
  Proved
```

`interval_add` combines lower with lower and upper with upper;
`interval_mul` does the same for multiplication. The latter formula uses
non-negative rational values and does not apply unchanged to signed values.
An inverted input still has a well-defined arithmetic result: it need not
produce another empty interval because the enclosure laws require members
of both inputs.

## 4. Laws & proofs

The addition and multiplication enclosure proofs split both stored endpoints,
then project the lower and upper comparisons from each membership premise.
The matching monotonicity theorem lifts the two lower comparisons to the
result's lower endpoint and the two upper comparisons to its upper endpoint.
The Boolean conjunction introduction proof assembles result membership.

A point interval contains its defining value by rational order reflexivity.
Conversely, membership in a point interval forces order in both directions,
so rational antisymmetry proves value equivalence. Endpoint-wise arithmetic
of two point intervals constructs exactly the same interval as the point at
the arithmetic result; those final equations use structural `Equal Interval`.

## 5. Design notes

Bounds use `rational_leq`, not structural equality of `Rational`: different
constructor spellings can denote the same exact value. By contrast,
`Equal Interval` concerns stored endpoint spellings, so equal membership sets
need not imply structural equality. The result of arithmetic on an empty
interval is defined by its endpoint formula; no result-emptiness law is
claimed. Subtraction, signed interval arithmetic, division near zero,
floating rounding, width, hull and inclusion isotonicity are outside this
non-negative exact interval API.

## 6. References

- **Wikipedia** —
  [Interval arithmetic](https://en.wikipedia.org/wiki/Interval_arithmetic)
  — orientation on interval enclosures; this exact non-negative formulation
  deliberately has a smaller operation set.
- **Ken specification** —
  [Exact numerical intervals](../../../../spec/50-stdlib/68-numerical.md)
  — normative contract for inclusive membership, endpoint arithmetic and laws.
- **Rational provider** — [Exact rational values](../../Data/Numeric/Rational.ken.md)
  — the value order and monotonicity proofs reused here.

## 7. Trust & derivation

1. **Public API.** `Interval`, `MkInterval`, `interval_contains`,
   `interval_add`, `interval_mul`, `interval_point`, and the six checked
   interval laws are the package's public contract.
2. **Source map.** [Definition](#2-definition) has every exported law and
   operation, [Using it](#3-using-it) checks construction and membership,
   and [Laws & proofs](#4-laws--proofs) explains the proof families.
3. **Derivation.** Carrier endpoints and exact arithmetic come from the
   rational package. Enclosure uses its public checked two-input addition
   and multiplication monotonicity; point soundness uses rational order
   reflexivity and antisymmetry.
4. **Trusted-base delta.** Zero: the six laws have checked proof bodies,
   and the package declares no `Axiom` or new primitive. Imported rational,
   natural-number and logical providers retain their inherited trust.
5. **Validation.** The
   [seed](../../../../conformance/stdlib/numerical/seed-rational-interval.md)
   supplies separately constructed positive values and false controls;
   those finite cases do not replace the generic checked proof bodies.
