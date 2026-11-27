# Reachability in an edgeless graph

Format: `../../README.md`.
Spec: `spec/50-stdlib/62-graphs.md` §23.
Frame: `docs/program/issues/SPEC-GRAPHS-REACHABILITY-EDGELESS.md`.
Architect AC-0: `evt_5ryk30k2kbm54`.

This seed is authored but unexecuted. No claim is made that the official
conformance runner has executed it.

The Bool fixtures start at `False` with target `is_true`, which is
`False` at the start and `True` at the other vertex. The complete graph
therefore reaches the target, while the edgeless graph does not. A3
starts at `True` in a separately named edgeless graph: it checks that the
law returns the target at the start, rather than always returning
`False` or ignoring the graph argument.

## Accepted prefix

The arbitrary client applies the public law. The closed complete-graph
and edgeless-graph rows are positive controls for `is_true` from
`False`.

```ken
import Data.Finite.Finite (Finite, bool_finite)

import Algorithm.Graphs.Dependency
  (Graph, MkGraph, graph_edgeless, graph_reachable, graph_reachable_edgeless)

theorem gen_edgeless (q : Type) (fq : Finite q) (s : q) (target : q → Bool)
  : Equal Bool (graph_reachable q fq (graph_edgeless q) s target) (target s) =
  graph_reachable_edgeless q fq s target

fn is_true (v : Bool) : Bool = v

const complete_bool : Graph Bool = MkGraph Bool (λu. λv. True)

theorem complete_reaches_true
  : Equal Bool (graph_reachable Bool bool_finite complete_bool False is_true) True =
  Proved

theorem edgeless_misses_true
  : Equal Bool (graph_reachable Bool bool_finite (graph_edgeless Bool) False is_true) False =
  Proved
```

## Accepted block

Every row below accepts. This block is the accepted prefix followed only
by the law application from `False`.

```ken
import Data.Finite.Finite (Finite, bool_finite)

import Algorithm.Graphs.Dependency
  (Graph, MkGraph, graph_edgeless, graph_reachable, graph_reachable_edgeless)

theorem gen_edgeless (q : Type) (fq : Finite q) (s : q) (target : q → Bool)
  : Equal Bool (graph_reachable q fq (graph_edgeless q) s target) (target s) =
  graph_reachable_edgeless q fq s target

fn is_true (v : Bool) : Bool = v

const complete_bool : Graph Bool = MkGraph Bool (λu. λv. True)

theorem complete_reaches_true
  : Equal Bool (graph_reachable Bool bool_finite complete_bool False is_true) True =
  Proved

theorem edgeless_misses_true
  : Equal Bool (graph_reachable Bool bool_finite (graph_edgeless Bool) False is_true) False =
  Proved

theorem edgeless_from_false
  : Equal Bool (graph_reachable Bool bool_finite (graph_edgeless Bool) False is_true) False =
  graph_reachable_edgeless Bool bool_finite False is_true
```

## A3 named-client control

Append this strengthening to the accepted program. The client-defined
constant has the same `MkGraph` body as the public named constant, so the
law application must work by conversion.

```ken
const edgeless : Graph Bool = MkGraph Bool (λu. λv. False)

theorem named_edgeless_from_true
  : Equal Bool (graph_reachable Bool bool_finite edgeless True is_true) True =
  graph_reachable_edgeless Bool bool_finite True is_true
```

### stdlib/graphs/reachability-edgeless

- spec: §23; reachability in the edgeless graph equals `target s`.
- promise class: durable invariant.
- given: arbitrary `q`, `Finite q`, start `s`, and `target : q → Bool`.
- expect: `gen_edgeless` accepts by the public law.
- measured: AC-0 D0 checked the open public-law client on scratch.
- claimed: the law applies for every finite carrier and target.
- gap: this client does not test the theorem's proof body.
- why: the open arguments prevent the law application from being
  established solely by reducing the closed fixtures.

### stdlib/graphs/edgeless-and-complete-controls

- spec: §23 and §1.
- promise class: durable invariant.
- given: Bool vertices, start `False`, target `is_true`, and both the
  complete and edgeless graphs.
- expect: `complete_reaches_true` accepts with `True`, and
  `edgeless_misses_true` accepts with `False`.
- measured: AC-0 D0 checked both closed Boolean computations on scratch.
- claimed: graph structure distinguishes an available edge from no edges.
- gap: the closed values alone do not prove the generic law.
- controls: both computations share the carrier, start and target; only
  the graph changes.
- why: the complete graph reaches `True` from `False` in one step, while
  no edge leaves `False` in the edgeless graph.

### stdlib/graphs/edgeless-law-application-from-false

- spec: §23; the law equates reachability with `target s`.
- promise class: durable invariant.
- given: the accepted prefix and `edgeless_from_false`.
- expect: the entire accepted block accepts; the law gives `False` from
  start `False` with target `is_true`.
- measured: AC-0 D0 checked this law application on scratch.
- claimed: the law's result is the target test at the start vertex.
- gap: the closed instance alone does not establish the generic theorem.
- controls: `gen_edgeless` applies the law with open arguments; the
  complete and edgeless computations give opposite outcomes.
- why: `is_true False` is `False`.

### stdlib/graphs/named-edgeless-converts-to-public-law

- spec: §23; a `MkGraph` client with the same edgeless body uses the
  named constant by conversion.
- promise class: durable invariant.
- given: `edgeless` defined as `MkGraph Bool (λu. λv. False)`, start
  `True`, and target `is_true`.
- expect: `named_edgeless_from_true` accepts with `True`.
- measured: AC-0 D0 checked this named-constant client on scratch.
- claimed: the law works for a client-defined graph with the same body,
  and returns the target at the start.
- gap: this finite client does not prove conversion for arbitrary `q`.
- controls: the generic law is applied directly in `gen_edgeless`; this
  client uses the same `MkGraph` body through a named constant.
- why: `is_true True` is `True`; an always-`False` result or a result
  that ignores the graph fails this row.

## Refuse the complete-graph result for an edgeless search

Run a fresh copy of the accepted prefix, then append only the theorem
below. It differs from `edgeless_from_false` only in claiming `True`
rather than `False`. Require refusal inside this theorem. Do not pin a
message or byte offsets.

```ken
import Data.Finite.Finite (Finite, bool_finite)

import Algorithm.Graphs.Dependency
  (Graph, MkGraph, graph_edgeless, graph_reachable, graph_reachable_edgeless)

theorem gen_edgeless (q : Type) (fq : Finite q) (s : q) (target : q → Bool)
  : Equal Bool (graph_reachable q fq (graph_edgeless q) s target) (target s) =
  graph_reachable_edgeless q fq s target

fn is_true (v : Bool) : Bool = v

const complete_bool : Graph Bool = MkGraph Bool (λu. λv. True)

theorem complete_reaches_true
  : Equal Bool (graph_reachable Bool bool_finite complete_bool False is_true) True =
  Proved

theorem edgeless_misses_true
  : Equal Bool (graph_reachable Bool bool_finite (graph_edgeless Bool) False is_true) False =
  Proved

theorem edgeless_from_false
  : Equal Bool (graph_reachable Bool bool_finite (graph_edgeless Bool) False is_true) True =
  graph_reachable_edgeless Bool bool_finite False is_true
```

### stdlib/graphs/reject-edgeless-reaches-true-from-false

- spec: §23; the result is `target s`, which is `False` for these inputs.
- promise class: durable invariant.
- given: a fresh accepted prefix with the same graph, start and target
  as `edgeless_from_false`.
- expect: the prefix accepts; `edgeless_from_false` with result `True`
  refuses with `KernelRejected(TypeMismatch)` inside that theorem.
- measured: AC-0 D0 refused the result-`True` twin on a fresh prefix.
- claimed: the edgeless search cannot return the complete graph's `True`
  result for this start and target.
- gap: this one input does not establish every instance of the law.
- controls: `edgeless_from_false` accepts with the same prefix and proof
  form, differing only in the claimed result.
- why: `is_true False` is `False`, and no edge leads from `False` to
  `True` in the edgeless graph.

## Evidence boundary

Architect AC-0 checked the client forms, the accepted A3 case, and the
refused twin on scratch at `0a53d387daa602e12718183263de9bd911de3a32`.
This does not mean the Markdown seed was executed. No official
conformance run, assembled AC-T, CI, publication, or landing claim is made.
