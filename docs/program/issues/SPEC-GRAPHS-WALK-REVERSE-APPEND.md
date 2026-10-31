---
id: SPEC-GRAPHS-WALK-REVERSE-APPEND
title: "Spec 62 composes walks (§3, §4) and reverses them (§8), but states no law relating the two, so a client cannot reverse a composed walk without unfolding both definitions. Rule from a checked D0 at zero trust that walk_reverse of walk_append is the walk_append of the reversed walks in the opposite order, and state it with a conformance seed"
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

# Reversing a composed walk

## Objective

Spec 62 states, as a public checked law over `Algorithm.Graphs.Dependency`,
that for every graph `g`, vertices `s`, `t` and `u`, and walks
`first : Walk q g s t` and `second : Walk q g t u`:

```ken
Equal
  (Walk q (graph_reverse q g) u s)
  (walk_reverse q g s u (walk_append q g s t u first second))
  (walk_append q (graph_reverse q g) u t s
     (walk_reverse q g t u second)
     (walk_reverse q g s t first))
```

It holds for every walk, with no premise.

## Settled inputs (main `42ba094f3`)

- **The gap.** Spec 62 §3 and §4 state `walk_append` and its left unit
  and associativity laws; §8 states `graph_reverse` and `walk_reverse`.
  No section relates reversal to composition.
- **The operations.** `walk_append` recurses on its second walk
  (`Dependency.ken.md:95`). `walk_reverse` turns each `WalkStep` into a
  private `walk_prepend` onto the reversed prefix (`:268`, helper at
  `:707`).
- **Proof route, to be confirmed by D0.** Induct on `second`.
  - The `WalkHere` case is the public `walk_append_here_left` (`:110`),
    used in the symmetric direction.
  - The step case needs `walk_append` to commute with `walk_prepend`.
    No such lemma exists today.
- **Seeds.** Under `conformance/stdlib/graphs/`, next to
  `seed-reachability-reverse.md`, whose V0 → V1 → V2 chain is
  reusable.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

One candidate:

- a §9 in spec 62 stating the law by its public type;
- the catalog theorem in `Dependency`;
- a closed conformance seed;
- the status lines.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules:
  - the theorem name, argument order and placement;
  - which helpers stay private, including the prepend/append lemma;
  - the seed's closed discriminator.

  It also confirms that no §1 to §8 operation or law type changes.
- **AC-1.** §9 states the law by its public type. The Spec Leader and
  CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the law to a closed
  two-part walk, such as V0 → V1 then V1 → V2. Its refused block pins a
  well-typed false `Walk` equality between two distinct walks with the
  same endpoints. In the same scope, an accepted same-form control
  separates false content from a proof-shape refusal.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or any premise.
- A change to an existing §1 to §8 operation or law type.

Checks: 4 fired: the obligation was written against `walk_append` and
`walk_reverse` at `Dependency.ken.md:95` and `:268` on `42ba094f3`, and
the endpoints of both sides type-check as `Walk q (graph_reverse q g) u
s`. 8 fired: composing the reversed walks in the original order is
ill-typed, so it cannot serve as the false row. AC-2 therefore requires
a same-endpoint false equality with an accepted same-form control.
