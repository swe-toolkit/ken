---
id: SPEC-STATISTICS-EVENT-MASS-INCLUSION-EXCLUSION
title: "Spec 63 section 1.3 proves event mass additive only for pointwise-disjoint events, and states no monotonicity: nothing proves that a weaker event has at least the mass of a stronger one, or relates the masses of overlapping events. Rule from a checked D0 inclusion-exclusion and the monotonicity of event mass and expectation weight, at zero trust, and state them as a section 4 with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 63 section 1.3's laws cover only disjoint unions and pointwise-equal events. Statistics is the roadmap layer after graphs (docs/program/06-catalog-campaign.md); non-Lane-B layers are deepened in place. Same shape as SPEC-GRAPHS-CYCLE-REFUTES-ORDER. Steward-filed per COORDINATION section 2."
---

# Event mass is monotone and obeys inclusion-exclusion

## Objective

Spec 63 states, as checked laws over `Algorithm.Statistics.Exact`, that
event mass satisfies inclusion-exclusion for any two events, and that
event mass and expectation weight are monotone under a pointwise
premise.

## Settled inputs (spec 63 §1, on `main`)

- **Operations (§1.1, §1.2).** `event_mass q d p` and
  `expectation_weight q d f` are natural-valued, and
  `event_union q p r x = bool_or (p x) (r x)`. There is no public
  intersection builder.
- **Laws today (§1.3).** `disjoint_mass` needs the checked pointwise
  hypothesis `(x : q) → Equal Bool (bool_and (p x) (r x)) False`.
  `event_mass_ext` and `expectation_weight_ext` take pointwise
  equality, with no function extensionality.
- **Repeated atoms (§1.3).** Each occurrence contributes its weight
  once, so a law must hold on a distribution that lists a vertex twice.
- **Package.** `catalog/packages/Algorithm/Statistics/Exact.ken.md`,
  tested by `crates/ken-elaborator/tests/cat_statistics_exact_distribution.rs`,
  with seed `conformance/stdlib/statistics/seed-exact-distribution.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

A new spec 63 section stating each ruled law by its public type, with a
closed conformance seed under `conformance/stdlib/statistics/`.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed §1 base,
  with rc 0 and no `Axiom`. It rules:
  - inclusion-exclusion as a `Nat` equation with no disjointness
    premise, and whether the intersection is a new public builder or
    is spelled inline;
  - monotonicity of `event_mass` under a pointwise implication, and of
    `expectation_weight` under a pointwise `leq_nat`, each with the
    premise in delivered vocabulary;
  - which helpers are public laws and which stay private.
- **AC-1.** The section states each ruled law by its public type, and
  §1.4's boundary text is updated to match. The Spec leader and CV
  review it, with a Decision.
- **AC-2.** The seed checks inclusion-exclusion on a closed distribution
  with overlapping events and a repeated atom, has a disjoint positive
  control that agrees with `disjoint_mass`, and has a refused block
  that claims monotonicity from a premise that fails at one atom.

## Stop conditions

- A law that needs subtraction, division, a `DecEq q` or a premise
  beyond §1's data, unless the D0 rules it as the stated premise.
- Any new primitive, kernel or surface feature.
- An ambient name in the package beyond its current census row
  (`lang_mod_strict_resolution_d0`): import it explicitly.
