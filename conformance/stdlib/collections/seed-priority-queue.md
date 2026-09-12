# Persistent priority queue — conformance seed

Format: `../../README.md`. These cases pin the contract in
`spec/50-stdlib/58a-priority-queues.md`. This seed lands before the package
implementation, so every case is **RED-UNTIL-CAT-PRIORITY-QUEUE**. A later
GREEN report must name the exact producer and test blobs and the executed case
population; the presence of this file is not execution evidence.

The cases use the public module `Data.Collections.PriorityQueue`, the canonical
Axiom-free `Ord Nat` provider, and a finite payload with no order or equality
constraint:

```
data Tag = A | B | C | D | E | F
```

`up : Ord Nat` is the canonical dictionary whose comparator is `leq_nat`.
`down : Ord Nat` is a separate real law-carrying dictionary with
`down.leq x y = up.leq y x`; its reflexivity, antisymmetry, transitivity, and
totality fields are obtained by swapping the corresponding arguments to the
proved `up` laws. Neither dictionary contains an `Axiom` or stub. A raw
Boolean comparator without those law fields is not an acceptable substitute.

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
- why: rejects fabricated defaults and partial empty extraction.

### stdlib/priority-queue/singleton-peek-pop-and-remainder
- spec: `58a §4.2`, `§4.4`.
- promise: durable invariant.
- given: `q0 = empty Nat Tag up` and
  `q1 = insert Nat Tag up 2 A q0`.
- expect: `find_min up q1 = Some (2,A)`;
  `pop_min up q1 = Some ((2,A),q2)`; both `find_min up q2` and
  `pop_min up q2` are `None`; and the original `q1` still peeks and pops as the
  same singleton after `q2` is observed.
- why: independently rejects missing insertion, peek/pop disagreement, an
  unchanged pop remainder, and mutation of the original.

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

### stdlib/priority-queue/opposite-order-merge-is-a-type-error
- spec: `58a §2.1`, `§8` item 2.
- promise: normative compatibility vector.
- given: `q_up : PriorityQueue Nat Tag (ord_leq_at Nat up)` and
  `q_down : PriorityQueue Nat Tag (ord_leq_at Nat down)`, each constructed by
  its own successful public insertion. Attempt `merge Nat Tag up q_up q_down`.
- expect: kernel `TypeMismatch` between the opposite comparator indices. The
  matched controls `merge up q_up q_up` and `merge down q_down q_down` accept
  and drain in their respective orders.
- why: the negative cannot pass merely because either dictionary or operation
  is broken; both same-order controls reach the public merge boundary.

## PQ4 — entries, multiplicity, and unspecified ties

### stdlib/priority-queue/equal-priority-distinct-payloads-survive
- spec: `58a §3`, `§4.1`, `§4.3`.
- promise: durable invariant.
- given: under `up`, insert
  `[(1,C),(2,A),(2,B),(2,A),(3,D)]`.
- expect: drain groups are exactly
  `[(1,C), 2:{A*2,B*1}, (3,D)]`. No assertion fixes the order of `A,B,A`
  within the priority-2 group.
- why: catches priority-only deduplication, payload detachment, and accidental
  FIFO as a conformance requirement without rejecting an implementation that
  happens to choose FIFO for this one shape.

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
- given: every insertion sequence of length at most three over priorities
  `{0,1,2}` and payloads `{A,B}`, plus every pairwise merge of queues built by
  sequences of length at most two. Run the same fixed cases under `up` and
  `down`. A separate list-multiset model starts from the literal operations.
- expect: every peek is a model-minimum entry; every pop removes exactly its
  returned occurrence from the model; every remainder and original queue can be
  drained independently; termination occurs at model count zero. Equal-priority
  groups compare as multisets. The executed population is reported explicitly,
  never as “suite green”.
- why: finite closure across short histories supplements the named cases. It is
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

### stdlib/priority-queue/meld-step-count-follows-right-spines
- spec: `58a §6`.
- promise: durable invariant for the selected realization.
- given: valid fixtures with independently counted right-spine lengths,
  including both-empty, one-empty, singleton, balanced unequal-size, and
  equal-priority roots. Count production meld node steps and priority
  comparisons without using wall-clock thresholds.
- expect: `merge q1 q2` takes no more than
  `right_spine(q1) + right_spine(q2)` meld steps and no more than one priority
  comparison per two-nonempty step. `find_min` performs none. Rank comparison
  work and each priority comparator's internal cost are reported separately.
- why: measures the structural claim the contract makes and no stronger native
  complexity claim.

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
| collapse equal priorities | `equal-priority-distinct-payloads-survive` loses a fixed literal occurrence | fixed tie-group multiset |
| shortcut equal-identity merge to one operand | `self-merge-doubles-entry-multiplicity` returns the original rather than doubled multiset | original and doubled literal multisets |
| detach priority from payload | `interleaved-shared-priority-merge-persists` returns an entry outside the fixed multiset | independent left/right drains |

If a production validity detector is the sole mechanism for one recursive
clause, mutate that detector to constant success while keeping the malformed
fixture unchanged; the corresponding case must fail because its independent
recomputation disagrees. This detector-side mutation is separate from each
population-side mutation above.

## Evidence and deferral record

Before `CAT-PRIORITY-QUEUE` lands, every case above is a specified expected
observation and none is reported GREEN. The build must report separately:

1. real public computation and the exact named finite observations it executed;
2. private abstraction/validity and structural-step observations;
3. production-side and detector-side mutation failures with restoration;
4. the still-deferred general kernel proofs.

`CAT-PRIORITY-QUEUE-LAWS` owns the general validity-preservation,
`count_by`-conservation, global-minimum, and nondecreasing-drain proofs over
arbitrary valid queues under one fixed lawful order. Machine-checked complexity
bounds remain a separate residual. A successful aggregate test run proves only
its executed observations and is not evidence that those general laws exist.
