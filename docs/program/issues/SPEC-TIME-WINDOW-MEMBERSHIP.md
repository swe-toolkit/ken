---
id: SPEC-TIME-WINDOW-MEMBERSHIP
title: "Spec 69 section 1 proves that trace_window and trace_merge preserve time order and that merge preserves counts, but states no window-membership law: nothing proves that the window keeps exactly the events of the trace whose time lies in [lo, hi). Rule from a checked D0 which window-membership soundness and completeness laws hold at zero trust, and state them as section 2 with a conformance seed"
status: active
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 69 section 1.3 excludes generic window-membership soundness and completeness from its checked claims. Time/events/traces is the roadmap layer after numerical (docs/program/06-catalog-campaign.md); non-Lane-B layers are deepened in place. Same shape as SPEC-NUMERICAL-INTERVAL-HULL. Steward-filed per COORDINATION section 2."
---

# The window keeps exactly the in-range events

## Objective

Spec 69 §2 states, as checked laws, that `trace_window p lo hi tr`
contains an event exactly when the event occurs in `tr` and its time
satisfies `lo ≤ t < hi`.

## Settled inputs (spec 69 §1, on `main`)

- **Operations (§1.2).** `event_in_window p lo hi e` is
  `bool_and (leq_nat lo (event_time p e)) (lt_nat (event_time p e) hi)`.
  `trace_window` is `filter (Event p) (event_in_window p lo hi) tr`. It
  keeps each selected event unchanged and in relative order, for any
  input, and returns the empty list when `lo ≥ hi`.
- **The gap (§1.3).** "No generic window-membership soundness or
  completeness law ... is included among the checked claims."
- **No assumed equality on payloads (§1.3).** The count law takes an
  arbitrary Boolean classifier rather than decidable equality on `p`.
  A membership law must choose its membership notion with the same care.
- **Deferred and still out of scope (§1.4).** Wall-clock time, durations,
  causality, streams, deduplication, k-way merge, and time over an
  arbitrary `Ord`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

Spec 69 §2: the window-membership laws, with the membership notion they
use, and a closed conformance seed under `conformance/stdlib/`.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed §1 base,
  with rc 0 and no `Axiom`. It rules:
  - the membership notion: an existing `Membership` provider, a list
    `Elem`-style inductive, or a classifier-count form like §1.3's;
  - soundness: a member of the window is a member of `tr`, with its
    time in range;
  - completeness: an in-range member of `tr` is a member of the window;
  - any catalog prerequisite that must be exported.
- **AC-1.** §2 states each ruled law by its public type, with the
  membership notion's definition. The Spec leader and CV review it, with
  a Decision.
- **AC-2.** The seed instantiates each law on closed traces. It includes
  the boundary events at `lo` (kept) and at `hi` (dropped), and a false
  membership block that is refused.

## Stop conditions

- A law that needs decidable equality on every payload type, unless the
  D0 rules that as the stated premise.
- Any new primitive, kernel or surface feature.
