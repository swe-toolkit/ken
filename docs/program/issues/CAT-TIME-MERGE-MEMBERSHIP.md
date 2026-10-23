---
id: CAT-TIME-MERGE-MEMBERSHIP
title: "Data.Time.EventTrace exports spec 69 section 4: premise-free trace_merge membership, sound and complete, proved at zero trust from the Architect's D0 development, with the section 4 seed run by a package test"
status: active
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-TIME-MERGE-MEMBERSHIP]
blocks: []
github: null
origin: "Operator 2026-09-13: every catalog package fully proved. SPEC-TIME-MERGE-MEMBERSHIP states section 4 from the Architect's checked D0, not yet exported. Same shape as CAT-NUMERIC-INTERVAL-DISTRIBUTIVITY. Steward-filed per COORDINATION section 2."
---

# EventTrace exports merge membership

## Objective

`Data.Time.EventTrace` exports spec 69 §4's merge membership laws, with
exactly §4's public types, each a checked proof.

## Settled inputs (spec 69 §4; Architect D0)

- **The surface.** Exactly the names and types §4 states, with no
  ordering premise.
- **The D0.** The Architect's SPEC-TIME-MERGE-MEMBERSHIP AC-0
  development, checked at rc 0 with no `Axiom`.
- **Seed.** The §4 seed under `conformance/stdlib/time-and-events/`. It
  must be on `main` before this WP starts.
- **Package.** `catalog/packages/Data/Time/EventTrace.ken.md` and its
  existing package tests under `crates/ken-elaborator/tests/`, with a
  census row in `lang_mod_strict_resolution_d0.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

- **Seed fix (Steward amendment 2026-10-23).** The landed positive fence
  ends one `)` short in `right_result_member_is_sound` (stop
  `evt_5f0ddkgkdqsn4`). The CV adds that one parenthesis as this
  candidate's first commit, on the seed path only. Spec-author and Spec
  Leader review it inside the candidate.

## Deliverable

The §4 exports and their private helpers in `EventTrace.ken.md`; the
spec 69 §4 status lines updated to say the exports are delivered; and a
new package test that root-loads the §4 names and runs the seed. Every
seed execution-status statement changes in the same candidate to say the
package test executes it.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass. The test root-loads
  the §4 names as package-owned and transparent and runs the seed's
  positive cases. Its refused block is pinned to the seed's proposition,
  and the same-input control checks. The §1 to §3 suites still pass. The
  Spec leader reviews the spec/ status lines, and CV the seed's.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom or
  primitive. The EventTrace census row is unchanged unless a new
  Derived import is required; measured on the exact candidate.
- **AC-3 (mutation, QA).** Each mutant fails and is restored: completeness
  stated for `xs` alone with `ys` dropped from the merge fails to check,
  and an unrelated false proposition in the refused block fails the pin.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a public type
  other than §4's.
- A change to a §1 to §3 operation or law.
