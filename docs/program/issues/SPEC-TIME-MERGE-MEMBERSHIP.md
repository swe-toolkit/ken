---
id: SPEC-TIME-MERGE-MEMBERSHIP
title: "Spec 69 states membership one window at a time and says a premise-free membership law for trace_merge was not established, so a client cannot conclude that merging keeps exactly the events of its inputs. Rule premise-free merge membership (sound and complete) from a checked D0 at zero trust, and state it as a section 4 with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: [CAT-TIME-MERGE-MEMBERSHIP]
github: null
origin: "Operator 2026-09-13: every catalog package fully proved. Spec 69 §3.2 says a premise-free window/merge membership law was not established by D0 and that section 2's laws do not state membership of a merge. Time/events is the next layer in the in-place rotation after numerical. Same shape as SPEC-TIME-WINDOW-MERGE-DISTRIBUTION. Steward-filed per COORDINATION section 2."
---

# Merging keeps exactly its inputs' events

## Objective

Spec 69 states checked laws that an event is a member of
`trace_merge p xs ys` exactly when it is a member of `xs` or of `ys`,
with no ordering premise.

## Settled inputs (spec 69 §1 to §3, on `main`)

- **Operations (§1.2).** `trace_merge` merges by current heads with no
  pre-sort, and a tie takes the left head. `trace_merge_count` (§1.3)
  already states that the merge's length is the sum of its inputs'
  lengths, for any inputs.
- **Membership.** `list_elem (Event p)` from `Data.Collections.Derived`
  is `‖ Or (Equal a x y) (list_elem a x rest) ‖` on a cons, so no
  decidable payload equality is needed. Derived also exports
  `list_elem_append_left` and `list_elem_append_right`. §2.3's
  `trace_window_member_sound` and `_complete` are the pattern.
- **The gap.** §3.2: a premise-free membership law "was not established
  by D0 and is not claimed here", and section 2's laws "do not by
  themselves state membership of a merge".
- **Package.** `catalog/packages/Data/Time/EventTrace.ken.md`. The
  seeds are under `conformance/stdlib/time-and-events/`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

A new spec 69 §4 stating each ruled law by its public type, the §3.2
"not established" text corrected, and a closed conformance seed under
`conformance/stdlib/time-and-events/`, with its status lines changed in
the same candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  rc 0 and no `Axiom`. It rules:
  - the law names and public types, for example
    `list_elem (Event p) e (trace_merge p xs ys) → ‖ Or (list_elem
    (Event p) e xs) (list_elem (Event p) e ys) ‖` and its converse,
    either as one pair or as left and right completeness;
  - whether window-of-merge membership follows as a corollary from §2.3,
    and whether that corollary is public;
  - the private helpers the proof needs, and which Derived lemmas it
    reuses.
- **AC-1.** The section states each ruled law by its public type, and
  the §3.2 text is corrected. The Spec leader and CV review it, with a
  Decision.
- **AC-2.** The seed checks membership on a closed pair of unordered
  traces with a cross-input tie, one event from each side. Its refused
  block claims an event of neither input is a member of the merge, with
  a same-input positive control.

## Stop conditions

- A law that needs decidable equality on the payload or an ordering
  premise, unless the D0 rules it.
- Any new primitive, kernel or surface feature.
- An ambient name in the package beyond its current census row
  (`lang_mod_strict_resolution_d0`): import it explicitly.

Checks: 4 fired: the membership proposition is written in the package's
vocabulary (`list_elem`, a truncated `Or` of `Equal`); no decidable
equality is assumed. 8 fired: AC-2's refused block uses an event of
neither input, which a merge that drops or invents events would not
refuse.
