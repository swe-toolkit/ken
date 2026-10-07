---
id: CAT-DERIVED-ZIP-PROJECTION-LAWS
title: "Proof-backfill for Data/Collections/Derived.ken.md: prove privately that zip's first projection is the first list truncated to the second's length, and its second projection the second list truncated to the first's, so zip's contents and not only its length are checked"
status: active
owner: foundation
size: S
tier: T2
gate: architect
depends_on: []
blocks: []
github: null
origin: "Survey row Data/Collections/Derived (docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md): list and zip operations lack general structural laws. Operator 2026-09-13: a catalog package is not finished until its proofs are complete. Architect evt_5f1ewknxv3m6h: L3 turns to proof backfill. Steward-filed per COORDINATION section 2."
---

# `zip` is checked for length only

## Objective

Derived proves what `zip` returns, not only how long it is.

## Settled inputs (measured at `c3843e968`)

- `fn zip (a b : Type) (xs : List a) (ys : List b) : List (Pair a b)` in
  `catalog/packages/Data/Collections/Derived.ken.md` is transparent and
  structural, with `Nil` on either empty side and
  `Cons (mk_pair a b h h2) (zip a b t t2)` otherwise.
- `zip_length` is its only law. A `zip` with wrong pair contents and the
  same length still satisfies it, and the concrete
  `zip_truncates_at_shorter_list_concrete_example` is tested-only.
- `map`, `take` and `cong` are already in scope in Derived. `length` is
  imported from `Data.Collections.List`. `pair_fst` and `pair_snd` are
  transparent prelude projections over the `Sigma` behind `Pair`.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

Two private checked theorems in Derived, over every `a`, `b`, `xs`, `ys`:

```
theorem zip_fst (a : Type) (b : Type) (xs : List a) (ys : List b)
  : Equal (List a) (map (Pair a b) a (pair_fst a b) (zip a b xs ys))
                   (take a (length b ys) xs)
theorem zip_snd (a : Type) (b : Type) (xs : List a) (ys : List b)
  : Equal (List b) (map (Pair a b) b (pair_snd a b) (zip a b xs ys))
                   (take b (length a xs) ys)
```

Match both lists in every arm: `take` is stuck on an abstract length. No new
import, export, operation, axiom or trust. Follow
`docs/program/07-catalog-style-guide.md` and
`agent/playbooks/tools/write-ken.md`.

## Acceptance

- **AC-1.** Both theorems check, and the stated types are the literal ones
  above (expressibility first: state them before proving).
- **AC-2 (proposition pin).** A test decodes each private declaration's
  checked type and asserts its binders and exact `Equal` endpoints by
  global identity and de Bruijn index, not by definitional equality.
  Control: replacing either statement with a reflexive one keeps the
  package loading and reddens the pin.
- **AC-3 (mutation, QA).** Two scratch mutants of `zip` that keep
  `zip_length`'s statement true: one with a wrong first component after
  the head pair, one with a wrong second component. Each reddens its own
  law at that law's span. Restore byte-identically.
- **AC-4.** The loaded closure's trust ledger is unchanged. Targeted builds
  only, through `scripts/ken-cargo`. No-regression means green in CI.

## Stop conditions

- Any trust delta, `Axiom`, kernel, prelude or spec change, or a needed
  change to `zip`, `map`, `take` or `length`.
