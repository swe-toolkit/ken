---
id: SPEC-NUMERICAL-INTERVAL-HULL-INTERSECT-IDEMPOTENT
title: "Spec 68 and the RationalInterval prose say structural idempotence of hull and intersection is not claimed, but rational_min x x and rational_max x x return x as stored, so interval_hull i i and interval_intersect i i equal i as stored endpoints. Rule from a checked D0 at zero trust the two structural idempotence laws, with the Rational min and max self laws they need, and state them with a seed that separates same-operand idempotence from equal-value operands"
status: merged
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Numerical follows Graphs in the L3 layer rotation (docs/program/06-catalog-campaign.md). The hull and intersection laws stop at inclusion, with no idempotence law. Steward-filed per COORDINATION section 2."
---

# Hull and intersection are idempotent as stored

## Objective

`Algorithm.Numeric.RationalInterval` exports two public checked laws:
`Equal Interval (interval_hull i i) i` and
`Equal Interval (interval_intersect i i) i` for every `i`. They hold at zero
trust with no premise, including for inverted `i` and for non-normal
endpoint spellings. Spec 68 states them by their public type.

## Settled inputs (main `dfbe3c81c`)

- **The operations.** `interval_hull` and `interval_intersect`
  (`RationalInterval.ken.md:569`, `:577`) rebuild `MkInterval` from
  `rational_min` and `rational_max` of the endpoints.
- **Why it holds as stored.** `rational_min x y` is
  `rational_pick (rational_leq x y) x y` and `rational_max` is
  `rational_pick (rational_leq x y) y x` (`Rational.ken.md:721`, `:724`).
  `rational_pick` (`:761`) is private and matches on its `Bool`. At
  `x = y`, both arms return the same `x`, whatever `rational_leq x x`
  computes to.
- **What a client cannot do today.** `rational_pick` is private, so a
  client cannot state the motive needed to case on `rational_leq x x`.
  Rational's public min and max proofs (`:727`-`:754`) are order bounds,
  not self laws. So the interval laws are expected to need two public
  Rational laws, for `rational_min x x` and `rational_max x x`.
- **The under-claim.** `RationalInterval.ken.md:1789` and
  `spec/50-stdlib/68-numerical.md:349` say structural idempotence is not
  claimed. They give the reason that the operations do not normalize
  spellings. That reason holds for commutativity and associativity, but not
  for an operand combined with itself.
- **Spec and tests.** Spec 68 ends at §18 (`68-numerical.md:1703`), so
  the new section is §19. The seed pattern is
  `conformance/stdlib/numerical/seed-interval-associativity-inside.md`,
  where `MkRational n k` is `n / (k + 1)`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 68 §19, the two interval laws, the Rational self laws they need
(`rational_min` is specified only in spec 68, so §19 states them too), the
two under-claim sentences narrowed to commutativity and associativity, a
seed, package tests and the status lines. No §1 to §18 operation or law
type moves, and no kernel or surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-NUMERICAL-INTERVAL-WIDTH-BOUNDS-DISTANCE.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules the names, the forms (a `proof ... for` on
  `rational_min` and `rational_max`, or a theorem) and the placement of the
  Rational self laws. It also rules whether any route avoids them.
- **AC-1.** §19 states both interval laws by their public type, and the
  two under-claim sentences say exactly what stays unclaimed. The Spec
  Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted block applies both laws at closed
  intervals:
  - `[1/2, 2]`, written `MkInterval (MkRational 1 1) (MkRational 2 0)`;
  - `[2/4, 2]`, a non-normal lower spelling, written
    `MkInterval (MkRational 2 3) (MkRational 2 0)`;
  - the inverted `[2, 1/2]`.

  Its refused block pins, in the same form as an accepted row and on a
  fresh accepted prefix:
  - `interval_hull [1/2, 2] [2/4, 2]` claimed `Equal Interval` to
    `[2/4, 2]`. The operands are equal in value, but the hull keeps the
    `1/2` spelling, so the claim is false.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A law needs an `Axiom`, a kernel or surface change, or a premise.
- A change to an existing operation or law type in spec 68 §1 to §18 or in
  Rational.

Checks: 4 fired. I wrote the obligation in the delivered vocabulary:
`rational_pick b x x` reduces to `x` in both arms of its match, and the
interval laws then follow by `cong` on `MkInterval`. The one gap is that
`rational_pick` is private, which is why the frame expects the Rational
self laws. 3 fired: I grepped `RationalInterval.ken.md`, `Rational.ken.md`,
spec 68 and `conformance/stdlib/numerical` for idempotence and for min/max
self laws. There are none, and the two hits are the under-claim sentences.
8 fired: the refused row's operands are equal in value and differ only in
spelling, so it fails exactly when "idempotent" is read as value-level
equality, while the same-operand rows hold.
