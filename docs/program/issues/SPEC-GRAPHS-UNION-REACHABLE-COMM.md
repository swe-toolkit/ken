---
id: SPEC-GRAPHS-UNION-REACHABLE-COMM
title: "Spec 62 §15 preserves each operand's reachability into graph_union but states no law relating the two operand orders, so a client that built union g h cannot use a reachability fact about union h g. Rule from a checked D0 at zero trust that graph_reachable over graph_union g h equals graph_reachable over graph_union h g as Equal Bool, and state it with a conformance seed"
status: active
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Graphs follows Linear algebra in the L3 layer rotation (docs/program/06-catalog-campaign.md). Steward-filed per COORDINATION section 2."
---

# Reachability in a union does not depend on operand order

## Objective

`Algorithm.Graphs.Dependency` exports a public checked law that, for
any `fq : Finite q`, `graph_reachable q fq (graph_union q g h) s
target` equals `graph_reachable q fq (graph_union q h g) s target` as
`Equal Bool`. It holds at zero trust, and spec 62 states it by its
public type.

## Settled inputs (main `cb25f536d`)

- **The gap.** §15 (`spec/50-stdlib/62-graphs.md:1316`) adds
  `graph_union` with six declarations. Two of them preserve one
  operand's reachability (`graph_union_reachable_left`, `_right`). No
  law relates `union g h` to `union h g`, at the edge or for
  reachability. No `bool_or` commutativity law is public in `Core`.
- **The operation.** `graph_union` is `MkGraph q (λs. λt. bool_or
  (edge q g s t) (edge q h s t))` (`Dependency.ken.md:478`).
- **The bridges exist.** `graph_union_edge_cases` (`:496`) gives the
  surviving operand of a true union edge. `graph_union_edge_left` and
  `_right` (`:486`, `:491`) inject it into the other order.
  `graph_reachable_graph_monotone` (`:230`) lifts that edge inclusion
  to reachability.
- **Seeds and tests.** The seed sits beside
  `conformance/stdlib/graphs/seed-graph-union.md`. The test pattern is
  `crates/ken-elaborator/tests/cat_graphs_union.rs`. Spec 62 ends at
  §15, so the new section is §16.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 62 §16, the catalog law with any private helper, a seed, a
package test and the status lines. No §1 to §15 operation or law type
moves, and there is no kernel or surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-OPTIMIZATION-LEAST-K-COST-ORDER-INVARIANCE.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules:
  - the name and the binder order, following
    `graph_union_reachable_left`;
  - the placement in the export order;
  - whether the Bool equality from the two true-implications is a
    private Dependency helper or a public `Core` law, naming its
    owner;
  - whether the edge-level commutation is also public.
- **AC-1.** §16 states the ruled law by its public type. The Spec
  Leader and the CV review it, with a Decision.
- **AC-2.** The seed works on §15's closed three-vertex chain, where
  `g_only` has only V0 → V1 and `h_only` only V1 → V2.
  - **Accepted block.** It applies the law from V0 to the target V2,
    on both operand orders. Both sides are `True`.
  - **Refused block.** It is on a fresh accepted prefix and in the
    same form as an accepted row. It claims that the union's
    reachability equals `g_only`'s alone, which is `False`.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise
  beyond `Finite q`.
- A change to an existing §1 to §15 operation or law type.

Checks:
- **4 fired.** The obligation is written out:
  - **Edge inclusion.** For each `u` and `v`,
    `graph_union_edge_cases` on a true `union g h` edge gives `Inl`
    or `Inr`. `graph_union_edge_right h g` or `_left h g` then gives
    the true `union h g` edge.
  - **Reachability.** `graph_reachable_graph_monotone` with that
    inclusion gives the true-to-true implication. The symmetric
    instance gives the converse.
  - **Bool equality.** A case split on the two Booleans closes it.
    No public lemma turns the two implications into `Equal Bool`, so
    AC-0 rules the helper.
- **3 fired.** No `_comm` law occurs in `Algorithm/Graphs` or spec 62.
  `graph_union` occurs in Dependency, its seed, its test and spec 62.
- **8 fired.** A law that drops one operand accepts the AC-2 refused
  row. The real union reaches V2 from V0 through both edges, while
  `g_only` alone does not, so the real union refuses that row.
