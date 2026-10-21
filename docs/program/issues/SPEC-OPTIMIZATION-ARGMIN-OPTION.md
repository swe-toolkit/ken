---
id: SPEC-OPTIMIZATION-ARGMIN-OPTION
title: "Spec 72 section 1 defers an Option-returning least-cost selector over a possibly empty List: a client holding a List must rebuild the NonEmpty split itself before it can call argmin. Rule the selector's name, its Nil and Cons equations, and any membership or optimality corollaries from a checked D0 at zero trust, and state them as a section 1 extension with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 72 sections 1.3 and 2.3 defer \"an Option-returning selector for a possibly empty List\" and place it as a section 1 extension that wraps argmin after distinguishing Nil from Cons. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md). Same shape as SPEC-SYMBOLIC-ALGEBRA-SUBSTITUTION-COMPOSITION. Steward-filed per COORDINATION section 2."
---

# A least-cost selector over a possibly empty List

## Objective

Spec 72 states a public selector that takes a cost and a `List a` and
returns `None` on the empty list and `Some` of `argmin` on a non-empty
one, with checked laws that make that equivalence usable by a client.

## Settled inputs (spec 72 §1, on `main` at `b22a41649`)

- **`argmin` (§1.1).** `pub fn argmin (a : Type) (cost : a → Nat)
  (xs : NonEmpty a) : a`, in
  `catalog/packages/Algorithm/Optimization/Argmin.ken.md`, with the
  seven §1.2 laws, including membership, least cost and the
  earliest-tie rule.
- **The deferral.** §1.3 defers "an `Option`-returning selector for a
  possibly empty `List`". §2.3 places it: "It would wrap `argmin` after
  distinguishing `Nil` from `Cons` ... it belongs as a §1 extension,
  not as another §2 operation."
- **Construction.** `NonEmpty`'s constructor is private. Clients build
  one with `nonempty_cons` and reason through `nonempty_to_list` and
  `nonempty_to_list_shape`. `Option` with `None` and `Some` is a
  prelude data type.
- **Seeds.** Under `conformance/stdlib/optimization/`.

Treat anchors as perishable. If a fixed input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §1 extension in spec 72 stating the ruled selector and each law by
its public type, the §1.3 and §2.3 deferral sentences corrected, and a
closed conformance seed under `conformance/stdlib/optimization/`, with
the status lines changed in the same candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0 and no `Axiom`. It rules:
  - the selector's name and definition;
  - its `Nil` and `Cons` equations as public laws, the `Cons` one
    stated against `argmin` of `nonempty_cons`;
  - whether a `Some y` result carries membership in the input list and
    least cost as public corollaries, or whether those follow from the
    `Cons` equation and the §1.2 laws without a new statement;
  - the private helpers the proof needs.
- **AC-1.** The extension states each ruled operation and law by its
  public type, and the deferral sentences are corrected. The Spec
  leader and CV review it, with a Decision.
- **AC-2.** The seed checks the selector on `Nil`, on a singleton, and
  on a list whose earliest least-cost element is not its head and ties
  with a later one, so a selector that returns the head, the last tie
  or `None` on a non-empty list fails. It has a refused block that
  claims `Some` on `Nil`.

## Stop conditions

- A law that needs a decidable equality on `a`, an order on `a`, or
  function extensionality, unless the D0 rules it as the stated premise.
- Any new primitive, kernel or surface feature, or a change to
  `argmin` or its seven laws.
- An ambient name in the package beyond its current census row
  (`lang_mod_strict_resolution_d0`): import it explicitly.

Checks: 4 fired: `Option`/`None`/`Some` are prelude data
(`crates/ken-elaborator/src/prelude.rs:905`); `nonempty_cons`,
`nonempty_to_list_shape` and `argmin` are `pub` in their modules at
`b22a41649`.
