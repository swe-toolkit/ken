# Finite event traces over natural times

> **Status: DRAFT v0 (SPEC-TIME-EVENT-TRACE-CONTRACT §1;
> SPEC-TIME-WINDOW-MEMBERSHIP §2;
> SPEC-TIME-WINDOW-MERGE-DISTRIBUTION §3;
> SPEC-TIME-MERGE-MEMBERSHIP §4;
> SPEC-TIME-WINDOW-COMPOSITION §5;
> SPEC-TIME-MERGE-ASSOCIATIVITY §6;
> SPEC-TIME-WINDOW-ADJACENT-SPLIT §7;
> SPEC-TIME-WINDOW-APPEND-DISTRIBUTION §8;
> SPEC-TIME-INVERTED-WINDOW-EMPTY §9).** The §1–2 EventTrace
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
> Section 5 specifies one unconditional window-composition equality
> and two merge-empty unit laws. Architect's D0 on `98970bde7`
> checked all three generic proofs, owned Transparent exports and
> scratch trusted identities 114 → 114 without a new `Axiom`.
> Section 5's three generic proofs are checked catalog exports on
> main. A targeted package test runs the authored accepted
> client and paired refused union-bounds claim against those exports;
> the exact package-local trusted identity set stays 114 → 114.
> This does not establish official conformance-runner execution, CI,
> publication, landing or a landed trust measurement.
> Section 6's unconditional structural merge-associativity law and its
> private proof helpers are on main; earlier candidate-local checks
> alone were not official conformance or landed trust. Section 7
> specifies a premise-bearing split of an ordered trace across adjacent
> half-open windows. Architect's isolated D0 on `d94af9ee5` checked
> its generic proof with no new `Axiom` and equal scratch trusted
> GlobalIds (114 → 114). The §7 catalog export was absent on the
> spec author's base. This assembled candidate adds the checked law
> and a package-owned test of the complete accepted seed and isolated
> swapped refusal; that test is not official conformance-runner
> execution, CI, publication, landing or landed trust. Section 8
> specifies a premise-free structural append/window law over arbitrary
> traces and bounds. Architect's D0 on `70982b77e` checked the generic
> Derived prerequisite and time proof as scratch design evidence only;
> the separately owned catalog, seed and test need exact-tip review in
> the assembled candidate. No official conformance-runner execution,
> CI, publication, landing or landed trust follows from this spec turn.
> Section 9 states a public empty-window law for arbitrary traces whenever
> `hi ≤ lo`, including equal bounds. Architect's isolated AC-0 on
> `f97544426` checked the generic proof with rc 0, no `Axiom`, and
> identical scratch trusted GlobalId sets 114 → 114. The assembled
> candidate carries the §9 catalog export, its package test and the
> conformance seed; none of these establishes official conformance-runner
> execution, CI, publication, landing or landed trust.

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
laws, and §3 states distribution under ordered-input premises. They
do not give structural window/merge distribution on unordered inputs.
Section 6 separately states unconditional structural **merge
associativity**, and §7 specifies splitting an ordered windowed trace
at an intermediate timestamp; neither is an additional §1 law. The
count law alone does not choose an order for equal timestamps;
left-biased output is fixed by §1.2's definition.

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
checks do not establish the generic quantified law. Section 7's
split of **one** ordered trace at an intermediate bound does not
weaken this section's **two** ordered-input premises for merging:
the unordered `xs=[5,1]` counterexample above is unaffected by
partitioning one trace into adjacent windows.

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
package acceptance; CI and a landed trust measurement remain separate
measurements.

The catalog export makes the existing generic `filter_starts_after`
proof public in `Data.Collections.Derived`. `Data.Time.EventTrace`
reaches it by adding the name to its existing
`import Data.Collections.Derived` selection, not by adding a new
import declaration. No duplicate local proof or new ambient name is
specified. The new merge/window proof helpers remain private **at §3**.
Section 5 promotes the existing private `trace_merge_nil_right` helper
as one of its two public merge-unit laws, with the same name and proof;
no second declaration is added. The other merge/window proof helpers
remain private. The only new §3 public law is `trace_window_merge`.

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
catalog exports. A package-owned test runs the corrected seed's positive
client and its paired refusal on the same fixtures and measures the exact
`trusted_base()` identity sets before and after loading EventTrace. That
package test is not official conformance-runner execution, CI,
publication, landing or a landed trust measurement.

## 5. Composition of windows and empty merge units

`Data.Time.EventTrace` must export **three checked public laws** with
exactly the following types. The first two bounds describe the inner
window, and the next two describe the outer window:

```ken
pub theorem trace_window_window
  (p : Type) (lo1 : Nat) (hi1 : Nat) (lo2 : Nat) (hi2 : Nat)
  (tr : List (Event p)) :
  Equal (List (Event p))
    (trace_window p lo2 hi2 (trace_window p lo1 hi1 tr))
    (trace_window p (max lo1 lo2) (min hi1 hi2) tr)

pub theorem trace_merge_nil_left
  (p : Type) (ys : List (Event p)) :
  Equal (List (Event p)) (trace_merge p (Nil (Event p)) ys) ys

pub theorem trace_merge_nil_right
  (p : Type) (xs : List (Event p)) :
  Equal (List (Event p)) (trace_merge p xs (Nil (Event p))) xs
```

Window composition is **one structural list equality**: a window
`[lo1, hi1)` followed by `[lo2, hi2)` keeps exactly the events in
`[max lo1 lo2, min hi1 hi2)`, each unchanged and in its original
order. The two sides filter the **same list**; neither needs
`time_ordered`, a payload equality decision, a validity proof for
the bounds, or §3's two ordered-merge premises. The equality holds
also for disjoint or inverted windows: if the intersected lower
bound meets or exceeds its upper bound, both sides are empty by
§1.2. This is not the weaker pair of a commutation law and an
idempotence law, and it does not require public max/min
commutativity or idempotence. `max` and `min` are the existing
`Data.Numeric.Nat.Order` exports, which the catalog module must
import alongside its existing `lt_nat` import.

The two unit laws make §1.2's empty-input behavior explicit as
checked equalities of **lists of stored events in order**, with no
ordering premise. `trace_merge_nil_left` is definitionally closed
by `Refl`; `trace_merge_nil_right` **promotes the existing private
§3 helper** with that exact name and argument order, retaining its
checked proof rather than introducing another declaration. On an
open `xs`, structural `Refl` does not close the right-unit type;
its existing inductive proof is essential. Neither law changes
`trace_merge` or chooses a new tie or payload ordering rule.

The new proof tools `leq_nat_max_split`, `lt_nat_min_split`,
`bool_and_interchange`, `event_in_window_inter`, `window_keep`,
`window_filter_fuse_cons` and `window_filter_fuse` stay **private**
to EventTrace. They are not additional public laws or exports. If
the generic Nat split or filter-fusion tools later gain a public
use, their natural homes are `Data.Numeric.Nat.Order` and
`Data.Collections.Derived`, respectively, not a second EventTrace
API. Other §1–4 laws and operations are unchanged.

On the landed ancestor `98970bde7`, Architect's isolated D0 checked
all three generic proof bodies, their owner-local Transparent
identities, a generic public-only client and six private import
refusals, with no new `Axiom` and identical `trusted_base()` sets
114 → 114. Closed D0 clients accepted a `[1,6)` then `[3,9)`
window over events at times 1, 4 and 7, yielding only the time-4
event on both sides, and both merge-unit instances. They rejected
the union-bounds proposition over that same trace while accepting
closed exact-value controls. That is **scratch D0** evidence, not
execution of the authored §5 seed or of checked catalog exports.
The landed catalog promotes the original `trace_merge_nil_right` proof and
updates its privacy-pin consumer. Its package test root-loads all three
owner-local Transparent exports, checks their quantified types, and runs
the authored accepted client and same-input union-bounds refusal. The
exact trusted-identity set is equal before and after loading EventTrace
(114 → 114); its imported providers retain their inherited trust.
The package test is not official conformance-runner execution, CI,
publication, landing or a landed trust measurement.

## 6. Structural associativity of left-biased trace merge

> **Section 6 status: DRAFT v0 (SPEC-TIME-MERGE-ASSOCIATIVITY).**
> Architect checked the generic proof in isolated D0 on `df0b57c98`,
> with `ken check` and `ken fmt --check` rc 0, no added `Axiom`,
> two targeted tests passing, and identical scratch `trusted_base()`
> GlobalIds (114 → 114). The catalog export and closed seed are now on
> main. The earlier candidate-local checks did not establish official
> conformance-runner execution, CI, publication, landing or landed
> trust.

`Data.Time.EventTrace` must export one Transparent checked theorem,
with exactly this name, binder order and structural list equality:

```ken
pub theorem trace_merge_assoc
      (p : Type) (xs : List (Event p)) (ys : List (Event p)) (zs : List (Event p))
    : Equal (List (Event p))
        (trace_merge p (trace_merge p xs ys) zs)
        (trace_merge p xs (trace_merge p ys zs))
```

The law holds for **every** payload type and all three finite event
lists, including unordered inputs. It has **no `time_ordered` premise**;
that premise belongs to the different §3 structural
**window/merge-distribution** law. `trace_merge` does not first sort
or reject an unordered input. At each comparison, both groupings
select the same event as the next head by §1.2's total natural-time
order and left-biased tie rule, retaining each event and its payload
without respelling it. This is equality of the resulting **lists of
stored events in order**, not mere membership, count, or equality
of timestamps. It neither asserts commutativity nor licenses
swapping the two operands of an outer merge.

The public theorem leads seven **private** proof helpers in the
catalog module: `trace_merge_assoc_cons`,
`trace_merge_assoc_cons_cons`, `trace_merge_assoc_step`,
`trace_merge_assoc_left_first`, `trace_merge_assoc_right_first`,
`trace_merge_assoc_right_last`, and `event_before_skip`. They reuse
the now-public `trace_merge_nil_right` plus three private lemmas:
`trace_merge_cons_left`, `trace_merge_cons_right` and
`trace_head_after_decide`. The other private order lemmas
`trace_merge_starts_after`, `event_before_trans` and
`event_before_flip` remain available to their proofs. The dependent case
splits bind each Boolean comparison result and its equality before
matching, rather than assuming a computed comparison refines a
convertible expression. There is no new import, Rational or Nat
export, axiom, primitive, kernel or surface rule. All §1–§5 laws
and operations retain their signatures and bodies.

The closed seed uses three interleaved two-event traces. Two events
from different inputs, `a1` and `c0`, tie at time 4. Both groupings
reduce independently to the exact list
`[a0, b0, a1, c0, b1, c1]`, and applying the public theorem yields
their equality. Swapping the operands of the **outer** merge instead
puts `c0` before `a1`, so the single isolated false equality refuses.
The same fixture with only `c0`'s timestamp changed to 3 admits
that swapped equality; this D0 no-tie control shows that the
time-4 tie is the discriminating axis, not a generic `Refl` refusal.
Neither closed calculation nor that refusal establishes the generic
law; the checked proof body is the catalog obligation. The
Architect's scratch checks and the former candidate's local checks
remain distinct from shipped and official evidence.

## 7. Adjacent half-open window split

> **Section 7 status: DRAFT v0
> (SPEC-TIME-WINDOW-ADJACENT-SPLIT).** Architect's isolated D0 on
> landed base `d94af9ee53389c12582baced90c0b71620300563`
> checked the generic owner-local Transparent proof, a public client,
> and closed true/false controls, with no new `Axiom` and unchanged
> scratch `trusted_base()` GlobalIds (114 → 114). The §7 catalog
> export was absent on that authoring base. The assembled candidate
> now exports the checked law and root-loads the complete accepted
> seed and isolated refusal in a package-owned test. Neither D0 nor
> this package test establishes official conformance-runner execution,
> CI, publication, landing or landed trust.

### 7.1 Public checked law and necessary premises

`Data.Time.EventTrace` must export one public Transparent theorem, after
`trace_window_window` and before `trace_merge_nil_left`, with precisely
the following name, argument order, three premises and structural list
equality:

```ken
pub theorem trace_window_split
      (p : Type) (lo : Nat) (mid : Nat) (hi : Nat) (tr : List (Event p))
    : time_ordered p tr
      → Equal Bool (leq_nat lo mid) True
      → Equal Bool (leq_nat mid hi) True
      → Equal
        (List (Event p))
        (trace_window p lo hi tr)
        (list_append (Event p) (trace_window p lo mid tr) (trace_window p mid hi tr))
```

Here `list_append` is the existing `Data.Collections.Derived`
operation, not a new trace primitive. EventTrace's **named** import
from `Data.Collections.Derived` on the authoring base omits it; the
catalog half must add **`list_append` to that existing import list**,
not assume a whole-module import and not add another declaration.
The seed client also selectively imports
`Data.Collections.Derived (list_append)` to state the equality.
All §1–§6 operations and laws retain their types and bodies.

The single ordered input makes concatenation of the two window
results reproduce the original trace order. The range assumptions
require `lo ≤ mid ≤ hi` as the two displayed `Equal Bool` premises.
Window `[lo,mid)` excludes events stamped exactly `mid`, whereas
`[mid,hi)` includes them; `[lo,hi)` includes `lo` and excludes `hi`.
The equality is **of lists of stored events in order**, including
payloads and repeated timestamps, not merely membership or count.
It holds when a subwindow is empty (for example `lo = mid` or
`mid = hi`) under the shown premises. It does not sort an unordered
trace, imply a premise-free split, or relax the two ordered-input
requirements of §3's distinct window/merge law.

Each premise matters independently; Architect's D0 checked that the
corresponding law with **that premise dropped** implies `Bottom` in
a closed instance:

- Without `time_ordered`, `tr=[e5,e1]` at times 5 and 1 with
  `lo=0`, `mid=3`, `hi=10` gives whole window `[e5,e1]` but
  appended subwindows `[e1,e5]`. A first-event observer
  distinguishes the lists.
- Without `lo ≤ mid`, `tr=[e1]` with `lo=2`, `mid=0`, `hi=5`
  gives an empty whole window but the event appears in `[mid,hi)`.
- Without `mid ≤ hi`, `tr=[e4]` with `lo=0`, `mid=5`, `hi=3`
  gives an empty whole window but the event appears in `[lo,mid)`.

These closed counterexamples show why the premises are retained;
they do not by themselves prove the generic conditional theorem.
The premise spelling follows §2's `Equal Bool (leq_nat _ _) True`,
not a new `IsTrue` convention for EventTrace.

### 7.2 Derivation and evidence boundary

The checked proof proceeds by induction on `tr`. At a nonempty
trace, it case-splits on a **bound Boolean decision** for whether
the head time is below `mid`, carrying an equality between that
decision and `lt_nat (event_time p h) mid`. It does not directly
match a computed scrutinee and assume that its convertible forms
are refined. Below `mid`, the head occurs in the whole window
exactly when it occurs in the left subwindow and never in the right;
the private `window_keep_append` combines that fact with the
induction hypothesis. At or above `mid`, ordering propagates the
lower bound down the tail: the left subwindow is empty by
`trace_window_past` and the whole window equals the right by
`trace_window_raise`. This branch does not use the induction
hypothesis.

Nine new owner-local Transparent helpers stay **private**:
`trace_window_split_cons`, `trace_window_split_below`,
`trace_window_split_above`, `trace_window_past`,
`trace_window_raise`, `event_in_window_past`,
`event_in_window_raise`, `lt_nat_reverse_leq_false`, and
`window_keep_append`. The proof also reuses private `window_keep`.
None becomes a selective-import API or a new public Nat lemma.
There is no new axiom, postulate, primitive, kernel rule or surface
feature.

In Architect's closed D0 fixture, the ordered trace `[e1,e3,e7]`
has timestamps 1, 3 and 7, with `lo=1`, `mid=3`, `hi=9`.
The whole window is `[e1,e3,e7]`, the left is `[e1]`, and
the right is `[e3,e7]`; a true structural split is independently
checked. Swapping the two **nonempty, distinct** subwindows is
refused at its own `Refl` after a fresh accepted prefix. An
independent closed value check establishes that the time-3 event
is **not** in the left `[1,3)` window, while the true split is
accepted in the same proof shape. A closed fixture illustrates the
boundary, not the generic law; the D0 patch and its seed-shaped
client did not establish execution of the assembled catalog export
or official conformance. The separately owned package test now reads
the complete seed, checks the export and private-helper boundary,
and compares exact trusted identity sets; those candidate-local checks
remain distinct from CI, publication and landed trust.

## 8. Windowing distributes over trace append

> **Section 8 status: DRAFT v0
> (SPEC-TIME-WINDOW-APPEND-DISTRIBUTION).** Architect's isolated D0 on
> main base `70982b77eb020e920dfc4406c025504de9e2ebb9` checked the
> generic proof and public client in scratch, without a new `Axiom` or
> change to scratch `trusted_base()` GlobalIds (114 → 114). The Derived
> and EventTrace catalog additions, conformance seed and package test
> are separately owned parts of the assembled candidate; this spec-only
> turn does not establish their delivery, official conformance-runner
> execution, CI, publication, landing or landed trust.

### 8.1 Public law and generic prerequisite

`Data.Time.EventTrace` exports one new public Transparent theorem,
placed immediately after `trace_window_split` and before
`trace_merge_nil_left`. Its arguments follow `trace_window p lo hi`
and then supply the two traces; no premise restricts their ordering
or the bounds:

```ken
pub theorem trace_window_append
      (p : Type) (lo : Nat) (hi : Nat) (xs : List (Event p)) (ys : List (Event p))
    : Equal
        (List (Event p))
        (trace_window p lo hi (list_append (Event p) xs ys))
        (list_append (Event p) (trace_window p lo hi xs) (trace_window p lo hi ys))
```

The equality is structural equality of the lists of stored events in
order, including their payloads and duplicate timestamps. It holds
for arbitrary `xs` and `ys`, even when either trace or their append
is not `time_ordered`. It also holds for empty or inverted windows:
when `lo ≥ hi`, both sides are empty. The law does not assert that
windowing sorts events, preserves the length of an input trace, or
commutes with `trace_merge` without §3's ordered-input premises.
Unlike §7's adjacent split, it needs no relationship among `lo`,
`hi` and the event times.

The proof applies the public generic list law `filter::append` from
`Data.Collections.Derived` once, with element type `Event p` and
predicate `event_in_window p lo hi`. The EventTrace body is exactly
one application:

```ken
(proof append for filter) (Event p) (event_in_window p lo hi) xs ys
```

The prerequisite is exported as `pub proof append for filter`,
immediately after Derived's `filter`, with arguments
`(a : Type) (keep : a → Bool) (xs : List a) (ys : List a)`:

```ken
Equal
  (List a)
  (filter a keep (list_append a xs ys))
  (list_append a (filter a keep xs) (filter a keep ys))
```

It is proved by induction on the first list, with a private
`filter_keep_step_append` helper in Derived to carry the head
predicate decision through the append. EventTrace adds no helper
or import: its existing import of `filter` brings its attached
proof. `filter::append` is a Derived export, not a second §69 public
law. The existing §7-private `window_keep` and `window_keep_append`
remain as they are. All §1–§7 operation and law types are unchanged;
no new axiom, primitive, kernel rule or effect is needed.

### 8.2 Closed controls and evidence boundary

The assigned seed uses `[e5,e1]` for `xs` (unordered), `[e3,e8]`
for `ys`, and the half-open window `[2,6)`. With distinct Bool
payloads, windowing the two traces retains `[e5]` and `[e3]`, and
windowing their append retains `[e5,e3]`; `Refl` checks each closed
calculation. Applying `trace_window_append` agrees with an independent
same-shape `Refl` control. An independently defined first-event time
observer confirms that the first event of the swapped append
`list_append (Event Bool) (trace_window Bool 2 6 ys)
(trace_window Bool 2 6 xs)` is time 3, not time 5. The false swapped
claim is refused in isolation after a fresh accepted prefix, both
with `Refl` and with the true law as its body. The refused `Refl`
diagnostic's wording is not itself the oracle; refusal of that
well-formed false equality is. These closed checks rule out a
swapped-order reading but do not prove the generic law.

Architect's D0 checked the generic Derived and EventTrace proof bodies,
their owner-local Transparent exports, an accepted public client,
and refusal of an attempted import of the private helper. Its
candidate-local package checks include the Derived export inventory
fix needed to import `list_append` when probing `filter::append`.
Those are scratch findings, not a checked-in seed run or official
conformance. The separate catalog and conformance owners must deliver
and test the same contract on the assembled exact tip before their
respective reviews; no D0 result transfers to CI or landed trust.

## 9. Equal or inverted windows select no events

> **Section 9 status: DRAFT v0 (SPEC-TIME-INVERTED-WINDOW-EMPTY).**
> Architect's isolated AC-0 on
> `f975444269fd5c3863973fcb1fcde3f638cac2d6` checked the generic
> proof and closed controls with rc 0, no `Axiom`, and unchanged
> scratch `trusted_base()` GlobalId identities (114 → 114). This
> assembled candidate has the checked `trace_window_inverted_empty`
> export with its private `event_in_window_inverted` helper, the
> package-owned `cat_time_inverted_window_empty` test and the authored
> `conformance/stdlib/time-and-events/seed-inverted-window-empty.md`.
> None of these establishes official conformance-runner execution, CI,
> publication, landing or landed trust.

`Data.Time.EventTrace` must export one public Transparent theorem
immediately after `trace_window_append` and before `trace_merge_nil_left`.
It follows `trace_window`'s argument order, then takes exactly the
bound-order premise:

```ken
pub theorem trace_window_inverted_empty
      (p : Type) (lo : Nat) (hi : Nat) (tr : List (Event p))
    : Equal Bool (leq_nat hi lo) True
      → Equal (List (Event p)) (trace_window p lo hi tr) (Nil (Event p))
```

The premise `hi ≤ lo` includes equal bounds as well as strictly
inverted ones. The result is structural equality of event lists: an
empty half-open window selects no event in **any** trace, including an
unordered trace and one with repeated timestamps or arbitrary payloads.
There is no `time_ordered p tr` or other premise. This is the checked
form of §1.2's total empty-window behavior, not a law about only the
ordered traces accepted by §7's adjacent split.

The proof descends through the list. At each event time `t`, if
`lo ≤ t`, transitivity with `hi ≤ lo` gives `hi ≤ t`, so the existing
private `event_in_window_past` makes `event_in_window` false. If
`lo ≤ t` is false, `bool_and False _` already makes it false.
The Boolean decision is bound with its equality **before** matching,
so the match refines the actual `leq_nat lo t`; matching a computed
expression alone does not supply that equality. Existing private
`window_keep` then drops the event, and induction gives the empty
windowed tail. One new helper, `event_in_window_inverted`, stays private
beside `event_in_window_past`; `event_in_window_past` and `window_keep`
stay private too. The ordered-trace-only `trace_window_past` is not
needed. No new import or change to any §1–§8 operation or law type is
required; no Axiom, kernel or surface change is admitted.

The separate CV seed will apply the law to `[4,4)` and `[7,4)` over a
trace with events at times 1, 4 and 7, and also to the unordered
permutation at `[7,4)`. The value `trace_window Bool 4 4 tr = Nil`
is a true `Equal` goal discharged by `Proved`, **not** `Refl`; the
accepted nonempty `[4,5)` control keeps the time-4 event and uses
`Refl`. An isolated refused `[4,5)`-equals-`Nil` claim must use the
same well-formed `Proved` proof form as the true empty-window value:
otherwise `Refl` would reject even a true `Nil` goal and conceal the
false-window boundary. A separate refused law application at `[4,5)`
needs the absent `leq_nat 5 4 = True` premise. Architect observed these
accept/refuse pairs in scratch D0; this Spec Author turn does not run
or deliver the §9 seed or catalog theorem.
