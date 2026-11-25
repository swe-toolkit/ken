# Ordered trace merge and sorting the append

Format: `../../README.md`.
Spec: `spec/50-stdlib/69-time-and-events.md` §22.

## Shared accepted prefix

Use this prefix for each case independently. The inputs are ordered, but
their append is not: time 2 follows time 3. The two result blocks differ
only in the event order in `expected`.

```ken
import Core.Logic.Conjunction (and_intro)

import Core.Logic.Transport (trans)

import Data.Collections.Derived (sort, list_append)

import Data.Time.EventTrace
  (Event, MkEvent, event_before, time_ordered, trace_merge, trace_merge_ordered_sort)

const e1 : Event Bool = MkEvent Bool (Suc Zero) True

const e2 : Event Bool = MkEvent Bool (Suc (Suc Zero)) True

const e3 : Event Bool = MkEvent Bool (Suc (Suc (Suc Zero))) True

const xs : List (Event Bool) = Cons (Event Bool) e1 (Cons (Event Bool) e3 (Nil (Event Bool)))

const ys : List (Event Bool) = Cons (Event Bool) e2 (Nil (Event Bool))

theorem xs_ordered : time_ordered Bool xs =
  and_intro
    (Equal Bool (event_before Bool e1 e3) True)
    (time_ordered Bool (Cons (Event Bool) e3 (Nil (Event Bool))))
    Proved
    Proved

theorem ys_ordered : time_ordered Bool ys = Proved
```

## stdlib/time-and-events/ordered-merge-is-sorted-append

- spec: §22, `trace_merge_ordered_sort`.
- promise class: durable invariant.
- given: the shared prefix with `xs = [e1, e3]` and `ys = [e2]`.
- expect: accept. The law identifies the merge with the sort of the append;
  `Refl` identifies that sort with `[e1, e2, e3]`.
- measured: acceptance of this ordering witness and law-based result.
- claimed: on these inputs the merge returns the sorted append.
- gap: the closed client does not establish the generic theorem or measure
  the trusted base.

```ken
const expected : List (Event Bool) =
  Cons (Event Bool) e1 (Cons (Event Bool) e2 (Cons (Event Bool) e3 (Nil (Event Bool))))

theorem merge_result : Equal (List (Event Bool)) (trace_merge Bool xs ys) expected =
  trans
    (List (Event Bool))
    (trace_merge Bool xs ys)
    (sort (Event Bool) (event_before Bool) (list_append (Event Bool) xs ys))
    expected
    (trace_merge_ordered_sort Bool xs ys xs_ordered ys_ordered)
    Refl
```

## stdlib/time-and-events/reject-ordered-merge-is-plain-append

- spec: §22, `trace_merge_ordered_sort`.
- promise class: durable invariant.
- given: a fresh copy of the shared prefix, without the accepted result block.
- expect: refuse at this theorem's closing `Refl` with elaborator
  `TypeMismatch` (`` Refl expects an `Eq`-shaped goal ``).
- measured: the unchanged law-based proof with the claimed result changed
  to `[e1, e3, e2]`.
- claimed: having the same events is insufficient; the result must be in
  sorted order, not the original append order.
- gap: this one refusal does not check tie handling or establish the generic
  law. The generic checked declaration and separate package controls cover
  those boundaries.

```ken
const expected : List (Event Bool) =
  Cons (Event Bool) e1 (Cons (Event Bool) e3 (Cons (Event Bool) e2 (Nil (Event Bool))))

theorem merge_result : Equal (List (Event Bool)) (trace_merge Bool xs ys) expected =
  trans
    (List (Event Bool))
    (trace_merge Bool xs ys)
    (sort (Event Bool) (event_before Bool) (list_append (Event Bool) xs ys))
    expected
    (trace_merge_ordered_sort Bool xs ys xs_ordered ys_ordered)
    Refl
```

These are source cases, not a report of official conformance execution.
