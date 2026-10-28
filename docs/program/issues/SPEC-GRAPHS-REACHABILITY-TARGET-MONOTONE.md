---
id: SPEC-GRAPHS-REACHABILITY-TARGET-MONOTONE
title: "Spec 62 decides reachability of a caller's target predicate, but no law says the decision is monotone in that predicate. Rule from a checked D0 at zero trust that reachability of a target carries to any pointwise-larger target, and state it with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Graphs is next in the layer rotation after Linear algebra. Same shape as SPEC-GRAPHS-REACHABILITY-ALONG-WALK. Steward-filed per COORDINATION section 2."
---

# Reachability is monotone in its target

## Objective

Spec 62 states, as a public checked law over `Algorithm.Graphs.Dependency`,
that if every vertex satisfying `target` also satisfies `wider`, then
`graph_reachable q fq g s target = True` implies
`graph_reachable q fq g s wider = True`, for every graph and start.

## Settled inputs (spec 62 §1 to §5, main `6d256cb0d`)

- **The decision.** `graph_reachable` decides whether some walk from `s`
  ends at a vertex where the caller's `target` is `True` (§1.2;
  `Dependency.ken.md:151`). It takes the predicate as an argument, with no
  `DecEq q`.
- **No law varies the target.** §1.2's `graph_reachable_complete` and
  `graph_find_walk_some` fix one `target`. §5's
  `graph_reachable_along_walk` varies the start and keeps the target. No
  section relates two targets.
- **Why it should hold.** `graph_find_walk_some` with the private
  `require_some` gives a `ReachWitness` for `target`: an endpoint `t`, a
  hit and a walk (`:172`). The inclusion turns the hit into one for
  `wider`, and `graph_reachable_complete` (`:161`) consumes the same walk.
  This is §5's proof shape. Whether it needs a new private helper is the
  D0's to measure.
- **The premise is necessary.** Without the inclusion the claim is false.
  On the one-way edge `V0 → V1` from start `V1`, the target "is `V1`" is
  reachable and the target "is `V0`" is not.
- **Seeds.** Under `conformance/stdlib/graphs/`, next to
  `seed-reachability-along-walk.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §6 in spec 62 stating the ruled law by its public type, and a closed
conformance seed under `conformance/stdlib/graphs/`, with the status lines
changed in the same candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged. It rules the
  name, the public type and argument order (including how the inclusion
  premise is stated), and which helpers stay private.
- **AC-1.** The new section states the ruled law by its public type.
  The Spec leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the law on a closed graph
  of at least three vertices, with two distinct targets where `wider`
  strictly contains `target`. Both reachability results are also computed
  independently by `Proved`. Its refused block is pinned to one false
  proposition on the same graph with the right type: the converse,
  reachability of a smaller target from reachability of a larger one.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or a premise
  beyond the target inclusion.
- A change to an existing §1 to §5 law or operation.

Checks: 4 fired: the law is written against the catalog's own signatures
(`graph_reachable :151`, `graph_reachable_complete :161`,
`graph_find_walk_some :172` in `Dependency.ken.md`), and the proof route
is the one §5's spec text records as checked. 6 fired: "no law varies the
target" was measured by reading every `pub theorem` in `Dependency.ken.md`
(14) and spec 62 §1 to §5.
