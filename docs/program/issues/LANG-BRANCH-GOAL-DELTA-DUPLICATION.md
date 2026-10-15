---
id: LANG-BRANCH-GOAL-DELTA-DUPLICATION
title: "The branch-goal simplifier whnf-unfolds every node and then re-simplifies both children of an App, so a named helper that duplicates its argument (dupf f x = f x x) nested n deep makes elaboration time and the stored term grow as 2^n through delta rather than beta. 44e4087d1 bounded only beta. Bound the simplifier so branch-goal cost stays polynomial in the source"
status: ready
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

D0 first: the Architect rules the bound, for example head-only `whnf`
without re-simplifying a duplicated argument, and re-measures the
`LANG-BRANCH-GOAL-CONTRACTION-COST` rows under it. Then the repair, with
an acceptance row for this repro beside the existing λ row.

## Acceptance

- **AC-1.** The `dupf` repro at n=16 returns within the bound the D0
  names, and if Ok stores at most 64·n² nodes. Control: the `linf` repro
  at n=16 returns Ok within the same bounds.
- **AC-2.** `lang_branch_goal_contraction_cost.rs`,
  `lang_elaborated_beta_redex_at_kernel_inference` and the catalog census
  are unchanged, per file.
- **AC-3 (mutation, QA).** Restoring the unguarded re-simplification
  reddens AC-1.

## Stop conditions

- Any kernel, conversion or `trusted_base()` change.
- A previously checking catalog file or test stops checking under the
  bound: stop to the Architect.
