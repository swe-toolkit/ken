# Finite event traces over natural times

> **Status: DRAFT v0 (SPEC-TIME-EVENT-TRACE-CONTRACT §1;
> SPEC-TIME-WINDOW-MEMBERSHIP §2;
> SPEC-TIME-WINDOW-MERGE-DISTRIBUTION §3;
> SPEC-TIME-MERGE-MEMBERSHIP §4;
> SPEC-TIME-WINDOW-COMPOSITION §5;
> SPEC-TIME-MERGE-ASSOCIATIVITY §6;
> SPEC-TIME-WINDOW-ADJACENT-SPLIT §7;
> SPEC-TIME-WINDOW-APPEND-DISTRIBUTION §8;
> SPEC-TIME-INVERTED-WINDOW-EMPTY §9;
> SPEC-TIME-WINDOW-ALL-IDENTITY §10;
> SPEC-TIME-WINDOW-COMMUTATION §11;
> SPEC-TIME-WINDOW-LENGTH-BOUND §12;
> SPEC-TIME-MERGE-SEPARATED-APPEND §13;
> SPEC-TIME-WINDOW-NONE-EMPTY §14;
> SPEC-TIME-TRACE-MERGE-ALL §15).** The §1–2 EventTrace
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
> alone were not official conformance or landed trust. Section 7's
> checked adjacent-window split, seed and package-owned test are now on
> main. Its isolated D0 had equal scratch trusted identities 114 → 114;
> earlier package checks are not official conformance-runner execution
> or a measured landed trust result. Section 8's premise-free
> append/window law, Derived prerequisite, seed and package-owned test
> are now on main. Its D0 was scratch evidence, not an official runner
> result or landed trust measurement. Section 9's checked empty-window
> law for `hi ≤ lo`, seed and package-owned test are now on main.
> Architect AC-0 checked its generic proof with equal scratch trusted
> identities 114 → 114; its candidate-local checks do not establish
> official conformance-runner execution or measured landed trust.
> Section 10 specifies identity of a window covering every member of
> a trace. Its checked catalog theorem, authored CV seed and package-
> owned test are present on this base; Architect's earlier AC-0 was
> scratch evidence with equal trusted identities 114 → 114. Neither
> scratch nor package testing establishes an official conformance-runner
> result or measured landed trust. Section 11 states one premise-free
> commutation law for two windows. Architect AC-0 on `80525e894` checked
> its proof with rc 0, no new `Axiom`, and a discriminating closed
> control; the trusted identity set was **not measured** in that D0.
> The historical Spec-only child `096d6f5e1` stated the law without a
> catalog export, package test or CV seed. Foundation reports that its
> assembled working tree includes all three: the named package test
> passed 2/2, checking the actual CV seed's accepted and refused fences,
> and the full `trusted_base()` GlobalId identity set was 114 → 114
> before and after loading EventTrace. This is working-tree package-test
> evidence, not an exact committed-tip gate or official conformance-
> runner execution; it does not establish CI, publication, landing or
> measured landed trust.

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
lazy streams, strictly increasing traces, deduplication and time
over an arbitrary `Ord` are deferred too. Finite-list k-way merge
is specified separately in §15, not by the three §1 laws.

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
§1.2. It is stronger than a commutation law and an idempotence law, and it
does not itself require public max/min commutativity or idempotence;
§11 derives window commutation from it with Data.Numeric.Nat.Order's
`comm for max` and `comm for min`. `max` and `min` are the existing
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
Section 12 states the length bound that does hold: windowing never
adds events. Unlike §7's adjacent split, this equality needs no
relationship among `lo`, `hi` and the event times.

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

## 10. A window covering every event leaves the trace unchanged

> **Section 10 status: DRAFT v0
> (SPEC-TIME-WINDOW-ALL-IDENTITY).** Architect AC-0 on exact base
> `289a4bb90` checked the generic proof with `ken check` and
> `ken fmt --check` rc 0, no new `Axiom`, and equal scratch
> `trusted_base()` identity sets 114 → 114. This is isolated design
> evidence, not a candidate catalog check or landed trust result.
> The checked §10 theorem, CV-authored seed and package-owned test are
> present on this base. Candidate package testing does not establish
> official conformance-runner execution or a measured landed trust
> result; it is separate from the earlier scratch AC-0.

`Data.Time.EventTrace` must export one public Transparent theorem
immediately after `trace_window_inverted_empty` and before
`trace_merge_nil_left`. The argument order follows `trace_window`.
Its only premise says that **each member of the original list**
satisfies the actual window predicate:

```ken
pub theorem trace_window_all_identity
      (p : Type) (lo : Nat) (hi : Nat) (tr : List (Event p))
    : ((e : Event p) → list_elem (Event p) e tr → Equal Bool (event_in_window p lo hi e) True)
      → Equal (List (Event p)) (trace_window p lo hi tr) tr
```

The result is structural equality of the **lists**, including event
order and duplicate occurrences, not merely mutual membership or
equality of the selected event set. For arbitrary `p`, bounds and
trace, the premise covers every event represented in `tr`, and
`trace_window` retains each event unchanged and in its original
position. The empty trace satisfies the law as well. No
`time_ordered p tr`, `lo ≤ hi`, `hi ≤ lo`, uniqueness, payload
comparison or other condition is required. Section 9's separate
claim remains: equal or inverted bounds discard every event from any
trace.

The checked proof inducts on the **written** `tr`. In a `Cons` case,
the public `list_elem_head` from `Data.Collections.Derived` supplies
membership for the head, so the function premise gives
`Equal Bool (event_in_window p lo hi h) True`. Congruence through the
existing private `window_keep` rewrites the head decision to keep
that event. For the tail, `list_elem_later` lifts a tail membership
to a membership of the original trace; this restricts the same
premise to the recursive call, and congruence under `Cons` assembles
the full list equality. Neither the two Derived lemmas nor private
`window_keep` is re-exported from `EventTrace`; no new helper,
`Axiom`, kernel rule, surface form or §1–§9 operation or law type is
needed. The later Foundation catalog child adds the necessary
`list_elem_head` import and the corresponding §2 package-lede
sentence; this spec-only child does neither.

The later CV seed can use `tr=[(4,T),(6,F),(5,T)]`. At `[4,8)`, a
closed discharge of every member's `event_in_window` predicate
allows the generic law and the true equality
`trace_window Bool 4 8 tr = tr`, checked by `Refl`. At `[4,6)`,
`trace_window` instead keeps `[(4,T),(5,T)]`, also a true `Refl`
control; the isolated false claim `trace_window Bool 4 6 tr = tr`
must refuse at its own `Refl`. The true `[4,8)` equality is an
accepted **same-form** control, so the refusal pins the time-6
boundary rather than an invalid equality proof form. These closed
cases do not replace the generic proof. No seed is authored or run
in this spec-only turn.

## 11. Commutation of two time windows

> **Section 11 status: DRAFT v0
> (SPEC-TIME-WINDOW-COMMUTATION).** Architect AC-0 on `80525e894`
> checked the proposed generic proof and closed discriminator with
> `ken check` and `ken fmt --check` rc 0 for the accepted forms, no
> new `Axiom`, and a wrong-form refusal at its own `Refl`. The D0
> trusted-identity set was **not measured** in D0. The historical
> Spec-only child `096d6f5e1` stated the law type but lacked the §11
> catalog theorem, package test and CV seed. Foundation reports its
> assembled working tree now has these three artifacts. Its named
> `cat_time_window_commutation` package test passed 2/2, including the
> actual CV seed's accepted fence and isolated own-span `Refl` refusal;
> the full `trusted_base()` GlobalId set was unchanged at 114 → 114
> across provider and EventTrace loading. This is Foundation's reported
> working-tree evidence, not a committed-tip gate, official
> conformance-runner execution, CI, publication, landing or measured
> landed trust.

`Data.Time.EventTrace` must export one public Transparent theorem
immediately after `trace_window_window` and before
`trace_window_split`, with the same argument order as the window-
composition law. It has no premise:

```ken
pub theorem trace_window_commute
      (p : Type) (lo1 : Nat) (hi1 : Nat) (lo2 : Nat) (hi2 : Nat) (tr : List (Event p))
    : Equal
        (List (Event p))
        (trace_window p lo2 hi2 (trace_window p lo1 hi1 tr))
        (trace_window p lo1 hi1 (trace_window p lo2 hi2 tr))
```

The left side first applies the inner window `[lo1,hi1)` and then the
outer window `[lo2,hi2)`. The right side reverses that order. Both
sides retain the **same original list elements in the same order**;
this is structural `Equal (List (Event p))`, not an equality of
membership predicates. It holds for arbitrary `p`, trace and bounds,
including overlapping, disjoint, equal and inverted windows. Neither
`time_ordered`, a bounds-order premise nor a payload equality decision
is needed. The theorem changes none of the §§1–10 operation or law
types.

The checked proof composes the public `trace_window_window` law for
the first order with the symmetric instance for the reversed order.
Between the two fused windows, `cong` transports Nat.Order's public
`comm for max` across the lower bound and `comm for min` across the
upper bound; `trans` composes those steps and `sym` reverses the
second fusion equality. `max`, `min`, `cong`, `sym` and `trans` are
already imported by the catalog module. No new import, private helper,
axiom, kernel rule or surface form is required. Unlike §5's stronger
fusion law, commutation alone does not identify the fused bounds.

The CV-owned seed can use `[1,6)` and `[3,9)` on a trace containing
`e1`, `e4` and `e7`: `e1` lies only in the first window, `e7`
only in the second, and `e4` in both. Both window orders yield
`[e4]` by `Refl`, while the outer `[3,9)` window alone yields
`[e4,e7]`. The accepted client applies `trace_window_commute` to
this trace; a same-form `Refl` control equates the two compositions.
The isolated false claim replaces the right-hand composition with
the outer window alone, so it would improperly retain `e7`; its
`Refl` must refuse at the claim's own span. Architect AC-0 measured
accepted control rc 0 and refusal rc 1. `Proved` refuses **both** and
is not a discriminator for these list equalities. These closed
observations distinguish the dropped inner window; they do not
replace the generic checked proof or constitute official conformance
execution. The CV seed and committed-tip package test remain separate
from this Spec-only child.

## 12. A time window does not increase trace length

> **Section 12 status: DRAFT v0
> (SPEC-TIME-WINDOW-LENGTH-BOUND).** Architect AC-0 on the detached
> `dff99e382` scratch base checked the generic Derived proof, the
> EventTrace instance and a closed discriminator. D0 and its corrected
> harness run D0b measured the same `trusted_base()` GlobalId identity
> set at all three points, 114 → 114 → 114, in one environment;
> Derived's `Axiom` count stayed 7 → 7 and EventTrace's 1 → 1.
> These are scratch design results, not an assembled candidate gate,
> official conformance execution, CI or measured landed trust. The
> catalog export, named package test and CV seed require separate
> delivery and verification; this Spec-only section does not attest
> their assembled execution.

`Data.Time.EventTrace` must export one public Transparent theorem
immediately after `trace_window_all_identity`, with the same argument
order as `trace_window`. It has no premise beyond the trace and bounds:

```ken
pub theorem trace_window_length_bound
      (p : Type) (lo : Nat) (hi : Nat) (tr : List (Event p))
    : IsTrue (leq_nat (length (Event p) (trace_window p lo hi tr)) (length (Event p) tr))
```

The result compares natural lengths, not structural equality of the
lists or preservation of length. It holds for every `p`, every trace
whether ordered or not, and every pair of bounds, including empty and
inverted windows. A window discards zero or more events but cannot
create one. The theorem changes none of the §§1–11 operations or law
types; it adds no ordered-trace or valid-window premise.

The generic checked proof is `Data.Collections.Derived`'s public
`filter::length`, exported as `pub proof length for filter` after
`filter::append`. It establishes
`IsTrue (leq_nat (length a (filter a keep xs)) (length a xs))` for any
`a`, predicate and list. Its private `filter_keep_step_length` uses
the successor/successor reduction of `leq_nat` when the head is kept
and Nat.Order's public `leq_nat_weaken_right` when it is dropped.
The EventTrace body applies that proof once:

```ken
(proof length for filter) (Event p) (event_in_window p lo hi) tr
```

`filter::length` is a Derived export, not another §69 law. The
private Graphs `filter_length_cons` equation stays unchanged: a
length bound cannot replace the equation used by its consumer.
No new `Axiom`, kernel rule or surface form is part of this contract.

The CV-owned closed seed can apply `trace_window_length_bound` to a
trace containing events `e1`, `e4` and `e7` at `[3,9)`; the window
keeps `e4` and `e7`, so the bound reads 2 ≤ 3. Its isolated false
claim reverses the comparison at those bounds (3 ≤ 2), while an
accepted same-form control reverses it at `[0,9)`, where all three
events remain (3 ≤ 3). Both reverse claims must use `Proved`:
Architect's corrected scratch D0b accepted the control and refused
the false claim at its own `KernelRejected TypeMismatch`; `Refl`
refused both and cannot distinguish them. The seed must explicitly
import `IsTrue` and `leq_nat` in its catalog-addressed client.
These observations do not constitute official conformance execution
or substitute for the generic proof or a committed-tip package test.

## 13. A separated merge is an append

> **Section 13 status: DRAFT v0
> (SPEC-TIME-MERGE-SEPARATED-APPEND).** Architect AC-0
> `evt_7nw2d96tvqzv1` ruled D0-CLEAN on isolated scratch at
> `a3f9b487b2abdf3768214e88c3d548545ce74a13`. Its generic
> harness passed 2/2; the before/after `trusted_base()` identity
> sets contained the same 114 GlobalIds, with none added or removed.
> Eleven named time suites passed 28/28, and strict resolution passed
> 7/7 with the scratch-only reject fixtures removed from its census
> population; the original unexcluded run was 6/7. No `Axiom`
> declaration was added. These are scratch design results, not a
> Spec-child or assembled-candidate gate, official conformance
> execution, CI, publication, landing or measured landed trust.

`Data.Time.EventTrace` must export one public Transparent checked law
immediately after `trace_merge_nil_right` and before the existing
private `trace_merge_cons_left`. Its exact public type is:

```ken
pub theorem trace_merge_separated
      (p : Type) (xs : List (Event p)) (ys : List (Event p))
    : ((x : Event p)
        → list_elem (Event p) x xs
        → (y : Event p)
        → list_elem (Event p) y ys
        → Equal Bool (event_before p x y) True)
      → Equal (List (Event p)) (trace_merge p xs ys) (list_append (Event p) xs ys)
```

Separation is a **membership-quantified function**: for every member
`x` of `xs` and every member `y` of `ys`, `event_before p x y`
returns `True`. By §1.1, this compares event times with `leq_nat`;
equal timestamps are permitted and §1.2's left tie rule retains the
left event first. The premise does not say either input trace is
`time_ordered`, and the theorem holds for unordered traces as well.
If a side is empty, the separation function is vacuously supplied.
The result is structural equality of lists, not mere membership or
an equality up to event permutation.

This refines §4's statement that merge is not append *in general*:
when all left events precede or tie all right events, the merge takes
the left trace in its original order before the right trace, so it
**is** append. Section 4's text and its three membership laws stay
unchanged. Neither the converse of the separation law nor a
`trace_merge_starts_before` head-form law is specified; the latter
was a scratch probe, not a public export or a retained private law.

The checked body inducts on `xs`, using private
`trace_merge_cons_left` and a new private
`trace_separated_starts_after` to supply the comparison between the
left head and each right member. The helper follows the public
`trace_merge_separated` declaration and precedes
`trace_merge_cons_left`. The recursive call restricts the separation
premise to the left tail via `list_elem_later`. The body changes no
§1–§12 operation or law type, adds no `Axiom`, kernel rule or surface
form, and requires no `time_ordered` premise or `list_all` bridge.

For an ordered trace split into adjacent windows by §7's
`trace_window_split`, the separated-merge law can rejoin the windows.
A client can establish the quantified premise from
`trace_window_member_sound` and the shared split bound; the checked
D0 client did so over open `p`, `lo`, `mid`, `hi` and `tr`. The CV-owned
closed seed must pair an interleaved append-equation refusal with an
accepted same-form separated control, both using `Refl`. Scratch D0
refused the interleaved claim with E0038 but accepted the separated
control. `Proved` refused even the true separated equation with
K0004, so it cannot distinguish interleaving. The seed and its
package-owned execution are separate deliveries; this Spec-only
section claims neither their execution nor official conformance.

## 14. A window missing every event selects the empty trace

> **Section 14 status: DRAFT v0 (SPEC-TIME-WINDOW-NONE-EMPTY).**
> Architect AC-0 `evt_5z26g2mqnc9p` ruled FEASIBLE from Foundation's isolated
> scratch run on `f3c016f898139a6ab7af0faf6b72bc8ff82ede54`. The generic
> proof and a closed accepted client passed 2/2; the `trusted_base()` GlobalId
> identity sets were equal before and after, 114 → 114. The new law was owned
> and Transparent; the `Axiom` grep found only existing prose, and seven
> neighbouring package tests passed. These are scratch design findings, not
> Spec-child or assembled-candidate tests, official conformance execution,
> CI, publication, landing or measured landed trust.

`Data.Time.EventTrace` must export one public Transparent theorem directly
behind `trace_window_all_identity` and before `trace_window_length_bound`.
The argument order follows `trace_window`. Its sole premise says that **every
member of the original trace** misses the half-open window:

```ken
pub theorem trace_window_none_empty
      (p : Type) (lo : Nat) (hi : Nat) (tr : List (Event p))
    : ((e : Event p) → list_elem (Event p) e tr → Equal Bool (event_in_window p lo hi e) False)
      → Equal (List (Event p)) (trace_window p lo hi tr) (Nil (Event p))
```

The result is structural equality of lists, not merely absence of a selected
member. The law holds for any payload type, trace and bounds, including an
empty trace, unordered or repeated events, and a non-inverted window whose
bounds enclose no event of `tr`. It requires neither `time_ordered`, an order
relation between `lo` and `hi`, uniqueness, nor an additional premise. It is
the miss counterpart of §10's all-members-kept law; unlike §9's inverted-window
law, it does not require the window itself to be empty.

The checked proof inducts on the written `tr`. `list_elem_head` supplies the
head's membership, so the premise makes its `event_in_window` decision
`False`. Congruence through the existing private `window_keep` drops the
head. `list_elem_later` lifts tail membership to the original trace for the
recursive call; the empty tail result then establishes the list equality.
These membership lemmas are existing `Data.Collections.Derived` imports; the
law adds no helper, import, `Axiom`, kernel rule or surface form. No §1–§13
operation or law type changes.

The CV-owned closed seed may use events at times 1 and 5. A membership-quantified
miss for `[2,5)` permits the law to conclude `Nil`, even though the window is
non-inverted. Changing only the lower bound to `[1,5)` makes the time-1 event
fall inside, so that miss premise must refuse; the accepted `[2,5)` application
is the same-form control. Architect's scratch D0 localized the refusal to the
premise theorem, then attributed its failing event through the expected type
and control; it did not show an arm-local diagnostic. The CV seed can separate
per-member miss lemmas to make the time-1 refusal local. Seed authoring and
execution belong to CV and the package owner, not this Spec-only child.

## 15. Merging a finite list of event traces

> **Section 15 status: DRAFT v0
> (SPEC-TIME-TRACE-MERGE-ALL).** Architect's AC-0 on base `ea5842524`
> checked the operation and three public laws in scratch without a
> new `Axiom`: trusted `GlobalId` identity sets stayed 114 → 114,
> public exports grew 30 → 34 and public theorems 21 → 24. The
> Foundation implementation parent `2d278b635` separately contains
> the checked exports and package-owned tests; neither the D0 nor
> that parent is this assembled Spec-and-seed candidate, official
> conformance, CI, publication, landing or measured landed trust.
> Direct operation mutants first broke the ordered provider proof;
> a separate overlay without those proof gates made the exact-output
> V1 control refuse at its own `Refl`. This is not a claim that the
> direct mutation itself reached that client.

`Data.Time.EventTrace` must export this public pure operation directly
after `trace_merge`:

```ken
pub fn trace_merge_all
      (p : Type) (traces : List (List (Event p))) : List (Event p)
```

The definition is a **right fold** of the existing left-biased
`trace_merge`: `trace_merge_all p (Nil (List (Event p)))` computes to
`Nil (Event p)`. The `Cons t rest` case is
`trace_merge p t (trace_merge_all p rest)`. Earlier input traces
win ties against later ones under §1.2's left-biased two-way merge;
no arbitrary `Ord` time, deduplication, stream processing or new
carrier is introduced. The operation is total even for unordered
inputs, but the ordered-result law below requires an explicit
premise for every listed trace.

The following public Transparent checked laws follow the existing
`trace_merge_assoc` proof-helper family, in the order ordered,
complete, sound:

```ken
pub theorem trace_merge_all_ordered
      (p : Type) (traces : List (List (Event p)))
    : ((t : List (Event p)) → list_elem (List (Event p)) t traces → time_ordered p t)
      → time_ordered p (trace_merge_all p traces)

pub theorem trace_merge_all_member_complete
      (p : Type) (e : Event p) (t : List (Event p)) (traces : List (List (Event p)))
    : list_elem (List (Event p)) t traces
      → list_elem (Event p) e t
      → list_elem (Event p) e (trace_merge_all p traces)

pub theorem trace_merge_all_member_sound
      (p : Type) (e : Event p) (traces : List (List (Event p)))
    : list_elem (Event p) e (trace_merge_all p traces)
      → list_elem (Event p) e (list_concat (Event p) traces)
```

The ordered law quantifies over **all** traces appearing in the
outer list; it does not infer that premise from sorted output or
only from the first trace. The empty result is ordered by `Proved`.
Completeness takes both a membership of `t` in the outer list and
a membership of `e` in that trace before concluding membership in
the merged result. Soundness concludes membership in the
**flattened input list**; it does not return a chosen input trace
or an untruncated witness of which occurrence supplied the event.
All membership propositions are the existing truncated `list_elem`,
and the complete proof's `J` transports a trace equality into the
Ω-valued event-membership motive rather than asserting structural
identity of alternative occurrences. The checked laws reuse the
existing two-way merge ordering and membership laws, with no new
public helper, axiom, kernel or surface rule and no change to any
§1–§14 operation or law type.

The operation's right-fold definition and left-bias rule fix the
actual event sequence; the laws alone do not characterize it. In
particular, a constant-`Nil` operation would satisfy the ordered
and sound laws without retaining any input event. The separate
package test must compare an explicit nonempty three-trace result
and a cross-input equal-time tie (V1 and V3) so both the retained
events and the earlier-input tie priority are observed. The
CV-reviewed conformance child separately checks an accepted
three-trace ordering and third-trace membership and pairs an
unordered-input premise refusal with a same-form accepting control.
Closed cases do not replace these generic checked proofs. A
fabricating operation that is still ordered and complete remains a
soundness discriminator to review; no soundness-isolating mutation
was measured by the AC-0 report.
