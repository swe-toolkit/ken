---
id: SPEC-TIME-WINDOW-SORT-COMMUTE
title: "Spec 69 ties merging to sorting (§21 to §23) and proves that a window distributes over merge and append, but no law says that windowing commutes with sorting: a client that sorts an unordered trace and then selects a time window cannot rewrite the result to the sort of the window without unfolding insertion sort. Rule from a checked D0 at zero trust that trace_window over sort by event_before equals sort of trace_window, and state it with a seed whose accepted row is open in the trace"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Time follows Numerical in the L3 layer rotation (docs/program/06-catalog-campaign.md). Window laws cover merge, append and merge_all; none covers sort, which §21 to §23 made the merge's normal form. Steward-filed per COORDINATION section 2."
---

# A window of a sorted trace is the sort of the window

## Objective

`Data.Time.EventTrace` exports a public checked law. For every `p`, `lo`,
`hi` and `tr` it states

`Equal (List (Event p)) (trace_window p lo hi (sort (Event p) (event_before p)
tr)) (sort (Event p) (event_before p) (trace_window p lo hi tr))`.

It holds at zero trust, and spec 69 states it by its public type.

## Settled inputs (main `0ebc86105`; anchors re-checked at release)

- **The operations.**
  - `trace_window` (`EventTrace.ken.md:125`) is `filter` by
    `event_in_window`.
  - `event_before` (`:116`) is `leq_nat` on event times, so it is total and
    transitive.
  - Derived's `sort` (`Derived.ken.md:1935`) is insertion sort.
    `insert x (Cons h t)` (`:1925`) puts `x` first when `le x h`, and
    otherwise recurses past `h`.
- **The argument, for AC-0 to check.**
  - Induct on `tr`. The step reduces to
    `filter q (insert x s) = if q x then insert x (filter q s) else filter q s`
    for a sorted `s`.
  - This needs `s` sorted and `le` transitive: when `le x h` holds and `h` is
    filtered out, `x` must still precede the next kept element.
  - `sorted for insert` and the sortedness of `sort`'s output exist in
    Derived. `filter_sorted` (`:4836`) shows that filtering keeps order.
- **No such law exists.**
  - EventTrace has 43 public functions and theorems. Its window laws cover
    window, commute, split, append, merge, merge_all, and the inverted, all
    and none windows.
  - Derived has no law that relates `filter` to `insert` or `sort`.
- **Spec and tests.**
  - Spec 69 ends with §23 (`69-time-and-events.md:1857`), so the new section
    is §24.
  - The seed directory is `conformance/stdlib/time-and-events/`.
  - The test pattern is `crates/ken-elaborator/tests/cat_time_window_split.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 69 §24, the catalog law with any private helper, a seed, the package
test rows and the status lines. No §1 to §23 operation or law type moves,
and there is no kernel or surface change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-TIME-MERGE-ALL-ORDERED-SORT-CONCAT.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules:
  - the name, the binder order and the placement;
  - whether the generic filter-insert step stays private in EventTrace;
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`, including any exact
    export roster and count-bearing sentence.
- **AC-1.** §24 states the law by its public type. CV and the Architect
  review it, with a Decision.
- **AC-2.** The seed's accepted block must not close by computation alone.
  - Its accepted row is open in the trace: for every `tr`, it applies the law
    to a fixed window. AC-0 shows that the row's closing term needs the law.
  - Its refused twin, on a fresh accepted prefix and in the same form, claims
    that the window of the sorted trace is the window of `tr` unsorted. That
    claim must refuse.
  - A closed usage row, such as times `[3, 1, 5]` in the window `[1, 4)`
    giving `[1, 3]`, is allowed only as an example, not as the witness.
  - AC-0 picks the closing terms and may strengthen the seed.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise.
- The proof needs a new public export from Derived; report it rather than
  widening another package.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §23 operation or law type.

Checks: 4 fired. I read `trace_window`, `event_before`, `insert`, `sort`,
`filter` and `filter_sorted` on `0ebc86105`; the step needs sortedness and
transitivity, both available. 3 fired. I listed EventTrace's 43 public names
and Derived's sort and filter laws, and found no filter-sort relation.
8 fired. A closed seed row computes without the law (ADD-POINT-EXACT AC-0,
`evt_1b1n2fsh3bvws`), so the witness row is open in the trace, and the
refused twin drops the sort.
