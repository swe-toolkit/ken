---
id: CAT-GRAPHS-REACHABILITY-ALONG-WALK
title: "Algorithm.Graphs.Dependency exports spec 62 section 5: graph_reachable_along_walk, proved at zero trust from the Architect's D0, with the section 5 seed run by a package test"
status: merged
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-GRAPHS-REACHABILITY-ALONG-WALK]
blocks: []
github: null
origin: "Catalog half of SPEC-GRAPHS-REACHABILITY-ALONG-WALK (spec candidate 7b885ef50, Decision dec_37h3r655qnpa9). Same shape as CAT-GRAPHS-WALK-APPEND-UNIT-ASSOC. Steward-filed per COORDINATION section 2."
---

# Reachability carries back along a walk, exported

## Objective

`Algorithm.Graphs.Dependency` exports `graph_reachable_along_walk` with
exactly spec 62 §5's public type, as a checked Transparent proof.

## Settled inputs (spec 62 §5 at `7b885ef50`, D0 on `454adf947`)

- **The surface.** §5 states `graph_reachable_along_walk (q : Type)
  (fq : Finite q) (g : Graph q) (s : q) (t : q) (target : q → Bool)
  (walk : Walk q g s t) : Equal Bool (graph_reachable q fq g t target)
  True → Equal Bool (graph_reachable q fq g s target) True`.
- **The D0.** The Architect's checked D0 is saved as
  `/workspaces/ken/local/architect-shared/reach-walk-d0/d0.patch`
  (sha256 `d18ca75b…`). `rc.txt` records build, check, fmt and test
  rc 0, and the probe test passed 2/2. Its one catalog hunk places the
  theorem after `graph_find_walk_some` (`Dependency.ken.md:184`). Its
  second file, `arch_reach_walk_d0.rs`, is scratch and does not ship.
- **No new helper.** The proof reuses the private `require_some`
  (`:686`), with `walk_append` and `graph_reachable_complete`. None of
  `require_some`, `walk_reachable` or `edge_reach_trans` becomes public
  (§5).
- **The consumers.** `cat_graphs_dependency.rs:53` and
  `cat_graphs_walk_composition.rs:113` list named exports. Other
  importers run in CI; do not run them locally.
- **Seed.** `conformance/stdlib/graphs/seed-reachability-along-walk.md`
  arrives with the spec candidate.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The theorem in `Dependency.ken.md`, with any law-count prose corrected,
and a new
`crates/ken-elaborator/tests/cat_graphs_reachability_along_walk.rs`
that root-loads the name and runs the seed against the shipped export.
Once the spec candidate has landed, update spec 62 §5's status lines and
the seed's execution-status statements in the same candidate to say the
package test checks them. Make no conformance-runner, CI or landing
claim.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass on the package. The
  test root-loads `graph_reachable_along_walk` as Dependency-owned and
  Transparent, refuses importing `require_some`, and runs the seed's
  whole accepted client, including the `WalkHere` case. Seed cases are
  selected by fence name, not by fence count. Its refused block is
  pinned to the seed's end-to-start reachability claim.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom or
  primitive. The package's census row is unchanged, measured on the
  candidate.
- **AC-3 (mutation, QA).** Each mutant fails and is restored:
  - the proof with `graph_reachable_complete` applied to the witness
    walk alone, dropping `walk_append`, fails check;
  - the refused claim replaced by start-to-end reachability on the same
    chain, a true claim, fails the pin.

## Stop conditions

- The law needs an Axiom, a public helper, a kernel or surface change,
  or a public type other than §5's.
- A change to an existing Dependency law or operation, or a census row
  change.

Checks: 3 fired: grepped crates/, catalog/, conformance/ and examples/
for `graph_find_walk_some`; the named-export lists are in
`cat_graphs_dependency.rs` and `cat_graphs_walk_composition.rs`, no
exhaustive pin found. 4 fired: the type, placement and rc are read from
spec 62 §5 at `7b885ef50`, the patch's hunk header and `rc.txt`. 8
fired: the second AC-3 mutant makes the refusal an accepted true claim,
so a pin that only checks that some block was refused stays green.
