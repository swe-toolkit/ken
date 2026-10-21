---
id: SPEC-GRAPHS-WALK-COMPOSITION-ORDER
title: "Spec 62 has walks and dependency orders but no public way to join two walks, and the fact that an order puts the start of every walk before its end is private (walk_before). Rule walk concatenation and the order-respects-walk law from a checked D0 at zero trust, and state them as a section 3 with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 62 section 2 proves cycle_refutes_order through the private walk_before; clients cannot compose walks or use the order on a walk. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md). Same shape as SPEC-LINEAR-ALGEBRA-MATRIX-SUM-DISTRIBUTIVITY. Steward-filed per COORDINATION section 2."
---

# Walk composition and order

## Objective

Spec 62 states a public concatenation of two walks in
`Algorithm.Graphs.Dependency`, and a checked law that every
`TopoOrder` puts the start of a walk that takes at least one edge
before its end.

## Settled inputs (spec 62 §1 and §2, on `main` at `3809ccdd4`)

- **Walks (§1.1).** `Walk q g s t` is built by `WalkHere` and by
  `WalkStep`, which appends one checked edge at the end. There is no
  public concatenation.
- **Order (§1.3, §2).** `TopoOrder` carries `all_edges_forward`. §2
  has `before_irreflexive`, `before_transitive` and
  `cycle_refutes_order`. Its proof uses the private `walk_before`
  (`catalog/packages/Algorithm/Graphs/Dependency.ken.md`, near line
  1880), which spec 62 §2 names as a private helper.
- **Package.** Tested by
  `crates/ken-elaborator/tests/cat_graphs_dependency.rs`, with a
  census row in `lang_mod_strict_resolution_d0.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

A new spec 62 section stating each ruled operation and law by its
public type, and a closed conformance seed under
`conformance/stdlib/graphs/`, with the status lines changed in the
same candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0 and no `Axiom`. It rules:
  - the concatenation's name, argument order and definition by
    recursion on the second walk;
  - the order law's exact statement, for a walk with a first edge,
    and whether it is `walk_before` made public or a new statement
    over it;
  - whether concatenation with `WalkHere` on either side needs a
    stated law, or holds by computation only on one side.
- **AC-1.** The section states each ruled operation and law by its
  public type. The Spec leader and CV review it, with a Decision.
- **AC-2.** The seed checks a closed concatenation of two non-trivial
  walks on a small graph, and the order law on a three-vertex chain.
  It has a refused block that claims the end of that walk is before
  its start, with a same-input accepting control for the correct
  direction.

## Stop conditions

- A law that needs a premise beyond §1 and §2's existing evidence,
  unless the D0 rules it as the stated premise.
- Any new primitive, kernel or surface feature, or a change to a §1 or
  §2 operation or law.
- An ambient name in the package beyond its current census row: import
  it explicitly.

Checks: 4 fired: `Walk`, `WalkStep`, `TopoOrder` and
`cycle_refutes_order` are `pub` in spec 62 at `3809ccdd4`, and
`walk_before` is private in the package. 7 fired: the refused block
reverses the order on the same walk, which a seed checking only that
some order exists would accept.
