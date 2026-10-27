---
id: CAT-GRAPHS-WALK-APPEND-UNIT-ASSOC
title: "Algorithm.Graphs.Dependency exports spec 62 section 4: walk_append_here_left and walk_append_assoc, proved at zero trust from the Architect's D0, with the section 4 seed run by a package test"
status: merged
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-GRAPHS-WALK-APPEND-UNIT-ASSOC]
blocks: []
github: null
origin: "Catalog half of SPEC-GRAPHS-WALK-APPEND-UNIT-ASSOC (D0 evt_2yj6y7jhbfteg, spec a7fb20048). Same shape as CAT-LINEAR-ALGEBRA-SUM-COMM-ASSOC. Steward-filed per COORDINATION section 2."
---

# Dependency exports the walk-append laws

## Objective

`Algorithm.Graphs.Dependency` exports `walk_append_here_left` and
`walk_append_assoc` with exactly spec 62 §4's public types, each a
checked proof.

## Settled inputs (spec 62 §4 and its seed, from `a7fb20048`)

- **The surface.** §4 states the left unit and left-to-right
  associativity of `walk_append` as equalities of walks,
  `Equal (Walk q g s u)` and `Equal (Walk q g s v)`, with no premise.
  The right unit stays a reduction with no theorem.
- **The D0.** The Architect's checked D0 on `d4f2b202c`
  (`evt_2yj6y7jhbfteg`) gives both bodies verbatim, saved as
  `local/architect-shared/walk-append-d0/dependency-laws.patch` (sha256
  `85fa698c…`). They go in the public block immediately after
  `walk_append`. No helper, public or private, is added. `cong` comes
  from the existing `Core.Logic.Transport` import. `trusted_base()` is
  114 before and after; both laws are Transparent.
- **Load-bearing spellings.** The left unit's `WalkHere` branch must be
  `Proved`, because `Refl` is refused by the elaborator. The
  associativity `WalkHere` branch must be `Refl`, because `Proved` is
  refused by the kernel. Do not swap them.
- **The consumers.** Two consumers change. The package's §7 count of
  eleven kernel-checked laws becomes thirteen. The export and
  Transparent loop in `cat_graphs_walk_composition.rs` gains the two
  names. No export-set pin exists elsewhere (the Architect's sweep).
- **Seed.** `conformance/stdlib/graphs/seed-walk-append-unit-assoc.md`
  must be on `main` before this WP starts. It has not been run against
  shipped exports.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The two exports in `Dependency.ken.md` with the §7 count corrected, the
migrated loop, the spec 62 §4 status lines updated to say the exports
are delivered, and a new
`crates/ken-elaborator/tests/cat_graphs_walk_append_unit_assoc.rs` that
root-loads both names and runs the seed. Seed execution-status
statements change in the same candidate to say the package test checks
them, with no conformance-runner, CI or landing claim, and no fence
change.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass. The test root-loads
  both names as package-owned and transparent and runs the seed's
  accepted client: the left unit, associativity with `WalkHere` in each
  position, and the closed `Refl` instance. Its refused block is pinned
  to the seed's other-route proposition, and the same-input route
  control checks. `cat_graphs_dependency` and
  `cat_graphs_walk_composition` still pass. The Spec leader reviews the
  spec/ status lines, and CV the seed's.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom
  or primitive. The census row for `Algorithm.Graphs.Dependency` is
  unchanged, measured on the candidate.
- **AC-3 (mutation, QA).** Each mutant fails, and is restored:
  - the associativity `WalkStep` case with its `cong` replaced by
    `Refl` fails check;
  - `walk_append_here_left` restated with `walk` on both sides fails
    check or the seed's left-unit application;
  - the refused block's route changed to the true `D1` route fails the
    pin.

## Stop conditions

- A law needs an Axiom, a helper, a kernel or surface change, or a
  public type other than §4's.
- A change to `Walk`, `walk_append` or an existing law, a new import,
  or a census row change.

Checks: 4 fired: both bodies, the placement, the `cong` import and the
two consumers are read from `evt_2yj6y7jhbfteg` on `d4f2b202c`; the
seed and §4 are on `a7fb20048`. 8 fired: the third AC-3 mutant turns
the refusal into the accepted route, so a pin that only checks that
some block was refused stays green on it.
