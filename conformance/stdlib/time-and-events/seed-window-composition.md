# Window composition and empty merge units — seed cases

Format: `../../README.md`.
Spec: `spec/50-stdlib/69-time-and-events.md` §5.

These closed clients pin the three checked public law interfaces and
independent exact-list controls. A closed instance does not establish a
generic law. At the candidate base, §5's catalog exports have not
landed; scratch D0 results do not establish execution against shipped
exports or by the official conformance runner.

## Accepted client

The trace contains `e1`, `e4`, and `e7` in that order, at times 1, 4,
and 7. The inner window `[1,6)` keeps `e1,e4`; the outer window
`[3,9)` keeps `e4,e7` on the original trace but only `e4` on the
inner result. Their intersected bounds are `[3,6)`, which keeps
exactly `e4`. Union bounds `[1,9)` instead keep all three events.
The time-1 and time-7 events each belong to **only one** of the two
original windows, so they discriminate intersection from union.

```ken
import Data.Numeric.Nat.Order (max, min)
import Data.Time.EventTrace
  (Event, MkEvent, trace_window, trace_merge, trace_window_window,
   trace_merge_nil_left, trace_merge_nil_right)

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
  Cons (Event Bool) e1
    (Cons (Event Bool) e4
      (Cons (Event Bool) e7 (Nil (Event Bool))))
const only4 : List (Event Bool) =
  Cons (Event Bool) e4 (Nil (Event Bool))
const inner_value : List (Event Bool) =
  Cons (Event Bool) e1 (Cons (Event Bool) e4 (Nil (Event Bool)))
const outer_value : List (Event Bool) =
  Cons (Event Bool) e4 (Cons (Event Bool) e7 (Nil (Event Bool)))

theorem inner_keeps_first_only :
  Equal (List (Event Bool)) (trace_window Bool t1 t6 tr) inner_value = Refl

theorem outer_keeps_last_only :
  Equal (List (Event Bool)) (trace_window Bool t3 t9 tr) outer_value = Refl

theorem nested_window_law_instance :
  Equal (List (Event Bool))
    (trace_window Bool t3 t9 (trace_window Bool t1 t6 tr))
    (trace_window Bool (max t1 t3) (min t6 t9) tr) =
  trace_window_window Bool t1 t6 t3 t9 tr

theorem nested_window_exact_value :
  Equal (List (Event Bool))
    (trace_window Bool t3 t9 (trace_window Bool t1 t6 tr))
    only4 = Refl

theorem intersection_window_exact_value :
  Equal (List (Event Bool))
    (trace_window Bool (max t1 t3) (min t6 t9) tr)
    only4 = Refl

theorem union_window_exact_value :
  Equal (List (Event Bool))
    (trace_window Bool (min t1 t3) (max t6 t9) tr)
    tr = Refl

theorem nil_left_law_instance :
  Equal (List (Event Bool))
    (trace_merge Bool (Nil (Event Bool)) tr) tr =
  trace_merge_nil_left Bool tr

theorem nil_right_law_instance :
  Equal (List (Event Bool))
    (trace_merge Bool tr (Nil (Event Bool))) tr =
  trace_merge_nil_right Bool tr
```

### stdlib/time-and-events/window-of-window-intersection

- spec: `spec/50-stdlib/69-time-and-events.md` §5.
- promise class: durable invariant.
- given: the same trace under `[1,6)` then `[3,9)`.
- expect: the application of `trace_window_window` accepts and both
  sides independently reduce to the exact one-event list `[e4]`.
- measured: the public law application, two separate closed exact
  results, and first/last-only input-window controls.
- claimed: nested windows on any trace equal a single window using
  the maximum lower bound and minimum upper bound, with no ordering
  premise.
- gap: a single ordered Bool-payload trace does not establish the
  quantified law for other traces, payloads, or disjoint bounds.
- why: separate direct values prevent the law application from being
  its own expected-result oracle; `e1` and `e7` isolate the distinction
  between the two input windows.

### stdlib/time-and-events/merge-empty-units

- spec: `spec/50-stdlib/69-time-and-events.md` §5.
- promise class: durable invariant.
- given: the nonempty three-event `tr` and an empty `List (Event Bool)`
  on each side of `trace_merge` in turn.
- expect: both public laws are applied and return `tr` structurally.
- measured: `nil_left_law_instance` and `nil_right_law_instance` on
  that same nonempty trace, independently of the window law.
- claimed: both empty-trace units hold for every list and payload.
- gap: these closed applications do not prove the generic bodies or
  that open right-unit equality reduces by `Refl` (it does not).
- why: the right-unit application needs the promoted checked theorem;
  the existing §1.2 merge operation is not modified by this seed.

## Refused union-bounds claim

### stdlib/time-and-events/reject-window-union-bounds

- spec: `spec/50-stdlib/69-time-and-events.md` §5.
- promise class: durable invariant.
- given: a fresh copy of the **entire accepted client above** and
  the *same* `[1,6)`/`[3,9)` pair and three-event trace.
- expect: append only the single false proposition below and reject
  at its theorem declaration, not at an import or earlier fixture.
- measured: the accepted prefix pins the nested window to `[e4]`,
  while its exact union-window control yields `[e1,e4,e7]`; the
  appended equality with union bounds must be refused.
- claimed: intersected bounds, not union bounds, characterize nested
  windows on this trace.
- gap: one refusal cannot prove the generic law or preclude every
  other incorrect bounds formula. It does not demonstrate an
  ordering-premise boundary: no such premise occurs in §5.
- why: the accepted prefix uses `Refl` on two true closed list
  equalities with the same nested expression, so a refusal cannot
  be attributed merely to the proof form or to an unbound name.

```ken
-- Refuse: the union interval retains e1 and e7, but the nested one does not.
theorem refuse_union_bounds :
  Equal (List (Event Bool))
    (trace_window Bool t3 t9 (trace_window Bool t1 t6 tr))
    (trace_window Bool (min t1 t3) (max t6 t9) tr) = Refl
```

## Evidence boundary

Architect's separate D0 at `98970bde7` checked all three generic
proof bodies, their ownership and Transparent status, six private
import refusals, and scratch trust identity sets 114 → 114 without
a new `Axiom`. Its own closed clients accepted `[e4]` on both sides
and both empty-merge units, and refused the union-bounds claim.
Those are checks of an isolated proposal, not execution against
shipped §5 exports. The authored fences were also checked by the
author in an isolated worktree on `98970bde7`, with the Architect's
catalog-only D0 patch (SHA-256
`2d66f7957f9a2b826ce9cc59778ba82a1c240200564ff40343ebd22c78292a18`)
and the locally built D0-scratch CLI (SHA-256 at this check:
`3a5307f8f414f5f3427bbb1fa6b4467288a9dc413df1235d2a2a606131422938`).
The full eight-theorem accepted client returned rc 0. A **fresh copy**
with only the union-bounds refusal appended returned rc 1/E0038
`TypeMismatch`: `Refl expects an Eq-shaped goal` at the appended
`Refl` bytes 2399–2403, inside `refuse_union_bounds` starting at
byte 2229. The same accepted prefix checks `Refl` on both true
closed nested/intersection list equalities, so the refusal is tied
to the false union result rather than an import or a broken form.
These D0-overlay checks establish neither shipped §5 exports nor
official conformance execution, CI, publication, landing or landed
trust.
