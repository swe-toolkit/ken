---
id: SPEC-TIME-INVERTED-WINDOW-EMPTY
title: "Spec 69 §1.2 says a window with lo ≥ hi returns the empty list, and §5, §8 and the package's design notes lean on that, but no law checks it: Data.Time.EventTrace has no theorem that an equal or inverted window selects nothing. Rule from a checked D0 at zero trust a public law that trace_window p lo hi tr is Nil whenever hi ≤ lo, for every trace, and state it with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Time is next in the layer rotation after Numerical. Steward-filed per COORDINATION section 2."
---

# An inverted window is empty

## Objective

Spec 69 states, as a public checked law over `Data.Time.EventTrace`,
that for every payload type `p`, bounds `lo` and `hi` with
`Equal Bool (leq_nat hi lo) True`, and trace `tr`, including unordered
traces, `trace_window p lo hi tr` is `Nil (Event p)`.

## Settled inputs (spec 69 §1 to §8, main `9ff8bc0f6`)

- **The operation.** `event_in_window` is
  `bool_and (leq_nat lo t) (lt_nat t hi)`, and `trace_window` filters
  by it (`EventTrace.ken.md:92`, `:95`).
- **Claimed but unchecked.** §1.2 says "When `lo ≥ hi`, it returns the
  empty list" (`69-time-and-events.md:112`). §5 (:531) and §8 (:787)
  reason from that sentence, and the package design notes repeat it
  (`EventTrace.ken.md:3029`). No theorem states it.
- **The proof half exists, privately.** An event at time `t` with
  `lo ≤ t` has `hi ≤ t` by `proof trans for leq_nat`, so the private
  `event_in_window_past` (`:2833`) makes its decision `False`. The
  private `window_keep` (`:2385`) then drops it. §7's private
  `trace_window_past` (`:2718`) is the ordered-trace analogue; this law
  needs no ordering premise.
- **Premise form.** §7 states bound premises as
  `Equal Bool (leq_nat _ _) True` (`trace_window_split`, `:245`).
- **Seeds.** Under `conformance/stdlib/time-and-events/`, next to
  `seed-window-composition.md`, whose `tr = [e1, e4, e7]` can be
  reused.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §9 in spec 69 stating the ruled law by its public type, the catalog
theorem in `EventTrace`, a closed conformance seed under
`conformance/stdlib/time-and-events/`, and the status lines, all in one
candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules:
  - the name, argument order, premise form and placement;
  - which helpers stay private, and whether `event_in_window_past` is
    reused.

  It also confirms that no §1 to §8 operation or law type changes.
- **AC-1.** §9 states the ruled law by its public type. The Spec Leader
  and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the law to `tr` at the
  equal bounds `[4,4)` and the inverted bounds `[7,4)`.
  - Its refused block pins `trace_window Bool t4 t5 tr` equal to `Nil`.
  - `[4,5)` keeps `e4`, so the claim is false, and the law cannot
    discharge it because `leq_nat 5 4` is `False`.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, an ordering
  premise, or any premise beyond `hi ≤ lo`.
- A change to an existing §1 to §8 operation or law type.

Checks: 4 fired: `event_in_window`, `trace_window`,
`event_in_window_past`, `window_keep` and `trace_window_past` were read
at their lines on `9ff8bc0f6`, and the §1.2 sentence at :112. The
package was grepped for an empty-window theorem; the only hits are prose
at :3019 and :3029. 8 fired: the refused row widens the equal bounds by
one step. `[4,5)` keeps `e4`, so it is false while both accepted rows
are true, and the boundary is the discriminating axis.
