# Verified insertion sort

Insertion sort places each element into an already-sorted tail. The comparator
and its laws travel together as an explicit `Ord` dictionary. The operations
and proofs below specialize the generic insertion-sort development in
`Data.Collections.Derived` to that dictionary's ordering.

## Definition

The public surface keeps the `Ord` interface while using the same checked
comparator-indexed operations and proofs for sorting and permutation.

```ken
import Core.Classes.LawfulClasses (Ord, ord_leq_at)

import Data.Collections.Derived (eq_from_ord)

import Data.Collections.Derived as DC

fn sort (a : Type) (d : Ord a) (xs : List a) : List a = DC.sort a (ord_leq_at a d) xs

proof sorted for sort
      (a : Type) (d : Ord a) (xs : List a)
    : is_sorted a (ord_leq_at a d) (sort a d xs) =
  DC.sort::sorted a (ord_leq_at a d) d.total xs

proof permutation for sort
      (a : Type) (d : Ord a) (xs : List a)
    : permutation a d xs (sort a d xs) =
  DC.sort::perm a (ord_leq_at a d) xs (eq_from_ord a (ord_leq_at a d))

fn permutation (a : Type) (d : Ord a) (xs : List a) (ys : List a) : Prop =
  DC.Perm a (eq_from_ord a (ord_leq_at a d)) xs ys

fn insert (a : Type) (d : Ord a) (x : a) (xs : List a) : List a =
  DC.insert a (ord_leq_at a d) x xs

proof sorted for insert
      (a : Type) (d : Ord a) (x : a) (xs : List a)
    : is_sorted a (ord_leq_at a d) xs → is_sorted a (ord_leq_at a d) (insert a d x xs) =
  DC.insert::sorted a (ord_leq_at a d) d.total x xs

proof permutation for insert
      (a : Type) (d : Ord a) (x : a) (xs : List a)
    : permutation a d (Cons a x xs) (insert a d x xs) =
  λq. DC.insert::count a (ord_leq_at a d) x xs (eq_from_ord a (ord_leq_at a d)) q
```

## Sortedness

The `Ord` dictionary supplies totality for every comparison made by the
generic insertion and sort proofs. Neither wrapper needs to reconstruct an
ordering argument: both pass `d.total` directly to the corresponding checked
proof in `Derived`.

## Permutation

The package represents permutation extensionally: every query has the same
count on both sides, using equality induced by the lawful total order. The
insertion proof instantiates the generic count-preservation law for each query;
the sort proof uses the generic permutation law with that same equality.

## Trust and derivation

1. **Public API.** `insert`, `sort`, `permutation`, `insert::sorted`,
   `sort::sorted`, `insert::permutation`, and `sort::permutation`.
2. **Derivation path.** The operations instantiate `Derived`'s structural
   insertion and sort at `ord_leq_at a d`. Their sortedness laws pass the
   dictionary's totality field to the generic proofs; their permutation laws
   use `Derived`'s comparator-independent count proofs at `eq_from_ord`.
3. **Proof families.** `insert::sorted` and `sort::sorted` preserve sortedness;
   `insert::permutation` abstracts count preservation over each query;
   `sort::permutation` composes sorting with the generic permutation proof.
4. **`trusted_base()` delta.** **Zero.** The entry contains no postulate,
   `Axiom`, foreign declaration, primitive, or unresolved proof hole.
5. **Validation.** Focused acceptance elaborates every checked fence, confirms
   the public proof globals, checks concrete Boolean sorting vectors, and
   compares `trusted_base()` before and after loading this entry.

## References

- Donald E. Knuth, *The Art of Computer Programming, Volume 3: Sorting and
  Searching* — the standard insertion-sort algorithm and its ordering
  invariant.
