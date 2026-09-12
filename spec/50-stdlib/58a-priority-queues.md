# Persistent priority queues

> Status: **LANDED COMPUTATION (CAT-PRIORITY-QUEUE).** This chapter is
> normative for the abstract carrier, its five public operations, and their
> behavioral and structural-cost contract. The complete interface is present as
> a tested computational implementation; the general kernel proofs remain
> separately deferred to `CAT-PRIORITY-QUEUE-LAWS`. This is ordinary
> standard-package Ken: no new kernel rule, primitive, postulate, or trusted-base
> entry.

A priority queue stores **entries** made of a priority and a payload. Extraction
chooses an entry with minimum priority under one fixed lawful order. The payload
is not part of that order, duplicate entries are retained, and equal-priority
entries have no stable order.

This is a dedicated collection contract rather than an addition to chapter 57.
Chapter 57's collection-law template expects a different proof posture, while
this abstraction deliberately lands computation and discriminating tests before
its general proofs. Detailed leftist representation choices belong to the
component design, not to this public contract.

## 1. Derivation and trust boundary

The package is `Data.Collections.PriorityQueue`. It derives from:

- the prelude `Nat`, `Bool`, `Option`, and canonical compiler-origin `Pair`;
- `Ord` and `ord_leq_at` from `Core.Classes.LawfulClasses` (`51 §2.3`);
- ordinary inductive declarations, structural recursion, and checked functions;
- ordinary `Equal`/`IsTrue` propositions for the later law proofs.

The carrier lives in `Type`: it contains priorities, payloads, and private
structural data. Validity and the later laws live in `Omega` through `Equal` and
`IsTrue`. There is no proof-relevant path carrier in `Omega`. The package adds
no `Axiom`, primitive operation, kernel declaration form, runtime comparator
registry, or `trusted_base()` entry.

A checked abstract `data` declaration establishes that carrier values are
well-typed. It does **not** establish that the queue algorithms preserve their
representation invariant. Similarly, an `Ord k` dictionary certifies the laws
of the supplied priority order; it does not certify the queue algorithms. The
proof-status boundary is stated in §7.

## 2. The six-name abstract interface

The public surface is exactly the carrier plus five operations. The following
is a signature synopsis, not a second implementation. `Q` and `E` below are
explanatory abbreviations only:

```
Q := PriorityQueue k v (ord_leq_at k d)
E := Pair k v

pub data PriorityQueue (k : Type) (v : Type)
                       (leq : k -> k -> Bool) : Type

pub fn empty (k : Type) (v : Type) (d : Ord k) : Q
pub fn insert (k : Type) (v : Type) (d : Ord k)
              (priority : k) (payload : v) (q : Q) : Q
pub fn find_min (k : Type) (v : Type) (d : Ord k)
                (q : Q) : Option E
pub fn pop_min (k : Type) (v : Type) (d : Ord k)
               (q : Q) : Option (Pair E Q)
pub fn merge (k : Type) (v : Type) (d : Ord k)
             (left : Q) (right : Q) : Q
```

Expanded, `pop_min` returns
`Option (Pair (Pair k v) (PriorityQueue k v (ord_leq_at k d)))`. It never
returns only a payload, only a priority, or a remainder without the removed
entry.

Constructors, structural metadata, balancing helpers, raw-comparator workers,
and validity checkers are private. There is no public delete-only operation,
arbitrary-key deletion, decrease-key, mutable handle, stable variant, or bulk
construction family in this contract.

### 2.1 One order belongs to one queue type

Every operation takes an explicit `d : Ord k`. The queue type is indexed by the
projected function `ord_leq_at k d`, not by the proof record's identity. Thus:

- queues built with definitionally equal comparator functions interoperate even
  when their proof records are not identical;
- a queue indexed by the opposite comparator cannot be supplied to `merge` for
  `d`; the mismatch is a type error at the public client boundary;
- no run-time token or registry compares dictionary identities.

The priority requires `Ord k`. The payload requires no `Ord v`, `Eq v`, or
`DecEq v`. In particular, the contract does not order `Pair k v` by its first
component: that would violate pair-order antisymmetry when distinct payloads
share one priority.

## 3. Abstract contents, counts, and validity

For the contract, `Entries(q)` is the finite multiset of stored `(priority,
payload)` occurrences. It is a semantic model, not a public conversion to a
list and not a representation choice. Two identical occurrences appear twice.
Queue equality is not defined as kernel `Equal` on private representations.

For any decidable observation `p : k -> v -> Bool`, define the specification
measure

```
count_by p q = the Nat number of occurrences e in Entries(q) with p e = True
bit_p (priority, payload) = if p priority payload then Suc Zero else Zero
```

`count_by` and `bit_p` are law-level notation, not a seventh public operation.
They avoid any payload-equality requirement: the law quantifies over a supplied
predicate rather than demanding `DecEq v`.

`Valid_d(q)` is the abstract recursive representation-validity proposition for
queues indexed by `ord_leq_at k d`. Its private realization must establish all
of these facts:

1. every immediate subqueue is valid under the same comparator;
2. every nonempty queue root has priority less than or equal to every priority
   in its descendants, using `ord_leq_at k d`;
3. cached structural information and balance relations agree with the actual
   subqueues and justify the meld-spine bound in §6;
4. the private representation contains exactly the occurrences described by
   `Entries(q)`.

Those obligations are normative. Their constructor fields, cached measure, and
local balancing equations are not public and are not frozen here. The selected
leftist implementation supplies one realization in its component design; a
future representation may differ only if it preserves this contract and its
cost bound.

## 4. Computational behavior

The equations in this section state required behavior. The build must exercise
them through real public values. Their general proofs remain obligations, not
preconditions smuggled into the five result types.

### 4.1 Empty, insertion, and merge

For every lawful `d`:

- `Entries(empty k v d)` is empty and `Valid_d(empty k v d)` holds;
- inserting `(priority, payload)` adds exactly one occurrence and retains all
  prior occurrences;
- merging retains every occurrence of both operands, including duplicates and
  aliasing: `merge d q q` contains two copies of every occurrence in `q`;
- `merge d (empty d) q` and `merge d q (empty d)` have the same entry
  multiplicities as `q`.

Precisely, for every `p : k -> v -> Bool` and valid operands:

```
count_by p (empty k v d) = Zero

count_by p (insert k v d priority payload q)
  = add (bit_p (priority, payload)) (count_by p q)

count_by p (merge k v d q1 q2)
  = add (count_by p q1) (count_by p q2)
```

Each displayed equality is an `Equal Nat` proposition when realized as a Ken
law. These equations characterize entries, not a set of priorities. They imply
that reversing merge operands preserves all entry multiplicities, but they do
not assert `Equal Q` for two private queue shapes.

### 4.2 Minimum and optional extraction

Let `e = (minimum_priority, payload)`. For every valid `q`:

- `find_min k v d q = None` if and only if `Entries(q)` is empty;
- `pop_min k v d q = None` if and only if `Entries(q)` is empty;
- if `find_min d q = Some e`, then `e` is one occurrence of `Entries(q)` and,
  for every stored occurrence `(priority, payload2)`,
  `IsTrue (ord_leq_at k d minimum_priority priority)` holds;
- if `pop_min d q = Some (e, remainder)`, then
  `find_min d q = Some e`, `Valid_d(remainder)` holds, and exactly that one
  occurrence is removed.

The exactly-one statement is entry-wise. For every decidable `p`:

```
pop_min k v d q = Some (e, remainder)
  implies
count_by p q = add (bit_p e) (count_by p remainder)
```

This is again an `Equal Nat` conclusion under the displayed premise. It does
not collapse entries that have equal priorities or identical payloads. Repeated
successful `pop_min` calls therefore remove one occurrence at a time and return
priorities in nondecreasing order under `d` until the first `None`.

### 4.3 Equal-priority ties are non-stable

When several minimum entries have equal priorities, `find_min` and `pop_min`
may choose any one of those occurrences. On one fixed queue value they must
agree as §4.2 states, but insertion order and merge-operand order do not impose
FIFO or any other stable payload order. Conformance compares each equal-priority
group as a multiset. A client that depends on a particular order within such a
group is outside this contract.

### 4.4 Persistence

All five operations are persistent. Observing a result does not consume or
mutate an input queue. After `merge q1 q2`, clients may independently inspect
and drain `q1`, `q2`, and the merged result. After `pop_min q`, the original `q`
and the returned remainder are independently usable. Structural sharing is
allowed, but it cannot change entry multiplicities or make `merge q q` an
idempotent shortcut.

## 5. Validity obligations

For the same fixed lawful order, the contract requires:

```
Valid_d (empty k v d)

Valid_d q
  implies Valid_d (insert k v d priority payload q)

Valid_d q1 and Valid_d q2
  implies Valid_d (merge k v d q1 q2)

Valid_d q and pop_min k v d q = Some (e, remainder)
  implies Valid_d remainder
```

The computational build must observe these properties nonvacuously on empty,
ascending, descending, mixed insert/merge histories, and every returned
remainder. It must also reject independently malformed private fixtures for
heap-order, cached-structure, and balance faults while accepting matched valid
fixtures. Draining in sorted order is not evidence for the cached-structure or
balance clauses: a degenerate sorted representation can return correct values
and still violate the structural bound.

No public signature returns `Valid_d(q)` or a refinement containing it in the
computational tranche. Such a result would require its proof at construction
time and could not honestly be called deferred.

## 6. Structural cost account

The cost claim counts a precisely charged private meld descent and its priority
comparisons, not native instructions or wall-clock time. Let `rho(q)` be the
number of nonempty nodes on the private meld-descent spine. In the selected
leftist realization this is the right-child spine. Let `M(q1,q2)` count only
calls whose two current operands are nonempty:

```
M(empty,h) = Zero
M(h,empty) = Zero
M(q1,q2) = Suc (M(next1,next2))  when q1 and q2 are nonempty
```

In the last equation, `(next1,next2)` are exactly the operands of the one
recursive meld call selected after comparing the two current roots. One
operand advances along its meld-descent spine and the other remains current.
That unchanged operand may be inspected again; the next two-nonempty call
receives its own charge. `rho` and `M` are specification and review measures,
not public operations or runtime instrumentation.

Each two-nonempty call makes exactly one priority comparison. The terminal
empty-base call makes none. Therefore one merge makes exactly `M(q1,q2)`
priority comparisons and invokes the private meld worker exactly
`Suc (M(q1,q2))` times, including its terminal base call. The descent satisfies

```
M(q1,q2) <= rho(q1) + rho(q2).
```

For valid queues, the balance part of `Valid_d` must bound `rho(q)` by stored
occurrences. A sufficient concrete obligation is
`2 ^ rho(q) <= count_by (lambda _ _. True) q + 1`; this is not an equivalence
with an arbitrary native-time `O(log n)` statement. The computational costs are:

- `empty` constructs a constant number of nodes and makes no priority
  comparison;
- `find_min` inspects a constant number of nodes, invokes no meld worker, and
  makes no priority comparison;
- `merge q1 q2` has the exact `M`, comparison, worker-invocation, and spine
  bounds above;
- `insert e q`, implemented through singleton merge, has the corresponding
  `M(singleton,q)` and singleton-plus-`rho(q)` bounds;
- successful `pop_min` inspects the root and has the `M` cost of merging its
  two child queues.

Each charged step and the terminal base call perform only a bounded constant
amount of heap-constructor and cached-metadata access outside the selected
recursive call. There is no hidden whole-tree traversal. This account makes
neither an arbitrary priority comparison nor comparison of unary `Nat`
structural metadata constant-time. Their internal costs are additional to the
charged `M` and priority-comparison counts. No measured native or wall-clock
`O(log n)` claim follows. A machine-checked general complexity proof
is separately deferred; the build must still match this structural account.

## 7. Delivery and proof status

The three stages are deliberately separate:

1. **This contract:** fixes the six-name surface, behavioral semantics,
   validity obligations, and structural cost account. It adds no implementation.
2. **`CAT-PRIORITY-QUEUE` (landed):** ships one complete computational
   implementation of the carrier and all five operations. It executes the public construction,
   extraction, multiplicity, order, persistence, validity, abstraction, and
   structural-cost discriminators in the paired seed. The result is **tested**,
   not a verified priority queue or type-certified invariant.
3. **`CAT-PRIORITY-QUEUE-LAWS`:** proves, for arbitrary valid queues under the
   same fixed lawful order, the §4 count equations, §4.2 minimum/extraction
   statements, §5 validity preservation, and nondecreasing repeated extraction.
   It introduces no second queue. Machine-checked complexity is a separate
   residual.

The semantic requirements are not deferred: a build that loses an occurrence,
returns a non-minimum entry, mutates an operand, or produces an invalid
remainder is nonconforming even before the general proofs land. What is deferred
is general kernel evidence for those already-fixed propositions.

## 8. Acceptance boundary

A conforming computational build must satisfy all of these together:

1. the structured module export inventory is exactly `PriorityQueue`, `empty`,
   `insert`, `find_min`, `pop_min`, and `merge`; constructors and workers remain
   unimportable;
2. the queue is indexed by `ord_leq_at k d`, needs `Ord k` only, and rejects an
   opposite-order merge by typing;
3. empty/singleton behavior, optional extraction, global minimum, one-at-a-time
   removal, duplicate retention, self-merge doubling, and persistence match §4;
4. equal-priority observations compare payload groups as multisets and impose no
   stable order;
5. independent validity checks cover all recursive clauses and distinguish
   structural faults from correct sorted output;
6. the implementation is kernel-untouched and `Axiom`-free, and its measured
   cost claims use the structural account in §6;
7. tested observations and deferred general proofs are reported separately.

Reaching cases are in
`../../conformance/stdlib/collections/seed-priority-queue.md`.
