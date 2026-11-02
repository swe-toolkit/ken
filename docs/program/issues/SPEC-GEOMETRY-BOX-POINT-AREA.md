---
id: SPEC-GEOMETRY-BOX-POINT-AREA
title: "Spec 67 §4.1 says a point box has area one, and the area seed checks one closed point box, but no public law states it. A client that needs the area of `MkBox p p` must unfold `area` and reach the private `axis_span_point`. Rule from a checked D0 at zero trust and state the law with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Geometry follows Optimization in the layer rotation. Steward-filed per COORDINATION section 2."
---

# A point box has area one

## Objective

Spec 67 states, as a public checked law over `Algorithm.Geometry.NatBox`,
for every `p : Point`:

```ken
Equal Nat (area (MkBox p p)) (Suc Zero)
```

## Settled inputs (main `a54bf0d04`)

- **The neighbours exist.** In `NatBox.ken.md`:
  - `area` (`:2077`) is the product of two inclusive axis spans;
  - the private `axis_span` (`:2460`) is `sub (Suc hi) lo`;
  - the private `axis_span_point` (`:2454`) concludes
    `Equal Nat (axis_span n n) (Suc Zero)`;
  - `area_inverted_x` and `area_inverted_y` (`:2134`, `:2153`) are the
    public precedent for an `Equal Nat (area …)` law on a constructed
    box.
  - `cong` and `trans` (`:57`) and `mul` (`:52`) are already imported.
- **Not yet stated.** §4.1 (`67-geometry.md:461`) says in prose that a
  point box has area one, and `seed-box-area.md` checks one closed
  point box. No public theorem concludes it. The package, spec 67 and
  the geometry seeds were grepped for `area_point` and for a point-box
  area law, with no hit. Spec 67 runs to §13 (`:1187`).
- **Seeds.** Under `conformance/stdlib/geometry/`, next to
  `seed-box-area.md`, whose fixtures `n0` to `n9` can be reused.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A spec 67 subsection stating the ruled law by its public type, the
catalog theorem in `NatBox`, a closed conformance seed under
`conformance/stdlib/geometry/`, the public-inventory pin advanced, and
the status lines, all in one candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged as an identity
  set, or that measurement assigned to the candidate's package test. It
  rules:
  - the theorem name, argument order and placement;
  - the subsection number;
  - whether `axis_span_point` stays private;
  - whether a point-box self-containment twin,
    `IsTrue (contains (MkBox p p) p)`, travels in this candidate or as
    its own node.

  It confirms that no §1 to §13 operation or law type changes.
- **AC-1.** The subsection states each ruled law by its public type.
  The Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the law at the origin
  and at a point with distinct nonzero coordinates.
  - Its refused block pins the area of a point box as `Zero`. The area
    is one, so the claim is false.
  - Its accepted same-form control is the same box's area as
    `Suc Zero`, so the refusal is the value, not the form.
  - The D0 rules the proof form that separates the two rows.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or any premise.
- A change to an existing §1 to §13 operation or law type.

Checks: 4 fired: the law is written from public `area` and `MkBox`, and
the private `axis_span` and `axis_span_point`, each read at its line,
with `cong`, `trans` and `mul` already imported; the package, spec 67
and the geometry seeds were grepped for a point-box area law, with no
hit. 8 fired: the refused row and its control share the box and the
claim form and differ only in the claimed value, zero against one.
