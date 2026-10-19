---
id: CAT-SYMBOLIC-ALGEBRA-SUBSTITUTION
title: "Algorithm.SymbolicAlgebra.NatPolynomial exports spec 66 section 3: compose_env, substitute, eval_substitute and the True-direction poly_eq_substitute. Proved as checked terms at zero trust, from the Architect's D0 development, with the substitution seed run by the package test"
status: ready
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-SYMBOLIC-ALGEBRA-SUBSTITUTION]
blocks: []
github: null
origin: "Operator 2026-09-13: every catalog package fully proved. SPEC-SYMBOLIC-ALGEBRA-SUBSTITUTION (routed 04f47049e) states section 3 and records its laws as checked in the Architect's D0 scratch development, not yet exported. Same shape as CAT-LINEAR-ALGEBRA-PRODUCT-COMPATIBILITY. Steward-filed per COORDINATION section 2."
---

# NatPolynomial exports substitution and its laws

## Objective

`Algorithm.SymbolicAlgebra.NatPolynomial` exports `compose_env`,
`substitute`, `eval_substitute` and `poly_eq_substitute` with exactly
spec 66 §3's public types, each law a checked proof.

## Settled inputs (spec 66 §3, from `04f47049e`)

- **The surface.** §3 gives the four public names and types. The
  composed environment is the named `compose_env`, because a lambda
  cannot sit in a theorem type. `poly_eq_substitute` is the `True`
  direction only.
- **The proofs (Architect `evt_79z6qn2b22b9h`).** `eval_substitute` is
  structural on `e`, using the existing private `add_cong` and a new
  private `mul_cong`. `poly_eq_substitute` is `poly_eq_complete` over
  `eval_substitute`, `poly_eq_sound` at `compose_env env s`, and `sym`.
  The D0 package delta is `local/arch-subst-d0-package.diff.txt`; it
  checked with no Axiom and `trusted_base()` 114 before and after.
- **Seed.** The substitution seed,
  `conformance/stdlib/symbolic-algebra/seed-nat-polynomial-substitution.md`.
  Closed equations close with `Proved`, not `Refl`. The refused block's
  kernel error is a generic mismatch.
- **Package.** `catalog/packages/Algorithm/SymbolicAlgebra/NatPolynomial.ken.md`,
  tested by `crates/ken-elaborator/tests/cat_symbolic_algebra_canonical_equality.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The four exports and `mul_cong` in `NatPolynomial.ken.md`, and test rows
that root-load each new name and run the substitution seed.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass. The test root-loads
  the four names as package-owned and transparent, and runs the seed's
  positive cases and identity control. Its refused block is pinned to
  the uncomposed-`env` proposition built from the positive operands, so
  an unrelated false equation does not satisfy it.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom or
  primitive. `lang_mod_strict_resolution_d0`'s
  `catalog_ambient_passthrough_migration_census` row for the package is
  unchanged, measured on the candidate.
- **AC-3 (mutation, QA).** Each mutant fails, and is restored:
  - `substitute` leaving `EVar v` unreplaced fails `eval_substitute`;
  - `compose_env` evaluating `s v` under a shifted `env` fails the
    seed's composed-evaluation case;
  - an unrelated false closed equation in the refused block fails the
    proposition pin.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a public type
  other than §3's.
- A census row, or a consumer of `NatPolynomial` outside this package,
  changes.
