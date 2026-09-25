---
id: CAT-DERIVED-SORT-LAWS
title: "Proof-backfill slice for Data/Collections/Derived.ken.md: prove that the generic insertion sort preserves every element count for every comparator, and returns a sorted list under a stated totality hypothesis on the comparator, over the existing insert and sort with no new trust"
status: merged
owner: foundation
size: S
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Proof-backfill slice of CAT-DERIVED-COLLECTIONS-LAWS named by docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md ('generic sort has only a concrete Bool proof family'), under operator ruling 2026-09-13 ('schedule the proof backfill before extending the catalog'). L3 successor after CAT-DERIVED-FILTER-MEMBERSHIP-LAW. Steward-filed per COORDINATION section 2."
---

# The generic `sort` has no laws

## Fixed inputs -- measured at `002ad0855`

- `catalog/packages/Data/Collections/Derived.ken.md` `§4.3` defines
  `insert (a) (le : a → a → Bool) (x) (xs)`, which matches `le x h` at each
  cell, and `sort (a) (le) (xs)` by insertion. It also defines `count (a)
  (eqf) (x) (xs) : Nat` and `Perm (a) (eqf) (xs) (ys) : Prop` as
  count equality at every `x`.
- The only proofs in `§4.3` are about `sort_bool`, a separate direct
  implementation over `List Bool`. No theorem mentions the generic `insert`
  or `sort`.
- `is_sorted (a) (leq) (xs) : Prop` is the prelude's adjacent-pair form
  (`crates/ken-elaborator/src/prelude.rs`, `is_sorted`).
- `catalog/packages/Algorithm/Sorting/InsertionSort.ken.md` proves both
  laws for its own `Ord`-dictionary insertion sort. Its sortedness proof
  needs totality only, not transitivity (`leq_right_of_left_false`,
  `head_ordered_after_insert`). It imports `Derived`, so `Derived` cannot
  import it.
- `CAT-DERIVED-FILTER-MEMBERSHIP-LAW` pinned the technique for a law over a
  function that matches an applied comparator: name the outcome through a
  return-type-Π equation and rewrite with `cong` over the branch λ.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Statement -- pinned by the Architect (`evt_5eja2xscse1h7`)

Four **attached** proofs in `§4.3`, all private because `insert` and
`sort` are private. `Derived` owns `insert` and `sort`, so this is the
attached-proof ownership form `InsertionSort` uses, not free `theorem`s.

```
proof count for insert
      (a : Type) (le : a → a → Bool) (x : a) (xs : List a) (eqf : a → a → Bool) (q : a)
    : Equal Nat (count a eqf q (Cons a x xs)) (count a eqf q (insert a le x xs))

proof perm for sort
      (a : Type) (le : a → a → Bool) (xs : List a) (eqf : a → a → Bool)
    : Perm a eqf xs (sort a le xs)

proof sorted for insert
      (a : Type) (le : a → a → Bool)
      (total : (x : a) → (y : a) → IsTrue (bool_or (le x y) (le y x)))
      (x : a) (xs : List a)
    : is_sorted a le xs → is_sorted a le (insert a le x xs)

proof sorted for sort
      (a : Type) (le : a → a → Bool)
      (total : (x : a) → (y : a) → IsTrue (bool_or (le x y) (le y x)))
      (xs : List a)
    : is_sorted a le (sort a le xs)
```

- **The count laws need no hypothesis.** They hold for every `le` and
  every `eqf`: swapping two adjacent cells never changes a count, whatever
  the two `eqf q _` decisions are.
- **Totality is `Ord.total`'s field type verbatim**
  (`Core.Classes.LawfulClasses:124`). A consumer holding `d : Ord a` passes
  `d.total` at `le := ord_leq_at a d` with no adapter. Add `bool_or` to
  `Derived`'s existing `import Core.Classes.LawfulClasses (...)` line; this
  adds no new dependency and no trust. The pre-digested premise
  `le x y = False → le y x = True` is rejected.
- **The step the proof needs, with no `J`:** `trans Bool (le y x) (bool_or
  (le x y) (le y x)) True (sym Bool (bool_or (le x y) (le y x)) (le y x)
  (cong Bool Bool (le x y) False (λq. bool_or q (le y x)) e)) (total x y)`,
  where `e : Equal Bool (le x y) False`. The `cong` endpoint `bool_or False
  (le y x)` δι-reduces to `le y x`.
- **No transitivity:** insertion never moves an element past a head that
  was not already before it, as in `InsertionSort`'s
  `head_ordered_after_insert`.

## Technique (Architect `evt_5eja2xscse1h7`)

Two forms are licensed. In both, the rewrite moves the goal, not the match.

1. The filter slice's form: name the outcome through a return-type-Π
   equation on a variable `b`, then rewrite the head with `cong` over the
   branch λ (`mem_filter_cons_case`, landing in `3cc16e8d1`).
2. `InsertionSort`'s form: `match le x h eqn : e { … }`, where each arm
   rewrites with `J` or `cong` over the explicit branch λ `match decision {
   True ↦ Cons a x (Cons a h t); False ↦ Cons a h (insert a le x t) }`
   against `sym … e`.

Proof-local helpers are allowed: generic analogues of `InsertionSort`'s
`count_cons_cong`, `count_cons_swap`, `sorted_cons`, `sorted_tail`,
`sorted_head` and `head_ordered`. The count helpers range over `eqf` only,
not `le`. A private helper `fn` used only by proofs, like `count_after_two`,
is allowed.

These laws are not redundant with `InsertionSort`: it imports `Derived`, so
`Derived` cannot reuse its lemmas. Making `InsertionSort` import the generic
count helpers is a later slice, not this one.

## Deliverable

The four pinned proofs in `Derived.ken.md` `§4.3`, and the `bool_or`
import. No change to `insert`, `sort`, `count`, `Perm`, `sort_bool` or any
other definition.

## Acceptance

- **AC-1 (no new trust).** The added lines contain no `Axiom`, postulate,
  primitive or kernel change. The roots-loaded `Derived` `trusted_base()` is
  equal as a set at base and candidate.
- **AC-2 (falsifiers).**
  - (i) Scratch mutations, restored afterwards. Report the full red and
    green set for each.
    - (i-a) In `insert`, `False ↦ insert a le x t` (head drop) must redden
      `insert::count` or `sort::perm`.
    - (i-b) Its mirror, `False ↦ Cons a x (Cons a h t)`, preserves every
      count and breaks order. It must redden `insert::sorted` or
      `sort::sorted`.
  - (ii) A committed discriminating pair on one shared fixture:
    `le_none (x : Nat) (y : Nat) : Bool = False`, `leq_nat`, and
    `xs = Cons Nat (Suc Zero) (Cons Nat Zero (Nil Nat))`.
    - Positive controls: `sort Nat le_none xs` and `sort Nat leq_nat xs`
      are both `Proved` equal to `[Zero, Suc Zero]`, so the verdict flips
      only on the comparator.
    - `is_sorted Nat leq_nat [Zero, Suc Zero]` must be **accepted**, and
      `is_sorted Nat le_none (sort Nat le_none xs)` must be **rejected** as
      `KernelRejected(TypeMismatch)` at that closed proof. Use the same
      closed proof term for both, and have every fixture declaration
      elaborate before the rejection. `le_none` is non-total because
      `bool_or False False = False`.
  - **Harness pins.** Each of the four raw kernel types references
    `Data.Collections.Derived.insert` or `.sort`, never
    `Algorithm.Sorting.InsertionSort.sort`. Generic consumers re-derive all
    four full binder lists.
- **AC-3.** Targeted builds only, through `scripts/ken-cargo`. No-regression
  means green in CI.

## Stop conditions

Stop if a law needs transitivity, a lawful `DecEq` or `Ord` class
constraint, or a change to any definition. A split on an applied scrutinee
is allowed only as technique form (1) or (2), with the goal moved by a
`cong`/`J` rewrite over the branch λ. Any proof that relies on a proof-side
`match` refining the goal is a STOP for a ruling.
