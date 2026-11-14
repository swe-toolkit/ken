---
id: SPEC-TIME-TRACE-MERGE-ALL-COUNT
title: "Spec 69 §15 gives the k-way trace merge ordered, complete and sound membership laws but no count law, so an operation that drops or duplicates repeated events still satisfies all three. Rule from a checked D0 at zero trust that trace_merge_all preserves every classifier's count of the flattened input, with the count-over-append list law it needs, and state them with a conformance seed"
status: active
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Time follows Numerical in the L3 layer rotation (docs/program/06-catalog-campaign.md). Steward-filed per COORDINATION section 2."
---

# The k-way merge keeps every event

## Objective

`Data.Time.EventTrace` exports a law that `trace_merge_all` preserves,
for any classifier `eqf` and query `q`, the `count` of the flattened
input `list_concat` of its traces. It holds at zero trust, and spec 69
states it by its public type, with any list law it needs.

## Settled inputs (main `750f1622b`)

- **The gap.** Spec 69 §15 states only `trace_merge_all_ordered`,
  `trace_merge_all_member_complete` and `trace_merge_all_member_sound`
  (`catalog/packages/Data/Time/EventTrace.ken.md:2468`, `:2487`,
  `:2519`). Membership is truncated `list_elem`, so an operation that
  keeps one copy of a repeated event, or adds one, satisfies all three.
- **The two-way law exists.** `trace_merge_count` (`EventTrace.ken.md:228`)
  states `count q (trace_merge p xs ys) = add (count q xs) (count q ys)`
  for any `eqf`, with no ordering premise.
- **The operations.** `trace_merge_all` is the right fold
  `trace_merge p t (trace_merge_all p rest)` (`:122`). `list_concat` is
  the right fold of `list_append` (`Data/Collections/Derived.ken.md:108`),
  and `count` is at `Derived.ken.md:1793`.
- **The enabler is absent.** No catalog law states that `count`
  distributes over `list_append`: Derived's public count laws are
  `count_cons`, `count_left_first` and `count_right_first`
  (`Derived.ken.md:3339`, `:3361`, `:3415`). The `Cons` step of the
  induction needs `count q (list_append t u) = add (count q t)
  (count q u)`.
- **Seeds and tests.** The seed sits under
  `conformance/stdlib/time-and-events/`, beside `seed-trace-merge-all.md`.
  The test pattern is
  `crates/ken-elaborator/tests/cat_time_trace_merge_all.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

A new spec 69 section, the catalog laws, a seed and the status lines.
No §1 to §15 operation or law type moves, and no kernel or surface
change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-NUMERICAL-INTERVAL-WIDTH-INSIDE-MONO.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules the law set and names, starting from
  `Equal Nat (count (Event p) eqf q (trace_merge_all p traces))
  (count (Event p) eqf q (list_concat (Event p) traces))`. It rules
  whether the count-over-append law is a public
  `Data.Collections.Derived` export or a private EventTrace helper, and
  names its owner.
- **AC-1.** The new section states each ruled law by its public type.
  The Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted block applies the law to a closed
  three-trace input in which one event occurs in two input traces, and
  pins the merged count of that event at 2. Its refused block pins, in
  the same form as an accepted row, the claim that the count is 1, on a
  fresh accepted prefix.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise
  such as ordered inputs.
- A change to an existing §1 to §15 operation or law type.

Checks: 4 fired: the obligation's `Cons` step was written out; it
needs `trace_merge_count` (present, `:228`) and a count-over-append law,
which a grep of `catalog/packages` for `count` over `list_append` does
not find. 3 fired: the grep covered all of `catalog/packages`; the only
count laws are Derived's three public `count_*` laws and its private
helpers. 8 fired: a merge that keeps one copy of a repeated event
satisfies §15's ordered, complete and sound laws but refuses the AC-2
row at count 2; the refused twin at count 1 is accepted by that mutant
and refused by the real operation.
