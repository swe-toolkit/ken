---
id: CAT-NUMERIC-INTERVAL-HULL-INTERSECT-BOUNDS
title: "Algorithm.Numeric.RationalInterval exports spec 68 section 7: interval_hull_least and interval_intersect_greatest, proved at zero trust from the Architect's D0, with the section 7 seed run by a package test"
status: ready
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-NUMERICAL-INTERVAL-HULL-INTERSECT-BOUNDS]
blocks: []
github: null
origin: "Catalog half of SPEC-NUMERICAL-INTERVAL-HULL-INTERSECT-BOUNDS (spec candidate db746beb1, Decision dec_vjrggenwxa3p). Same shape as CAT-NUMERIC-INTERVAL-ASSOCIATIVITY-INSIDE. Steward-filed per COORDINATION section 2."
---

# Interval hull and intersection export their tightest-bound laws

## Objective

`Algorithm.Numeric.RationalInterval` exports `interval_hull_least` and
`interval_intersect_greatest`, each with exactly spec 68 §7's public type,
as checked Transparent proofs.

## Settled inputs (spec 68 §7 at `db746beb1`, D0 on `1363b09b4`)

- **The surface.** `interval_hull_least (i j k)` takes
  `IsTrue (interval_inside i k)` then `IsTrue (interval_inside j k)` and
  concludes `IsTrue (interval_inside (interval_hull i j) k)`.
  `interval_intersect_greatest (k i j)` takes the inclusions of `k` in `i`
  then in `j` and concludes `k` is inside `interval_intersect i j` (§7).
- **The D0.** The Architect's checked D0 is
  `/workspaces/ken/local/architect-shared/interval-bounds-d0/d0.patch`
  (sha256 `e448b72c…`). Its `rc.txt` records build, check and fmt rc 0,
  and the three sibling package tests rc 0. Its law hunk adds the two
  theorems directly after `interval_mul_assoc_rev_inside`
  (`RationalInterval.ken.md` hunk `@@ -973`). Its test file
  `arch_interval_bounds_d0.rs` is scratch and does not ship.
- **No new helper.** The proofs use Rational's public
  `proof greatest for rational_min` and `proof least for rational_max`.
  Keep the D0 bodies.
- **The prose.** The law-count sentences in the package's §2 Definition
  (`:30` to `:41`) and §4 Laws & proofs (`:1048`) change with the exports.
- **The consumers.** `cat_numeric_interval_{associativity_inside,
  distributivity,hull,inclusion,inside_order}.rs` and
  `cat_numeric_rational_interval.rs` assert trust sets and named rows.
  The two names are free in catalog/, crates/, conformance/, spec/ and
  examples/ on `1363b09b4`. Other importers run in CI; do not run them
  locally.
- **Seed.** `conformance/stdlib/numerical/seed-interval-hull-intersect-bounds.md`
  arrives with the spec candidate.
- **Out of scope.** The §2 seed `seed-interval-hull-intersection.md`
  imports `IsTrue` from `Core.Classes.LawfulClasses`, which the Architect
  measured as AmbiguousReference on main. That class is already carried
  to the L2 flip (`evt_2b1dvrpg6z3wj`). Do not edit that seed here.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The two laws in `RationalInterval.ken.md` with its prose corrected, and a
new `crates/ken-elaborator/tests/cat_numeric_interval_hull_intersect_bounds.rs`
that root-loads the two names and runs the §7 seed against the shipped
exports. Once the spec candidate has landed, update spec 68 §7's status
lines, spec 68's SPEC-PROGRESS row and the seed's execution-status
statements in the same candidate to say the package test checks them. Make
no conformance-runner, CI or landing claim.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass on the package. The
  test root-loads both laws as RationalInterval-owned and Transparent and
  runs the seed's whole accepted client. Seed cases are selected by fence
  or theorem name, not by fence count. Its refused block is pinned to the
  seed's false hull claim, and the seed's same-shape `Proved` row checks
  as the control.
- **AC-2.** `trusted_base()` is equal before and after as an identity set,
  with no Axiom or primitive. The package's census row is unchanged,
  measured on the candidate.
- **AC-3 (mutation, QA).** Each mutant fails and is restored:
  - in `interval_hull_least`, the lower-endpoint `proof greatest` given
    the `j` premise's upper projection in place of its lower one fails
    check;
  - the seed's refused claim replaced by a true inclusion of the same
    shape fails the pin.

## Stop conditions

- A law needs an Axiom, a public helper, a kernel or surface change, or a
  public type other than §7's.
- A change to an existing RationalInterval law or operation, or a census
  row change.

Checks: 4 fired: the D0 patch exists at the stated path with sha256
`e448b72c`, its `rc.txt` records build, check and fmt rc 0, and its law
hunk adds exactly `interval_hull_least` and `interval_intersect_greatest`
after `interval_mul_assoc_rev_inside`. 3 fired: grepped catalog/, crates/,
conformance/, spec/ and examples/ on `1363b09b4`; neither name occurs. The
six `cat_numeric_*` interval tests are listed from `crates/ken-elaborator/tests`.
