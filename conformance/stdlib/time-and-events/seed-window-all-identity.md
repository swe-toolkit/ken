# A window containing every event leaves the trace unchanged

Format: `../../README.md`.
Spec: `spec/50-stdlib/69-time-and-events.md` §10.
Frame: `docs/program/issues/SPEC-TIME-WINDOW-ALL-IDENTITY.md`.

The accepted client uses `tr=[(4,True),(6,False),(5,True)]` over
`Bool`. At `[4,8)`, it closes the premise for every list member,
applies the generic law, and independently checks the same list
identity by `Refl`. At `[4,6)`, it checks the exact filtered trace
`[(4,True),(5,True)]`; the isolated refusal claims that this shorter
window leaves `tr` unchanged. The two identity claims have the same
proof form and trace; only the upper bound changes. These closed
instances exercise the contract but do not prove the generic law.

## Accepted client

Check the entire client before appending the sole refused claim below.
`measured` describes the intended oracle, not a report that this seed
was executed.

```ken
import Core.Logic.Transport (cong, trans)
import Core.Logic.Or (Or, Inl, Inr)
import Data.Collections.Derived (list_elem)
import Data.Time.EventTrace
  (Event, MkEvent, event_in_window, trace_window,
   trace_window_all_identity)

const t0 : Nat = Zero
const t1 : Nat = Suc t0
const t2 : Nat = Suc t1
const t3 : Nat = Suc t2
const t4 : Nat = Suc t3
const t5 : Nat = Suc t4
const t6 : Nat = Suc t5
const t7 : Nat = Suc t6
const t8 : Nat = Suc t7

const e4 : Event Bool = MkEvent Bool t4 True
const e6 : Event Bool = MkEvent Bool t6 False
const e5 : Event Bool = MkEvent Bool t5 True
const tr : List (Event Bool) =
  Cons (Event Bool) e4
    (Cons (Event Bool) e6
      (Cons (Event Bool) e5 (Nil (Event Bool))))

theorem every_member_is_in_4_8 :
  (e : Event Bool) → list_elem (Event Bool) e tr →
    Equal Bool (event_in_window Bool t4 t8 e) True =
  λe. λm0.
    elim_trunc
      (Equal Bool (event_in_window Bool t4 t8 e) True)
      (λc0. match c0 {
        Inl s0 ↦
          trans Bool (event_in_window Bool t4 t8 e)
            (event_in_window Bool t4 t8 e4) True
            (cong (Event Bool) Bool e e4
              (event_in_window Bool t4 t8) s0)
            Proved;
        Inr m1 ↦
          elim_trunc
            (Equal Bool (event_in_window Bool t4 t8 e) True)
            (λc1. match c1 {
              Inl s1 ↦
                trans Bool (event_in_window Bool t4 t8 e)
                  (event_in_window Bool t4 t8 e6) True
                  (cong (Event Bool) Bool e e6
                    (event_in_window Bool t4 t8) s1)
                  Proved;
              Inr m2 ↦
                elim_trunc
                  (Equal Bool (event_in_window Bool t4 t8 e) True)
                  (λc2. match c2 {
                    Inl s2 ↦
                      trans Bool (event_in_window Bool t4 t8 e)
                        (event_in_window Bool t4 t8 e5) True
                        (cong (Event Bool) Bool e e5
                          (event_in_window Bool t4 t8) s2)
                        Proved;
                    Inr m3 ↦ absurd m3
                  })
                  m2
            })
            m1
      })
      m0

theorem identity_by_public_law :
  Equal (List (Event Bool)) (trace_window Bool t4 t8 tr) tr =
  trace_window_all_identity Bool t4 t8 tr every_member_is_in_4_8

theorem identity_by_refl_control :
  Equal (List (Event Bool)) (trace_window Bool t4 t8 tr) tr = Refl

theorem shorter_window_value :
  Equal (List (Event Bool)) (trace_window Bool t4 t6 tr)
    (Cons (Event Bool) e4 (Cons (Event Bool) e5 (Nil (Event Bool)))) = Refl
```

### stdlib/time-and-events/window-all-identity

- spec: §10 and §2.1.
- promise class: durable invariant.
- given: `tr=[(4,True),(6,False),(5,True)]` and the windows
  `[4,8)` and `[4,6)`.
- expect: every member is shown inside `[4,8)`; the public law and
  same-form `Refl` control accept; `[4,6)` yields `[(4,True),(5,True)]`.
- measured: the accepted source contains a closed proof of the full
  membership premise, one public-law application, and direct closed
  list results; this is not a seed execution report.
- claimed: a window containing every member returns the original
  trace with its order and occurrences unchanged.
- gap: one trace and its closed premise do not prove the generic law
  for every payload, window, or trace.
- maintenance: extensions preserving half-open filtering and the
  all-members premise must leave this instance green; contract changes
  require review of the law and seed.
- why: the law application tests the public interface; the independent
  `Refl` and exact shorter-window value are controls over the same trace.

## Isolated refused shorter-window identity

On a fresh complete copy of the accepted client, append only the
following theorem. The `[4,8)` identity is the accepted same-form
control, and `shorter_window_value` shows the result for `[4,6)`.
The shorter identity must refuse at `Refl`; no diagnostic variant or
message is pinned.

### stdlib/time-and-events/reject-window-all-identity-at-upper-bound

- spec: §10 and §2.1.
- promise class: durable invariant.
- given: the same trace as the accepted client, with upper bound
  reduced from `8` to `6`.
- expect: the claim that `[4,6)` leaves the trace unchanged is
  refused at `Refl`.
- measured: the accepted client checks `[4,6)` as
  `[(4,True),(5,True)]`, not `tr`; this seed itself is unexecuted.
- claimed: an event at the exclusive upper boundary is removed, so a
  window that omits a source member cannot be asserted to return the
  full trace.
- gap: this counterexample does not establish the generic law or
  characterize every trace that a window filters.
- maintenance: preserving the half-open upper boundary and the
  trace-identity law must keep this false equality refused.
- why: the same-form control accepts at `[4,8)`; `[4,6)` excludes
  only the time-6 event and retains the later time-5 event.

```ken
theorem refuse_shorter_identity :
  Equal (List (Event Bool)) (trace_window Bool t4 t6 tr) tr = Refl
```

## Evidence boundary

Architect's D0 checked the generic §10 law and its proof route; the
Foundation package test checks a closed all-in-window law instance and
an isolated shorter-window refusal on its fixture. Neither is execution
of this seed. This seed is authored and not Ken-checked or officially
conformance-run. No official conformance-runner execution, CI,
publication, landing, or landed-trust measurement is claimed.
