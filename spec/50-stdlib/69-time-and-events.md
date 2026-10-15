# Finite event traces over natural times

> **Status: DRAFT v0 (SPEC-TIME-EVENT-TRACE-CONTRACT).** Section 1
> binds finite events, half-open windows and left-biased merging, with
> exactly three generically checked Architect D0 laws. This is a
> normative contract for a prospective ordinary Ken package, not a
> claim of catalog delivery, a shipped conformance suite, CI, or a
> measured trusted-base delta.

## 1. Finite event traces over natural times

### 1.1 Events, traces and time order

`Data.Time.EventTrace`, at
`catalog/packages/Data/Time/EventTrace.ken.md`, uses the existing
`Nat` timestamp and `List` carrier. An event records a given time
and a payload of **any** `p : Type`:

```ken
pub data Event (p : Type) : Type where {
  MkEvent : (time : Nat) → (payload : p) → Event p
}
export MkEvent

pub fn event_time (p : Type) (e : Event p) : Nat
pub fn event_payload (p : Type) (e : Event p) : p
pub fn event_before (p : Type) (e : Event p) (f : Event p) : Bool
pub fn time_ordered (p : Type) (tr : List (Event p)) : Prop
```

The projections return the stored time and payload, respectively:
`event_time p (MkEvent p t x) = t` and
`event_payload p (MkEvent p t x) = x`.
`event_before p e f` is `leq_nat (event_time p e)
(event_time p f)`: payloads are **not compared**. A trace is
**plain `List (Event p)`**, not a second carrier, a newtype, or a
list carrying an ordering proof. An unordered list is still a
well-formed trace.

`time_ordered p tr` is the prelude's
`is_sorted (Event p) (event_before p) tr`. Empty and singleton lists
satisfy it. For each adjacent pair `e, f` in a longer list it
requires `Equal Bool (event_before p e f) True`, together with
ordering of the remaining list. Times are **non-decreasing**, not
strictly increasing: two events stamped with the same `Nat` time
may occur consecutively. The `Prop` predicate supplies a premise
for an order-preservation law; it is not enforced by construction.
A module client needs the explicit `export MkEvent` to construct
fixture events: `pub data` alone publishes the type, not its
constructor (`../30-surface/33 §4.1`).

### 1.2 Half-open window and left-biased merge

```ken
pub fn event_in_window
  (p : Type) (lo : Nat) (hi : Nat) (e : Event p) : Bool
pub fn trace_window
  (p : Type) (lo : Nat) (hi : Nat) (tr : List (Event p))
  : List (Event p)
pub fn trace_merge
  (p : Type) (xs : List (Event p)) (ys : List (Event p))
  : List (Event p)
```

`event_in_window p lo hi e` is
`bool_and (leq_nat lo (event_time p e))
(lt_nat (event_time p e) hi)`. Thus the window **includes** time
`lo` and **excludes** time `hi`: `lo ≤ t < hi`. `trace_window` is
`filter (Event p) (event_in_window p lo hi) tr`. It keeps each
selected event unchanged and in its original relative order, even
if the input was unordered. When `lo ≥ hi`, it returns the empty
list; there is no range-validity proof field, `Option`, or error.

`trace_merge p xs ys` merges the two lists by their current heads,
without sorting either input first. Empty input returns the other
input unchanged. With heads `x` and `y`, if
`event_before p x y` is true it emits `x` and continues with the
tail of `xs` and all of `ys`; otherwise it emits `y` and
continues with all of `xs` and the tail of `ys`. Since `≤` is true
at **equal** timestamps, a tie takes the left head before the
right head. Each input's internal order is retained, including
among its equal-time events. The result is time-ordered **when
both inputs are time-ordered**; neither operation rejects an
unordered input. The checked D0 uses a private head-insertion
helper to express the structurally recursive two-list merge; it
is not an additional public operation or law.

The public `trace_merge` spelling avoids colliding with the
existing `Data.Collections.PriorityQueue.merge`. A bare `merge`
produced a typed `AmbiguousReference` when both packages were
imported in D0; the prefixed spelling made the same imports
unambiguous. This distinction is an ordinary module interface
choice, not a new resolution or kernel rule.

### 1.3 Three checked generic laws

The prospective package must export **checked proof bodies** for
exactly these three laws, not assumptions. They quantify over every
payload type, timestamp bound, displayed trace and, in the count
law, **every Boolean classifier** `eqf` and query `q`. The two
order laws require their shown ordering premises. The count law
requires **no** ordering premise: merging arbitrary lists still
retains the count recognized by any supplied classifier.

```ken
pub theorem trace_window_ordered
  (p : Type) (lo : Nat) (hi : Nat) (tr : List (Event p)) :
  time_ordered p tr → time_ordered p (trace_window p lo hi tr)

pub theorem trace_merge_ordered
  (p : Type) (xs : List (Event p)) :
  (ys : List (Event p)) →
  time_ordered p xs → time_ordered p ys →
  time_ordered p (trace_merge p xs ys)

pub theorem trace_merge_count
  (p : Type) (eqf : Event p → Event p → Bool)
  (q : Event p) (xs : List (Event p)) :
  (ys : List (Event p)) →
  Equal Nat
    (count (Event p) eqf q (trace_merge p xs ys))
    (add (count (Event p) eqf q xs) (count (Event p) eqf q ys))
```

`trace_window_ordered` uses transitivity of `leq_nat`: filtering
an ordered list preserves its adjacent-pair order even across
dropped elements. `trace_merge_ordered` also uses totality of that
order: if the left head does not precede the right, the right head
precedes the left. The count proof uses the checked natural-addition
zero, associativity and commutativity laws. `count` is the existing
`Data.Collections.Derived` function: the arbitrary `eqf` is
applied as `eqf q e` to each occurrence, with no assumed equality
law on `p`. With an event equality classifier it expresses
multiset preservation; **no** decidable equality on every payload
is required by the general law. The D0 checked these three laws
generically. Closed examples in a conformance seed will exercise
the interfaces, not establish their quantifiers.

No window/merge distribution law, no generic window-membership
soundness or completeness law, and no structural list equality of
merged traces is included among the checked claims. In particular,
the count law alone does not choose an order for equal timestamps;
left-biased output is fixed by the operation's definition in §1.2.

### 1.4 Derivation, trust and deferred scope

`Event p` is an ordinary inductive over prelude `Nat` and an
arbitrary existing type; traces reuse prelude `List`. The ordered
predicate uses prelude `is_sorted` and checked `leq_nat`; windowing
uses `lt_nat`, `bool_and` and the existing `filter`; counting uses
the existing `count` and checked `add`. The D0 checked its 771-line
module on base `48cda520b`, rc=0, and declared no `Axiom`,
primitive or foreign item. These facts establish a source-shaped
proof development, **not** a published catalog package or a
measured zero `trusted_base()` delta across imports. No new
primitive, kernel rule or surface feature is required. The future
catalog factoring may move private generic list proofs into
`Data.Collections.Derived` and reuse its existing private sorted-
head helpers; that is a separate delivery step, not an extra law
of this section.

A timestamp here is **given data**, not a reading of a clock.
`ClockWallNow` is a host effect outside this pure interface.
Wall-clock time, durations and units, and causality between processes
(including vector and Lamport clocks) are deferred. Infinite or
lazy streams, strictly increasing traces, deduplication, k-way
merge and time over an arbitrary `Ord` are deferred too. None is
implicitly proved by the three finite-list laws.
