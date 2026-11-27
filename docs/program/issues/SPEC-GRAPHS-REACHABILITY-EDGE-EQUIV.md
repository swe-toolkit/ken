---
id: SPEC-GRAPHS-REACHABILITY-EDGE-EQUIV
title: "Spec 62 says reachability grows with the graph under pointwise true-edge inclusion (graph_reachable_graph_monotone), and each union law proves its Boolean equality by applying that monotonicity both ways, but no law says that two graphs whose edge functions agree pointwise have equal reachability: Graph has no η, so a client holding two edge functions that agree at every pair cannot rewrite one search into the other without re-deriving both inclusions. Rule from a checked D0 at zero trust that pointwise-equal edges give equal graph_reachable, and state it with a seed whose accepted row is open"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Graphs follows Linear algebra in the L3 layer rotation (docs/program/06-catalog-campaign.md). Architect carry evt_7b0ghzv1kwss6: a general law that graphs with pointwise-equal edges have equal reachability. Steward-filed per COORDINATION section 2."
---

# Graphs whose edges agree pointwise reach the same vertices

## Objective

`Algorithm.Graphs.Dependency` exports a public checked law. For every `q`,
`fq : Finite q`, `g`, `h : Graph q`, `s : q` and `target : q → Bool`, from

`(u : q) → (v : q) → Equal Bool (edge q g u v) (edge q h u v)`

it states

`Equal Bool (graph_reachable q fq g s target) (graph_reachable q fq h s
target)`.

It holds at zero trust, and spec 62 states it by its public type.

## Settled inputs (main `3c2b7066d`; anchors re-checked at release)

- **The argument, for AC-0 to check.**
  - `graph_reachable_graph_monotone` (`Dependency.ken.md:230`) takes a
    pointwise true-edge inclusion. Each direction of the edge equation
    gives one inclusion, by rewriting `Equal Bool (edge q g u v) True`
    along the premise.
  - The public `bool_eq_from_iff` (`LawfulClasses.ken.md:496`) closes the
    two implications into the Boolean equality. This is the same shape as
    `graph_union_reachable_comm` (`:556`).
- **No such law exists.** Dependency has 39 public theorems. Its
  reachability equalities are about union, reversal and the edgeless
  graph. None takes an edge equation between two arbitrary graphs.
- **Graph equality is not available.** Spec 62 §15.2 and §16
  (`62-graphs.md:1511`) say that `Equal (Graph q)` cannot be inferred from
  pointwise edge equations, because `Graph` has no η. This law is the
  reachability consequence a client can use instead.
- **Spec and tests.**
  - Spec 62 ends with §23 (`62-graphs.md:1998`), so the new section is
    §24.
  - The seed directory is `conformance/stdlib/graphs/`.
  - The test pattern is
    `crates/ken-elaborator/tests/cat_graphs_union_reachable_comm.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 62 §24, the catalog law with any private helper, a seed, the package
test rows and the status lines. No §1 to §23 operation or law type moves,
and there is no kernel or surface change. Rewriting the existing union
laws through the new law is out of scope unless AC-0 shows it removes a
private helper with no other consumer.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-GRAPHS-REACHABILITY-EDGELESS.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules:
  - the name, the binder order and the placement;
  - whether the two inclusions are a private helper or inline;
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`, including any exact
    export roster and count-bearing sentence.
- **AC-1.** §24 states the law by its public type. CV and the Architect
  review it, with a Decision.
- **AC-2.** The seed's accepted block must not close by computation alone.
  - Its accepted row is open in `target`, over two fixed closed graphs
    whose edge functions agree at every pair but are not definitionally
    equal (for example, Boolean conjunction of the endpoints in either
    order), with the edge equation discharged by case analysis. AC-0 shows
    that the row's closing term needs the law.
  - Its refused twin, on a fresh accepted prefix and in the same form,
    claims equal reachability from a one-way true-edge inclusion only.
    That claim must refuse: the edgeless graph is included in a graph
    with one edge `V0 → V1`, and only the second reaches `V1` from `V0`.
  - AC-0 picks the closing terms and may strengthen the seed.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Each touched package test target passes on the exact
  candidate with no name filter. Full-workspace CI is green on the exact
  candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise beyond
  the edge equation.
- The proof needs a new public export outside Dependency; report it rather
  than widening Core.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §23 operation or law type.

Checks: 4 fired. I read `graph_reachable_graph_monotone`,
`graph_union_reachable_comm` and the public `bool_eq_from_iff` on
`3c2b7066d`; the law is two monotone applications closed by that Core law.
3 fired. I listed Dependency's 39 public theorems and found no
reachability equality over an arbitrary edge equation. 8 fired. A closed
seed row computes without the law (ADD-POINT-EXACT AC-0,
`evt_1b1n2fsh3bvws`), so the witness row is open, and the refused twin
weakens the equation to one inclusion, which the edgeless graph refutes.
