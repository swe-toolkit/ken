---
id: CAT-GEOMETRY-NAT-BOX
title: "Spec 67 section 1 specifies Algorithm.Geometry.NatBox with six proved laws, but no catalog package delivers it. Deliver the package from the Architect's checked D0 at zero trust, publish the two Nat.Order bounds its proof needs, and run its seed as an acceptance target"
status: active
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-GEOMETRY-EXACT-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 67 section 1 from SPEC-GEOMETRY-EXACT-CONTRACT (Architect D0 evt_3546vedrwbdqx, review evt_fncq5h32b7pf). Same shape as CAT-SYMBOLIC-ALGEBRA-NAT-POLYNOMIAL. Steward-filed per COORDINATION section 2."
---

# Exact boxes over natural coordinates, delivered and proved

## Objective

`Algorithm.Geometry.NatBox` exists in `catalog/`, matches spec 67 section
1 token for token, and proves all six laws with no Axiom. A client outside
the module can build points and boxes and use every operation.

## Settled inputs (on `813346f95` plus the spec chapter)

- **The contract.** Spec 67 section 1 fixes:
  - the carriers `Point` and `Box`, with an empty box written as inverted
    corners;
  - the operations `axis_contains`, `contains`, `intersect`, `hull` and
    `manhattan`;
  - the six laws `intersect_sound`, `intersect_complete`,
    `hull_contains_left`, `hull_contains_right`, `manhattan_symm` and
    `manhattan_self`, with exact signatures in section 1.3.
- **A checked development exists.** The Architect's D0 checks all six
  generic laws together, rc=0, with no Axiom in the geometry source.
- **Two bounds must become public** (section 1.4).
  `Data.Numeric.Nat.Order` keeps `greatest for min` and `least for max`
  private, and `intersect_complete` needs both. With them private the
  development fails `UnboundName` at `max::least`. Adding `pub` to both
  declarations is enough, so do not clone their proofs.
  `tightness_laws_stay_private_without_new_trust` in
  `crates/ken-elaborator/tests/cat_nat_min_max_bound_laws.rs` pins their
  privacy and must change with them.
- **Constructor exports are required.** `pub data` publishes only the type
  (`33 §4.1`). The chapter adds `export MkPoint` and `export MkBox`, and a
  client checked only with both.
- **A new catalog leaf joins the WP-4 ambient census.**
  `catalog_ambient_passthrough_migration_census` in
  `crates/ken-elaborator/tests/lang_mod_strict_resolution_d0.rs` pins
  every package's ambient dependencies. CAT-SYMBOLIC's first candidate went
  red on CI for lacking its row, so add the measured row for the new
  package.
- **The seed** is `conformance/stdlib/geometry/seed-nat-box.md`. Its
  positive block has 22 assertions, and it has five refusals.
- **Known limits.**
  - Pass `map` a named helper, not a λ
    (`LANG-ELABORATED-BETA-REDEX-AT-KERNEL-INFERENCE`).
  - A dependent `match` refines only its scrutinee as written (CHECKS 11).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. The package at `catalog/packages/Algorithm/Geometry/NatBox.ken.md`. It
   holds the D0's declarations and proof bodies, the two export lines, and
   the section 1.3 laws as public theorems.
2. `pub` on `greatest for min` and `least for max` in
   `Data.Numeric.Nat.Order`. The privacy test is updated to pin that both
   are now reachable and that Nat Order still adds no trust.
3. An acceptance target in `crates/ken-elaborator/tests/` that:
   - loads the package;
   - runs the seed's positive block as a client import;
   - refuses all five negatives;
   - restates the six law types generically;
   - records `trusted_base()` before and after loading.
4. The package's measured row in the WP-4 ambient census.

## Acceptance

- **AC-1.** The package checks, every section 1.3 law is a checked proof
  body, and `trusted_base()` does not grow on loading it or `Nat.Order`.
- **AC-2.** The seed's 22 positive assertions pass from a client module,
  and all five refusals are refused. The census test passes with the new
  row and every other row unchanged.
- **AC-3 (mutation, QA).**
  - Deleting the vertical condition from `contains` (the seed's one-line
    mutation) makes the package's own proofs or the seed's one-box refusal
    fail.
  - Removing the constructor exports reddens the client import.

## Stop conditions

- A law needs an Axiom, or a bound proof must be cloned instead of
  published.
- `trusted_base()` grows on loading either package.
- A change outside the package, `Nat.Order`, the two tests named above and
  the new acceptance target.
