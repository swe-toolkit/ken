---
id: CAT-STATISTICS-EXACT-DISTRIBUTION
title: "No catalog package gives an exact finite distribution, event mass or expectation. Deliver section 1 of spec/50-stdlib/63-statistics.md as Algorithm.Statistics.Exact: a natural-weight atom list with a checked positive total, event mass and natural-score expectation, and nine laws, fully proved at zero trust"
status: active
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-STATISTICS-EXACT-DISTRIBUTION-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. 06-catalog-campaign.md:375, statistics/probability after graphs. Spec section 1 from SPEC-STATISTICS-EXACT-DISTRIBUTION-CONTRACT (Architect D0 evt_1a2b2fhb035nw). Steward-filed per COORDINATION section 2."
---

# Exact distributions, delivered and proved

## Objective

`Algorithm.Statistics.Exact` lands as section 1 of
`spec/50-stdlib/63-statistics.md` specifies, with every law proved and no
new trust.

## Settled inputs (spec at `d6605d7db`)

- **Section 1 is normative.** It names 20 public declarations:
  `ExactDistribution` with the exported `MkExactDistribution`; `entries`,
  `one_less_total`, `total`, `event_mass`, `expectation_weight`; the
  builders `always_true`, `always_false`, `event_union`, `constant_one`,
  `score_add`; and the laws `total_positive`, `impossible_event`,
  `sure_event`, `complement_mass`, `disjoint_mass`, `expectation_unit`,
  `expectation_linearity`, `event_mass_ext` and `expectation_weight_ext`.
- **The development exists.** The ruled D0 scratch (sha256 prefix
  `8ba4809e`) checked every declaration and law rc=0 with no Axiom, and
  each congruence-premise mutant kernel-rejected. It is the starting
  point, not a design to revisit.
- **Reuse.** Arithmetic is `Data.Numeric.Nat.Arithmetic`. Complement is
  `Core.Classes.EffectfulClasses.compose q Bool Bool bool_not p`, not a
  new helper. There is no `Finite q`, `DecEq q` or ratio type.
- **The seed** is `conformance/stdlib/statistics/seed-exact-distribution.md`:
  eight cases with Nat cross-multiplication oracles.
- **The oracle.** Seed results are closed `Equal` theorems checked by
  kernel reduction, as in `CAT-GRAPHS-DEPENDENCY`. Reference eval may be
  used only where it returns a constructor; Unknown is never a pass.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. The package, in section 1's order, reading top-down, with its own
   acceptance test target and a strict-resolution ambient-census row.

## Acceptance

- **AC-1.** `ken fmt --check` and `ken check` pass. The acceptance
  target root-loads all 20 public names and runs the eight seed cases
  with their listed results.
- **AC-2.** `trusted_base()` is equal before and after. The package
  declares no Axiom, postulate, primitive, foreign declaration or
  `Float`.
- **AC-3 (consumers).** No existing package's public surface changes. A
  whole-root sweep over `catalog/`, `crates/*/tests`, `r_layer_tests`,
  `examples/`, `conformance/` and the CLI fixtures finds every pin or
  importer of the Nat arithmetic and `EffectfulClasses` modules, and
  keeps each green. The Rosetta runner and SEAL-2 pass.
- **AC-4 (mutation, QA).** Each mutant makes a proof or seed oracle
  kernel-reject, and is restored: `event_mass` counting an atom whose
  event is false; `expectation_weight` ignoring the weight; a
  constructor accepting an all-zero list.

## Stop conditions

- A law needs an Axiom, `Finite q`, `DecEq q`, a ratio type, or a kernel
  or surface-syntax change.
- Reuse needs a change to a Nat arithmetic or `EffectfulClasses` public
  surface.
- A seed case fails on the delivered package.
