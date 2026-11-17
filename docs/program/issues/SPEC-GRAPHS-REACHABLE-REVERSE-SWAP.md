---
id: SPEC-GRAPHS-REACHABLE-REVERSE-SWAP
title: "Spec 62 §8 lets a client turn a supplied walk into reachability in the reversed graph, but no law states that the reachability decision itself is preserved, so a client holding only a Bool result cannot cross to the reversed graph. Rule from a checked D0 at zero trust that, for a decidable carrier, graph_reachable g s (d.eq t) equals graph_reachable (graph_reverse g) t (d.eq s), and state it with a conformance seed"
status: ready
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Graphs follows Linear algebra in the L3 layer rotation (docs/program/06-catalog-campaign.md). graph_reachable_reverse needs a walk and gives one direction only. Steward-filed per COORDINATION section 2."
---

# Reachability is preserved by reversing the graph and swapping the endpoints

## Objective

`Algorithm.Graphs.Dependency` exports a public checked law that, for every
`q`, `fq : Finite q`, `d : DecEq q`, `g : Graph q` and `s t : q`,
`graph_reachable q fq g s (d.eq t)` equals
`graph_reachable q fq (graph_reverse q g) t (d.eq s)` as `Equal Bool`. It
holds at zero trust with no premise, and spec 62 states it by its public
type.

## Settled inputs (main `943d14a06`)

- **The one-way law.** `graph_reachable_reverse`
  (`Dependency.ken.md:303`) takes a `Walk q g s t` and a target true at
  `s`, and concludes reachability from `t` in `graph_reverse q g`. It
  needs the walk, so it cannot start from a `Bool` result.
- **The tools, all public.** `graph_find_walk_some` (`:172`) with
  `require_some` and `MkReach u hit walk` turns a true reachability result
  into a walk, as `graph_reachable_trans` (`:261`) does. Also public are
  `walk_reverse` (`:294`), `graph_reachable_complete` (`:161`) and
  `graph_reverse_involutive` (`:408`). `DecEq` gives `d.sound` and
  `d.complete`. Private `bool_eq_from_iff` (`:1409`) closes a `Bool`
  equality from two implications, as `graph_union_reachable_comm` (`:528`)
  does.
- **The gap.** `graph_reachable` with `graph_reverse` appears in
  `catalog/packages`, `spec/` and `conformance/` only through
  `graph_reachable_reverse` and its seed
  (`conformance/stdlib/graphs/seed-reachability-reverse.md`). No `Bool`
  equality relates the two decisions.
- **Spec and tests.** Spec 62 ends at §17
  (`spec/50-stdlib/62-graphs.md:1533`), so the new section is §18. The
  test pattern is
  `crates/ken-elaborator/tests/cat_graphs_reverse_reachability.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 62 §18, the catalog law with any private helper, a seed, a package
test and the status lines. No §1 to §17 operation or law type moves, and
no kernel or surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-GRAPHS-UNION-REACHABLE-ASSOC.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules the name and the binder order. The
  expected route is one direction from a witness walk and `walk_reverse`,
  and the other from the same direction at `graph_reverse q g`,
  transported by `graph_reverse_involutive`.
- **AC-1.** §18 states the ruled law by its public type. The Spec Leader
  and CV review it, with a Decision.
- **AC-2.** The seed's accepted block applies the law on the closed
  `Vertex3` chain `V0 → V1 → V2` of the §8 seed, with a seed-local
  `DecEq Vertex3` dictionary:
  - `s = V0`, `t = V2`, where both sides are `True`;
  - `s = V2`, `t = V0`, where both sides are `False`.

  Its refused block pins, in the same form as an accepted row and on a
  fresh accepted prefix:
  - `graph_reachable chain V0 (eq V2)` claimed equal to
    `graph_reachable chain V2 (eq V0)`, the same swap with no reversal.
    They are `True` and `False`.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise
  beyond `DecEq q` and `Finite q`.
- A change to an existing §1 to §17 operation or law type.

Checks: 4 fired. I wrote the obligation in the package's own vocabulary.
`graph_reachable_trans` already extracts a walk from a true result keyed on
`d.eq t`, and `d.complete s s Refl` gives the reversed side's target at
`s`. 3 fired: I grepped `catalog/packages`, `spec/` and `conformance/` for
`graph_reachable` with `graph_reverse`. The only hits are the one-way law
and its seed. 8 fired: the refused row drops only `graph_reverse`. On the
chain it is `True` against `False`, so it fails exactly when the reversal
is ignored, while both accepted rows hold.
