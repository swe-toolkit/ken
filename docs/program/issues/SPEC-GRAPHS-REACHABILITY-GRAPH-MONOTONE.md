---
id: SPEC-GRAPHS-REACHABILITY-GRAPH-MONOTONE
title: "Spec 62 proves reachability monotone in the target predicate, but no law says it is monotone in the graph: adding edges cannot lose a reachable target. Rule from a checked D0 at zero trust the public law and whether the walk lifting it needs is a public operation or a private helper, and state the law with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Graphs is next in the layer rotation after Linear algebra. Same shape as SPEC-GRAPHS-REACHABILITY-TARGET-MONOTONE. Steward-filed per COORDINATION section 2."
---

# Reachability is monotone in the graph

## Objective

Spec 62 states, as a public checked law over `Algorithm.Graphs.Dependency`,
that when every edge of `g` is an edge of `h`, affirmative reachability of
`target` from `s` in `g` gives affirmative reachability in `h`, at the same
`fq`, `s` and `target`.

## Settled inputs (spec 62 §1 to §6, main `96a6a8312`)

- **The operations.** `edge` (`Dependency.ken.md:70`) reads a `Graph q`'s
  Boolean step. A `Walk q g s t` step carries
  `Equal Bool (edge q g u v) True` (`:75`). `graph_reachable` (`:151`) is the
  reachability decision over a `Finite q`.
- **The pattern exists.** `graph_reachable_target_monotone` (`:209`) takes a
  `ReachWitness` from `graph_find_walk_some` and private `require_some`, then
  closes with `graph_reachable_complete` (`:161`) along the same walk. The new
  law needs one more step: the witness walk in `g` re-typed as a walk in `h`,
  edge by edge, through the inclusion premise.
- **No such law.** §1 to §6 vary the target (§6) and the start (§5), never the
  graph. §1.4's exclusions (components, shortest paths, weights, adaptors,
  extremal orders) do not cover it.
- **The premise, in the delivered vocabulary.** For all `u` and `v`,
  `Equal Bool (edge q g u v) True → Equal Bool (edge q h u v) True`.
- **Seeds.** Under `conformance/stdlib/graphs/`, next to
  `seed-reachability-target-monotone.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §7 in spec 62 stating the ruled law by its public type, a closed conformance
seed under `conformance/stdlib/graphs/`, and the status lines, all in one
candidate. The catalog half travels in the candidate when AC-0 rules it.
Otherwise it is a separate Foundation WP built from the D0.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base, with
  rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity set. It
  rules:
  - the law's name and input order;
  - whether the walk lifting is a public operation or a private helper;
  - the proof form of each seed row (`Refl` or `Proved`).

  It also confirms that no §1 to §6 operation or law type changes.
- **AC-1.** §7 states the law by its public type. The Spec Leader and CV
  review it, with a Decision.
- **AC-2.** The seed's accepted client uses a closed `h` that is `g` plus one
  edge, with a target that both graphs reach from the start. It applies the
  law across a real edge, and checks both reachability decisions
  independently. Its refused block pins the converse on the same graphs: a
  target that `h` reaches and `g` does not, claimed reachable in `g`.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or a premise beyond
  edge inclusion.
- A change to an existing §1 to §6 operation or law type.

Checks: 4 fired: the inclusion premise was written in the delivered `edge` and
`Equal Bool` vocabulary, and the walk constructor at `Dependency.ken.md:75`
carries exactly that edge proof per step, so a lift by induction on the walk
can supply it. 8 fired: the refused row is the converse, which fails exactly
when `h` has an edge `g` lacks, so a law misstated with the graphs swapped
would accept it.
