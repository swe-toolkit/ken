# Finite directed graphs and dependency order

> **Status: DRAFT v0 (SPEC-GRAPHS-DEPENDENCY-CONTRACT).** Section 1 is
> normative for a finite directed graph, checked reachability, and a
> topological-order-or-real-cycle result in one ordinary `Algorithm.Graphs`
> package. The contract does not claim catalog delivery. It introduces no
> kernel, trust, or surface-syntax change and does not revise §61 §2.

## 1. Graphs, walks, and dependency order

A graph's vertex carrier can be any `q : Type`; its directed edge relation
is a Boolean function, not an intrinsically finite set or a list of
successors. Finiteness is explicit evidence **supplied to operations**,
not stored in the graph. A Boolean adjacency collapses parallel edges;
it permits loops and imposes no symmetry. All signatures in this section
are for level-zero `q : Type` and checked `Omega` propositions at level
zero. The single catalog package is `Algorithm.Graphs.Dependency`; the
module path is not a second graph representation.

### 1.1 Vertices and genuine paths

The public carrier and edge observation have these exact types:

```ken
pub data Graph (q : Type) : Type where {
  MkGraph : (q → q → Bool) → Graph q
}
export MkGraph

pub fn edge (q : Type) (g : Graph q) (s : q) (t : q) : Bool
```

`edge q (MkGraph adjacency) s t` evaluates to `adjacency s t`.
`True` means the directed edge `s → t` exists; `False` means it does
not. The graph does not require `Finite q` or `DecEq q` at construction.
The following public, proof-carrying data define actual walks and
cycles, rather than treating a word of proposed vertices as edges:

```ken
pub data Walk (q : Type) (g : Graph q) (s : q) : q → Type where {
  WalkHere : Walk q g s s;
  WalkStep : (u : q) → (v : q) →
    Equal Bool (edge q g u v) True →
    Walk q g s u → Walk q g s v
}
export WalkHere, WalkStep

pub data Cycle (q : Type) (g : Graph q) : Type where {
  MkCycle : (s : q) → (next : q) →
    Equal Bool (edge q g s next) True →
    Walk q g next s → Cycle q g
}
export MkCycle

pub data ReachWitness
  (q : Type) (g : Graph q) (s : q) (target : q → Bool) : Type where {
  MkReach : (t : q) → Equal Bool (target t) True → Walk q g s t →
    ReachWitness q g s target
}
export MkReach
```

`WalkHere` supplies the **zero-edge** walk, so every vertex reaches
itself. `WalkStep` extends a walk at its destination only after a
checked proof of the new edge. A `Cycle` contains a real first edge
and a return walk to its start: even a self-loop has at least one
edge, while a zero-edge `WalkHere` alone is **not** a cycle. The
cycle may revisit vertices; simplicity, shortestness, and a chosen
start are not promised. A reach witness carries both a target hit
and a genuine walk to that vertex. `Walk`, `Cycle`, and
`ReachWitness` are `Type`-sorted evidence; their `Equal Bool` fields
are checked `Omega` propositions, not permission to invent an edge.

### 1.2 Deciding reachability without a second fixed point

The graph reuses `Data.Finite.Finite` and the **landed**
`Algorithm.FormalLanguages.Reachability` decision from §61 §2. The
following are its public operations and checked laws:

```ken
pub fn graph_reachable
  (q : Type) (fq : Finite q) (g : Graph q) (s : q)
  (target : q → Bool) : Bool

pub fn graph_find_walk
  (q : Type) (fq : Finite q) (g : Graph q) (s : q)
  (target : q → Bool) : Option (ReachWitness q g s target)

pub theorem graph_reachable_complete
  (q : Type) (fq : Finite q) (g : Graph q) (s : q) (t : q)
  (target : q → Bool) (walk : Walk q g s t) :
  Equal Bool (target t) True →
  Equal Bool (graph_reachable q fq g s target) True

pub theorem graph_find_walk_some
  (q : Type) (fq : Finite q) (g : Graph q) (s : q)
  (target : q → Bool) :
  Equal Bool (graph_reachable q fq g s target) True →
  Equal Bool
    (is_some (ReachWitness q g s target)
      (graph_find_walk q fq g s target))
    True
```

The decision is **reflexive-transitive**: a target hit at `s` succeeds
by `WalkHere`, and a sequence of true edges extends it. `True` means a
finite walk from `s` ends at some `t` where `target t = True`;
`False` means no such walk exists. `is_some` is the public
`Data.Sums.Combinators` operation; it does not define a new graph
presence test. `graph_find_walk` returns `Some` with precisely that
checked endpoint-and-walk evidence when the
Boolean decision is `True`; a `Some` gives a real path, never just a
Boolean claim or an unchecked word. `None` means no target walk. The
first law supplies completeness from **any** such walk, regardless of
length. The second ensures an affirmative decision produces a `Some`;
construction of `ReachWitness` supplies soundness of its payload.
Neither operation asks for `DecEq q`: a caller's `target : q → Bool`
determines what a target means. A query for a particular vertex uses
a separately supplied `DecEq q` to construct such a predicate.

The CAT implementation **must reuse** the §61 §2 fixed point, not
copy it. Its private DFA encoding has state and alphabet `q`, initial
state `s`, and a transition that moves from `u` to letter `v` when
`edge g u v` is `True`, otherwise stays at `u`. The same `fq` certifies
states and letters, and `graph_reachable` is exactly the landed
`reachable q q fq fq` on that encoding and caller's target. A word
returned by the underlying `find_word` may include letters that
**stutter on absent edges**. It is not a public graph-path witness:
the implementation constructs a `Walk`, discarding those stutters,
and checks the target before forming `MkReach`. Both the walk and the
order/cycle development checked at Architect D0 without an `Axiom`;
that feasibility evidence does not mean the CAT package or this
conformance seed has been executed. No new fixed-point search,
second semantic notion of reachability, or §61 §2 law change belongs
in this contract.

### 1.3 Complete dependency order or a real cycle

The public `Core.Classes.LawfulClasses.DecEq q` dictionary is required
**here** for comparing vertices in a list and deduplicating a
`Finite q` certificate that may repeat them. It is
not retroactively required by the graph, walks, or predicate
reachability. These five public predicates fix the meaning of the
ordered-vertex certificate:

```ken
pub fn contains (q : Type) (d : DecEq q) (x : q) (xs : List q) : Bool
pub fn before
  (q : Type) (d : DecEq q) (x : q) (y : q) (xs : List q) : Bool
pub fn no_duplicates (q : Type) (d : DecEq q) (xs : List q) : Bool
pub fn all_vertices (q : Type) (d : DecEq q) (xs : List q) : Omega
pub fn all_edges_forward
  (q : Type) (d : DecEq q) (g : Graph q) (xs : List q) : Omega
```

`contains d x xs` scans `xs` with `d.eq x` and is `True` exactly when
some list entry is equal to `x`. `before d x y xs` is `True` exactly
when the **first occurrence** of `x` precedes a later occurrence of
`y`; it is `False` if an earlier occurrence of `y` intervenes, if
one is missing, or if `x = y`. `no_duplicates d xs` rejects any
repeated vertex according to `d.eq`. These are ordinary transparent
Boolean list scans, not a new primitive. The two proposition-valued
predicates are fixed by:

```ken
all_vertices q d xs =
  (x : q) → Equal Bool (contains q d x xs) True

all_edges_forward q d g xs =
  (x : q) → (y : q) →
    Equal Bool (edge q g x y) True →
    Equal Bool (before q d x y xs) True
```

In particular, `all_vertices` quantifies over **every vertex**,
including one missed by an enumeration in a faulty implementation.
The certificate forbids duplicates and requires every edge to point
forward in the list:

```ken
pub data TopoOrder (q : Type) (d : DecEq q) (g : Graph q) : Type where {
  MkTopoOrder : (xs : List q) →
    Equal Bool (no_duplicates q d xs) True →
    all_vertices q d xs →
    all_edges_forward q d g xs →
    TopoOrder q d g
}
export MkTopoOrder

pub fn ordered_vertices
  (q : Type) (d : DecEq q) (g : Graph q)
  (order : TopoOrder q d g) : List q

pub theorem ordered_complete
  (q : Type) (d : DecEq q) (g : Graph q)
  (order : TopoOrder q d g) :
  all_vertices q d (ordered_vertices q d g order)

pub theorem ordered_unique
  (q : Type) (d : DecEq q) (g : Graph q)
  (order : TopoOrder q d g) :
  Equal Bool (no_duplicates q d (ordered_vertices q d g order)) True

pub theorem ordered_forward
  (q : Type) (d : DecEq q) (g : Graph q)
  (order : TopoOrder q d g) :
  all_edges_forward q d g (ordered_vertices q d g order)
```

`ordered_vertices` projects `xs`; the three theorems project the
checked fields, rather than inferring coverage from a count or an
absence of duplicate entries. A complete order of an empty carrier
is `Nil` and has no cycle. No declaration prescribes the ordering of
unrelated vertices or a canonical representative for duplicate
finite-certificate entries.

The result is an ordinary **Type-level tagged choice**, not
`Core.Logic.Or`: that `Or` accepts `Omega` parameters, whereas
`TopoOrder` and `Cycle` carry relevant `Type` data. The landed
`Data.Sums.Combinators` source defines `Either` without `pub`, so it
is not an importable public sum. The one package-local result has
exactly these two public constructors:

```ken
pub data OrderOrCycle (q : Type) (d : DecEq q) (g : Graph q) : Type where {
  HasOrder : TopoOrder q d g → OrderOrCycle q d g;
  HasCycle : Cycle q g → OrderOrCycle q d g
}
export HasOrder, HasCycle

pub fn dependency_order_or_cycle
  (q : Type) (fq : Finite q) (d : DecEq q) (g : Graph q) :
  OrderOrCycle q d g
```

The total operation returns either an order satisfying all three
certificate fields or a cycle whose first edge and return walk are
real. On a cycle hit, the result is `HasCycle`; with no hit, it returns
`HasOrder`. A finite search considers certified vertex pairs for an
edge `v → u` with a walk `u →* v`. Its no-cycle branch supplies a
checked condition on **all** pairs via `Finite.covers`, including
entries omitted by a faulty enumeration. In that branch, every edge
strictly increases the number of vertices able to reach its
destination. Deduplicate `elements fq`, rank each vertex by that
ancestor count, and use the landed `Data.Collections.Derived.sort`
with the total Nat comparator. The checked sort-permutation law
carries complete and unique coverage to the result; its sorted law
and strict rank inequality force each edge forward. Duplicate
entries in `Finite q` neither duplicate output vertices nor weaken
the argument. This is the checked D0 derivation, not a shortest-path
algorithm and not a requirement to freeze which equally ranked
vertices appear first. No second reachability fixed point or
unproved `Ord q` instance is introduced.

Two more public **proved** laws connect the Boolean membership scan
to the generic Ω membership in `Data.Collections.Derived`:

```ken
pub theorem contains_sound
  (q : Type) (d : DecEq q) (x : q) (xs : List q) :
  Equal Bool (contains q d x xs) True → list_elem q x xs

pub theorem contains_complete
  (q : Type) (d : DecEq q) (x : q) (xs : List q) :
  list_elem q x xs → Equal Bool (contains q d x xs) True
```

The proof of `contains_sound` uses `d.sound`; the other direction uses
`d.complete` and eliminates truncated `list_elem` only into an
`Omega` proposition. No `Eq`-by-spelling or assumption about
certificate uniqueness enters either law.

### 1.4 Trust and limits

For `q : Type` at level zero, `Graph q`, `Walk`, `Cycle`,
`ReachWitness`, `TopoOrder`, and `OrderOrCycle` are ordinary `Type`
data. `Equal Bool`, `list_elem`, and the two universally quantified
order predicates are `Omega_0`: the Π over a level-zero vertex takes
the predicative maximum with its `Omega_0` codomain. The genuine
edge, walk, coverage, and order claims are checked proof arguments;
calling a Boolean predicate is not itself a proof. Every public
`theorem` in this section requires a kernel-checked proof body;
`Axiom`, a postulate, an open obligation, a new primitive, or a new
`trusted_base()` entry cannot substitute for one. The inductive
constructors carry the cycle and order guarantees in their types.
The D0 scratch checked with rc=0 and both relevant wrong-branch
mutations kernel-rejected; these are feasibility measurements, not
CAT-delivery, acceptance-test, or CI results.

Strongly connected components, shortest paths, weighted edges, a
successor-list adaptor, and maximal or lexicographically least
orders remain outside §1. So does an exported **cycle-refutes-order**
theorem: it would need checked irreflexivity and transitivity of
`before`, and no §1 consumer requires it. The result still never
presents a missing edge as a cycle witness or a non-covering list as
a topological order. These exclusions do not change the existing
`Data.Finite.Finite`, Dfa, or formal-languages reachability contracts.
