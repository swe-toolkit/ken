---
id: SPEC-SYMBOLIC-ALGEBRA-SUBSTITUTION-COMPOSITION
title: "Spec 66 section 3 defers composition of substitutions: nothing states that substituting twice is one substitution by the composed map, so a client cannot fuse two rewrites. Rule a named composition builder, the structural composition law, and whether an identity law holds, from a checked D0 at zero trust, and state them as a section 4 with a conformance seed"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 66 section 3 ends with \"Composition of substitutions remains deferred.\" Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md). Same shape as SPEC-TIME-WINDOW-MERGE-DISTRIBUTION. Steward-filed per COORDINATION section 2."
---

# Substituting twice is one composed substitution

## Objective

Spec 66 states a checked law that `substitute s (substitute t e)` is
`substitute` of the composition of `s` and `t` applied to `e`, as
structural `Equal Expr`.

## Settled inputs (spec 66 §3, on `main`)

- **Substitution (§3).** `substitute s e` replaces each variable of
  the original `Expr` by `s v`, keeps the constructors and operand
  order, and does not substitute an inserted `s v` again. It is a
  single simultaneous substitution.
- **Named builders.** Ken's theorem types cannot take an inline lambda
  as a function argument here, so §3 makes `compose_env` a public named
  builder. A composition of substitutions needs the same.
- **Laws today.** `eval_substitute` and `poly_eq_substitute`. §3 says
  "Composition of substitutions remains deferred."
- **Package.** `catalog/packages/Algorithm/SymbolicAlgebra/NatPolynomial.ken.md`.
  The seeds are under `conformance/stdlib/symbolic-algebra/`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

A new spec 66 section stating each ruled builder and law by its public
type, the §3 deferral sentence corrected, and a closed conformance seed
under `conformance/stdlib/symbolic-algebra/`, with its status lines
changed in the same candidate (Steward `evt_5rtkgkdf42qb`).

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0 and no `Axiom`. It rules:
  - the composition builder's name and definition, and the composition
    law's name and statement as `Equal Expr`, for every pair of
    substitutions and every expression;
  - whether `substitute` by the variable constructor is the identity as
    a public law, and whether it needs its own named builder;
  - whether an evaluation corollary through `compose_env` is a public
    law or follows from `eval_substitute`;
  - the private helpers the proof needs.
- **AC-1.** The section states each ruled builder and law by its public
  type, and the deferral sentence is corrected. The Spec leader and CV
  review it, with a Decision.
- **AC-2.** The seed checks composition on a closed pair of
  substitutions where the inner one inserts a variable that the outer
  one replaces, so that composing in the other order gives a different
  expression. It has a refused block that claims the reversed order.

## Stop conditions

- A law that needs function extensionality, a decidable equality on
  `Expr` stated as `Equal`, or normalization, unless the D0 rules it as
  the stated premise.
- Any new primitive, kernel or surface feature.
- An ambient name in the package beyond its current census row
  (`lang_mod_strict_resolution_d0`): import it explicitly.
