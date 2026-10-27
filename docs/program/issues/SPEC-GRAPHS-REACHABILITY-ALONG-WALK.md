---
id: SPEC-GRAPHS-REACHABILITY-ALONG-WALK
title: "Spec 62 decides reachability from one start and composes walks, but no law carries a reachability answer back along a walk: if t reaches a target and a walk runs from s to t, then s reaches it. Rule from a checked D0 at zero trust that it holds, and state it with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Graphs is next in the layer rotation after Linear algebra. Same shape as SPEC-LINEAR-ALGEBRA-MATRIX-VECTOR-LINEARITY. Steward-filed per COORDINATION section 2."
---

# Reachability carries back along a walk

## Objective

Spec 62 states, as a public checked law over
`Algorithm.Graphs.Dependency`, that a walk from `s` to `t` and
`graph_reachable q fq g t target = True` give
`graph_reachable q fq g s target = True`.

## Settled inputs (spec 62 §1 to §4, main `aa6370449`)

- **The pieces exist.** §1.2 states `graph_reachable_complete` (a walk
  to a target vertex gives `True`) and `graph_find_walk_some` (`True`
  gives `Some` witness). `ReachWitness` is `MkReach t proof walk`
  (`Dependency.ken.md:89-93`). §3 defines `walk_append`.
- **No law composes them.** No section relates the reachability answers
  of two start vertices.
- **Why it holds.** From `True` at `t`, `graph_find_walk_some` yields a
  witness walk from `t` to some `t'` with `target t' = True`.
  Appending it to the walk from `s` gives a walk from `s` to `t'`, so
  `graph_reachable_complete` gives `True` at `s`. The converse is
  false. On the one-edge graph `a → b` with target "is `a`", `a`
  reaches it by `WalkHere` and `b` does not, although a walk runs from
  `a` to `b`.
- **Seeds.** Under `conformance/stdlib/graphs/`, next to
  `seed-walk-append-unit-assoc.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §5 in spec 62 stating the ruled law by its public type, and a closed
conformance seed under `conformance/stdlib/graphs/`, with the status
lines changed in the same candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged. It rules the
  name, the public type (the walk as an argument or as a premise), and
  which helpers stay private.
- **AC-1.** The new section states the ruled law by its public type.
  The Spec leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the law on a closed
  three-vertex chain whose start is not itself a target. Its refused
  block is pinned to one false proposition on the same graph, such as
  reachability from the chain's end to its start.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or a premise
  beyond the walk.
- A change to an existing §1 to §4 law or operation.
