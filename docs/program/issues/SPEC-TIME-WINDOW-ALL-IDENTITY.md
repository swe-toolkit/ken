---
id: SPEC-TIME-WINDOW-ALL-IDENTITY
title: "Spec 69 says which events a window keeps (§2 membership) and how windows compose, split and distribute, but not that a window covering every event of a trace returns the trace itself: membership cannot yield list equality, which needs order and duplicates. Rule from a checked D0 at zero trust that if every member of a trace lies in the window, the window is the trace, and state the law with a conformance seed"
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

# A window that keeps every event is the trace

## Objective

Spec 69 states, as a public checked law over `Data.Time.EventTrace`,
that for every `p`, `lo`, `hi` and trace `tr`, if every member `e` of
`tr` satisfies `event_in_window p lo hi e`, then
`trace_window p lo hi tr` equals `tr` as an `Equal (List (Event p))`.
No ordering premise is needed.

## Settled inputs (main `86b7cac5e`)

- **The operation.** `trace_window` is `filter` by `event_in_window`,
  which is `lo ≤ time ∧ time < hi` (`EventTrace.ken.md:94`, `:97`).
- **Not yet stated.** Spec 69 §1 to §9 and the package's 16 public
  laws give membership (`trace_window_member_sound`/`_complete`, `:111`,
  `:150`) but no list equality. Membership cannot produce one, since it
  sees neither order nor multiplicity.
- **The proof half exists.** `list_elem_head` and `list_elem_later`
  (`Data/Collections/Derived.ken.md:173`, `:176`) restrict a membership
  premise to the head and the tail. The `window_keep` rewrite pattern at
  `trace_window_inverted_empty` (`EventTrace.ken.md:293-314`) handles the
  head decision. The premise is a function over `list_elem`, so no
  list-wide predicate type is added.
- **Seeds.** Under `conformance/stdlib/time-and-events/`, next to the
  window seeds.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §10 in spec 69 stating the ruled law by its public type, the catalog
theorem in `EventTrace`, a closed conformance seed, and the status
lines, all in one candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set. It rules:
  - the name, argument order and premise form (`IsTrue` or
    `Equal Bool … True`, matching the package);
  - which helpers stay private.

  It also confirms that no §1 to §9 operation or law type changes.
- **AC-1.** §10 states the ruled law by its public type. The Spec Leader
  and CV review it, with a Decision.
- **AC-2.** The seed uses `tr = [(4,T),(6,F),(5,T)]` over `Bool`.
  - Its accepted client applies the law at `[4,8)`, with a closed
    premise discharge, and also checks `trace_window Bool 4 8 tr = tr`
    by `Refl`.
  - Its refused block pins `trace_window Bool 4 6 tr = tr` by `Refl`.
    The window computes to `[(4,T),(5,T)]`, because `6 < 6` is false.
    The accepted `[4,8)` row is the same-form control.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a premise beyond
  window membership of every member.
- A change to an existing §1 to §9 operation or law type.

Checks: 4 fired: on `86b7cac5e`, the head step is `list_elem_head` into
the premise, the tail step is `list_elem_later`, and the `window_keep`
rewrite follows `:293-314`. Each anchor was read at its line, and the
package was grepped for a window-identity law, with no hit. 8 fired: the
refused row and its control differ only in `hi`, and only the event at
time 6 separates them.
