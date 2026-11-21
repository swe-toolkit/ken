# Distinct-time merge commutativity

Format: `../../README.md`.
Spec: `spec/50-stdlib/69-time-and-events.md` §20.
Frame: `docs/program/issues/SPEC-TIME-MERGE-DISTINCT-TIMES-COMMUTE.md`.

This seed is an authored expectation for the §20 public law. The
clients explicitly import every package-home name they use. No seed
execution, official conformance run, CI, publication, landing, or
trust measurement is claimed.

The positive and refused clients use the same payloads, left trace,
imports, premise form, and law application. Their input difference is
only the timestamp of `b`: time 2 in the positive client and time 3
in the refused client. The explicit merge outputs are separate
controls, not conclusions inferred from applying the law.

## Accepted distinct timestamps

The accepted client has `left = [a@1, c@3]` and `right = [b@2]`.
Each pair across the two traces has distinct timestamps. The
pairwise premise is discharged from the two finite `list_elem`
proofs; the public law is applied separately. Each merge is also
checked independently against the same explicit list.

```ken
import Core.Logic.Transport (sym)

import Core.Logic.Or (Or, Inl, Inr)

import Core.Classes.LawfulClasses (bool_or)

import Data.Numeric.Nat.Order (lt_nat)

import Data.Collections.Derived (list_elem, list_elem_head, list_elem_later)

import Data.Time.EventTrace
  (Event, MkEvent, event_time, trace_merge, trace_merge_distinct_commute)

fn distinct_times (x : Event Bool) (y : Event Bool) : Bool =
  bool_or
    (lt_nat (event_time Bool x) (event_time Bool y))
    (lt_nat (event_time Bool y) (event_time Bool x))

theorem distinct_at
      (x : Event Bool) (a : Event Bool) (y : Event Bool) (b : Event Bool)
    : Equal (Event Bool) x a
      → Equal (Event Bool) y b
      → Equal Bool (distinct_times a b) True
      → Equal Bool (distinct_times x y) True =
  λx_is_a.
    λy_is_b.
      λbase.
        J
          (λz _. Equal Bool (distinct_times z y) True)
          (J (λw _. Equal Bool (distinct_times a w) True) base (sym (Event Bool) y b y_is_b))
          (sym (Event Bool) x a x_is_a)

const one : Nat = Suc Zero

const two : Nat = Suc one

const three : Nat = Suc two

const a : Event Bool = MkEvent Bool one True

const c : Event Bool = MkEvent Bool three False

const b : Event Bool = MkEvent Bool two True

const left : List (Event Bool) = Cons (Event Bool) a (Cons (Event Bool) c (Nil (Event Bool)))

const right : List (Event Bool) = Cons (Event Bool) b (Nil (Event Bool))

const merged : List (Event Bool) =
  Cons (Event Bool) a (Cons (Event Bool) b (Cons (Event Bool) c (Nil (Event Bool))))

theorem merge_left_output : Equal (List (Event Bool)) (trace_merge Bool left right) merged =
  Refl

theorem merge_right_output : Equal (List (Event Bool)) (trace_merge Bool right left) merged =
  Refl

theorem left_right_distinct
    : (x : Event Bool)
      → list_elem (Event Bool) x left
      → (y : Event Bool)
      → list_elem (Event Bool) y right
      → Equal Bool (distinct_times x y) True =
  λx.
    λin_left.
      λy.
        λin_right.
          elim_trunc
            (Equal Bool (distinct_times x y) True)
            (λhead_case.
              match head_case {
                Inl x_is_a ↦
                  elim_trunc
                    (Equal Bool (distinct_times x y) True)
                    (λright_case.
                      match right_case {
                        Inl y_is_b ↦ distinct_at x a y b x_is_a y_is_b Proved;
                        Inr none ↦ absurd none
                      })
                    in_right;
                Inr later ↦
                  elim_trunc
                    (Equal Bool (distinct_times x y) True)
                    (λtail_case.
                      match tail_case {
                        Inl x_is_c ↦
                          elim_trunc
                            (Equal Bool (distinct_times x y) True)
                            (λright_case.
                              match right_case {
                                Inl y_is_b ↦ distinct_at x c y b x_is_c y_is_b Proved;
                                Inr none ↦ absurd none
                              })
                            in_right;
                        Inr none ↦ absurd none
                      })
                    later
              })
            in_left

theorem commute_on_distinct_times
    : Equal (List (Event Bool)) (trace_merge Bool left right) (trace_merge Bool right left) =
  trace_merge_distinct_commute Bool left right left_right_distinct
```

### stdlib/time-and-events/merge-distinct-times-commute

- spec: `spec/50-stdlib/69-time-and-events.md §20`;
  `trace_merge_distinct_commute`
- promise class: **durable invariant** — traces with no cross-input
  timestamp tie merge to the same ordered list in either argument order
- given: the accepted client above, with left `[a@1, c@3]` and right
  `[b@2]`; its source imports and pairwise premise are complete
- expect: both independent merge equalities reduce by `Refl` to
  `[a@1, b@2, c@3]`; the public law application accepts after the
  pairwise premise is discharged
- measured: two `Refl` output calculations and one public law
  application over the same fixture
- claimed: the public law applies with only the pairwise distinct-time
  premise; this source supplies no `time_ordered` evidence. This closed
  instance agrees with the generic structural equality of §20
- gap: the finite `Bool` fixture does not establish the theorem's
  quantification over all payload types or all traces; the checked
  catalog proof owns that obligation
- controls: every external name in the client is imported from its
  home; the two independent `Refl` checks do not use the public law
- why: the explicit list is an oracle independent of the law
  application, and its order includes the cross-input event `b`
  between `a` and `c`

## Refuse a shared cross-input timestamp

Start from a fresh copy of the accepted client. Change only the input
value of `b`, from time 2 to time 3, preserving its `True` payload.
Retain all imports, the pairwise-premise proof, and the law application
unchanged. Replace `merged` and the two accepted merge-output
assertions with the tied-output declarations below; these output
oracles change to reflect the changed input. The complete copied
prefix, including both output assertions, must check before the
pairwise-premise refusal is submitted.

The shared-time outputs differ only in the order of the two events
at time 3. This seed observes that list order; it does not assert a
separate equality or inequality of the event values.

```ken
const b : Event Bool = MkEvent Bool three True

const tied_left : List (Event Bool) =
  Cons (Event Bool) a (Cons (Event Bool) c (Cons (Event Bool) b (Nil (Event Bool))))

const tied_right : List (Event Bool) =
  Cons (Event Bool) a (Cons (Event Bool) b (Cons (Event Bool) c (Nil (Event Bool))))

theorem tied_merge_left_output
    : Equal (List (Event Bool)) (trace_merge Bool left right) tied_left =
  Refl

theorem tied_merge_right_output
    : Equal (List (Event Bool)) (trace_merge Bool right left) tied_right =
  Refl
```

After those tied-output assertions, retain the same
`left_right_distinct` premise proof and `commute_on_distinct_times`
law application from the accepted client. The premise proof must
refuse at its `Proved` for `c@3` against `b@3`, before the law
application. The two tied-output controls must remain accepted in the
complete prefix.

### stdlib/time-and-events/reject-shared-time-merge-commute

- spec: `spec/50-stdlib/69-time-and-events.md §20`;
  `trace_merge_distinct_commute`
- promise class: **durable invariant** — a shared timestamp does not
  satisfy the law's pairwise distinct-time premise
- given: a fresh copy of the accepted client, changing only `b`'s
  timestamp to 3; retain the same imports, payload, pairwise proof,
  and law application. Assert the two tied outputs independently.
- expect: both tied merge outputs accept by `Refl`, respectively as
  `[a@1, c@3, b@3]` and `[a@1, b@3, c@3]`. The
  `left_right_distinct` premise proof refuses at the shared-time
  `Proved`; do not attribute the refusal to an import or to the law
  application.
- measured: two independent tied-output calculations, followed by
  refusal while discharging the pairwise premise
- claimed: the shared-time pair is excluded by the condition that one
  timestamp is strictly earlier than the other
- gap: the single refused premise instance does not prove that every
  possible shared-time pair is excluded; §20's type and checked proof
  state the generic law
- controls: the accepted different-time fixture has the same proof
  form and law application. The tied outputs show that both merge
  calls themselves remain usable and expose the left-biased order.
- why: with `c@3` on the left and `b@3` on the right,
  `bool_or (lt_nat 3 3) (lt_nat 3 3)` is false; the refusal is
  therefore localized to the property under test

The accepted fence and refused source variation are authored
expectations, not execution observations. Their displayed merge
results are exact expected values.
No official conformance-runner execution, CI, publication, landing,
or trust measurement is claimed.
