---
id: LANG-BRANCH-GOAL-DELTA-DUPLICATION
title: "The branch-goal simplifier whnf-unfolds every node and then re-simplifies both children of an App, so a named helper that duplicates its argument (dupf f x = f x x) nested n deep makes elaboration time and the stored term grow as 2^n through delta rather than beta. 44e4087d1 bounded only beta. Bound the simplifier so branch-goal cost stays polynomial in the source"
status: active
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-BRANCH-GOAL-CONTRACTION-COST]
blocks: []
github: null
origin: "Adversary hunt of 44e4087d1 (evt_7c4mekb6rcfnv): a sibling producer that the LANG-BRANCH-GOAL-CONTRACTION-COST criterion was never run on. Lesson enclave/resource-blowup-on-small-code-is-a-checker-bug. Steward-filed per COORDINATION section 2."
---

# Branch-goal simplification stays polynomial under delta

## Objective

A branch goal built from a duplicating named helper elaborates in time,
and stores a term, polynomial in the source, as the λ spelling now does.

## Settled inputs (Adversary `evt_7c4mekb6rcfnv`, on `44e4087d1`)

- **The site.** `simplify_branch_goal_unguarded` (elab.rs:4859-4891)
  `whnf`s every node, and its `App` arm at :4888 recurses into both
  children. When `whnf` δβ-unfolds a duplicating named function, it copies
  the compound argument and simplifies each copy again. No λ redex forms,
  so the occurrence guard in `contract_applied_lambdas` never fires.
- **Repro.** `fn dupf (f : Nat → Nat → Nat) (x : Nat) : Nat = f x x`, and
  `theorem t1 … : Equal Nat (D^n) (D^n) = match n { Zero ↦ Refl; Suc m ↦
  Refl }`, where `D^n` nests `dupf f` n deep. The program is accepted. In
  the debug profile:
  - n=12 takes 1.62 s and stores 33104 nodes;
  - n=14 takes 7.90 s and stores 131456 nodes, 4x per two levels.
- **Controls.**
  - The same source with no `match`: 1.5-3.2 ms, linear.
  - The linear helper `linf f x = f x Zero` with the match: 3-13 ms,
    linear.
- **Provenance.** Lines :4859-4891 predate `44e4087d1`. This is not a
  regression of that squash.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The repair the Architect's D0 rules (`evt_2yxvr07dmkd0b`, on `45c3bd6db`):
a work allowance for each branch-goal simplification of
16384 + 64·|goal| reduct nodes. Each `whnf` reduct that
`simplify_branch_goal_unguarded` visits is charged by its node count. On
exhaustion, both sites in
`simplify_branch_goal_keeping_refinements_uncontracted` fall back to the
original goal. The state is thread-local, so the recursive frame does not
grow. The D0 gives the code, and it rejects head-only `whnf`, equal-argument
detection and a size-ratio guard. Acceptance rows for `dupf` and `dup2`
go beside the existing λ rows.

## Acceptance

- **AC-1.** In a debug build, `dupf` and `dup2`
  (`dup2 f x = f (Suc x) (Suc (Suc x))`) at n=16 each return in under 1 s,
  and if Ok each stores at most 64·n² nodes. Control: `linf` at n=16 is Ok
  within the same bounds. Below the allowance, `dupf` at n=8 stores more
  nodes than `linf` at n=8 (D0: 2288 > 312), which excludes a fix that
  never simplifies.
- **AC-2.** Unchanged, per file: `lang_branch_goal_contraction_cost.rs`,
  `lang_elaborated_beta_redex_at_kernel_inference`,
  `elab_recursive_frame_budget` and the catalog census.
- **AC-3 (mutation, QA).** An unbounded allowance reddens the `dupf` and
  `dup2` rows. A zero allowance reddens the below-allowance row.

## Stop conditions

- Any kernel, conversion or `trusted_base()` change.
- A previously checking catalog file or test stops checking under the
  bound: stop to the Architect.
- Out of scope: the cost inside one kernel `whnf` call, which is kernel
  domain.
