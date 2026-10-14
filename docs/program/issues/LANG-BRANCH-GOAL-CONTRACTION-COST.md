---
id: LANG-BRANCH-GOAL-CONTRACTION-COST
title: "Since 473ccf9b4 contract_applied_lambdas fully beta-normalizes every branch goal, under binders and inside stuck eliminator methods, and stores the normal form; a duplicating λ argument nested n deep makes elaboration time and the stored term grow as 2^n. Bound the contraction so elaboration cost stays proportional to the source"
status: active
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-ELABORATED-BETA-REDEX-AT-KERNEL-INFERENCE]
blocks: []
github: null
origin: "Adversary hunt of 473ccf9b4 (evt_4jtqrh7sy9c20): a resource/liveness defect in the LANG-ELABORATED-BETA-REDEX-AT-KERNEL-INFERENCE repair. Lesson enclave/resource-blowup-on-small-code-is-a-checker-bug: a resource wall on small input is a root-cause bug. Steward-filed per COORDINATION section 2."
---

# Branch-goal contraction stays proportional to the source

## Objective

`left_head`'s λ spelling still checks, and a branch goal is never
normalized beyond what kernel inference needs. Elaboration time and the
stored declaration grow at most polynomially in the source.

## Settled inputs (Adversary `evt_4jtqrh7sy9c20`, on `473ccf9b4`)

- **The site.** `contract_applied_lambdas` (elab.rs:5100-5115), called
  from the `simplify_branch_goal_keeping_refinements` wrapper at :5091,
  contracts under binders and inside stuck eliminator methods. It
  recontracts each substituted body (:5112) and stores the β-normal form.
- **Repro.** The 4-line program in the report (`lmap`, `mapdup` with a
  `g` body nested n deep, `mkl` passing `λx. f x x`, and `theorem blow`
  matching on `n`). Release build:
  - n=8 takes 65 ms, n=12 1.56 s, n=14 7.5 s and n=16 33 s;
  - the stored Debug length grows 4x per two levels.
- **Controls.**
  - `Refl` with no match: 1 ms at every n up to 20.
  - The linear argument `λx. f x Zero` with the match: under 20 ms at
    n=20.
- **Base.** On `eb74b0073` every match row was refused in 1-4 ms with the
  defect `473ccf9b4` fixed. No previously passing program slowed down.
- **The original D0** (`evt_3yey05qyvbds0`) named the alternatives:
  normalize head redexes where the unfolded term is built, or check
  instead of inferring at those positions. Neither normalizes below the
  head.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

D0 first: the Architect rules the bound, for example head-only
contraction or contraction that does not enter binders or stuck methods,
and re-measures `left_head` under it. Then the repair and an acceptance
row for the repro.

## Acceptance

- **AC-1.** The repro at n=16 elaborates in a bounded time named by the
  D0, and its stored declaration grows at most polynomially in n.
- **AC-2.** `lang_elaborated_beta_redex_at_kernel_inference` (λ and named
  `left_head`) and the 74-file catalog census are unchanged, per file.
- **AC-3 (mutation, QA).** Restoring full normalization reddens AC-1.

## Stop conditions

- Any kernel, conversion or `trusted_base()` change.
- `left_head`'s λ spelling stops checking under the bound: stop to the
  Architect.
