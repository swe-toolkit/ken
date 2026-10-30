---
id: SPEC-GRAPHS-REVERSE-REACHABILITY
title: "Spec 62 decides what a vertex reaches, but gives no way to ask what reaches it: Algorithm.Graphs.Dependency has no reversed graph, so a reverse-dependency query has no checked route. Rule from a checked D0 at zero trust a public graph_reverse whose edges are the original's flipped, a walk reversal into it, and the law that a walk in g makes its start reachable from its end in the reversed graph, and state them with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Graphs is next in the layer rotation after Linear algebra. Steward-filed per COORDINATION section 2."
---

# Reachability in the reversed graph

## Objective

Spec 62 states, as public checked operations and laws over
`Algorithm.Graphs.Dependency`:

- `graph_reverse q g : Graph q`, whose edge `s → t` is `g`'s edge
  `t → s`;
- a walk reversal that turns a `Walk q g s t` into a
  `Walk q (graph_reverse q g) t s`;
- for every `fq`, `g`, walk from `s` to `t`, and `target` with
  `target s = True`, that `graph_reachable q fq (graph_reverse q g) t target`
  is `True`.

## Settled inputs (spec 62 §1 to §7, main `c0be2d445`)

- **The carrier.** `Graph q` is `MkGraph` over a Boolean adjacency, and
  `edge` matches it (`Dependency.ken.md:64`, `:70`). A reversed graph is
  `MkGraph` over the flipped adjacency; no second carrier is needed.
- **Walks grow at the end.** `WalkStep` extends a walk at its destination,
  and `walk_append` recurses on its second walk (`Dependency.ken.md:95`).
  A reversed walk therefore prepends each step, which `walk_append` in the
  reversed graph supplies.
- **Not yet stated.** Spec 62 and the package have no reverse, transpose
  or converse graph. The only "reverse" hits in the package are a private
  order lemma (`strict_excludes_reverse`) and prose about §7's false
  converse.
- **The reachability half exists.** §1.2's `graph_reachable_complete`
  turns any walk with a target hit into affirmative reachability, so the
  law follows from the walk reversal.
- **Seeds.** Under `conformance/stdlib/graphs/`, next to
  `seed-reachability-graph-monotone.md`, whose three-vertex `chain`
  (V0 → V1 → V2) can be reused.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §8 in spec 62 stating the ruled operations and law by their public types,
the catalog exports in `Dependency`, a closed conformance seed under
`conformance/stdlib/graphs/`, and the status lines, all in one candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base, with
  rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity set. It
  rules:
  - the names, argument order and placement;
  - whether the edge flip is stated as a public law, and in what form;
  - which helpers stay private.

  It also confirms that no §1 to §7 operation or law type changes.
- **AC-1.** §8 states each ruled operation and law by its public type. The
  Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client uses `chain`.
  - It reverses the two-edge walk V0 → V1 → V2 and applies the law, so that
    in `graph_reverse chain`, V2 reaches target V0.
  - Its refused block pins the same claim on `chain` itself: from V2,
    target V0 is reachable. `chain` has no edge out of V2, so a kernel
    that accepted it would show the reversal was not checked.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or any premise beyond
  the walk and the target hit.
- A change to an existing §1 to §7 operation or law type.

Checks: 4 fired: the reversal was written against `MkGraph`, `edge` and
`walk_append` at `Dependency.ken.md:64`, `:70` and `:95` on `c0be2d445`,
and spec 62 and the package were grepped for reverse, transpose and
converse; none is a graph operation. 8 fired: the seed's refused row is the
accepted claim with `graph_reverse` dropped. In `chain` no edge leaves V2,
so its right side is false while the accepted side is true.
