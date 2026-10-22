# Premise-free membership of a merge

Format: `../../README.md`.
Spec: `spec/50-stdlib/69-time-and-events.md` §4, as stated by AC-1
candidate `d68d2a45846e171a1260393f800108ddb3eefac1`.
Frame: `docs/program/issues/SPEC-TIME-MERGE-MEMBERSHIP.md`.

This source-shaped client covers the three §4 membership laws on a closed
pair of unordered traces. Each input starts with a distinct event at the
same time, so the fixture has a cross-input tie; each following event is
earlier, so neither input is time-ordered. The client supplies no ordering
premise.

The current authoring base is `a0311a8a758dac707ee64368a00cea8339d81db7`.
It lacks the three §4 catalog exports. Architect D0 `evt_1dwxvhjh9p81j`
checked their proof development in scratch on base
`747b105cfaf6c50aa03cee8294a3fff4d08da650`; that is not seed execution.
No Ken fence or conformance runner was executed. Do not claim execution
until a verified base contains the exports. This is a separate source
candidate; no CI, publication, landing, or trust measurement is claimed.

## Closed unordered pair and positive controls

`xs` and `ys` each have times `[2,1]`. The two heads are tied across the
inputs, and both adjacent comparisons within the inputs are false. The
closed merge result is included as a fixture control. The positive law
instances retain one member from each same input; the soundness instance
starts from a member of the merged result. Together they exercise all
three public law types without using an ordering premise.

```ken
import Core.Logic.Or (Or, Inl)
import Data.Collections.Derived (list_elem, list_elem_head, list_elem_later)
import Data.Time.EventTrace
  (Event, MkEvent, event_before, trace_merge,
   trace_merge_member_sound, trace_merge_member_left,
   trace_merge_member_right)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two

const left_tie : Event Bool = MkEvent Bool two True
const left_early : Event Bool = MkEvent Bool one False
const right_tie : Event Bool = MkEvent Bool two False
const right_early : Event Bool = MkEvent Bool one True
const neither : Event Bool = MkEvent Bool three True

const xs : List (Event Bool) =
  Cons (Event Bool) left_tie
    (Cons (Event Bool) left_early (Nil (Event Bool)))
const ys : List (Event Bool) =
  Cons (Event Bool) right_tie
    (Cons (Event Bool) right_early (Nil (Event Bool)))
const merged : List (Event Bool) =
  Cons (Event Bool) left_tie
    (Cons (Event Bool) left_early
      (Cons (Event Bool) right_tie
        (Cons (Event Bool) right_early (Nil (Event Bool)))))

theorem left_input_decreases :
  Equal Bool (event_before Bool left_tie left_early) False = Proved

theorem right_input_decreases :
  Equal Bool (event_before Bool right_tie right_early) False = Proved

theorem heads_are_tied_across_inputs :
  Equal Bool (event_before Bool left_tie right_tie) True = Proved

theorem merge_unordered_pair_exact :
  Equal (List (Event Bool)) (trace_merge Bool xs ys) merged = Refl

theorem left_member_is_retained :
  list_elem (Event Bool) left_tie (trace_merge Bool xs ys) =
  trace_merge_member_left
    Bool left_tie xs ys
    (list_elem_head
      (Event Bool)
      left_tie
      (Cons (Event Bool) left_early (Nil (Event Bool))))

theorem right_member_is_retained :
  list_elem (Event Bool) right_tie (trace_merge Bool xs ys) =
  trace_merge_member_right
    Bool right_tie xs ys
    (list_elem_head
      (Event Bool)
      right_tie
      (Cons (Event Bool) right_early (Nil (Event Bool))))

theorem right_result_member_is_sound :
  ‖ Or
      (list_elem (Event Bool) right_tie xs)
      (list_elem (Event Bool) right_tie ys)
  ‖ =
  trace_merge_member_sound
    Bool
    right_tie
    xs
    ys
    (list_elem_later
      (Event Bool)
      right_tie
      left_tie
      (Cons (Event Bool) left_early
        (Cons (Event Bool) right_tie
          (Cons (Event Bool) right_early (Nil (Event Bool)))))
      (list_elem_later
        (Event Bool)
        right_tie
        left_early
        (Cons (Event Bool) right_tie
          (Cons (Event Bool) right_early (Nil (Event Bool))))
        (list_elem_head
          (Event Bool)
          right_tie
          (Cons (Event Bool) right_early (Nil (Event Bool)))))
```

### stdlib/time-and-events/merge-membership-unordered-tie

- spec: `spec/50-stdlib/69-time-and-events.md` §4.
- promise class: durable invariant.
- given: unordered `xs=[left_tie,left_early]` and
  `ys=[right_tie,right_early]`, with a cross-input tie at time `2`.
- expect: each same-input member is in the merge, and a merged member
  is in at least one input, without ordering premises.
- measured: not executed in this seed. The authored assertions check both
  decreasing head-to-tail comparisons, the cross-input tie, the closed merge
  value, both completeness instances, and one soundness instance.
- claimed: the three generic merge-membership laws apply to this closed
  unordered pair.
- gap: these finite Bool-payload instances do not establish the generic
  quantifiers or payload independence.
- why: each input has two events and a decreasing timestamp pair; one
  event from each side is tied at the heads.

## Refuse a member from neither input

Runner assembly: first build and accept a client containing the entire
positive Ken fence above. Copy that accepted client fresh and append only
the refusal fence below. Do not run the negative fence alone. The combined
client must reach the false `Equal (Event Bool) neither left_tie`/`Refl`
witness after the positive prefix; rejection for a missing import or
fixture, or at an earlier positive assertion, is not the expected result.

`neither` has time `3`, while every input event has time `1` or `2`. The
false proof attempts the head-equality branch of the merged list's
`list_elem`; it asks `Refl` to identify `neither` with `left_tie`. The
closed merge control and same-input positive law instances guard against
an unrelated failure.

### stdlib/time-and-events/reject-event-from-neither-input

- spec: `spec/50-stdlib/69-time-and-events.md` §4.
- promise class: durable invariant.
- given: a fresh copy of the accepted client containing the entire
  positive fence, plus only this refusal fence; `neither` has time `3` and
  occurs in neither input.
- expect: the combined client is refused at the false head-equality
  witness, not at a missing fixture or an earlier assertion.
- measured: not executed in this seed. The positive prefix asserts the
  closed merge value and both same-input membership instances.
- claimed: an event absent from both inputs is not a member of their
  merge.
- gap: this refusal checks one attempted head witness, not every possible
  witness or the generic soundness law.
- why: it uses the same merge result and head branch as the left-member
  control; only the queried event changes to one from neither input.

```ken
-- Refused: `neither` at time 3 is not the `left_tie` at time 2.
theorem refuse_event_from_neither_input :
  list_elem (Event Bool) neither (trace_merge Bool xs ys) =
  trunc_intro
    (Inl
      (Equal (Event Bool) neither left_tie)
      (list_elem (Event Bool) neither
        (Cons (Event Bool) left_early
          (Cons (Event Bool) right_tie
            (Cons (Event Bool) right_early (Nil (Event Bool))))))
      Refl)
```
