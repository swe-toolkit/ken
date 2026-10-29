---
id: SPEC-TIME-WINDOW-APPEND-DISTRIBUTION
title: "Spec 69 splits one window at an interior bound, but no law relates a window to a concatenated trace. Rule from a checked D0 at zero trust that windowing the append of two traces equals the append of their windows, for every bound pair and every input order, and state the law with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Time is next in the layer rotation after Numerical. Same shape as SPEC-TIME-WINDOW-ADJACENT-SPLIT. Steward-filed per COORDINATION section 2."
---

# A window distributes over trace concatenation

## Objective

Spec 69 states, as a public checked law over `Data.Time.EventTrace`, that
for every `lo`, `hi`, `xs` and `ys`, `trace_window p lo hi
(list_append (Event p) xs ys)` equals `list_append (Event p)
(trace_window p lo hi xs) (trace_window p lo hi ys)` as structural
`Equal (List (Event p))`, with no premise.

## Settled inputs (spec 69 §1 to §7, main `dabef61d8`)

- **The operations.** `trace_window` is `filter` by `event_in_window`
  (`EventTrace.ken.md:95`). `list_append` recurses on its first list
  (`Derived.ken.md:99`) and is already in EventTrace's named import list
  (`:55`).
- **No append law.** §7 splits one ordered trace at an interior bound.
  No section windows a concatenation, and no catalog module states
  filter over append: `git grep 'theorem .*append' -- catalog` has no
  filter hit.
- **Why no premise is expected.** Filtering inspects each event alone,
  so input order, inverted bounds and repeated times do not matter. The
  proof is expected to go by induction on `xs`. The `Nil` case is
  `Refl`. The `Cons` case binds the head's window decision and can reuse
  the private `window_keep` (`:2377`) and `window_keep_append` (`:2895`)
  with the induction hypothesis.
- **Seeds.** Under `conformance/stdlib/time-and-events/`, next to
  `seed-window-adjacent-split.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

An §8 in spec 69 stating the ruled law by its public type, a closed
conformance seed under `conformance/stdlib/time-and-events/`, and the
status lines, all in one candidate. The catalog half travels in the
candidate when AC-0 rules it. Otherwise it is a separate Foundation WP
built from the D0.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules:
  - the name, the argument order, and where it sits among the exports;
  - whether the generic filter-over-append step stays private to
    EventTrace or is exported from `Data.Collections.Derived`;
  - which helpers stay private.

  It also confirms that no §1 to §7 operation or law type changes.
- **AC-1.** §8 states the law by its public type. The Spec Leader and CV
  review it, with a Decision.
- **AC-2.** The seed's accepted client uses two closed traces, one of
  them not time-ordered, and a window that keeps at least one event
  from each and drops at least one event from each.
  - It applies the law and checks each window independently by `Refl`.
  - Its refused block pins the false claim with the two windows swapped
    on the right. For `xs = [e5, e1]`, `ys = [e3, e8]` and window
    `[2, 6)`, that is `[e3, e5]` against `[e5, e3]`. The block also
    carries an accepted same-shape control in the unswapped order and an
    independent `False` value from a first-event observer.

## Stop conditions

- The law needs a premise, an Axiom, or a kernel or surface change.
- A change to an existing §1 to §7 operation or law type.

Checks: 4 fired: the `Cons` obligation was written against the landed
`filter` and `window_keep` bodies on `dabef61d8`. Both match one Boolean
on the head, so `window_keep_append` applies with the induction
hypothesis as its split argument. 11 fired: the frame names a bound head
decision, not a match on `event_in_window p lo hi h` as computed. 8 fired:
the refused row has the accepted law's shape with only the windows
swapped, and for the seed's traces its first event is `e3`, not `e5`, so
a kernel that accepted it would show the law was not checked.
