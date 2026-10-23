# Finite event traces over natural times

> **Status: DRAFT v0 (SPEC-TIME-EVENT-TRACE-CONTRACT §1;
> SPEC-TIME-WINDOW-MEMBERSHIP §2;
> SPEC-TIME-WINDOW-MERGE-DISTRIBUTION §3;
> SPEC-TIME-MERGE-MEMBERSHIP §4).** The §1–2 EventTrace
> operations, five checked laws, two generic filter prerequisites and
> both existing seeds are present on main. Section 3 specifies the
> Architect-D0-checked ordered-input distribution law. The checked §3
> catalog export is now present in `Data.Time.EventTrace`; package
> acceptance runs the authored seed at:
> `conformance/stdlib/time-and-events/seed-window-merge-distribution.md`.
> D0 scratch checks alone were not shipped seed execution. CI and a
> measured landed trust delta are not established by this status.
> Section 4's three premise-free merge membership laws are now checked
> catalog exports. A targeted package test executes the authored seed's
> positive and refused clients, but neither official conformance-runner
> execution, CI nor a landed trust measurement is established here.

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

The landed package exports **checked proof bodies** for exactly these
three §1 laws, not assumptions. They quantify over every
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

These three §1 claims do not include window membership or structural
window/merge distribution; §2 separately states the two membership
laws, and §3 states distribution under ordered-input premises. No
unconditional structural list equality for unordered merged traces
follows. The count law alone does not choose an order for equal
timestamps; left-biased output is fixed by §1.2's definition.

### 1.4 Derivation, trust and deferred scope

`Event p` is an ordinary inductive over prelude `Nat` and an
arbitrary existing type; traces reuse prelude `List`. The ordered
predicate uses prelude `is_sorted` and checked `leq_nat`; windowing
uses `lt_nat`, `bool_and` and the existing `filter`; counting uses
the existing `count` and checked `add`. The historical §1 D0 checked
its 771-line module on base `48cda520b`, rc 0, and declared no
`Axiom`, primitive or foreign item. The §1–2 catalog module and both
conformance seeds have since landed; the D0 check itself was scratch
evidence, **not** a shipped seed run or a landed trust measurement.
The catalog reuses generic filter, sorted-list and count helpers from
`Data.Collections.Derived`. No new primitive, kernel rule or surface
feature is required by §1.

A timestamp here is **given data**, not a reading of a clock.
`ClockWallNow` is a host effect outside this pure interface.
Wall-clock time, durations and units, and causality between processes
(including vector and Lamport clocks) are deferred. Infinite or
lazy streams, strictly increasing traces, deduplication, k-way
merge and time over an arbitrary `Ord` are deferred too. None is
implicitly proved by the three finite-list laws.

## 2. Exact membership in a time window

### 2.1 Membership without payload comparison

The membership proposition is the existing `list_elem` from
`Data.Collections.Derived`. For any `a : Type`, it is an Ω-valued,
truncated disjunction of equality with the current head and membership
in the tail:

```ken
pub fn list_elem (a : Type) (x : a) (xs : List a) : Omega =
  match xs {
    Nil ↦ Bottom;
    Cons y rest ↦ ‖ Or (Equal a x y) (list_elem a x rest) ‖
  }
```

Window membership uses `list_elem (Event p)`, including when `p` is a
function type; it does not require `DecEq p`, `Ord p` or a Boolean
comparison of payloads. The proposition establishes whether an event
occurs, not how many times it occurs. Repeated events are all retained by
`filter`, but the laws below do not state their multiplicities; §1.3's
`trace_merge_count` remains the separate count law.

### 2.2 Generic filter prerequisites

`Data.Collections.Derived` exports two **checked generic theorems** for
its existing `filter` and `list_elem`, with proof bodies rather than
assumptions. They hold for every element type, predicate, query and list:

```ken
pub theorem list_elem_filter_sound
  (a : Type) (keep : a → Bool) (x : a) (xs : List a) :
  list_elem a x (filter a keep xs) →
  And (list_elem a x xs) (Equal Bool (keep x) True)

pub theorem list_elem_filter_complete
  (a : Type) (keep : a → Bool) (x : a) (xs : List a) :
  list_elem a x xs → Equal Bool (keep x) True →
  list_elem a x (filter a keep xs)
```

`And` here and below is the ambient **Ω-valued** conjunction, introduced
by `and_intro` and projected by `and_fst` and `and_snd`, not the
Type-valued `Core.Logic.And` carrier. Soundness transports a kept
member back to the input and proves its predicate true; completeness
transports an input member with a true predicate into the filtered
list. No equality decision on `a` is involved. Their private helpers
case-split the computed `keep h` through a named Boolean decision; none
is a new public law.

### 2.3 Two checked generic window laws

`Data.Time.EventTrace` exports exactly these **two additional checked
proofs** over the existing §1 operations. A member of the window must
occur in the input and satisfy each half-open bound separately:

```ken
pub theorem trace_window_member_sound
  (p : Type) (lo : Nat) (hi : Nat)
  (tr : List (Event p)) (e : Event p) :
  list_elem (Event p) e (trace_window p lo hi tr) →
  And
    (list_elem (Event p) e tr)
    (And
      (Equal Bool (leq_nat lo (event_time p e)) True)
      (Equal Bool (lt_nat (event_time p e) hi) True))
```

Conversely, membership in the input together with both Boolean bound
facts supplies membership in the window. There is no ordering premise:

```ken
pub theorem trace_window_member_complete
  (p : Type) (lo : Nat) (hi : Nat)
  (tr : List (Event p)) (e : Event p) :
  list_elem (Event p) e tr →
  Equal Bool (leq_nat lo (event_time p e)) True →
  Equal Bool (lt_nat (event_time p e) hi) True →
  list_elem (Event p) e (trace_window p lo hi tr)
```

The package imports `list_elem`, `list_elem_filter_sound` and
`list_elem_filter_complete` from `Data.Collections.Derived`, and
`and_true` from `Core.Classes.LawfulClasses` to join the two bound
facts for the filter-completeness proof. It does **not** import
`Core.Logic.And (And)`; both law statements use the ambient Ω `And`.
The two generic filter proofs and these two instantiated window proofs
checked on the Architect's D0 scratch tree on base `a0e9a1d30`, rc 0,
without a new `Axiom`. All four proofs are now exported on main from
`Data.Collections.Derived` and `Data.Time.EventTrace` respectively;
that earlier scratch check is not a shipped-seed execution or a
landed `trusted_base()` measurement.

These laws characterize **membership**, not an equality of lists or
multiplicity. Filtering retains input order as specified in §1.2, and
`trace_window_ordered` in §1.3 is the existing order law; §2 adds no
second ordering law. The §1.3 exclusion of window-membership laws from
its three checked claims describes §1, not these additional §2 claims.
For `lo ≥ hi`, both completeness bound facts cannot hold for an event,
and soundness over the empty window has an empty premise; neither case
requires an extra public law. Window/merge distribution is separately
specified in §3; the other §1.4 deferred topics remain deferred.

## 3. Windowing an ordered merge

> **Section 3 status: DRAFT v0
> (SPEC-TIME-WINDOW-MERGE-DISTRIBUTION).** Architect D0 checked the
> generic law below on the landed §1–2 base. The checked §3 law is
> now a catalog export; package acceptance runs the authored seed at:
> `conformance/stdlib/time-and-events/seed-window-merge-distribution.md`.
> CI and a landed trust measurement are not established by this status.

### 3.1 Structural distribution with two ordering premises

The package must export a **checked proof body** for this public law,
with precisely the displayed `Equal (List (Event p))` conclusion:

```ken
pub theorem trace_window_merge
      (p : Type) (lo : Nat) (hi : Nat) (xs : List (Event p))
    : (ys : List (Event p))
      → time_ordered p xs
      → time_ordered p ys
      → Equal
        (List (Event p))
        (trace_window p lo hi (trace_merge p xs ys))
        (trace_merge p (trace_window p lo hi xs) (trace_window p lo hi ys))
```

Both input traces must be non-decreasing by time. The law holds for
**every** payload type `p` without a payload `DecEq` or `Ord`, including
traces with equal-time events. It equates the two resulting lists of
**stored events in order**, not merely their membership or counts.
Windowing preserves the relative order of kept events, and merging
continues to favor the left event on a cross-input timestamp tie.
Neither operation validates ordering or sorts an unordered input;
the two proofs are law premises, not fields of a new trace carrier.
The half-open bounds may be equal or inverted, in which case both
sides are empty.

The two-premise condition is necessary in general. With window
`[0,4)`, an unordered `xs=[5,1]` and ordered `ys=[3]` produce
`trace_window (trace_merge xs ys)=[3,1]`, while merging the two
windowed inputs yields `[1,3]`. Swapping the roles to ordered
`xs=[3]` and unordered `ys=[5,1]` gives the same difference. Thus
neither a premise-free structural equality nor a law requiring only
one ordered input is asserted. In these two closed counterexamples,
both outputs contain the same events; that closed observation does
not imply list equality or establish a generic premise-free
window/merge membership or count-distribution law. Section 4
separately proves direct merge membership without ordering.

A closed ordered illustration uses `xs=[0a,3a,6a]` and
`ys=[3b,5b,8b]` with distinct `Bool` payloads `a` and `b`, window
`[1,7)`. Both arrangements yield `[3a,3b,5b,6a]`: the window
drops the early left event and late right event but keeps the left
`3a` before the tied right `3b`. A conformance seed will check the
law application and this exact result separately; these closed
checks do not establish the generic quantified law.

### 3.2 Consequences and excluded claims

Applying `count` to both sides of `trace_window_merge` and then
using §1.3's `trace_merge_count` gives a **count corollary under the
same two ordering premises**: the count of a windowed merge equals
the sum of the counts of the windowed inputs, for any classifier and
query. It needs no additional public theorem. The §3 D0 did not
establish a premise-free structural window/merge equality or a
premise-free window/merge count-distribution law, and this section
claims neither. Section 2's membership laws characterize one window
at a time; they do not by themselves state membership of a merge.
Section 4 separately specifies premise-free **merge membership** in
both directions, with a private one-way window-of-merge membership
consequence. Section 4 adds no further public window/merge law,
payload equality decision, sorted carrier type, primitive, kernel
or surface rule.

### 3.3 Evidence and catalog boundary

On landed base `81ab720ff5170b4d0f45ea320101760e56fd98ca`,
Architect D0 checked the generic EventTrace proof and a necessary
`Data.Collections.Derived` provider export in scratch with rc 0 and
no new `Axiom`, reporting zero added trust. Its positive client
checked the ordered law, cross-input tie and count corollary.
Separate closed negative clients rejected both one-sided-order
counterexamples; claims of an unordered ordering premise and of
the false equality were also rejected. These are **scratch D0**
results, not publication of the §3 catalog export, execution of a
shipped §3 seed, CI, or a landed trust measurement. The delivered
catalog candidate checks this law and runs the authored seed in targeted
package acceptance; CI and landing remain separate measurements.

The catalog export makes the existing generic `filter_starts_after`
proof public in `Data.Collections.Derived`. `Data.Time.EventTrace`
reaches it by adding the name to its existing
`import Data.Collections.Derived` selection, not by adding a new
import declaration. No duplicate local proof or new ambient name is
specified. The new merge/window proof helpers remain private. The
only new §3 public law is `trace_window_merge`.

## 4. Premise-free membership of a merge

An event occurs in `trace_merge p xs ys` exactly when it occurs in at
least one input trace, regardless of timestamp order. This is
**membership**, not structural equality of lists or a claim about
multiplicities. It uses §2.1's Ω-valued `list_elem (Event p)` for any
`p : Type`, including payloads without decidable equality. The
truncated `Or` in the soundness result records the two possible
sources without exposing a position or requiring a payload
comparison. The package must export **three checked proof bodies**
with exactly these public types:

```ken
pub theorem trace_merge_member_sound
      (p : Type) (e : Event p) (xs : List (Event p))
    : (ys : List (Event p))
      → list_elem (Event p) e (trace_merge p xs ys)
      → ‖ Or (list_elem (Event p) e xs) (list_elem (Event p) e ys) ‖

pub theorem trace_merge_member_left
      (p : Type) (e : Event p) (xs : List (Event p))
    : (ys : List (Event p))
      → list_elem (Event p) e xs
      → list_elem (Event p) e (trace_merge p xs ys)

pub theorem trace_merge_member_right
      (p : Type) (e : Event p) (xs : List (Event p))
    : (ys : List (Event p))
      → list_elem (Event p) e ys
      → list_elem (Event p) e (trace_merge p xs ys)
```

`trace_merge_member_sound` excludes events not in either input.
`trace_merge_member_left` and `trace_merge_member_right` retain a
member from each side, including equal-time events whose merge order
is decided by §1.2's left tie rule. Together they supply a
source-to-merge direction for a truncated choice of input: a client
can eliminate that choice into the Ω-valued merge membership goal
and apply the corresponding law. That derived combination has no
fourth public name. No `time_ordered` premise is needed for any of
the three laws; each input can be unordered. The laws do not imply
the ordered-input structural distribution of §3.1 when its premises
are absent.

There is a **private**, one-way window-of-merge consequence. A
member of `trace_window p lo hi (trace_merge p xs ys)` is a member
of the merge by `trace_window_member_sound` (§2.3), then belongs to
`xs` or `ys` by `trace_merge_member_sound`. This yields a truncated
choice between `list_elem (Event p) e xs` and
`list_elem (Event p) e ys` without any ordering premise. The
checked D0 names this composition `trace_window_merge_member_sound`
privately; §4 adds no public window/merge membership or count law.
The §2.3 window law also supplies both half-open bound facts, but
neither those facts nor this consequence assert a structural
window/merge equality for unordered inputs.

The checked proof follows the existing structurally recursive
`trace_merge`/private `trace_merge_into` arrangement (§1.2).
Three private `trace_merge_into_member_*` helpers carry the
respective recursive soundness, left and right membership premises
through the right input; their three private `_cons` branches
transport the selected head comparison. The development reuses
`list_elem_later` from `Data.Collections.Derived` and imports
`Or`, `Inl` and `Inr` explicitly from `Core.Logic.Or`. It does
**not** reuse `list_elem_append_left` or `_right`: merge is not
append. `elim_trunc`, `trunc_intro` and `absurd` are existing
surface forms, not new ambient names or primitives.

On base `747b105cfaf6c50aa03cee8294a3fff4d08da650`, the Architect
reported a scratch check of the three generic package laws and the
private window consequence, rc 0 without a code `Axiom`. A mutation
changing one helper's `Inl` to `Inr` was kernel-refused. Those are
AC-0 proof-development observations, not themselves §4 catalog delivery
or execution of a shipped seed. The three §4 laws are now checked
catalog exports. A package-owned test runs the unchanged seed's positive
client and its paired refusal on the same fixtures and measures the exact
`trusted_base()` identity sets before and after loading EventTrace. That
package test is not official conformance-runner execution, CI,
publication, landing or a landed trust measurement.
