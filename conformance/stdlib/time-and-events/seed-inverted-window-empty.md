# Equal and inverted windows select no events — seed cases

Format: `../../README.md`.
Spec: `spec/50-stdlib/69-time-and-events.md` §9.
Frame: `docs/program/issues/SPEC-TIME-INVERTED-WINDOW-EMPTY.md`.

The public law applies to every trace when `hi ≤ lo`, including equal
bounds and unordered traces. These closed instances exercise the public
interface and distinguish an empty window from `[4,5)`, which retains
the event at time 4. The instances do not prove the generic law.

The Architect's AC-0 and the Foundation package-test result are separate
evidence; neither is execution of this CV seed. This source-shaped seed
has not been run by Ken or the official conformance runner.

## Accepted law instances and value controls

The ordered trace is `tr = [e1,e4,e7]`, with events at times 1, 4,
and 7. The same three events are rearranged as `unordered` so the law
is also exercised without an ordering premise. The window `[4,5)` is
nonempty and contains exactly `e4`.

```ken
import Core.Classes.LawfulClasses (leq_nat)
import Data.Time.EventTrace
  (Event, MkEvent, event_before, trace_window,
   trace_window_inverted_empty)

theorem generic_empty_window
      (p : Type) (lo : Nat) (hi : Nat) (events : List (Event p))
    : Equal Bool (leq_nat hi lo) True
      → Equal
        (List (Event p))
        (trace_window p lo hi events)
        (Nil (Event p)) =
  trace_window_inverted_empty p lo hi events

const t0 : Nat = Zero
const t1 : Nat = Suc t0
const t2 : Nat = Suc t1
const t3 : Nat = Suc t2
const t4 : Nat = Suc t3
const t5 : Nat = Suc t4
const t6 : Nat = Suc t5
const t7 : Nat = Suc t6

const e1 : Event Bool = MkEvent Bool t1 True
const e4 : Event Bool = MkEvent Bool t4 False
const e7 : Event Bool = MkEvent Bool t7 True
const tr : List (Event Bool) =
  Cons (Event Bool) e1
    (Cons (Event Bool) e4 (Cons (Event Bool) e7 (Nil (Event Bool))))
const unordered : List (Event Bool) =
  Cons (Event Bool) e7
    (Cons (Event Bool) e1 (Cons (Event Bool) e4 (Nil (Event Bool))))
const only4 : List (Event Bool) = Cons (Event Bool) e4 (Nil (Event Bool))

theorem equal_bounds_law :
  Equal (List (Event Bool)) (trace_window Bool t4 t4 tr) (Nil (Event Bool)) =
  trace_window_inverted_empty Bool t4 t4 tr Proved

theorem inverted_bounds_law :
  Equal (List (Event Bool)) (trace_window Bool t7 t4 tr) (Nil (Event Bool)) =
  trace_window_inverted_empty Bool t7 t4 tr Proved

theorem unordered_inverted_law :
  Equal (List (Event Bool))
    (trace_window Bool t7 t4 unordered) (Nil (Event Bool)) =
  trace_window_inverted_empty Bool t7 t4 unordered Proved

theorem equal_bounds_value :
  Equal (List (Event Bool)) (trace_window Bool t4 t4 tr) (Nil (Event Bool)) = Proved

theorem inverted_bounds_value :
  Equal (List (Event Bool)) (trace_window Bool t7 t4 tr) (Nil (Event Bool)) = Proved

theorem unordered_pair_is_out_of_order :
  Equal Bool (event_before Bool e7 e1) False = Proved

theorem unordered_inverted_value :
  Equal (List (Event Bool))
    (trace_window Bool t7 t4 unordered) (Nil (Event Bool)) = Proved

theorem nonempty_window_has_false_inversion_premise :
  Equal Bool (leq_nat t5 t4) False = Proved

theorem nonempty_window_exact_value :
  Equal (List (Event Bool)) (trace_window Bool t4 t5 tr) only4 = Refl
```

### stdlib/time-and-events/inverted-window-empty

- spec: `spec/50-stdlib/69-time-and-events.md` §§1.1–1.2, §9.
- promise class: durable invariant.
- given: the accepted generic client and closed fixture above, with the
  public `trace_window_inverted_empty` law.
- expect: the open client accepts with only the `hi ≤ lo` premise. The
  law applies at `[4,4)` and `[7,4)` and on the unordered
  permutation at `[7,4)`. The direct values agree: each inverted or
  equal-bounds window is `Nil`; `[4,5)` is exactly `[e4]`.
- measured: an open generic client with one premise, three closed law
  applications, three direct empty-window values, an out-of-order
  adjacent pair, the false `leq_nat 5 4` result, and the exact
  nonempty `[4,5)` result.
- claimed: `trace_window` returns `Nil` for any trace when `hi ≤ lo`,
  including equality, and the public theorem has no ordering premise.
- gap: the open client and finite Bool-payload instances do not
  independently establish the generic proof body or cover every bound
  and trace.
- why: `[4,4)` is controlled by the same `Proved` form as the false
  `[4,5)` Nil claim below; the `[4,5)` value control shows that `e4`
  is retained. The out-of-order pair makes the unordered instance
  non-degenerate.

## Refused nonempty-window Nil claim

In a fresh environment, accept the client above first. Then submit the
following declaration alone. The same `Proved` form accepts the true
`[4,4)` Nil claim, but `[4,5)` retains `e4`.

```ken
-- Refuse: [4,5) contains e4, so it is not Nil.
theorem refuse_nonempty_window_as_nil :
  Equal (List (Event Bool))
    (trace_window Bool t4 t5 tr) (Nil (Event Bool)) = Proved
```

### stdlib/time-and-events/reject-nonempty-window-as-empty

- spec: `spec/50-stdlib/69-time-and-events.md` §§1.1–1.2, §9.
- promise class: durable invariant.
- given: a fresh accepted prefix containing the fixture, the true
  `[4,4)` Nil value, and the `[4,5)` exact `[e4]` control; then the
  isolated false declaration above.
- expect: the false `[4,5)`-equals-`Nil` claim is refused.
- measured: an accepted same-form empty-window claim and an accepted
  nonempty exact-list result precede the isolated refusal.
- claimed: a nonempty half-open window cannot be claimed equal to `Nil`.
- gap: this one false value does not establish refusal for every
  nonempty interval or trace.
- why: the accepted `[4,5)` result is `[e4]`; the false claim uses
  `Proved`, not `Refl`, so it cannot refuse because `Refl` is the
  wrong proof form for an empty-list equality.

## Refused law application without its premise

In another fresh environment, accept the client above first. The
`leq_nat 5 4 = False` control confirms that `[4,5)` does not satisfy
the law's sole premise. Submit this declaration alone:

```ken
-- Refuse: leq_nat t5 t4 is False, not True.
theorem refuse_empty_law_without_bound_premise :
  Equal (List (Event Bool))
    (trace_window Bool t4 t5 tr) (Nil (Event Bool)) =
  trace_window_inverted_empty Bool t4 t5 tr Proved
```

### stdlib/time-and-events/reject-empty-law-without-inverted-premise

- spec: `spec/50-stdlib/69-time-and-events.md` §9.
- promise class: durable invariant.
- given: a fresh accepted prefix containing the valid law
  applications, the false bound control, and the nonempty `[4,5)`
  exact-list control; then the isolated law application above.
- expect: the law application is refused because its required premise
  `Equal Bool (leq_nat t5 t4) True` is false.
- measured: the same law accepts with valid `Proved` premises at
  `[4,4)` and `[7,4)`; the exact `[4,5)` value is `[e4]`, and
  `leq_nat t5 t4` is `False`.
- claimed: the public law cannot discharge an empty-window result
  outside its sole `hi ≤ lo` premise.
- gap: this one false premise does not establish behavior for every
  invalid bound pair.
- why: the accepted law instances use the same public theorem and
  proof form; the explicit false bound and nonempty output identify
  why this application must refuse.

## Evidence boundary

Architect's AC-0 is scratch proof-development evidence. The Foundation
package target is a separate package test, not execution of this CV seed.
This file records source-shaped input and expected behavior only. No Ken
check, package-test execution, official conformance run, CI, publication,
or landing is claimed for this seed.
