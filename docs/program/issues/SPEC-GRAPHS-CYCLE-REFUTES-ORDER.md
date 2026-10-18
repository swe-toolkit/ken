---
id: SPEC-GRAPHS-CYCLE-REFUTES-ORDER
title: "Spec 62 section 1 decides OrderOrCycle but excludes the theorem that makes its two outcomes exclusive: nothing proves that a graph with a Cycle has no TopoOrder, so HasCycle does not refute every order. Rule from a checked D0 the cycle-refutes-order law and the irreflexivity and transitivity of before it needs, at zero trust, and state them as section 2 with a conformance seed"
status: active
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 62 section 1.4 excludes an exported cycle-refutes-order theorem. Graphs/dependency is a roadmap layer (docs/program/06-catalog-campaign.md); non-Lane-B layers are deepened in place. Same shape as SPEC-TIME-WINDOW-MEMBERSHIP. Steward-filed per COORDINATION section 2."
---

# A cycle refutes every topological order

## Objective

Spec 62 §2 states, as a checked law, that a graph with a `Cycle` has no
`TopoOrder`, so `HasCycle` from `dependency_order_or_cycle` refutes
every order and the two outcomes are exclusive.

## Settled inputs (spec 62 §1, on `main`)

- **Data (§1.1, §1.3).** `Cycle q g` is `MkCycle s next` with a checked
  edge `s → next` and a `Walk q g next s`; a zero-edge walk alone is not
  a cycle. `TopoOrder q d g` carries `xs` with `no_duplicates`,
  `all_vertices` and `all_edges_forward`, the last stated by
  `before q d x y xs`.
- **`before` (§1.3).** A transparent Boolean list scan for the first
  entry equal to `x` or `y`. It is `True` exactly when that entry equals
  `x` but not `y` and some entry equal to `y` occurs.
- **The gap (§1.4).** "So does an exported cycle-refutes-order theorem:
  it would need checked irreflexivity and transitivity of `before`, and
  no §1 consumer requires it."
- **Package.** `catalog/packages/Algorithm/Graphs/Dependency.ken.md`,
  tested by `crates/ken-elaborator/tests/cat_graphs_dependency.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

Spec 62 §2: the cycle-refutes-order law and the `before` laws it uses,
each by its public type, with a closed conformance seed under
`conformance/stdlib/graphs/`.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed §1 base,
  with rc 0 and no `Axiom`. It rules:
  - the law's statement, `TopoOrder q d g → Cycle q g → ` an empty
    proposition, and which empty type;
  - irreflexivity and transitivity of `before`, with any premise they
    need (`no_duplicates`, `DecEq` laws), stated in delivered
    vocabulary;
  - which helpers are public laws and which stay private.
- **AC-1.** §2 states each ruled law by its public type, and §1.4's
  exclusion is rewritten to match. The Spec leader and CV review it,
  with a Decision.
- **AC-2.** The seed refutes a closed order on a closed cyclic graph,
  including a self-loop. It has a positive control (an acyclic graph
  whose order checks) and a refused block that builds a `Cycle` on that
  acyclic graph from an edge proof the graph does not supply.

## Stop conditions

- A law that needs a premise beyond §1's data and `DecEq`, unless the D0
  rules it as the stated premise.
- Any new primitive, kernel or surface feature.
