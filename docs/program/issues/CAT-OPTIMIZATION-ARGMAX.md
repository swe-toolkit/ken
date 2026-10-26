---
id: CAT-OPTIMIZATION-ARGMAX
title: "Implement spec 72 §3's argmax and its checked laws in the Algorithm.Optimization package, with a package test that runs the §3 seed"
status: merged
owner: foundation
size: M
tier: T2
gate: architect
depends_on: [SPEC-OPTIMIZATION-ARGMAX]
blocks: []
github: null
origin: "Catalog half of SPEC-OPTIMIZATION-ARGMAX. Same shape as CAT-OPTIMIZATION-ARGMIN-OPTION. Steward-filed per COORDINATION section 2."
---

# argmax in Algorithm.Optimization

## Objective

The package exports spec 72 §3's `argmax` and each law §3 rules, as
checked proof bodies at zero trust.

## Settled inputs (spec 72 §3, AC-1 `218387582`; Architect D0 `evt_6y59qv5tjtmw1`)

- **Surface (§3.1, §3.2).** In `Algorithm.Optimization.Argmin`
  (`catalog/packages/Algorithm/Optimization/Argmin.ken.md`):
  - `pub fn argmax (a : Type) (cost : a → Nat) (xs : NonEmpty a) : a`;
  - exactly seven public laws: `argmax_member`, `argmax_greatest`,
    `argmax_no_costlier`, `argmax_head_on_tie`, `argmax_singleton`,
    `argmax_keeps_head` and `argmax_takes_tail`, with the statements in
    §3.2.
- **Tie rule.** A tail candidate replaces the head only when the head's
  cost is strictly less under `lt_nat`, so the earliest tied maximum wins.
- **Proof route (§3.3, D0).** The D0 shares a private selection fold
  (`select_from`, with `cheaper` and `costlier`) and its membership proof
  between `argmin` and `argmax`, plus a private greatest-cost proof and
  order helpers. The Architect's patch is
  `/workspaces/ken/local/architect-shared/SPEC-OPTIMIZATION-ARGMAX-D0.patch`
  (sha256 `2b8c1d6f...`, against `136f959ad`). It is a route, not a
  requirement.
- **D0 evidence (§3.3).** `ken check` and `ken fmt --check` rc 0,
  `trusted_base()` 114 to 114, and no `Axiom`, `Opaque` or `Primitive`
  among the package's declarations.
- **Seed.** `conformance/stdlib/optimization/seed-argmax.md` (AC-2
  `ae0696091`). With `seed_cost` on `[1, 3, 2, 4]` the positive selects
  `three`, and the refusal claims `four`.
- **Test shape.** `crates/ken-elaborator/tests/cat_optimization_argmin_option.rs`.

## Deliverable

The package extension, plus a package test in
`crates/ken-elaborator/tests/` that runs the seed's positive client and
refusal.

## Acceptance

- **AC-1.** Each §3 law is a checked public export. `trusted_base()` is
  unchanged.
- **AC-2.** The seed's positive client is accepted. Its refusal is
  rejected at the claimed result, not earlier.
- **AC-3 (mutation, QA).** A tie rule that keeps the later tied
  candidate reddens the seed's positive client or a tie law.

## Stop conditions

- Any law that §3 rules cannot be proved without an assumption.
- Any `trusted_base()` change, or any change to the public surface of
  `argmin`, its seven laws, or `argmin_option` and its four laws.

Checks: 4 fired: §3's surface is read from landed spec 72 at
`218387582`, and the seed's exact SHA is named. It must be on main
before kickoff. 8 fired: AC-3's later-tie selector is the configuration
the seed's refusal accepts (D0 measured it), so a correct build reddens
on it.
