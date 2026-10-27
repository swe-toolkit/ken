---
id: CAT-TIME-WINDOW-COMPOSITION
title: "Data.Time.EventTrace exports spec 69 section 5: trace_window_window and the two trace_merge empty-unit laws, proved at zero trust from the Architect's D0, with the section 5 seed run by a package test"
status: ready
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-TIME-WINDOW-COMPOSITION]
blocks: []
github: null
origin: "Catalog half of SPEC-TIME-WINDOW-COMPOSITION (spec candidate 822465423, Decision dec_3ca2vy7xrmh8p). Same shape as CAT-NUMERIC-INTERVAL-INSIDE-ORDER. Steward-filed per COORDINATION section 2."
---

# EventTrace exports window composition and the merge units

## Objective

`Data.Time.EventTrace` exports `trace_window_window`,
`trace_merge_nil_left` and `trace_merge_nil_right`, with exactly spec 69
§5's public types, each a checked proof.

## Settled inputs (spec 69 §5 at `822465423`, D0 on `98970bde7`)

- **The surface.** §5 states the three laws by type. Composition is one
  structural list equality over `max lo1 lo2` and `min hi1 hi2`, with no
  ordering premise.
- **The D0.** The Architect's checked D0 is saved as
  `/workspaces/ken/local/architect-shared/time-window-d0/time-window-d0.patch`
  (sha256 `2d66f795…`): rc 0, fmt rc 0, `trusted_base()` 114 before and
  after, with no `Axiom`. It edits `EventTrace.ken.md` (the
  `Data.Numeric.Nat.Order` import gains `max` and `min`, three exports,
  seven private helpers) and the privacy pin in
  `cat_time_window_merge_distribution.rs:144`, which expects
  `trace_merge_nil_right` to be private.
- **Promotion, not a new declaration.** `trace_merge_nil_right` is the
  existing private helper (`EventTrace.ken.md:952`), made public with its
  proof unchanged. The seven new helpers stay private.
- **Seed.** `conformance/stdlib/time-and-events/seed-window-composition.md`
  arrives with the spec candidate.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The three exports in `EventTrace.ken.md`, the privacy pin updated, and a
new `crates/ken-elaborator/tests/cat_time_window_composition.rs` that
root-loads all three and runs the seed. Once the spec candidate has
landed, update spec 69 §5's status lines and the seed's execution-status
statements in the same candidate to say the package test checks them.
Make no conformance-runner, CI or landing claim.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass. The test root-loads
  the three names as package-owned and transparent, and runs the seed's
  accepted client. Its refused block is pinned to the seed's union-bounds
  proposition, and the same-input control checks. The existing
  `cat_time_*` binaries still pass.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom or
  primitive. The census row for `Data.Time.EventTrace` changes only by
  the D0's `max` and `min` import, measured on the candidate.
- **AC-3 (mutation, QA).** Each mutant fails and is restored:
  - `trace_window_window`'s right side with `min` and `max` swapped fails
    check;
  - the refused block put back at the intersected bounds fails the pin.

## Stop conditions

- A law needs an Axiom, a new public helper, a kernel or surface change,
  or a public type other than §5's.
- A change to an existing §1 to §4 law or operation.
