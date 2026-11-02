# Window commutation — seed cases

Format: `../../README.md`.
Spec: `spec/50-stdlib/69-time-and-events.md` §11.

The cases below are closed clients for the public `trace_window_commute`
law and independent exact-list controls. A closed instance does not
establish the law's generic quantifiers.

The Architect's AC-0 reports that its scratch clients accepted the
same-form commutation control and refused the dropped-inner-window claim
at `Refl` (rc 0 and rc 1, respectively). Those D0 scratch observations
are separate from Foundation's later assembled-working-tree package test.
Foundation reports that `cat_time_window_commutation` passed 2/2 on the
actual seed fences, including the accepted block and isolated refusal, and
measured the full `trusted_base()` GlobalId identity set as 114→114. This
is Foundation's assembled-working-tree package-test evidence, not an
exact committed-tip gate or official conformance-runner run. CV ran no
tests; AC-0 did not measure the identity set. These results do not
establish CI, publication, landing, or a landed trust measurement.

## Accepted client

The trace contains `e1`, `e4`, and `e7` in that order, at times 1, 4,
and 7. The windows `[1,6)` and `[3,9)` overlap and are not nested:
`e1` belongs only to the first, `e7` only to the second, and `e4` to
both. Applying either window after the other keeps only `e4`. Applying
`[3,9)` to the original trace keeps `e4,e7`.

```ken
import Data.Time.EventTrace
  (Event, MkEvent, trace_window, trace_window_commute)

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
const only4and7 : List (Event Bool) =
  Cons (Event Bool) e4
    (Cons (Event Bool) e7 (Nil (Event Bool)))

theorem window_commutation_law_instance :
  Equal (List (Event Bool))
    (trace_window Bool t3 t9 (trace_window Bool t1 t6 tr))
    (trace_window Bool t1 t6 (trace_window Bool t3 t9 tr)) =
  trace_window_commute Bool t1 t6 t3 t9 tr

theorem first_then_second_exact_value :
  Equal (List (Event Bool))
    (trace_window Bool t3 t9 (trace_window Bool t1 t6 tr))
    only4 = Refl

theorem second_then_first_exact_value :
  Equal (List (Event Bool))
    (trace_window Bool t1 t6 (trace_window Bool t3 t9 tr))
    only4 = Refl

theorem second_window_alone_exact_value :
  Equal (List (Event Bool)) (trace_window Bool t3 t9 tr) only4and7 = Refl

theorem same_form_commutation_control :
  Equal (List (Event Bool))
    (trace_window Bool t3 t9 (trace_window Bool t1 t6 tr))
    (trace_window Bool t1 t6 (trace_window Bool t3 t9 tr)) = Refl
```

### stdlib/time-and-events/window-commutation

- spec: `spec/50-stdlib/69-time-and-events.md` §11.
- promise class: durable invariant.
- given: the same trace under overlapping, non-nested windows `[1,6)`
  and `[3,9)`.
- expect: the public law instance accepts; both orders independently
  reduce to `[e4]`; `[3,9)` alone reduces to `[e4,e7]`; the same-form
  commutation control accepts by `Refl`.
- measured: the law application, both exact nested-window values, the
  single-window value, and the same-form control.
- claimed: applying these two windows in either order yields the same
  list for every payload type, bound pair, and trace, without an
  ordering premise.
- gap: this fixed `Bool` trace and these bounds do not establish the
  generic quantifiers or the checked proof body.
- why: `e1` and `e7` each occur in only one input window, while `e4`
  occurs in both. The exact values are independent of the law instance.

## Refused dropped-inner-window claim

Use a fresh copy of the complete accepted client above. Append only the
following declaration. It compares the nested result to the outer
window applied to the original trace, which retains `e7`.

```ken
-- Refuse: e7 survives the outer window alone, not the inner window.
theorem refuse_dropping_inner_window :
  Equal (List (Event Bool))
    (trace_window Bool t3 t9 (trace_window Bool t1 t6 tr))
    (trace_window Bool t3 t9 tr) = Refl
```

### stdlib/time-and-events/reject-dropped-inner-window

- spec: `spec/50-stdlib/69-time-and-events.md` §11.
- promise class: durable invariant.
- given: a fresh copy of the accepted client and the same trace and
  windows.
- expect: reject `refuse_dropping_inner_window` at its `Refl` with
  `E0038` (`Refl expects an Eq-shaped goal`); the accepted prefix and
  its same-form control pass.
- measured: the nested result is `[e4]`, the outer window alone is
  `[e4,e7]`, and the same nested expression equals the swapped
  composition by `Refl`.
- claimed: the outer window alone is not interchangeable with the
  composition that first applies the inner window.
- gap: one closed refusal does not establish the generic law or rule
  out every other incorrect transformation.
- why: the refusal and accepted control share the left-hand side,
  equality form, trace, windows, and `Refl` proof form. Only the
  right-hand side changes; `e7` makes the two results differ.

## Evidence boundary

Architect AC-0 measured the same-form commutation control at rc 0 and
the dropped-inner-window claim at rc 1, with `E0038` at `Refl`. Those
scratch clients ground the expected behavior but are not execution of
this seed. Foundation later reports that its assembled working tree
contains the separate §11 Spec and catalog law and that its named
package test checked the actual seed fences 2/2 with GlobalId identities
114→114. This CV child contains no Spec or catalog edits; the Foundation
result is working-tree evidence, not a committed-tip gate or official
conformance-runner execution. CV ran no tests. No CI, publication,
landing, or landed trust measurement is claimed.
