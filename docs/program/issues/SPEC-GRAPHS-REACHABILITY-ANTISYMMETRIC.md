---
id: SPEC-GRAPHS-REACHABILITY-ANTISYMMETRIC
title: "Spec 62 §11 makes reachability to a vertex a preorder, and §1.3 gives a dependency order whose walks run forward, but no public law joins them: a client holding a TopoOrder and reachability both ways between two vertices cannot conclude they are equal without the private walk extraction. Rule from a checked D0 at zero trust a public antisymmetry law under a dependency order, with a conformance seed"
status: active
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-LINEAR-ALGEBRA-MATRIX-PRODUCT-ZERO]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Graphs follows Linear algebra in the layer rotation. Steward-filed per COORDINATION section 2."
---

# Reachability is antisymmetric under an order

## Objective

`Algorithm.Graphs.Dependency` exports a public checked law that, for
every `q`, `fq : Finite q`, `d : DecEq q`, `g : Graph q`,
`order : TopoOrder q d g` and `s`, `t : q`:

```ken
Equal Bool (graph_reachable q fq g s (d.eq t)) True
  → Equal Bool (graph_reachable q fq g t (d.eq s)) True
  → Equal q s t
```

With §11, reachability to a vertex is then a partial order on a graph
that has a dependency order. Spec 62 states the law by its public type.

## Settled inputs (main `a892501a7`)

- **The legs.** `graph_find_walk_some` (`Dependency.ken.md:172`) and
  the private `require_some` (`:1070`) extract a walk from a true
  decision. The private `walk_transport_end` (`:1076`) moves its
  endpoint to the queried vertex through `d.sound`. §11's
  `graph_reachable_trans` (`:261`, proof at `:269` to `:283`) is the
  precedent route.
- **The order.** A `WalkHere` walk gives `Equal q s t` directly. A
  `WalkStep` walk gives `before q d s t (ordered_vertices …)` through
  the public `ordered_walk_forward` (`:608`), which takes the first edge
  and the rest of the walk; a `Walk` is built by its last step, so the
  first edge must be recovered. `before_transitive` (`:566`) and
  `before_irreflexive` (`:557`) then refute two forward legs.
- **Not yet stated.** Spec 62 runs §1 to §11 (`62-graphs.md`, §11 at
  `:1028`, file end `:1097`). No public law takes a `TopoOrder` and a
  reachability decision together.
- **Seeds.** Under `conformance/stdlib/graphs/`, next to
  `seed-reachability-trans.md`, whose one-way chain `V0 → V1 → V2` and
  `vertex3_eq` can be reused. The package test pattern is
  `crates/ken-elaborator/tests/cat_graphs_reachability_trans.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

The ruled law in Dependency, a spec 62 section stating it by its public
type, a closed conformance seed under `conformance/stdlib/graphs/`, a
named package test, and spec 62's status lines, all in one candidate.

## Acceptance

- **AC-0 (D0).** The Architect specifies a development on the landed
  base: the source, and the named-binary commands with a bounded
  timeout. The Foundation implementer runs it on scratch and reports
  rc, the `Axiom` grep and `trusted_base()` as an identity set before
  and after. The Architect rules from that report, and does not build
  or test. It rules:
  - the law's name, argument order and placement;
  - the section number;
  - how the first edge of a `WalkStep` walk reaches
    `ordered_walk_forward`: an existing private helper, a new private
    one, or a public first-edge law.

  It confirms that no §1 to §11 operation or law type changes.
- **AC-1.** The section states the ruled law by its public type. The
  Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client builds a `TopoOrder` for the
  one-way chain and uses the law with the forward leg to refute
  `V1` reaching `vertex3_eq V0`.
  - Its refused block makes the same claim on the two-cycle
    `V0 ⇄ V1`, where no `TopoOrder` exists, so the order's
    construction is what is refused.
  - Its accepted same-form control is the chain client itself, so the
    refusal is the back edge, not the form.
  - The D0 rules the proof form for the order's obligations; per §11,
    `Refl` does not discriminate an `Equal Bool` goal.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or a premise
  beyond the order and the two legs.
- A change to an existing §1 to §11 operation or law type.

Checks: 4 fired: the law is written from public `graph_reachable`,
`TopoOrder`, `ordered_walk_forward`, `before_transitive` and
`before_irreflexive`, each read at its line, and the private
`require_some` and `walk_transport_end` route of §11; the one gap,
recovering a walk's first edge for `ordered_walk_forward`, is named
after reading `Walk`'s last-step constructor. 3 fired: Dependency's
public names and spec 62 §1 to §11 were read for a law taking a
`TopoOrder` with a reachability decision; none exists. 8 fired: the
refused row and its control share the claim and the client, and differ
only in the back edge that makes the order unconstructible.
