---
id: CAT-VECTOR-TO-LIST-ZIP-LAWS
title: "Add Vector's to_list, zip and unzip and prove the zip-unzip round trip spec 60 §5 defers, with no import that raises Vector's zero trusted_base() delta"
status: merged
owner: foundation
size: S
tier: T1
gate: architect
depends_on: [CAT-VECTOR-DEFERRED-LAWS]
blocks: []
github: null
origin: "L3 proof backfill (operator 2026-09-13: 'A catalog package is not finished until its proofs are complete'; Architect evt_5f1ewknxv3m6h: L3 turns to proof backfill). The two laws spec/50-stdlib/60-length-indexed-vectors.md §5 defers that CAT-VECTOR-DEFERRED-LAWS left out because the package lacked the functions. Steward-filed per COORDINATION section 2."
---

# Vector's to_list, zip and unzip, and the zip/unzip round trip

## Objective

`Data/Vector/Vector.ken.md` defines `to_list`, `zip` and `unzip` and proves
the `zip`-`unzip` round trip spec 60 §5 defers. Its import closure stays
Combinators and Transport, so its trust delta stays zero (§7).

## Settled inputs (read at `5131e183e`; recut on Architect `evt_17t0yjaee4atj`)

- **Spec.** §5 defers "the `zip`-`unzip` round-trip". §4 types `zip` as
  `(A B : Type) → (n : Nat) → Vec A n → Vec B n → Vec (Pair A B) n`. §7 is
  normative: zero `trusted_base()` delta and zero `Axiom` for Vec.
- **Enablers.** `Pair`, `mk_pair`, `pair_fst` and `pair_snd` are prelude
  (`prelude.rs:1291`). The kernel converts Σ by η (`conv.rs:2723`).
- **Vec-zero uniqueness** (Architect `evt_79v962excq5ab`): match `ys` only
  at a variable index, through the private `vec_nil_case`,
  `vec_nil_case_holds` and `vec_zero_vnil` given in that ruling. A direct
  match on `ys : Vec b Zero` builds an ill-typed motive.
- **No `length` here.** `Data.Collections.Derived` imports modules that
  declare five trusted axioms, so importing its `length` raises Vector's
  cold trust by 5. A private `length` is a redundant reimplementation. The
  `length ∘ to_list` law moves to `CAT-VECTOR-TO-LIST-LENGTH-LAW`, after
  `CAT-LIST-LENGTH-TRUST-FREE-BASE`.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

Define `to_list`, `zip` and `unzip` in Vector, with the same visibility as
its other functions, and prove `unzip_zip` and `zip_unzip` as private checked
theorems, arranged top-down as the Architect ruled. Rewrite §5's
`zip`-`unzip` bullet in the package prose so it names them. Revert any
Derived import and the fixture additions that came with it.

## Acceptance

- **AC-1.** `unzip (zip xs ys)` equals the pair of `xs` and `ys`, and `zip`
  applied to the components of `unzip ps` equals `ps`, for every `n`.
- **AC-2 (controls).** Every existing Vector theorem and example still
  checks, and the cold Vector trust test passes unchanged: no rebaseline.
- **AC-3 (mutation, QA).** Swapping the components in `unzip` reddens AC-1.

## Stop conditions

- Any trust delta, `Axiom`, kernel or spec change.

## Symptom inventory

```text
SYMPTOM INVENTORY (append one line per hard-stop; never rewrite history)
1. uniqueness of Vec b Zero needed under a dependent match; a match on a scrutinee at a non-variable index whose goal mentions a constructor at that index builds an ill-typed motive -- keyed on the elaborator abstracting index terms rather than refining by unification
2. the canonical List length lives in a module whose import closure carries trusted axioms, so a zero-delta package cannot import it -- keyed on trust being inherited per loaded module closure, not per used declaration
```

## Closeout

Merged `ac3a35879` from exact `282f6d1d5` (PR #4551). Foundation QA
`evt_2vrtd4ad774eq`, Architect `evt_2njrtc4jzz4ws`, Decision
`dec_1aa0dqhb7y0gf`. §1a count 2.

- `to_list`, `zip` and `unzip` are defined beside `head`, `tail` and `map`.
  `unzip_zip` (with its `fst` and `snd` halves) and `zip_unzip` are checked
  for every `n`, Vec-zero uniqueness going through `vec_nil_case`.
- The imports stay Combinators and Transport; cold trust 109 = 109.
- The `length ∘ to_list` law moves to `CAT-VECTOR-TO-LIST-LENGTH-LAW`, after
  `CAT-LIST-LENGTH-TRUST-FREE-BASE`.
