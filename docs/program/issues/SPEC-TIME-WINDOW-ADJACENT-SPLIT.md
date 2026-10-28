---
id: SPEC-TIME-WINDOW-ADJACENT-SPLIT
title: "Spec 69 composes nested windows but no law splits one window at an interior bound. Rule from a checked D0 at zero trust that an ordered trace's window [lo, hi) is the append of its windows [lo, mid) and [mid, hi), with the premises the D0 shows necessary, and state it with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Time is next in the layer rotation after Numerical. Same shape as SPEC-TIME-MERGE-ASSOCIATIVITY. Steward-filed per COORDINATION section 2."
---

# Adjacent windows split an ordered trace

## Objective

Spec 69 states, as a public checked law over `Data.Time.EventTrace`,
that `trace_window p lo hi tr` equals
`list_append (Event p) (trace_window p lo mid tr)
(trace_window p mid hi tr)` as structural `Equal (List (Event p))`,
under the premises the D0 shows necessary.

## Settled inputs (spec 69 §1 to §6, main `4a4e67ae3`)

- **The operation.** `trace_window` is `filter` by the half-open
  `lo ≤ t < hi` test, and keeps the input's order even when the input
  is unordered (§1.2). `list_append` is the existing
  `Data.Collections.Derived` export, which EventTrace already imports
  whole.
- **No split law.** §5's `trace_window_window` composes nested windows
  into their intersection. No section splits one window at an interior
  bound. The only append prose in spec 69 says merge is not append.
- **The premises are expected to be necessary.** Each fails on a closed
  one- or two-event trace:
  - `time_ordered tr`: `tr = [e5, e1]` with `lo = 0`, `mid = 3`,
    `hi = 10`. The left side is `[e5, e1]`, the right `[e1, e5]`;
  - `lo ≤ mid`: `tr = [e1]` with `lo = 2`, `mid = 0`, `hi = 5`. The
    left side is empty, the right `[e1]`;
  - `mid ≤ hi`: `tr = [e4]` with `lo = 0`, `mid = 5`, `hi = 3`. The
    left side is empty, the right `[e4]`.

  The D0 measures each. It keeps a premise only with its
  counterexample, and states how the bound premises are spelled.
- **Seeds.** Under `conformance/stdlib/time-and-events/`, next to
  `seed-window-composition.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §7 in spec 69 stating the ruled law by its public type, and a closed
conformance seed under `conformance/stdlib/time-and-events/`, with the
status lines changed in the same candidate. The catalog half is a
separate `CAT-TIME-` node, framed from the D0 after this lands, unless
the Architect rules that it travels in this candidate as
SPEC-TIME-MERGE-ASSOCIATIVITY's did.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged. It rules the
  name, the public type and argument order, and which helpers stay
  private. Each premise it keeps comes with a closed counterexample to
  the law without it.
- **AC-1.** The new section states the ruled law by its public type.
  The Spec leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the law to a closed
  ordered trace with events on both sides of `mid`, one of them at
  exactly `mid`, so the half-open boundary is exercised. Each side is
  also computed independently by `Refl`. Its refused block is pinned to
  one false proposition of the same shape: the split with the two
  appended windows swapped. It also carries an accepted same-shape
  control and an independent `False` value.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or a premise
  beyond `time_ordered` and the two bound orderings.
- A change to an existing §1 to §6 law or operation.

Checks: 3 fired: grepped spec 69 and `EventTrace.ken.md` for `append`,
`split`, `adjacent` and `concat` on `4a4e67ae3`. The hits are the
merge-is-not-append notes (`69-time-and-events.md:468`,
`EventTrace.ken.md:2500`), the §5 split helpers named for `max` and
`min`, and the adjacent-pair sortedness prose. None splits a window.
8 fired: each premise was checked against a closed counterexample above,
and AC-2's swapped-append refusal is false on the seed's trace only
because both windows are non-empty, which the event at `mid` and an
event below it guarantee.
