---
id: CAT-GRAPHS-WALK-COMPOSITION-ORDER
title: "Algorithm.Graphs.Dependency exports spec 62 section 3: walk_append and ordered_walk_forward, proved at zero trust from the Architect's D0 development, with the section 3 seed run by a package test"
status: ready
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-GRAPHS-WALK-COMPOSITION-ORDER]
blocks: []
github: null
origin: "Operator 2026-09-13: every catalog package fully proved. SPEC-GRAPHS-WALK-COMPOSITION-ORDER states section 3 (cb48b8ddb) from the Architect's checked D0 scratch development, not yet exported. Same shape as CAT-LINEAR-ALGEBRA-MATRIX-SUM. Steward-filed per COORDINATION section 2."
---

# Dependency exports walk composition

## Objective

`Algorithm.Graphs.Dependency` exports `walk_append` and
`ordered_walk_forward` with exactly spec 62 §3's public types, the law
a checked proof.

## Settled inputs (spec 62 §3, on `main` at `cb48b8ddb`; Architect D0)

- **The surface.** `walk_append` joins two walks in path order by
  recursion on the second walk; the right `WalkHere` identity holds by
  computation. `ordered_walk_forward` puts the start of a walk with a
  checked first edge before its end in every `TopoOrder`. No left-unit
  or associativity law is public, and `walk_before` stays private.
- **The D0.** `/workspaces/ken/local/arch-gwc-d0/dependency-d0.diff` on
  base `10b1ec623`, with the probes and `trust.log` beside it. Checked
  rc 0; `public_surface_generic_laws_and_zero_trust_delta` passed with
  `trusted_base()` unchanged. Its private `walk_append_here_left` and
  `walk_append_assoc` stay private, and only if the proof uses them.
- **Seed.** The §3 seed under `conformance/stdlib/graphs/`, from
  SPEC-GRAPHS-WALK-COMPOSITION-ORDER AC-2. It must be on `main` before
  this WP starts.
- **Package.** `catalog/packages/Algorithm/Graphs/Dependency.ken.md`,
  tested by `crates/ken-elaborator/tests/cat_graphs_dependency.rs`, with
  a census row in `lang_mod_strict_resolution_d0.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The two exports and any private helpers in `Dependency.ken.md`; the
spec 62 §3 status lines updated to say the exports are delivered; and
a new `crates/ken-elaborator/tests/cat_graphs_walk_composition.rs` that
root-loads both names and runs the seed. If
`cat_graphs_dependency.rs` pins the exact public-name set, it gains
exactly these two names and nothing else. Every seed execution-status
statement changes in the same candidate to say the package test
executes it, with no conformance-runner, CI or landing claim, and no
fence or fixture change.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass. The test root-loads
  both names as package-owned and transparent, keeps the helpers
  private, and runs the seed's positive cases. Its refused block is
  pinned to the seed's reversed-order proposition, and the same-walk
  control checks. The §1 and §2 suite still passes. The Spec leader
  reviews the spec/ status lines, and CV the seed's.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom or
  primitive. The census row for `Algorithm.Graphs.Dependency` is
  unchanged, measured on the candidate.
- **AC-3 (mutation, QA).** Each mutant fails, and is restored:
  - `walk_append` returning its first walk unchanged fails the seed's
    composition case;
  - an unrelated false closed proposition in the refused block fails
    the pin.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a public type
  other than §3's.
- A new public lemma, or any change outside `Dependency` other than
  the inventory pin above.
- A change to a §1 or §2 operation or law, or a census row change.

Checks: 4 fired: `dependency-d0.diff` adds exactly the two public and
two private names, `trust.log` and `cat_graphs_dependency.rs` exist,
and §3 is on `main` at `cb48b8ddb`. 7 fired: AC-3 mutates the
composition itself, which the order law alone would not catch.
