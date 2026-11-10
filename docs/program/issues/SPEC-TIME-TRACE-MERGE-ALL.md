---
id: SPEC-TIME-TRACE-MERGE-ALL
title: "Spec 69 defers k-way merge (69-time-and-events.md:229), so a client holding several time-ordered event traces, one per source, can merge only two at a time and has no checked statement that the combined trace is ordered or keeps every input event. Add trace_merge_all over a list of traces, folding the existing two-way trace_merge, with checked laws at zero trust: ordered inputs give an ordered result, and an event of any input trace is a member of the result, with a conformance seed"
status: active
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Time follows Numerical in the L3 layer rotation (docs/program/06-catalog-campaign.md:377, time/events/traces); spec 69 names k-way merge among its deferred topics. Steward-filed per COORDINATION section 2."
---

# K-way trace merge

## Objective

`Data.Time.EventTrace` exports `trace_merge_all`, which merges a list of
traces by folding the existing two-way `trace_merge`. Two public checked
laws hold. **Ordered:** if every input trace is `time_ordered`, so is the
result. **Complete:** an event that is a member of some input trace is a
member of the result. Spec 69 states the operation and laws by their
public types.

## Settled inputs (main `22876f808`)

- **The carrier.** `Event p` (`EventTrace.ken.md:80`) is `MkEvent time
  payload`. `time_ordered` (`:99`) is a `Prop`, `is_sorted` under
  `event_before`. `trace_merge` (`:108`) is a left-biased two-way merge,
  with `trace_merge_nil_left` (`:433`) and `trace_merge_nil_right`
  (`:1165`).
- **The two-way laws to fold.** `trace_merge_ordered` (`:175`, both
  inputs ordered), `trace_merge_count` (`:214`),
  `trace_merge_member_left` and `_right` (`:1445`, `:1464`),
  `trace_merge_member_sound` (`:1422`, a truncated `Or`) and
  `trace_merge_assoc` (`:1897`).
- **The deferral.** Spec 69 lists k-way merge as deferred
  (`69-time-and-events.md:229`). The spec ends at §14 (`:1200`), so the
  new section is §15.
- **Premise form.** §14's law quantifies over trace members with
  `list_elem`; the per-trace premise here takes the same form over the
  list of traces.
- **No sum combinator.** `Data.Collections.Derived` has no `sum` in this
  floor (`Derived.ken.md:1307`; none under `Data/Collections` or
  `Data/Numeric`), so a count law over k traces is not stated by a closed
  sum.
- **Seeds and tests.** Seeds go under `conformance/stdlib/time-and-events/`;
  tests are `crates/ken-elaborator/tests/cat_time_<slug>.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

One candidate:
- `trace_merge_all` and the two laws in `Data.Time.EventTrace`, with
  names, fold direction and placement as AC-0 rules them;
- a §15 in spec 69, and the deferral at `:229` narrowed to what is still
  deferred;
- a closed conformance seed;
- a package test;
- the status lines.

## Acceptance

- **AC-0 (D0, Architect specifies, Foundation implementer runs on
  scratch).** A checked development on the landed base, with rc 0, no
  `Axiom`, and `trusted_base()` unchanged as an identity set. It rules:
  - the fold direction, and whether the empty list of traces gives `Nil`
    by computation;
  - whether a soundness law (a result member is a member of some input)
    is public here, and its form, given the truncated `Or` of the two-way
    law;
  - that no §1 to §14 operation or law type changes.
- **AC-1.** §15 states `trace_merge_all` and both laws by their public
  types. The Spec Leader and CV review it.
- **AC-2.** The seed's accepted client merges three closed ordered
  traces, checks the result ordered through the ordered law, and checks a
  third-trace event's membership through the complete law. Its refused
  block pins, in the same form as an accepted control, the ordered law
  claimed with one unordered input. A kernel that accepts it would show
  the premise was not checked.
- **AC-3.** Every EventTrace package test that checked before still
  checks. The new package test fails if `trace_merge_all` is redefined to
  drop its last input trace.

## Stop conditions

- A law needs an Axiom, or a kernel or surface change.
- A change to an existing §1 to §14 operation or law type.
- The laws need time over an arbitrary `Ord`, deduplication or a
  strictly increasing trace: each stays deferred as a separate node.

Checks: 4 fired: `Event`, `time_ordered`, `trace_merge` and the cited
two-way laws were read at the cited lines on `22876f808`. 3 fired:
`merge_all`, `k-way` and `kway` were grepped over spec, catalog and
issues; the only hits are the two deferral mentions, so no k-way merge
exists to reuse.
