---
id: SPEC-TIME-MERGE-ALL-APPEND
title: "Spec 69 §15-§17 give trace_merge_all ordering, membership, count and windowing laws, but none relates the merge of a concatenated list of traces to the merges of its parts, so a client that merges two batches of traces separately cannot conclude that merging the results equals merging all of them at once. Rule from a checked D0 at zero trust that trace_merge_all over list_append xs ys equals trace_merge of the two partial merges, and state it with a conformance seed"
status: active
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Time follows Numerical in the L3 layer rotation (docs/program/06-catalog-campaign.md, whose Time entry is time/events/traces). trace_merge_all has no law across a split of its input list. Steward-filed per COORDINATION section 2."
---

# Merging in batches equals merging at once

## Objective

`Data.Time.EventTrace` exports a public checked law that, for every `p`,
`xs` and `ys` of type `List (List (Event p))`,
`trace_merge_all p (list_append (List (Event p)) xs ys)` equals
`trace_merge p (trace_merge_all p xs) (trace_merge_all p ys)` as
`Equal (List (Event p))`. It holds at zero trust with no premise, and spec
69 states it by its public type.

## Settled inputs (main `4c07ec73f`)

- **The operation.** `trace_merge_all` (`EventTrace.ken.md:124`) is the
  right fold `Nil ↦ Nil; Cons t rest ↦ trace_merge p t (trace_merge_all p
  rest)`, and `trace_merge` (`:118`) computes `Nil ↦ ys` on its left
  operand.
- **The order tool.** Public `trace_merge_assoc` (`:1913`) is
  unconditional: `trace_merge (trace_merge xs ys) zs` equals
  `trace_merge xs (trace_merge ys zs)`.
- **The gap.** The public `trace_merge_all` laws are `_ordered` (`:2470`),
  `_member_complete` (`:2489`), `_member_sound` (`:2521`), `_count`
  (`:2545`) and `trace_window_merge_all` (`:2578`). None takes a
  `list_append` of trace lists.
- **The list form.** `list_append` (`Data/Collections/Derived.ken.md:102`)
  recurses on its left operand, and EventTrace imports it (`:71`), as
  `trace_window_append` (`:351`) uses it.
- **Spec and tests.** Spec 69 ends at §17 (`69-time-and-events.md:1439`),
  so the new section is §18. The test pattern is
  `crates/ken-elaborator/tests/cat_time_window_merge_all.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 69 §18, the catalog law with any private helper, a seed, a package test
and the status lines. No §1 to §17 operation or law type moves, and no kernel
or surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-TIME-WINDOW-MERGE-ALL.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules the name, the binder order and the proof route.
- **AC-1.** §18 states the ruled law by its public type. The Spec Leader and
  CV review it, with a Decision.
- **AC-2.** The seed's accepted block applies the law at closed arguments
  where the two batches interleave in time: `xs` holds one trace with events
  at times 1 and 3, and `ys` one trace with an event at time 2.

  Its refused block pins, in the same form as an accepted row and on a fresh
  accepted prefix:
  - the batch merge claimed equal to `list_append` of the two partial merges
    for that fixture.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise.
- A change to an existing §1 to §17 operation or law type.

Checks: 4 fired. By induction on `xs`: the `Nil` case is
`trace_merge_all ys` against `trace_merge Nil (trace_merge_all ys)`, which
computes to the same list. The `Cons t rest` case rewrites by the induction
hypothesis to `trace_merge t (trace_merge (trace_merge_all rest)
(trace_merge_all ys))`, which `sym` of `trace_merge_assoc` closes. 3 fired:
I grepped `catalog/packages`, `spec/`, `conformance/` and `crates/` for a
`trace_merge_all` law over an appended or concatenated trace list. The only
hit is `_count`, which compares against `list_concat` of the traces, not
against merges of two batches. 8 fired: on the fixture, the two partial merges
are `[1, 3]` and `[2]`. Their merge is `[1, 2, 3]` and their append is
`[1, 3, 2]`, so the refused row is false on the closed fixture. It differs
from the accepted row only in `list_append` for `trace_merge`.
