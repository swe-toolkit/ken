---
id: SPEC-TIME-MERGE-ALL-ORDERED-SORT-CONCAT
title: "Spec 69 proves that merging two ordered traces is sorting their append (§22), and that merging a list of ordered traces is ordered (trace_merge_all_ordered), but no law identifies the merge of a list of ordered traces: a client holding trace_merge_all over ordered batches cannot transport Derived's perm and stable_class proofs to it. Rule from a checked D0 at zero trust that, when every trace is time-ordered, trace_merge_all equals the sort of the traces' concatenation, and state it with a seed that separates the sort from plain concatenation"
status: merged
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Time follows Numerical in the L3 layer rotation (docs/program/06-catalog-campaign.md). §22 tied the two-trace merge to sort; trace_merge_all is the list form, and only its order, membership and count are stated. Steward-filed per COORDINATION section 2."
---

# Merging ordered batches is sorting their concatenation

## Objective

`Data.Time.EventTrace` exports a public checked theorem. For every `p` and
`traces : List (List (Event p))`, from `(t : List (Event p)) → list_elem
(List (Event p)) t traces → time_ordered p t`, the law concludes `Equal
(List (Event p)) (trace_merge_all p traces) (sort (Event p) (event_before
p) (list_concat (Event p) traces))`. It holds at zero trust, and spec 69
states it by its public type.

## Settled inputs (main `fb353916f`; anchors re-checked at release)

- **The operations.**
  - `trace_merge_all` (`EventTrace.ken.md:132`) folds `trace_merge` from
    the right, with `Nil` at the end.
  - `sort` (`Derived.ken.md:1935`) is insertion sort, also folded from the
    right.
  - `list_concat` (`Derived.ken.md:109`) is already imported (`:84`).
- **The neighbours.**
  - `trace_merge_ordered_sort` (`:3881`, §22) is the two-trace law, with
    both inputs ordered.
  - `trace_merge_all_ordered` (`:2612`) carries the exact all-ordered
    premise shape used here.
  - `trace_merge_all_count` (`:2687`) states the count against
    `list_concat`, which is the same flattening.
- **The argument, for AC-0 to check.** Induct on `traces`. At `Cons t
  rest`:
  - By the induction hypothesis, `trace_merge_all p rest` is `sort` of
    `list_concat rest`, and that is ordered.
  - §22, at `t` and that sort, gives `sort (list_append t (sort zs))`.
  - What remains is `sort (list_append xs (sort zs)) = sort (list_append
    xs zs)`. It follows from `sort (list_append xs ys)` being a right fold
    of `insert` over `xs` onto `sort ys`, together with §22's private
    proof that sorting an ordered list leaves it unchanged.
  - AC-0 may find a shorter route.
- **Time carry.** A sort law's `Nil` arm can refuse `Refl` and need
  `Proved` (Architect `evt_24ejty59dzrgc`). AC-0 states which form checks
  here.
- **No such law exists.** Nothing in `catalog/` or `spec/` equates
  `trace_merge_all` with `sort`, except over singletons (§21).
- **Spec and tests.**
  - Spec 69 ends with §22 (`69-time-and-events.md:1809`), so the new
    section is §23.
  - The seed directory is `conformance/stdlib/time-and-events/`, after
    `seed-trace-merge-ordered-sort.md`.
  - The test pattern is
    `crates/ken-elaborator/tests/cat_time_merge_ordered_sort.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 69 §23, the catalog law with any private helper, a seed, a package
test and the status lines. No §1 to §22 operation or law type moves, and
there is no kernel or surface change, and no change to Derived.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-TIME-MERGE-ORDERED-SORT-APPEND.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules:
  - the name, the binder order and the placement;
  - whether the helper `sort (xs ++ sort ys) = sort (xs ++ ys)` stays
    private or is public, and in which package;
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`.
- **AC-1.** §23 states the law by its public type. CV and the Architect
  review it, with a Decision.
- **AC-2.** The seed's traces are `[[1, 3], [2]]`.
  - Its accepted block applies the law and checks the sort's value `[1, 2,
    3]` by `Refl`.
  - Its refused block, on a fresh accepted prefix and in the same form,
    closes the same proof at the concatenation `[1, 3, 2]`, and the
    closing `Refl` refuses.
  - A premise row shows that the law has no premise-free form. For the
    single unordered trace `[[5, 1]]`, the merge returns `[5, 1]` but the
    sort gives `[1, 5]`.
  - AC-0 may replace this seed with a stronger discriminator.
- **AC-T.** The package test compares `trusted_base()` before and after
  EventTrace as identity sets, and that comparison fails when the package
  adds an `Axiom`. No Decision opens until it passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, a change to
  Derived, or a premise beyond the all-ordered premise.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §22 operation or law type.

Checks: 4 fired. I read `trace_merge_all`, `sort`, `list_concat`,
`trace_merge_ordered_sort`, `trace_merge_all_ordered` and
`trace_merge_all_count` on `fb353916f`. 3 fired. I listed every public
EventTrace export and found no list-form sort law. 8 fired. `[1, 3, 2]` is
the concatenation and `[1, 2, 3]` the sort, so the refused row tells sort
apart from append. `[[5, 1]]` shows the premise is needed.
