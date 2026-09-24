---
id: CAT-DERIVED-FILTER-MEMBERSHIP-LAW
title: "Proof-backfill slice for Data/Collections/Derived.ken.md: prove, for every element type, comparator, predicate and list, that membership in filter p xs is membership in xs together with p, under a stated comparator/predicate compatibility hypothesis, over the existing prelude filter and Derived mem with no new trust"
status: ready
owner: foundation
size: S
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Proof-backfill slice of CAT-DERIVED-COLLECTIONS-LAWS named by docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md, under operator ruling 2026-09-13 ('schedule the proof backfill before extending the catalog'). L3 successor while CAT-ARGPARSE-LAWS laws 1-2 wait on HS3 Research and BYTES D2 waits on the kenfmt repair. Steward-filed per COORDINATION section 2."
---

# `filter` has no membership law

## Fixed inputs -- measured at `d7f6c2bcf`

- `filter` is a prelude combinator, not a catalog function
  (`crates/ken-elaborator/src/prelude.rs`, the `combinator_trusted_before`
  bracket): `Nil ↦ Nil a; Cons h t ↦ match p h { True ↦ Cons a h (filter a p
  t); False ↦ filter a p t }`. `map_length` in the same package is the
  precedent for a law over a prelude combinator.
- `catalog/packages/Data/Collections/Derived.ken.md` defines
  `mem (a : Type) (eqf : a → a → Bool) (x : a) (xs : List a) : Bool` (`§4.1`,
  `:161`), matching `eqf x h` at each cell. It imports `IsTrue`,
  `bool_and`, `cong`, `sym` and `trans`.
- The package holds the law out twice, in `§4.1` (`:142`) and in `§6`
  Findings (`:998`), "until its comparator/Iff statement is pinned". No bare
  `Prop` wrapper was shipped.
- `mem` compares with an arbitrary `eqf`, and `p` is arbitrary. Without a
  hypothesis linking them, `mem x (filter p xs)` and `p x` are unrelated:
  a cell `h` with `eqf x h = True` and `p h ≠ p x` breaks any equation.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Statement -- the Architect pins it at frame review

The Steward's candidate, for the Architect to confirm or replace:

- **Characterization.** Given `compat : (y : a) → IsTrue (eqf x y) → Equal
  Bool (p y) (p x)`, prove `Equal Bool (mem a eqf x (filter a p xs))
  (bool_and (mem a eqf x xs) (p x))`, by induction on `xs`. An `Equal Bool`
  form avoids a new `Iff`.
- **Soundness, hypothesis-free.** `IsTrue (mem a eqf x (filter a p xs)) →
  IsTrue (mem a eqf x xs)`.

## Deliverable

The pinned laws as theorems in `Derived.ken.md` `§4.1`, general over `a`,
`eqf`, `p`, `x` and `xs`. Replace both hold-out sentences with the shipped
statement. No change to `filter`, `mem` or any other definition.

## Acceptance

- **AC-1 (no new trust).** The added lines contain no `Axiom`, postulate,
  primitive or kernel change. The roots-loaded `Derived` `trusted_base()` is
  equal as a set at base and candidate.
- **AC-2 (falsifiers).** Each is a scratch mutation, restored afterwards,
  and each must fail the named law at its own obligation:
  - swap the `True`/`False` arms of `filter`'s `match p h` in the prelude;
  - drop the `compat` hypothesis from the characterization (it must stop
    checking, showing the hypothesis is used).
  A consumer-view harness in `crates/ken-elaborator/tests` states each law
  at a concrete instance that reaches both `filter` arms.
- **AC-3.** Targeted builds only, through `scripts/ken-cargo`. No-regression
  means green in CI.

## Stop conditions

Stop if the pinned statement needs a new `Iff`, a lawful-`DecEq` class
constraint or a change to `mem` or `filter`, or if a proof needs a fact that
no existing law states.
