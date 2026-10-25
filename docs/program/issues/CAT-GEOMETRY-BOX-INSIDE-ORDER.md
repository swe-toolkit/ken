---
id: CAT-GEOMETRY-BOX-INSIDE-ORDER
title: "Algorithm.Geometry.NatBox exports spec 67 section 5: inside_trans, intersect_greatest and hull_least over corner-wise inside, proved at zero trust from the Architect's D0, with the section 5 seed run by a package test"
status: active
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-GEOMETRY-BOX-INSIDE-ORDER]
blocks: []
github: null
origin: "Catalog half of SPEC-GEOMETRY-BOX-INSIDE-ORDER (merged 6e5faa70c). Same shape as CAT-GEOMETRY-BOX-AREA. Steward-filed per COORDINATION section 2."
---

# NatBox exports the inclusion-order laws

## Objective

`Algorithm.Geometry.NatBox` exports `inside_trans`,
`intersect_greatest` and `hull_least` with exactly spec 67 §5.1's
public types, each a checked proof.

## Settled inputs (spec 67 §5 and its seed, from `6e5faa70c`)

- **The surface.** §5.1 states the three laws for every `Box`,
  including inverted boxes, with `IsTrue (inside ...)` premises and
  conclusions only. No point-set or `Equal Box` statement.
- **The proof shape.** §5.2: the per-axis `axis_within_trans`,
  `axis_within_meet` and `axis_within_join` are private, as
  `axis_within` already is. They use the public §1.4 bounds
  `greatest for min` and `least for max` and `leq_nat` transitivity,
  then `bool_and` introduction across the two axes.
- **The D0.** The Architect's checked D0 on `4f337ec26`
  (`evt_208zkb44spn9r`): `ken check` and `ken fmt --check` rc 0, no
  new Axiom, `trusted_base()` 114 to 114 over four providers.
- **Seed.** `conformance/stdlib/geometry/seed-box-inside-order.md`
  covers each law on closed boxes with an inverted premise, and refuses
  the reverse inclusions against same-operand controls. It must be on
  `main` before this WP starts. It has not been run against shipped
  exports.
- **Package and tests.** `NatBox.ken.md` exports `inside` and its
  seven §3.2 laws (`:819-1046`) and §4's area. They are tested by
  `cat_geometry_nat_box.rs`, `cat_geometry_manhattan_metric.rs` and
  `cat_geometry_box_area.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The three exports and the three private helpers in `NatBox.ken.md`, the
spec 67 §5 status lines updated to say the exports are delivered, and a
new `crates/ken-elaborator/tests/cat_geometry_box_inside_order.rs` that
root-loads each new public name and runs the seed. Every seed
execution-status statement changes in the same candidate to say the
package test checks it, with no conformance-runner, CI or landing
claim, and no fence or fixture change.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass. The test root-loads
  the three names as package-owned and transparent, keeps the four
  per-axis helpers private, and runs the seed's positive cases. Its
  refused blocks are pinned to the seed's propositions. The three
  predecessor suites still pass. The Spec leader reviews the spec/
  status lines, and CV the seed's.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom
  or primitive. The census row for `Algorithm.Geometry.NatBox` is
  unchanged, measured on the candidate.
- **AC-3 (mutation, QA).** Each mutant fails, and is restored:
  - `axis_within_meet` proved against `max` and `min` swapped, or
    `intersect_greatest` stated with one premise dropped, fails check;
  - `hull_least` concluding `inside (hull b c) b` fails check or the
    seed's refused hull block;
  - `inside_trans` with its premises reversed (`inside b a`) fails
    check.

## Stop conditions

- A law needs an Axiom, a validity premise, a canonical empty box,
  point-set equality, a kernel or surface change, or a public type
  other than §5.1's.
- A change to `inside`, `intersect`, `hull`, `area` or their laws, a
  new public Nat order lemma, or a census row change.

Checks: 4 fired: §5.1's three types and §5.2's three private helpers
are read from `6e5faa70c`; the three predecessor test files are on
`main` at `4f337ec26`. 7 fired: AC-3's mutants each drop or reverse
the premise that §5.2's closed counterexamples show is necessary.
