---
id: CAT-GRAPHS-REACHABILITY-TARGET-MONOTONE
title: "Algorithm.Graphs.Dependency exports spec 62 section 6: graph_reachable_target_monotone, proved at zero trust from the Architect's D0, with the section 6 seed run by a package test"
status: merged
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-GRAPHS-REACHABILITY-TARGET-MONOTONE]
blocks: []
github: null
origin: "Catalog half of SPEC-GRAPHS-REACHABILITY-TARGET-MONOTONE (spec candidate 067b734c2, Decision dec_7crmzk1w4rg5y). Same shape as CAT-GRAPHS-REACHABILITY-ALONG-WALK. Steward-filed per COORDINATION section 2."
---

# Dependency exports target monotonicity of reachability

## Objective

`Algorithm.Graphs.Dependency` exports `graph_reachable_target_monotone`
with exactly spec 62 §6's public type, as a checked proof.

## Settled inputs (spec 62 §6 at `067b734c2`, D0 on `abafe3963`)

- **The surface.** Spec 62 §6 states `graph_reachable_target_monotone`
  with its ruled argument order and inclusion premise. Copy the type
  from §6, not from this frame.
- **The D0.** The Architect's checked D0 is saved as
  `/workspaces/ken/local/architect-shared/reach-mono-d0/d0.patch`
  (sha256 `02da799e…`): build, check, fmt and test rc 0, probe test 2/2,
  `trusted_base()` 114 before and after, no `Axiom`. Its one
  `Dependency.ken.md` hunk places the law after
  `graph_reachable_along_walk` and adds no helper. Its file
  `arch_reach_mono_d0.rs` is scratch and does not ship.
- **The consumers.** The package prose says "fourteen kernel-checked
  laws" (`Dependency.ken.md:2257`); it goes to fifteen. Sweep the other
  `cat_graphs_*` tests for an exact export-set or law-count pin before
  editing, and report what you find.
- **Seed.** `conformance/stdlib/graphs/seed-reachability-target-monotone.md`
  arrives with the spec candidate.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

- The export in `Dependency.ken.md`, with its count prose corrected.
- A new `crates/ken-elaborator/tests/cat_graphs_reachability_target_monotone.rs`
  that root-loads the law and runs the seed, in the shape of
  `cat_graphs_reachability_along_walk.rs`.
- Spec 62 §6's status lines, the spec 62 row in `spec/SPEC-PROGRESS.md`
  and the seed's execution-status statements, updated to say the package
  test checks them. No other status row changes.

Make no conformance-runner, CI or landing claim.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass.
  - The test root-loads the law as package-owned and transparent.
  - It runs the seed's accepted client, with cases selected by theorem
    name, not by fence count.
  - Its refused block is pinned to the seed's false converse, with the
    same-form accepted control checking.
  - The existing `cat_graphs_*` binaries pass.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom or
  primitive. The census row for `Algorithm.Graphs.Dependency` is
  unchanged, measured on the candidate.
- **AC-3 (mutation, QA).** Each mutant fails and is restored:
  - the law's body with the inclusion premise's application removed (the
    original hit passed to `graph_reachable_complete`) fails check;
  - the seed's refused converse replaced by a true same-form claim fails
    the pin.

## Stop conditions

- The law needs an Axiom, a new public helper, a kernel or surface
  change, or a public type other than §6's.
- A change to an existing §1 to §5 law or operation, or a census row
  change.

Checks: 3 fired: the prose count was grepped at `abafe3963`
(`Dependency.ken.md:2257`); no exact export-set pin was found by grepping
`graph_reachable_along_walk` across crates/, so the implementer's sweep is
required rather than assumed. 8 fired: the second AC-3 mutant turns the
refusal into an accepted true claim, so a pin that only checks that some
block was refused stays green.
