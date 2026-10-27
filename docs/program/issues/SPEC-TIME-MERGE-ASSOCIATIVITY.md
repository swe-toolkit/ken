---
id: SPEC-TIME-MERGE-ASSOCIATIVITY
title: "Spec 69 defines a left-biased trace_merge and states its order, count, membership and empty units, but no law regroups three merges. Rule from a checked D0 at zero trust whether trace_merge is structurally associative and under which premises, and state it with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Time is next in the layer rotation after Numerical. Same shape as SPEC-NUMERICAL-INTERVAL-ASSOCIATIVITY-INSIDE. Steward-filed per COORDINATION section 2."
---

# Trace merge is associative

## Objective

Spec 69 states, as a public checked law over `Data.Time.EventTrace`,
that `trace_merge p (trace_merge p xs ys) zs` equals `trace_merge p xs
(trace_merge p ys zs)` as structural `Equal (List (Event p))`, under the
premises the D0 shows necessary.

## Settled inputs (spec 69 §1 to §5, main `edc1da359`)

- **The operation.** `trace_merge` compares current heads by
  `event_before` and takes the left head on a tie (§1.2). It does not
  sort its inputs and keeps each input's internal order.
- **No regrouping law.** §1.3 to §5 state ordering, count, window
  distribution, membership and the two empty units. No section regroups
  three merges.
- **Why structural equality can hold.** Unlike spec 68's interval laws,
  both sides are lists of the same events in an order fixed by head
  comparisons and left bias. No value is respelled, so the law can be
  `Equal`, not an inclusion. Whether it needs the `time_ordered`
  premises, as §3's window distribution does, is the D0's to measure.
- **Seeds.** Under `conformance/stdlib/time-and-events/`, next to
  `seed-window-composition.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §6 in spec 69 stating the ruled law by its public type, and a closed
conformance seed under `conformance/stdlib/time-and-events/`, with the
status lines changed in the same candidate.

**Amended 2026-10-27 (`evt_5xar42txx6wnr`).** The Architect's D0
(`evt_6mr5nvg6z9jat`) proves the law with no premise. The candidate also
carries the catalog addition: `trace_merge_assoc` and its seven private
helpers in `catalog/packages/Data/Time/EventTrace.ken.md`, in `ken fmt`
form, with that module's prose and law counts updated. No
`CAT-TIME-MERGE-ASSOCIATIVITY` node is framed. The Architect reviews the
exact tip for the catalog half. The seed keeps the a1/c0 tie at time 4,
which is the only thing that makes the refused block bite.

**Held 2026-10-27 (`evt_5teqh3b03e0gx`).** Approved `a4d1662df` conflicts
with the queued `CAT-TIME-WINDOW-COMPOSITION` (`27d8b4d70`) in both
EventTrace and spec 69. After that lands, merge main without rebasing,
re-derive the law counts from the merged module, and say that the proof
reuses the now-public `trace_merge_nil_right` (`evt_4etpas5xnb38k`). Then
take fresh votes and a fresh Decision.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged. It rules the
  name, the public type and which helpers stay private. Each premise it
  keeps comes with a closed counterexample to the law without it.
- **AC-1.** The new section states the ruled law by its public type.
  The Spec leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the law to three closed
  traces that interleave, with at least one equal-time pair across two
  inputs, so the tie rule is exercised. Each side is also computed
  independently by `Refl`. Its refused block is pinned to one false
  proposition on the same inputs. One example is merge with the right
  and left operands of the outer call swapped.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or a premise
  beyond the existing `time_ordered`.
- A change to an existing §1 to §5 law or operation.

Checks: 3 fired: grepped spec 69 and `EventTrace.ken.md` for `assoc`;
the only hit (`69-time-and-events.md:148`) is about `count`'s Nat laws,
not merge. 8 fired: AC-2's swapped-operand refusal differs from the law
only by the tie rule and head order, so a seed that the left bias does
not separate would accept it; the equal-time pair is what makes it
fail.
