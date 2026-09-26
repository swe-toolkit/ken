---
id: CAT-DERIVED-FILTER-MEMBERSHIP-LAW
title: "Proof-backfill slice for Data/Collections/Derived.ken.md: prove, for every element type, comparator, predicate and list, that membership in filter p xs is membership in xs together with p, under a stated comparator/predicate compatibility hypothesis, over the existing prelude filter and Derived mem with no new trust"
status: merged
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

## Statement -- pinned by the Architect (`evt_7wqwqvccgr8bz`)

Two private `theorem`s in `§4.1`, like `map_length`:

- **`mem_filter`** `(a : Type) (eqf : a → a → Bool) (p : a → Bool) (x : a)
  (compat : (y : a) → IsTrue (eqf x y) → Equal Bool (p y) (p x))
  (xs : List a) : Equal Bool (mem a eqf x (filter a p xs))
  (bool_and (mem a eqf x xs) (p x))`
- **`mem_filter_sound`** `(a : Type) (eqf : a → a → Bool) (p : a → Bool)
  (x : a) (xs : List a) : IsTrue (mem a eqf x (filter a p xs)) → IsTrue
  (mem a eqf x xs)`

Both are by induction on `xs`, with `filter` the installed prelude
combinator. There is no `Iff`, no `DecEq` and no new definition. Proof-local
helper lemmas are allowed.

## Technique (Architect `evt_7wqwqvccgr8bz`)

A proof-side `match p h` or `match eqf x h` does **not** refine the goal.
The goal holds those scrutinees only inside the δ-unfolded bodies of
`filter` and `mem`, and Ken abstracts only occurrences as written. This was
measured: after `match p h`, the arm's goal still reads
`filter a p (Cons a h t)`. It is the CAT-ARGPARSE-LAWS §1b predicate, and a
projection fix does not apply because `p h` is not a field of a variable.
Use three things instead:

1. **Name the outcome with an equation, through a return-type Π.** The step
   theorem takes `(b : Bool) (c : Bool)` and returns `Equal Bool (p h) b →
   Equal Bool (eqf x h) c → Goal`, matching on the variables `b` and `c`.
   The induction calls it at `(p h) (eqf x h) Refl Refl`.
2. **Rewrite the head with `cong` over the branch λ.** The λ's body is
   convertible to the combinator's body:

   ```
   theorem mem_filter_head_true
         (a : Type) (eqf : a → a → Bool) (p : a → Bool) (x : a) (h : a) (t : List a)
         (eb : Equal Bool (p h) True)
       : Equal Bool (mem a eqf x (filter a p (Cons a h t))) (mem a eqf x (Cons a h (filter a p t))) =
     cong Bool Bool (p h) True
       (λq. mem a eqf x (match q { True ↦ Cons a h (filter a p t); False ↦ filter a p t }))
       eb

   theorem mem_cons_head_true
         (a : Type) (eqf : a → a → Bool) (x : a) (h : a) (t : List a)
         (ec : Equal Bool (eqf x h) True)
       : Equal Bool (mem a eqf x (Cons a h t)) True =
     cong Bool Bool (eqf x h) True (λq. match q { True ↦ True; False ↦ mem a eqf x t }) ec
   ```

   The `_false` twins change `True` to `False` in the index and the
   right-hand side: `mem a eqf x (filter a p t)` and `mem a eqf x t`
   respectively. `match` is not allowed in a type position, so write one
   True and one False helper rather than a helper indexed by `b`.
3. **Close the cases.**
   - `(True, True)`: `compat h ec` with `sym eb` gives
     `Equal Bool True (p x)`.
   - `(False, True)`: `compat` with `eb` gives `Equal Bool (p x) False`.
     Close with the recursive hypothesis, then `bool_and_false_right (b : Bool) : Equal Bool (bool_and b
     False) False` (match `b`, `Proved` in both arms).
   - `(_, False)`: the recursive hypothesis.

   Soundness uses the same helpers, with no `compat`.

The Architect's scratch probe of both laws is at
`/tmp/cat-derived-filter-membership-architect-probe.ken.md` (reference, not a
deliverable; built with a 2026-09-21 `ken`, so re-measure on current main).

## Deliverable

The two pinned theorems in `Derived.ken.md` `§4.1`, general over `a`,
`eqf`, `p`, `x` and `xs`. Replace both hold-out sentences with the shipped
statement. No change to `filter`, `mem` or any other definition.

## Acceptance

- **AC-1 (no new trust).** The added lines contain no `Axiom`, postulate,
  primitive or kernel change. The roots-loaded `Derived` `trusted_base()` is
  equal as a set at base and candidate.
- **AC-2 (falsifiers).** Two, of different kinds.
  - (i) A scratch mutation, restored afterwards, that must fail the law at
    its own obligation: swap the `True`/`False` arms of `filter`'s
    `match p h` in the prelude.
  - (ii) A committed discriminating pair in the consumer-view harness, on
    the same shape. **Without `compat`, the equation is false:** with `eqf` constant
    `True`, `p` "is `Zero`", `x = Suc Zero` and `xs = [Zero]`, the left side
    is `True` and the right side is `bool_and True False = False`, so
    `Proved` for that instance must be **rejected**. **With `compat`, it
    holds:** with `eqf` Nat equality and the same `p`, `x` and `xs`,
    `Proved` must be **accepted**.
  The harness in `crates/ken-elaborator/tests` also pins that both statements resolve
  `filter` to the installed prelude identity, as `map_length` does for `map`
  (`cat3_collections_package.rs:252`).
- **AC-3.** Targeted builds only, through `scripts/ken-cargo`. No-regression
  means green in CI.
- **AC-4 (admitted census growth; Architect `evt_4grsne5p7yyc1`).** Full CI
  on `3cc16e8d1` reddened the directional
  `catalog_ambient_passthrough_migration_census`: the laws name the prelude
  combinator `filter` ambiently, and no catalog provider or import form for
  it exists. This one delta is admitted as prelude-convenience migration
  debt, the first admitted growth of that census.
  - Exactly one name, `filter`, is added, only to
    `Data.Collections.Derived` and to the rows whose closure roots-loads it
    (20 closure rows, 21 rows in all). No other row changes, no name is removed, and
    `discovered`, `clean` and the residual set are unchanged.
  - A one-line comment at the Derived row says `filter` is admitted
    prelude-convenience debt under this ruling and names this AC. The other
    rows need no comment.
  - The re-cut is one new commit on top of `3cc16e8d1` touching only
    `crates/ken-elaborator/tests/lang_mod_strict_resolution_d0.rs`. The
    `Derived.ken.md` and filter-test blobs stay byte-identical. Foundation
    QA re-runs the census targeted. A fresh Decision binds the new SHA.

## Stop conditions

Stop if the pinned statement needs a new `Iff`, a lawful-`DecEq` class
constraint or a change to `mem` or `filter`, or if a proof needs a fact that
no existing law states. A split on a scrutinee that is neither a variable
nor an outcome named through the return-type-Π equation of Technique step 1
is a STOP for a ruling.
