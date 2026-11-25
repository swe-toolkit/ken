---
id: SPEC-TIME-MERGE-ORDERED-SORT-APPEND
title: "Spec 69 proves that merging singleton traces is insertion sort (§21), but says nothing like it for two ordered traces: a client holding a merge of time-ordered xs and ys cannot transport Derived's perm and stable_class proofs to it without re-deriving the merge step. Rule from a checked D0 at zero trust that, for time-ordered xs and ys, trace_merge xs ys is structurally the Derived sort of xs ++ ys under event_before, and state it with a seed that separates the sorted merge from the plain append"
status: ready
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Time follows Numerical in the L3 layer rotation (docs/program/06-catalog-campaign.md). SPEC-TIME-MERGE-SINGLETONS-SORT tied the merge of singletons to sort; this ties the binary merge of two ordered traces to it. Steward-filed per COORDINATION section 2."
---

# Merging ordered traces is sorting their append

## Objective

`Data.Time.EventTrace` exports a public checked law. For every `p`, `xs`
and `ys`, from `time_ordered p xs` and `time_ordered p ys`, it concludes
`Equal (List (Event p)) (trace_merge p xs ys) (sort (Event p)
(event_before p) (list_append (Event p) xs ys))`. It holds at zero trust,
and spec 69 states it by its public type.

## Settled inputs (main `f19404552`; anchors re-checked at release)

- **The operations.**
  - `trace_merge` (`EventTrace.ken.md:126`) is left-biased. Its step is
    `trace_merge_into` (`:484`), which takes `x` when `event_before x y`.
  - `sort` and `insert` (`Data/Collections/Derived.ken.md:1871`,
    `:1861`). `insert` places `x` before the first `h` with `le x h`.
  - `time_ordered` (`:117`) is `is_sorted (Event p) (event_before p)`.
- **The argument, for AC-0 to check.** By induction on `xs`, `sort (x ::
  xt ++ ys)` is `insert x (trace_merge xt ys)`. When `x` precedes the
  head of `xt` and `ys` is ordered, inserting `x` into that merge puts it
  where `trace_merge_into` would.
  - If `event_before x y`, the merge's head is at or after `x`.
  - Otherwise the merge's head is `y`, and the induction steps on `ys`.

  Ties go to `xs` on both sides, because `insert` places `x` ahead of an
  equal-time head.
- **Both premises are needed.** With `xs = [e5, e1]` at times 5 and 1
  and `ys = []`, the merge is `[e5, e1]`, but the sort is `[e1, e5]`.
  Swapping the two gives the same failure with `ys` unordered.
- **Neighbours.** `trace_merge_single_insert` (`:3828`) and
  `trace_merge_all_singletons_sort` (`:3847`) are §21. `trace_merge_ordered`
  (`:202`) gives sortedness only.
- **Spec and tests.** Spec 69 ends with §21 (`69-time-and-events.md:1749`),
  so the new section is §22. The seed directory is
  `conformance/stdlib/time-and-events/`. The test pattern is
  `crates/ken-elaborator/tests/cat_time_merge_singletons_sort.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 69 §22, the catalog law with any private helper, a seed, a package
test and the status lines. No §1 to §21 operation or law type moves, and
there is no kernel, surface or Derived change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-TIME-MERGE-SINGLETONS-SORT.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules:
  - the name, the binder order and the placement;
  - whether the proof reuses §21's insert equation;
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`.
- **AC-1.** §22 states the law by its public type. CV and the Architect
  review it, with a Decision.
- **AC-2.** The seed takes `xs = [e1, e3]` and `ys = [e2]`, at times 1, 3
  and 2. Its accepted block concludes, through the law, that the merge
  equals `[e1, e2, e3]`. Its refused block, on a fresh accepted prefix and
  in the same form, claims the merge equals `xs ++ ys`, which is `[e1, e3,
  e2]`. The two rows differ only in the claimed list.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The equation is false as stated, for example on equal-time events. Stop
  with the counterexample; do not weaken it to a permutation.
- The law needs an `Axiom`, a kernel, surface or Derived change, or a
  premise beyond the two orderings.
- AC-T finds `trusted_base()` changed as an identity set.

Checks: 4 fired. I read `trace_merge`, `trace_merge_into`, `insert`,
`sort` and `time_ordered` on `f19404552`. The argument above uses only
their definitions. 8 fired. The refused row claims the append, which
differs from the sorted merge only in the order, so a law that drops the
sort fails it. The `[e5, e1]` case shows why each ordering premise is
needed. 3 fired. §21's consumers are the package, its seed, its test,
spec 69 and SPEC-PROGRESS; AC-0 sweeps the same roots.
