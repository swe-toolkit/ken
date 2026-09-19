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

pub fn min (m : Nat) (n : Nat) : Nat =
  match m {
    Zero ↦ Zero;
    Suc m2 ↦
      match n {
        Zero ↦ Zero;
        Suc n2 ↦ Suc (min m2 n2)
      }
  }

pub proof zero_left for min (n : Nat) : Equal Nat (min Zero n) Zero = Proved

pub proof leq_left for min (m : Nat) (n : Nat) : IsTrue (leq_nat (min m n) m) =
  match m {
    Zero ↦ Proved;
    Suc m2 ↦
      match n {
        Zero ↦ Proved;
        Suc n2 ↦ proof leq_left for min m2 n2
      }
  }

pub proof leq_right for min (m : Nat) (n : Nat) : IsTrue (leq_nat (min m n) n) =
  match m {
    Zero ↦ Proved;
    Suc m2 ↦
      match n {
        Zero ↦ Proved;
        Suc n2 ↦ proof leq_right for min m2 n2
      }
  }

pub fn max (m : Nat) (n : Nat) : Nat =
  match m {
    Zero ↦ n;
    Suc m2 ↦
      match n {
        Zero ↦ m;
        Suc n2 ↦ Suc (max m2 n2)
      }
  }

pub proof zero_left for max (n : Nat) : Equal Nat (max Zero n) n = Refl

pub proof left_leq for max (m : Nat) (n : Nat) : IsTrue (leq_nat m (max m n)) =
  match m {
    Zero ↦ Proved;
    Suc m2 ↦
      match n {
        Zero ↦ proof refl for leq_nat (Suc m2);
        Suc n2 ↦ proof left_leq for max m2 n2
      }
  }

pub proof right_leq for max (m : Nat) (n : Nat) : IsTrue (leq_nat n (max m n)) =
  match m {
    Zero ↦ proof refl for leq_nat n;
    Suc m2 ↦
      match n {
        Zero ↦ Proved;
        Suc n2 ↦ proof right_leq for max m2 n2
      }
  }

pub fn sub (a : Nat) (b : Nat) : Nat =
  match b {
    Zero ↦ a;
    Suc n ↦
      match a {
        Zero ↦ Zero;
        Suc m ↦ sub m n
      }
  }

pub proof zero_right for sub (a : Nat) : Equal Nat (sub a Zero) a = Refl

pub proof self_is_zero for sub (n : Nat) : Equal Nat (sub n n) Zero =
  match n {
    Zero ↦ Proved;
    Suc n2 ↦ proof self_is_zero for sub n2
  }

pub proof zero_left for sub (b : Nat) : Equal Nat (sub Zero b) Zero =
  match b {
    Zero ↦ Proved;
    Suc b2 ↦ Proved
  }

pub proof saturates for sub
      (a : Nat)
    : (b : Nat) → IsTrue (leq_nat a b) → Equal Nat (sub a b) Zero =
  match a {
    Zero ↦
      λb.
        match b {
          Zero ↦ λh. Proved;
          Suc b2 ↦ λh. Proved
        };
    Suc a2 ↦
      λb.
        match b {
          Zero ↦ λh. absurd h;
          Suc b2 ↦ λh. proof saturates for sub a2 b2 h
        }
  }

pub proof suc_decreases for sub
      (a : Nat)
    : (b : Nat)
      → IsTrue (leq_nat (Suc b) a)
      → IsTrue (leq_nat (Suc (sub a (Suc b))) (sub a b)) =
  match a {
    Zero ↦ λb. λh. absurd h;
    Suc a2 ↦
      λb.
        match b {
          Zero ↦ λh. proof refl for leq_nat a2;
          Suc b2 ↦ λh. proof suc_decreases for sub a2 b2 h
        }
  }

pub fn compare (a : Nat) (b : Nat) : OrdResult =
  match leq_nat a b {
    True ↦
      match leq_nat b a {
        True ↦ Eq;
        False ↦ Lt
      };
    False ↦ Gt
  }

pub proof lt_implies_leq for compare
      (a : Nat)
    : (b : Nat) → Equal OrdResult (compare a b) Lt → IsTrue (leq_nat a b) =
  match a {
    Zero ↦ λb. λh. Proved;
    Suc a2 ↦
      λb.
        match b {
          Zero ↦ λh. absurd h;
          Suc b2 ↦ λh. (proof lt_implies_leq for compare) a2 b2 h
        }
  }

pub proof eq_implies_equal for compare
      (a : Nat)
    : (b : Nat) → Equal OrdResult (compare a b) Eq → Equal Nat a b =
  match a {
    Zero ↦
      λb.
        match b {
          Zero ↦ λh. (proof antisym for leq_nat) Zero Zero Proved Proved;
          Suc b2 ↦ λh. absurd h
        };
    Suc a2 ↦
      λb.
        match b {
          Zero ↦ λh. absurd h;
          Suc b2 ↦
            λh.
              let
                equal_tail = (proof eq_implies_equal for compare) a2 b2 h;
                tail_leq_forward : IsTrue (leq_nat a2 b2) =
                  J (λb' _. IsTrue (leq_nat a2 b')) ((proof refl for leq_nat) a2) equal_tail;
                tail_leq_reverse : IsTrue (leq_nat b2 a2) =
                  J (λb' _. IsTrue (leq_nat b' a2)) ((proof refl for leq_nat) a2) equal_tail
              in
                (proof antisym for leq_nat) (Suc a2) (Suc b2) tail_leq_forward tail_leq_reverse
        }
  }

pub proof gt_implies_reverse_leq for compare
      (a : Nat)
    : (b : Nat) → Equal OrdResult (compare a b) Gt → IsTrue (leq_nat b a) =
  match a {
    Zero ↦
      λb.
        match b {
          Zero ↦ λh. Proved;
          Suc b2 ↦ λh. absurd h
        };
    Suc a2 ↦
      λb.
        match b {
          Zero ↦ λh. Proved;
          Suc b2 ↦ λh. (proof gt_implies_reverse_leq for compare) a2 b2 h
        }
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

`min`/`max`/`sub` earn their place with three exported computation facts:
`min::zero_left`, `max::zero_left`, and `sub::zero_right`.

`min::zero_left` closes with `Proved`: `min Zero n` reduces to the literal
`Zero`
regardless of `n` (both sides collapse to the same nullary constructor,
`§1` of `catalog/guide/proof-techniques.ken.md`). `max::zero_left` and
`sub::zero_right` close with `Refl`: `max Zero n`'s recursive definition and
`sub`'s own `b = Zero` branch make `n`/`a` (an abstract, stuck variable)
appear literally unchanged on the reduced side without any further
constructor-level reduction — the goal stays `Eq`-shaped, not collapsed to
`Top`.

The four further subtraction laws use structural induction. `self_is_zero`
and `zero_left` close the two zero endpoints. `saturates` proves that
`sub a b` is `Zero` whenever `a ≤ b`, while `suc_decreases` proves
`Suc (sub a (Suc b)) ≤ sub a b` whenever `Suc b ≤ a`.

Both conditional laws carry their Boolean hypotheses as `IsTrue` propositions,
matching this package's order examples and letting downstream consumers pass
canonical `leq_nat` evidence without restating the underlying Boolean equation.
The four `min`/`max` bounds use the same form. The three `compare` agreements
return the forward order for `Lt`, propositional equality for `Eq`, and the
reverse order for `Gt`. The equality proof recurses through the two `Nat`
arguments, transports reflexivity along the tail equality to recover both
order directions, and applies `leq_nat::antisym` at each step.

`self_is_zero` needs its induction: `sub`'s structural recursion does not reduce
for an abstract `n` matched against itself, so `Refl` alone cannot close the
goal. That is asserted here, not merely asserted about:

```ken reject
-- Fails: `Refl` cannot prove `sub n n = Zero` for abstract `n`, because
-- `sub` does not reduce until induction exposes `n`.
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
   `leq_nat` with their provider identities. It exports its defined-at `min`,
   `max`, `sub`, and `compare` operations; their three zero computation facts;
   the four `min`/`max` bounds; the three `compare` agreements; and the four
   inductive `sub` proofs. `OrdResult` remains package-local.
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
   package's checked laws use structural recursion for the `min`/`max` bounds
   and subtraction theory, and case analysis on the canonical relation for the
   `compare` agreements. The equality arm closes through `leq_nat::antisym`.
6. **Consumers.** Generic ordered algorithms can resolve `Ord Nat` through this
   facade; direct callers can selectively import the local arithmetic and
   comparison operations.
7. **Validation evidence.** Deferred-boundary and compatibility-root identity
   controls check the carried dictionary, canonical relation and bridge
   identities, zero local registration, zero trust delta, examples, and
   rejection fences. These controls are not Strict evidence; Strict closure and
   the identity assertions rerun after `LANG-MOD-CANONICAL-PAIR-PACKAGE`.
