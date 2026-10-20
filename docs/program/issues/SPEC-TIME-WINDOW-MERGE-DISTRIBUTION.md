---
id: SPEC-TIME-WINDOW-MERGE-DISTRIBUTION
title: "Spec 69 defers how windowing interacts with merging: nothing states that the window of a merged trace is the merge of the windowed inputs, so a client that windows two ordered traces cannot exchange the two steps. Rule the distribution law, its ordering premise, and its form (structural list equality) from a checked D0 at zero trust, and state it as a section 3 with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 69 sections 1.3, 1.4 and 2.3 list window/merge distribution as deferred. Time/events is the roadmap layer after numerical (docs/program/06-catalog-campaign.md); non-Lane-B layers are deepened in place. Same shape as SPEC-NUMERICAL-INCLUSION-ISOTONICITY. Steward-filed per COORDINATION section 2."
---

# Windowing distributes over merging

## Objective

Spec 69 states a checked law that `trace_window` of a `trace_merge` is
the `trace_merge` of the two windowed inputs, with the ordering premise
under which it holds.

## Settled inputs (spec 69 §1 and §2, on `main`)

- **Operations (§1.2).** `trace_window p lo hi` is `filter` by
  `event_in_window`, the half-open `lo ≤ t < hi`, and keeps input
  order. `trace_merge` merges by current heads with no pre-sort, and a
  tie takes the left head.
- **Laws today.** §1.3's `trace_window_ordered`,
  `trace_merge_ordered` and `trace_merge_count`, and §2.3's two window
  membership laws. §1.3 includes no structural list equality of merged
  traces, and §1.4 and §2.3 defer window/merge distribution.
- **The premise is needed.** On unordered input the steps do not
  commute. With times `xs = [5, 1]`, `ys = [3]` and window `[0, 4)`,
  windowing the merge gives `[3, 1]`, while merging the windows gives
  `[1, 3]`.
- **Package.** `catalog/packages/Data/Time/EventTrace.ken.md`. The
  seeds are under `conformance/stdlib/time-and-events/`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

A new spec 69 section stating each ruled law by its public type, the
§1.3, §1.4 and §2.3 deferral text corrected, and a closed conformance
seed under `conformance/stdlib/time-and-events/`, with its status lines
changed in the same candidate (Steward `evt_5rtkgkdf42qb`).

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0 and no `Axiom`. It rules:
  - the distribution law's name, its statement as `Equal (List (Event
    p))`, and its premise, which is at least `time_ordered` on both
    inputs. The D0 states whether one ordered input suffices;
  - whether a premise-free form holds for membership or `count`, and
    whether it is a public law or follows from §1.3 and §2.3;
  - the private helpers the proof needs, and which existing ones it
    reuses.
- **AC-1.** The section states each ruled law by its public type, and
  the deferral text is corrected. The Spec leader and CV review it,
  with a Decision.
- **AC-2.** The seed checks distribution on a closed pair of ordered
  traces whose window drops events from both inputs and keeps a
  cross-input tie. It has a refused block that claims distribution on
  the unordered pair above.

## Stop conditions

- A law that needs decidable equality on the payload, a total order
  other than `leq_nat`, or a sorted carrier type, unless the D0 rules
  it as the stated premise.
- Any new primitive, kernel or surface feature.
- An ambient name in the package beyond its current census row
  (`lang_mod_strict_resolution_d0`): import it explicitly.
