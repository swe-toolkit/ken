# Graph reachability and dependency order

Format: `../../README.md`.
Spec: `spec/50-stdlib/62-graphs.md` §1.

These are **authored behavioral oracles**, not executed results. The
Architect-approved D0 development at `evt_5kbcdx6e29rkg` checked the
generic declarations and proofs on a separate scratch file; it did not
run this seed or deliver `Algorithm.Graphs.Dependency`. The author's
scratch used an existing Oct 6 CLI binary, not a freshly built CLI on
the candidate base. The three shared fixture code fences were appended
to a separate copy of that scratch and `ken check` returned rc=0; that
checks fixture typing only. No execution of the cases, Cargo test, CI
result, catalog build, or publication is claimed. Each case's promise class
is **durable invariant**: it observes computed reach decisions,
constructor alternatives, genuine witness edges, or order validity,
not a milestone census or a source-file spelling.

## Shared fixtures

Use `q = Bool`, `fq = bool_finite`, and the checked
`DecEq_instance_Bool` for all nonempty fixtures. Their adjacency
functions are explicit; no compiler graph primitive or theorem about
an unordered iteration order is assumed:

```ken
const forward : Graph Bool = MkGraph Bool
  (λu. λv. match u {
    True ↦ match v { True ↦ False; False ↦ True };
    False ↦ False
  })

const backward : Graph Bool = MkGraph Bool
  (λu. λv. match u {
    True ↦ False;
    False ↦ match v { True ↦ True; False ↦ False }
  })

const two_cycle : Graph Bool = MkGraph Bool
  (λu. λv. match u {
    True ↦ match v { True ↦ False; False ↦ True };
    False ↦ match v { True ↦ True; False ↦ False }
  })

const is_true : Bool → Bool = λv. v
const is_false : Bool → Bool = λv. bool_not v
```

`forward` has **only** `True → False`; `backward` has **only**
`False → True`; `two_cycle` has both and no self-loops. For a
`ReachWitness`, observe its `MkReach` endpoint and checked `Walk`
constructors; no test treats a raw DFA word, which may stutter, as a
graph path. To observe a `TopoOrder`, project `ordered_vertices` and
evaluate `contains`, `no_duplicates`, and `before` on that list. Its
complete/unique/forward theorem fields are checked by the CAT package,
not fabricated by the seed. A separate valid but duplicate-containing
certificate probes the boundary that `Finite q` does not ensure uniqueness:

```ken
const duplicate_bool : Finite Bool =
  MkFinite Bool
    (Cons Bool True (Cons Bool False (Cons Bool True (Nil Bool))))
    (λb. match b {
      True ↦
        list_elem_head Bool True
          (Cons Bool False (Cons Bool True (Nil Bool)));
      False ↦
        list_elem_later Bool False True
          (Cons Bool False (Cons Bool True (Nil Bool)))
          (list_elem_head Bool False (Cons Bool True (Nil Bool)))
    })
```

The zero-vertex fixture uses the same shape that typechecked in D0:

```ken
data EmptyVertex : Type where {}

fn empty_eq (x : EmptyVertex) (y : EmptyVertex) : Bool = match x {}
theorem empty_eq_sound (x : EmptyVertex) (y : EmptyVertex) :
  Equal Bool (empty_eq x y) True → Equal EmptyVertex x y = match x {}
theorem empty_eq_complete (x : EmptyVertex) (y : EmptyVertex) :
  Equal EmptyVertex x y → Equal Bool (empty_eq x y) True = match x {}

instance DecEq EmptyVertex {
  eq = empty_eq;
  sound = empty_eq_sound;
  complete = empty_eq_complete
}

const empty_vertices : Finite EmptyVertex =
  MkFinite EmptyVertex (Nil EmptyVertex) (λv. match v {})
const empty_graph : Graph EmptyVertex =
  MkGraph EmptyVertex (λv. match v {})
```

There is no start vertex on `EmptyVertex`; do not call
`graph_reachable` with a fictional one. The instance and finite
certificate are local fixture data, not new axioms or trust entries.

## Dependency outcome

### stdlib/graphs/forward-edge-has-complete-order

- spec: `spec/50-stdlib/62-graphs.md` §1.3.
- promise class: durable invariant.
- given: `dependency_order_or_cycle Bool bool_finite
  DecEq_instance_Bool forward`.
- expect: `HasOrder order`, with `ordered_vertices order` equal to
  `[True, False]`. `contains` for each Bool is `True`,
  `no_duplicates` is `True`, and `before True False` is `True`;
  `before False True` is `False`. `edge forward True False` is
  `True`, while its reverse is `False`. `ordered_complete`,
  `ordered_unique`, and `ordered_forward` have positive instances.
- measured: the result tag, both distinct output vertices, and
  direction-sensitive Boolean observations on the resulting list.
- claimed: a single real dependency orders both vertices once, with
  the source before its destination.
- the gap: one two-vertex fixture does not prove the universally
  quantified order predicates; the CAT node must check the fields
  and projection theorems on arbitrary vertices.
- why: a flipped comparator gives `[False, True]`, making
  `before True False` false despite the actual `True → False` edge.

### stdlib/graphs/reversing-only-edge-reverses-order

- spec: `spec/50-stdlib/62-graphs.md` §1.3.
- promise class: durable invariant.
- given: compare the `HasOrder` result for `forward` with that for
  `backward`, keeping `Bool`, `bool_finite`, and
  `DecEq_instance_Bool` fixed. The only changed graph fact is the
  direction of its sole edge.
- expect: `forward` yields `[True, False]`, `backward` yields
  `[False, True]`. Each returned list has both vertices exactly once;
  `before` is `True` in exactly the direction of that graph's edge.
- measured: opposite lists and opposite `before` observations for
  a pair on one carrier and one certificate.
- claimed: order direction comes from adjacency, not from the
  certificate's enumeration order or hardcoded Bool preference.
- the gap: the pair does not choose an order between unrelated
  vertices in a larger graph.
- why: always echoing `bool_finite`'s `[True, False]` passes the first
  case but fails on `backward`.

### stdlib/graphs/two-vertex-cycle-has-real-witness

- spec: `spec/50-stdlib/62-graphs.md` §1.1 and §1.3.
- promise class: durable invariant.
- given: `dependency_order_or_cycle Bool bool_finite
  DecEq_instance_Bool two_cycle`. Independently evaluate the four
  adjacency values for both Bool vertices.
- expect: `HasCycle (MkCycle start next first return_walk)`, never
  `HasOrder`. `start` and `next` are distinct; `edge two_cycle start
  next = True`, and the checked `first` field proves it. The return
  `Walk two_cycle next start` contains at least one real `WalkStep`,
  since neither self-loop exists; each traversed step has a checked
  `edge = True`. No particular start vertex or shortest witness is
  selected by the contract.
- measured: cycle tag, distinct endpoints, the actual directed first
  edge and its return route, versus both absent self-loops.
- claimed: a cycle result cannot be a zero-edge walk or a fabricated
  transition on an absent edge.
- the gap: runtime inspection illustrates one closed witness; the
  constructor types and generic checked search establish the
  certificate for every returned cycle.
- why: treating reflexive zero-step reachability as a positive
  self-loop would admit a fake one-vertex cycle despite both
  `edge v v` values being `False`; accepting a nonedge is also
  incompatible with the checked `MkCycle` edge field.

### stdlib/graphs/zero-vertex-graph-has-empty-order

- spec: `spec/50-stdlib/62-graphs.md` §1.3.
- promise class: durable invariant.
- given: `dependency_order_or_cycle EmptyVertex empty_vertices
  DecEq_instance_EmptyVertex empty_graph` using the local, checked
  fixture above.
- expect: `HasOrder order` with
  `ordered_vertices EmptyVertex DecEq_instance_EmptyVertex empty_graph
  order = Nil EmptyVertex` and `no_duplicates = True`.
  `all_vertices` and `all_edges_forward` hold by empty-carrier
  elimination, and `HasCycle` cannot contain a start vertex.
- measured: the runtime result tag and empty output list; the
  source fixture independently provides zero possible vertices.
- claimed: the algorithm handles an empty `Finite` enumeration
  without inventing a vertex or a cycle.
- the gap: this case cannot exercise `graph_reachable`, because its
  API correctly demands a start vertex. The nonempty cases do so.
- why: an algorithm that assumes a first listed vertex cannot
  produce this valid empty-order result.

### stdlib/graphs/duplicate-enumeration-is-not-duplicate-order

- spec: `spec/50-stdlib/62-graphs.md` §1.3.
- promise class: durable invariant.
- given: the same `forward` graph and `DecEq_instance_Bool` as the
  forward-edge case, but a `Finite Bool` certificate whose covered
  list is `[True, False, True]`: `duplicate_bool` above. Its coverage
  is checked by head membership for `True` and later/head membership
  for `False`; no uniqueness premise is supplied to `Finite`.
- expect: `HasOrder order` with `[True, False]`, not a repeated
  `True`. `no_duplicates` is `True`, both `contains` tests are
  `True`, and `before True False` is `True`.
- measured: same graph, different **certificate multiplicity**, and
  the identical complete/distinct two-vertex output.
- claimed: duplicate finite evidence does not duplicate a graph
  vertex in a topological order or weaken its coverage.
- the gap: this closed enumeration cannot establish the general
  deduplication/permutation proof; `ordered_unique` and
  `ordered_complete` must be checked over every input certificate.
- why: sorting the raw three entries without deduplication yields a
  repeated `True`, failing the public uniqueness field.

## Predicate reachability

### stdlib/graphs/reachable-edge-has-walk

- spec: `spec/50-stdlib/62-graphs.md` §1.1–§1.2.
- promise class: durable invariant.
- given: `graph_reachable Bool bool_finite forward True is_false`
  and `graph_find_walk` with the same arguments. Evaluate
  `edge forward True False` independently.
- expect: the decision is `True`, and `graph_find_walk` is
  `Some (MkReach end hit walk)` with `end = False`, target
  `is_false end = True`, and at least one real `WalkStep` from
  `True` to `False`. Every edge of `walk` tests `True`; the
  target is false at the start. The hypotheses of both
  `graph_reachable_complete` and `graph_find_walk_some` are
  nonvacuous here.
- measured: nonzero traversal, affirmative Boolean decision,
  `Some` endpoint, and the independent real-edge/target observations.
- claimed: an accepted witness is a genuine path to the target,
  not the DFA's unchecked word of candidate letters.
- the gap: one concrete walk does not prove generic completeness;
  the CAT theorem handles arbitrarily long walks.
- why: a runner ignoring the true edge cannot reach `False`, while
  one fabricating a `Some` without `MkReach` evidence cannot typecheck.

### stdlib/graphs/unreachable-reverse-and-reflexive-control

- spec: `spec/50-stdlib/62-graphs.md` §1.1–§1.2.
- promise class: durable invariant.
- given: the same `forward` graph, `Bool` carrier and certificate;
  query from `False` with `is_true`, and then from that **same**
  `False` start with `is_false`. In both queries the first edge
  candidate `False → True` is absent.
- expect: for target `is_true`, `graph_reachable` is `False` and
  `graph_find_walk` is `None`: no edge leaves `False`. For target
  `is_false`, `graph_reachable` is `True` and `graph_find_walk`
  is `Some (MkReach False hit WalkHere)`: no true outgoing edge can
  inhabit a `WalkStep` from `False`. That target succeeds at the
  start without manufacturing a cycle.
- measured: opposite decisions and `None`/`Some` on the same start
  and graph as only the target predicate changes; one target is
  true already at the start and the other cannot be reached.
- claimed: reflexive reachability accepts an empty walk while an
  absent outgoing edge does not become a directed path or cycle.
- the gap: these closed results do not alone prove both public
  reachability laws for every `Finite q` and target predicate.
- why: treating any target as an initial hit incorrectly accepts
  the unreachable reverse query; treating `WalkHere` as a positive
  edge would create a false cycle at `False`.

## Coverage boundary

The order cases cover both `HasOrder` and `HasCycle`, singleton-edge
orientation both ways, duplicate certificates, and the zero-vertex
carrier. The reachable/unreachable pair holds the graph and start
fixed while changing only the target; the positive edge traversal
separately exercises a nonempty walk. Every public checked law has a
closed positive instance: `contains_sound`/`contains_complete` on
both vertices, the two reachability laws on the real edge, and the
three order projections on `forward`. Runtime examples do not replace
those seven general checked laws, and they do not assert SCCs,
shortest paths, weighted edges, cross-instance cursor behavior, a
canonical order, or an exported cycle-refutes-order theorem.
