# Finite-list trace merge — source-shaped seed

Format: `../../README.md`.
Spec: `spec/50-stdlib/69-time-and-events.md` §15.
Frame: `docs/program/issues/SPEC-TIME-TRACE-MERGE-ALL.md`.

The accepted client supplies three ordered Bool-payload traces. Their
six events have times `[0,5]`, `[2,7]` and `[5,9]`; the two events
at time 5 have different payloads. A separate expected list tests
the right fold's value and left-biased tie, rather than deriving the
expected result from a law that uses the same operation. The client
also applies the ordered and complete laws, including an event from
the third trace, and uses the sound law to locate it in the flattened
input. These finite rows do not replace the generic checked proofs.

## Accepted three-trace client

```ken
import Data.Time.EventTrace
  (Event, MkEvent, event_before, time_ordered, trace_merge_all,
    trace_merge_all_ordered, trace_merge_all_member_complete,
    trace_merge_all_member_sound)

import Data.Collections.Derived
  (list_elem, list_elem_head, list_elem_later, list_concat)

import Core.Logic.Conjunction (and_intro)
import Core.Logic.Or (Or, Inl, Inr)
import Core.Logic.Transport (sym)

const zero : Nat = Zero
const one : Nat = Suc zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const five : Nat = Suc four
const six : Nat = Suc five
const seven : Nat = Suc six
const eight : Nat = Suc seven
const nine : Nat = Suc eight

const x0 : Event Bool = MkEvent Bool zero True
const x5 : Event Bool = MkEvent Bool five True
const y2 : Event Bool = MkEvent Bool two False
const y7 : Event Bool = MkEvent Bool seven True
const z5 : Event Bool = MkEvent Bool five False
const z9 : Event Bool = MkEvent Bool nine False

const xs : List (Event Bool) =
  Cons (Event Bool) x0 (Cons (Event Bool) x5 (Nil (Event Bool)))
const ys : List (Event Bool) =
  Cons (Event Bool) y2 (Cons (Event Bool) y7 (Nil (Event Bool)))
const zs : List (Event Bool) =
  Cons (Event Bool) z5 (Cons (Event Bool) z9 (Nil (Event Bool)))

const all_three : List (List (Event Bool)) =
  Cons (List (Event Bool)) xs
    (Cons (List (Event Bool)) ys
      (Cons (List (Event Bool)) zs (Nil (List (Event Bool)))))

const expected : List (Event Bool) =
  Cons (Event Bool) x0
    (Cons (Event Bool) y2
      (Cons (Event Bool) x5
        (Cons (Event Bool) z5
          (Cons (Event Bool) y7
            (Cons (Event Bool) z9 (Nil (Event Bool)))))))

theorem empty_list_of_traces_is_empty
    : Equal (List (Event Bool))
        (trace_merge_all Bool (Nil (List (Event Bool))))
        (Nil (Event Bool)) = Proved

theorem exact_three_trace_result
    : Equal (List (Event Bool)) (trace_merge_all Bool all_three) expected =
  Refl

theorem ordered_xs : time_ordered Bool xs =
  and_intro
    (Equal Bool (event_before Bool x0 x5) True)
    (time_ordered Bool (Cons (Event Bool) x5 (Nil (Event Bool))))
    Proved Proved

theorem ordered_ys : time_ordered Bool ys =
  and_intro
    (Equal Bool (event_before Bool y2 y7) True)
    (time_ordered Bool (Cons (Event Bool) y7 (Nil (Event Bool))))
    Proved Proved

theorem ordered_zs : time_ordered Bool zs =
  and_intro
    (Equal Bool (event_before Bool z5 z9) True)
    (time_ordered Bool (Cons (Event Bool) z9 (Nil (Event Bool))))
    Proved Proved

theorem ordered_three_members
      (a : List (Event Bool)) (b : List (Event Bool)) (c : List (Event Bool))
      (oa : time_ordered Bool a) (ob : time_ordered Bool b)
      (oc : time_ordered Bool c) (t : List (Event Bool))
    : list_elem
        (List (Event Bool)) t
        (Cons (List (Event Bool)) a
          (Cons (List (Event Bool)) b
            (Cons (List (Event Bool)) c (Nil (List (Event Bool))))))
      → time_ordered Bool t =
  λmember.
    elim_trunc
      (time_ordered Bool t)
      (λcase.
        match case {
          Inl same_a ↦
            J (λu _. time_ordered Bool u) oa
              (sym (List (Event Bool)) t a same_a);
          Inr later_a ↦
            elim_trunc
              (time_ordered Bool t)
              (λcase_b.
                match case_b {
                  Inl same_b ↦
                    J (λu _. time_ordered Bool u) ob
                      (sym (List (Event Bool)) t b same_b);
                  Inr later_b ↦
                    elim_trunc
                      (time_ordered Bool t)
                      (λcase_c.
                        match case_c {
                          Inl same_c ↦
                            J (λu _. time_ordered Bool u) oc
                              (sym (List (Event Bool)) t c same_c);
                          Inr later_c ↦ absurd later_c
                        })
                      later_b
                })
              later_a
        })
      member

theorem every_input_ordered (t : List (Event Bool))
    : list_elem (List (Event Bool)) t all_three
      → time_ordered Bool t =
  ordered_three_members xs ys zs ordered_xs ordered_ys ordered_zs t

theorem ordered_three_trace_result
    : time_ordered Bool (trace_merge_all Bool all_three) =
  trace_merge_all_ordered Bool all_three every_input_ordered

theorem third_trace_in_inputs
    : list_elem (List (Event Bool)) zs all_three =
  list_elem_later
    (List (Event Bool)) zs xs
    (Cons (List (Event Bool)) ys
      (Cons (List (Event Bool)) zs (Nil (List (Event Bool)))))
    (list_elem_later
      (List (Event Bool)) zs ys
      (Cons (List (Event Bool)) zs (Nil (List (Event Bool))))
      (list_elem_head (List (Event Bool)) zs (Nil (List (Event Bool)))))

theorem third_head_in_trace : list_elem (Event Bool) z5 zs =
  list_elem_head (Event Bool) z5 (Cons (Event Bool) z9 (Nil (Event Bool)))

theorem third_head_in_merged_trace
    : list_elem (Event Bool) z5 (trace_merge_all Bool all_three) =
  trace_merge_all_member_complete
    Bool z5 zs all_three third_trace_in_inputs third_head_in_trace

theorem third_head_in_flattened_inputs
    : list_elem (Event Bool) z5 (list_concat (Event Bool) all_three) =
  trace_merge_all_member_sound Bool z5 all_three third_head_in_merged_trace
```

### stdlib/time-and-events/trace-merge-all-three-ordered

- spec: §15, `trace_merge_all_ordered`.
- given: `xs`, `ys`, `zs`, each proved ordered, and a membership-indexed
  premise for every trace in `all_three`.
- expect: `ordered_three_trace_result` checks by applying the public
  law; the empty-list base also reduces to `Nil`.
- measured: the explicit three sortedness proofs, quantified
  premise, public law application and empty-input computation.
- claimed: sorted input traces give a sorted right-fold result.
- gap: a finite Bool instance does not prove this for arbitrary `p`
  or lists; the provider's generic checked proof does.
- why: the quantified premise requires each of the three actual
  input traces, not only the first one.

### stdlib/time-and-events/trace-merge-all-third-member

- spec: §15, `trace_merge_all_member_complete` and
  `trace_merge_all_member_sound`.
- given: `zs` is third in the outer list, and `z5` is its head.
- expect: the complete law finds `z5` in the merged trace, and the
  sound law finds it in the flattened input.
- measured: both independently constructed membership premises
  and both distinct public law applications.
- claimed: later input traces keep their events, and a merged member
  comes from the flattened inputs.
- gap: this does not prove generic completeness or distinguish two
  identical events from different input occurrences.
- why: an implementation that drops `zs` cannot satisfy the checked
  complete proof with its existing body; the exact-output row below
  separately detects loss without relying on that proof.

### stdlib/time-and-events/trace-merge-all-exact-order

- spec: §15 right fold and §1.2 left-biased `trace_merge`.
- given: the three traces with `x5` and `z5` both at time 5 but with
  distinct payloads.
- expect: `exact_three_trace_result` computes `[x0,y2,x5,z5,y7,z9]`;
  `x5` precedes `z5` at the tie.
- measured: an independently specified six-event list and one
  closed `Refl` equality against `trace_merge_all`.
- claimed: the right fold retains all six events in left-biased order.
- gap: this one closed trace does not characterize arbitrary merges;
  ordered and sound alone also permit a constant-empty operation.
- why: value and tie order distinguish a lawful-looking but wrong
  merge even if its proof gates are removed in diagnostic scratch.

## Isolated refused unordered-input premise

On a fresh complete copy of the accepted client, append only this
fence. The operation is still defined on `ys_unordered = [y7,y2]`;
what fails is the ordering premise needed to apply the public law.
The accepted `ordered_ys` and `ordered_three_trace_result` have the
same proof and law-application forms for `ys = [y2,y7]`. The failed
proof must be `claimed_ordered_ys_unordered` at its `Proved` head
comparison, not an import error or a refusal of the operation. The
following two declarations show the attempted law application but
are not reached once the false premise refuses.

```ken
const ys_unordered : List (Event Bool) =
  Cons (Event Bool) y7 (Cons (Event Bool) y2 (Nil (Event Bool)))

const with_unordered_second : List (List (Event Bool)) =
  Cons (List (Event Bool)) xs
    (Cons (List (Event Bool)) ys_unordered
      (Cons (List (Event Bool)) zs (Nil (List (Event Bool)))))

theorem claimed_ordered_ys_unordered : time_ordered Bool ys_unordered =
  and_intro
    (Equal Bool (event_before Bool y7 y2) True)
    (time_ordered Bool (Cons (Event Bool) y2 (Nil (Event Bool))))
    Proved Proved

theorem claimed_every_input_ordered (t : List (Event Bool))
    : list_elem (List (Event Bool)) t with_unordered_second
      → time_ordered Bool t =
  ordered_three_members xs ys_unordered zs
    ordered_xs claimed_ordered_ys_unordered ordered_zs t

theorem refuse_ordered_merge_with_unordered_second
    : time_ordered Bool (trace_merge_all Bool with_unordered_second) =
  trace_merge_all_ordered Bool with_unordered_second claimed_every_input_ordered
```

### stdlib/time-and-events/reject-trace-merge-all-unordered-input

- spec: §15, the premise of `trace_merge_all_ordered`.
- given: the accepted prefix and the same second trace with its
  two events swapped to `[y7,y2]`.
- expect: the appended `claimed_ordered_ys_unordered` rejects at its
  false `event_before` proof; no claim of an executed law application.
- measured: the accepting twin `ordered_ys` checks with the same
  `and_intro`/`Proved` shape; the refused premise changes only order.
- claimed: the ordered-result theorem does not accept an unordered
  input trace without evidence for every input.
- gap: the rejection of one false premise does not prove a generic
  negative or establish that unordered merges themselves are refused.
- why: the same outer-list shape, public-law call and quantified
  premise helper accept with the ordered twin; only y's head order
  changes, so an incidental import or proof-form failure is excluded.

## Evidence boundary

The Architect's AC-0 scratch checked the generic laws with equal
114-element `trusted_base()` identity sets, a public export delta
30 → 34 and no added `Axiom`. Foundation's implementation child
`2d278b635` separately reports 14 named EventTrace targets passing
35/35; that is candidate-local, not an assembled candidate or a
run of this seed. For an author-local check, the accepted fence was extracted into a
scratch module under a copy of the exact Foundation-parent catalog
root. `ken check` accepted it (rc 0). A fresh copy with only the
refusal fence appended rejected (rc 1, `KernelRejected(TypeMismatch)`)
at `claimed_ordered_ys_unordered`, after the accepted prefix. The
original rootless scratch-file check instead refused an unresolved
catalog import (`UnboundName`); it is not credited as a refusal of
the law. The accepted fence is not formatter-canonical (`ken fmt
--check` rc 1); no formatting acceptance is claimed. These local
checks are not execution through the official conformance runner,
assembled QA, CI, publication, landing or landed-trust measurement.
