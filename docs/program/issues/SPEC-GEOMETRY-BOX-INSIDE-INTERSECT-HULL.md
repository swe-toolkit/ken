---
id: SPEC-GEOMETRY-BOX-INSIDE-INTERSECT-HULL
title: "Spec 67 orders boxes corner-wise by `inside` (§5, §12) and gives intersect and hull their lattice laws (§6 to §9, §13), but no checked law connects the order to the operations: nothing says a box inside another is its own intersection with it, or that the outer box is their hull. Rule from a checked D0 at zero trust the two public consistency laws, with the Nat Order selector laws they need and a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-OPTIMIZATION-BOUNDED-SEARCH-AGREE]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Geometry follows Optimization in the layer rotation. Steward-filed per COORDINATION section 2."
---

# The order agrees with the lattice

## Objective

`Algorithm.Geometry.NatBox` exports two public checked laws. For every
`b`, `c : Box`, if `inside b c` holds, then:

```ken
Equal Box (intersect b c) b
Equal Box (hull b c) c
```

Spec 67 states both by their public types.

## Settled inputs (main `aae4ffa6f`)

- **The operations.** `intersect` (`NatBox.ken.md:87`) takes `max` of
  the low corners and `min` of the high corners; `hull` (`:110`) takes
  the reverse. `inside` (`:1514`) is `axis_within` on each axis
  (`:2237`): `leq_nat l2 l1` and `leq_nat h1 h2`. Both are corner-wise,
  so the laws need no validity premise.
- **The neighbouring laws.** `intersect_greatest` (`:1998`) and
  `hull_least` (`:2064`) give the order's bounds. `intersect_idempotent`
  (`:663`) and `hull_idempotent` (`:678`) are the `b = c` case. No law
  concludes an equation from `inside`.
- **The Nat Order gap.** `Data/Numeric/Nat/Order.ken.md` exports `min`
  (`:58`) and `max` (`:182`) with bounds, comm, assoc, absorb and
  distrib, but no law that `leq_nat m n` makes `min m n` equal `m` or
  `max m n` equal `n`. Spec 67 §6.1 is the precedent for adding Nat
  Order selector laws from a Geometry node.
- **The spec.** Spec 67 runs §1 to §15 (file end `:1335`); §15
  (`:1282`) is the latest section.
- **Seeds and tests.** Under `conformance/stdlib/geometry/`. The package
  test pattern is
  `crates/ken-elaborator/tests/cat_geometry_box_inside_antisym.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

The ruled laws in NatBox and any Nat Order selector laws they need, a
spec 67 section stating them by their public types, a closed
conformance seed, a named package test, and spec 67's status lines, all
in one candidate.

## Acceptance

- **AC-0 (D0).** The Architect specifies a development on the landed
  base: the source, and the named-binary commands with a bounded
  timeout. The Foundation implementer runs it on scratch and reports
  rc, the `Axiom` grep and `trusted_base()` as an identity set before
  and after. The Architect rules from that report, and does not build
  or test. It rules the law names, their placement, the section
  number, and whether the Nat Order selector laws are public. It
  confirms that no §1 to §15 operation or law type changes.
- **AC-1.** The section states the ruled laws by their public types.
  The Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies each law to a closed box
  inside another. Its refused block claims the same intersect equation
  for a closed outer box that does not contain the inner one on one
  axis. Its accepted same-form control differs only in that
  coordinate, so the refusal is the failed premise, not the form.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a premise
  beyond `inside`.
- A change to an existing §1 to §15 operation or law type.

Checks: 4 fired: `intersect`, `hull`, `inside` and `axis_within` were
read at their lines, and `min`/`max` in Nat Order at `:58` and `:182`.
3 fired: NatBox's public laws and Nat Order's `min`/`max` proofs were
listed; none concludes an equation from an order premise. 8 fired: the
refused row and its control differ in one coordinate of the outer box.
