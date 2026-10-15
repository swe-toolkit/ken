# Finite event traces over natural times — seed cases

Format: `../../README.md`.
Spec: `spec/50-stdlib/69-time-and-events.md` §1.

These authored source-shaped cases target the prospective
`Data.Time.EventTrace` package. They pin exact closed results for both trace
operations, a non-decreasing trace with equal-time events, and one specialization
of each of the three checked generic laws. The closed law instances exercise the
public interfaces; they do not establish the universal theorems.

The expected merge order is derived from the stated head comparison, not from
sorting either input: time 1 precedes time 2; at time 3, the left payload comes
before the right payload. The window `[2,3)` keeps the event at its lower bound
and excludes both events at its upper bound. The classifier used for the count
law only observes the payload and is not an equality test; the law assumes no
ordering premise.

Architect D0 `evt_42rzxmvjwgb6d` checked the generic proof development in
scratch. The source-shaped closed client below is authored conformance material,
not an executed shipped seed. It does not claim catalog delivery, a measured
`trusted_base()` delta, Cargo-test or CI results, or publication.

## Shared fixtures and positive assertions

The two inputs are ordered and overlap at time 3. Their distinct payloads make
the left-biased tie observable. The window examples exercise both endpoints,
retain equal-time events in input order, and include the empty equal-bounds
result. `scrambled_left` and `scrambled_right` are deliberately unordered for
the count-law instance.

```ken
import Data.Time.EventTrace
  (Event, MkEvent, event_before, event_in_window, event_payload,
   time_ordered, trace_window, trace_merge, trace_window_ordered,
   trace_merge_ordered, trace_merge_count)
import Data.Collections.Derived (count)
import Data.Numeric.Nat.Arithmetic (add)

const t1 : Nat = Suc Zero
const t2 : Nat = Suc t1
const t3 : Nat = Suc t2
const t4 : Nat = Suc t3

const e1 : Event Bool = MkEvent Bool t1 True
const e2 : Event Bool = MkEvent Bool t2 False
const e3 : Event Bool = MkEvent Bool t3 False
const e3b : Event Bool = MkEvent Bool t3 True

const left : List (Event Bool) =
  Cons (Event Bool) e1 (Cons (Event Bool) e3 (Nil (Event Bool)))
const right : List (Event Bool) =
  Cons (Event Bool) e2 (Cons (Event Bool) e3b (Nil (Event Bool)))
const ordered_input : List (Event Bool) =
  Cons (Event Bool) e1
    (Cons (Event Bool) e2
      (Cons (Event Bool) e3 (Cons (Event Bool) e3b (Nil (Event Bool)))))
const merged : List (Event Bool) =
  Cons (Event Bool) e1
    (Cons (Event Bool) e2
      (Cons (Event Bool) e3 (Cons (Event Bool) e3b (Nil (Event Bool)))))
const window_half_open : List (Event Bool) =
  Cons (Event Bool) e2 (Nil (Event Bool))
const window_keeps_tie : List (Event Bool) =
  Cons (Event Bool) e2
    (Cons (Event Bool) e3 (Cons (Event Bool) e3b (Nil (Event Bool))))
const window_keeps_unordered : List (Event Bool) =
  Cons (Event Bool) e3 (Cons (Event Bool) e1 (Nil (Event Bool)))
const scrambled_left : List (Event Bool) =
  Cons (Event Bool) e3 (Cons (Event Bool) e1 (Nil (Event Bool)))
const scrambled_right : List (Event Bool) =
  Cons (Event Bool) e3b (Cons (Event Bool) e2 (Nil (Event Bool)))
const merged_unordered : List (Event Bool) =
  Cons (Event Bool) e3
    (Cons (Event Bool) e1
      (Cons (Event Bool) e3b (Cons (Event Bool) e2 (Nil (Event Bool)))))
const tie_lr : List (Event Bool) =
  Cons (Event Bool) e3 (Cons (Event Bool) e3b (Nil (Event Bool)))
const tie_rl : List (Event Bool) =
  Cons (Event Bool) e3b (Cons (Event Bool) e3 (Nil (Event Bool)))

fn payload_flag (q : Event Bool) (e : Event Bool) : Bool =
  event_payload Bool e

-- Both endpoints are tested with the same half-open window.
theorem window_lower_included :
  Equal Bool (event_in_window Bool t2 t3 e2) True = Proved
theorem window_upper_excluded :
  Equal Bool (event_in_window Bool t2 t3 e3) False = Proved
theorem window_result_half_open :
  Equal (List (Event Bool))
    (trace_window Bool t2 t3 ordered_input) window_half_open = Refl
theorem window_preserves_tie_order :
  Equal (List (Event Bool))
    (trace_window Bool t2 t4 ordered_input) window_keeps_tie = Refl
theorem window_equal_bounds_is_empty :
  Equal (List (Event Bool))
    (trace_window Bool t3 t3 ordered_input) (Nil (Event Bool)) = Refl
theorem window_keeps_unordered_relative_order :
  Equal (List (Event Bool))
    (trace_window Bool t1 t4 scrambled_left) window_keeps_unordered = Refl

-- The exact interleaving and each tie direction are observable values.
theorem merge_interleaves_and_takes_left_tie :
  Equal (List (Event Bool)) (trace_merge Bool left right) merged = Refl
theorem merge_left_tie :
  Equal (List (Event Bool))
    (trace_merge Bool
      (Cons (Event Bool) e3 (Nil (Event Bool)))
      (Cons (Event Bool) e3b (Nil (Event Bool)))) tie_lr = Refl
theorem merge_reversed_inputs_reverse_tie :
  Equal (List (Event Bool))
    (trace_merge Bool
      (Cons (Event Bool) e3b (Nil (Event Bool)))
      (Cons (Event Bool) e3 (Nil (Event Bool)))) tie_rl = Refl
theorem merge_empty_left :
  Equal (List (Event Bool))
    (trace_merge Bool (Nil (Event Bool)) right) right = Refl
theorem merge_empty_right :
  Equal (List (Event Bool))
    (trace_merge Bool left (Nil (Event Bool))) left = Refl
theorem merge_unordered_inputs_keep_head_order :
  Equal (List (Event Bool))
    (trace_merge Bool scrambled_left scrambled_right) merged_unordered = Refl

-- These are explicit witnesses for the ordering premises, not law-derived
-- input facts. Equal timestamps satisfy the non-decreasing relation.
theorem seed_left_ordered : time_ordered Bool left =
  and_intro
    (Equal Bool (event_before Bool e1 e3) True)
    (time_ordered Bool (Cons (Event Bool) e3 (Nil (Event Bool))))
    Proved Proved
theorem seed_right_ordered : time_ordered Bool right =
  and_intro
    (Equal Bool (event_before Bool e2 e3b) True)
    (time_ordered Bool (Cons (Event Bool) e3b (Nil (Event Bool))))
    Proved Proved
theorem seed_ordered_tail :
  time_ordered Bool
    (Cons (Event Bool) e2
      (Cons (Event Bool) e3 (Cons (Event Bool) e3b (Nil (Event Bool))))) =
  and_intro
    (Equal Bool (event_before Bool e2 e3) True)
    (time_ordered Bool
      (Cons (Event Bool) e3 (Cons (Event Bool) e3b (Nil (Event Bool)))))
    Proved
    (and_intro
      (Equal Bool (event_before Bool e3 e3b) True)
      (time_ordered Bool (Cons (Event Bool) e3b (Nil (Event Bool))))
      Proved Proved)
theorem seed_ordered_input : time_ordered Bool ordered_input =
  and_intro
    (Equal Bool (event_before Bool e1 e2) True)
    (time_ordered Bool
      (Cons (Event Bool) e2
        (Cons (Event Bool) e3 (Cons (Event Bool) e3b (Nil (Event Bool)))))
    Proved seed_ordered_tail

theorem seed_window_ordered :
  time_ordered Bool (trace_window Bool t2 t4 ordered_input) =
  trace_window_ordered Bool t2 t4 ordered_input seed_ordered_input

theorem seed_merge_ordered :
  time_ordered Bool (trace_merge Bool left right) =
  trace_merge_ordered Bool left right seed_left_ordered seed_right_ordered

-- The classifier counts True payloads and ignores its query argument.
-- The inputs need not be ordered for this law.
theorem seed_merge_count_without_order_premises :
  Equal Nat
    (count (Event Bool) payload_flag e3
      (trace_merge Bool scrambled_left scrambled_right))
    (add
      (count (Event Bool) payload_flag e3 scrambled_left)
      (count (Event Bool) payload_flag e3 scrambled_right)) =
  trace_merge_count Bool payload_flag e3 scrambled_left scrambled_right
theorem count_true_payload_left :
  Equal Nat (count (Event Bool) payload_flag e3 scrambled_left) (Suc Zero) = Proved
theorem count_true_payload_right :
  Equal Nat (count (Event Bool) payload_flag e3 scrambled_right) (Suc Zero) = Proved
theorem count_true_payload_after_merge :
  Equal Nat
    (count (Event Bool) payload_flag e3
      (trace_merge Bool scrambled_left scrambled_right))
    (Suc (Suc Zero)) = Proved
```

## Window selection and order preservation

### stdlib/time-and-events/window-half-open

- spec: `spec/50-stdlib/69-time-and-events.md` §§1.1–1.2.
- promise class: durable invariant.
- given: `ordered_input` and the window `[t2,t3)`.
- expect: `event_in_window` includes `e2` at `t2` and excludes `e3` at
  `t3`; `trace_window` returns exactly `[e2]`.
- measured: both boundary comparisons and the exact filtered list.
- claimed: the window is lower-inclusive and upper-exclusive.
- gap: a closed input does not prove the definition for every event or bound.
- why: the same input has events at both bounds, so changing either strictness
  changes a named result.

### stdlib/time-and-events/window-order-and-ties

- spec: `spec/50-stdlib/69-time-and-events.md` §§1.1–1.3.
- promise class: durable invariant.
- given: the ordered four-event trace and `[t2,t4)`.
- expect: the output is `[e2,e3,e3b]`, retaining both equal-time events in
  their original order; equal bounds produce `Nil`. On the unordered
  `scrambled_left`, `[t1,t4)` retains `[e3,e1]`. `seed_window_ordered`
  specializes `trace_window_ordered` on the ordered input.
- measured: a nonempty multi-event output, its tied pair, and the closed
  generic-law instance.
- claimed: filtering preserves relative order and the checked law supplies
  ordered output from an ordered input.
- gap: this instance does not establish the generic theorem or every window.
- why: dropping or reordering a tied or unordered event changes the direct
  list oracle; the law instance separately exercises its stated premise and
  conclusion.

## Left-biased merge

### stdlib/time-and-events/merge-order-and-tie

- spec: `spec/50-stdlib/69-time-and-events.md` §§1.1–1.3.
- promise class: durable invariant.
- given: `left = [e1,e3]` and `right = [e2,e3b]`, with distinct payloads at
  the shared time `t3`.
- expect: the ordered inputs merge to `[e1,e2,e3,e3b]`; swapping the
  singleton inputs at `t3` swaps the tie order. Unordered inputs produce
  `[e3,e1,e3b,e2]` by current-head comparison, not by pre-sorting. Empty-input
  controls return the other input.
- measured: the exact interleaving, both directions of the equal-time tie, and
  the non-sorted output from unordered inputs.
- claimed: merge reads current heads, does not sort first, and chooses the left
  head when times are equal.
- gap: finite Bool payload fixtures do not prove behavior for every payload.
- why: `e3` and `e3b` have equal time but different payloads, so the asserted
  order cannot be attributed to payload comparison.

### stdlib/time-and-events/merge-ordered

- spec: `spec/50-stdlib/69-time-and-events.md` §1.3.
- promise class: durable invariant.
- given: the two explicitly ordered inputs `left` and `right`.
- expect: `seed_merge_ordered` specializes `trace_merge_ordered` and establishes
  `time_ordered` for their exact merged result.
- measured: both ordering premises and the result predicate are closed terms.
- claimed: merging two non-decreasing inputs produces a non-decreasing trace.
- gap: this closed Bool instance does not establish the universal theorem.
- why: the result contains both the strictly ordered heads and the equal-time
  left/right pair, so the instance is not a singleton or empty success.

## Merge count without order premises

### stdlib/time-and-events/merge-count

- spec: `spec/50-stdlib/69-time-and-events.md` §1.3.
- promise class: durable invariant.
- given: the unordered inputs `scrambled_left` and `scrambled_right`, query
  `e3`, and `payload_flag`, which returns the candidate event's payload.
- expect: `seed_merge_count_without_order_premises` specializes
  `trace_merge_count`; each unordered input has one `True` payload and the
  merged count is two.
- measured: the law instance has no `time_ordered` premise, and the direct
  result count equals the sum while exceeding either input count.
- claimed: merge preserves the count recognized by any Boolean classifier,
  regardless of input order.
- gap: this one classifier and fixture do not prove the generic quantifiers.
- why: an equality-only or order-dependent classifier is not assumed; the
  classifier ignores its query and detects one `True` payload from each side.

## Refused out-of-time-order result

### stdlib/time-and-events/reject-out-of-time-order-merge-result

- spec: `spec/50-stdlib/69-time-and-events.md` §1.2.
- promise class: durable invariant.
- given: the same `left` and `right` inputs as the positive merge oracle.
- expect: the claim that `trace_merge Bool left right` equals
  `[e1,e3,e2,e3b]` is refused; the checked result is `[e1,e2,e3,e3b]`.
- measured: the false claim swaps time 2 and time 3 while keeping the input
  fixtures and merge operation unchanged.
- claimed: the result follows the stated head comparison and is not the
  out-of-time-order list.
- gap: this fixed false equality does not define a runtime error for unordered
  inputs or establish a generic sortedness theorem.
- why: the positive exact-output oracle accepts the actual result, while this
  false `Equal (List (Event Bool))` claim cannot be proved by `Refl` or
  `Proved`; it is a value refusal, not an invalid-input rejection.

```ken
-- Refused: this expected merge value places time 3 before time 2.
theorem refuse_out_of_time_order_merge_result :
  Equal (List (Event Bool))
    (trace_merge Bool left right)
    (Cons (Event Bool) e1
      (Cons (Event Bool) e3
        (Cons (Event Bool) e2 (Cons (Event Bool) e3b (Nil (Event Bool))))))
  = Proved
```

## Coverage boundary

The positive client exercises the point window predicate, both trace operations,
all three exact D0 laws, empty merge inputs, the half-open bounds, preservation
of unordered window order, left-biased ties, and current-head merging without
pre-sorting. The orderedness proofs supply both premises required by the
order laws. The merge-count instance deliberately supplies no order proofs,
matching the law's broader contract. These finite cases are not a substitute
for checking the generic D0 theorem bodies. The refused equality tests an
incorrect claimed result; it does not claim that `trace_merge` rejects
unordered inputs, which the specification explicitly allows.
