---
id: SPEC-TIME-WINDOW-COMPOSITION
title: "Spec 69 states no law for a window of a window, and gives merging with an empty trace only as definition prose, not as a public law. Rule window composition and the merge unit laws from a checked D0 at zero trust, and state them as a section 5 with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Time is next in the layer rotation after Numeric. Same shape as SPEC-NUMERICAL-INTERVAL-INSIDE-ORDER. Steward-filed per COORDINATION section 2."
---

# Time windows compose, and merge has an empty unit

## Objective

Spec 69 states, as public checked laws over `Data.Time.EventTrace`,
that windowing a windowed trace equals one window over the
intersected bounds. It also states that `trace_merge` with an empty
trace on either side returns the other trace.

## Settled inputs (spec 69 §1 to §4, main `98970bde7`)

- **The window.** `trace_window p lo hi` is `filter` by
  `event_in_window`, which is `bool_and (leq_nat lo t) (lt_nat t hi)`
  (§1.2). It keeps selected events unchanged and in order, on any
  input. No §1 to §4 law relates two windows.
- **Why composition can be unconditional.** Both sides are filters of
  the same list. The candidate bound is `max lo1 lo2` and `min hi1
  hi2`, which `Data.Numeric.Nat.Order` exports (`Order.ken.md:58`,
  `:106`). No ordering premise is needed.
- **The unit.** §1.2 says "Empty input returns the other input
  unchanged", but only as definition prose. The package proves
  `trace_merge_nil_right` privately (`EventTrace.ken.md:952`).
- **Seeds.** Under `conformance/stdlib/time-and-events/`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §5 in spec 69 stating each ruled law by its public type, and a closed
conformance seed under `conformance/stdlib/time-and-events/`, with the
status lines changed in the same candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged. It rules the
  names, argument order and public types, whether composition is
  stated as one equality over `max` and `min` or as commutation and
  idempotence, and which helpers stay private.
- **AC-1.** The §5 states each ruled law by its public type. The Spec
  leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies each law. Its refused
  block is pinned to one false proposition: windows composed at the
  union bounds (`min lo1 lo2`, `max hi1 hi2`) on a trace with an event
  inside only one window.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or an ordering
  premise.
- A change to an existing §1 to §4 law or operation.
