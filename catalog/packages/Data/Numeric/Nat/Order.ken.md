# `Ord Nat` — the reader-facing order facade and `Nat` operations

`Nat` is inductive and kernel-proved, so its total order is a real,
zero-`Axiom` dictionary. The canonical relation, laws, and dictionary live with
the `Ord` class in `Core.Classes.LawfulClasses`. This entry re-exports that
surface under the natural-number path and keeps the small `Nat` operations
`min`, `max`, `sub`, and `compare` beside their readers.

## Contents

1. [Motivation](#1-motivation)
2. [Canonical order facade](#2-canonical-order-facade)
3. [Using it](#3-using-it)
4. [Laws & proofs](#4-laws--proofs)
5. [Design notes](#5-design-notes)
6. [References](#6-references)
7. [Trust & derivation](#7-trust--derivation)

## 1. Motivation

Natural numbers have a structural total order: it is reflexive,
antisymmetric, transitive, and total. The class-owning package defines that
single canonical dictionary for the compiler-floor `Nat` identity. This entry
re-exports the class surface without redeclaration, so generic code sees the
same dictionary through either package path. It also provides the everyday
operations `min`, `max`, `sub`, and `compare`.

## 2. Canonical order facade

`Core.Classes.LawfulClasses` is the defined-at home of the canonical `Ord Nat`
component. A facade import binds the names for this package's own definitions;
the matching export republishes the same provider identities to consumers. A
re-export never mints an `Order.leq_nat` alias, and instance carry keeps the
single class-owned dictionary available through this path.

```ken
import Core.Classes.LawfulClasses (Ord, IsTrue, bool_or, leq_nat)

export Core.Classes.LawfulClasses (Ord, IsTrue, bool_or, leq_nat)
```

`min` and `max` follow `leq_nat`'s recursion directly. `sub` is saturating
natural-number subtraction, and `compare` returns the three-way result
`OrdResult`:

```ken
data OrdResult = Lt | Eq | Gt

fn min (m : Nat) (n : Nat) : Nat =
  match m {
    Zero ↦ Zero;
    Suc m2 ↦
      match n {
        Zero ↦ Zero;
        Suc n2 ↦ Suc (min m2 n2)
      }
  }

fn max (m : Nat) (n : Nat) : Nat =
  match m {
    Zero ↦ n;
    Suc m2 ↦
      match n {
        Zero ↦ m;
        Suc n2 ↦ Suc (max m2 n2)
      }
  }

fn sub (a : Nat) (b : Nat) : Nat =
  match b {
    Zero ↦ a;
    Suc n ↦
      match a {
        Zero ↦ Zero;
        Suc m ↦ sub m n
      }
  }

fn compare (a : Nat) (b : Nat) : OrdResult =
  match leq_nat a b {
    True ↦
      match leq_nat b a {
        True ↦ Eq;
        False ↦ Lt
      };
    False ↦ Gt
  }
```

## 3. Using it

```ken example
proof two_leq_three for leq_nat : IsTrue (leq_nat (Suc (Suc Zero)) (Suc (Suc (Suc Zero)))) =
  Proved

const min_of_two_and_three : Nat = min (Suc (Suc Zero)) (Suc (Suc (Suc Zero)))

const max_of_two_and_three : Nat = max (Suc (Suc Zero)) (Suc (Suc (Suc Zero)))

const compare_two_three : OrdResult = compare (Suc (Suc Zero)) (Suc (Suc (Suc Zero)))

const compare_three_three : OrdResult = compare (Suc (Suc (Suc Zero))) (Suc (Suc (Suc Zero)))
```

A consumer asks for `Ord Nat` through an ordinary constraint. Resolution uses
the single dictionary carried by the facade; no generated private global or
Order-local declaration is part of the interface:

```ken example
fn carried_nat_leq (x : Nat) (y : Nat) : Bool where Ord Nat = d.leq x y

const ord_nat_leq : Bool = carried_nat_leq (Suc Zero) (Suc (Suc Zero))
```

## 4. Laws & proofs

`min`/`max`/`sub` earn their place with the computation facts a caller
relies on:

```ken example
proof zero_left for min (n : Nat) : Equal Nat (min Zero n) Zero = Proved

proof zero_left for max (n : Nat) : Equal Nat (max Zero n) n = Refl

proof zero_right for sub (a : Nat) : Equal Nat (sub a Zero) a = Refl
```

`min::zero_left` closes with `Proved`: `min Zero n` reduces to the literal
`Zero`
regardless of `n` (both sides collapse to the same nullary constructor,
`§1` of `catalog/guide/proof-techniques.ken.md`). `max::zero_left` and
`sub::zero_right` close with `Refl`: `max Zero n`'s recursive definition and
`sub`'s own `b = Zero` branch make `n`/`a` (an abstract, stuck variable)
appear literally unchanged on the reduced side without any further
constructor-level reduction — the goal stays `Eq`-shaped, not collapsed to
`Top`.

The companion fact `sub n n = Zero` (self-subtraction) is also true, but —
unlike `sub::zero_right` — needs induction on `n` (`sub`'s own structural
recursion doesn't reduce for an ABSTRACT `n` matched against itself), so
`Refl` alone cannot close it; this entry deliberately doesn't prove that
separately-inductive law, to keep scope small, and names the gap here
rather than carrying an unproved claim:

```ken reject
proof self_is_zero_wrong for sub (n : Nat) : Equal Nat (sub n n) Zero = Refl
```

## 5. Design notes

**The facade preserves one identity.** The canonical relation, attached proofs,
Boolean bridge, and dictionary are declared with the `Ord` class. This package
imports the relation for its own `compare` implementation and re-exports the
same identities for readers. A facade export changes reachability, not
provenance, so it cannot create a second comparator or dictionary.

**Local operations keep computation direct.** `min`, `max`, `sub`, and
`compare` remain structural definitions in this package. Their computation
rules stay visible beside their laws, while `compare` consumes the imported
canonical `leq_nat`.

## 6. References

- **Wikipedia** — [Total order](https://en.wikipedia.org/wiki/Total_order)
  — general orientation on the reflexive/antisymmetric/transitive/total
  axioms this entry's `Ord Nat` instantiates.
- **Lean 4 core** — `Nat.le` and its `LinearOrder`/`Nat`-specific decidable
  order instances (`Init/Data/Nat/Basic.lean`, part of the Lean 4
  repository, Apache-2.0) — <https://github.com/leanprover/lean4> —
  consulted for the general shape of a structural `Nat` order (no source
  copied, `CLEAN-ROOM.md`).

## 7. Trust & derivation

1. **Public API.** This facade re-exports `Ord`, `IsTrue`, `bool_or`, and
   `leq_nat` with their provider identities. `OrdResult`, `min`, `max`, `sub`,
   and `compare` remain package-local until their dedicated visibility step.
2. **Source map.**

   | Task | Section |
   |---|---|
   | Find the canonical surface | [Canonical order facade](#2-canonical-order-facade) |
   | Use the carried dictionary | [Using it](#3-using-it) |
   | Check the local computation facts | [Laws & proofs](#4-laws--proofs) |
   | Understand facade identity | [Design notes](#5-design-notes) |

3. **Derivation path.** The imported `leq_nat` and carried `Ord Nat` dictionary
   come from `Core.Classes.LawfulClasses`. The facade republishes their existing
   identities. `min`, `max`, `sub`, and `compare` are ordinary recursive
   functions; `compare` uses the imported relation.
4. **`trusted_base()` delta.** **Zero.** Re-exporting a checked identity adds no
   declaration or trust. The local operations introduce no `Axiom`, primitive,
   or postulate.
5. **Proof families.** The provider owns the structural `Nat` order proofs. This
   package's checked laws cover the local arithmetic operations.
6. **Consumers.** Generic ordered algorithms can resolve `Ord Nat` through this
   facade; direct callers can use the local arithmetic and comparison operations
   once their visibility is intentionally widened.
7. **Validation evidence.** Deferred-boundary and compatibility-root identity
   controls check the carried dictionary, canonical relation and bridge
   identities, zero local registration, zero trust delta, examples, and
   rejection fences. These controls are not Strict evidence; Strict closure and
   the identity assertions rerun after `LANG-MOD-CANONICAL-PAIR-PACKAGE`.
