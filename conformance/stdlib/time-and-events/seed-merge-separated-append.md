# Separated trace merges are append

Format: `../../README.md`.
Spec: `spec/50-stdlib/69-time-and-events.md` §13.
Frame: `docs/program/issues/SPEC-TIME-MERGE-SEPARATED-APPEND.md`.

The accepted client splits one closed ordered trace at `mid` and uses
`trace_merge_separated` to show that merging the adjacent windows gives the
whole window. A separate same-form `Refl` control checks a closed separated
pair. The isolated refusal claims the append equation for interleaved inputs,
where merge reorders an event. These closed cases exercise the public law but
do not establish its generic quantifiers.

## Accepted client

Check this complete client before appending the isolated refusal below. The
`windows_join` proof derives the separation premise from membership in the two
adjacent windows, then composes the law with the public window-split law.

```ken
import Core.Logic.Conjunction (And, and_fst, and_snd, and_intro)

import Core.Logic.Transport (sym, trans)

import Core.Classes.LawfulClasses (leq_nat)

import Data.Numeric.Nat.Order (lt_nat)

import Data.Collections.Derived (list_append, list_elem)

import Data.Time.EventTrace
  (Event,
    MkEvent,
    event_time,
    event_before,
    time_ordered,
    trace_window,
    trace_merge,
    trace_window_split,
    trace_window_member_sound,
    trace_merge_separated)

theorem windows_join
      (p : Type) (lo : Nat) (mid : Nat) (hi : Nat) (tr : List (Event p))
    : time_ordered p tr
      → Equal Bool (leq_nat lo mid) True
      → Equal Bool (leq_nat mid hi) True
      → Equal
        (List (Event p))
        (trace_merge p (trace_window p lo mid tr) (trace_window p mid hi tr))
        (trace_window p lo hi tr) =
  λordered.
    λlo_mid.
      λmid_hi.
        trans
          (List (Event p))
          (trace_merge p (trace_window p lo mid tr) (trace_window p mid hi tr))
          (list_append (Event p) (trace_window p lo mid tr) (trace_window p mid hi tr))
          (trace_window p lo hi tr)
          (trace_merge_separated
            p
            (trace_window p lo mid tr)
            (trace_window p mid hi tr)
            (λx.
              λlow.
                λy.
                  λhigh.
                    (proof trans for leq_nat)
                      (event_time p x)
                      mid
                      (event_time p y)
                      ((proof true_leq for lt_nat)
                        (event_time p x)
                        mid
                        (and_snd
                          (Equal Bool (leq_nat lo (event_time p x)) True)
                          (Equal Bool (lt_nat (event_time p x) mid) True)
                          (and_snd
                            (list_elem (Event p) x tr)
                            (And
                              (Equal Bool (leq_nat lo (event_time p x)) True)
                              (Equal Bool (lt_nat (event_time p x) mid) True))
                            (trace_window_member_sound p lo mid tr x low))))
                      (and_fst
                        (Equal Bool (leq_nat mid (event_time p y)) True)
                        (Equal Bool (lt_nat (event_time p y) hi) True)
                        (and_snd
                          (list_elem (Event p) y tr)
                          (And
                            (Equal Bool (leq_nat mid (event_time p y)) True)
                            (Equal Bool (lt_nat (event_time p y) hi) True))
                          (trace_window_member_sound p mid hi tr y high)))))
          (sym
            (List (Event p))
            (trace_window p lo hi tr)
            (list_append (Event p) (trace_window p lo mid tr) (trace_window p mid hi tr))
            (trace_window_split p lo mid hi tr ordered lo_mid mid_hi))

theorem g_separated
      (p : Type) (xs : List (Event p)) (ys : List (Event p))
    : ((x : Event p)
        → list_elem
        (Event p)
        x
        xs
        → (y : Event p)
        → list_elem
        (Event p)
        y
        ys
        → Equal
        Bool
        (event_before p x y)
        True)
      → Equal (List (Event p)) (trace_merge p xs ys) (list_append (Event p) xs ys) =
  trace_merge_separated p xs ys

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

theorem closed_join
    : Equal
        (List (Event Bool))
        (trace_merge Bool (trace_window Bool t0 t4 tr) (trace_window Bool t4 t9 tr))
        (trace_window Bool t0 t9 tr) =
  windows_join Bool t0 t4 t9 tr tr_ordered Proved Proved

const sep_left : List (Event Bool) =
  Cons (Event Bool) e1 (Cons (Event Bool) e3 (Nil (Event Bool)))

const sep_right : List (Event Bool) = Cons (Event Bool) e7 (Nil (Event Bool))

theorem separated_control
    : Equal
        (List (Event Bool))
        (trace_merge Bool sep_left sep_right)
        (list_append (Event Bool) sep_left sep_right) =
  Refl
```

### stdlib/time-and-events/merge-separated-append

- spec: §13 and §7; `trace_merge_separated` and `trace_window_split`.
- promise class: durable invariant.
- given: ordered trace `[e1,e3,e7]` at times 1, 3, and 7, split at 4.
- expect: the law application equates the merge of the adjacent windows
  with the whole window; the separated closed pair also reduces to append.
- measured: Architect AC-0 reports the generic accepted client and its
  closed instance accepted on scratch D0.
- claimed: whenever every event in the left trace is at or before every
  event in the right trace, left-biased merge equals list append.
- gap: this ordered closed instance does not establish the generic law
  for arbitrary payloads, traces, or separation proofs.
- why: window membership supplies the left-before-mid and mid-before-right
  facts; the public law composes with the split on the same ordered trace.

## Isolated refused interleaved append claim

On a fresh complete copy of the accepted client, append only the false claim
below. Its inputs interleave: the left trace contains times 1 and 7, while the
right trace contains time 3. Merge places `e3` before `e7`; append does not.
The separated control above has the same equality form and `Refl` proof form.

### stdlib/time-and-events/reject-interleaved-merge-append

- spec: §13 and §1.2; `trace_merge_separated` and left-biased merge.
- promise class: durable invariant.
- given: left trace `[e1,e7]` and right trace `[e3]` at times 1, 7,
  and 3.
- expect: the direct false append equality is refused at its own `Refl`.
- measured: Architect AC-0 reports the interleaved `Refl` claim refused
  as E0038; the separated same-form control accepts.
- claimed: the generic append law cannot be applied when the two traces
  interleave.
- gap: this one closed refusal does not establish the universal
  separation condition or the generic theorem.
- why: the inputs change only the cross-trace ordering relative to the
  accepted separated control; the claim and proof forms stay fixed.

```ken
-- Refuse: merge reorders e3 before e7, unlike append.
const interleaved_left : List (Event Bool) =
  Cons (Event Bool) e1 (Cons (Event Bool) e7 (Nil (Event Bool)))

const interleaved_right : List (Event Bool) = Cons (Event Bool) e3 (Nil (Event Bool))

theorem refuse_interleaved_merge_is_append
    : Equal
        (List (Event Bool))
        (trace_merge Bool interleaved_left interleaved_right)
        (list_append (Event Bool) interleaved_left interleaved_right) =
  Refl
```

## Evidence boundary

Architect AC-0 ruling `evt_7nw2d96tvqzv1` reports the generic law and the
`windows_join` client checked on scratch, with the ordered separated instance
accepted and the interleaved `Refl` claim refused. Those are Architect-reported
D0 results, not execution of this CV seed or of an assembled candidate. CV
performed source review and formatting checks only; it did not run `ken check`,
package tests, or the official conformance runner. No CI, publication, landing,
or landed-trust result is claimed. CV consulted no `local/refs` or external
reference source.
