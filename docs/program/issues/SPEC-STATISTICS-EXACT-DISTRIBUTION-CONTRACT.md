---
id: SPEC-STATISTICS-EXACT-DISTRIBUTION-CONTRACT
title: "No statistics or probability contract exists in spec/50-stdlib, so the catalog's statistics layer has nothing to build against. Author spec/50-stdlib/63-statistics.md with its first section: an exact finite distribution, event probability and expectation, each law proved at zero trust"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\". 06-catalog-campaign.md:375 places statistics/probability (exact/empirical/approximate tiers) after graphs/dependency structures. Same shape as SPEC-GRAPHS-DEPENDENCY-CONTRACT: an Architect boundary ruling from a checked development, then the contract, then the CAT node. Steward-filed per COORDINATION section 2."
---

# An exact finite distribution contract

## Objective

`spec/50-stdlib/63-statistics.md` exists, and its first section is the
normative contract for the exact tier, which the first statistics catalog
node builds against.

## Settled inputs (on `c9a025f09`)

- **Finite evidence exists.** `Data.Finite.Finite` (spec 61 section 2.1)
  is a certificate over any carrier, with no `DecEq` in the record.
- **Natural arithmetic is proved.** `Data.Numeric.Nat.Arithmetic` (`add`,
  `mul`, their laws) and `Data.Numeric.Nat.Order` exist, and
  `Algorithm.Numeric.Gcd` proves `divides_gcd`.
- **No rational type exists.** Nothing in `catalog/` or `spec/50-stdlib`
  declares a rational number. `Int` and `Float` are audited primitives.
- **No layer slot is fixed.** `06-catalog-campaign.md` names the layer,
  not its Section. The package imports no `Capability.*`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (Architect boundary ruling, measured).** Rule from a checked
   development (rc=0, no Axiom) on:
   - the package's Section and module path;
   - the representation of an exact distribution over a `Finite`
     carrier, and how a probability is stated: natural weights with laws
     by cross-multiplication, or a rational type. A rational package is
     in this scope only if the Architect places it here; otherwise it is
     a prerequisite node the D0 names;
   - the operations and their exact laws: at least the total weight,
     the probability of a decidable event, complement and disjoint
     additivity, and expectation of a natural-valued function;
   - whether `pure` and `bind` with the monad laws belong in section 1;
   - what stays deferred: the empirical and approximate tiers,
     conditional probability, variance, continuous distributions and any
     `Float` result.
2. **Section 1, normative** for every ruled declaration and law, each
   stated as a proved theorem, with the trust boundary: no Axiom,
   primitive, `Float`, kernel form or `trusted_base()` entry. Index the
   chapter in `spec/50-stdlib/README.md` and `spec/SPEC-PROGRESS.md`.
3. **A conformance seed** under `conformance/stdlib/` in the existing
   seed shape: a uniform distribution, a skewed one, a sure and an
   impossible event, a complement pair, and one expectation.

## Acceptance

- **AC-1.** Every declaration and law the CAT node must deliver is named
  with its type, and nothing required is outside the Architect's ruling.
- **AC-2.** Conformance-validator vote on the exact SHA of the assembled
  scope: the spec paths plus the seed.

## Stop conditions

- The contract needs a kernel, trust, `Float` or surface-syntax change.
- A law cannot be stated without a type that does not exist and that the
  D0 does not place in this scope.
