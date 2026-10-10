---
id: CAT-GRAPHS-DEPENDENCY
title: "No catalog package gives a finite directed graph a decided reachability or a checked dependency order. Deliver section 1 of spec/50-stdlib/62-graphs.md as Algorithm.Graphs.Dependency: Graph, genuine walks and cycles, reachability reused from the landed automata decision, and a complete topological order or a real cycle, fully proved at zero trust"
status: active
owner: foundation
size: M
tier: T2
gate: architect
depends_on: [SPEC-GRAPHS-DEPENDENCY-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. 06-catalog-campaign.md:374, graphs/dependency structures after automata. Spec section 1 from SPEC-GRAPHS-DEPENDENCY-CONTRACT (Architect D0 evt_5kbcdx6e29rkg). Steward-filed per COORDINATION section 2."
---

# Graph reachability and dependency order, delivered and proved

## Objective

`Algorithm.Graphs.Dependency` lands as section 1 of
`spec/50-stdlib/62-graphs.md` specifies, with every law proved and no
new trust.

## Settled inputs (spec at `9501f1a76`)

- **Section 1 is normative.** It names 23 public declarations with their
  types: `Graph`, `edge`, `Walk`, `Cycle`, `ReachWitness`,
  `graph_reachable`, `graph_find_walk`, `graph_reachable_complete`,
  `graph_find_walk_some`, `contains`, `before`, `no_duplicates`,
  `all_vertices`, `all_edges_forward`, `TopoOrder`, `ordered_vertices`,
  `ordered_complete`, `ordered_unique`, `ordered_forward`, `OrderOrCycle`,
  `dependency_order_or_cycle`, `contains_sound` and `contains_complete`.
- **The development exists.** The Architect's D0 `evt_5kbcdx6e29rkg`
  checked the declarations and proofs rc=0 with no Axiom, and both
  wrong-branch mutations kernel-rejected. It is the starting point, not
  a design to revisit.
- **Reuse, no second fixed point.** Reachability reuses the landed
  `Algorithm.FormalLanguages.Reachability` decision and
  `Data.Finite.Finite`. The order ranks deduplicated vertices by
  ancestor count and sorts with `Data.Collections.Derived.sort`. `DecEq q`
  enters only in section 1.3. Section 1 changes no existing Finite, Dfa
  or Reachability contract.
- **The seed** is `conformance/stdlib/graphs/seed-dependency.md`: ten
  cases over `q = Bool` fixtures (eight dependency, two reachability).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. The package, in section 1's order, reading top-down, with its own
   acceptance test target and a strict-resolution ambient-census row.

## Acceptance

- **AC-1.** `ken fmt --check` and `ken check` pass. The acceptance
  target root-loads all 23 public names and runs the ten seed cases with
  their listed results.
- **AC-2.** `trusted_base()` is equal before and after. The package
  declares no Axiom, postulate, primitive or foreign declaration, and no
  unproved `Ord q` instance.
- **AC-3 (consumers).** No existing package's public surface changes. A
  whole-root sweep over `catalog/`, `crates/*/tests`, `r_layer_tests`,
  `examples/`, `conformance/` and the CLI fixtures finds every pin or
  importer of Reachability, Finite and Derived, and keeps each green.
  The Rosetta runner and SEAL-2 pass.
- **AC-4 (mutation, QA).** Each mutant makes a proof kernel-reject, and
  is restored: `dependency_order_or_cycle` returning `HasCycle` on a
  missing edge; an order that drops one vertex; `graph_reachable`
  ignoring `target`.

## Stop conditions

- A law needs an Axiom, `Ord q`, `DecEq` outside section 1.3, or a
  kernel or surface-syntax change.
- Reuse needs a change to Reachability's or Finite's public surface or
  laws.
- A seed case fails on the delivered package.
