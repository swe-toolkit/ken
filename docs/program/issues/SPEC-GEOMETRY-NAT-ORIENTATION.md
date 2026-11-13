---
id: SPEC-GEOMETRY-NAT-ORIENTATION
title: "Spec 67 defers orientation, so a client cannot ask whether three points turn left, turn right or are collinear, and convex hulls and segment tests have no checked primitive to build on. Over natural points the sign of the cross product is an exact comparison of two Nat sums and needs no signed carrier. Rule from a checked D0 at zero trust an orient operation on three Points with its swap, rotation and repeated-point laws, and state them with a conformance seed"
status: ready
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Geometry follows Optimization in the L3 layer rotation (docs/program/06-catalog-campaign.md, geometry exact-before-float). The section 1 to 17 box algebra is complete; orientation is the first deferred item (spec 67 section 1.4). Steward-filed per COORDINATION section 2."
---

# Orientation of three natural points

## Objective

`Algorithm.Geometry` exports a checked `orient` on three `Point`s that
returns `OrdResult`: `Gt` for a counter-clockwise turn, `Lt` for a
clockwise turn, `Eq` for collinear points. Three public laws hold at
zero trust, and spec 67 states them by their public types.

## Settled inputs (main `5a3cc337b`)

- **The deferral.** `spec/50-stdlib/67-geometry.md:205` defers "signed
  coordinates and signed-dependent orientation, cross products and
  rotation". A grep for `orient` over `catalog/packages/Algorithm/Geometry`,
  spec 67 and `conformance/stdlib/geometry` finds only deferral prose.
- **No signed carrier is needed.** For `a`, `b`, `c` the cross product
  `(bx-ax)(cy-ay) - (by-ay)(cx-ax)` equals `L - R`, where
  `L = ax*by + bx*cy + cx*ay` and `R = ay*bx + by*cx + cy*ax`. The
  `ax*ay` terms cancel. So `orient a b c = compare L R` over Nat. For
  `(0,0), (1,0), (0,1)`, `L = 1` and `R = 0`, which gives `Gt`.
- **Why the laws hold.** Swapping `a` and `b` exchanges `L` and `R`.
  Rotating `a b c` to `b c a` permutes the three summands of each side.
  With `a = b`, `L` and `R` agree term by term up to `mul`
  commutativity.
- **What exists.** `Point` is `MkPoint Nat Nat`
  (`catalog/packages/Algorithm/Geometry/NatBox.ken.md:62`). `compare`
  is `catalog/packages/Data/Numeric/Nat/Order.ken.md:393`, built on
  `leq_nat`, and has no exported swap or reflexivity law. `OrdResult` has
  no exported flip (`catalog/packages/Core/Logic/OrdResult.ken.md`).
  Nat `add` and `mul` commutativity and associativity are reached as
  named-proof claims, as in `(proof comm for add)`
  (`catalog/packages/Algorithm/LinearAlgebra/NatMatrix.ken.md:797`), and
  `mul_add_distrib_l`/`_r` are in `Data.Numeric.Nat.Arithmetic`.
- **Seeds and tests.** Seeds sit under `conformance/stdlib/geometry/`.
  The test pattern is
  `crates/ken-elaborator/tests/cat_geometry_box_inside_intersect_hull.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

A new spec 67 section, the catalog operation and laws, a seed and the
status lines. No §1 to §17 operation or law type moves, and no kernel or
surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-OPTIMIZATION-LEAST-K.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules:
  - the module home (in `NatBox` beside `Point`, or a new module that
    imports it);
  - the three law names and statements: swap (`orient b a c` is the flip
    of `orient a b c`), rotation (`orient a b c = orient b c a`), and
    repeated point (`orient a a c = Eq`);
  - whether the `compare` swap and reflexivity facts and an `OrdResult`
    flip are local helpers or new exports, and the owner of any privacy
    change.
- **AC-1.** The new section states `orient` and the three laws by their
  public types. The Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted block evaluates `orient` on closed
  counter-clockwise, clockwise and collinear triples to `Gt`, `Lt` and
  `Eq`, and applies each law to a closed triple. Its refused block pins,
  in the same form as an accepted row, the claim that the
  counter-clockwise triple orients `Lt`, and a rotation claim with the
  arguments out of cyclic order. The collinear triple has no zero
  coordinate, for example `(1,1), (2,2), (3,3)`, where `L = R = 11` and
  every summand is positive, so dropping any one summand of `L` or `R`
  leaves `Eq` false. The counter-clockwise row catches an
  implementation that compares `R` with `L`. Each refusal sits on a
  fresh accepted prefix.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A law needs an `Axiom`, Int, a kernel or surface change, or a premise.
- A change to an existing §1 to §17 operation or law type.

Checks: 4 fired: the `L`/`R` expansion was checked by hand, and
`(0,0), (1,0), (0,1)` gives `L = 1`, `R = 0`. `compare`, `OrdResult` and
the Nat claim route were read on `5a3cc337b` at the cited lines. 3 fired:
the `orient` grep covered the geometry package, spec 67 and the geometry
seeds. 8 fired: a turning triple cannot see every summand drop (on
`(1,1), (3,2), (2,4)`, `L = 16`, `R = 11`, and dropping a `2` from `L`
still gives `Gt`), so the summand control is the zero-free collinear
triple, `L = 2 + 6 + 3 = R = 2 + 6 + 3`, where any drop breaks `Eq`.
