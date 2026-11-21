---
id: SPEC-GRAPHS-UNION-REACHABLE-ABSORB
title: "Spec 62 says union only adds reachability (§15) and is invariant under operand order and grouping (§16, §17), but no law says a union adds nothing when one operand's edges are already edges of the other, so a client cannot drop a redundant or repeated operand without proving edge inclusion and monotonicity both ways itself. Rule from a checked D0 at zero trust that, when every edge of h is an edge of g, graph_reachable over graph_union g h equals graph_reachable over g, with union of a graph with itself as the corollary, and state both with a seed that separates an included operand from one adding an edge"
status: merged
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Graphs follows Linear algebra in the L3 layer rotation (docs/program/06-catalog-campaign.md). graph_union has left and right inclusion, commutation and association laws, and no absorption or idempotence law. Steward-filed per COORDINATION section 2."
---

# A union with an included graph adds no reachability

## Objective

`Algorithm.Graphs.Dependency` exports two public checked laws. For every
`q`, `fq : Finite q`, `g`, `h`, `s` and `target`, given that every edge of
`h` is an edge of `g`, `graph_reachable q fq (graph_union q g h) s target`
equals `graph_reachable q fq g s target` as `Equal Bool`. As a corollary,
the union of `g` with itself has the same reachability as `g`, with no
premise. Both hold at zero trust. Spec 62 states them by their public
types.

## Settled inputs (main `b3aa83d7d`; anchors re-checked at release)

- **The operation.** `graph_union` (`Dependency.ken.md:506`) is
  `MkGraph` of the pointwise `bool_or` of the two edge relations.
  `graph_union_edge_cases` (`:524`) splits a union edge into an `Or` of
  the two operand edges.
- **The neighbours.** `graph_union_reachable_left` (`:530`) gives the
  direction from `g` to the union. `graph_reachable_graph_monotone`
  (`:230`) lifts an edge inclusion to reachability.
  `graph_union_reachable_comm` (`:556`) and `_assoc` (`:581`) already
  join two monotone directions with the package's private
  `bool_eq_from_iff` (used at `:561`, `:592`).
- **The gap.** `git grep -iE 'idem|absorb'` over the package and spec 62
  has no hit.
- **Spec and tests.** Spec 62 ends with §18
  (`spec/50-stdlib/62-graphs.md:1629`), so the new section is §19. The
  seed directory is `conformance/stdlib/graphs/`. The test pattern is
  `crates/ken-elaborator/tests/cat_graphs_union_reachable_comm.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 62 §19, the two catalog laws with any private helper, a seed, a
package test and the status lines. No §1 to §18 operation or law type
moves, and no kernel or surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-GRAPHS-UNION-REACHABLE-COMM.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules the names, the binder order, the form of
  the inclusion premise (matching `graph_reachable_graph_monotone`'s
  `included`), and whether the corollary is stated as its own law or left
  to clients.
- **AC-1.** §19 states both laws by their public types. The Spec Leader
  and CV review it, with a Decision.
- **AC-2.** The seed's accepted block, on a closed three-vertex graph,
  applies the absorption law where `h` holds one edge of `g`, and the
  corollary to `g` itself, each concluding the stated `Equal Bool`.

  Its refused block pins, in the same form as an accepted row and on a
  fresh accepted prefix: the absorption law applied to an `h` holding
  an edge that `g` lacks, with the inclusion premise discharged as in the
  accepted row. The premise fails to check, so the law is not usable when
  `h` adds an edge. The row differs from the accepted one only in `h`.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate, and no Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A law needs an `Axiom`, a kernel or surface change, or a premise beyond
  the edge inclusion.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §18 operation or law type.

Checks: 3 fired. I grepped the package and spec 62 for `idem` and
`absorb`: no hits; the union laws are inclusion, commutation and
association. 4 fired. I wrote the obligation in the delivered
vocabulary: one direction is `graph_union_reachable_left`; the other is
`graph_reachable_graph_monotone` from the union to `g`, whose `included`
splits the union edge by `graph_union_edge_cases`, taking the left arm
as is and the right arm through the premise; `bool_eq_from_iff` joins
them, as in `graph_union_reachable_comm`. The corollary instantiates
`h := g` with the identity inclusion. 8 fired. The refused row's
inclusion premise is false, so it refuses exactly when the law is read
as holding for an `h` that adds an edge.
