# Windowing an ordered merge — seed cases

Format: `../../README.md`.
Spec: `spec/50-stdlib/69-time-and-events.md` §3.

This authored source-shaped seed targets the public
`trace_window_merge` law. The ordered inputs have explicit closed
proofs. Separate exact-output cases show the operations' values and
make the kept cross-input tie observable. The unordered refusals are
false value claims, not invalid-input checks: `trace_merge` accepts
unordered lists.

Architect D0 checked the generic proof and clients in scratch on base
`81ab720ff5170b4d0f45ea320101760e56fd98ca`. This seed has not been
executed; the §3 catalog export is not delivered. No shipped-seed
execution, CI, publication, landing, or landed trust measurement is
claimed.

## Closed fixtures and ordered positive case

The window `[1,7)` drops `0a` and `8b` while retaining both events at
time 3. Payloads `True` and `False` make the cross-input tie visible.
The law instance and the two exact outputs are separate assertions.

```ken
import Data.Time.EventTrace
  (Event, MkEvent, event_before, time_ordered, trace_merge,
   trace_window, trace_window_merge)

const t0 : Nat = Zero
const t1 : Nat = Suc t0
const t2 : Nat = Suc t1
const t3 : Nat = Suc t2
const t4 : Nat = Suc t3
const t5 : Nat = Suc t4
const t6 : Nat = Suc t5
const t7 : Nat = Suc t6
const t8 : Nat = Suc t7

const e0a : Event Bool = MkEvent Bool t0 True
const e3a : Event Bool = MkEvent Bool t3 True
const e6a : Event Bool = MkEvent Bool t6 True
const e3b : Event Bool = MkEvent Bool t3 False
const e5b : Event Bool = MkEvent Bool t5 False
const e8b : Event Bool = MkEvent Bool t8 False

const xs_ordered : List (Event Bool) =
  Cons (Event Bool) e0a
    (Cons (Event Bool) e3a
      (Cons (Event Bool) e6a (Nil (Event Bool))))
const ys_ordered : List (Event Bool) =
  Cons (Event Bool) e3b
    (Cons (Event Bool) e5b
      (Cons (Event Bool) e8b (Nil (Event Bool))))
const expected_ordered : List (Event Bool) =
  Cons (Event Bool) e3a
    (Cons (Event Bool) e3b
      (Cons (Event Bool) e5b
        (Cons (Event Bool) e6a (Nil (Event Bool)))))

theorem xs_is_time_ordered : time_ordered Bool xs_ordered =
  and_intro
    (Equal Bool (event_before Bool e0a e3a) True)
    (time_ordered Bool
      (Cons (Event Bool) e3a
        (Cons (Event Bool) e6a (Nil (Event Bool)))))
    Proved
    (and_intro
      (Equal Bool (event_before Bool e3a e6a) True)
      (time_ordered Bool (Cons (Event Bool) e6a (Nil (Event Bool))))
      Proved Proved)

theorem ys_is_time_ordered : time_ordered Bool ys_ordered =
  and_intro
    (Equal Bool (event_before Bool e3b e5b) True)
    (time_ordered Bool
      (Cons (Event Bool) e5b
        (Cons (Event Bool) e8b (Nil (Event Bool)))))
    Proved
    (and_intro
      (Equal Bool (event_before Bool e5b e8b) True)
      (time_ordered Bool (Cons (Event Bool) e8b (Nil (Event Bool))))
      Proved Proved)

theorem ordered_distribution_law_instance :
  Equal (List (Event Bool))
    (trace_window Bool t1 t7 (trace_merge Bool xs_ordered ys_ordered))
    (trace_merge Bool
      (trace_window Bool t1 t7 xs_ordered)
      (trace_window Bool t1 t7 ys_ordered)) =
  trace_window_merge
    Bool t1 t7 xs_ordered ys_ordered xs_is_time_ordered ys_is_time_ordered

theorem window_of_merge_exact_value :
  Equal (List (Event Bool))
    (trace_window Bool t1 t7 (trace_merge Bool xs_ordered ys_ordered))
    expected_ordered = Refl

theorem merge_of_windows_exact_value :
  Equal (List (Event Bool))
    (trace_merge Bool
      (trace_window Bool t1 t7 xs_ordered)
      (trace_window Bool t1 t7 ys_ordered))
    expected_ordered = Refl
```

### stdlib/time-and-events/window-merge-distribution-law

- spec: `spec/50-stdlib/69-time-and-events.md` §3.1.
- promise class: durable invariant.
- given: `xs_ordered`, `ys_ordered`, and the window `[1,7)`.
- expect: `ordered_distribution_law_instance` applies the public law
  with both explicit `time_ordered` proofs.
- measured: the closed theorem application and both ordering premises.
- claimed: the generic law applies to two time-ordered inputs.
- the gap: this closed Bool instance does not prove the quantified
  theorem for every payload, bound, or trace.
- why: each input has three events, and both nontrivial ordering
  premises are built from their adjacent comparisons.

### stdlib/time-and-events/window-merge-exact-output

- spec: `spec/50-stdlib/69-time-and-events.md` §3.1.
- promise class: durable invariant.
- given: the same ordered traces and `[1,7)`.
- expect: both sides reduce to `[3a,3b,5b,6a]`; the window drops
  one event from each input and keeps the cross-input tie in left-first
  order.
- measured: the exact closed list values from `window_of_merge_exact_value`
  and `merge_of_windows_exact_value`.
- claimed: the structural distribution result and its left-biased tie
  are observable on this fixture.
- the gap: this fixture is not a generic proof and does not test every
  tied input.
- why: different payloads at time 3 distinguish which input's event
  appears first.

## Unordered-trace controls

The frame's unordered-left case uses `[5a,1a]`, `[3b]` and `[0,4)`.
Its outputs are `[3b,1a]` on the window-of-merge side and `[1a,3b]`
on the merge-of-windows side. The swapped case keeps the left input
ordered and makes only the right input unordered; its outputs are
`[3a,1b]` and `[1b,3a]`. These are false list-equality claims, not
refusals of unordered inputs to `trace_merge`.

### stdlib/time-and-events/unordered-window-merge-controls

- spec: `spec/50-stdlib/69-time-and-events.md` §3.1.
- promise class: durable invariant.
- given: the frame's unordered-left pair and its swapped counterpart,
  each with window `[0,4)`.
- expect: this fixture fence is accepted; the four exact output
  controls produce the two differing list pairs above. The opposite
  input in each pair is time-ordered.
- measured: the two singleton ordering proofs and all four `*_is_exact`
  controls in this non-refusing fence.
- claimed: `trace_window` and `trace_merge` accept these unordered
  inputs, and the output order witnesses each counterexample.
- the gap: these finite fixtures do not establish a generic law or
  every unordered counterexample.
- why: each pair shares its events but differs in order, while the
  other input remains ordered.

```ken
const u5a : Event Bool = MkEvent Bool t5 True
const u1a : Event Bool = MkEvent Bool t1 False
const u3b : Event Bool = MkEvent Bool t3 True
const u3a : Event Bool = MkEvent Bool t3 False
const u5b : Event Bool = MkEvent Bool t5 False
const u1b : Event Bool = MkEvent Bool t1 True

const unordered_left : List (Event Bool) =
  Cons (Event Bool) u5a (Cons (Event Bool) u1a (Nil (Event Bool)))
const ordered_right : List (Event Bool) =
  Cons (Event Bool) u3b (Nil (Event Bool))
const left_window_of_merge : List (Event Bool) =
  Cons (Event Bool) u3b (Cons (Event Bool) u1a (Nil (Event Bool)))
const left_merge_of_windows : List (Event Bool) =
  Cons (Event Bool) u1a (Cons (Event Bool) u3b (Nil (Event Bool)))

const ordered_left : List (Event Bool) =
  Cons (Event Bool) u3a (Nil (Event Bool))
const unordered_right : List (Event Bool) =
  Cons (Event Bool) u5b (Cons (Event Bool) u1b (Nil (Event Bool)))
const right_window_of_merge : List (Event Bool) =
  Cons (Event Bool) u3a (Cons (Event Bool) u1b (Nil (Event Bool)))
const right_merge_of_windows : List (Event Bool) =
  Cons (Event Bool) u1b (Cons (Event Bool) u3a (Nil (Event Bool)))

theorem ordered_right_is_time_ordered : time_ordered Bool ordered_right = Proved
theorem ordered_left_is_time_ordered : time_ordered Bool ordered_left = Proved

theorem left_window_of_merge_is_exact :
  Equal (List (Event Bool))
    (trace_window Bool t0 t4 (trace_merge Bool unordered_left ordered_right))
    left_window_of_merge = Refl

theorem left_merge_of_windows_is_exact :
  Equal (List (Event Bool))
    (trace_merge Bool
      (trace_window Bool t0 t4 unordered_left)
      (trace_window Bool t0 t4 ordered_right))
    left_merge_of_windows = Refl

theorem right_window_of_merge_is_exact :
  Equal (List (Event Bool))
    (trace_window Bool t0 t4 (trace_merge Bool ordered_left unordered_right))
    right_window_of_merge = Refl

theorem right_merge_of_windows_is_exact :
  Equal (List (Event Bool))
    (trace_merge Bool
      (trace_window Bool t0 t4 ordered_left)
      (trace_window Bool t0 t4 unordered_right))
    right_merge_of_windows = Refl
```

## Refused one-sided-order claims

The ordered-positive fence above and the unordered-fixture fence form
one common client when concatenated in that order. Once the §3 catalog
export exists, that client is expected to accept. For each refusal,
start from a fresh copy of this base client and append only that
refusal's fence. The appended client is expected to reject. Never
combine the two refusals: the first rejection would mask whether the
other target is independently refused. This seed has not been run.

### stdlib/time-and-events/reject-left-unordered-window-merge

- spec: `spec/50-stdlib/69-time-and-events.md` §3.1.
- promise class: durable invariant.
- given: unordered `xs=[5a,1a]`, ordered `ys=[3b]`, and `[0,4)`.
- expect: the common client accepts; appending only the left-unordered
  false equality claim is refused.
- measured: the accepted fixture fence contains both left exact-output
  controls; this refusal fence contains only the false `Equal` claim.
- claimed: the law cannot be weakened to require only the right input
  to be ordered.
- the gap: one finite payload fixture does not establish every such
  counterexample or the generic law.
- why: the outputs share events but disagree in order.

```ken
-- Refused: the left input is not time-ordered.
theorem refuse_left_unordered_distribution :
  Equal (List (Event Bool))
    (trace_window Bool t0 t4 (trace_merge Bool unordered_left ordered_right))
    (trace_merge Bool
      (trace_window Bool t0 t4 unordered_left)
      (trace_window Bool t0 t4 ordered_right)) = Proved
```

### stdlib/time-and-events/reject-right-unordered-window-merge

- spec: `spec/50-stdlib/69-time-and-events.md` §3.1.
- promise class: durable invariant.
- given: ordered `xs=[3a]`, unordered `ys=[5b,1b]`, and `[0,4)`.
- expect: the common client accepts; appending only the right-unordered
  false equality claim is refused.
- measured: the accepted fixture fence contains both right exact-output
  controls; this refusal fence contains only the false `Equal` claim.
- claimed: the law cannot be weakened to require only the left input
  to be ordered.
- the gap: one finite payload fixture does not establish every such
  counterexample or the generic law.
- why: the outputs share events but disagree in order.

```ken
-- Refused: the right input is not time-ordered.
theorem refuse_right_unordered_distribution :
  Equal (List (Event Bool))
    (trace_window Bool t0 t4 (trace_merge Bool ordered_left unordered_right))
    (trace_merge Bool
      (trace_window Bool t0 t4 ordered_left)
      (trace_window Bool t0 t4 unordered_right)) = Proved
```
