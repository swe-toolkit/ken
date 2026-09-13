# Persistent priority queue — conformance seed

Format: `../../README.md`. These cases pin the contract in
`spec/50-stdlib/58a-priority-queues.md`. This seed now accompanies the landed
`CAT-PRIORITY-QUEUE` computational implementation. Every case is **GREEN for
that tested finite population**; the closing evidence record names the exact
producer and test blobs and the executed case population. This is not evidence
for the deferred general laws.

The cases use the public module `Data.Collections.PriorityQueue`, the canonical
Axiom-free `Ord Nat` provider, and a finite payload with no order or equality
constraint:

```
data Tag = A | B | C | D | E | F
```

`up : Ord Nat` is the canonical dictionary whose comparator is `leq_nat`.
`up2 : Ord Nat` is a separately constructed record whose `leq` field is also
`leq_nat` and whose four law fields use the same already-proved Nat laws; it is
not an alias for the `up` record. Thus `ord_leq_at Nat up` and
`ord_leq_at Nat up2` are definitionally equal functions even though the proof
records are separate values. `down : Ord Nat` is a separate real law-carrying
dictionary with `down.leq x y = up.leq y x`; its reflexivity, antisymmetry,
transitivity, and totality fields are obtained by swapping the corresponding
arguments to the proved `up` laws. None of these dictionaries contains an
`Axiom` or stub. A raw Boolean comparator without those law fields is not an
acceptable substitute.

Test-side `drain d q` repeatedly calls public `pop_min` until `None`. It is an
observer, not a package operation. Expected entry multisets and priority lists
below are fixed from case literals or maintained by a separate list-multiset
oracle. The oracle does not call queue workers, reuse their recurrence, or
compute an expectation from a queue-produced collection.

Equal-priority groups are written with braces and multiplicities. Their internal
order is intentionally unspecified. For example,
`[(1,A), 2:{A*2,B}, (3,C)]` fixes priority-group order and entry counts but
accepts every permutation within the priority-2 group.

## PQ1 — exact public abstraction boundary

### stdlib/priority-queue/public-client-constructs-and-drains
- spec: `58a §2`, `§8` item 1.
- promise: normative compatibility vector.
- given: a fresh-roots client imports the structured export inventory
  `{PriorityQueue, empty, insert, find_min, pop_min, merge}` plus the lawful
  `Ord Nat` provider. Through those imported identities it constructs two
  nonempty queues, peeks one, merges them, and drains the result.
- expect: the export inventory is exactly those six names, obtained from the
  elaborated module export table rather than a source-text scan. The client
  accepts and observes the fixed drain `[(0,D),(1,B),(2,C),(3,A)]` from inputs
  `[(3,A),(1,B)]` and `[(2,C),(0,D)]`.
- why: reaches every public identity through a real external client. A global
  name count or an abstract queue parameter would not prove construction and
  extraction are public.

### stdlib/priority-queue/private-constructors-unbound-selective
- spec: `58a §2`, `§8` item 1.
- promise: durable invariant.
- given: the successful public client above, changed only to selectively import
  the selected leftist realization's private `Empty` and `Node` constructors,
  one variant per constructor.
- expect: each import independently rejects with `UnboundName` naming `Empty`
  or `Node`, while the unchanged six-name import accepts.
- why: visibility is tested at the external import boundary. The positive
  client prevents an unrelated missing-module failure from satisfying the
  negative.

### stdlib/priority-queue/private-constructors-unbound-qualified
- spec: `58a §2`, `§8` item 1.
- promise: durable invariant.
- given: the successful public client above, changed only to use
  `Data.Collections.PriorityQueue.Empty` or
  `Data.Collections.PriorityQueue.Node`, one variant per constructor.
- expect: each qualified use independently rejects with `UnboundName` naming
  `Empty` or `Node`; construction through `empty` and `insert` still accepts.
- why: selective-import privacy alone does not establish qualified-name privacy.

### stdlib/priority-queue/private-workers-unbound
- spec: `58a §2`, `§8` item 1.
- promise: durable invariant.
- given: separate variants of the successful public client attempt selective
  and qualified access to the selected realization's private `meld` and
  `make_node` workers.
- expect: all four attempts independently reject with `UnboundName` naming the
  attempted worker, while public `merge` and `pop_min` accept on the same
  operand types.
- why: the exact public export-table assertion is the closure over any other
  private helper; these attempts prove the two design-named workers reach the
  visibility boundary rather than relying on a name census alone.

## PQ2 — empty, singleton, and exactly-one removal

### stdlib/priority-queue/empty-peek-and-pop-are-none
- spec: `58a §4.2`.
- promise: durable invariant.
- given: `q0 = empty Nat Tag up`.
- expect: `find_min Nat Tag up q0 = None` and
  `pop_min Nat Tag up q0 = None` as two independent observations.
- why: pins the empty-to-`None` direction separately for both operations and
  rejects fabricated defaults or partial empty extraction.

### stdlib/priority-queue/singleton-peek-pop-and-remainder
- spec: `58a §4.2`, `§4.4`.
- promise: durable invariant.
- given: `q0 = empty Nat Tag up` and
  `q1 = insert Nat Tag up 2 A q0`.
- expect: `find_min up q1 = Some (2,A)`;
  `pop_min up q1 = Some ((2,A),q2)`; both `find_min up q2` and
  `pop_min up q2` are `None`; and the original `q1` still peeks and pops as the
  same singleton after `q2` is observed.
- why: pins the nonempty-to-`Some` direction separately for both operations
  while also rejecting missing insertion, peek/pop disagreement, an unchanged
  pop remainder, and mutation of the original.

### stdlib/priority-queue/two-identical-entries-pop-one-at-a-time
- spec: `58a §3`, `§4.1`, `§4.2`.
- promise: durable invariant.
- given: insert `(2,A)` twice into empty.
- expect: the first pop returns `(2,A)` and a remainder whose first pop returns
  `(2,A)` and whose next pop is `None`. Test-side literal count changes
  `2 -> 1 -> 0`.
- why: distinguishes a multiset from a set and makes “exactly one” observable
  even when the two entries are identical.

## PQ3 — priority order and comparator binding

### stdlib/priority-queue/nonmonotone-insert-drains-up
- spec: `58a §2.1`, `§4.2`.
- promise: durable invariant.
- given: under `up`, insert in literal order
  `[(3,C),(1,A),(4,D),(2,B)]`.
- expect: repeated public pop returns exactly
  `[(1,A),(2,B),(3,C),(4,D)]`, then `None`.
- why: priority, not insertion position or a pre-sorted input, selects the root.

### stdlib/priority-queue/same-priorities-drain-down
- spec: `58a §2.1`, `§4.2`.
- promise: durable invariant.
- given: under the separate lawful `down`, insert the same four entries in the
  same literal order as the preceding case.
- expect: repeated public pop returns exactly
  `[(4,D),(3,C),(2,B),(1,A)]`, then `None`.
- why: the comparator parameter is operational. Reusing ascending order while
  merely accepting a second dictionary makes this pair fail.

### stdlib/priority-queue/reversed-merge-operands-exercise-both-arms
- spec: `58a §4.1`, `§4.2`.
- promise: durable invariant.
- given: under `up`, build `left = [(1,A),(4,D)]` and
  `right = [(2,B),(3,C)]`; evaluate both `merge up left right` and
  `merge up right left`.
- expect: each drain is exactly `[(1,A),(2,B),(3,C),(4,D)]`. The test records
  that the two-root production comparison takes each orientation on these two
  calls rather than crediting one aggregate result for both arms.
- why: catches a one-sided root-selection recurrence and an ignored operand.

### stdlib/priority-queue/equal-function-orders-merge-opposite-order-rejects
- spec: `58a §2.1`, `§8` item 2.
- promise: normative compatibility vector.
- given: `q_up : PriorityQueue Nat Tag (ord_leq_at Nat up)` contains `(1,A)`,
  `q_up2 : PriorityQueue Nat Tag (ord_leq_at Nat up2)` contains `(2,B)`, and
  `q_down : PriorityQueue Nat Tag (ord_leq_at Nat down)` contains `(3,C)`.
  Each is constructed by its own successful public insertion. Attempt both
  `merge Nat Tag up q_up q_up2` and `merge Nat Tag up q_up q_down`.
- expect: the `up`/`up2` merge accepts and drains exactly `[(1,A),(2,B)]`.
  The `up`/`down` merge rejects with kernel `TypeMismatch` between the opposite
  comparator indices. Matched controls `merge up q_up q_up` and
  `merge down q_down q_down` accept and drain in their respective orders.
- why: the positive rejects proof-record-identity compatibility while the
  negative pins comparator-function incompatibility. Same-record controls keep
  a broken dictionary or merge operation from satisfying the rejection.

## PQ4 — entries, multiplicity, and unspecified ties

### stdlib/priority-queue/equal-priority-payloads-survive-and-peek-pop-agree
- spec: `58a §3`, `§4.1`–`§4.3`.
- promise: durable invariant.
- given: under `up`, insert `q = [(1,C),(2,A),(2,B),(2,A),(3,D)]` and separately
  build `q_tie = [(1,A),(1,B),(2,C)]`.
- expect: drain groups for `q` are exactly
  `[(1,C), 2:{A*2,B*1}, (3,D)]`. For the unchanged `q_tie`, the joint
  `(find_min,pop_min)` observation is exactly one of these two literal rows:
  `Some (1,A)` with `Some ((1,A),rA)` and drain `rA = [(1,B),(2,C)]`, or
  `Some (1,B)` with `Some ((1,B),rB)` and drain `rB = [(1,A),(2,C)]`.
  No assertion chooses between the rows or fixes the order of `A,B,A` within
  `q`'s priority-2 group.
- why: the two literal rows require peek and pop to choose the same actual
  tied-minimum occurrence without imposing stability. The first queue still
  catches priority-only deduplication and payload detachment.

### stdlib/priority-queue/self-merge-doubles-entry-multiplicity
- spec: `58a §4.1`, `§4.4`.
- promise: durable invariant.
- given: `q` contains `[(1,C),(2,A),(2,B),(2,A)]`; compute `merge up q q`.
- expect: the merged drain is exactly
  `[(1,C),(1,C), 2:{A*4,B*2}]`, while an independent later drain of `q` is still
  `[(1,C), 2:{A*2,B*1}]`.
- why: rejects set semantics, alias-based idempotence, and destructive merge.

### stdlib/priority-queue/count-by-laws-on-fixed-predicates
- spec: `58a §3`, `§4.1`, `§4.2`.
- promise: durable invariant.
- given: the fixed queue `q = [(1,A),(2,A),(2,B),(3,C)]`; test predicates
  `p_all = lambda _ _. True`, `p_A = lambda _ tag. tag_is_A tag`, and
  `p_even_A = lambda priority tag. nat_even priority && tag_is_A tag` are
  independent fixture observers. Insert `(2,A)`, merge with
  `r = [(0,D),(2,B)]`, and pop once from each resulting queue.
- expect: before modification the literal counts are `4`, `2`, and `1`;
  insertion changes them to `5`, `3`, and `2`; its pop returns `(1,A)` and the
  remainder counts are `4`, `2`, and `2`. Merge changes the original counts to
  `6`, `2`, and `1`; its pop returns `(0,D)` and the remainder counts are `5`,
  `2`, and `1`. In each pop equation, independently evaluating the returned
  entry's `0/1` contribution reconstructs the stated pre-pop count.
- why: three differently selective predicates prevent an entry-wise law from
  collapsing into a total-size check. Expected literals come from the fixture,
  not the queue output.

## PQ5 — meld completeness and persistence

### stdlib/priority-queue/merge-empty-on-both-sides
- spec: `58a §4.1`, `§4.4`.
- promise: durable invariant.
- given: `q = [(2,B),(1,A),(3,C)]` under `up`.
- expect: drains of `merge up (empty up) q` and
  `merge up q (empty up)` are both `[(1,A),(2,B),(3,C)]`; a later drain of `q`
  has the same result.
- why: exercises both empty meld bases and persistence separately.

### stdlib/priority-queue/interleaved-shared-priority-merge-persists
- spec: `58a §4.1`–`§4.4`.
- promise: durable invariant.
- given: `left = [(1,A),(2,E),(4,D)]` and
  `right = [(2,B),(3,C),(5,F)]`, constructed independently under `up`.
- expect: merged drain is
  `[(1,A), 2:{B*1,E*1}, (3,C),(4,D),(5,F)]`; later drains of `left` and `right`
  remain exactly `[(1,A),(2,E),(4,D)]` and
  `[(2,B),(3,C),(5,F)]`. At every pop an independent literal multiset removes
  exactly the returned occurrence and the remainder matches what is left.
- why: rejects an ignored operand, dropped child, detached payload, corrupt
  remainder, and destructive sharing. The reference multiset shares no queue
  recurrence.

### stdlib/priority-queue/bounded-short-traces-match-independent-multiset
- spec: `58a §4`, `§7` item 2.
- promise: durable invariant.
- given: the entry alphabet is the six literal pairs from priorities `{0,1,2}`
  and payloads `{A,B}`. Per comparator, all insertion histories of length zero
  through three give `1 + 6 + 36 + 216 = 259` traces. Histories of length zero
  through two give `1 + 6 + 36 = 43` queue operands; their full ordered
  Cartesian product, including both orientations and all 43 self-pairs, gives
  `43 * 43 = 1849` merge traces. Run both domains under `up` and `down`, for the
  precomputed total `2 * (259 + 1849) = 4216` traces. A separate list-multiset
  model starts from the literal operations.
- expect: exactly 4,216 traces execute: 259 insertion traces and 1,849 ordered
  merge traces for each comparator. Before every observation, the independent
  model records whether its entry count is zero. `find_min` returns `None`
  exactly at model count zero, and independently `pop_min` returns `None`
  exactly at model count zero. At every nonzero count both return `Some`, their
  returned entries agree, the peek is a model-minimum entry, and the pop removes
  exactly its returned occurrence from the model. Every remainder and original
  queue can be drained independently. Equal-priority groups compare as
  multisets.
- why: the pre-execution count prevents a zero or partial run from becoming its
  own population oracle. The domain also exercises both directions of each
  `None` equivalence across finite empty and nonempty histories. It remains
  finite evidence, not a general heap theorem.

## PQ6 — recursive validity and structural cost

The public contract keeps representation clauses abstract. The following are
**realization-reaching gates for CAT-PRIORITY-QUEUE's selected leftist design**,
not additional public operations or a promise that every conforming future
representation uses cached ranks.

### stdlib/priority-queue/produced-leftist-validity-recomputed
- spec: `58a §3`, `§5`, `§6`.
- promise: durable invariant for the selected realization.
- given: ascending, descending, alternating-high-low, equal-priority, two-queue
  merge, and repeated-pop histories that reach actual production nodes. The
  test traverses each produced node through a private test boundary and
  independently recomputes its child validity, root-to-child priority order,
  cached structural measure, balance relation, and right-spine length.
- expect: every recursive clause holds at every node; the histories include
  both child-swap outcomes and unequal child measures. For each produced queue,
  the independently counted entries satisfy
  `2 ^ right_spine_length <= entry_count + 1`.
- why: reading a top-level cached measure or calling the production validator
  would restate the implementation. Recomputing from actual children makes each
  clause observable.

### stdlib/priority-queue/malformed-cache-fails-validity-only
- spec: `58a §3` item 3, `§5`.
- promise: durable invariant for the selected realization.
- given: matched private fixtures differ only in one internal cached measure;
  child layout, priorities, payloads, and actual descendants are identical.
- expect: the matched valid fixture passes; the bad-cache fixture fails at the
  exact cache-consistency clause even though both can yield the same sorted
  drain when inspected without trusting the cache.
- why: isolates cache validity from heap order and output order.

### stdlib/priority-queue/right-heavy-shape-fails-validity-only
- spec: `58a §3` item 3, `§5`, `§6`.
- promise: durable invariant for the selected realization.
- given: matched private fixtures have the same entries and heap-ordered
  priorities, and each cache agrees with its own actual children; one satisfies
  the selected leftist balance relation and one is right-heavy.
- expect: the valid fixture passes; the right-heavy fixture fails only at the
  exact balance clause. Both fixtures' entry multisets and sorted-drain
  observations agree, so output order cannot satisfy this case accidentally.
- why: distinguishes the structural logarithmic bound from priority semantics.

### stdlib/priority-queue/heap-order-inversion-fails-validity-only
- spec: `58a §3` item 2, `§5`.
- promise: durable invariant for the selected realization.
- given: matched private fixtures differ only by one parent/child priority pair;
  their shape, cache, balance relation, and payload placement are identical.
- expect: the valid fixture passes; the inverted fixture fails at the exact
  heap-order clause.
- why: independently reaches the ordering arm rather than relying on one large
  malformed fixture to fail somewhere.

### stdlib/priority-queue/meld-charge-count-follows-right-spines
- spec: `58a §6`.
- promise: durable invariant for the selected realization.
- given: valid fixtures with independently counted right-spine lengths,
  including both-empty, one-empty, singleton, balanced unequal-size,
  equal-priority roots, and successful pops whose roots have two nonempty
  children. A test-local fail-closed semantic verifier consumes the actual
  kernel-checked producer terms, follows aliases and call arguments, and closes
  every reachable queue-touching helper. It certifies both empty terminals,
  each two-root branch's one saturated priority comparison and one recursive
  edge, bounded reconstruction, root-only rank/empty/find behavior, make-node
  metadata roles, and the exact meld operands used by public merge, insert, and
  pop. Unknown executable calls, eliminators, or operand roles reject. Fixture
  `M`, comparison, worker-invocation, and structural-access accounts are then
  derived from that certified relation and actual private children; no
  wall-clock threshold or public counter is used.
- expect: `empty` returns the selected constant-size single-constructor empty
  form with no subtree visit or priority comparison. Both-empty and one-empty
  merges have `M = 0`, zero priority comparisons, and one terminal meld-worker
  invocation. Every merge has exactly `M` priority
  comparisons, exactly `M + 1` meld-worker invocations, and
  `M <= right_spine(q1) + right_spine(q2)`. Direct `insert e q` satisfies the
  same equalities for `M(singleton(e),q)` and the bound
  `M <= 1 + right_spine(q)`. A successful `pop_min q`, with root children
  `left` and `right`, inspects that root and satisfies the same equalities
  for `M(left,right)` and the bound
  `M <= right_spine(left) + right_spine(right)`. Each trace touches only current
  roots, bounded constant metadata and reconstruction sites, and the selected
  descent spines; none performs a hidden whole-tree traversal. `find_min` has
  zero meld charges, priority comparisons, and meld-worker invocations.
  Unary-`Nat` metadata comparison work and each priority comparator's internal
  cost are reported separately.
- why: binds the contract's exact charged population and zero/base convention
  to the checked program rather than a second recurrence. Direct insert/pop
  certification rejects an extra traversal that merge-only evidence cannot
  see.

## PQ7 — required mutation provenance

Each mutation changes the production path while leaving the relevant test and
its independently fixed expectation unchanged. Restore the producer after each
mutation and re-run the named positive control. A compile-breaking mutation is
not evidence for an execution property.

| Production mutation | Required failing observation | Positive control |
|---|---|---|
| invert two-root priority choice | both `nonmonotone-insert-drains-up` and `same-priorities-drain-down` return a non-minimum first priority | both lawful-order drains |
| discard either nonempty merge operand | `interleaved-shared-priority-merge-persists` misses that operand's fixed entries | `merge-empty-on-both-sides` plus the interleaved case after restoration |
| omit child balancing | `produced-leftist-validity-recomputed` reaches its balance failure on a production history | matched valid shape |
| cache the wrong child measure | `produced-leftist-validity-recomputed` reaches cache inconsistency on a production history | matched valid cache |
| return the unchanged queue as pop remainder | `singleton-peek-pop-and-remainder` observes a nonempty second pop | empty and singleton controls |
| collapse equal priorities | `equal-priority-payloads-survive-and-peek-pop-agree` loses a fixed literal occurrence | fixed tie-group multiset |
| make tied `find_min` choose a different payload than `pop_min` | `equal-priority-payloads-survive-and-peek-pop-agree` matches neither permitted literal row | tied-minimum two-row observation |
| shortcut equal-identity merge to one operand | `self-merge-doubles-entry-multiplicity` returns the original rather than doubled multiset | original and doubled literal multisets |
| detach priority from payload | `interleaved-shared-priority-merge-persists` returns an entry outside the fixed multiset | independent left/right drains |
| traverse an untouched subtree before `insert` returns | `meld-charge-count-follows-right-spines` rejects the extra direct-insert eliminator/callback outside the certified meld operands | direct-insert certification after restoration |
| traverse an untouched subtree before successful `pop_min` returns | `meld-charge-count-follows-right-spines` rejects the extra direct-pop eliminator/callback outside the certified child meld | direct-pop certification after restoration |

If a production validity detector is the sole mechanism for one recursive
clause, mutate that detector to constant success while keeping the malformed
fixture unchanged; the corresponding case must fail because its independent
recomputation disagrees. This detector-side mutation is separate from each
population-side mutation above.

## Evidence and deferral record

`CAT-PRIORITY-QUEUE` lands every case above as a tested observation. Producer
blob `1eec578603cdef349af21d944ac174af5919705c` and acceptance-test blob
`229bbcd89e2041771af44c51e70e9c1f12c02639` execute all 22 case records through
nine passing tests, including exactly 4,216 bounded traces. Fifteen
compile-preserving production-side and detector-side mutations reddened their
named observations, and both files were restored byte-identically. The result
reports separately:

1. real public computation and the exact named finite observations it executes;
2. private abstraction/validity and structural-charge observations;
3. production-side and detector-side mutation failures with restoration;
4. the still-deferred general kernel proofs.

`CAT-PRIORITY-QUEUE-LAWS` owns the general validity-preservation,
`count_by`-conservation, global-minimum, and nondecreasing-drain proofs over
arbitrary valid queues under one fixed lawful order. Machine-checked complexity
bounds remain a separate residual. A successful aggregate test run proves only
its executed observations and is not evidence that those general laws exist.
