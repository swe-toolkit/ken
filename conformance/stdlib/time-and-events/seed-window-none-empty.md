# A window with no events in it is empty — seed cases

Format: `../../README.md`.
Spec: `spec/50-stdlib/69-time-and-events.md` §14.
Frame: `docs/program/issues/SPEC-TIME-WINDOW-NONE-EMPTY.md`.

The public law states that if every member of a trace misses a window, then
filtering the trace by that window returns `Nil`. It needs no ordering or bound
premise. These cases use the trace `[e1, e5]`, with events at times 1 and 5.
The accepted window `[2, 5)` contains neither event. The refused window
`[1, 5)` contains `e1` at its included lower bound.

`list_elem` is propositionally truncated. The client helper eliminates that
membership only into `Equal Bool ... : Omega`, as permitted by `16 §6`. It
splits the two-member trace into named `e1_misses` and `e5_misses` obligations;
the refused case therefore points directly at the false obligation for `e1`.

## Closed fixture and membership premise

```ken
import Core.Logic.Or (Or, Inl, Inr)
import Core.Logic.Transport (cong, trans)
import Data.Collections.Derived (list_elem)
import Data.Time.EventTrace
  (Event, MkEvent, event_in_window, trace_window,
   trace_window_none_empty)

const zero : Nat = Zero
const one : Nat = Suc zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const five : Nat = Suc four

const e1 : Event Bool = MkEvent Bool one True
const e5 : Event Bool = MkEvent Bool five False
const tr : List (Event Bool) =
  Cons (Event Bool) e1
    (Cons (Event Bool) e5 (Nil (Event Bool)))

theorem two_member_misses
      (p : Type) (lo : Nat) (hi : Nat)
      (first : Event p) (last : Event p)
      (first_misses : Equal Bool (event_in_window p lo hi first) False)
      (last_misses : Equal Bool (event_in_window p lo hi last) False)
    : (e : Event p) →
      list_elem (Event p) e
        (Cons (Event p) first (Cons (Event p) last (Nil (Event p)))) →
      Equal Bool (event_in_window p lo hi e) False =
  λe. λmember.
    elim_trunc
      (Equal Bool (event_in_window p lo hi e) False)
      (λhead_or_tail.
        match head_or_tail {
          Inl same =>
            trans
              Bool
              (event_in_window p lo hi e)
              (event_in_window p lo hi first)
              False
              (cong
                (Event p)
                Bool
                e
                first
                (λx. event_in_window p lo hi x)
                same)
              first_misses;
          Inr later =>
            elim_trunc
              (Equal Bool (event_in_window p lo hi e) False)
              (λtail_or_empty.
                match tail_or_empty {
                  Inl same =>
                    trans
                      Bool
                      (event_in_window p lo hi e)
                      (event_in_window p lo hi last)
                      False
                      (cong
                        (Event p)
                        Bool
                        e
                        last
                        (λx. event_in_window p lo hi x)
                        same)
                      last_misses;
                  Inr impossible => absurd impossible
                })
              later
        })
      member
```

## Accepted: neither member lies in `[2, 5)`

```ken
theorem e1_misses_two_five :
  Equal Bool (event_in_window Bool two five e1) False = Proved

theorem e5_misses_two_five :
  Equal Bool (event_in_window Bool two five e5) False = Proved

theorem every_member_misses_two_five :
  (e : Event Bool) →
  list_elem (Event Bool) e tr →
  Equal Bool (event_in_window Bool two five e) False =
  two_member_misses Bool two five e1 e5
    e1_misses_two_five e5_misses_two_five

theorem accepted_empty_by_public_law :
  Equal (List (Event Bool))
    (trace_window Bool two five tr) (Nil (Event Bool)) =
  trace_window_none_empty Bool two five tr every_member_misses_two_five

theorem accepted_empty_value :
  Equal (List (Event Bool))
    (trace_window Bool two five tr) (Nil (Event Bool)) = Proved
```

### stdlib/time-and-events/window-none-empty

- spec: `spec/50-stdlib/69-time-and-events.md` §14.
- given: the closed trace, `[2, 5)`, and the two per-member miss proofs.
- expect: the public law accepts and concludes that the filtered trace is
  `Nil`; the direct value check is also accepted.
- why: time 1 is below the inclusive lower bound 2, and time 5 is excluded by
  the strict upper bound.
- evidence status: CV did not run this source-shaped case. AC-0 scratch
  evidence concerns the generic law and separate closed probes.
- claimed: a non-inverted window that contains no trace member selects `Nil`.
- the gap: this finite trace does not establish the theorem for all traces,
  events or bounds.

## Refused: `e1` lies in `[1, 5)`

This row keeps the same trace, public law, helper and proof form. Only the
lower bound changes from 2 to 1. The in-window control confirms that `e1` is
inside; the invalid per-member miss is named `e1_misses_one_five`. The expected
refusal is at that premise obligation, before the law application can receive
the required function.

```ken
theorem e1_is_in_one_five :
  Equal Bool (event_in_window Bool one five e1) True = Proved

theorem e5_misses_one_five :
  Equal Bool (event_in_window Bool one five e5) False = Proved

theorem e1_misses_one_five :
  Equal Bool (event_in_window Bool one five e1) False = Proved

theorem every_member_misses_one_five :
  (e : Event Bool) →
  list_elem (Event Bool) e tr →
  Equal Bool (event_in_window Bool one five e) False =
  two_member_misses Bool one five e1 e5
    e1_misses_one_five e5_misses_one_five

theorem refuse_nonempty_window_by_public_law :
  Equal (List (Event Bool))
    (trace_window Bool one five tr) (Nil (Event Bool)) =
  trace_window_none_empty Bool one five tr every_member_misses_one_five
```

### stdlib/time-and-events/reject-window-with-member

- spec: `spec/50-stdlib/69-time-and-events.md` §§1–2, §14.
- given: the closed trace and attempted law application at `[1, 5)`.
- expect: the `e1_misses_one_five` obligation is refused; `e1_is_in_one_five`
  and `e5_misses_one_five` are accepted controls.
- why: `[1, 5)` includes the event at time 1, so the universal miss premise is
  false for that member. The `[2, 5)` application above controls the law form.
- expected locus: the false premise is `e1_misses_one_five`; the other
  member's miss and the lower-bound inclusion are separate controls. CV did not
  run this source-shaped case.
- claimed: the law cannot conclude empty output when a trace member is inside
  the window.
- the gap: this refusal covers one in-window member, not every false miss
  premise or nonempty trace/window pair.

## Evidence boundary

The Architect's AC-0 report is scratch evidence for the generic law and its
closed probes. This seed is authored source-shaped conformance material; CV did
not run it or the official conformance runner. No package-test, CI, publication,
landing or landed trust claim is made here.
