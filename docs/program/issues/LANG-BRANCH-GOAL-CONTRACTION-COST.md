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

The Architect's D0 (`evt_1q0gv51yjdkq4`) rules **non-duplicating
contraction**: `contract_applied_lambdas` contracts `App(Lam(_, body), arg)`
only when the bound variable occurs at most once in `body` or `arg` is a
single-node term, so the work is quadratic in the goal. The D0 gives the
exact elab.rs patch at `48cda520b`.

- **Anchors moved.** The wrapper is at elab.rs:5145 and
  `contract_applied_lambdas` at :5159-5208.
- **Premise corrected.** The contraction drives the stored growth. The
  time goes to kernel conversion of the contracted `refl` witness, so the
  repro is now refused in about 10 ms rather than accepted.
- **Repro.** The D0's reconstruction; the report's text is unreachable.

## Acceptance

- **AC-1 (bound).** In the debug test profile, in-process
  `elaborate_file` of the D0 repro at n=16 returns within 1 s. If it
  returns Ok, the stored `blow` declaration has at most 16384 nodes
  (64·n², `ty` plus `body` via `Term::children()`). The row asserts the
  bound, not the refusal. Control in the same row: the linear spelling at
  n=16 returns Ok within 1 s with at most 16384 stored nodes.
- **AC-2.** `lang_elaborated_beta_redex_at_kernel_inference` (λ and named
  `left_head`) and the 74-file catalog census are unchanged, per file.
- **AC-3 (mutation, QA).** Restoring unconditional contraction reddens
  both halves of AC-1.

Out of scope (D0 residuals): **R1**, accepting a duplicating λ argument in
polynomial time; **R2**, a typed elaborator refusal naming the argument.

## Stop conditions

- Any kernel, conversion or `trusted_base()` change.
- `left_head`'s λ spelling stops checking under the bound: stop to the
  Architect.
