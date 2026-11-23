# Singleton trace merge and insertion order

Format: `../../README.md`.
Spec: `spec/50-stdlib/69-time-and-events.md` §21 in Spec child
`88781a478a18ccf9e0e0b9e3d043114a086a96b1`.
Frame: `docs/program/issues/SPEC-TIME-MERGE-SINGLETONS-SORT.md`.

This conformance-only child is based on
`aadce7362dee2e2d5e936d73d7b13a7e7dd830ea` and is intended to be assembled
with the named Spec child. It has not been executed. The closed value controls
and generic law application below are source-shaped cases, not proof of the
generic theorem or evidence of an official runner, package-test, CI,
publication, landing or trust result.

## Accepted tie equation

Both events use one shared timestamp and distinct Bool payloads. The expected
list is written independently of the law applications. Closed value controls
pin first-first order for `trace_merge`/`insert` and for merge-all/sort; the
client also applies both public laws at these inputs.

```ken
import Data.Collections.Derived (insert, map, sort)

import Data.Time.EventTrace
  (Event, MkEvent, event_time, event_before, event_singleton, trace_merge,
   trace_merge_all, trace_merge_single_insert,
   trace_merge_all_singletons_sort)

const tie_time : Nat = Suc Zero

const first : Event Bool = MkEvent Bool tie_time True

const second : Event Bool = MkEvent Bool tie_time False

const expected : List (Event Bool) =
  Cons (Event Bool) first
    (Cons (Event Bool) second (Nil (Event Bool)))

const two_events : List (Event Bool) =
  Cons (Event Bool) first (Cons (Event Bool) second (Nil (Event Bool)))

const singleton_traces : List (List (Event Bool)) =
  map (Event Bool) (List (Event Bool)) (event_singleton Bool) two_events

theorem events_share_timestamp :
  Equal Nat (event_time Bool first) (event_time Bool second) = Refl

theorem first_precedes_tied_second :
  Equal Bool (event_before Bool first second) True = Proved

theorem second_precedes_tied_first :
  Equal Bool (event_before Bool second first) True = Proved

theorem merge_tie_value_is_first_first :
  Equal (List (Event Bool))
    (trace_merge Bool
      (Cons (Event Bool) first (Nil (Event Bool)))
      (Cons (Event Bool) second (Nil (Event Bool))))
    expected = Refl

theorem insert_tie_value_is_first_first :
  Equal (List (Event Bool))
    (insert (Event Bool) (event_before Bool) first
      (Cons (Event Bool) second (Nil (Event Bool))))
    expected = Refl

theorem trace_merge_single_insert_first_tie :
  Equal (List (Event Bool))
    (trace_merge Bool
      (Cons (Event Bool) first (Nil (Event Bool)))
      (Cons (Event Bool) second (Nil (Event Bool))))
    (insert (Event Bool) (event_before Bool) first
      (Cons (Event Bool) second (Nil (Event Bool)))) =
  trace_merge_single_insert
    Bool first (Cons (Event Bool) second (Nil (Event Bool)))

theorem merge_all_singletons_tie_value_is_first_first :
  Equal (List (Event Bool))
    (trace_merge_all Bool singleton_traces)
    expected = Refl

theorem sort_tie_value_is_first_first :
  Equal (List (Event Bool))
    (sort (Event Bool) (event_before Bool) two_events)
    expected = Refl

theorem trace_merge_all_singletons_sort_first_tie :
  Equal (List (Event Bool))
    (trace_merge_all Bool singleton_traces)
    (sort (Event Bool) (event_before Bool) two_events) =
  trace_merge_all_singletons_sort Bool two_events
```

### stdlib/time-and-events/singleton-merge-tie-laws

- spec: §21, `trace_merge_single_insert` and
  `trace_merge_all_singletons_sort`.
- promise class: durable invariant.
- given: two singleton traces at the same timestamp, with payloads `True` and
  `False`.
- expect: closed value controls put `first` before `second`; both public laws
  accept their exact equations at this fixture.
- measured: the shared timestamp, both `event_before` results, the closed
  outputs of merge, insertion, merge-all and sort, and both law applications.
- claimed: both equations preserve first-event-first order at a tie, not merely
  the same event multiset.
- gap: these closed Bool instances are not proofs of the generic quantifiers.
- why: distinct payloads make the two event orders observably different.

## Isolated refusal of the swapped insertion

On a fresh copy of the complete accepted client above, append only the refusal
fence below. Do not run this fence alone. The inputs, tie witnesses and accepted
law application must first succeed. The only changed claim is that the same
merge equals `insert` with the two events swapped; the proof term remains the
same law application. The refusal must be `KernelRejected(TypeMismatch)` at
this theorem. Do not use `Refl` as the refused proof.

```ken
-- Refuse: equal-time insertion puts the first event before the second.
theorem refuse_swapped_singleton_insert_order :
  Equal (List (Event Bool))
    (trace_merge Bool
      (Cons (Event Bool) first (Nil (Event Bool)))
      (Cons (Event Bool) second (Nil (Event Bool))))
    (insert (Event Bool) (event_before Bool) second
      (Cons (Event Bool) first (Nil (Event Bool)))) =
  trace_merge_single_insert
    Bool first (Cons (Event Bool) second (Nil (Event Bool)))
```

### stdlib/time-and-events/reject-swapped-singleton-insert-order

- spec: §21, `trace_merge_single_insert`.
- promise class: durable invariant.
- given: a fresh copy of the accepted client and only the appended refusal
  theorem.
- expect: the accepted prefix succeeds; the swapped equation is refused at
  the law application as `KernelRejected(TypeMismatch)`.
- measured: the same tied events and law application, with only the claimed
  insertion order swapped.
- claimed: the exported equation fixes tie order rather than permitting any
  permutation of the events.
- gap: this one counterexample does not establish the generic theorem or other
  consequences such as sortedness and stability.
- why: the different payloads keep first-first and second-first lists distinct.

## Evidence boundary

The exact Spec child named above supplies §21 in the assembled candidate. This
seed has not been executed against an assembled candidate. It does not claim
execution of the package test, official conformance runner, AC-T, CI,
publication, landing or a measured landed trust set. The generic proofs and
assembled AC-T remain separate evidence.
