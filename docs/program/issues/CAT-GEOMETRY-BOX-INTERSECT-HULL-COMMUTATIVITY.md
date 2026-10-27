---
id: CAT-GEOMETRY-BOX-INTERSECT-HULL-COMMUTATIVITY
title: "Data.Numeric.Nat.Order exports comm for min and max, and Algorithm.Geometry.NatBox exports spec 67 section 6: intersect_comm and hull_comm, proved at zero trust from the Architect's D0, with the section 6 seed run by a package test"
status: merged
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-GEOMETRY-BOX-INTERSECT-HULL-COMMUTATIVITY]
blocks: []
github: null
origin: "Catalog half of SPEC-GEOMETRY-BOX-INTERSECT-HULL-COMMUTATIVITY (spec candidate 11b11001d, Decision dec_wbxhgv6s8tdk). Same shape as CAT-OPTIMIZATION-BOUNDED-SEARCH-BOUND-STABILITY. Steward-filed per COORDINATION section 2."
---

# Box intersection and hull export commutativity

## Objective

`Data.Numeric.Nat.Order` exports `proof comm for min` and
`proof comm for max`, and `Algorithm.Geometry.NatBox` exports
`intersect_comm` and `hull_comm`, each with exactly spec 67 §6's
public type, as checked proofs.

## Settled inputs (spec 67 §6 at `11b11001d`, D0 on `0bb872591`)

- **The surface.** §6.1 states `comm for min (m : Nat) (n : Nat) :
  Equal Nat (min m n) (min n m)` and the same for `max`, as proof
  selectors, measured as globals `Data.Numeric.Nat.Order.min::comm` and
  `max::comm`. §6.2 states `intersect_comm (b : Box) (c : Box) : Equal
  Box (intersect b c) (intersect c b)` and `hull_comm` likewise, with
  no premise.
- **The D0.** The Architect's checked D0 is saved as
  `/workspaces/ken/local/architect-shared/box-comm-d0/d0.patch`
  (sha256 `f013faa9…`; ruling `ruling.md` beside it): check and fmt rc
  0 on both packages, probe test 2/2, `trusted_base()` 114 before and
  after with equal GlobalId sets, no `Axiom`. Its hunks place
  `comm for min` after `proof greatest for min` and `comm for max`
  after `proof least for max` in `Order.ken.md`; `intersect_comm` and
  `hull_comm` after `hull_contains_right`, and the private helpers
  after `point_equal`, in `NatBox.ken.md`. No import list changes. Its
  third file, `arch_box_comm_d0.rs`, is scratch and does not ship.
- **Proof forms are measured, not stylistic.** The two open `max`
  corners take `Refl`, and closed `Equal Box` goals take `Refl`;
  `Proved` was refused at both (`ruling.md`). Keep the D0 bodies.
- **Three private helpers.** `box_with_lo`, `box_with_hi` and
  `box_equal` stay private in NatBox.
- **The consumers.** The NatBox prose counts its laws
  (`NatBox.ken.md:29-32`), and the D0 does not edit it.
  `cat_geometry_nat_box.rs:134` lists named NatBox laws and
  `cat_nat_min_max_bound_laws.rs:48` pins Order selectors by
  `{operation}::{law}`. The Architect found no exhaustive export pin
  in the six Order and geometry tests it inspected. Order has many
  importers, so CI is the breadth gate for them; do not run them
  locally.
- **Seed.**
  `conformance/stdlib/geometry/seed-box-intersect-hull-commutativity.md`
  arrives with the spec candidate. Its fences include
  `nat-extrema-commute`, which imports `min` and `max` and applies both
  selectors.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The two selectors in `Order.ken.md`, the two laws and three helpers in
`NatBox.ken.md` with its law prose corrected, and a new
`crates/ken-elaborator/tests/cat_geometry_box_intersect_hull_commutativity.rs`
that root-loads the four names and runs the seed against the shipped
exports. Once the spec candidate has landed, update spec 67 §6's status
lines and the seed's execution-status statements in the same candidate
to say the package test checks them. Make no conformance-runner, CI or
landing claim.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass on both packages.
  The test root-loads `min::comm` and `max::comm` as Order-owned and
  transparent, and `intersect_comm` and `hull_comm` as NatBox-owned and
  transparent. It refuses importing each of the three helpers, and runs
  the seed's whole accepted client, including `nat-extrema-commute`.
  Seed cases are selected by fence name, not by fence count. Its refused
  block is pinned to `seed_neg`'s false `Equal Box`, and
  `hull_swapped_closed` checks as the same-form control.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom or
  primitive. The census rows for both packages are unchanged, measured
  on the candidate.
- **AC-3 (mutation, QA).** Each mutant fails and is restored:
  - `hull_comm` with its two `max` corner arguments to `box_equal`
    built from `comm for min` fails check;
  - `seed_neg`'s right side replaced by `hull b13 b02`, a true claim,
    fails the pin.

## Stop conditions

- A law needs an Axiom, a public helper beyond the four, a kernel or
  surface change, or a public type other than §6's.
- A change to an existing Order or NatBox law or operation, or a census
  row change.

Checks: 3 fired: grepped catalog/, crates/, conformance/ for the four
names; only substring hits (`interval_hull_comm_inside`,
`interval_intersect_comm_inside`), no collision. 4 fired: §6's types,
placement, trust count and proof forms are read from `ruling.md`, the
patch's hunk headers and `exact-rc.txt`/`test-rc.txt` (all 0). 8 fired:
the second AC-3 mutant turns the refusal into an accepted true claim,
so a pin that only checks that some block was refused stays green.
