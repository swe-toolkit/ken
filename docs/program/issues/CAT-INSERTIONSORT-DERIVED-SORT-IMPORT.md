---
id: CAT-INSERTIONSORT-DERIVED-SORT-IMPORT
title: "InsertionSort re-derives, lemma for lemma, the generic insertion-sort development Derived §4.3 already proves. Publish Derived's sort development and make InsertionSort's public surface instantiations of it, retiring 11 duplicated locals"
status: ready
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [CAT-FOKRIPKE-TRANSPORT-IMPORT]
blocks: []
github: null
origin: "Architect nomination evt_654w10cneaq4q on Steward evt_16mw0f8zb8e9d: next L3 proof-backfill slice (operator 2026-09-13), catalog §2a factoring. Steward-filed per COORDINATION section 2."
---

# InsertionSort instantiates Derived's sort

## Objective

`Algorithm/Sorting/InsertionSort.ken.md` keeps only what is specific to it,
the `Ord`-dictionary packaging. Its public `sort`, `insert`, `permutation`
and their laws are instantiations of `Data/Collections/Derived.ken.md`
§4.3 at `ord_leq_at a d`, and it re-derives nothing.

## Settled inputs (Architect `evt_654w10cneaq4q`, read at `bde3de6cf`)

- **The duplication.** There are 11 locals, about 300 lines, and they
  differ from Derived only in specializing the comparator:

  | InsertionSort | Derived |
  |---|---|
  | `head_ordered` | `derived_sort_head_ordered` |
  | `sorted_cons/_tail/_head` | `derived_sort_sorted_cons/_tail/_head` |
  | `leq_right_of_left_false` | `derived_sort_right_of_left_false` |
  | `head_ordered_after_insert` | `derived_sort_head_after_insert` |
  | `count_cons_cong` | `derived_sort_count_cons_cong` |
  | `count_after_two` | `derived_sort_count_after_two` |
  | `count_swap_decisions` | `derived_sort_count_swap_decisions` |
  | `count_cons_swap` | `derived_sort_count_cons_swap` |
  | `insert::count`, `insert/sort::sorted` | Derived's `insert::count`, `insert/sort::sorted` |
  | `sort::permutation` | `sort::perm` |

- **Direction.**
  - Derived is the provider. InsertionSort already imports `count` and
    `eq_from_ord` from it (`:18`), so there is no new dependency edge.
  - Derived imports nothing from `Algorithm.*`, so there is no cycle.
  - The generic form needs only a total comparator, not the whole `Ord`
    dictionary.
- **Spellings.**
  - `Ord.total : (x y : a) → IsTrue (bool_or (leq x y) (leq y x))`
    (`LawfulClasses.ken.md:124`) has the shape of Derived's `total`
    parameter, with `le := ord_leq_at a d`.
  - The qualified alias `import Data.Collections.Derived as DC` is
    delivered (`EffectfulClasses.ken.md:67`, `LawfulFunctors.ken.md:38`).
  - A fully qualified attached proof is delivered
    (`Config/Decoder.ken.md:911`).
  - Spec 33 §8.2 (`:1176-1181`): `pub proof p for s` is exportable iff `s`
    is, and it is reachable as `M.s::p`.
- **Consumers.**
  - The 11 retired names occur only in `InsertionSort.ken.md` and
    `cat_sort_insertion_sort_acceptance.rs`. Map's `sorted_tail` is a
    different, keyed theorem.
  - All 20 Derived imports on main are selective lists or qualified, and
    none is open. Publishing `insert` and `sort` therefore cannot capture a
    bare name.
  - Derived's export is pinned in three suites: `cat_derived_pub_export.rs`
    (authoritative per Derived §8), `lang_mod_catalog_completeness.rs` and
    `cat_derived_sort_laws.rs`.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **Derived: `pub` markers only, bodies byte-unchanged.**
   - Mark `insert` (`:1116`), `sort` (`:1126`), `Perm` (`:1113`),
     `proof sorted for insert`, `proof count for insert`,
     `proof sorted for sort` and `proof perm for sort` as `pub`.
   - Update the §4.3 "package-local" prose and the §8 Public API list.
   - Update the export pins in the three suites. The doc comment in
     `cat_derived_sort_laws.rs` that calls them private changes too.
2. **InsertionSort.** Retire the 11 locals. Arrange the file top-down, with
   `sort` and its two laws first. The public surface becomes:

   ```ken
   import Data.Collections.Derived as DC
   fn sort (a : Type) (d : Ord a) (xs : List a) : List a = DC.sort a (ord_leq_at a d) xs
   proof sorted for sort (a : Type) (d : Ord a) (xs : List a)
       : is_sorted a (ord_leq_at a d) (sort a d xs) = DC.sort::sorted a (ord_leq_at a d) d.total xs
   proof permutation for sort (a : Type) (d : Ord a) (xs : List a)
       : permutation a d xs (sort a d xs) = DC.sort::perm a (ord_leq_at a d) xs (eq_from_ord a (ord_leq_at a d))
   fn permutation (a : Type) (d : Ord a) (xs : List a) (ys : List a) : Prop = DC.Perm a (eq_from_ord a (ord_leq_at a d)) xs ys
   fn insert (a : Type) (d : Ord a) (x : a) (xs : List a) : List a = DC.insert a (ord_leq_at a d) x xs
   proof sorted for insert (a : Type) (d : Ord a) (x : a) (xs : List a)
       : is_sorted a (ord_leq_at a d) xs → is_sorted a (ord_leq_at a d) (insert a d x xs) = DC.insert::sorted a (ord_leq_at a d) d.total x xs
   proof permutation for insert (a : Type) (d : Ord a) (x : a) (xs : List a)
       : permutation a d (Cons a x xs) (insert a d x xs) = λq. DC.insert::count a (ord_leq_at a d) x xs (eq_from_ord a (ord_leq_at a d)) q
   ```

   If `d.total` does not elaborate as a bare value, the eta form
   `λx. λy. d.total x y` is allowed. No other adaptor is. Drop any import
   that becomes unused, and report it.
3. **`cat_sort_insertion_sort_acceptance.rs`.**
   - The expected inventory becomes the 7 public names, and the retired
     list gains the 11 locals.
   - The decision-head classification and the 145/62/50 provider call
     populations measured the re-derivation. Replace them with two pins:
     - each of the 7 bodies has as its head the exact `GlobalId` of the
       matching Derived symbol (`insert`, `sort`, `Perm`, `insert::sorted`,
       `sort::sorted`, `insert::count`, `sort::perm`);
     - InsertionSort's own bodies contain no Boolean decisions.
   - The concrete Bool vectors and the zero trust delta stay unchanged.
   - If InsertionSort's ambient-name list in `lang_mod_strict_resolution_d0.rs`
     shrinks (`and_*` may drop), update it and report it.

## Acceptance

- **AC-0 (probe; no edit).** `DC.insert::sorted` resolves through `as DC`.
  If only `Data.Collections.Derived.insert::sorted` resolves, use it and
  report. Re-measure the 11 locals and their consumers.
- **AC-1.** `ken check` and `ken fmt --check` pass on both packages. The
  two Derived-alias packages, Config/Decoder, the three pin suites and the
  concrete sort vectors keep their verdicts. `trusted_base()` has no delta.
- **AC-2 (falsifiers; each must redden).**
  - M1: restoring `sorted_cons` turns the inventory pin red.
  - M2: replacing `DC.insert::sorted` with a local re-derivation turns the
    provider-identity pin red.
  - M3: flipping the comparator in `insert` to
    `λx. λy. ord_leq_at a d y x` stops `insert::sorted` from elaborating.
  - M4: withdrawing `pub` from Derived's `proof sorted for insert` stops
    InsertionSort from loading.
- **AC-3.** Foundation QA runs the Architect's §2a review: the factoring,
  and the top-down arrangement.

## Stop conditions (to the Architect)

- S1: neither the `DC.` nor the fully qualified spelling resolves.
- S2: `d.total`, or its eta form, does not check against `total` without a
  new lemma.
- S3: any Derived change beyond `pub` markers, prose and the export pins.
- S4: a verdict changes in any consumer named in AC-1.
- S5: a retired name has a consumer outside InsertionSort and its
  acceptance test.
