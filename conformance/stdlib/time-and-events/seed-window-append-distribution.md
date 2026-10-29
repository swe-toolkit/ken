# Window append — seed cases

Format: `../../README.md`.
Spec: `spec/50-stdlib/69-time-and-events.md` §8.
Frame: `docs/program/issues/SPEC-TIME-WINDOW-APPEND-DISTRIBUTION.md`.

This closed source-shaped client checks one specialization of
`trace_window_append`; it does not establish the generic theorem. `xs` is
intentionally unordered. The half-open window `[2,6)` keeps one event from
each input and drops one from each. The accepted prefix checks exact window
values independently, applies the public law, and includes an unswapped
same-shape `Refl` control. Each refusal appends only one false claim to a fresh
copy of that prefix.

Architect's D0 is scratch evidence only. The §8 catalog export and package test
are separate candidate work; this seed is authored conformance material, not an
executed official conformance case. No catalog, test, CI, publication, landing,
or landed-trust result is claimed here.

## Accepted client

The input `xs = [e5,e1]` is unordered: its first adjacent comparison is false.
`ys = [e3,e8]` is ordered. The window `[2,6)` retains `e5` at time 5 and `e3`
at time 3, while dropping `e1` at time 1 and `e8` at time 8. Filtering the
concatenation preserves that input order, so both sides of the law are
`[e5,e3]`.

```ken
import Core.Classes.LawfulClasses (leq_nat)
import Data.Collections.Derived (list_append)
import Data.Time.EventTrace
  (Event, MkEvent, event_before, event_time, trace_window,
   trace_window_append)

const t0 : Nat = Zero
const t1 : Nat = Suc t0
const t2 : Nat = Suc t1
const t3 : Nat = Suc t2
const t4 : Nat = Suc t3
const t5 : Nat = Suc t4
const t6 : Nat = Suc t5
const t7 : Nat = Suc t6
const t8 : Nat = Suc t7

const e5 : Event Bool = MkEvent Bool t5 True
const e1 : Event Bool = MkEvent Bool t1 False
const e3 : Event Bool = MkEvent Bool t3 False
const e8 : Event Bool = MkEvent Bool t8 True

const xs : List (Event Bool) =
  Cons (Event Bool) e5 (Cons (Event Bool) e1 (Nil (Event Bool)))
const ys : List (Event Bool) =
  Cons (Event Bool) e3 (Cons (Event Bool) e8 (Nil (Event Bool)))
const xs_window : List (Event Bool) =
  Cons (Event Bool) e5 (Nil (Event Bool))
const ys_window : List (Event Bool) =
  Cons (Event Bool) e3 (Nil (Event Bool))
const appended_window : List (Event Bool) =
  Cons (Event Bool) e5 (Cons (Event Bool) e3 (Nil (Event Bool)))
const swapped_windows : List (Event Bool) =
  list_append (Event Bool)
    (trace_window Bool t2 t6 ys)
    (trace_window Bool t2 t6 xs)

fn first_event_time (events : List (Event Bool)) : Nat =
  match events {
    Nil ↦ Zero;
    Cons head tail ↦ event_time Bool head
  }

theorem xs_head_comparison_is_false :
  Equal Bool (event_before Bool e5 e1) False = Proved

theorem xs_window_is_exact :
  Equal (List (Event Bool)) (trace_window Bool t2 t6 xs) xs_window = Refl

theorem ys_window_is_exact :
  Equal (List (Event Bool)) (trace_window Bool t2 t6 ys) ys_window = Refl

theorem append_window_is_exact :
  Equal (List (Event Bool))
    (trace_window Bool t2 t6 (list_append (Event Bool) xs ys))
    appended_window = Refl

theorem trace_window_append_instance :
  Equal (List (Event Bool))
    (trace_window Bool t2 t6 (list_append (Event Bool) xs ys))
    (list_append (Event Bool)
      (trace_window Bool t2 t6 xs)
      (trace_window Bool t2 t6 ys)) =
  trace_window_append Bool t2 t6 xs ys

theorem unswapped_same_shape_refl_control :
  Equal (List (Event Bool))
    (trace_window Bool t2 t6 (list_append (Event Bool) xs ys))
    (list_append (Event Bool)
      (trace_window Bool t2 t6 xs)
      (trace_window Bool t2 t6 ys)) = Refl

theorem swapped_windows_start_at_three :
  Equal Nat (first_event_time swapped_windows) t3 = Proved

theorem swapped_windows_do_not_start_at_five :
  Equal Bool (leq_nat t5 (first_event_time swapped_windows)) False = Proved
```

### stdlib/time-and-events/window-append-law-instance

- spec: `spec/50-stdlib/69-time-and-events.md` §8.1.
- promise class: durable invariant.
- given: `xs`, `ys`, and the window `[t2,t6)` from the accepted client.
- expect: `trace_window_append_instance` applies the public law; each input
  window and the window of the concatenation reduce by `Refl` to the exact
  lists recorded above. The unswapped, same-shape law expression also reduces
  by `Refl`.
- claimed: this specialization agrees with filtering each input and appending
  the results in original input order.
- gap: one finite Bool instance does not prove the universal law for every
  payload, bound pair, or trace.
- why: the explicit unordered witness prevents this instance from silently
  testing only an ordered input; separate `Refl` values keep the law application
  from serving as its own output oracle.

## Refused swapped-window claims

For each case, use a fresh copy of the entire accepted client above and append
only that case's theorem. The prefix must accept, including its unswapped
same-shape `Refl` control and independent first-event observer. Do not combine
the refusals: the first rejection would mask the second.

### stdlib/time-and-events/reject-swapped-window-append-by-refl

- spec: `spec/50-stdlib/69-time-and-events.md` §8.1.
- promise class: discriminating.
- given: the accepted client above followed by only the false equality theorem
  below.
- expect: the prefix accepts and the appended `Refl` claim is refused. Do not
  pin a diagnostic variant or message.
- claimed: the windowed concatenation is not equal to the concatenation with
  the two windows swapped on this input.
- gap: this finite refusal does not prove the generic law or establish every
  possible counterexample.
- why: the left side is `[e5,e3]` and the swapped right side is `[e3,e5]`; the
  independent observer confirms the swapped side starts at time 3, not 5.

```ken
-- Refused: the swapped windows reverse the retained events.
theorem reject_swapped_window_append_by_refl :
  Equal (List (Event Bool))
    (trace_window Bool t2 t6 (list_append (Event Bool) xs ys))
    (list_append (Event Bool)
      (trace_window Bool t2 t6 ys)
      (trace_window Bool t2 t6 xs)) = Refl
```

### stdlib/time-and-events/reject-swapped-window-append-by-public-law

- spec: `spec/50-stdlib/69-time-and-events.md` §8.1.
- promise class: discriminating.
- given: a separate fresh copy of the accepted client above, followed by only
  the false equality theorem below.
- expect: the prefix accepts and the theorem body using the public law is
  refused. Do not pin a diagnostic variant or message.
- claimed: `trace_window_append` does not prove the same equality when the
  right-hand windows are swapped.
- gap: this closed application does not prove the generic law or every false
  reordered claim.
- why: the public law concludes the unswapped structural equality; the swapped
  right side has the opposite first event, as the independent observer shows.

```ken
-- Refused: the public law proves the unswapped order, not this goal.
theorem reject_swapped_window_append_by_public_law :
  Equal (List (Event Bool))
    (trace_window Bool t2 t6 (list_append (Event Bool) xs ys))
    (list_append (Event Bool)
      (trace_window Bool t2 t6 ys)
      (trace_window Bool t2 t6 xs)) =
  trace_window_append Bool t2 t6 xs ys
```

## Evidence boundary

The D0 and this closed seed have separate roles. D0 checked the generic proof
and public client in scratch; this seed records a finite accepted instance and
two false-claim refusals for the future assembled candidate. Neither is an
official conformance-runner result, CI result, publication, landing, or landed
trust measurement.
