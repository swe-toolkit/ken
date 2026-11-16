# Windowing a merged list of traces

Format: `../../README.md`.
Spec: `spec/50-stdlib/69-time-and-events.md` §17.
Frame: `docs/program/issues/SPEC-TIME-WINDOW-MERGE-ALL.md`.

The public law requires every trace in the outer list to be ordered.
W-GEN is an open, direct public application. W-CLOSED checks both
sides of an ordered three-trace instance by reduction and fixes its
closed output; W-LAW applies the public theorem with the explicit
all-members premise. Neither closed reduction establishes the generic
proof. X-FIRST and X-LAST separately make only the first or the last
trace unordered; each false equality must refuse after four accepted
value controls. A one-trace case does not require orderedness because
merging it with `Nil` gives the same trace; that observation does not
weaken the exported theorem's uniform premise.

## Open public-client control

Check this client independently. It imports and applies the named
public theorem rather than restating the law as a closed reduction.

```ken
import Data.Collections.Derived (list_elem, map)
import Data.Time.EventTrace
  (Event, time_ordered, trace_window, trace_merge_all, trace_window_merge_all)

theorem window_merge_all_open
    (p : Type) (lo : Nat) (hi : Nat) (traces : List (List (Event p)))
    : ((t : List (Event p)) → list_elem (List (Event p)) t traces → time_ordered p t)
      → Equal
        (List (Event p))
        (trace_window p lo hi (trace_merge_all p traces))
        (trace_merge_all p
          (map (List (Event p)) (List (Event p))
            (trace_window p lo hi) traces)) =
  trace_window_merge_all p lo hi traces
```

### stdlib/time-and-events/window-merge-all-open

- spec: §17, `trace_window_merge_all`.
- promise class: durable invariant.
- given: an arbitrary payload, bounds and finite outer trace list.
- expect: W-GEN accepts through a direct import and application of the
  public checked theorem.
- measured: public resolution and an open application at the exact
  quantified premise and conclusion type.
- claimed: callers can reuse the checked theorem without a new
  assumption or narrower premise.
- gap: an imported theorem's availability is not a separate proof of
  its body; the package checks the generic body.
- why: unlike closed `Refl` cases, this client cannot establish its
  goal by computing a finite fixture.

## Accepted three-trace client

Check the entire client. Its window `[2,6)` retains events at times
2, 3 and 4, including both distinct payloads at time 4. Both sides
are `[2T,3T,4T,4F]`, not a one-trace identity. The explicit premise
transports each of the three ordering proofs along outer-list
membership; it does not assume an unproved ordering predicate.

```ken
import Data.Time.EventTrace
  (Event, MkEvent, event_before, time_ordered, trace_window, trace_merge_all,
    trace_window_merge_all)
import Data.Collections.Derived (list_elem, map)
import Core.Logic.Transport (sym, trans)
import Core.Logic.Conjunction (and_intro)
import Core.Logic.Or (Or, Inl, Inr)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const five : Nat = Suc four
const six : Nat = Suc five
const seven : Nat = Suc six

const first_trace : List (Event Bool) =
  Cons (Event Bool) (MkEvent Bool two True)
    (Cons (Event Bool) (MkEvent Bool four True) (Nil (Event Bool)))
const second_trace : List (Event Bool) =
  Cons (Event Bool) (MkEvent Bool one False)
    (Cons (Event Bool) (MkEvent Bool four False) (Nil (Event Bool)))
const third_trace : List (Event Bool) =
  Cons (Event Bool) (MkEvent Bool three True)
    (Cons (Event Bool) (MkEvent Bool seven False) (Nil (Event Bool)))
const traces : List (List (Event Bool)) =
  Cons (List (Event Bool)) first_trace
    (Cons (List (Event Bool)) second_trace
      (Cons (List (Event Bool)) third_trace (Nil (List (Event Bool)))))
const windowed_merge_expected : List (Event Bool) =
  Cons (Event Bool) (MkEvent Bool two True)
    (Cons (Event Bool) (MkEvent Bool three True)
      (Cons (Event Bool) (MkEvent Bool four True)
        (Cons (Event Bool) (MkEvent Bool four False) (Nil (Event Bool)))))

-- W-CLOSED: the two closed sides agree and the merged window has this value.
theorem w_closed_distributes
    : Equal
      (List (Event Bool))
      (trace_window Bool two six (trace_merge_all Bool traces))
      (trace_merge_all Bool
        (map (List (Event Bool)) (List (Event Bool))
          (trace_window Bool two six) traces)) =
  Refl

theorem w_closed_value
    : Equal (List (Event Bool))
        (trace_window Bool two six (trace_merge_all Bool traces))
        windowed_merge_expected =
  Refl

theorem ordered_first : time_ordered Bool first_trace =
  and_intro
    (Equal Bool (event_before Bool (MkEvent Bool two True) (MkEvent Bool four True)) True)
    (time_ordered Bool (Cons (Event Bool) (MkEvent Bool four True) (Nil (Event Bool))))
    Proved Proved

theorem ordered_second : time_ordered Bool second_trace =
  and_intro
    (Equal Bool (event_before Bool (MkEvent Bool one False) (MkEvent Bool four False)) True)
    (time_ordered Bool (Cons (Event Bool) (MkEvent Bool four False) (Nil (Event Bool))))
    Proved Proved

theorem ordered_third : time_ordered Bool third_trace =
  and_intro
    (Equal Bool (event_before Bool (MkEvent Bool three True) (MkEvent Bool seven False)) True)
    (time_ordered Bool (Cons (Event Bool) (MkEvent Bool seven False) (Nil (Event Bool))))
    Proved Proved

theorem every_trace_ordered (t : List (Event Bool))
    : list_elem (List (Event Bool)) t traces → time_ordered Bool t =
  λmember.
    elim_trunc
      (time_ordered Bool t)
      (λcase_first.
        match case_first {
          Inl same_first ↦
            J (λu _. time_ordered Bool u) ordered_first
              (sym (List (Event Bool)) t first_trace same_first);
          Inr later_first ↦
            elim_trunc
              (time_ordered Bool t)
              (λcase_second.
                match case_second {
                  Inl same_second ↦
                    J (λu _. time_ordered Bool u) ordered_second
                      (sym (List (Event Bool)) t second_trace same_second);
                  Inr later_second ↦
                    elim_trunc
                      (time_ordered Bool t)
                      (λcase_third.
                        match case_third {
                          Inl same_third ↦
                            J (λu _. time_ordered Bool u) ordered_third
                              (sym (List (Event Bool)) t third_trace same_third);
                          Inr later_third ↦ absurd later_third
                        })
                      later_second
                })
              later_first
        })
      member

-- W-LAW: apply the public theorem with all three explicit premises.
theorem w_law_windowed_merge
    : Equal (List (Event Bool))
        (trace_window Bool two six (trace_merge_all Bool traces))
        windowed_merge_expected =
  trans
    (List (Event Bool))
    (trace_window Bool two six (trace_merge_all Bool traces))
    (trace_merge_all Bool
      (map (List (Event Bool)) (List (Event Bool))
        (trace_window Bool two six) traces))
    windowed_merge_expected
    (trace_window_merge_all Bool two six traces every_trace_ordered)
    Refl
```

### stdlib/time-and-events/window-merge-all-closed

- spec: §17 and §15's right-fold merge order.
- promise class: durable invariant.
- given: three ordered traces and window `[2,6)`.
- expect: W-CLOSED accepts both the distribution equality and the
  independently specified four-event result.
- measured: two `Refl` assertions with exact closed expressions and
  a separately authored expected list.
- claimed: this fixture's merge-window value and left-biased time-4
  tie match the public behavior.
- gap: these finite reductions do not prove a generic law or a
  different bound interval.
- why: both time-4 events survive with distinguishable Bool payloads;
  a merge that drops one or swaps their order changes the list value.

### stdlib/time-and-events/window-merge-all-law-instance

- spec: §17, `trace_window_merge_all`.
- promise class: durable invariant.
- given: `ordered_first`, `ordered_second`, `ordered_third` and
  `every_trace_ordered` on the same outer list.
- expect: W-LAW accepts by applying the public theorem, followed by
  the independently checked closed result.
- measured: the three explicit ordering proofs, membership-indexed
  all-traces premise and public law application.
- claimed: the public law composes with a proved ordered three-trace
  input and the observed result.
- gap: an instance still does not establish the generic proof body.
- why: all three positions require evidence, not merely the head.

## One-sided-order controls and isolated refusals

For each refusal, check a fresh copy of the common accepted prefix
below, then append only its corresponding negative fence. The prefix
must accept **all four** `x_*` value theorems. X-FIRST makes the first
trace unordered and leaves the last singleton ordered; X-LAST makes
the last unordered and leaves the first singleton ordered. In both
cases the window is `[0,4)`. `trace_merge_all` is defined on both
inputs: its window is `[3F,1T]`, but merging the individually windowed
traces gives `[1T,3F]`. Refusal is at the false equality's own `Refl`,
not a failed ordering proof, missing import or rejection of the input.

```ken
import Data.Time.EventTrace (Event, MkEvent, trace_window, trace_merge_all)
import Data.Collections.Derived (map)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const five : Nat = Suc four

const unordered_trace : List (Event Bool) =
  Cons (Event Bool) (MkEvent Bool five True)
    (Cons (Event Bool) (MkEvent Bool one True) (Nil (Event Bool)))
const ordered_trace : List (Event Bool) =
  Cons (Event Bool) (MkEvent Bool three False) (Nil (Event Bool))
const unordered_first : List (List (Event Bool)) =
  Cons (List (Event Bool)) unordered_trace
    (Cons (List (Event Bool)) ordered_trace (Nil (List (Event Bool))))
const unordered_last : List (List (Event Bool)) =
  Cons (List (Event Bool)) ordered_trace
    (Cons (List (Event Bool)) unordered_trace (Nil (List (Event Bool))))
const window_keeps_merge_order : List (Event Bool) =
  Cons (Event Bool) (MkEvent Bool three False)
    (Cons (Event Bool) (MkEvent Bool one True) (Nil (Event Bool)))
const merge_of_windows : List (Event Bool) =
  Cons (Event Bool) (MkEvent Bool one True)
    (Cons (Event Bool) (MkEvent Bool three False) (Nil (Event Bool)))

theorem x_first_window_of_merge
    : Equal (List (Event Bool))
        (trace_window Bool Zero four (trace_merge_all Bool unordered_first))
        window_keeps_merge_order =
  Refl

theorem x_first_merge_of_windows
    : Equal (List (Event Bool))
        (trace_merge_all Bool
          (map (List (Event Bool)) (List (Event Bool))
            (trace_window Bool Zero four) unordered_first))
        merge_of_windows =
  Refl

theorem x_last_window_of_merge
    : Equal (List (Event Bool))
        (trace_window Bool Zero four (trace_merge_all Bool unordered_last))
        window_keeps_merge_order =
  Refl

theorem x_last_merge_of_windows
    : Equal (List (Event Bool))
        (trace_merge_all Bool
          (map (List (Event Bool)) (List (Event Bool))
            (trace_window Bool Zero four) unordered_last))
        merge_of_windows =
  Refl
```

### stdlib/time-and-events/reject-window-merge-all-first-unordered

- spec: §17, necessity of orderedness for every member.
- promise class: durable invariant.
- given: `[5T,1T]` first, `[3F]` last and `[0,4)`.
- expect: X-FIRST refuses at `x_first_distributes`' own `Refl`
  after the common prefix, in a fresh client.
- measured: both exact-value controls for this input accept using
  `Refl`; its false structural equality compares those distinct lists.
- claimed: requiring only later input traces to be ordered is unsound.
- gap: one closed counterexample does not prove all possible
  weakening forms false or verify an arbitrary mutated implementation.
- controls: `x_first_window_of_merge` and
  `x_first_merge_of_windows` check the same expressions independently.
- why: the two closed outputs contain the same events in different
  orders, so the equality cannot hold.

```ken
theorem x_first_distributes
    : Equal (List (Event Bool))
        (trace_window Bool Zero four (trace_merge_all Bool unordered_first))
        (trace_merge_all Bool
          (map (List (Event Bool)) (List (Event Bool))
            (trace_window Bool Zero four) unordered_first)) =
  Refl
```

### stdlib/time-and-events/reject-window-merge-all-last-unordered

- spec: §17, necessity of orderedness for every member.
- promise class: durable invariant.
- given: `[3F]` first, `[5T,1T]` last and `[0,4)`.
- expect: X-LAST refuses at `x_last_distributes`' own `Refl`
  after a fresh copy of the common prefix.
- measured: both exact-value controls for this input accept with
  `Refl`; its false equality compares their distinct output lists.
- claimed: requiring only the first input trace to be ordered is
  unsound.
- gap: this closed refusal does not prove the generic law or
  characterize every failure to satisfy the all-members premise.
- controls: `x_last_window_of_merge` and `x_last_merge_of_windows`.
- why: both expressions are defined and disagree only in output order.

```ken
theorem x_last_distributes
    : Equal (List (Event Bool))
        (trace_window Bool Zero four (trace_merge_all Bool unordered_last))
        (trace_merge_all Bool
          (map (List (Event Bool)) (List (Event Bool))
            (trace_window Bool Zero four) unordered_last)) =
  Refl
```

## Evidence boundary

Architect's AC-0 ruling `evt_2n80r8c2db1m4` covers a detached scratch
D0 run by Foundation Implementer at `fd523a842465c0bc185140167143ad457d51bc37`.
W-GEN, W-CLOSED and W-LAW accepted in scratch. Each false claim was
checked on a fresh prefix after all four value controls and refused
at its own `Refl` with `ElabError::TypeMismatch` and the message
“Refl expects an `Eq`-shaped goal”. Architect corrected its earlier
prediction of `KernelRejected(TypeMismatch)`; that is **not** the
observed diagnostic for these `Refl` rows. Scratch trusted GlobalId
sets were equal 114 → 114 under the same provider-first load,
EventTrace-owned declarations 87 → 88, and actual public roots-loader
heads 35 → 36. Those D0 observations are not execution of these
newly authored fences, official conformance, an assembled-tip gate,
CI, publication, landing or measured landed trust. No external or
`local/refs` material was used.
