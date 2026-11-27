---
id: SPEC-GRAPHS-REACHABILITY-EDGELESS
title: "Spec 62 says reachability grows with the graph (graph_reachable_graph_monotone) and that a union with an included graph adds nothing (§19), but no law gives reachability in the graph with no edges, the bottom of that order: a client that starts a search from an edgeless graph cannot rewrite the result to the target test at the start vertex without unfolding the search. Rule from a checked D0 at zero trust that graph_reachable over MkGraph q (λs. λt. False) from s equals target s, and state it with a seed that separates the start vertex from a vertex one edge away"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Graphs follows Linear algebra in the L3 layer rotation (docs/program/06-catalog-campaign.md). Monotonicity, union and reversal laws exist; nothing states reachability at the empty edge relation. Steward-filed per COORDINATION section 2."
---

# In a graph with no edges, only the start is reachable

## Objective

`Algorithm.Graphs.Dependency` exports a public checked law. For every
`q`, `fq`, `s` and `target` it states

`Equal Bool (graph_reachable q fq (MkGraph q (λs. λt. False)) s target)
(target s)`.

It holds at zero trust, and spec 62 states it by its public type.

## Settled inputs (main `fa83c81f1`; anchors re-checked at release)

- **The operations.** `Graph` is `MkGraph` over an edge function
  (`Dependency.ken.md:64`), `edge` applies it (`:70`), and `graph_reachable`
  runs the reachability DFA from `s` (`:151`). A `Walk` is `WalkHere` or a
  `WalkStep` that carries `Equal Bool (edge q g u v) True` (`:75`).
- **The argument, for AC-0 to check.**
  - Forward: `graph_find_walk_some` (`:172`) turns a True search into a
    present `ReachWitness` (`MkReach t hit walk`, `:89`). By induction on
    `walk`, every `WalkStep` carries `Equal Bool False True` and is absurd,
    so `walk` is `WalkHere` and `hit` is `target s`.
  - Backward: `graph_reachable_complete` (`:161`) with `WalkHere`.
  - The private `bool_eq_from_iff` (`:1658`) and `require_some` (`:1474`)
    in the same file close the Bool equation.
  AC-0 may name the edgeless graph as a new `pub fn` only if the law cannot
  be stated cleanly over `MkGraph`; otherwise there is no new operation.
- **No such law exists.** The reachability laws are `_complete`,
  `_along_walk`, `_target_monotone`, `_graph_monotone`, `_trans`,
  `_reverse`, `_reverse_swap`, `_antisymmetric` and the five union laws.
  None fixes the edge relation, and nothing in `catalog/` or `spec/` names
  an edgeless or empty graph.
- **Spec and tests.**
  - Spec 62 ends with §22 (`62-graphs.md:1939`), so the new section is §23.
  - The seed directory is `conformance/stdlib/graphs/`.
  - The test pattern is
    `crates/ken-elaborator/tests/cat_graphs_reachability_graph_monotone.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 62 §23, the catalog law, a seed, the package test rows and the status
lines. No §1 to §22 operation or law type moves, and there is no kernel or
surface change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-GRAPHS-UNION-REACHABLE-ABSORB.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules:
  - the name, the binder order and the placement;
  - whether the edgeless graph is stated over `MkGraph` or named;
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`.
- **AC-1.** §23 states the law by its public type. CV and the Architect
  review it, with a Decision.
- **AC-2.** The seed works over `Bool` vertices from `s = False` with the
  target `λv. v`, which is False at the start and True one edge away.
  - Its accepted block applies the law and checks that the search in the
    edgeless graph is `False`.
  - Its refused block, on a fresh accepted prefix and in the same form,
    claims the search is `True`, the result in the complete graph. That
    claim must refuse.
  - The two rows differ only in the claimed result. AC-0 picks the closing
    terms and may strengthen the seed.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §22 operation or law type.

Checks: 4 fired. I read `Graph`, `edge`, `Walk`, `ReachWitness`,
`graph_reachable`, `graph_find_walk_some`, `graph_reachable_complete`,
`bool_eq_from_iff` and `require_some` on `fa83c81f1`. 3 fired. I listed every
public reachability law in Dependency and grepped `catalog/` and `spec/` for
an edgeless or empty graph, and found none. 8 fired. From `False` with
target `λv. v`, the edgeless search is False while one edge to `True` makes
it True, so a law that drops the edge relation fails the refused row.
