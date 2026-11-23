---
id: SPEC-TIME-MERGE-SINGLETONS-SORT
title: "Spec 69 states trace_merge's ordering, count, membership and argument-order laws, and Data.Collections.Derived proves insertion sort ordered, a permutation and stable, but no law connects the two, so a client that merges single-event traces cannot use the sort laws for the result. Rule from a checked D0 at zero trust that merging a single event into a trace is insert by event_before, and that trace_merge_all over single-event traces is sort by event_before, and state it with a seed that separates the tie order"
status: merged
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Time follows Numerical in the L3 layer rotation (docs/program/06-catalog-campaign.md, time/events/traces). The merge laws restate sortedness and count facts that Derived already proves for sort, with no bridge between them. Steward-filed per COORDINATION section 2."
---

# Merging single events is insertion sort

## Objective

`Data.Time.EventTrace` exports public checked laws. For every `p`, `x`
and `ys`, `trace_merge p (Cons x Nil) ys` equals `insert (Event p)
(event_before p) x ys`. For every `p` and `tr`, `trace_merge_all p` over
the single-event traces of `tr` equals `sort (Event p) (event_before p)
tr`. They hold at zero trust, and spec 69 states them by their public
types. With them, Derived's `sorted`, `perm` and `stable_class` laws for
sort apply to a merge of single events.

## Settled inputs (main `7e8e1cc38`; anchors re-checked at release)

- **The merge.** `trace_merge` (`EventTrace.ken.md:121`) is `ys` on
  `Nil`, and on `Cons x xt` is `trace_merge_into p x xt (trace_merge p xt)
  ys`. `trace_merge_into` (`:476`) keeps `x` first when `event_before p x
  y` is True, and otherwise emits `y` and recurses on `yt`. With `xt =
  Nil`, its `rest` is `trace_merge p Nil`, which returns its argument.
- **The sort.** `insert` (`Derived.ken.md:1861`) keeps `x` first when
  `le x h` is True, and otherwise emits `h` and recurses. `sort` is
  `insert h (sort t)`. So both functions break a tie the same way, with
  the incoming event first.
- **The sort laws.** Derived exports `sorted for insert` (`:1877`),
  `sorted for sort` (`:1899`), `perm for sort` (`:1929`) and
  `stable_class for insert` (`:2020`). EventTrace imports neither
  `insert` nor `sort` today (its import list is `:70`).
- **trace_merge_all** (`:127`) is `trace_merge p t (trace_merge_all p
  rest)`, so over single-event traces it folds the same way `sort` does.
- **Spec and tests.** Spec 69 ends with §20, merge commutes across
  distinct timestamps (`69-time-and-events.md:1680`), so the new section
  is §21. The seed directory is `conformance/stdlib/time-and-events/`.
  The test pattern is
  `crates/ken-elaborator/tests/cat_time_merge_distinct_commute.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 69 §21, the two catalog laws with any private helper, the two
Derived imports, a seed, a package test and the status lines. No §1 to
§20 operation or law type moves, and no kernel or surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-TIME-MERGE-DISTINCT-TIMES-COMMUTE.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules:
  - both names and binder orders;
  - how the single-event traces of `tr` are written, as `map` with a
    singleton function or as a private helper, by what a client can
    state today;
  - whether the sort law is exported, or left as a client composition of
    the insert law. The ruling says which Derived sort law a client
    reaches in one step from each choice.
- **AC-1.** §21 states the laws by their public types. The Spec Leader
  and CV review them, with a Decision.
- **AC-2.** The seed's accepted block merges two single-event traces
  with the same `event_time` and different payloads. It concludes the
  equation with `insert`, which places the first trace's event first.
  Its refused block, on a fresh accepted prefix, claims the equation
  with the two events swapped, and refuses. So the law fixes the tie
  order, not only the multiset.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A law needs an `Axiom`, a kernel or surface change, or a premise such
  as `time_ordered ys`.
- AC-T finds `trusted_base()` changed as an identity set.
- A change to an existing §1 to §20 operation or law type, or to
  `insert` or `sort` in Derived.

Checks: 3 fired. I listed every public law in `EventTrace` and every
public `insert` and `sort` proof in Derived on `7e8e1cc38`. No EventTrace
law names `insert` or `sort`, and EventTrace imports neither. 4 fired. I
wrote the obligation in the delivered vocabulary: a case on `ys`, the
`event_before p x y` match against `insert`'s `le x h` match, and `rest`
reducing to the identity when `xt` is `Nil`. 8 fired. A law that
equated the merge only up to permutation would accept both tie orders,
so the refused swapped row separates the equation from `perm for sort`.
