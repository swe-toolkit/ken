---
id: SPEC-GRAPHS-REACHABILITY-TRANS
title: "Spec 62 §5 carries reachability back along a walk, but states no law that reachability composes: if s reaches the vertex t and t reaches a target, then s reaches that target. A client holding two reachability decisions must extract a witness, transport its endpoint through DecEq and rebuild a walk, using private helpers it cannot import. Rule from a checked D0 at zero trust a public transitivity law, with a conformance seed"
status: active
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Graphs follows Linear algebra in the layer rotation. Steward-filed per COORDINATION section 2."
---

# Reachability is transitive

## Objective

`Algorithm.Graphs.Dependency` exports a public checked law that, for
every `q`, `fq : Finite q`, `d : DecEq q`, `g`, `s`, `t` and
`target`:

```ken
Equal Bool (graph_reachable q fq g s (λv. d.eq v t)) True
  → Equal Bool (graph_reachable q fq g t target) True
  → Equal Bool (graph_reachable q fq g s target) True
```

Spec 62 states it by its public type.

## Settled inputs (main `928313aa6`)

- **The neighbours exist.** In `Dependency.ken.md`:
  - `graph_reachable_along_walk` (`:187`) is §5; it consumes a walk,
    not a reachability decision, for the first leg;
  - `graph_find_walk_some` (`:172`) yields the witness for the first
    leg;
  - the private `require_some` (`:1040`) and `walk_transport_end`
    (`:1046`) already extract the witness and move its endpoint along
    `d.sound`;
  - the private `edge_reach_trans` (`:1083`) is the one-edge special
    case, and the private `graph_reachable_refl` (`:1109`) states
    `graph_reachable q fq g s (named q d s) = True` over the private
    `named` (`:1081`, `d.eq v x`).
- **DecEq.** `Core.Classes.LawfulClasses` `:81`: fields `eq`, `sound`,
  `complete`. Spec 62 §1.2 (`62-graphs.md:169`) already says a query
  for a particular vertex builds its predicate from a separate
  `DecEq q`.
- **Not yet stated.** `spec/`, `catalog/` and `conformance/` were
  grepped for a reachability transitivity law, with no hit beyond the
  private `edge_reach_trans`. The package exports 21 theorems (`:2630`).
  Spec 62 runs to §10 (file end `:1025`).
- **Seeds.** Under `conformance/stdlib/graphs/`, next to
  `seed-reachability-along-walk.md`, whose `V0 → V1 → V2` chain can be
  reused.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

The ruled law in `Dependency`, a spec 62 section stating it by its
public type, a closed conformance seed under
`conformance/stdlib/graphs/`, the public-inventory pin and the
package's law count advanced, and the status lines, all in one
candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set, or that measurement assigned to the candidate's package test. It
  rules:
  - the law's name, argument order, the form of the vertex predicate
    (an inline `λv. d.eq v t`, or exporting `named`), and placement;
  - the section number;
  - whether the reflexive base, `graph_reachable_refl`, is exported in
    the same candidate, so the two laws state that reachability is a
    preorder;
  - whether `edge_reach_trans` is restated through the new law or left
    as it is.

  It confirms that no §1 to §10 operation or law type changes.
- **AC-1.** The section states each ruled law by its public type. The
  Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the law on the
  `V0 → V1 → V2` chain from `V0` through `V1` to the target "is `V2`".
  - Its refused block pins `V2` reaching "is `V0`". That is false on
    the one-way chain, so composition cannot be run backwards.
  - Its accepted same-form control pins `V0` reaching "is `V2`", so
    the refusal is the value, not the form.
  - The D0 rules the proof form that separates the two rows.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or any premise
  beyond the two reachability facts.
- A change to an existing §1 to §10 operation or law type.

Checks: 4 fired: the law is written from public `graph_reachable`,
`graph_find_walk_some`, `graph_reachable_along_walk` and the `DecEq`
fields, with the private `require_some` and `walk_transport_end`,
each read at its line; spec, catalog and conformance were grepped for a
reachability transitivity law, with no public hit. 8 fired: the refused
row and its control share the chain, the operation and the claim form,
and differ only in the start and the target vertex.
