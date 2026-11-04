---
id: SPEC-GEOMETRY-BOX-POINT-SINGLETON
title: "Spec 67 §14 says every point box contains its own point, but no public law says it contains only that point. A client holding `contains (MkBox p p) q` cannot conclude `q = p`, so it cannot derive from public laws that a box containing a point includes that point's box, or that such a box has positive area. Rule from a checked D0 at zero trust a public singleton law, with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-OPTIMIZATION-BOUNDED-SEARCH-LEAST-HIT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Geometry follows Optimization in the layer rotation. Steward-filed per COORDINATION section 2."
---

# A point box holds only its point

## Objective

`Algorithm.Geometry.NatBox` exports a public checked law that, for
every `p`, `q : Point`:

```ken
IsTrue (contains (MkBox p p) q) → Equal Point q p
```

With `contains_point_box` (§14), it states that a point box is exactly
the singleton of its point. Spec 67 states it by its public type.

## Settled inputs (main `fa36c20ef`)

- **The forward law.** `contains_point_box` (`NatBox.ken.md:1166`)
  gives `IsTrue (contains (MkBox p p) p)`. No public law concludes an
  `Equal Point` from membership.
- **The proof steps.** `contains` is two `axis_contains` conjunctions,
  each `bool_and (leq_nat lo x) (leq_nat x hi)` (spec 67 §1.1). For a
  point box each axis gives both `leq_nat` directions, and
  `(proof antisym for leq_nat)` (as used by the private
  `axis_within_antisym_low`, `:2260`) concludes each coordinate. The
  private `point_equal` (`:1368`) joins the two coordinates.
- **The derived uses.** With the law, `inside_complete` (`:1708`) at
  witness `p` gives `contains c p → inside (MkBox p p) c`; then
  `area_inside` (`:2108`) and `area_point_box` (`:2186`) give
  `leq_nat (Suc Zero) (area c)`. Neither corollary is public today.
- **Not yet stated.** Spec 67 runs §1 to §14 (`67-geometry.md`, §14
  at `:1230`, file end `:1278`). §1.1 keeps `Eq Box` structural, and
  this law is about membership, not box equality.
- **Seeds.** Under `conformance/stdlib/geometry/`, next to
  `seed-box-point.md`, whose `off` `(3,5)` and `other` `(3,4)` can be
  reused. The package test is
  `crates/ken-elaborator/tests/cat_geometry_box_point.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

The ruled law in NatBox, a spec 67 section stating it by its public
type, a closed conformance seed under `conformance/stdlib/geometry/`,
a named package test, and spec 67's status lines, all in one
candidate.

## Acceptance

- **AC-0 (D0).** The Architect specifies a development on the landed
  base: the source, and the named-binary commands with a bounded
  timeout. The Foundation implementer runs it on scratch and reports
  rc, the `Axiom` grep and `trusted_base()` as an identity set before
  and after. The Architect rules from that report, and does not build
  or test. It rules:
  - the law's name, argument order and placement;
  - the section number;
  - whether either derived use (membership as point-box inclusion,
    positive area of a box with a member) is also exported.

  It confirms that no §1 to §14 operation or law type changes.
- **AC-1.** The section states each ruled law by its public type. The
  Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the law generically,
  for any `q` contained in `MkBox off off`, concluding `Equal Point q
  off`.
  - Its refused block claims the same conclusion, `Equal Point other
    off`, for `other` in the two-point box `MkBox other off`. That is
    false, because `other` is a member and differs from `off` in `y`.
  - Its accepted same-form control claims `Equal Point off off` for
    `off` in `MkBox off off`, so the refusal is the second member, not
    the form.
  - The D0 rules the proof form that separates the two rows.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or any premise
  beyond the membership.
- A change to an existing §1 to §14 operation or law type.

Checks: 4 fired: the law is written from public `contains`,
`contains_point_box`, `inside_complete`, `area_inside` and
`area_point_box`, each read at its line, and the proof steps from the
private `point_equal` and the `leq_nat` antisymmetry proof that
`axis_within_antisym_low` already uses. 3 fired: NatBox's 46 public
names and spec 67 §1 to §14 were read for a law concluding `Equal
Point` from membership; none exists. 8 fired: the refused row and its
control share the claimed point `off`, the box's upper corner and the
claim form, and differ only in the box's lower corner, `other` against
`off`.
