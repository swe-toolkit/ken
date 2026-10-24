---
id: CAT-OPTIMIZATION-ARGMAX-OPTION
title: "Algorithm.Optimization.Argmin exports spec 72 section 3.4: the selector argmax_option over a possibly empty List, its Nil and Cons equations, and the membership and greatest-cost corollaries, proved at zero trust, with a package test that runs the section 3.4 seed"
status: ready
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-OPTIMIZATION-ARGMAX-OPTION]
blocks: []
github: null
origin: "Catalog half of SPEC-OPTIMIZATION-ARGMAX-OPTION (routed 951246274). Same shape as CAT-OPTIMIZATION-ARGMIN-OPTION. Steward-filed per COORDINATION section 2."
---

# Argmin exports the greatest-cost Option selector

## Objective

`Algorithm.Optimization.Argmin` exports `argmax_option`,
`argmax_option_nil`, `argmax_option_cons`, `argmax_option_member` and
`argmax_option_greatest` with exactly spec 72 §3.4's public types, each
law a checked proof.

## Settled inputs (spec 72 §3.4 and its seed, from `951246274`)

- **The surface.** §3.4 states the definition, the two selection
  equations (`Nil` to `None a`; `Cons` against `argmax` of
  `nonempty_cons`), and the two corollaries over `list_elem` on the
  input list, each conditioned on an observed `Some y`. No equality or
  order on `a` is a premise.
- **The proof shape.** The `Nil` law by `Proved`, the `Cons` law by
  `Refl`. Three private helpers, `option_greatest`,
  `argmax_option_member_base` and `argmax_option_greatest_base`, reuse
  the existing private `option_member` unchanged.
- **The D0.** The Architect's checked §3.4 D0 on `914cfbda0`
  (`evt_32sjn56k5n42r`): `ken check` and `ken fmt --check` rc 0, no new
  Axiom, `trusted_base()` 114 to 114 over eight providers.
- **Seed.** `conformance/stdlib/optimization/seed-argmax-option.md`
  covers `Nil`, a singleton, and the non-head earliest greatest-cost
  tie `[3,2,0,1]`, with refused claims. It must be on `main` before
  this WP starts. It has not been run against shipped exports.
- **Package and tests.** `Argmin.ken.md` already exports `argmin`,
  `argmin_option` and `argmax` with their laws. They are tested by
  `cat_optimization_argmin.rs`, `cat_optimization_argmin_option.rs`
  and `cat_optimization_argmax.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The five exports and the three private helpers in `Argmin.ken.md`, the
spec 72 §3.4 status lines updated to say the exports are delivered, and
a new `crates/ken-elaborator/tests/cat_optimization_argmax_option.rs`
that root-loads each new public name and runs the seed. Every seed
execution-status statement changes in the same candidate to say the
package test executes it, with no conformance-runner, CI or landing
claim, and no fence or fixture change.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass. The test root-loads
  the five names as package-owned and transparent, keeps the helpers
  private, and runs the seed's positive cases: `Nil`, the singleton,
  and `[3,2,0,1]`, where the earliest greatest-cost element is neither
  the head nor the last tie. Its refused blocks are pinned to the
  seed's propositions. The three predecessor suites still pass. The
  Spec leader reviews the spec/ status lines, and CV the seed's.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom
  or primitive. The census row for `Algorithm.Optimization.Argmin` is
  unchanged, measured on the candidate.
- **AC-3 (mutation, QA).** Each mutant fails, and is restored:
  - `Cons x rest ↦ Some a x` (the head) fails the seed's tie case;
  - `Cons` returning `argmin` of the same list fails the tie case;
  - `Cons x rest ↦ None a` fails `argmax_option_cons` or the seed.

## Stop conditions

- A law needs an Axiom, function extensionality, a decidable equality
  or order on `a`, a kernel or surface change, or a public type other
  than §3.4's.
- A change to `argmin`, `argmin_option`, `argmax` or their laws, a
  census row change, or a change to a consumer of `Argmin` outside
  this package.

Checks: 4 fired: §3.4's four laws and three helpers are read from
`951246274`; the three predecessor test files are on `main` at
`80beabbd3`. 7 fired: AC-3 mutates the head, `argmin` and `None`
returns that the seed's tie and `Nil` cases are built to refuse.
