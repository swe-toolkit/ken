---
id: LANG-BRANCH-GOAL-BUDGET-VISIT-CHARGE
title: "a04ec5865 bounds the branch-goal simplifier, but it charges each visited reduct its full size and then charges the reduct's children again, so the charge is quadratic on a spine. A true theorem with a large non-duplicating closed list now exhausts the allowance and is refused as raw KernelRejected. Charge one unit per simplifier visit"
status: merged
owner: verify
size: S
tier: T2
gate: architect
depends_on: [LANG-BRANCH-GOAL-DELTA-DUPLICATION]
blocks: []
github: null
origin: "Adversary M8 hunt of a04ec5865 (evt_2k7j7ba62jaah), F1: a completeness regression that fails closed. Architect ruling evt_6h6zc63svyzha: real, S, T2, landed regression; the fallback stays as is. Steward-filed per COORDINATION section 2."
---

# The simplifier budget charges visits, not reduct sizes

## Objective

A theorem that checked before `a04ec5865` and has no duplicating helper
checks again. The branch-goal budget still stops exponential duplication.

## Settled inputs (Architect `evt_6h6zc63svyzha`, on `a04ec5865`)

- **Repro.** The `lang_embedded_context_convoy.rs` fixture (`DualFin`,
  `DualEnv`, `dual_lookup`), plus `Tri`, a tail-recursive
  `build (n : Nat) (acc : List Nat)`, and theorem `r1`. Its goal is
  `Equal (Tri a) L L`, where
  `L = TR a (build K (Nil Nat)) (dual_lookup … xs …) (dual_lookup … ys …)`
  and K is a written-out Suc chain. The source is in the Adversary's
  report. On `a04ec5865`, K=0 and K=60 are Ok, and K=80, 100 and 160 are
  refused with `KernelRejected TypeMismatch`. On `0cd98b0cd` all are Ok.
- **Cause.** `charge_branch_goal_budget` (elab.rs:5143) bills each visited
  reduct its full `bounded_term_size`, and the walk then bills the
  reduct's children. On a spine, that sum is quadratic, so the allowance
  16384 + 64·|goal| trips with no duplication present.
- **The ruled repair.** The ruling gives the code.
  - `charge_branch_goal_budget()` takes no size and charges one unit per
    visit. It returns false once the allowance is spent.
  - The call site at :4872 becomes `if !charge_branch_goal_budget() {`.
  - `bounded_term_size` stays, because it computes the allowance (:5173).
  - The spent check at :4860 guarantees `left >= 1` at charge time.
- **The doc at :5124-5128 is false.** Replace it with the ruling's text.
  It states the visit charge, and it states that the unsimplified
  fallback is convertible but not route-neutral.
- **Out of scope.** Making the fallback route-neutral (:5190, :5203) is
  a separate design: a zero allowance reddens 23 rows across 6 suites,
  because consumers such as `plan_coherent_frame_motive` (:4571, :4581)
  dispatch on the simplified syntax.
- **Known residual.** A tiny source with a large closed result, such as
  `build (pow2 p) Nil`, can still reach the allowance. That residual is
  inherent to any finite allowance, and its cliff is unmeasured. It is
  not a criterion here.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The visit charge and the corrected doc comment, in `elab.rs` only, plus
an acceptance test with the repro rows.

## Acceptance

- **AC-1.** The repro at K=80 and K=160 is Ok, with `trusted_base()`
  unchanged.
- **AC-2.** `lang_branch_goal_delta_duplication` stays green, including
  dupf and dup2 at n=16 within its bounds and the below-allowance row.
  These stay green: `lang_convoy_c1_after_c2_dependency`,
  `lang_convoy_embedded_sentinel_under_binder`,
  `lang_convoy_ambient_binder_coexistence`,
  `lang_embedded_context_convoy`,
  `dependent_match_coherent_frame_acceptance` and
  `elab_recursive_frame_budget`.
- **AC-3 (mutation, QA).** Restoring the size charge refuses the K=80
  row.

## Stop conditions

- Any kernel, conversion or `trusted_base()` change.
- A previously checking source stops checking, or the repair needs the
  fallback made route-neutral.
- The edit reaches outside the budget charge, its call site and its doc
  (LANG-LET-ANNOTATION-MOVED-FIELD-COORDINATE is editing `elab.rs`
  :1148 and :1371 concurrently).
