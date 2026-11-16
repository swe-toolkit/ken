---
id: SPEC-GRAPHS-UNION-REACHABLE-ASSOC
title: "Spec 62 §16 makes graph-union reachability independent of operand order but states nothing about grouping, so a client that unions three graphs one way cannot use a reachability fact about the other grouping. Rule from a checked D0 at zero trust that graph_reachable over graph_union (graph_union g h) k equals it over graph_union g (graph_union h k), and state it with a conformance seed"
status: active
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Graphs follows Linear algebra in the L3 layer rotation (docs/program/06-catalog-campaign.md:376); graph_union has a commutation law for reachability (spec 62 §16) but no grouping law. Steward-filed per COORDINATION section 2."
---

# Reachability is invariant under union grouping

## Objective

`Algorithm.Graphs.Dependency` exports a public checked law that, for every
`fq`, `g`, `h`, `k`, `s` and `target`,
`graph_reachable q fq (graph_union q (graph_union q g h) k) s target`
equals `graph_reachable q fq (graph_union q g (graph_union q h k)) s target`
as `Equal Bool`. It holds at zero trust, and spec 62 states it by its public
type.

## Settled inputs (main `b99258223`)

- **The operation.** `graph_union` (`Dependency.ken.md:478`) is the
  edge-wise `bool_or` of its operands.
- **The precedent.** `graph_union_reachable_comm` (`:528`) closes the
  commuted law with private `bool_eq_from_iff` over two
  `graph_reachable_graph_monotone` calls. Each call takes an edge-inclusion
  premise, private `graph_union_edge_swap` (`:1308`).
- **The edge tools.** `graph_union_edge_left`, `graph_union_edge_right` and
  `graph_union_edge_cases` (`:486`-`:500`) are public.
- **The gap.** No law in `catalog/packages`, `spec/` or `conformance/` relates
  two groupings of `graph_union`, at the edge level or for reachability.
- **Spec and tests.** Spec 62 ends at §16 (`62-graphs.md:1450`), so the new
  section is §17. The test pattern is
  `crates/ken-elaborator/tests/cat_graphs_union_reachable_comm.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 62 §17, the catalog law with any private helper, a seed, a package test
and the status lines. No §1 to §16 operation or law type moves, and no kernel
or surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-GRAPHS-UNION-REACHABLE-COMM.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules:
  - the name and binder order;
  - whether the edge-level grouping step is private, or a public law;
  - the proof route.
- **AC-1.** §17 states the ruled law by its public type. The Spec Leader and
  CV review it, with a Decision.
- **AC-2.** The seed's accepted block applies the law at closed arguments on
  a finite vertex type. There are three graphs, each with one edge the others
  lack, and a target reachable only through a walk that uses an edge from
  each of the three.

  Its refused block pins, in the same form as an accepted row and on a fresh
  accepted prefix:
  - reachability over `graph_union q (graph_union q g h) k` claimed equal to
    reachability over `graph_union q g h` alone, for that target.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise.
- A change to an existing §1 to §16 operation or law type.

Checks: 4 fired. Both groupings have the same edge relation up to `bool_or`
associativity, so two monotonicity steps with an edge-inclusion premise each
close the law, as for commutation. 3 fired: I grepped for `graph_union`
grouping laws and for `bool_or` associativity across `catalog/packages`,
`spec/` and `conformance/`, and found neither. 8 fired: with each graph
contributing one edge of the only walk to the target, the target is
reachable in the three-way union but not in `graph_union q g h`, so the
refused row is false on the closed fixture, and it differs from the accepted
row only in the dropped operand.
