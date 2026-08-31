# Decidable ordered search

Ordered search decides membership in a sorted list. Its result retains the
membership proof on success and a refutation on failure, while sortedness lets
the search stop as soon as the query lies strictly before the current head.

## Definition

`elem` is the Boolean observation of membership induced by a lawful total
order: two values are equal exactly when each is below the other. The checked
membership proposition is `Equal Bool (elem a d x xs) True`, so it fits the
standard `Dec` carrier without turning proof evidence into runtime data.

`sorted_for_search` is stronger than adjacent-pair sortedness. At every head it
carries the fact that the head is below every member of the tail, together with
the recursively sorted tail. This is the exact invariant used by the pruning
branch.

```ken
import Core.Classes.LawfulClasses (Ord, ord_leq_at)

fn elem_step (tail_member : Bool) (x_before_head : Bool) (head_before_x : Bool) : Bool =
  match x_before_head {
    True ↦
      match head_before_x {
        True ↦ True;
        False ↦ tail_member
      };
    False ↦ tail_member
  }

fn elem (a : Type) (d : Ord a) (x : a) (xs : List a) : Bool =
  match xs {
    Nil ↦ False;
    Cons head tail ↦ elem_step (elem a d x tail) (ord_leq_at a d x head) (ord_leq_at a d head x)
  }

fn sorted_for_search (a : Type) (d : Ord a) (xs : List a) : Prop =
  match xs {
    Nil ↦ Top;
    Cons head tail ↦
      And
        ((x : a) → Equal Bool (elem a d x tail) True → Equal Bool (ord_leq_at a d head x) True)
        (sorted_for_search a d tail)
  }
```

## Laws and proofs

The four `elem_step` lemmas are the decision bridge. They rewrite the two
ordering observations to the branch constructors and expose either the head
witness or the tail proposition. `search` then has only three semantic cases:
equal at the head, strictly before the head, or recurse into the tail.

In the strict-before case, any assumed tail membership is passed to the head's
sortedness evidence. That produces `head <= query`, contradicting the branch's
checked `head <= query = False` observation. Both negative paths therefore
construct real functions from membership evidence to `Empty`.

```ken
theorem search_sym (ty : Type) (x : ty) (y : ty) (p : Equal ty x y) : Equal ty y x =
  J (λy2 _. Equal ty y2 x) Refl p

theorem search_trans
      (ty : Type) (x : ty) (y : ty) (z : ty) (p : Equal ty x y) (q : Equal ty y z)
    : Equal ty x z =
  J (λz2 _. Equal ty x z2) p q

fn boolean_contradiction
      (value : Bool) (is_true : Equal Bool value True) (is_false : Equal Bool value False)
    : Empty =
  absurd (search_trans Bool True value False (search_sym Bool value True is_true) is_false)

theorem elem_step_both_true
      (tail_member : Bool)
      (x_before_head : Bool)
      (head_before_x : Bool)
      (x_before : Equal Bool x_before_head True)
      (head_before : Equal Bool head_before_x True)
    : Equal Bool (elem_step tail_member x_before_head head_before_x) True =
  J
    (λx_decision _. Equal Bool (elem_step tail_member x_decision head_before_x) True)
    (J
      (λhead_decision _. Equal Bool (elem_step tail_member True head_decision) True)
      Proved
      (search_sym Bool head_before_x True head_before))
    (search_sym Bool x_before_head True x_before)

theorem elem_step_from_tail_after_head
      (tail_member : Bool)
      (x_before_head : Bool)
      (head_before_x : Bool)
      (x_after : Equal Bool x_before_head False)
      (tail_present : Equal Bool tail_member True)
    : Equal Bool (elem_step tail_member x_before_head head_before_x) True =
  J
    (λx_decision _. Equal Bool (elem_step tail_member x_decision head_before_x) True)
    tail_present
    (search_sym Bool x_before_head False x_after)

theorem elem_step_to_tail_after_head
      (tail_member : Bool)
      (x_before_head : Bool)
      (head_before_x : Bool)
      (x_after : Equal Bool x_before_head False)
      (member : Equal Bool (elem_step tail_member x_before_head head_before_x) True)
    : Equal Bool tail_member True =
  J
    (λx_decision _.
      Equal Bool (elem_step tail_member x_decision head_before_x) True
      → Equal Bool tail_member True)
    (λmember_at_false. member_at_false)
    (search_sym Bool x_before_head False x_after)
    member

theorem elem_step_to_tail_before_head
      (tail_member : Bool)
      (x_before_head : Bool)
      (head_before_x : Bool)
      (x_before : Equal Bool x_before_head True)
      (head_after : Equal Bool head_before_x False)
      (member : Equal Bool (elem_step tail_member x_before_head head_before_x) True)
    : Equal Bool tail_member True =
  J
    (λx_decision _.
      Equal Bool (elem_step tail_member x_decision head_before_x) True
      → Equal Bool tail_member True)
    (J
      (λhead_decision _.
        Equal Bool (elem_step tail_member True head_decision) True
        → Equal Bool tail_member True)
      (λmember_at_false. member_at_false)
      (search_sym Bool head_before_x False head_after))
    (search_sym Bool x_before_head True x_before)
    member

fn search
      (a : Type) (d : Ord a) (x : a) (xs : List a)
    : sorted_for_search a d xs → Dec (Equal Bool (elem a d x xs) True) =
  match xs {
    Nil ↦ λsorted. No (Equal Bool (elem a d x (Nil a)) True) (λmember. absurd member);
    Cons head tail ↦
      λsorted.
        match ord_leq_at a d x head eqn : x_before_head {
          True ↦
            match ord_leq_at a d head x eqn : head_before_x {
              True ↦
                Yes
                  (Equal Bool (elem a d x (Cons a head tail)) True)
                  (elem_step_both_true
                    (elem a d x tail)
                    (ord_leq_at a d x head)
                    (ord_leq_at a d head x)
                    x_before_head
                    head_before_x);
              False ↦
                No
                  (Equal Bool (elem a d x (Cons a head tail)) True)
                  (λmember.
                    let
                      tail_member =
                        elem_step_to_tail_before_head
                          (elem a d x tail)
                          (ord_leq_at a d x head)
                          (ord_leq_at a d head x)
                          x_before_head
                          head_before_x
                          member;
                      head_before_member =
                        and_fst
                          ((x2 : a)
                            → Equal
                            Bool
                            (elem a d x2 tail)
                            True
                            → Equal
                            Bool
                            (ord_leq_at a d head x2)
                            True)
                          (sorted_for_search a d tail)
                          sorted
                          x
                          tail_member
                    in
                      boolean_contradiction
                        (ord_leq_at a d head x)
                        head_before_member
                        head_before_x)
            };
          False ↦
            match search a d x tail
              (and_snd
                ((x2 : a)
                  → Equal
                  Bool
                  (elem a d x2 tail)
                  True
                  → Equal
                  Bool
                  (ord_leq_at a d head x2)
                  True)
                (sorted_for_search a d tail)
                sorted) {
              Yes tail_member ↦
                Yes
                  (Equal Bool (elem a d x (Cons a head tail)) True)
                  (elem_step_from_tail_after_head
                    (elem a d x tail)
                    (ord_leq_at a d x head)
                    (ord_leq_at a d head x)
                    x_before_head
                    tail_member);
              No refute_tail ↦
                No
                  (Equal Bool (elem a d x (Cons a head tail)) True)
                  (λmember.
                    refute_tail
                      (elem_step_to_tail_after_head
                        (elem a d x tail)
                        (ord_leq_at a d x head)
                        (ord_leq_at a d head x)
                        x_before_head
                        member))
            }
        }
  }
```

## Trust and derivation

`elem` and `search` recurse structurally over `List`. Membership evidence is the
checked proposition that the transparent membership observation reduces to
`True`; the `Yes` constructor carries that proof, while `No` carries its
refutation into `Empty`. The ordering dictionary supplies only its checked
`refl`, `antisym`, and comparison fields. The package introduces no axiom,
postulate, primitive, foreign declaration, or unresolved hole, so its
`trusted_base()` delta is zero.

## References

- Donald E. Knuth, *The Art of Computer Programming, Volume 3: Sorting and
  Searching* — ordered sequential search and its early-termination invariant.
