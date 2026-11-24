---
id: SPEC-GRAPHS-TOPO-ORDER-REVERSE
title: "Spec 62 reverses a graph (§§9-12, §18) and keeps a dependency order across edge removal (§13), but no law carries an order across reversal: a client holding a TopoOrder for g cannot obtain one for graph_reverse g without rerunning the pair scan, although the reversed list is one. Rule from a checked D0 at zero trust that reversing an order's vertex list gives a TopoOrder of the reversed graph, and state it with a seed that separates the reversed list from the original"
status: ready
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Graphs follows Linear algebra in the L3 layer rotation (docs/program/06-catalog-campaign.md). graph_reverse has edge, walk and reachability laws and TopoOrder has the edge-antitone transfer; nothing relates the two. Steward-filed per COORDINATION section 2."
---

# A reversed order orders the reversed graph

## Objective

`Algorithm.Graphs.Dependency` exports a public checked transfer. For every
`q`, `d : DecEq q`, `g : Graph q` and `order : TopoOrder q d g`, it
constructs a `TopoOrder q d (graph_reverse q g)` whose `ordered_vertices`
equals, as `Equal (List q)`, `DC.reverse q (ordered_vertices q d g
order)`. It holds at zero trust, and spec 62 states it by its public type.

## Settled inputs (main `a7bcf9d18`; anchors re-checked at release)

- **The operations.** `graph_reverse` (`Dependency.ken.md:287`) is
  `MkGraph` of the flipped edge, and `graph_reverse_edge` (`:289`) states
  it. `TopoOrder` (`:692`) carries a list, `no_duplicates`,
  `all_vertices` and `all_edges_forward`; `before` (`:658`) scans for the
  first of its two vertices. `DC.reverse`
  (`Data/Collections/Derived.ken.md:1540`) appends each head after the
  reversed tail, with `reverse_snoc` (`:1546`) and `reverse_length`
  (`:1600`).
- **The neighbours.** `topo_order_edge_antitone` (`:731`) is the shape of
  a public `TopoOrder` transfer. The private `contains_perm` (`:2298`) and
  `nodup_perm` (`:2580`) move membership and uniqueness along a
  `DC.Perm`. The private `contains_append` (`:3072`) splits membership
  over an append.
- **The new step.** Uniqueness is needed: on `[a, b, a]`, `before a b`
  is `True` and `before b a` of the reversal is `False`. With
  `no_duplicates`, `before y x xs = True` gives `before x y (DC.reverse q
  xs) = True`, which with `graph_reverse_edge` discharges
  `all_edges_forward`.
- **Spec and tests.** Spec 62 ends with §20 (`62-graphs.md:1794`), so the
  new section is §21. The seed directory is `conformance/stdlib/graphs/`.
  The test pattern is
  `crates/ken-elaborator/tests/cat_graphs_topo_order_edge_antitone.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 62 §21, the catalog transfer and its vertex law with any private
helper, any generic `Derived` list law the proof needs, a seed, a package
test and the status lines. No §1 to §20 operation or law type moves, and
there is no kernel or surface change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-GRAPHS-SUCCESSOR-APPEND-UNION.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules:
  - the names, the binder order and the placement after
    `topo_order_edge_antitone`;
  - whether completeness and uniqueness go through a `DC.Perm` of the
    reversal and the existing private helpers, or direct induction;
  - whether the `before` flip under reversal is a private helper, and
    whether any list law lands public in `Derived`;
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`.
- **AC-1.** §21 states the transfer and its vertex law by their public
  types. The Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed takes `q = Bool` and a graph whose only edge is from
  `True` to `False`, with the order `[True, False]`. Its accepted block
  concludes, through the law, that the reversed graph's order lists
  `[False, True]`. Its refused block, on a fresh accepted prefix and in
  the same form, claims that list equals `[True, False]`. The two rows
  differ only in the claimed list.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The transfer needs an `Axiom`, a kernel or surface change, or a premise
  beyond the order itself.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §20 operation or law type.

Checks: 4 fired. I read `graph_reverse`, `TopoOrder`, `before`,
`DC.reverse`, the perm helpers and spec 62's last section on `a7bcf9d18`.
8 fired. A transfer that keeps the original list fails the seed's reversed
edge, and the `[a, b, a]` case shows the flip needs the order's uniqueness,
so a proof that ignores it is wrong. 3 fired. The sibling's consumers are
the package, its seed, its test, spec 62 and SPEC-PROGRESS; AC-0 sweeps the
same roots.
