---
id: SPEC-GRAPHS-DEPENDENCY-CONTRACT
title: "No graph or dependency-structure contract exists in spec/50-stdlib, so the catalog's graphs layer has nothing to build against. Author spec/50-stdlib/62-graphs.md with its first section: a finite directed graph, decidable reachability, and a topological order or a cycle witness, each law proved at zero trust"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\". 06-catalog-campaign.md:374 places graphs/dependency structures after automata/formal-languages. Same shape as the formal-languages contracts: an Architect boundary ruling from a checked development, then the contract, then the CAT node. Steward-filed per COORDINATION section 2."
---

# A finite graph and dependency-order contract

## Objective

`spec/50-stdlib/62-graphs.md` exists, and its first section is the
normative contract that the first graphs catalog node builds against.

## Settled inputs (on `7101a05f7`)

- **Finite evidence exists.** `Data.Finite.Finite` (spec 61 section 2.1)
  is a certificate over any carrier, with no `DecEq` in the record.
- **A reachability decision exists, for automata.**
  `Algorithm.FormalLanguages.Reachability` (spec 61 section 2.2) decides
  reachability over a `Finite` state carrier by a bounded fixed point,
  with soundness and completeness proved. It is keyed on a `Dfa`, not on
  a graph.
- **The layer slot.** Graphs are an `Algorithm` package. Nothing under
  `Algorithm/` imports `Capability.*`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (Architect boundary ruling, measured).** Rule from a checked
   development (rc=0, no Axiom) on:
   - the representation: vertices as any carrier with `Finite` evidence
     and edges as a successor function, or another form, and whether
     `DecEq` on vertices is required and where;
   - reachability: whether the automata decision is generalised to
     graphs and reused by `Reachability`, or the graph decision is new.
     Two copies of one fixed point is the outcome to avoid;
   - the dependency operation and its exact laws: a topological order
     in which every edge goes forward, or a witness that is a real
     cycle, with the order's completeness over all vertices;
   - what stays deferred, such as strongly connected components,
     shortest paths and weighted edges.
2. **Section 1, normative** for every ruled declaration and law, each
   stated as a proved theorem, with the trust boundary: no Axiom,
   primitive, kernel form or `trusted_base()` entry. Index the chapter in
   `spec/50-stdlib/README.md` and `spec/SPEC-PROGRESS.md`.
3. **A conformance seed** under `conformance/stdlib/` in the existing
   seed shape: an acyclic graph and its order, a cyclic graph and its
   witness, the empty graph, and a reachable and an unreachable pair.

## Acceptance

- **AC-1.** Every declaration and law the CAT node must deliver is named
  with its type, and nothing required is outside the Architect's ruling.
- **AC-2.** Conformance-validator vote on the exact SHA of the assembled
  scope: the spec paths plus the seed.

## Stop conditions

- The contract needs a kernel, trust or surface-syntax change.
- Reusing the automata decision would change spec 61 section 2's
  normative laws: the D0 rules the change, and it is in this scope only
  if the Architect places it here.
