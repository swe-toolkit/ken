---
id: SPEC-GRAPHS-ORDER-EDGE-ANTITONE
title: "Spec 62 §7 makes reachability monotone when edges are added, but no law moves a dependency order the other way: a TopoOrder of a graph is not known to order a graph with fewer edges, so a client that prunes dependencies must rebuild the order's three proofs by hand. Rule from a checked D0 at zero trust the public law that a TopoOrder of g orders every h whose true edges are true in g, on the same vertex list, and state it with a conformance seed"
status: active
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-NAT-ADD-ZERO-INVERSION-PUBLIC]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Graphs follows Linear algebra in the L3 layer rotation (docs/program/06-catalog-campaign.md, graphs/dependency structures). Steward-filed per COORDINATION section 2."
---

# Fewer dependencies keep the order

## Objective

`Algorithm.Graphs.Dependency` exports a public checked law. For every
`q`, `d : DecEq q` and `g`, `h : Graph q`, given that each true edge of
`h` is a true edge of `g`, a `TopoOrder q d g` yields a
`TopoOrder q d h` whose `ordered_vertices` is the same list. Spec 62
states it by its public type.

## Settled inputs (main `c93262115`)

- **The order.** `TopoOrder` (`Dependency.ken.md:518`) packs a list
  with `no_duplicates` (`:502`), `all_vertices` (`:512`) and
  `all_edges_forward` (`:515`). Only the last mentions the graph: every
  true edge `x → y` puts `x` before `y`. The projections are
  `ordered_vertices` (`:529`), `ordered_complete`, `ordered_unique` and
  `ordered_forward` (`:536` to `:550`).
- **The neighbour.** `graph_reachable_graph_monotone` (`:230`, spec 62
  §7 at `62-graphs.md:695`) takes the pointwise `included` premise in
  the direction g ⊆ h. This law takes the same premise shape with the
  graphs the other way: h's true edges are g's.
- **Consumers.** `cycle_refutes_order` (`:588`) and
  `graph_reachable_antisymmetric` (`:631`) take a `TopoOrder`. No
  public law produces one except `dependency_order_or_cycle` (`:679`).
  `TopoOrder` has no use outside `Dependency.ken.md` in `catalog/`.
- **The spec.** Spec 62 runs §1 to §12; §12 starts at
  `62-graphs.md:1100` and the file ends at `:1171`.
- **Seeds and tests.** Seeds go under `conformance/stdlib/graphs/`;
  `seed-reachability-graph-monotone.md` is the nearest neighbour. The
  package test pattern is
  `crates/ken-elaborator/tests/cat_graphs_reachability_graph_monotone.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The ruled law in Dependency, a spec 62 section stating it by its public
type, a closed conformance seed and a named package test, in one
candidate.

## Acceptance

- **AC-0 (D0).** The Architect specifies a development on the landed
  base: the source, and the named-binary commands with a bounded
  timeout. The Foundation implementer runs it on scratch and reports
  rc, the `Axiom` grep, and `trusted_base()` as an identity set before
  and after. The Architect rules from the report and does not build or
  test. It rules:
  - the law's name, argument order and placement;
  - whether the same-list property is a definitional consequence or a
    second public law;
  - that no §1 to §12 operation or law type changes.
- **AC-1.** The section states the ruled law by its public type. The
  Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client orders a closed graph, removes
  one edge, and applies the law to obtain the subgraph's order. Its
  refused block applies the law to a graph that adds an edge running
  backward in the order, and the inclusion proof's arm for that edge
  fails. The accepted same-form control is the removal application, so
  the refusal comes from the failed inclusion, not the form.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or a premise
  beyond pointwise edge inclusion.
- A change to an existing §1 to §12 operation or law type.

Checks: 4 fired: `TopoOrder`, `all_edges_forward`, `no_duplicates`,
`all_vertices` and `graph_reachable_graph_monotone` were read at their
lines on `c93262115`; only `all_edges_forward` takes the graph. 3 fired:
grepped `catalog/` for `TopoOrder`, used only in `Dependency.ken.md`; no
public law there maps one `TopoOrder` to another. 8 fired: the refused
application and its control differ only in the target graph, and the
added edge runs backward in the order.
