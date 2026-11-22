---
id: SPEC-GRAPHS-SUCCESSOR-APPEND-UNION
title: "Spec 62 builds a graph from successor lists (§14) and unions two graphs (§15), but no law relates the two, so a client merging two adjacency sources by appending their lists cannot use the union's reachability laws without re-proving list membership over the append. Rule from a checked D0 at zero trust that the adaptor over pointwise-appended successor lists has the edges of the union of the two adaptors, and state it with a seed that separates the union from either operand alone"
status: ready
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Graphs follows Linear algebra in the L3 layer rotation (docs/program/06-catalog-campaign.md). graph_from_successors has sound and complete laws and graph_union has its edge and reachability laws; nothing connects them. Steward-filed per COORDINATION section 2."
---

# Appended successor lists build the union

## Objective

`Algorithm.Graphs.Dependency` exports a public checked law. For every
`q`, `d : DecEq q`, `f` and `g` of type `q → List q`, and `s` and `t`:
the edge from `s` to `t` in `graph_from_successors q d (λv. list_append q
(f v) (g v))` equals, as `Equal Bool`, the edge from `s` to `t` in
`graph_union q (graph_from_successors q d f) (graph_from_successors q d
g)`. It holds at zero trust, and spec 62 states it by its public type.
With it, the union's reachability laws apply to a graph built from merged
adjacency lists.

## Settled inputs (main `72f02b12a`; anchors re-checked at release)

- **The operations.** `graph_from_successors` (`Dependency.ken.md:940`)
  is `MkGraph` of `contains q d t (successors s)`. `contains` (`:652`)
  recurses on the list with `bool_or (d.eq x y)` at each cons.
  `graph_union` (`:506`) is `MkGraph` of the pointwise `bool_or` of the
  two edge relations. `bool_or` (`Core/Classes/LawfulClasses.ken.md:107`)
  matches on its first argument. `list_append`
  (`Data/Collections/Derived.ken.md:103`) recurses on its first list.
- **The neighbours.** `graph_from_successors_sound` and `_complete`
  (`:943`, `:949`). `graph_union_edge` (`:509`) and
  `graph_union_edge_cases` (`:524`). `list_elem_append_left` and
  `_right` (`Derived.ken.md:212`, `:235`) give membership into an append.
  No law gives membership out of one. The private
  `graph_union_edge_regroup_*` helpers (`:1456`, `:1484`) are
  implication-level, not `Equal Bool`.
- **The gap.** `git grep` of `graph_from_successors` finds only the
  package, its seed, its test, spec 62 §14 and SPEC-PROGRESS. None relates
  it to `graph_union` or to `list_append`.
- **Spec and tests.** Spec 62 ends with §19 (`62-graphs.md:1706`), so the
  new section is §20. The seed directory is `conformance/stdlib/graphs/`.
  The test pattern is
  `crates/ken-elaborator/tests/cat_graphs_successor_list_adaptor.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 62 §20, the catalog law with any private helper, a seed, a package
test and the status lines. No §1 to §19 operation or law type moves, and
there is no kernel or surface change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-GRAPHS-UNION-REACHABLE-ABSORB.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules:
  - the name, the binder order and the placement after
    `graph_from_successors_complete`;
  - whether the law is the edge `Equal Bool` alone, or also exports the
    `graph_reachable` equality it implies;
  - whether the list step, `contains` over `list_append` as a `bool_or`,
    is a private `Dependency` helper or a public `Derived` law;
  - every consumer root of an exported `Dependency` law, swept by the
    sibling `graph_from_successors_complete` across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`, so the candidate's
    scope is complete.
- **AC-1.** §20 states the law by its public type. The Spec Leader and
  CV review it, with a Decision.
- **AC-2.** The seed takes `q = Bool`, `f` listing nothing and `g`
  listing `False` from `True`. Its accepted block concludes, through the
  law, that the appended adaptor's edge from `True` to `False` equals the
  union's. Its refused block, on a fresh accepted prefix and in the same
  form, claims that edge equal to the edge of `graph_from_successors q d
  f` alone. That edge is `False` and the appended one is `True`, so the
  claim refuses. The two rows differ only in the right-hand graph.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §19 operation or law type.

Checks: 4 fired. I wrote the obligation in the delivered vocabulary on
`72f02b12a`. The goal is `contains q d t (list_append q xs ys)` against
`bool_or (contains q d t xs) (contains q d t ys)`, by induction on `xs`.
At `Nil`, both sides reduce to `contains q d t ys` through `bool_or`'s
`False` arm. At `Cons y rest`, a case on `d.eq t y` gives `True` on both
sides, or the induction hypothesis. No associativity law is needed. The
route is AC-0's to rule. 3 fired. The sibling's consumers are the
package, `seed-graph-from-successors.md`,
`cat_graphs_successor_list_adaptor.rs`, spec 62 and SPEC-PROGRESS. AC-0
sweeps the same roots. 8 fired. A law stated over `f` alone, which drops
`g`, is false at the seed's vertices: that edge is `False`, while the
appended edge is `True`. The refused row is exactly that wrong right-hand
side.
