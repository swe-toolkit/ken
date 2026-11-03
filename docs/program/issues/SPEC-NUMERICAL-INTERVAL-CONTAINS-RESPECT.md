---
id: SPEC-NUMERICAL-INTERVAL-CONTAINS-RESPECT
title: "Spec 68 §1.1 says interval membership is a question about rational values, not endpoint spellings, and interval_point_sound returns rational_equiv, yet the package exports no law that membership respects rational_equiv. A client that knows 1/2 is a member and 2/4 is the same value must re-prove membership from the endpoint order, through a private helper it cannot import. Rule from a checked D0 at zero trust a public respect law for interval_contains, with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Numerical follows Graphs in the layer rotation. Steward-filed per COORDINATION section 2."
---

# Interval membership respects rational value equality

## Objective

`Algorithm.Numeric.RationalInterval` exports a public checked law that,
for every `i : Interval` and `x`, `y : Rational`:

```ken
rational_equiv x y
  → IsTrue (interval_contains i x)
  → IsTrue (interval_contains i y)
```

Spec 68 states it by its public type.

## Settled inputs (main `ae649ede7`)

- **The neighbours exist.** In `RationalInterval.ken.md`:
  - `interval_contains` (`:172`) is
    `bool_and (rational_leq lo x) (rational_leq x hi)`;
  - `interval_point_sound` (`:148`) concludes `rational_equiv x y`
    from membership in `interval_point x`, so the converse direction
    needs this law;
  - the private `equiv_leq` (`:1301`) turns `rational_equiv` into
    `rational_leq`, and `proof intro for bool_and` (`:142`) rebuilds
    membership.
- **The enabler.** `Data.Numeric.Rational` exports `proof respect for
  rational_leq` (`Rational.ken.md:225`). It takes
  `rational_equiv x xp` and `rational_equiv y yp` to
  `Equal Bool (rational_leq x y) (rational_leq xp yp)`. Both endpoint
  comparisons transport through it with the endpoint's own `refl`.
- **The rule it states.** Spec 68 §1.1 (`68-numerical.md:105`) says
  that `interval_point (1/2)` and `interval_point (2/4)` contain the
  same values, and that the value equality is `rational_equiv`.
- **Not yet stated.** `spec/`, `catalog/` and `conformance/` were
  grepped for a membership respect law, with no hit beyond the private
  `equiv_leq`. The package exports 46 public theorems. Spec 68 runs to
  §11 (`:1106`, file end `:1183`).
- **Seeds.** Under `conformance/stdlib/numerical/`, next to
  `seed-interval-point-units.md`, whose `[1/2, 1]` values can be
  reused.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

The ruled law in `RationalInterval`, a spec 68 section stating it by
its public type, a closed conformance seed under
`conformance/stdlib/numerical/`, the public-inventory pin and the
package's law count advanced, and the status lines, all in one
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
  - whether the converse of `interval_point_sound`
    (`rational_equiv x y → IsTrue (interval_contains (interval_point x)
    y)`) is exported in the same candidate, or left as a composition
    with `interval_point_contains`;
  - whether the private `equiv_leq` is restated through the new law or
    left as it is.

  It confirms that no §1 to §11 operation or law type changes.
- **AC-1.** The section states each ruled law by its public type. The
  Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client takes `1/2` as a member of
  `[1/2, 1]`, and moves membership to `2/4` through the law.
  - Its refused block pins `2/6` as a member of `[1/2, 1]`. That is
    false, because `2/6` is the value `1/3`, which is below `1/2`.
  - Its accepted same-form control pins `2/4` as a member, so the
    refusal is the value, not the form.
  - The D0 rules the proof form that separates the two rows.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or any premise
  beyond the equivalence and the membership.
- A change to an existing §1 to §11 operation or law type.

Checks: 4 fired: the law is written from public `interval_contains`,
`rational_equiv` and Rational's `respect for rational_leq`, with the
private `equiv_leq`, each read at its line; spec, catalog and
conformance were grepped for a membership respect law, with no public
hit. 8 fired: the refused row and its control share the interval, the
operation and the claim form, and differ only in the claimed member,
`2/6` against `2/4`.
