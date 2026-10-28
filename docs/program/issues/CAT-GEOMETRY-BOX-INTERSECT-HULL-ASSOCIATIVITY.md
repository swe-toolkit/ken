---
id: CAT-GEOMETRY-BOX-INTERSECT-HULL-ASSOCIATIVITY
title: "Data.Numeric.Nat.Order exports assoc for min and max, and Algorithm.Geometry.NatBox exports intersect_assoc and hull_assoc, proved at zero trust from the Architect's D0, with the spec 67 seed run by a package test"
status: merged
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-GEOMETRY-BOX-INTERSECT-HULL-ASSOCIATIVITY]
blocks: []
github: null
origin: "Catalog half of SPEC-GEOMETRY-BOX-INTERSECT-HULL-ASSOCIATIVITY (Architect AC-0 evt_1f8atekb12n2w). Same shape as CAT-GEOMETRY-BOX-INTERSECT-HULL-COMMUTATIVITY. Steward-filed per COORDINATION section 2."
---

# Box intersection and hull are associative

## Objective

`Data.Numeric.Nat.Order` exports `assoc for min` and `assoc for max`, and
`Algorithm.Geometry.NatBox` exports `intersect_assoc` and `hull_assoc`,
each with exactly the AC-0 ruled public type, as checked proofs.

## Settled inputs (AC-0 `evt_1f8atekb12n2w`, D0 on `372a6a1a6`)

- **The surface.** `assoc for min (k : Nat) (m : Nat) (n : Nat) : Equal
  Nat (min (min k m) n) (min k (min m n))`, and the same for `max`, each
  placed directly after its `comm for` selector. `intersect_assoc (a :
  Box) (b : Box) (c : Box) : Equal Box (intersect (intersect a b) c)
  (intersect a (intersect b c))`, and `hull_assoc` likewise, placed after
  `hull_comm` and before `manhattan_symm`.
- **The D0.** The Architect's checked D0 is saved as
  `/workspaces/ken/local/architect-shared/box-assoc-d0/d0.patch` (sha256
  `91f7c910…`; `law-and-helpers.txt` and `rc-final.txt` beside it). Check
  and fmt are rc 0 on both packages, `trusted_base()` is 114 before and
  after as identity sets, there is no `Axiom`, and 24/24 existing tests
  pass. Its catalog hunks are Order +46 and NatBox +116. Its
  `arch_box_assoc_d0.rs` is scratch and does not ship.
- **Proof forms are measured, not stylistic.** The `min` Zero arms take
  `Proved` and the `max` Zero arms take `Refl`. Keep the D0 bodies.
- **No new helpers.** `point_equal`, `box_equal`, `box_with_lo` and
  `box_with_hi` stay private.
- **The prose goes stale.** The D0 does not edit prose. NatBox §2 counts
  its laws (`NatBox.ken.md:29-32`) and its API list names the
  commutativity laws (`:1948`). The Order §4 and §7 law counts and API
  lists also need updating. `cat_geometry_nat_box.rs:134` lists named
  NatBox laws, and `cat_nat_min_max_bound_laws.rs:48` pins Order
  selectors by `{operation}::{law}`. The Architect found no exhaustive
  export pin. Order has many importers, so CI is the breadth gate for
  them; do not run them locally.
- **Seed.** The seed under `conformance/stdlib/geometry/` arrives with
  the spec candidate. Until it lands, build on main against the queued
  spec candidate's seed and checkpoint as NOT REVIEWABLE. After it
  lands, merge main (no rebase) and hand off one exact tip.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The two selectors in `Order.ken.md` and the two laws in `NatBox.ken.md`,
with both packages' prose counts and API lists corrected. Add a new
`crates/ken-elaborator/tests/cat_geometry_box_intersect_hull_associativity.rs`
that root-loads the four names and runs the seed against the shipped
exports. Once the spec candidate has landed, update spec 67 §7's status
lines and the seed's execution-status statements in the same candidate
to say the package test checks them. Make no conformance-runner, CI or
landing claim.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass on both packages. The
  test root-loads the four names as module-owned and transparent, and
  refuses importing `box_equal`. It runs the seed's accepted client,
  selecting cases by theorem name, not by fence count. Its refused block
  is pinned to the seed's mixed-regrouping proposition. The existing
  `cat_geometry_*` and `cat_nat_min_max_bound_laws` binaries still pass.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom or
  primitive. The census rows for both modules are unchanged, measured on
  the candidate.
- **AC-3 (mutation, QA).** Each mutant fails and is restored:
  - `intersect_assoc` with `assoc for min` on the lo corners and
    `assoc for max` on the hi corners (the hull pairing) fails check;
  - the refused block's conclusion replaced by a true grouping of one
    operation (`intersect (intersect a b) c` equal to `intersect a
    (intersect b c)`) fails the pin.

## Stop conditions

- A law needs an Axiom, a new public helper, a kernel or surface change,
  or a public type other than the ruled one.
- A change to an existing Order or NatBox law or operation, or a census
  row change.

Checks: 4 fired: the types, placement, proof forms, base and trust count
are read from the AC-0 post and the patch on `372a6a1a6`; the prose
anchors `NatBox.ken.md:29-32` and `:1948` and the two test pins are read
on `372a6a1a6`. 8 fired: the second AC-3 mutant turns the refusal into
an accepted true claim, so a pin that only checks that some block was
refused stays green on it; the first swaps the corner selectors, which a
test that only root-loads names would not catch.
