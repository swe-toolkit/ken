# Finite directed graphs and dependency order

> **Status: DRAFT v0 (SPEC-GRAPHS-DEPENDENCY-CONTRACT §1;
> SPEC-GRAPHS-CYCLE-REFUTES-ORDER §2;
> SPEC-GRAPHS-WALK-COMPOSITION-ORDER §3;
> SPEC-GRAPHS-WALK-APPEND-UNIT-ASSOC §4;
> SPEC-GRAPHS-REACHABILITY-ALONG-WALK §5;
> SPEC-GRAPHS-REACHABILITY-TARGET-MONOTONE §6;
> SPEC-GRAPHS-REACHABILITY-GRAPH-MONOTONE §7;
> SPEC-GRAPHS-REVERSE-REACHABILITY §8;
> SPEC-GRAPHS-WALK-REVERSE-APPEND §9).** Section 1 binds finite graphs,
> checked reachability, and a topological-order-or-real-cycle result in
> `Algorithm.Graphs.Dependency`; its catalog package is present on the
> base. Section 2 adds three checked generic laws and has an authored
> §2 conformance seed. Section 3's walk-composition operation and
> first-edge order law now have checked catalog exports; the four §3 seed
> fences are exercised in a package-owned test. Section 4's left-unit and
> associativity laws now have checked catalog exports, and the closed §4
> seed is exercised by a package-owned test. These package tests are not
> official conformance-runner execution or CI. Section 5's public
> reachability law, checked catalog proof and package-owned seed test
> are now on main; the test is not official conformance or landed trust.
> Section 6 specifies one generic target-monotonicity law. Architect's
> isolated D0 was scratch evidence. The separate Foundation candidate
> exports the checked law and its package-owned test runs the complete
> accepted seed and isolated refusal; that is not official conformance-
> runner execution, CI, publication, landing or landed trust. Section 7
> specifies one public graph-monotonicity law; its checked catalog
> proof, seed and package-owned test are now on main. Architect's D0 was
> isolated scratch evidence, and earlier candidate-local checks were
> not official conformance-runner execution or landed trust. No kernel,
> trust or surface-syntax change or revision to §61 §2 is required.
> Section 8's four checked reverse-graph and reachability forms, authored
> seed and package-owned test are now on main. The named test previously
> passed 2/2 with reported candidate-local trusted identities 114 → 114;
> that result is not an official conformance run or a landed trust
> measurement. CV statically checked the seed but did not run it as Ken
> or through the official runner. The separate `cat_graphs_dependency`
> run timed out without a verdict. Section 9 states one premise-free
> walk-reversal law. Its Architect AC-0 is scratch-only (trusted identities
> 114 → 114); this spec-only child adds no §9 catalog export, package
> test or CV seed. No §9 official conformance, CI, publication, landing
> or measured landed trust is established.

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
some list entry is equal to `x`. `before d x y xs` scans for the first
entry equal to **either** `x` or `y`: it returns `True` exactly when
that entry equals `x` but not `y` and some entry equal to `y` occurs
later in the list. It returns `False` if `y` occurs first, either
vertex is absent, or `x = y`, even if a later occurrence of `y`
follows `x`. For `q = Bool`, `d = DecEq_instance_Bool` gives
`before d True False [False, True, False] = False`; an earlier
`False` is not erased by a later one.
`no_duplicates d xs` rejects any repeated vertex according to
`d.eq`. These are ordinary transparent Boolean list scans, not a
new primitive. The two proposition-valued predicates are fixed by:

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
orders remain outside §1. The two `OrderOrCycle` outcomes are
**exclusive**: §2 proves that a `Cycle q g` refutes every `TopoOrder q d g`.
The refutation uses only the order's `all_edges_forward` field, not its
coverage or uniqueness fields. The result still never presents a
missing edge as a cycle witness or a non-covering list as a topological
order. These exclusions do not change the existing `Data.Finite.Finite`,
Dfa, or formal-languages reachability contracts.

## 2. A cycle refutes every dependency order

The following three public laws add checked proof bodies to
`Algorithm.Graphs.Dependency`; they are not axioms. `before` is the
§1.3 Boolean scan. Its irreflexivity and transitivity hold for every
list and any `d : DecEq q`, with **no** premise of list uniqueness,
vertex coverage or `d.sound`/`d.complete`:

```ken
pub theorem before_irreflexive
  (q : Type) (d : DecEq q) (x : q) (xs : List q) :
  Equal Bool (before q d x x xs) False

pub theorem before_transitive
  (q : Type) (d : DecEq q) (x : q) (y : q) (z : q)
  (xs : List q) :
  Equal Bool (before q d x y xs) True →
  Equal Bool (before q d y z xs) True →
  Equal Bool (before q d x z xs) True
```

These laws are about the scan's ordering decisions, not a claim that
`d.eq` decides equality correctly without its dictionary laws. Each
occurrence of a query is compared through the same `d.eq` function.
With an order's forward-edge field, the first edge of a cycle puts its
start before the next vertex; transitivity carries this fact along
the return walk and would put the start before itself. Irreflexivity
makes that impossible. Even a self-loop has a first edge, so its
return walk may be `WalkHere` without weakening the refutation:

```ken
pub theorem cycle_refutes_order
  (q : Type) (d : DecEq q) (g : Graph q) :
  TopoOrder q d g → Cycle q g → Bottom
```

The `Bottom` here is the existing empty type used by §1's private
order proofs. A client with both evidence values eliminates their
contradiction with `cycle_refutes_order`; `HasCycle` thus excludes an
order for that same graph and `d`. This is stronger than merely
reporting whichever constructor the total operation chose. The
proof needs only `all_edges_forward`, even though a `TopoOrder` also
carries checked coverage and uniqueness. The private `walk_before`,
`before_contains_later`, and Boolean step lemmas support the public
proofs without adding further public laws. Their Boolean case splits
reuse the catalog's existing private `before_step` helper; no second
scan helper is introduced.

The Architect's generic D0 client on base `55d1ab578` checked with
rc 0 and no new `Axiom`. Five independent mutations, including a
false reflexive order claim and an invalid cycle-edge proof, were
kernel-rejected. The §2 seed is authored in this WP; its positive
and refused blocks were exercised separately as scratch clients, not
by a shipped harness or CI. Those checks do not establish a measured
landed `trusted_base()` delta. Reachability, finite enumeration, and §1's
choice behavior remain unchanged; the deferred graph operations in
§1.4 remain outside this section.

## 3. Walk composition and order along a walk

The same `Algorithm.Graphs.Dependency` package gains a public operation
for joining genuine walks. It introduces neither a new graph carrier nor
an adjacency decision. Its arguments name the start `s`, shared endpoint
`t`, and final endpoint `u` in path order, followed by the two walks in
that same order:

```ken
pub fn walk_append
  (q : Type) (g : Graph q) (s : q) (t : q) (u : q)
  (first : Walk q g s t) (second : Walk q g t u) :
  Walk q g s u
```

`walk_append` recurses on the **second** walk, which was built by
appending edges at its destination. If `second` is `WalkHere`, the
result is `first`. If it is `WalkStep prev end e prefix`, the result
is `WalkStep q g s prev end e` applied to
`walk_append q g s t prev first prefix`. In particular, the last
checked edge of the second walk remains the last checked edge of the
result; no list of proposed vertices is passed off as edge evidence.
Right `WalkHere` identity holds by computation, even for an open
`first : Walk q g s t`:
`walk_append q g s t t first (WalkHere q g t)` reduces to `first`.
Left `WalkHere` identity does **not** hold by computation on an open
second walk. Section 4 states a checked left-unit law and an
associativity law; neither is supplied by reduction on open walks.
No public right-unit theorem is added for an equality already supplied
by reduction.

A topological order places the start of every walk with a **first
edge** before its end. The nonempty shape is explicit rather than a
new predicate: `first` checks `s → next`, and `rest` walks from
`next` to `t`. The public checked law has exactly this type:

```ken
pub theorem ordered_walk_forward
  (q : Type) (d : DecEq q) (g : Graph q)
  (order : TopoOrder q d g)
  (s : q) (next : q) (t : q)
  (first : Equal Bool (edge q g s next) True)
  (rest : Walk q g next t) :
  Equal Bool
    (before q d s t (ordered_vertices q d g order)) True
```

The order's `all_edges_forward` field puts `s` before `next`;
transitivity of `before` carries this direction along `rest`. If
`rest` is `WalkHere`, this is the one-edge case, not a claim that a
zero-edge walk puts a vertex before itself. The proof reuses §2's
private `walk_before` as a transport lemma over the projected order
and its forward-edge field. It does **not** export `walk_before`,
change §2's `cycle_refutes_order`, require a separate list-coverage or
uniqueness premise, or add a new equality or graph primitive. Every
public theorem here must have a checked body without `Axiom`, a
postulate, or a new `trusted_base()` entry.

The Architect's AC-0 development on base `10b1ec623` checked the
`walk_append` definition and `ordered_walk_forward` proof with rc 0
and no `Axiom`. Its closed three-vertex chain clients accepted the
composed two-edge walk and forward order, while a reversed-order
claim on the same walk was refused; a false first-edge claim and a
client import of private `walk_before` were also refused. Those were
scratch checks. The catalog now exports both §3 names with checked
bodies; a package-owned test root-loads them, checks both positive
seed cases and the same-input forward-order control, and refuses the
isolated reversed-order proposition. It measures exact trusted-base
identities before and after loading the package. The unchanged census
row passes the targeted strict-resolution suite with zero D0 residuals.
These are package-test measurements, not official conformance-runner
execution, CI, publication, landing, or a landed trust measurement.
Section 1's reachability and finite certificates, §2's three laws,
and the deferred scope in §1.4 are unchanged.

## 4. Left unit and associativity of walk composition

The same `Algorithm.Graphs.Dependency` package proves that a zero-edge
walk at the start is a **left unit** of `walk_append` for every walk:

```ken
pub theorem walk_append_here_left
      (q : Type) (g : Graph q) (s : q) (u : q) (walk : Walk q g s u)
    : Equal (Walk q g s u) (walk_append q g s s u (WalkHere q g s) walk) walk
```

The equality is between walks, not only their endpoints. The law is
unconditional: it has no `Finite q`, `DecEq q`, or order premise. It
requires induction on `walk`, since `walk_append` recurses on its
second argument. In the `WalkHere` case, reduction closes the goal (`Proved` in the
checked D0); in a `WalkStep` case, the induction hypothesis is carried
through the same checked edge and prefix by congruence. A generic law body checks
this open-walk property; a closed example alone would not establish it.

The operation also **associates** without changing edge order. The
left side groups `first` and `second` before appending `third`; the
right side groups `second` and `third` before appending to `first`:

```ken
pub theorem walk_append_assoc
      (q : Type)
      (g : Graph q)
      (s : q)
      (t : q)
      (u : q)
      (v : q)
      (first : Walk q g s t)
      (second : Walk q g t u)
      (third : Walk q g u v)
    : Equal
        (Walk q g s v)
        (walk_append q g s u v (walk_append q g s t u first second) third)
        (walk_append q g s t v first (walk_append q g t u v second third))
```

Induction on `third` establishes this equality at every start and
intermediate endpoint. A `WalkHere` third operand leaves a reflexive
equality of stuck walk terms (`Refl` in the checked D0); in a
`WalkStep` case, congruence through the final `WalkStep` transports
the induction hypothesis without deleting or permuting an edge. No
new public helper or private proof helper is required. Neither theorem
changes the definition of `Walk`,
`walk_append`, or the existing §1–3 laws. In particular, right
`WalkHere` remains an **unexported reduction**, not a third public
unit law. These statements use level-zero `Walk` data in `Type` and
its checked `Equal` proposition at `Omega_0`; they do not introduce
an equality primitive or a new source of trust.

The Architect's isolated AC-0 development on landed base `d4f2b202c`
checked the two generic bodies and a separate generic client with rc 0,
no `Axiom`, Transparent package-owned theorems and unchanged exact
`trusted_base()` identities (114 → 114). On open inputs, `Refl` was
refused for both laws: the claims are not mere definitional equality.
The corresponding closed §4 seed uses a four-vertex diamond, exercises
`WalkHere` in each of the three associativity positions, and contrasts
two genuinely different routes with a same-input accepted route. That
D0 measurement predates the package delivery. The catalog now exports
both checked laws, its public-law count is thirteen, and a package-owned
test root-loads both names and exercises the §4 seed's accepted client
and isolated refused route. This is a targeted package-test observation,
not official conformance-runner execution, CI, publication, landing, or
a landed trust measurement.

## 5. Reachability carries back along a walk

> **Section 5 status: DRAFT v0
> (SPEC-GRAPHS-REACHABILITY-ALONG-WALK).** The Architect's isolated D0
> checked the generic proof on `454adf947` without a new `Axiom` or
> trusted GlobalId. The checked catalog export and targeted package
> test are now on main. That test root-loads the generic client and
> accepted §5 seed, rejects the isolated false claim, and measures no
> local trust delta; it is not official conformance or landed trust.

`Algorithm.Graphs.Dependency` must export this Transparent checked law,
with its public name, argument order and result type exactly as follows:

```ken
pub theorem graph_reachable_along_walk
      (q : Type)
      (fq : Finite q)
      (g : Graph q)
      (s : q)
      (t : q)
      (target : q → Bool)
      (walk : Walk q g s t)
    : Equal Bool (graph_reachable q fq g t target) True
      → Equal Bool (graph_reachable q fq g s target) True
```

The `walk` is a **data argument**, not a second premise. The single
premise says that `t` reaches a vertex satisfying `target`; the result
says the same of `s`. For `walk = WalkHere`, `s` and `t` coincide, so
the law is the identity on that premise. For a nonempty walk the
conclusion holds regardless of its length or how the graph's finite
vertex certificate enumerates vertices. This law complements §1.2's
`graph_reachable_complete`: that theorem consumes a walk from the
start to an already-targeted endpoint and a checked target hit;
this theorem consumes a walk to a vertex **from which** some target
is reachable. Neither reverses the direction of graph edges.

The converse is false. A walk from `s` to `t` and reachability from
`s` need not imply reachability from `t`. For instance, on the
one-way chain `V0 → V1 → V2`, take `s = V0`, `t = V2`, the composed
two-edge walk, and the target predicate "is `V0`": `V0` reaches
itself by `WalkHere`, but `V2` cannot get back to `V0`. The seed's
independent closed value checks confirm this asymmetry, and its
isolated false claim refuses reachability from `V2` to `V0`.
Neither a closed instance nor the false-converse refusal proves
the generic forward law.

The checked proof reuses the **existing private** `require_some` to
extract a `ReachWitness q g t target` from §1.2's
`graph_find_walk_some` result. The witness carries a walk from `t`
to a target vertex `u`; §3's `walk_append` joins it to `walk`, then
§1.2's `graph_reachable_complete` gives the result at `s`. This
adds **no new private helper** and exports none of the existing
private `require_some`, `walk_reachable`, or `edge_reach_trans`.
All §1–§4 operations and laws remain unchanged. There is no new
search, graph carrier, axiom, primitive, kernel rule, surface form,
or trust entry.

The Architect's D0 additionally checked the generic client including
the `WalkHere` case and a three-vertex seed with independent closed
`Proved` value controls. The sole proposed seed refusal, an end-to-start
reachability claim on that chain, was kernel-rejected with
`TypeMismatch` after the whole accepted client; an accepted
same-shape `Proved` control and a checked `False` result isolate the
false conclusion. Those earlier checks used an isolated proposed implementation. The
landed catalog's package test checks the §5 seed against its export;
neither observation is official conformance or landed trust.

## 6. Reachability is monotone in the target predicate

> **Section 6 status: DRAFT v0
> (SPEC-GRAPHS-REACHABILITY-TARGET-MONOTONE).** Architect's isolated
> D0 on `abafe3963` checked the generic proof and an independent client,
> with no new `Axiom`, `ken check` and `ken fmt --check` rc 0, and equal
> scratch `trusted_base()` GlobalId sets (114 → 114). The earlier
> spec-and-seed candidate did not deliver the §6 catalog export. The
> separate Foundation candidate exports the checked law, root-loads it,
> checks the complete accepted seed and refuses its isolated converse
> in a package-owned test. Those are candidate-local checks, not official
> conformance-runner execution, CI, publication, landing or landed trust.

`Algorithm.Graphs.Dependency` must export this public Transparent law
immediately after `graph_reachable_along_walk`. The pointwise
`included` proof is the **sole** premise: every vertex satisfying
`target` also satisfies `wider`. Its conclusion takes affirmative
reachability for `target` at `s` to affirmative reachability for
`wider` at the same `s`, `g`, and `fq`.

```ken
pub theorem graph_reachable_target_monotone
      (q : Type)
      (fq : Finite q)
      (g : Graph q)
      (s : q)
      (target : q → Bool)
      (wider : q → Bool)
      (included : (v : q) → Equal Bool (target v) True → Equal Bool (wider v) True)
    : Equal Bool (graph_reachable q fq g s target) True
      → Equal Bool (graph_reachable q fq g s wider) True
```

The input order is `q`, `fq`, `g`, `s`, `target`, `wider`,
`included`, then the reachability hypothesis as an arrow. The
pointwise proof uses only public names and the same checked
`Equal Bool (target v) True` hit form carried by `ReachWitness`.
No `DecEq q`, nonempty-walk premise, strict-inclusion premise, or
extra graph assumption is needed. The law holds when the two targets
are equal, when the start itself satisfies the target, and when a
nonempty walk reaches it. It does **not** state the converse:
reachability of a wider target need not imply reachability of its
narrower subset.

The proof reuses §1.2's `graph_find_walk_some` and private
`require_some` to obtain a `ReachWitness` for the narrower target.
Its checked target hit is transported through `included` at the
witness endpoint; §1.2's `graph_reachable_complete` then establishes
the wider result along the same genuine walk. No new helper, search,
operation, graph carrier, kernel rule, surface feature, axiom, or
trust entry is required. `require_some`, `walk_reachable`, and
`find_walk_selected` remain private. All §1–§5 operations and laws
are unchanged.

Architect's D0 accepted both an independent generic client and a
closed three-vertex one-way-chain client. The narrower predicate
selects `V1`; the wider one selects `V1` and `V2`. From `V0`, both
reachability decisions check as `True`, and the law applies across a
real edge. From `V2`, the wider predicate is `True` while the narrower
predicate is `False`; an isolated false-converse claim is rejected
after the same accepted prefix. These finite controls discriminate
the direction but do not prove the quantified law. The D0's checked
proof and seed client were scratch evidence, not a §6 catalog delivery
or official conformance result. The separate Foundation candidate now
exports the checked law; its package-owned test checks the complete
accepted seed and isolated false converse. Neither candidate-local
checking nor the earlier D0 establishes official conformance-runner
execution, CI, publication, landing or landed trust.

## 7. Reachability is monotone under edge inclusion

> **Section 7 status: DRAFT v0
> (SPEC-GRAPHS-REACHABILITY-GRAPH-MONOTONE).** Architect's isolated D0
> on `944cabd38344c9118f50692b510ac57da7ec8bf8` checked the generic
> proof and independent client with `ken check` and `ken fmt --check` rc 0,
> no `Axiom`, and equal scratch `trusted_base()` identity sets (114 → 114).
> This assembled candidate adds the catalog export, the conformance seed and
> a package-owned test; it does not establish official conformance execution,
> CI, publication, landing, or landed trust.

`Algorithm.Graphs.Dependency` must export the following public Transparent law
immediately after `graph_reachable_target_monotone`. The pointwise `included`
proof is the sole premise on the graphs: each true edge of `g` is also a true
edge of `h`. The final arrow takes affirmative reachability from `s` to the
same `target` in `g` and yields affirmative reachability in `h` at the same
`fq`, `s`, and `target`.

```ken
pub theorem graph_reachable_graph_monotone
      (q : Type)
      (fq : Finite q)
      (g : Graph q)
      (h : Graph q)
      (s : q)
      (target : q → Bool)
      (included : (u : q)
        → (v : q)
        → Equal
        Bool
        (edge q g u v)
        True
        → Equal
        Bool
        (edge q h u v)
        True)
    : Equal Bool (graph_reachable q fq g s target) True
      → Equal Bool (graph_reachable q fq h s target) True
```

The inputs occur in the order `q`, `fq`, `g`, `h`, `s`, `target`,
`included`, then the reachability hypothesis. No `DecEq q`, strict
inclusion, new search, target inclusion, or separate finite certificate
for `h` is required. In particular, the result includes the same-graph
case. It does not state the converse: an edge added to `h` may make a
previously unreachable target reachable.

The proof uses §1.2's `graph_find_walk_some` and private `require_some` to
extract a `ReachWitness` in `g`. A new **private** `walk_lift` recursively
rebuilds that witness walk in `h`: `WalkHere` stays at the same vertex,
and each `WalkStep` gets its checked `h` edge from `included` applied to
the corresponding checked `g` edge. The target-hit proof is unchanged;
§1.2's `graph_reachable_complete` applies to the lifted walk. `walk_lift`
is not a public operation, and `require_some` and `walk_reachable` remain
private. All §1–§6 operation and law types are unchanged. No new graph
carrier, axiom, kernel or surface rule, or trusted-base entry is needed.

The fixed §7 conformance seed path is
`conformance/stdlib/graphs/seed-reachability-graph-monotone.md`. Its
three-vertex `chain` has the genuine two-edge walk V0 → V1 → V2;
`looped` adds only V2 → V0. From V0, both graphs reach target V2.
Independent closed `Proved` controls check both reachability values,
the missing and added edge, and that V0 is not already a target. A
separate law application transports affirmative reachability in `chain`
to `looped` using pointwise edge inclusion. From V2, `looped` reaches
V0 but `chain` does not; an isolated `seed_neg` claiming the converse
is refused at its own `Proved` after the complete accepted prefix.
The inclusion proof uses `λe. Proved` for true-edge arms and
`λe. absurd e` for false-edge arms. The Architect measured that all
seven closed Bool rows refuse `Refl`; their proof form is `Proved`.
These finite cases discriminate direction but do not prove the generic
law. The D0's checked development is scratch evidence, not catalog
delivery, official conformance, CI, publication, landing, or landed trust.

## 8. Reverse a directed graph and its walks

> **Section 8 status: DRAFT v0
> (SPEC-GRAPHS-REVERSE-REACHABILITY).** Architect's isolated AC-0 on
> `77d13c94b22f7b9f7886edf610d8a7c3b6db31b9` checked the four
> generic forms and seed controls with rc 0, no `Axiom`, and unchanged
> scratch `trusted_base()` identities (114 → 114). All four checked
> catalog forms, the package-owned test and the authored
> `conformance/stdlib/graphs/seed-reachability-reverse.md` are now on
> main. The named test previously passed 2/2 with reported
> candidate-local trusted identities 114 → 114; CV statically checked
> the seed but did not run it as Ken or through the official runner.
> `cat_graphs_dependency` timed out without a verdict. Neither D0 nor
> those earlier candidate-local results establish official conformance
> execution or a measured landed trust result.

`Algorithm.Graphs.Dependency` must export the following four public
Transparent forms, in this order, immediately after
`graph_reachable_graph_monotone` and before `contains`. Their checked
signatures are:

```ken
pub fn graph_reverse (q : Type) (g : Graph q) : Graph q

pub theorem graph_reverse_edge
      (q : Type) (g : Graph q) (s : q) (t : q)
    : Equal Bool (edge q (graph_reverse q g) s t) (edge q g t s)

pub fn walk_reverse
      (q : Type) (g : Graph q) (s : q) (t : q) (walk : Walk q g s t)
    : Walk q (graph_reverse q g) t s

pub theorem graph_reachable_reverse
      (q : Type)
      (fq : Finite q)
      (g : Graph q)
      (s : q)
      (t : q)
      (target : q → Bool)
      (walk : Walk q g s t)
    : Equal Bool (target s) True
      → Equal Bool (graph_reachable q fq (graph_reverse q g) t target) True
```

`graph_reverse q g` flips the directed adjacency without changing the
carrier: its edge from `s` to `t` is `edge q g t s`. The public
`graph_reverse_edge` law states this equation as `Equal Bool`, closed by
`Refl`; callers need not inspect the reversed graph's representation.
Neither construction nor the edge law requires `Finite q` or `DecEq q`.

`walk_reverse` is a public **function returning `Walk` data**, not a
proposition about a walk. A `WalkHere` reverses to `WalkHere`; a
`WalkStep` from `prev` to `end` uses its edge proof as the first edge
from `end` to `prev` in the reversed graph, prepended to the recursively
reversed prefix. The existing `walk_prepend` helper stays private;
the same definitional edge flip that closes `graph_reverse_edge` by
`Refl` lets the existing checked edge proof serve the reversed step.
No new helper or alternative graph carrier is introduced.

`graph_reachable_reverse` consumes the supplied walk from `s` to `t`;
its **only** remaining premise is `Equal Bool (target s) True`. The
reversed walk starts at `t` and ends at `s`, so §1.2's existing public
`graph_reachable_complete` yields affirmative reachability from `t` in
`graph_reverse q g`, under the same `fq` and `target`. A walk is an
argument, not an additional propositional premise. There is no claim
that `s` is reachable from `t` in the original directed graph; reversing
only the walk or only the target would not establish this result. All
§§1–7 operation and law types remain unchanged, and no `Axiom`, kernel,
surface change or second fixed-point search is needed.

The authored §8 seed is
`conformance/stdlib/graphs/seed-reachability-reverse.md`. Its accepted
client constructs a two-edge walk V0 → V1 → V2 and states that its
reverse makes V2 reach target V0 in the reversed graph. An accepted
reversed-graph value control accompanies the isolated refusal of the
same reachability claim on the forward chain, where no edge leaves V2.
CV statically checked the seed but did not run it as Ken or through the
official conformance runner. Seed authorship and static checks are not
candidate QA or a conformance-execution result.

## 9. Reversal of a composed walk

> **Section 9 status: DRAFT v0
> (SPEC-GRAPHS-WALK-REVERSE-APPEND).** Architect AC-0 on exact base
> `29511b74f` checked the generic theorem and private helper with
> `ken check` and `ken fmt --check` rc 0, no added `Axiom`, and equal
> scratch `trusted_base()` identity sets 114 → 114. This is isolated
> design evidence, not a candidate catalog check or landed trust
> result. This spec-only child adds no §9 catalog export, package test
> or CV-authored seed. No §9 official conformance-runner execution,
> CI, publication, landing or measured landed trust is established.

`Algorithm.Graphs.Dependency` must export one public Transparent law
immediately after `graph_reachable_reverse` and before `contains`.
Its arguments follow §3's `walk_append`: a graph, start `s`, joining
vertex `t`, end `u`, and the two walks in forward order. Its checked
public type has no additional premise:

```ken
pub theorem walk_reverse_append
      (q : Type)
      (g : Graph q)
      (s : q)
      (t : q)
      (u : q)
      (first : Walk q g s t)
      (second : Walk q g t u)
    : Equal
        (Walk q (graph_reverse q g) u s)
        (walk_reverse q g s u (walk_append q g s t u first second))
        (walk_append q (graph_reverse q g) u t s
          (walk_reverse q g t u second)
          (walk_reverse q g s t first))
```

Both sides are **walk data** in the reversed graph from `u` to `s`:
reversal of a joined forward walk equals the join of its reversed
parts in the **opposite** order. Equality here is between the walks,
not merely their endpoints or their reachability results. The law
holds for arbitrary `q`, `g` and walks, including a zero-edge part;
it needs neither `Finite q`, `DecEq q` nor an edge proposition beyond
the checked edges already carried by `first` and `second`.

The generic proof inducts on the written `second` walk, as §3's
`walk_append` does. In the `WalkHere` case, the symmetric direction
of §4's public `walk_append_here_left` in the **reversed** graph
closes the goal. In a `WalkStep` case, §8's `walk_reverse` prepends
the reversed final edge. `cong` lifts the induction hypothesis under
the existing private `walk_prepend`; `trans` then uses a new private
`walk_prepend_append` lemma to commute that prepend across the
reversed append. The helper itself inducts on its written back walk;
it belongs immediately after `walk_prepend` and before
`walk_step_selected`. Neither helper is exported. The original edge
proof converts to the reversed-edge requirement by §8's definitional
flip; no new edge lemma, premise, kernel form, surface rule or source
of trust is required. All §1–§8 public operation and law types stay
unchanged.

The later CV seed must use a three-vertex chain **with a V0 → V2
shortcut**, not the chain alone: the reversed graph then has two
distinct walks from V2 to V0, one through V1 and one direct. On the
composed V0 → V1 → V2 walk, the accepted law and a closed equality of
its reversed result with the route through V1 form the positive
control. An isolated equality with the direct shortcut walk must be
refused at its own `Refl`, paired with an accepted same-form `Refl`
claim for the route through V1. `Proved` is refused even for a true
closed `Walk` equality, so it cannot discriminate this false walk
claim; edge proofs inside `WalkStep` instead use `Proved` for their
`Equal Bool` claims. A well-typed direct walk does not become equal to a different
same-endpoint walk by this law. These closed observations delimit the
contract but do not replace its generic checked proof; no §9 seed is
authored or executed by this spec-only turn.
