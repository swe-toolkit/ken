---
id: SPEC-OPTIMIZATION-ARGMIN-MAP
title: "Spec 72 proves that the k least-cost candidates commute with a map (least_k_map) and that argmin ignores an order-preserving cost change, but no law moves argmin or argmin_option through a map: a client that selects over mapped candidates cannot rewrite the choice to the original list under the composed cost. Rule from a checked D0 at zero trust that argmin over nonempty_map f xs is f applied to argmin over xs under cost composed with f, with the option form if AC-0 rules it, and state it with a seed that separates the composed cost from the original one"
status: merged
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Optimization follows Symbolic algebra in the L3 layer rotation (docs/program/06-catalog-campaign.md). least_k_map gave the map law for the k-selection; this gives it for argmin. Steward-filed per COORDINATION section 2."
---

# Argmin commutes with a map

## Objective

`Algorithm.Optimization.Argmin` exports a public checked law. For every
`a`, `b`, `f : a → b`, `cost : b → Nat` and `xs : NonEmpty a`, it states
`Equal b (argmin b cost (nonempty_map a b f xs)) (f (argmin a (comp a b Nat
cost f) xs))`. It holds at zero trust, and spec 72 states it by its public
type.

## Settled inputs (main `4337872ee`; anchors re-checked at release)

- **The operations.**
  - `argmin` (`Argmin.ken.md:86`) is `argmin_from` over the head and tail.
    It keeps the earlier candidate on a tie (`argmin_head_on_tie`, `:124`).
  - `nonempty_map` (`NonEmpty.ken.md:62`) maps the head and uses
    `Derived.map` on the tail. Argmin does not yet import it.
  - `argmin_option` (`:215`) is `None` on `Nil` and otherwise `Some` of
    `argmin`, so its map form follows by cases.
- **The argument, for AC-0 to check.** Induct on the tail through
  `argmin_from`. At each step, the comparison in the mapped list is
  `lt_nat (cost (f y)) (cost (f x))`, which is exactly the comparison under
  `comp a b Nat cost f`. So the same branch is chosen, and `f` commutes
  with the chosen candidate. Ties fall the same way on both sides.
- **The neighbours.**
  - `least_k_map` (`:563`) is the k-selection form, with the same binder
    shape.
  - `argmin_cost_order_invariant` (`:162`) changes the cost, not the
    candidates.
  - `argmax` (`:594`) has the same structure. AC-0 rules whether
    `argmax_map` joins this WP.
- **No such law exists.** Nothing in `catalog/` or `spec/` relates `argmin`
  to `map` or `nonempty_map`.
- **Spec and tests.**
  - The new section is placed by AC-0 in spec 72, near `least_k_map`.
  - The seed directory is `conformance/stdlib/optimization/`, after
    `seed-least-k-map.md`.
  - The test pattern is
    `crates/ken-elaborator/tests/cat_optimization_least_k_map.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The spec 72 section, the catalog law or laws with any private helper, a
seed, a package test and the status lines. No existing operation or law
type moves, and there is no kernel or surface change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-OPTIMIZATION-LEAST-K-MAP.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules:
  - the name, the binder order and the placement;
  - whether the `argmin_option` and `argmax` forms are also public;
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`.
- **AC-1.** The spec section states the law by its public type. CV and the
  Architect review it, with a Decision.
- **AC-2.** The seed uses `a = b = Nat`, `cost` the identity, and an `f`
  that reverses the order of the candidates' costs, over three candidates.
  - Its accepted block applies the law and checks the chosen value by
    `Refl`.
  - Its refused block, on a fresh accepted prefix and in the same form,
    closes the same proof at `f (argmin a cost xs)`, the uncomposed cost.
    The closing `Refl` refuses, because that candidate is the most costly
    after `f`.
  - AC-0 picks the concrete `f` and may replace this seed with a stronger
    discriminator.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise on `f`.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing operation or law type.

Checks: 4 fired. I read `argmin`, `argmin_option`, `nonempty_map`,
`least_k_map` and `argmin_cost_order_invariant` on `4337872ee`. 3 fired.
I listed every public Argmin export and grepped `catalog/` and `spec/` for
`argmin` with `map`, and found none. 8 fired. An `f` that reverses the
cost order makes the uncomposed choice the costliest mapped candidate, so
the refused row fails a law that drops the composition.
