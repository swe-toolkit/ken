---
id: CAT-TIME-WINDOW-MEMBERSHIP
title: "Spec 69 section 2 specifies two window-membership laws over trace_window, and Data.Collections.Derived carries their two generic filter prerequisites, but Data.Time.EventTrace does not export the laws. Add trace_window_member_sound and trace_window_member_complete at zero trust from the Architect's checked D0, and run the section 2 seed as an acceptance target"
status: ready
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-TIME-WINDOW-MEMBERSHIP]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 69 section 2 from SPEC-TIME-WINDOW-MEMBERSHIP (Architect D0 on a0e9a1d30, Decision dec_40wt3qx1wkzxj). Same shape as CAT-NUMERIC-INTERVAL-HULL. Steward-filed per COORDINATION section 2."
---

# The window keeps exactly the in-range events, delivered and proved

## Objective

`Data.Time.EventTrace` exports `trace_window_member_sound` and
`trace_window_member_complete` as proved theorems, and the spec 69 §2
seed checks against the shipped package.

## Settled inputs (spec 69 §2, SPEC-TIME-WINDOW-MEMBERSHIP)

- **Membership (§2.1).** `list_elem (Event p)` from
  `Data.Collections.Derived`. No `DecEq p`, `Ord p` or Boolean payload
  comparison.
- **Prerequisites (§2.2).** `list_elem_filter_sound` and
  `list_elem_filter_complete` are public, proved theorems in
  `Data.Collections.Derived`. The spec WP delivers them, not this node.
- **Laws (§2.3).** The two public types are as stated in §2.3, with
  the ambient Ω-valued `And`, not `Core.Logic.And`, and no ordering
  premise. The package imports `list_elem` and the two filter theorems
  from Derived and `and_true` from `Core.Classes.LawfulClasses`. It adds
  no other public law.
- **Checked in scratch (Architect D0, on `a0e9a1d30`).** The four
  proofs checked rc 0 with no new `Axiom`.
- **Seed.** `conformance/stdlib/time-and-events/seed-window-membership.md`:
  boundary fixtures, a soundness and a completeness instance, and one
  refused block, `reject-upper-bound-membership`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The two laws in `catalog/packages/Data/Time/EventTrace.ken.md`, with the
§2 seed run from an acceptance test beside
`crates/ken-elaborator/tests/cat_time_event_trace.rs`.

## Acceptance

- **AC-1.** EventTrace checks with both laws as public, proved theorems.
  `trusted_base()` identity count is unchanged.
- **AC-2.** The seed's positive cases check against the shipped package,
  and `reject-upper-bound-membership` is refused at its false `lt_nat`
  premise.
- **AC-3 (mutation, QA).** Changing `event_in_window`'s `lt_nat` to a
  non-strict comparison makes the seed's boundary row or a law fail.
- The §1 EventTrace suite stays green.

## Stop conditions

- Any `Axiom`, admitted body, primitive, or kernel or surface change.
- A §2.3 law that needs a payload equality decision or an ordering
  premise.
