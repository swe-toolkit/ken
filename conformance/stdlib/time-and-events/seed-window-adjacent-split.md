# Adjacent half-open window split — seed cases

Format: `../../README.md`.
Spec: `spec/50-stdlib/69-time-and-events.md` §7.

This source-shaped client specializes `trace_window_split` on one closed,
ordered Bool-payload trace. Its events are at 1, 3 and 7, with bounds 1, 3
and 9. The event at the split point is excluded from `[1,3)` and included
in `[3,9)`. Independent `Refl` checks compute the whole and both subwindows;
a separate `Refl` control checks the true split in the same equality shape.

Architect's AC-0 D0 checked a separate closed seed-shaped client and the
swapped refusal on scratch. That scratch result does not establish execution
of this file. No roots-based package execution is claimed at this seed turn;
no official conformance-runner execution, CI, publication, landing or landed
trust is claimed.

## Accepted client

The ordered trace is `[e1,e3,e7]`, at times 1, 3 and 7. The whole window
`[1,9)` keeps all three. The left window `[1,3)` keeps only `e1`, while the
right window `[3,9)` keeps `e3,e7`. The time-3 event therefore exercises the
half-open boundary: it is absent on the left and present on the right.

```ken
import Core.Logic.Conjunction (and_intro)

import Data.Collections.Derived (list_append)

import Data.Time.EventTrace
  (Event,
    MkEvent,
    event_before,
    event_in_window,
    time_ordered,
    trace_window,
    trace_window_split)

const t0 : Nat = Zero

const t1 : Nat = Suc t0

const t2 : Nat = Suc t1

const t3 : Nat = Suc t2

const t4 : Nat = Suc t3

const t5 : Nat = Suc t4

const t6 : Nat = Suc t5

const t7 : Nat = Suc t6

const t8 : Nat = Suc t7

const t9 : Nat = Suc t8

const e1 : Event Bool = MkEvent Bool t1 True

const e3 : Event Bool = MkEvent Bool t3 False

const e7 : Event Bool = MkEvent Bool t7 True

const tr : List (Event Bool) =
  Cons (Event Bool) e1 (Cons (Event Bool) e3 (Cons (Event Bool) e7 (Nil (Event Bool))))

const left_value : List (Event Bool) = Cons (Event Bool) e1 (Nil (Event Bool))

const right_value : List (Event Bool) =
  Cons (Event Bool) e3 (Cons (Event Bool) e7 (Nil (Event Bool)))

theorem tr_ordered : time_ordered Bool tr =
  and_intro
    (Equal Bool (event_before Bool e1 e3) True)
    (time_ordered Bool right_value)
    Proved
    (and_intro
      (Equal Bool (event_before Bool e3 e7) True)
      (time_ordered Bool (Cons (Event Bool) e7 (Nil (Event Bool))))
      Proved
      Proved)

theorem split_instance
    : Equal
        (List (Event Bool))
        (trace_window Bool t1 t9 tr)
        (list_append (Event Bool) (trace_window Bool t1 t3 tr) (trace_window Bool t3 t9 tr)) =
  trace_window_split Bool t1 t3 t9 tr tr_ordered Proved Proved

theorem whole_value : Equal (List (Event Bool)) (trace_window Bool t1 t9 tr) tr = Refl

theorem left_window_value : Equal (List (Event Bool)) (trace_window Bool t1 t3 tr) left_value =
  Refl

theorem right_window_value
    : Equal (List (Event Bool)) (trace_window Bool t3 t9 tr) right_value =
  Refl

theorem same_shape_control
    : Equal
        (List (Event Bool))
        (trace_window Bool t1 t9 tr)
        (list_append (Event Bool) (trace_window Bool t1 t3 tr) (trace_window Bool t3 t9 tr)) =
  Refl

theorem mid_event_excluded_left : Equal Bool (event_in_window Bool t1 t3 e3) False = Proved
```

### stdlib/time-and-events/window-adjacent-split

- spec: `spec/50-stdlib/69-time-and-events.md` §7.
- promise class: durable invariant.
- given: `tr`, the closed ordered trace, and bounds `[t1,t3)` and `[t3,t9)`.
- expect: `split_instance` applies the public law; each window and the
  whole-window result is also computed independently by `Refl`.
- asserted: the ordering proof, both bound premises, the three exact list
  results, the true same-shape split control, and the independent `False`
  value for the event at `t3` in the left window.
- claimed: the law's result holds for this closed ordered trace, and the
  split point belongs to the right subwindow only.
- gap: this finite instance does not establish the universal law or the
  necessity of its premises.
- why: the trace has an event below `t3`, one exactly at `t3`, and one above;
  the two nonempty subwindows are independently observable.

## Refused swapped-window split

Append only the following block to a fresh copy of the complete accepted
client above. The accepted client and its same-shape control must first pass;
the appended false list equality must be refused at its own `Refl`, not at an
import or earlier fixture declaration. The expected swapped result is
`[e3,e7,e1]`, not `[e1,e3,e7]`.

```ken
-- Refused: the append reverses the two nonempty window results.
theorem refuse_swapped_split
    : Equal
        (List (Event Bool))
        (trace_window Bool t1 t9 tr)
        (list_append (Event Bool) (trace_window Bool t3 t9 tr) (trace_window Bool t1 t3 tr)) =
  Refl
```

### stdlib/time-and-events/reject-swapped-window-split

- spec: `spec/50-stdlib/69-time-and-events.md` §7.
- promise class: durable invariant.
- given: a fresh accepted client above, followed only by the swapped claim.
- expect: the accepted prefix passes; the appended claim is refused at its
  own `Refl`.
- asserted: the true split control accepts in the same equality shape, while
  the false claim reverses the exact, nonempty subwindows.
- claimed: swapping the two subwindows does not preserve this trace's list.
- gap: this single refusal does not establish a generic noncommutativity law
  or reject unordered traces.
- why: the accepted controls compute the whole list as `[e1,e3,e7]` and the
  swapped append as `[e3,e7,e1]` on the same trace.

## Evidence boundary

Architect's AC-0 D0 (`evt_6q0ygegng78nh`) reports a checked generic proof,
no new `Axiom`, unchanged scratch trusted GlobalIds (114 → 114), and a
closed client plus swapped refusal on its scratch tree. Its client is not
execution of this seed file or of a shipped package test. This source
states the cases and expected behaviors; any seed-execution claim must
come from a test that reads this file. No official conformance-runner
execution, CI, publication, landing or landed trust is claimed.
