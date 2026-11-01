---
id: SPEC-GRAPHS-REVERSE-INVOLUTION
title: "Spec 62 §8 reverses a directed graph and its walks, and §9 reverses a composed walk, but states no law that reversal is an involution: reversing a graph twice gives back the graph, and reversing a walk twice gives back the walk. A client that reverses twice cannot return to its original graph or walk without reopening the representation. Rule from a checked D0 at zero trust and state the laws with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Graphs follows Linear algebra in the layer rotation. Steward-filed per COORDINATION section 2."
---

# Reversal is an involution

## Objective

Spec 62 states, as public checked laws over `Algorithm.Graphs.Dependency`,
for all `q` and `g`:

- `Equal (Graph q) (graph_reverse q (graph_reverse q g)) g`;
- reversing a walk twice gives back the walk, in the type the D0 rules.

## Settled inputs (main `a39d385c8`)

- **The public neighbours exist.** `Graph` is the one-constructor data
  `MkGraph : (q → q → Bool) → Graph q`, and `edge` matches on it
  (`Dependency.ken.md:64`, `:70`). `graph_reverse` (`:261`) is
  `MkGraph q (λs. λt. edge q g t s)`, `graph_reverse_edge` (`:263`)
  closes by `Refl`, and `walk_reverse` (`:268`) and
  `walk_reverse_append` (`:289`) are public.
- **Not yet stated.** No public theorem concludes either equation, and
  spec 62 §8 and §9 (`62-graphs.md:755` to `:911`) state none.
- **The type obstacle.** Twice-reversed, a walk lives in
  `Walk q (graph_reverse q (graph_reverse q g)) s t`. That graph equals
  `g` only by the first law, unless the kernel identifies them
  definitionally. Whether it does, for an open `g`, is the D0's first
  measurement.
- **Seeds.** Under `conformance/stdlib/graphs/`, next to
  `seed-reachability-reverse.md`, whose `Vertex3`, `chain` and `is_v0`
  can be reused. §9.2 records that `Proved` is refused for a true
  closed `Walk` equality, so the proof form for walk rows is not
  presumed.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §10 in spec 62 stating the ruled laws by their public types, the
catalog theorems in `Dependency`, a closed conformance seed under
`conformance/stdlib/graphs/`, and the status lines, all in one
candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It measures whether the twice-reversed graph is definitionally
  `g`. It then rules the walk law's statement: in `g` directly, at a
  matched `MkGraph q step`, or through a transport along the graph law.
  If no zero-trust form exists, the WP ships the graph law alone and
  says so in §10. It also rules names, argument order and placement,
  which helpers stay private, and confirms that no §1 to §9 operation
  or law type changes.
- **AC-1.** §10 states each ruled law by its public type. The Spec
  Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the laws to `chain`
  and to a two-edge walk on it.
  - Its refused block pins `graph_reverse Vertex3 chain` as equal to
    `chain`. The chain has an edge V0 to V1 and none from V1 to V0, so
    the claim is false.
  - Its accepted same-form control is the twice-reversed chain equal to
    `chain`, so the refusal is the single reversal, not the form.
  - The D0 rules the proof form that separates the two rows.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or any premise.
- A change to an existing §1 to §9 operation or law type.

Checks: 4 fired: `Graph`, `edge`, `graph_reverse`, `walk_reverse` and
`walk_reverse_append` were read at their lines on main; the package and
spec 62 were grepped for an involution law, with no hit. 8 fired: the
refused row and its control share `chain` and the claim form and differ
only in the number of reversals, one making the claim false and two
making it true.
