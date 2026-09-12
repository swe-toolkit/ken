---
id: CAT-PRIORITY-QUEUE
title: "the Band-A frontier-harvest priority-queue node: land Data.Collections.PriorityQueue as one complete computational persistent LEFTIST priority queue -- abstract carrier PriorityQueue k v leq with private ranked nodes, five public ops (empty/insert/find_min/pop_min/merge) over an explicit Ord k with payload separate, one private make_node + one private meld worker reusing leq_nat; a TESTED computational implementation (semantics load-bearing and covered, general kernel proof deferred to CAT-PRIORITY-QUEUE-LAWS), kernel-untouched and Axiom-free; do NOT split carrier/insert from meld/pop"
status: draft
owner: foundation
size: L
gate: none
depends_on: [SPEC-PRIORITY-QUEUE-CONTRACT]
blocks: []
github: null
tier: T1
origin: "Steward cut 2026-09-12 from the Architect priority-queue design ruling (Parts A/C/D: evt_3vfd1feghmm2c representation+interface, evt_15etn5hm102n2 decomposition+proof-posture+discriminators), grounded at main 1691160dd. Band-A frontier-harvest, the operator-named next L3 node after CAT-REL (2026-09-12). status draft: HELD until SPEC-PRIORITY-QUEUE-CONTRACT lands, then the Steward flips it ready and releases to foundation. The Architect actually PROBED buildability at 1691160dd (fresh ProbeHeap with both meld branches: meld Transparent; a same-operand recurrence is kernel-rejected NotTerminating 'SCT: idempotent self-loop'; opposite-Nat-comparator gives kernel TypeMismatch; ord_leq_at wrappers check with empty trusted-base delta; the (d.leq) type-argument spelling failed parsing -- use ord_leq_at). No syntax/elaborator repair needed. IN-LANE: computational-first, kernel-untouched/Axiom-free = no TCB, no operator touch. Re-measure every catalog anchor at the cut."
---

> # HELD until [[SPEC-PRIORITY-QUEUE-CONTRACT]] lands. Do NOT begin source edits.
>
> This is `draft` and `depends_on` the spec contract. The Steward flips it `ready`
> and releases to foundation once the contract merges. The design below is the
> Architect's ruling (Parts A/C/D); the normative behavior it must satisfy is the
> spec contract. Re-measure `catalog/packages/Data/Collections/` anchors and the
> `ord_leq_at`/`leq_nat` provider spellings at the cut.

## What this is

The operator-named next Band-A frontier-harvest node after CAT-REL
(priority-queue/heap). The Architect chose a persistent **leftist priority
queue** and ruled ONE complete computational node -- do NOT split carrier/insert
from meld/pop, because the same private meld and `make_node` implement the whole
abstraction, a write-only queue is not a useful release, and deferring extraction
would postpone the principal correctness discriminator.

## Representation and interface (Architect Part A)

Module `Data.Collections.PriorityQueue`. Abstract checked inductive
`PriorityQueue k v leq`. Private nodes carry a cached `Nat` rank, priority `k`,
payload `v`, and two recursive children; `Empty` has rank zero. A **valid node**
has: ordered children, root priority `<=` each nonempty child root, left rank
`>=` right rank, and cached rank `= Suc (right child rank)`. The invariant is
recursive; the lawful comparator's transitivity gives the root's
global-minimum property.

Priority and payload stay separate: require `Ord k`, preserve duplicate entries
and distinct equal-priority payloads; equal-priority extraction is NOT
stable/FIFO. Bind queues to their comparator through the ordinary function-valued
type parameter: public operations take an explicit `d : Ord k` and operate on
`PriorityQueue k v (ord_leq_at k d)` (an opposite-order merge is then a type
error; definitionally-equal comparators remain usable without identical proof
records). No runtime comparator-identity token or registry.

Public surface = the carrier plus exactly these five (`Q` = the queue type, `E` =
`Pair k v`):

```text
empty    k v d : Q
insert   k v d : k -> v -> Q -> Q
find_min k v d : Q -> Option E
pop_min  k v d : Q -> Option (Pair E Q)
merge    k v d : Q -> Q -> Q
```

`pop_min` returns the removed entry and the remainder together. Keep
constructors, rank, balancing, and raw comparator workers private. No partial
extraction, fabricated default, delete-only API, public mutable handle,
arbitrary-key deletion, decrease-key, stable ordering, or bulk-construction
family in this node.

## Implementation (Architect Part A)

- One private `make_node`: compares child ranks, swaps children if needed, caches
  `Suc (final right child rank)`. Reuse the existing public
  `Core.Classes.LawfulClasses.leq_nat` for rank comparison (no local Nat
  comparator).
- One private meld worker: handles `Empty` on either side; on two roots x/y,
  compares priorities and rebuilds the smaller root with `make_node`, recursively
  melding either `(right1, h2)` or `(h1, right2)`. `insert` melds a singleton;
  `pop_min` melds the two children.
- No whole-tree size pass, fuel wrapper, conversion to a sorted list, or alternate
  extraction engine.

Why leftist (Architect): a local, auditable rank invariant and logarithmically
bounded right-spine descent, unlike a skew/pairing heap's amortized account or a
binomial forest's rank/forest machinery. Sorted-list insertion is not an
equivalent asymptotic implementation.

**Buildability was probed at `1691160dd`** (Architect): a fresh `ProbeHeap` with
both meld branches checks; meld is Transparent; a same-operand recurrence is
kernel-rejected `NotTerminating("SCT: idempotent self-loop has no
strictly-decreasing parameter")`; an opposite-`Nat`-comparator queue gives kernel
`TypeMismatch`; `ord_leq_at` wrappers check with empty trusted-base delta. Use
the `ord_leq_at` projection spelling; the `(d.leq)` type-argument spelling fails
parsing. No syntax/elaborator repair is needed or requested. This establishes the
carrier/descent seam, NOT a tested implementation or a heap theorem.

## Proof posture (Architect Part C) -- read carefully

Ship a **tested computational implementation**, labelled as such -- NOT a
verified priority queue or a type-certified invariant. These semantics are
load-bearing and NOT deferred (their MEANING and acceptance COVERAGE, that is;
only their general kernel PROOF defers): empty behavior; minimum priority; every
key/payload occurrence preserved including duplicates; exactly-one removal;
remaining-queue validity; persistence. Co-land nonvacuous validity/contents
observations, checked defining/base equations, real public-client execution, and
the discriminators below.

Do NOT write signatures returning unproved refinements/validity certificates and
then call their proofs deferred; conversely, any proof obligation actually present
in a shipped result type MUST check before release. Keep Boolean decidable
predicates reflected into `Omega` and proof-bearing data in `Type`; no `Omega`
path carrier, postulate, or new primitive. State the structural cost account
honestly (log-many heap-node steps for valid operands, meld bounded by the two
right spines; NOT constant-time unary `Nat` rank, NOT native wall-clock O(log n))
and review it against the actual code. Existing/imported assumption dependencies
stay visible even though this package adds no `Axiom` or TCB mechanism.

## Reaching discriminators (Architect Part D) -- each a genuine differential

1. **Public construction and consumption.** A fresh-roots client imports the type
   plus all five ops and the `Ord` provider; creates empty, inserts, peeks, pops,
   and merges through real exported identities (construct AND drain values through
   the public API, unlike Map's abstract-parameter consumer). Pair with
   independent selective/qualified private-constructor and private-worker refusals
   (exact `UnboundName`). A name-presence/global-count test is insufficient.
2. **Empty / singleton / removal.** Peek and pop on empty are `None`. Insert one
   distinguishable key/payload: peek agrees with the popped entry, remainder is
   empty, a second pop is `None`. Rejects a default minimum, a missing insertion,
   and a removal that returns the old queue.
3. **Order, not insertion position.** Non-monotone insertion and reversed merge
   operands exercise both comparison arms; repeated pop yields nondecreasing
   priorities under the supplied lawful order. Include TWO different lawful orders
   on the same priority type, each with its own successful operations, plus a
   mismatched-order merge refusal at the real client boundary. No law-less
   comparator passed off as an `Ord` witness.
4. **Entries, not sets.** Equal priorities with distinct payload tags survive;
   identical entries survive with their multiplicity; `merge q q` doubles the
   contents. Compare tie groups as multisets; do not freeze unspecified
   equal-priority order. Rejects deduplication, payload detachment, and
   alias-based self-merge shortcutting.
5. **Meld and persistence.** Merge empty on both sides; two nonempty queues with
   interleaved and shared priorities, then drain the merged result AND each
   original independently. An independent small reference multiset loses exactly
   the returned entry per pop -- it must not share the implementation's recurrence
   or compute expected values from queue output. Rejects ignoring an operand,
   dropping a child/subtree, wrong payload, or corrupting remainder/originals.
6. **Rank/shape validity independent of sorted output.** Check every produced
   node's actual children, recomputed rank relation and cache, and priority
   order -- not a top-level stored rank or a validator's `True`. Ascending,
   descending, and mixed histories must exercise child swapping and unequal child
   ranks. Private malformed fixtures independently falsify bad cache, right-heavy
   layout, and heap-order inversion while matched valid fixtures accept. A
   sorted-list-like degenerate heap can drain correctly yet violate the bound.
7. **Mutation obligation.** Mutate the production path (not a hand-written
   mirror): invert root comparison; discard one nonempty merge operand; omit
   balancing; cache the wrong child's rank; return an unchanged pop remainder;
   collapse equal priorities or detach their payloads. Each needs its named
   failing observation, restoration, and green positive control. Mutate each
   detector that supplies the SOLE invariant evidence to constant-success too. A
   compile/type/termination rejection counts only for the property it actually
   rejects.

Bounded exhaustive short operation traces over a small priority/payload alphabet
are a good additional closure test -- finite evidence, NOT arbitrary-heap proofs.
No source-text grep promise, aggregate test-count substitution, wall-clock
asymptotic claim, local-workspace run, or runtime-instrumentation expansion.

## Acceptance criteria

- `Data.Collections.PriorityQueue` lands the carrier + five public ops per the
  interface, `Axiom`-free and kernel-untouched (no `Decl::Opaque`, primitive, TCB
  entry, `Omega` path carrier, postulate), verified by the catalog build.
- `Ord k` with payload separate; the comparator-parameter binding makes an
  opposite-order merge a type error; `make_node`/meld are private and reuse
  `leq_nat`.
- All seven discriminator families hold, none vacuous; the mutation obligation
  reds each named mutation with its restoration and positive control.
- The result is labelled a tested computational implementation; no shipped result
  type carries an unproved obligation; the cost account is structural and matches
  the code.
- Conforms to the reaching seed in `SPEC-PRIORITY-QUEUE-CONTRACT`.

## Not this node

- The general meld/insert/pop validity, multiplicity-conservation, and
  extract-min PROOFS under a fixed lawful order -- `CAT-PRIORITY-QUEUE-LAWS`
  (`draft`, the named proof follow-on).
- Machine-checked complexity bounds (separately deferred).
- Any extra API (delete/decrease-key/bulk/stable) beyond the five ops.

## Sizing / tier

**Size L, tier T1.** Ordinary total Ken over a fresh inductive, but the review
turns on design arguments: the recursive leftist validity invariant, the
payload-separate lawful-order binding, entry-wise (not set-wise) conservation,
and the honest computational/deferred-proof split. Architect is the required
reviewer.

## Contention

Foundation ring, `catalog/packages/Data/Collections/PriorityQueue.ken.md` (new
file) + its acceptance tests under `crates/ken-elaborator/tests/`. No cross-lane
contention (L1 runtime on `crates/ken-lowering`/`crates/ken-runtime`; L2 language
on `crates/ken-elaborator/src`). The new test files are under `crates/`, so a
candidate touching them is a CODE merge -> full CI, M8/M8a Adversary. Re-measure
provider spellings (`ord_leq_at`, `leq_nat`, `Pair`, `Option`) at the cut.
