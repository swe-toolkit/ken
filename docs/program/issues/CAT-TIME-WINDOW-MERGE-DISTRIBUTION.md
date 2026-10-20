---
id: CAT-TIME-WINDOW-MERGE-DISTRIBUTION
title: "Data.Time.EventTrace exports spec 69 section 3: trace_window_merge, the structural law that windowing an ordered merge equals merging the windowed inputs, proved at zero trust from the Architect's D0 development, with Data.Collections.Derived's filter_starts_after made public and the distribution seed run by a package test"
status: active
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-TIME-WINDOW-MERGE-DISTRIBUTION]
blocks: []
github: null
origin: "Operator 2026-09-13: every catalog package fully proved. SPEC-TIME-WINDOW-MERGE-DISTRIBUTION states section 3 and records the law as checked in the Architect's D0 scratch development, not yet exported. Same shape as CAT-NUMERIC-INTERVAL-INCLUSION. Steward-filed per COORDINATION section 2."
---

# EventTrace exports window/merge distribution

## Objective

`Data.Time.EventTrace` exports `trace_window_merge` with exactly spec 69
§3.1's public type, as a checked proof.

## Settled inputs (spec 69 §3; Architect D0)

- **The law.** `trace_window_merge p lo hi xs ys`, with premises
  `time_ordered p xs` and `time_ordered p ys`, concludes `Equal (List
  (Event p))` between the window of the merge and the merge of the
  windows. It is the only new public §3 law; the count corollary needs
  no theorem (§3.2).
- **Provider export (§3.3).** `filter_starts_after` in
  `Data.Collections.Derived` becomes `pub theorem`, and EventTrace adds
  the name to its existing `import Data.Collections.Derived` selection.
  No new import declaration and no duplicate local proof.
- **The D0.** `local/arch-twmd-d0/d0-catalog-81ab720ff.patch` (primary
  checkout), on base `81ab720ff`: the Derived visibility change plus the
  law, 12 private helpers and one private fn in EventTrace. Both
  packages check rc 0 with no Axiom. The wholesale Derived importers
  (Dependency, Decoder, Finite, the argmin, window-membership and
  event-trace seeds) check unchanged, and none of the 13 new names is
  defined elsewhere (`local/arch-twmd-d0/results-81ab720ff.txt`).
- **Seed.**
  `conformance/stdlib/time-and-events/seed-window-merge-distribution.md`,
  from SPEC-TIME-WINDOW-MERGE-DISTRIBUTION.
- **Package.** `catalog/packages/Data/Time/EventTrace.ken.md`, tested by
  `crates/ken-elaborator/tests/cat_time_event_trace.rs` and
  `cat_time_window_membership.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

`trace_window_merge` and its private helpers in `EventTrace.ken.md`, the
`filter_starts_after` visibility change in `Derived.ken.md`, the spec 69
§3 status lines updated to say the export is delivered, and a new
`crates/ken-elaborator/tests/cat_time_window_merge_distribution.rs`
that root-loads `trace_window_merge` and runs the seed.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass for both packages.
  The test root-loads `trace_window_merge` as package-owned and
  transparent, keeps the new helpers private, and runs the seed's
  positive cases. Each refused block is pinned to its own false claim
  on the unordered pair, so an unrelated failure does not satisfy it.
  The §1 and §2 EventTrace suites still pass. The status lines change
  with the export (Steward `evt_5rtkgkdf42qb`), and the Spec leader
  reviews those spec/ paths.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom or
  primitive. The `lang_mod_strict_resolution_d0`
  `catalog_ambient_passthrough_migration_census` rows for EventTrace
  and Derived are unchanged, measured on the candidate.
- **AC-3 (mutation, QA).** Each mutant fails, and is restored:
  - removing the `time_ordered p ys` premise fails to check;
  - making `trace_merge` take the right head on a tie fails the seed's
    exact tie output;
  - reverting `filter_starts_after` to private fails EventTrace's check.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, a payload
  equality decision, a sorted carrier, or a public type other than
  §3.1's.
- A census row changes, or a Derived importer outside these two
  packages changes behavior.
