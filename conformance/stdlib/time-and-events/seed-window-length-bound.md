# Window length bound — seed cases

Format: `../../README.md`.
Spec: `spec/50-stdlib/69-time-and-events.md` §12.
Frame: `docs/program/issues/SPEC-TIME-WINDOW-LENGTH-BOUND.md`.

This client applies the generic law to a closed `Bool` trace and checks
its concrete length. It uses the `e1`, `e4`, `e7` trace from the
neighboring time-window seeds. The client does not establish the law's
generic quantifiers or its checked proof body.

Architect's D0/D0b scratch observations ground the proof form: the
public law application was accepted; `window_keeps_two` and the
reverse-bound control under `[0,9)` were accepted with `Proved`. The
`keeps_three_wrong` claim and reverse-bound claim under `[3,9)` were
rejected at their own `Proved` claims. `Refl` refused both reverse-bound
rows with the same goal-shape error, so it does not discriminate them.
These are scratch design observations, not execution of this seed or an
official conformance run.

## Accepted client

The trace has events at times 1, 4 and 7. Under `[3,9)`, the window
keeps `e4` and `e7`, so its length is 2 and the trace length is 3.
Under `[0,9)`, all three events remain.

```ken
import Core.Classes.LawfulClasses (IsTrue, leq_nat)

import Data.Collections.List (length)

import Data.Time.EventTrace (Event, MkEvent, trace_window, trace_window_length_bound)

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

const e4 : Event Bool = MkEvent Bool t4 False

const e7 : Event Bool = MkEvent Bool t7 True

const tr : List (Event Bool) =
  Cons (Event Bool) e1 (Cons (Event Bool) e4 (Cons (Event Bool) e7 (Nil (Event Bool))))

theorem window_length_bound_instance
    : IsTrue
        (leq_nat (length (Event Bool) (trace_window Bool t3 t9 tr)) (length (Event Bool) tr)) =
  trace_window_length_bound Bool t3 t9 tr

theorem window_keeps_two
    : IsTrue (leq_nat (Suc (Suc Zero)) (length (Event Bool) (trace_window Bool t3 t9 tr))) =
  Proved

theorem reverse_bound_control_all_events_kept
    : IsTrue
        (leq_nat (length (Event Bool) tr) (length (Event Bool) (trace_window Bool t0 t9 tr))) =
  Proved
```

### stdlib/time-and-events/window-length-bound

- spec: `spec/50-stdlib/69-time-and-events.md` §12.
- promise class: durable invariant.
- given: the public law on the `e1`, `e4`, `e7` trace under `[3,9)`.
- expect: the law application accepts; the window has length 2 and the
  trace has length 3; the same-form reverse-bound control accepts under
  `[0,9)`.
- claimed: a window's length never exceeds its input trace's length,
  without an ordering or bounds premise.
- gap: these closed clients instantiate one carrier and trace; they do
  not establish the generic law or its checked proof body.
- why: the law instance exercises the public theorem. The concrete
  control checks the fixed trace independently of that instance.

## Refused window-count claim

The next block changes only the claimed lower bound from 2 to 3. The
`[3,9)` window contains two of the three events, so the claim is false.

```ken
-- Refuse: the [3,9) window contains two events, not three.
theorem refuse_keeps_three_wrong
    : IsTrue
        (leq_nat (Suc (Suc (Suc Zero))) (length (Event Bool) (trace_window Bool t3 t9 tr))) =
  Proved
```

### stdlib/time-and-events/reject-keeps-three-wrong

- spec: `spec/50-stdlib/69-time-and-events.md` §12.
- promise class: durable invariant.
- given: the same trace and window as `window_keeps_two`, claiming a
  lower bound of 3 instead of 2.
- expect: reject `refuse_keeps_three_wrong` at its own `Proved` claim
  with `KernelRejected TypeMismatch`; the accepted two-event control
  passes.
- claimed: the closed window count is not at least 3.
- gap: this refusal does not establish every false count is rejected.
- why: the positive and refused claims share the trace, window and proof
  form; only the lower bound changes.

## Refused reverse bound

The reverse claim is false under `[3,9)`: it says that the three-event
trace has length at most the two-event window. The accepted client above
controls the same claim under `[0,9)`, where all three events remain.

```ken
-- Refuse: 3 is not at most 2.
theorem refuse_reverse_window_length_bound
    : IsTrue
        (leq_nat (length (Event Bool) tr) (length (Event Bool) (trace_window Bool t3 t9 tr))) =
  Proved
```

### stdlib/time-and-events/reject-reverse-window-length-bound

- spec: `spec/50-stdlib/69-time-and-events.md` §12.
- promise class: durable invariant.
- given: the trace and reverse-bound claim under `[3,9)`.
- expect: reject `refuse_reverse_window_length_bound` at its own
  `Proved` claim with `KernelRejected TypeMismatch`; the same-form
  `[0,9)` control passes.
- claimed: discarding an event does not preserve or increase trace
  length.
- gap: one fixed trace and two windows do not establish the generic law.
- why: the refusal and control share the trace, operation, claim and
  proof form. Only the lower bound changes, from 3 to 0.

## Evidence boundary

The Architect's D0/D0b checks measured scratch clients, not this seed.
The seed must be exercised by the assembled candidate's named package
test. No official conformance-runner execution, CI, publication,
landing or landed-trust result is claimed here.
