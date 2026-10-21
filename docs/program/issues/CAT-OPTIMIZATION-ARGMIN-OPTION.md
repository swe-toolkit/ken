---
id: CAT-OPTIMIZATION-ARGMIN-OPTION
title: "Algorithm.Optimization.Argmin exports spec 72 section 1.4: the selector argmin_option over a possibly empty List, its Nil and Cons equations, and the membership and least-cost corollaries, proved at zero trust from the Architect's D0 development, with the section 1.4 seed run by a package test"
status: ready
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-OPTIMIZATION-ARGMIN-OPTION]
blocks: []
github: null
origin: "Operator 2026-09-13: every catalog package fully proved. SPEC-OPTIMIZATION-ARGMIN-OPTION states section 1.4 (1cffbef58) and its seed (408fa626d) from the Architect's checked D0 scratch development, not yet exported. Same shape as CAT-SYMBOLIC-ALGEBRA-SUBSTITUTION-COMPOSITION. Steward-filed per COORDINATION section 2."
---

# Argmin exports the Option selector

## Objective

`Algorithm.Optimization.Argmin` exports `argmin_option`,
`argmin_option_nil`, `argmin_option_cons`, `argmin_option_member` and
`argmin_option_least` with exactly spec 72 §1.4's public types, each
law a checked proof.

## Settled inputs (spec 72 §1.4 and its seed, on `main`; Architect D0)

- **The surface.** §1.4 states the definition (`Nil ↦ None a`;
  `Cons x rest ↦ Some a (argmin a cost (nonempty_cons a x rest))`), the
  two selection equations, and the two corollaries over `list_elem` on
  the input list. No equality or order on `a` is a premise.
- **The D0.** Scratch commit `04f60997a`, on base `1d82fd8ec`:
  `catalog/packages/Algorithm/Optimization/ArgminOptionD0.ken.md`, with
  the four private helpers `option_member`, `option_least`,
  `argmin_option_member_base` and `argmin_option_least_base`. Checked
  rc 0 with no Axiom, Opaque or postulate; a false `Some` on `Nil` is
  refused with E0038 (`/workspaces/ken/local/arch-amo-d0/`).
- **Seed.** `conformance/stdlib/optimization/seed-argmin-option.md`,
  from SPEC-OPTIMIZATION-ARGMIN-OPTION AC-2. It must be on `main`
  before this WP starts. It has not been executed.
- **Package.** `catalog/packages/Algorithm/Optimization/Argmin.ken.md`.
  §1.1 to §1.3 are tested by
  `crates/ken-elaborator/tests/cat_optimization_argmin.rs`. The census
  row in `lang_mod_strict_resolution_d0.rs` lists nine ambient names
  for this module.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The five exports and the four private helpers in `Argmin.ken.md`, the
spec 72 §1.4 status lines updated to say the exports are delivered, and
a new `crates/ken-elaborator/tests/cat_optimization_argmin_option.rs`
that root-loads each new public name and runs the seed. Every seed
execution-status statement changes in the same candidate to say the package test
executes it, with no conformance-runner, CI or landing claim, and no
fence or fixture change (Steward `evt_17wqzjhzc75vk`, `evt_72a93pyhmv8xc`).

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass. The test root-loads
  the five names as package-owned and transparent, keeps the helpers
  private, and runs the seed's positive cases: `Nil`, the singleton, and
  the earliest least-cost element that is neither the head nor the last
  tie. Its refused block is pinned to the seed's `Some`-on-`Nil`
  proposition, and the accepted `Nil` control passes. The §1.1 to §1.3
  suite still passes. The Spec leader reviews the spec/ status lines,
  and CV the seed's.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom or
  primitive. The census row for `Algorithm.Optimization.Argmin` is
  unchanged, measured on the candidate.
- **AC-3 (mutation, QA).** Each mutant fails, and is restored:
  - `Cons x rest ↦ Some a x` (the head) fails the seed's tie case;
  - `Cons x rest ↦ None a` fails `argmin_option_cons` or the seed;
  - an unrelated false closed proposition in the refused block fails
    the pin.

## Stop conditions

- A law needs an Axiom, function extensionality, a decidable equality
  or order on `a`, a kernel or surface change, or a public type other
  than §1.4's.
- A change to `argmin` or its seven §1.2 laws, a census row change, or
  a change to a consumer of `Argmin` outside this package.

Checks: 4 fired: `04f60997a` and `local/arch-amo-d0/` exist, and the
census row and `cat_optimization_argmin.rs` are on `main` at
`8546cb58c`. 7 fired: AC-3 mutates the head and `None` returns that the
seed is built to refuse.
