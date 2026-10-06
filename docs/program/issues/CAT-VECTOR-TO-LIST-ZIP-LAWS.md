---
id: CAT-VECTOR-TO-LIST-ZIP-LAWS
title: "Add Vector's to_list, zip and unzip and prove the laws spec 60 §5 still defers: length of to_list is the index, and zip and unzip are mutually inverse"
status: ready
owner: foundation
size: S
tier: T1
gate: architect
depends_on: [CAT-VECTOR-DEFERRED-LAWS]
blocks: []
github: null
origin: "L3 proof backfill (operator 2026-09-13: 'A catalog package is not finished until its proofs are complete'; Architect evt_5f1ewknxv3m6h: L3 turns to proof backfill). The two laws spec/50-stdlib/60-length-indexed-vectors.md §5 defers that CAT-VECTOR-DEFERRED-LAWS left out because the package lacked the functions. Steward-filed per COORDINATION section 2."
---

# Vector's to_list bridge and zip/unzip round trip

## Objective

`Data/Vector/Vector.ken.md` defines `to_list`, `zip` and `unzip` and proves
the two laws spec 60 §5 still lists as deferred: the length/`to_list`
bridge and the `zip`-`unzip` round trip.

## Settled inputs (read at `5131e183e`)

- **Spec.** §5 defers "the `zip`-`unzip` round-trip" and "the length /
  `to_list` bridge (`Vec A n → List A` and `length ∘ to_list ≡ n`)". §4
  types `zip` as `(A B : Type) → (n : Nat) → Vec A n → Vec B n →
  Vec (Pair A B) n`.
- **Why it was left out.** `CAT-VECTOR-DEFERRED-LAWS.md:111`: "Both need
  functions the package does not have." The package has `head`, `tail`,
  `map`, `zip_with` and `lookup` (`Vector.ken.md:50-87`), and its laws are
  private checked proofs.
- **Enablers.** `Pair`, `mk_pair`, `pair_fst` and `pair_snd` are prelude
  (`prelude.rs:1291`; used by `Deque.ken.md:45`). `length` is `pub` in
  `Data.Collections.Derived` (`Derived.ken.md:241`), which does not import
  Vector. The kernel converts Σ by η (`conv.rs:2723`
  `sigma_eta_convert`), so `mk_pair (pair_fst p) (pair_snd p)` is expected
  to be convertible to `p`.
- `zip_with` already peels both inputs at the shared index
  (`Vector.ken.md:66-75`), and `zip` can be defined through it.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

Define `to_list`, `zip` and `unzip` in Vector, with the same visibility as
its other functions, and prove the laws below as checked theorems. Rewrite
§5's deferred bullets in the package prose so they name these theorems.

## Acceptance

- **AC-1.** `length a (to_list a n xs)` equals `n` for every `xs : Vec a n`.
- **AC-2.** `unzip (zip xs ys)` equals the pair of `xs` and `ys`, and
  `zip` applied to the components of `unzip ps` equals `ps`, for every
  `n`.
- **AC-3 (controls).** Every existing Vector theorem and example still
  checks. Run `ken check` on the package, and the catalog tests that load
  it.
- **AC-4 (mutation, QA).** Swapping the components in `unzip`, or dropping
  the head element in `to_list`, reddens the matching law.

## Stop conditions

- The second half of AC-2 does not close because Σ η does not hold for
  `pair_fst`/`pair_snd` on a variable. Stop to the Architect; do not add an
  axiom.
- Importing `Derived` creates a cycle, or forces a change to a prelude
  name.
- Any kernel, `trusted_base()` or spec change.
