---
id: SPEC-TIME-WINDOW-COMMUTATION
title: "Spec 69 §5 fuses two windows into one at max of the lower bounds and min of the upper bounds, and says outright that this is not a commutation law. No law states that two windows commute: applying [lo1, hi1) then [lo2, hi2) selects the same list, in order, as applying them in the other order. A client must apply trace_window_window twice and rewrite the bounds with Nat.Order's comm for max and comm for min. Rule from a checked D0 at zero trust and state the law with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Time follows Numerical in the layer rotation. Steward-filed per COORDINATION section 2."
---

# Two time windows commute

## Objective

Spec 69 states, as a public checked law over `Data.Time.EventTrace`,
for all `p`, bounds `lo1`, `hi1`, `lo2`, `hi2` and trace `tr`:

```ken
Equal (List (Event p))
  (trace_window p lo2 hi2 (trace_window p lo1 hi1 tr))
  (trace_window p lo1 hi1 (trace_window p lo2 hi2 tr))
```

## Settled inputs (main `70dc30f97`)

- **The neighbours exist.** `trace_window` filters by
  `event_in_window` (`EventTrace.ken.md:97`, `:100`).
  `trace_window_window` (`:236`) fuses two windows into
  `trace_window p (max lo1 lo2) (min hi1 hi2) tr`. `comm for min` and
  `comm for max` are public in `Data.Numeric.Nat.Order` (`:106`,
  `:234`), and EventTrace already imports `max` and `min` (`:53`).
- **Not yet stated.** §5 (`69-time-and-events.md:538` to `:540`) says
  the fusion law "is not the weaker pair of a commutation law and an
  idempotence law". The package and spec 69 were grepped for a law
  concluding the swapped composition, with no hit.
- **Seeds.** Under `conformance/stdlib/time-and-events/`, next to
  `seed-window-composition.md`, whose `t0` to `t9` and `e1`, `e4`
  fixtures can be reused.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §11 in spec 69 stating the ruled law by its public type, the catalog
theorem in `EventTrace`, a closed conformance seed under
`conformance/stdlib/time-and-events/`, the public-inventory pin
advanced, and the status lines, all in one candidate. The status lines
describe the assembled candidate, not the spec-only child.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules the theorem name, argument order and placement, and
  whether the proof composes `trace_window_window` with the public
  `comm` laws or needs a private helper. It confirms that no §1 to §10
  operation or law type changes, and amends §5's sentence if the new
  law makes it misleading.
- **AC-1.** §11 states the ruled law by its public type. The Spec
  Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the law to two
  overlapping windows that are not nested, over a closed trace with an
  event inside only one of them.
  - Its refused block pins the outer window over the inner window as
    equal to the outer window alone. The trace's event outside the
    inner window makes that claim false.
  - Its accepted same-form control is the swapped composition, so the
    refusal is the dropped inner window, not the form.
  - The D0 rules the proof form that separates the two rows. §10's seed
    used `Refl` for closed list equalities, and the D0 confirms it here.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or any premise.
- A change to an existing §1 to §10 operation or law type.

Checks: 4 fired: the law is written from public `trace_window`,
`trace_window_window` and Nat.Order's `comm for max` and `comm for
min`, each read at its line; the package and spec 69 were grepped for
a swapped-composition law, with no hit. 8 fired: the refused row and
its control share the trace and the claim form and differ only in the
right-hand side, which keeps an event outside the inner window for the
refusal and drops it for the control.
