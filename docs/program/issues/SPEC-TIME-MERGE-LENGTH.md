---
id: SPEC-TIME-MERGE-LENGTH
title: "Spec 69 gives trace_merge and trace_merge_all count laws keyed on a caller's equality, but no law says the merge keeps every event, so a client holding two traces cannot conclude the length of their merge without re-proving count at an always-true test. Rule from a checked D0 at zero trust that length (trace_merge p xs ys) equals add (length xs) (length ys), and that length (trace_merge_all p traces) equals length (list_concat traces), and state both with a seed that separates keeping duplicates from deduplicating"
status: merged
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Time follows Numerical in the L3 layer rotation (docs/program/06-catalog-campaign.md). The merge laws stop at count and membership, with no length law. Steward-filed per COORDINATION section 2."
---

# Merging keeps every event

## Objective

`Data.Time.EventTrace` exports two public checked laws. For every `p`,
`xs` and `ys`, `length (Event p) (trace_merge p xs ys)` equals
`add (length (Event p) xs) (length (Event p) ys)` as `Equal Nat`. For every
`traces`, `length (Event p) (trace_merge_all p traces)` equals
`length (Event p) (list_concat (Event p) traces)`. Both hold at zero trust
with no premise, including for traces that are not `time_ordered`. Spec 69
states them by their public types.

## Settled inputs (main `93ee7bd8e`)

- **The operations.** `trace_merge` (`EventTrace.ken.md:118`) recurses on
  `xs` through the private `trace_merge_into` (`:454`). `trace_merge_all`
  (`:124`) folds `trace_merge` over the list.
- **The existing count laws.** `trace_merge_count` (`:230`) and
  `trace_merge_all_count` (`:2545`) give the count of any `q` under a
  caller's `eqf`. Their private step lemmas are `trace_merge_into_count`
  (`:782`) and its `_cons` form (`:700`).
- **Why a client cannot get length today.** The bridge from count to
  length is `derived_count_true_length` (`Derived.ken.md:2204`), and it is
  private, as is its `derived_always_true` (`:2202`). No public law in
  `EventTrace`, `Derived` or `List` relates `length` to either merge. The
  only length law in spec 69 is §12's window bound
  (`trace_window_length_bound`, `:444`).
- **Spec and tests.** Spec 69 ends at §18
  (`spec/50-stdlib/69-time-and-events.md:1503`), so the new section is
  §19. The seed directory is `conformance/stdlib/time-and-events/`. The
  test pattern is `crates/ken-elaborator/tests/cat_time_merge_all_append.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 69 §19, the two catalog laws with any private helper, a seed, a
package test and the status lines. No §1 to §18 operation or law type
moves, no `Derived` export changes, and no kernel or surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-TIME-MERGE-ALL-APPEND.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules the names, the binder order and the route.
  The route is either direct induction mirroring `trace_merge_into_count`,
  or the count laws at an always-true test bridged by a package-local
  lemma. It also rules whether `trace_merge_all`'s law follows from the
  two-trace law and a `list_concat` length step.
- **AC-1.** §19 states both laws by their public types. The Spec Leader
  and CV review it, with a Decision.
- **AC-2.** The seed's accepted block applies both laws at closed traces:
  - two traces whose append is not `time_ordered`, so the premise-free
    reading is exercised;
  - two traces that each hold the same event, at the same time and with
    the same payload, where the merge has length 2;
  - `trace_merge_all` over three traces, one of them empty.

  Its refused block pins, in the same form as an accepted row and on a
  fresh accepted prefix:
  - the length of the merge of the two single-event traces holding the
    same event, claimed `Equal Nat` to 1. That claim holds only if merge
    drops duplicates, so it fails exactly when the law is read as a set
    union.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A law needs an `Axiom`, a kernel or surface change, or a premise.
- A change to an existing operation or law type in spec 69 §1 to §18, or
  to a `Derived` export.

Checks: 3 fired. I grepped `EventTrace.ken.md`, spec 69 and the Derived
and List packages for `length` laws over the merges. The only length law
is §12's window bound, and the count-to-length bridge is private at
`Derived.ken.md:2204`. 4 fired. I wrote the obligation in the delivered
vocabulary: `trace_merge_into_count` already threads one `add` per merged
head, and the same induction at `length` closes it. The one gap is the
private bridge, which is why AC-0 rules the route. 8 fired. The refused
row's two events are equal, so it fails exactly when the merge is read as
deduplicating, while the accepted duplicate row holds.
