# Verified insertion sort

Insertion sort places each element into an already-sorted tail. The comparator
and its laws travel together as an explicit `Ord` dictionary, so both the
algorithm and its proofs use the same ordering evidence.

## Definition

`is_sorted` is the standard adjacent-pair predicate supplied by the prelude.
`Permutation` is represented extensionally: every query has the same count on
both sides, where equality is induced by the lawful total order.

```ken
import Core.Classes.LawfulClasses (ord_leq_at)

import Data.Collections.Derived (count, eq_from_ord)

fn permutation (a : Type) (d : Ord a) (xs : List a) (ys : List a) : Prop =
  (q : a)
    → Equal Nat
    (count a (eq_from_ord a (ord_leq_at a d)) q xs)
    (count a (eq_from_ord a (ord_leq_at a d)) q ys)

fn insert (a : Type) (d : Ord a) (x : a) (xs : List a) : List a =
  match xs {
    Nil ↦ Cons a x (Nil a);
    Cons h t ↦
      match ord_leq_at a d x h {
        True ↦ Cons a x (Cons a h t);
        False ↦ Cons a h (insert a d x t)
      }
  }

fn sort (a : Type) (d : Ord a) (xs : List a) : List a =
  match xs {
    Nil ↦ Nil a;
    Cons h t ↦ insert a d h (sort a d t)
  }
```

## Sortedness

The proof separates the local head-order obligation from the recursively sorted
tail. In the recursive insertion branch, totality turns the failed comparison
`x <= h` into `h <= x`; transitivity is unnecessary because insertion never
moves an element past a head that was not already before it.

```ken
fn head_ordered (a : Type) (d : Ord a) (x : a) (xs : List a) : Prop =
  match xs {
    Nil ↦ Top;
    Cons h t ↦ Equal Bool (ord_leq_at a d x h) True
  }

theorem sorted_cons
      (a : Type) (d : Ord a) (x : a) (xs : List a)
    : is_sorted a (ord_leq_at a d) xs
      → head_ordered a d x xs
      → is_sorted a (ord_leq_at a d) (Cons a x xs) =
  match xs {
    Nil ↦ λsorted_xs. λhead_before. Proved;
    Cons h t ↦
      λsorted_xs.
        λhead_before.
          and_intro
            (Equal Bool (ord_leq_at a d x h) True)
            (is_sorted a (ord_leq_at a d) (Cons a h t))
            head_before
            sorted_xs
  }

theorem sorted_tail
      (a : Type) (d : Ord a) (x : a) (xs : List a)
    : is_sorted a (ord_leq_at a d) (Cons a x xs) → is_sorted a (ord_leq_at a d) xs =
  match xs {
    Nil ↦ λsorted_cons. Proved;
    Cons h t ↦
      λsorted_cons.
        and_snd
          (Equal Bool (ord_leq_at a d x h) True)
          (is_sorted a (ord_leq_at a d) (Cons a h t))
          sorted_cons
  }

theorem sorted_head
      (a : Type) (d : Ord a) (x : a) (xs : List a)
    : is_sorted a (ord_leq_at a d) (Cons a x xs) → head_ordered a d x xs =
  match xs {
    Nil ↦ λsorted_cons. Proved;
    Cons h t ↦
      λsorted_cons.
        and_fst
          (Equal Bool (ord_leq_at a d x h) True)
          (is_sorted a (ord_leq_at a d) (Cons a h t))
          sorted_cons
  }

theorem leq_right_of_left_false
      (a : Type)
      (d : Ord a)
      (x : a)
      (y : a)
      (left_false : Equal Bool (ord_leq_at a d x y) False)
    : Equal Bool (ord_leq_at a d y x) True =
  J (λleft _. Equal Bool (bool_or left (ord_leq_at a d y x)) True) (d.total x y) left_false

theorem head_ordered_after_insert
      (a : Type) (d : Ord a) (h : a) (x : a) (xs : List a)
    : Equal Bool (ord_leq_at a d h x) True
      → head_ordered a d h xs
      → head_ordered a d h (insert a d x xs) =
  match xs {
    Nil ↦ λh_before_x. λh_before_xs. h_before_x;
    Cons y ys ↦
      match ord_leq_at a d x y eqn : comparison {
        True ↦
          λh_before_x.
            λh_before_xs.
              J
                (λdecision _.
                  head_ordered
                    a
                    d
                    h
                    (match decision {
                      True ↦ Cons a x (Cons a y ys);
                      False ↦ Cons a y (insert a d x ys)
                    }))
                h_before_x
                (sym Bool (ord_leq_at a d x y) True comparison);
        False ↦
          λh_before_x.
            λh_before_xs.
              J
                (λdecision _.
                  head_ordered
                    a
                    d
                    h
                    (match decision {
                      True ↦ Cons a x (Cons a y ys);
                      False ↦ Cons a y (insert a d x ys)
                    }))
                h_before_xs
                (sym Bool (ord_leq_at a d x y) False comparison)
      }
  }

proof sorted for insert
      (a : Type) (d : Ord a) (x : a) (xs : List a)
    : is_sorted a (ord_leq_at a d) xs → is_sorted a (ord_leq_at a d) (insert a d x xs) =
  match xs {
    Nil ↦ λsorted_xs. Proved;
    Cons h t ↦
      match ord_leq_at a d x h eqn : comparison {
        True ↦
          λsorted_xs.
            J
              (λdecision _.
                is_sorted
                  a
                  (ord_leq_at a d)
                  (match decision {
                    True ↦ Cons a x (Cons a h t);
                    False ↦ Cons a h (insert a d x t)
                  }))
              (sorted_cons a d x (Cons a h t) sorted_xs comparison)
              (sym Bool (ord_leq_at a d x h) True comparison);
        False ↦
          λsorted_xs.
            let
              tail_is_sorted = sorted_tail a d h t sorted_xs;
              inserted_tail_is_sorted = insert::sorted a d x t tail_is_sorted;
              h_before_x = leq_right_of_left_false a d x h comparison;
              h_before_tail = sorted_head a d h t sorted_xs;
              h_before_inserted_tail =
                head_ordered_after_insert a d h x t h_before_x h_before_tail;
              branch_is_sorted =
                sorted_cons
                  a
                  d
                  h
                  (insert a d x t)
                  inserted_tail_is_sorted
                  h_before_inserted_tail
            in
              J
                (λdecision _.
                  is_sorted
                    a
                    (ord_leq_at a d)
                    (match decision {
                      True ↦ Cons a x (Cons a h t);
                      False ↦ Cons a h (insert a d x t)
                    }))
                branch_is_sorted
                (sym Bool (ord_leq_at a d x h) False comparison)
      }
  }

proof sorted for sort
      (a : Type) (d : Ord a) (xs : List a)
    : is_sorted a (ord_leq_at a d) (sort a d xs) =
  match xs {
    Nil ↦ Proved;
    Cons h t ↦ insert::sorted a d h (sort a d t) (sort::sorted a d t)
  }
```

## Permutation

Insertion preserves each query count. Sorting first applies the induction
hypothesis beneath the original head, then composes that equality with the
insertion lemma.

```ken
theorem count_cons_cong
      (a : Type)
      (d : Ord a)
      (q : a)
      (h : a)
      (xs : List a)
      (ys : List a)
      (counts_equal : Equal
        Nat
        (count a (eq_from_ord a (ord_leq_at a d)) q xs)
        (count a (eq_from_ord a (ord_leq_at a d)) q ys))
    : Equal Nat
        (count a (eq_from_ord a (ord_leq_at a d)) q (Cons a h xs))
        (count a (eq_from_ord a (ord_leq_at a d)) q (Cons a h ys)) =
  match eq_from_ord a (ord_leq_at a d) q h eqn : occurrence {
    True ↦
      J
        (λdecision _.
          Equal
            Nat
            (match decision {
              True ↦ Suc (count a (eq_from_ord a (ord_leq_at a d)) q xs);
              False ↦ count a (eq_from_ord a (ord_leq_at a d)) q xs
            })
            (match decision {
              True ↦ Suc (count a (eq_from_ord a (ord_leq_at a d)) q ys);
              False ↦ count a (eq_from_ord a (ord_leq_at a d)) q ys
            }))
        (cong
          Nat
          Nat
          (count a (eq_from_ord a (ord_leq_at a d)) q xs)
          (count a (eq_from_ord a (ord_leq_at a d)) q ys)
          Suc
          counts_equal)
        (sym Bool (eq_from_ord a (ord_leq_at a d) q h) True occurrence);
    False ↦
      J
        (λdecision _.
          Equal
            Nat
            (match decision {
              True ↦ Suc (count a (eq_from_ord a (ord_leq_at a d)) q xs);
              False ↦ count a (eq_from_ord a (ord_leq_at a d)) q xs
            })
            (match decision {
              True ↦ Suc (count a (eq_from_ord a (ord_leq_at a d)) q ys);
              False ↦ count a (eq_from_ord a (ord_leq_at a d)) q ys
            }))
        counts_equal
        (sym Bool (eq_from_ord a (ord_leq_at a d) q h) False occurrence)
  }

fn count_after_two (tail_count : Nat) (first_occurs : Bool) (second_occurs : Bool) : Nat =
  match first_occurs {
    True ↦
      Suc
        (match second_occurs {
          True ↦ Suc tail_count;
          False ↦ tail_count
        });
    False ↦
      match second_occurs {
        True ↦ Suc tail_count;
        False ↦ tail_count
      }
  }

theorem count_swap_decisions
      (tail_count : Nat) (x_occurs : Bool) (y_occurs : Bool)
    : Equal Nat
        (count_after_two tail_count x_occurs y_occurs)
        (count_after_two tail_count y_occurs x_occurs) =
  match x_occurs {
    True ↦
      match y_occurs {
        True ↦ Refl;
        False ↦ Refl
      };
    False ↦
      match y_occurs {
        True ↦ Refl;
        False ↦ Refl
      }
  }

theorem count_cons_swap
      (a : Type) (d : Ord a) (q : a) (x : a) (y : a) (xs : List a)
    : Equal Nat
        (count a (eq_from_ord a (ord_leq_at a d)) q (Cons a x (Cons a y xs)))
        (count a (eq_from_ord a (ord_leq_at a d)) q (Cons a y (Cons a x xs))) =
  count_swap_decisions
    (count a (eq_from_ord a (ord_leq_at a d)) q xs)
    (eq_from_ord a (ord_leq_at a d) q x)
    (eq_from_ord a (ord_leq_at a d) q y)

proof count for insert
      (a : Type) (d : Ord a) (x : a) (xs : List a) (q : a)
    : Equal Nat
        (count a (eq_from_ord a (ord_leq_at a d)) q (Cons a x xs))
        (count a (eq_from_ord a (ord_leq_at a d)) q (insert a d x xs)) =
  match xs {
    Nil ↦ Refl;
    Cons h t ↦
      match ord_leq_at a d x h eqn : comparison {
        True ↦
          J
            (λdecision _.
              Equal
                Nat
                (count a (eq_from_ord a (ord_leq_at a d)) q (Cons a x (Cons a h t)))
                (count
                  a
                  (eq_from_ord a (ord_leq_at a d))
                  q
                  (match decision {
                    True ↦ Cons a x (Cons a h t);
                    False ↦ Cons a h (insert a d x t)
                  })))
            Refl
            (sym Bool (ord_leq_at a d x h) True comparison);
        False ↦
          let
            swapped_count = count_cons_swap a d q x h t;
            recursive_count = insert::count a d x t q;
            inserted_count =
              count_cons_cong a d q h (Cons a x t) (insert a d x t) recursive_count;
            branch_count =
              trans
                Nat
                (count a (eq_from_ord a (ord_leq_at a d)) q (Cons a x (Cons a h t)))
                (count a (eq_from_ord a (ord_leq_at a d)) q (Cons a h (Cons a x t)))
                (count a (eq_from_ord a (ord_leq_at a d)) q (Cons a h (insert a d x t)))
                swapped_count
                inserted_count
          in
            J
              (λdecision _.
                Equal
                  Nat
                  (count a (eq_from_ord a (ord_leq_at a d)) q (Cons a x (Cons a h t)))
                  (count
                    a
                    (eq_from_ord a (ord_leq_at a d))
                    q
                    (match decision {
                      True ↦ Cons a x (Cons a h t);
                      False ↦ Cons a h (insert a d x t)
                    })))
              branch_count
              (sym Bool (ord_leq_at a d x h) False comparison)
      }
  }

proof permutation for insert
      (a : Type) (d : Ord a) (x : a) (xs : List a)
    : permutation a d (Cons a x xs) (insert a d x xs) =
  λq. insert::count a d x xs q

proof permutation for sort
      (a : Type) (d : Ord a) (xs : List a)
    : permutation a d xs (sort a d xs) =
  match xs {
    Nil ↦ λq. Proved;
    Cons h t ↦
      λq.
        let
          original_count = count a (eq_from_ord a (ord_leq_at a d)) q (Cons a h t);
          tail_sorted_count =
            count a (eq_from_ord a (ord_leq_at a d)) q (Cons a h (sort a d t));
          final_count = count a (eq_from_ord a (ord_leq_at a d)) q (insert a d h (sort a d t));
          tail_counts_equal =
            count_cons_cong a d q h t (sort a d t) (sort::permutation a d t q);
          insertion_counts_equal = insert::count a d h (sort a d t) q
        in
          trans
            Nat
            original_count
            tail_sorted_count
            final_count
            tail_counts_equal
            insertion_counts_equal
  }
```

## Trust and derivation

1. **Public API.** `insert`, `sort`, `permutation`, `insert::sorted`,
   `sort::sorted`, `insert::permutation`, and `sort::permutation`.
2. **Derivation path.** Both operations are structural recursion over `List`.
   Sortedness uses the prelude's pairwise `is_sorted` predicate plus the laws in
   `Ord`. Permutation is count equality and uses only equality congruence and
   transitivity.
3. **Proof families.** `insert::sorted` preserves the sorted invariant;
   `sort::sorted` assembles the induction. `insert::count` preserves one query
   count; `insert::permutation` quantifies it; `sort::permutation` composes the
   tail and insertion equalities.
4. **`trusted_base()` delta.** **Zero.** The entry contains no postulate,
   `Axiom`, foreign declaration, primitive, or unresolved proof hole.
5. **Validation.** Focused acceptance elaborates every checked fence, confirms
   the public proof globals, checks concrete Boolean sorting vectors, and
   compares `trusted_base()` before and after loading this entry.

## References

- Donald E. Knuth, *The Art of Computer Programming, Volume 3: Sorting and
  Searching* — the standard insertion-sort algorithm and its ordering
  invariant.
