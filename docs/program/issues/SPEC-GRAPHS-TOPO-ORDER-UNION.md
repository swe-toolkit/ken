---
id: SPEC-GRAPHS-TOPO-ORDER-UNION
title: "Spec 62 carries a dependency order across edge removal (§13) and reversal (§21), and builds the union of two graphs (§15), but no law orders a union: a client holding a TopoOrder for g whose list also orders every edge of h cannot obtain a TopoOrder for graph_union g h without rerunning the pair scan. Rule from a checked D0 at zero trust that such an order orders the union with the same list, and state it with a seed that separates an h the list orders from one it does not"
status: ready
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Graphs follows Linear algebra in the L3 layer rotation (docs/program/06-catalog-campaign.md). §13 and §21 transfer TopoOrder across removal and reversal; union is the remaining graph operation with no order transfer. Steward-filed per COORDINATION section 2."
---

# One order for both graphs orders their union

## Objective

`Algorithm.Graphs.Dependency` exports a public checked transfer. For every
`q`, `d : DecEq q`, `g`, `h : Graph q`, `order : TopoOrder q d g` and
`forward_h : all_edges_forward q d h (ordered_vertices q d g order)`, it
constructs a `TopoOrder q d (graph_union q g h)` whose `ordered_vertices`
equals `ordered_vertices q d g order` as `Equal (List q)`. Both hold at
zero trust, and spec 62 states them by their public types.

## Settled inputs (main `da3f45295`; anchors re-checked at release)

- **The operations.**
  - `graph_union` (`Dependency.ken.md:506`) is `MkGraph` of the `bool_or`
    of the two edges.
  - `graph_union_edge_cases` (`:524`) splits a true union edge into an
    `Or` of the two true edges.
  - `TopoOrder` (`:692`) carries a list, `no_duplicates`, `all_vertices`
    and `all_edges_forward` (`:689`); its projections are at `:703-729`.
- **The argument, for AC-0 to check.** Reuse the list, its uniqueness and
  its completeness from `order`. For a true union edge, split it by
  `graph_union_edge_cases`: the `g` arm uses `ordered_forward`, and the
  `h` arm uses `forward_h`. No list transport is needed.
- **The neighbour.** `topo_order_edge_antitone` (`:731`) is the shape: the
  same three projections with a rewritten forward proof.
- **No such law exists.** Nothing in `catalog/` or `spec/` constructs a
  `TopoOrder` of a `graph_union`.
- **Spec and tests.**
  - Spec 62 ends with §21 (`62-graphs.md:1885`), so the new section is
    §22.
  - The seed directory is `conformance/stdlib/graphs/`, after
    `seed-topo-order-reverse.md`.
  - The test pattern is
    `crates/ken-elaborator/tests/cat_graphs_topo_order_edge_antitone.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 62 §22, the catalog transfer and its vertex law with any private
helper, a seed, a package test and the status lines. No §1 to §21
operation or law type moves, and there is no kernel or surface change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-GRAPHS-TOPO-ORDER-REVERSE.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules:
  - the names, the binder order and the placement;
  - whether a second public form taking two orders and an
    `Equal (List q)` of their lists is also exported, or left to clients;
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`.
- **AC-1.** §22 states the laws by their public types. CV and the
  Architect review it, with a Decision.
- **AC-2.** The seed uses three vertices, `g` with the one edge V0 → V1
  and the list `[V0, V1, V2]`.
  - Its accepted block takes `h` with the one edge V1 → V2, proves
    `forward_h`, and reads off the union's list through the vertex law.
  - Its refused block, on a fresh accepted prefix and in the same form,
    takes `h` with the one edge V2 → V1. The `forward_h` obligation at
    V2, V1 refuses, because `before V2 V1 [V0, V1, V2]` is `False`.
  - The two rows differ only in the direction of `h`'s edge.
  - AC-0 may replace this seed with a stronger discriminator.
- **AC-T.** The package test compares `trusted_base()` before and after
  Dependency as identity sets, and that comparison fails when the package
  adds an `Axiom`. No Decision opens until it passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The transfer needs an `Axiom`, a kernel or surface change, or a premise
  beyond `forward_h`.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §21 operation or law type.

Checks: 4 fired. I read `graph_union`, `graph_union_edge_cases`,
`TopoOrder`, its projections, `all_edges_forward` and
`topo_order_edge_antitone` on `da3f45295`. The argument uses only those
and an `Or` split. 3 fired. I grepped `catalog/` and `spec/` for a
`TopoOrder` of a union and found none. 8 fired. In the refused row the
list puts V1 before V2, so `before V2 V1` is `False` and the `h` arm of
the union edge has no forward proof.
