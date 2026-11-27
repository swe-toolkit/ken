# Sorting and windowing commute

Format: `../../README.md`.
Spec: `spec/50-stdlib/69-time-and-events.md` §24.
Architect AC-0: `evt_1pd1r18qcewg`.

This seed is authored and unexecuted. No claim is made that the official
conformance runner has executed it. Its discriminator is the accepted
open-trace client: the public law commutes sorting with windowing for
an arbitrary trace. The closed example is usage-only; it also computes
without applying the law.

Windowing keeps event times in `[lo, hi)`. The law compares filtering
a sorted trace with sorting its filtered result.

## Accepted clients

Use this complete prefix for the accepted cases and as the starting
point for the isolated refusal. The open witness is the discriminator.
The closed row with times `[3, 1, 5]` in `[1, 4)` is usage-only.

```ken
import Core.Logic.Transport (trans)
import Data.Collections.Derived (sort)
import Data.Time.EventTrace
  (Event, MkEvent, event_before, trace_window, trace_window_sort)

const one : Nat = Suc Zero
const four : Nat = Suc (Suc (Suc one))

theorem sorted_window_open (tr : List (Event Bool))
    : Equal
        (List (Event Bool))
        (trace_window Bool one four (sort (Event Bool) (event_before Bool) tr))
        (sort (Event Bool) (event_before Bool) (trace_window Bool one four tr)) =
  trace_window_sort Bool one four tr

const three : Nat = Suc (Suc one)
const five : Nat = Suc four
const e3 : Event Bool = MkEvent Bool three True
const e1 : Event Bool = MkEvent Bool one False
const e5 : Event Bool = MkEvent Bool five True
const unsorted : List (Event Bool) =
  Cons (Event Bool) e3
    (Cons (Event Bool) e1 (Cons (Event Bool) e5 (Nil (Event Bool))))

theorem closed_by_law
    : Equal
        (List (Event Bool))
        (trace_window Bool one four
          (sort (Event Bool) (event_before Bool) unsorted))
        (Cons (Event Bool) e1
          (Cons (Event Bool) e3 (Nil (Event Bool)))) =
  trans
    (List (Event Bool))
    (trace_window Bool one four
      (sort (Event Bool) (event_before Bool) unsorted))
    (sort (Event Bool) (event_before Bool) (trace_window Bool one four unsorted))
    (Cons (Event Bool) e1
      (Cons (Event Bool) e3 (Nil (Event Bool))))
    (trace_window_sort Bool one four unsorted)
    Refl
```

### stdlib/time-and-events/trace-window-sort-commute-open

- spec: §24, `trace_window_sort`.
- promise class: durable invariant.
- given: an arbitrary trace and window `[1, 4)`.
- expect: `sorted_window_open` accepts by applying the public law.
- measured: its body cites `trace_window_sort` at open `tr`.
- claimed: filtering after sorting gives the same trace as sorting after
  filtering.
- gap: the client uses the law; it does not establish its generic body.
- controls: the same open statement closed by `Refl` or `Proved` refuses
  in D0; these are package-test controls, not seed rows.
- why: the free trace keeps the law's quantified input open.

### stdlib/time-and-events/trace-window-sort-commute-closed-usage

- spec: §24, `trace_window_sort`.
- promise class: durable invariant.
- given: times `[3, 1, 5]` and window `[1, 4)`.
- expect: `closed_by_law` accepts with result `[1, 3]`.
- measured: the proof applies the law, then computes the expected list.
- claimed: the public theorem can be used on this closed trace.
- gap: this does not establish the generic law or discriminate it; D0
  measured the same client accepted by `Refl` alone.
- controls: `sorted_window_open` is the law-use discriminator.
- why: sorting and filtering both yield the two events at times 1 and 3.

## Isolated refused claim

Start from a fresh complete copy of the accepted client above and append
only the refusal. Require `KernelRejected(TypeMismatch)` within its own
theorem span. Do not pin a token span or exact diagnostic wording.

### stdlib/time-and-events/reject-window-before-sort

- spec: §24, `trace_window_sort`.
- promise class: durable invariant.
- given: a fresh accepted prefix and an arbitrary trace.
- expect: the appended `sorted_window_is_unsorted` refuses inside its
  theorem with `KernelRejected(TypeMismatch)`.
- measured: the claim omits sorting the trace after windowing.
- claimed: windowing a sorted trace equals windowing the original trace.
- gap: this refusal does not establish the law's generic body.
- controls: the accepted `sorted_window_open` keeps sorting on both sides;
  the refusal has its own fresh accepted prefix.
- why: for times `[3, 1]` in `[1, 4)`, sorting before filtering yields
  `[1, 3]`, while filtering without sorting leaves `[3, 1]`.

Append only:

```ken
theorem sorted_window_is_unsorted (tr : List (Event Bool))
    : Equal
        (List (Event Bool))
        (trace_window Bool one four (sort (Event Bool) (event_before Bool) tr))
        (trace_window Bool one four tr) =
  trace_window_sort Bool one four tr
```

## Evidence boundary

Architect AC-0 measured the generic law and client forms in scratch on
base `d7e8ca4b0`. It measured the open client accepted, the same open
statement by `Refl` and `Proved` refused, the unsorted-window twin
refused with `KernelRejected(TypeMismatch)`, and the closed usage row
accepted both by the law and by `Refl` alone. Those checks do not
execute this Markdown seed. No official conformance run, assembled
AC-T, CI, publication, or landing claim is made.
