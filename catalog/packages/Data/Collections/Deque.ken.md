# Persistent two-list deque

A deque stores its front in order and its back in reverse order. This makes
insertion at either end constant-time while preserving one front-to-back list
as the public observation.

## Definition

`MkDeque front back` represents `front ++ reverse back`. Popping uses the list
already facing the requested end when possible; when that list is empty, it
reverses the other list once and continues from there.

```ken
import Data.Collections.Derived (list_append, reverse)

data Deque a = MkDeque (List a) (List a)

const empty (a : Type) : Deque a = MkDeque a (Nil a) (Nil a)

fn pushFront (a : Type) (x : a) (q : Deque a) : Deque a =
  match q {
    MkDeque front back ↦ MkDeque a (Cons a x front) back
  }

fn pushBack (a : Type) (x : a) (q : Deque a) : Deque a =
  match q {
    MkDeque front back ↦ MkDeque a front (Cons a x back)
  }

fn toList (a : Type) (q : Deque a) : List a =
  match q {
    MkDeque front back ↦ list_append a front (reverse a back)
  }

fn popFront (a : Type) (q : Deque a) : Option (Pair a (Deque a)) =
  match q {
    MkDeque front back ↦
      match front {
        Nil ↦
          match reverse a back {
            Nil ↦ None (Pair a (Deque a));
            Cons x rest ↦
              Some (Pair a (Deque a)) (mk_pair a (Deque a) x (MkDeque a rest (Nil a)))
          };
        Cons x rest ↦ Some (Pair a (Deque a)) (mk_pair a (Deque a) x (MkDeque a rest back))
      }
  }

fn popBack (a : Type) (q : Deque a) : Option (Pair a (Deque a)) =
  match q {
    MkDeque front back ↦
      match back {
        Nil ↦
          match reverse a front {
            Nil ↦ None (Pair a (Deque a));
            Cons x rest ↦
              Some (Pair a (Deque a)) (mk_pair a (Deque a) x (MkDeque a (Nil a) rest))
          };
        Cons x rest ↦ Some (Pair a (Deque a)) (mk_pair a (Deque a) x (MkDeque a front rest))
      }
  }
```

## Laws and proofs

The abstraction function is a homomorphism for insertion at both ends. The
back law uses the one associativity direction needed to expose the appended
singleton. `PopPreserves` carries the residual deque together with the checked
fact that its abstract sequence is unchanged.

```ken
theorem deque_cong
      (a : Type) (b : Type) (x : a) (y : a) (f : a → b) (p : Equal a x y)
    : Equal b (f x) (f y) =
  J (λy2 _. Equal b (f x) (f y2)) Refl p

theorem deque_append_snoc_assoc
      (a : Type) (front : List a) (tail : List a) (x : a)
    : Equal
        (List a)
        (list_append a front (list_append a tail (Cons a x (Nil a))))
        (list_append a (list_append a front tail) (Cons a x (Nil a))) =
  match front {
    Nil ↦ Refl;
    Cons h rest ↦
      deque_cong
        (List a)
        (List a)
        (list_append a rest (list_append a tail (Cons a x (Nil a))))
        (list_append a (list_append a rest tail) (Cons a x (Nil a)))
        (Cons a h)
        (deque_append_snoc_assoc a rest tail x)
  }

theorem toList_pushFront
      (a : Type) (x : a) (q : Deque a)
    : Equal (List a) (toList a (pushFront a x q)) (Cons a x (toList a q)) =
  match q {
    MkDeque front back ↦
      deque_cong
        (List a)
        (List a)
        (list_append a front (reverse a back))
        (list_append a front (reverse a back))
        (Cons a x)
        Refl
  }

theorem toList_pushBack
      (a : Type) (x : a) (q : Deque a)
    : Equal
        (List a)
        (toList a (pushBack a x q))
        (list_append a (toList a q) (Cons a x (Nil a))) =
  match q {
    MkDeque front back ↦ deque_append_snoc_assoc a front (reverse a back) x
  }

data PopPreserves (a : Type) (x : a) (q : Deque a) : Option (Pair a (Deque a)) → Type where {
  MkPopPreserves :
    (q2 : Deque a)
    → Equal (List a) (toList a q2) (toList a q)
    → PopPreserves a x q (Some (Pair a (Deque a)) (mk_pair a (Deque a) x q2))
}

fn popFront_pushFront
      (a : Type) (x : a) (q : Deque a)
    : PopPreserves a x q (popFront a (pushFront a x q)) =
  match q {
    MkDeque front back ↦ MkPopPreserves a x (MkDeque a front back) (MkDeque a front back) Refl
  }

fn popBack_pushBack
      (a : Type) (x : a) (q : Deque a)
    : PopPreserves a x q (popBack a (pushBack a x q)) =
  match q {
    MkDeque front back ↦ MkPopPreserves a x (MkDeque a front back) (MkDeque a front back) Refl
  }
```

## Trust and derivation

`Deque` is an ordinary strictly positive inductive. Its operations reuse the
transparent `list_append` and `reverse` definitions from
`Data.Collections.Derived`; every law is a checked proof term. Relative to that
provider closure, the package adds no axiom, postulate, primitive, foreign
declaration, unresolved hole, or consumer-local `trusted_base()` entry. Loading
the full closure inherits the provider's existing audited trust footprint
without widening or duplicating it.

## References

- Chris Okasaki, *Purely Functional Data Structures* — the standard two-list
  persistent queue/deque representation and amortized-reversal technique.
