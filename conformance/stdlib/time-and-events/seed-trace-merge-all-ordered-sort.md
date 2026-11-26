# Ordered batch merge and sorting the concatenation

Format: `../../README.md`.
Spec: `spec/50-stdlib/69-time-and-events.md` §23.

The first two cases apply the closed law at its public boundary. The third is
a premise counterexample: it compares concrete read-offs and does not apply the
law. Each case runs on an independent fresh copy of the accepted prefix below.
They distinguish sorting
the concatenation from preserving plain concatenation; they do not establish
the generic theorem. Equal-time tie behavior remains the concern of §22 and
its package test.

## Shared accepted prefix

Use this prefix for each case independently. The batches are individually
ordered, but their concatenation is not: time 2 follows time 3.

```ken
import Core.Logic.Conjunction (and_intro)
import Core.Logic.Or (Or, Inl, Inr)
import Core.Logic.Transport (sym, trans)
import Data.Collections.Derived (list_elem, sort, list_concat)
import Data.Time.EventTrace
  (Event, MkEvent, event_before, time_ordered, trace_merge_all,
   trace_merge_all_ordered_sort)

const e1 : Event Bool = MkEvent Bool (Suc Zero) True
const e2 : Event Bool = MkEvent Bool (Suc (Suc Zero)) True
const e3 : Event Bool = MkEvent Bool (Suc (Suc (Suc Zero))) True
const xs : List (Event Bool) = Cons (Event Bool) e1 (Cons (Event Bool) e3 (Nil (Event Bool)))
const ys : List (Event Bool) = Cons (Event Bool) e2 (Nil (Event Bool))
const batches : List (List (Event Bool)) =
  Cons (List (Event Bool)) xs (Cons (List (Event Bool)) ys (Nil (List (Event Bool))))

theorem xs_ordered : time_ordered Bool xs =
  and_intro
    (Equal Bool (event_before Bool e1 e3) True)
    (time_ordered Bool (Cons (Event Bool) e3 (Nil (Event Bool))))
    Proved
    Proved
theorem ys_ordered : time_ordered Bool ys = Proved

theorem batches_ordered (t : List (Event Bool))
    : list_elem (List (Event Bool)) t batches → time_ordered Bool t =
  λmember.
    elim_trunc
      (time_ordered Bool t)
      (λcase.
        match case {
          Inl same_xs ↦ J (λu _. time_ordered Bool u) xs_ordered (sym (List (Event Bool)) t xs same_xs);
          Inr later ↦
            elim_trunc
              (time_ordered Bool t)
              (λcase_ys.
                match case_ys {
                  Inl same_ys ↦
                    J (λu _. time_ordered Bool u) ys_ordered (sym (List (Event Bool)) t ys same_ys);
                  Inr none ↦ absurd none
                })
              later
        })
      member
```

## stdlib/time-and-events/ordered-batches-merge-is-sorted-concatenation

- spec: §23, `trace_merge_all_ordered_sort`.
- promise class: durable invariant.
- given: a fresh copy of the shared prefix with batches `[e1,e3]` and `[e2]`.
- expect: accept. The law identifies the merge with the sort of the
  concatenation; `Refl` identifies that sort with `[e1,e2,e3]`.
- measured: acceptance of the ordered-batch premise and law-based result.
- claimed: on these batches, merge returns the sorted concatenation.
- gap: this closed client does not establish the generic theorem.
- controls: the batches have the same events as concatenation `[e1,e3,e2]`,
  but their sorted concatenation is `[e1,e2,e3]`.
- why: times `[1,3]` and `[2]` distinguish sorting the concatenation from
  returning it unchanged.

```ken
const expected : List (Event Bool) =
  Cons (Event Bool) e1
    (Cons (Event Bool) e2 (Cons (Event Bool) e3 (Nil (Event Bool))))

theorem merge_result :
    Equal (List (Event Bool)) (trace_merge_all Bool batches) expected =
  trans
    (List (Event Bool))
    (trace_merge_all Bool batches)
    (sort (Event Bool) (event_before Bool)
      (list_concat (Event Bool) batches))
    expected
    (trace_merge_all_ordered_sort Bool batches batches_ordered)
    Refl
```

## stdlib/time-and-events/reject-ordered-batches-merge-as-plain-concatenation

- spec: §23, `trace_merge_all_ordered_sort`.
- promise class: durable invariant.
- given: a fresh copy of the shared prefix without the accepted result block.
- expect: refuse at this theorem's closing `Refl` with elaborator
  `TypeMismatch`.
- measured: the unchanged law-based proof with the claimed result changed
  to `[e1,e3,e2]`.
- claimed: ordered batches do not make merge equal their plain concatenation.
- controls: the inputs and ordered-batch witness are identical to the
  accepted case; only the claimed result differs.
- why: the law's result is the sorted concatenation `[e1,e2,e3]`, not
  `[e1,e3,e2]`.

```ken
const expected : List (Event Bool) =
  Cons (Event Bool) e1
    (Cons (Event Bool) e3 (Cons (Event Bool) e2 (Nil (Event Bool))))

theorem merge_result :
    Equal (List (Event Bool)) (trace_merge_all Bool batches) expected =
  trans
    (List (Event Bool))
    (trace_merge_all Bool batches)
    (sort (Event Bool) (event_before Bool)
      (list_concat (Event Bool) batches))
    expected
    (trace_merge_all_ordered_sort Bool batches batches_ordered)
    Refl
```

## stdlib/time-and-events/reject-premise-free-ordered-batch-sort

- spec: §23, `trace_merge_all_ordered_sort` and its ordered-batch premise.
- promise class: durable invariant.
- given: a fresh copy of the shared prefix plus one batch `[e5,e1]`, at
  times `[5,1]`; no ordering premise is supplied.
- expect: accept the two concrete read-offs below, then refuse the
  premise-free equation at its own closing `Refl` with `TypeMismatch`.
- measured: merge is `[e5,e1]`; sorting its concatenation is `[e1,e5]`; the
  direct premise-free equality is refused.
- claimed: the sorted-concatenation equation requires every batch to be
  ordered.
- controls: the two concrete read-offs prove the fixture is reachable and
  the refusal is not caused by an unrelated name or import error.
- why: the single unordered batch is returned unchanged by merge, while
  sorting its concatenation reverses the distinct timestamps.

```ken
const e5 : Event Bool =
  MkEvent Bool (Suc (Suc (Suc (Suc (Suc Zero))))) True
const unordered_batch : List (Event Bool) =
  Cons (Event Bool) e5 (Cons (Event Bool) e1 (Nil (Event Bool)))
const unordered_batches : List (List (Event Bool)) =
  Cons (List (Event Bool)) unordered_batch (Nil (List (Event Bool)))
const merged_value : List (Event Bool) =
  Cons (Event Bool) e5 (Cons (Event Bool) e1 (Nil (Event Bool)))
const sorted_value : List (Event Bool) =
  Cons (Event Bool) e1 (Cons (Event Bool) e5 (Nil (Event Bool)))

theorem merge_keeps_one_batch :
    Equal (List (Event Bool))
      (trace_merge_all Bool unordered_batches) merged_value =
  Refl

theorem sort_orders_the_concatenation :
    Equal (List (Event Bool))
      (sort (Event Bool) (event_before Bool)
        (list_concat (Event Bool) unordered_batches)) sorted_value =
  Refl

theorem premise_free_merge_sorts_batches :
    Equal (List (Event Bool))
      (trace_merge_all Bool unordered_batches)
      (sort (Event Bool) (event_before Bool)
        (list_concat (Event Bool) unordered_batches)) =
  Refl
```

These cases do not establish the generic law. No official conformance-runner
execution, CI, publication, landing, or trust measurement is claimed.
