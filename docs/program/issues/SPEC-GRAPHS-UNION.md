---
id: SPEC-GRAPHS-UNION
title: "Spec 62 has no way to combine two graphs on one vertex carrier, so a client merging two dependency relations must write the Boolean disjunction by hand and re-prove what its edges mean. Rule from a checked D0 at zero trust a public graph_union with left and right edge inclusion, an edge case split, and reachability carried into the union, and state them with a conformance seed"
status: active
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Graphs follows Linear algebra in the L3 layer rotation (docs/program/06-catalog-campaign.md). Steward-filed per COORDINATION section 2."
---

# The union of two graphs

## Objective

`Algorithm.Graphs.Dependency` exports `graph_union` on two `Graph q`
values, whose edge from `s` to `t` is the Boolean disjunction of the two
edges. Its laws hold at zero trust, and spec 62 states them by their
public types.

## Settled inputs (main `a24a82d4c`)

- **The gap.** A grep for `graph_union` over `catalog/`, `spec/`,
  `conformance/` and `crates/` finds 0 hits. Spec 62 has §1 to §14.
- **The carrier.** `Graph q` is `MkGraph (q → q → Bool)` and `edge`
  reads it back (`catalog/packages/Algorithm/Graphs/Dependency.ken.md:64`,
  `:70`). So `graph_union q g h` is
  `MkGraph q (λs. λt. bool_or (edge q g s t) (edge q h s t))`.
- **The Bool laws already public.** `bool_or`
  (`catalog/packages/Core/Classes/LawfulClasses.ken.md:107`), `or_left`
  (`:782`), `or_right` (`:788`) and `or_cases` (`:794`, eliminating
  into an `Omega` goal only). `Or`, `Inl` and `Inr` are in
  `Core.Logic.Or`, already imported by Dependency (`:51`). `Or` is
  Type-sorted, so the case split cannot be `or_cases`; AC-0 ruled a
  private Bool match (`evt_7gy680pa9gfz8`).
- **The reachability route.** `graph_reachable_graph_monotone`
  (`Dependency.ken.md:230`) carries reachability along any edge
  inclusion. So the reachability laws reduce to the two inclusion laws.
- **Seeds and tests.** Seeds sit under `conformance/stdlib/graphs/`. The
  test pattern is
  `crates/ken-elaborator/tests/cat_graphs_successor_list_adaptor.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

A new spec 62 section, the catalog operation and laws, a seed and the
status lines. No §1 to §14 operation or law type moves, and no kernel or
surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-LINEAR-ALGEBRA-MATRIX-SCALE.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules the law set and names, starting from:
  - left inclusion: an edge of `g` is an edge of `graph_union q g h`;
  - right inclusion: the same for `h`;
  - case split: an edge of the union is an edge of `g` or an edge of
    `h`, as `Or` of the two `Equal Bool` facts;
  - reachability: `graph_reachable` in `g`, or in `h`, implies it in the
    union.
  It also rules which helpers stay private and whether any existing
  private helper must become public, naming the owner of that change.
- **AC-1.** The new section states `graph_union` and each ruled law by
  its public type. The Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted block builds two closed graphs on one
  three-vertex carrier, `g` with only the edge `0 → 1` and `h` with
  only `1 → 2`, applies each law to a closed instance, and shows the
  union reaches `2` from `0` while neither `g` nor `h` alone does. Its
  refused block pins, in the same form as an accepted row, the claim
  that the union has no edge `1 → 2`, and the claim that it has an
  edge `2 → 0`. Each refusal sits on a fresh accepted prefix.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A law needs an `Axiom`, a kernel or surface change, or a premise
  beyond the `Finite q` the reachability operations already take.
- A change to an existing §1 to §14 operation or law type.

Checks: 4 fired: `Graph`, `edge`, `bool_or`, `or_left`, `or_right`,
`or_cases`, the `Or` import and `graph_reachable_graph_monotone` were
read at `a24a82d4c` on the cited lines; each inclusion law's obligation
is `or_left` or `or_right` after `edge` of `MkGraph` reduces, and the
case split was framed as `or_cases` into `Or`, which is ill-typed;
AC-0 corrected it. 3 fired: the `graph_union` grep
covered `catalog/`, `spec/`, `conformance/` and `crates/`, with 0 hits.
8 fired: a union that ignores `h` satisfies left inclusion, the case
split and left reachability; a union built with `bool_and` satisfies
the case split; a union that is always `True` satisfies both inclusions
and both reachability laws. So no single law discriminates, the case
split and the two inclusions do so together, and the refused twins are
the ignore-`h` mutant (no `1 → 2`) and the always-`True` mutant
(`2 → 0`). The two-step reach from `0` to `2` needs both graphs, so it
separates the union from either input.
