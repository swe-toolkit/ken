# Exact membership in a time window — seed cases

Format: `../../README.md`.
Spec: `spec/50-stdlib/69-time-and-events.md` §2.
Frame: `docs/program/issues/SPEC-TIME-WINDOW-MEMBERSHIP.md`.

This source-shaped client targets the two generic window-membership laws. Its
membership-law instances use function-valued payloads, so no payload equality
decision is introduced. The exact-output oracle uses a separate Bool-payload
trace. The window includes the event at `lo` and excludes the event at
`hi`; the positive law instances exercise the lower boundary. A refused block
tries to establish membership for the upper-bound event using a false bound
fact. These cases instantiate the generic laws; they do not establish their
universal quantifiers.

Architect's D0 reports checked generic filter and window proof bodies on a
scratch tree. The cases below are authored conformance material, not an
executed shipped seed. They do not claim catalog delivery, a measured
`trusted_base()` delta, CI, or publication.

## Closed fixtures and boundary results

The membership-law input uses one function payload at distinct times. It
contains the lower-bound event, an interior event, and the upper-bound event.
The exact-output check uses a separate Bool-payload `first_order_input` with
those same times.

```ken
import Core.Classes.LawfulClasses (leq_nat)
import Data.Numeric.Nat.Order (lt_nat)
import Data.Collections.Derived
  (list_elem, list_elem_head, list_elem_later)
import Data.Time.EventTrace
  (Event, MkEvent, event_in_window, event_time, trace_window,
   trace_window_member_sound, trace_window_member_complete)

fn payload_id (n : Nat) : Nat = n

const lo : Nat = Suc Zero
const middle : Nat = Suc lo
const hi : Nat = Suc middle

const at_lo : Event (Nat → Nat) = MkEvent (Nat → Nat) lo payload_id
const at_middle : Event (Nat → Nat) = MkEvent (Nat → Nat) middle payload_id
const at_hi : Event (Nat → Nat) = MkEvent (Nat → Nat) hi payload_id

const input : List (Event (Nat → Nat)) =
  Cons (Event (Nat → Nat)) at_lo
    (Cons (Event (Nat → Nat)) at_middle
      (Cons (Event (Nat → Nat)) at_hi (Nil (Event (Nat → Nat)))))
const first_order_input : List (Event Bool) =
  Cons (Event Bool) (MkEvent Bool lo True)
    (Cons (Event Bool) (MkEvent Bool middle False)
      (Cons (Event Bool) (MkEvent Bool hi True) (Nil (Event Bool))))

theorem lower_endpoint_is_included :
  Equal Bool (event_in_window (Nat → Nat) lo hi at_lo) True = Proved

theorem upper_endpoint_is_excluded :
  Equal Bool (event_in_window (Nat → Nat) lo hi at_hi) False = Proved

theorem window_selects_exactly_the_half_open_range :
  Equal (List (Event Bool))
    (trace_window Bool lo hi first_order_input)
    (Cons (Event Bool) (MkEvent Bool lo True)
      (Cons (Event Bool) (MkEvent Bool middle False) (Nil (Event Bool)))) = Refl
```

### stdlib/time-and-events/window-membership-boundaries

- spec: `spec/50-stdlib/69-time-and-events.md` §§2.1–2.3.
- given: `first_order_input` and the half-open window `[lo, hi)`.
- expect: the exact window contains the events at `lo` and `middle`, with Bool
  payloads `True` and `False`; the event at `hi` is excluded.
- why: the two endpoint events and in-range control make either endpoint
  change observable in the exact output.
- measured: Architect reports this exact Bool-payload block rc 0 and its
  keep-`hi` mutation rc 1 on a candidate scratch tree with D0 EventTrace laws.
- claimed: this fixture includes `lo`, excludes `hi`, and retains the interior
  event.
- the gap: the D0 result and this closed instance do not establish the
  behavior for every trace, payload, or pair of bounds.
- limitation: an exact list equality over function payloads is not closed by
  `Refl` on this base; function payloads remain in the membership-law cases.

## Closed instances of both laws

Soundness is instantiated on the lower-bound event in the filtered list. Its
conclusion supplies input membership and both separate bound facts. A second
soundness instance shows that any claimed membership for `at_hi` implies
`Bottom` via the false upper-bound projection. Completeness uses the lower-bound
event, with a closed source-membership witness and closed proofs of
`lo ≤ event_time` and `event_time < hi`.

```ken
theorem lower_member_sound :
  And
    (list_elem (Event (Nat → Nat)) at_lo input)
    (And
      (Equal Bool (leq_nat lo (event_time (Nat → Nat) at_lo)) True)
      (Equal Bool (lt_nat (event_time (Nat → Nat) at_lo) hi) True)) =
  trace_window_member_sound
    (Nat → Nat)
    lo
    hi
    input
    at_lo
    (list_elem_head
      (Event (Nat → Nat))
      at_lo
      (Cons (Event (Nat → Nat)) at_middle
        (Nil (Event (Nat → Nat)))))

theorem lower_member_complete :
  list_elem (Event (Nat → Nat)) at_lo
    (trace_window (Nat → Nat) lo hi input) =
  trace_window_member_complete
    (Nat → Nat)
    lo
    hi
    input
    at_lo
    (list_elem_head
      (Event (Nat → Nat))
      at_lo
      (Cons (Event (Nat → Nat)) at_middle
        (Cons (Event (Nat → Nat)) at_hi (Nil (Event (Nat → Nat))))))
    Proved
    Proved

theorem upper_endpoint_membership_is_impossible :
  list_elem (Event (Nat → Nat)) at_hi
    (trace_window (Nat → Nat) lo hi input) → Bottom =
  λmember.
    absurd
      (and_snd
        (Equal Bool (leq_nat lo (event_time (Nat → Nat) at_hi)) True)
        (Equal Bool (lt_nat (event_time (Nat → Nat) at_hi) hi) True)
        (and_snd
          (list_elem (Event (Nat → Nat)) at_hi input)
          (And
            (Equal Bool (leq_nat lo (event_time (Nat → Nat) at_hi)) True)
            (Equal Bool (lt_nat (event_time (Nat → Nat) at_hi) hi) True))
          (trace_window_member_sound
            (Nat → Nat)
            lo
            hi
            input
            at_hi
            member)))
```

### stdlib/time-and-events/window-membership-soundness

- spec: `spec/50-stdlib/69-time-and-events.md` §2.3.
- given: the closed `input`, the lower-bound event in its window, and a
  hypothetical upper-bound event in the window.
- expect: soundness returns input membership and both lower/upper facts for
  `at_lo`; for `at_hi`, the upper-bound projection yields `Bottom`.
- why: the lower event exercises soundness positively while the upper event
  checks its exclusion consequence.
- measured: Architect reports the generic proof body checked on scratch and a
  `hi_dropped` client deriving `Bottom` from soundness's upper-bound projection.
  CV did not execute this seed's instance.
- claimed: members of the window occur in the input and satisfy both bound
  facts; the upper-bound event cannot be a member.
- the gap: one fixed trace and payload do not establish the generic law, and
  scratch evidence does not establish a shipped catalog artifact.

### stdlib/time-and-events/window-membership-completeness

- spec: `spec/50-stdlib/69-time-and-events.md` §2.3.
- given: `at_lo` in the closed input and closed proofs of both bound facts.
- expect: `lower_member_complete` supplies membership in the filtered window.
- why: `lo` is included by the stated `lo ≤ t < hi` contract, with a positive
  control for the upper-bound refusal below.
- measured: Architect reports the generic proof body checked on scratch and
  positive `lo_kept`/`inside_kept` clients using completeness. CV did not run
  this seed.
- claimed: an in-range source member is present in the filtered window.
- the gap: these closed clients do not prove the universal completeness law or
  establish delivery of the §2 EventTrace API.

## Refused upper-bound membership claim

The source trace contains `at_hi`, but the half-open window excludes it. The
following false block supplies `Proved` for `lt_nat hi hi`, then tries to use
completeness to prove that `at_hi` is in the window. It must be refused at that
false upper-bound premise. The positive lower-bound instance above controls
for unrelated failure of the completeness interface.

```ken
-- Refused: lt_nat hi hi is False, not True.
theorem refuse_upper_endpoint_membership :
  list_elem (Event (Nat → Nat)) at_hi
    (trace_window (Nat → Nat) lo hi input) =
  trace_window_member_complete
    (Nat → Nat)
    lo
    hi
    input
    at_hi
    (list_elem_later
      (Event (Nat → Nat))
      at_hi
      at_lo
      (Cons (Event (Nat → Nat)) at_middle
        (Cons (Event (Nat → Nat)) at_hi (Nil (Event (Nat → Nat)))))
      (list_elem_later
        (Event (Nat → Nat))
        at_hi
        at_middle
        (Cons (Event (Nat → Nat)) at_hi (Nil (Event (Nat → Nat))))
        (list_elem_head (Event (Nat → Nat)) at_hi (Nil (Event (Nat → Nat))))))
    Proved
    Proved
```

### stdlib/time-and-events/reject-upper-bound-membership

- spec: `spec/50-stdlib/69-time-and-events.md` §2.3.
- given: `at_hi` is in the source trace, but the attempted completeness
  instance supplies `Proved` for the false fact `lt_nat hi hi`.
- expect: the block is refused at the false upper-bound premise.
- why: the exact-window result and positive lower-bound completeness control
  show this is the half-open boundary, not an unrelated membership failure.
- measured: Architect reports the upper-bound completeness attempt refused at
  the false `lt_nat` premise in its scratch client. CV did not run this block.
- claimed: the upper-bound event is not admitted by a false in-range premise.
- the gap: one refused block does not establish refusal for every false
  membership claim or substitute for executing the shipped conformance seed.

Architect's D0 reports the generic laws hold for `p = Nat → Nat`, without a
payload equality decision. It also reports a refusal for upper-bound
completeness at the false `lt_nat` premise. That is scratch evidence, not an
execution of this seed or the §2 catalog laws after publication. No shipped
seed execution, CI, landing, or publication is claimed.
