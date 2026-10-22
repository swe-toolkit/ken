---
id: LANG-REFINED-PARAMETER-DEPENDENT-RESULT
title: "Structural recursion on a refined parameter is still refused with K0014 when the result refinement mentions that parameter, though base b22a41649 accepted it: the refined-worker plan rejects the projection of the coerced recursive argument substituted into the result type. Accept it as on base"
status: closed
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-REFINED-PARAMETER-STRUCTURAL-RECURSION]
blocks: []
github: null
origin: "Adversary M8 finding evt_2v7tpgnkbzra9 on LANG-REFINED-PARAMETER-STRUCTURAL-RECURSION (664d86ee8): an accept-to-refuse sibling the fix does not reach. Fails closed. Steward-filed per COORDINATION section 2; placed after LANG-RECURSIVE-BRANCH-EVIDENCE-BELOW, which is in flight."
---

# Recursion with a result that mentions the refined parameter

Closed 2026-10-21 as a documented limitation (Steward rescope
`evt_5q797p3jqwgsf` on the Architect's stop `evt_e23n6wvtsd2s`).
Refined structural recursion whose open obligations mention the
recursive call's result stays refused with K0014. It fails closed. WIP
`47c25c5c3` is never landed. The only repair that keeps obligation
identity is a kernel admission feature: an SCT group checked with
non-recursive transparent companion definitions. That is trust-root
growth and is the operator's call. The operator approved it on
2026-10-22 ("approve change"); it is KERNEL-SCT-GROUP-TRANSPARENT-COMPANIONS.

## Objective

With `def AnyNat = { x : Nat | Top }`, `fn g (n : AnyNat) : { y : Nat |
Equal Nat y n } = match n { Zero ↦ Zero; Suc m ↦ g m }` is accepted
with its open obligations as on base, and non-decreasing recursion is
still refused.

## Settled inputs (measured on `main` at `664d86ee8`, Adversary `evt_2v7tpgnkbzra9`)

- **The refusal.** Four shapes give K0014 on main and 3 open
  obligations each on `b22a41649`: d2 (above), d2s (`Suc (g m)`), r2
  (`: Nat ensures Equal Nat result n`), and r4 (an accumulator,
  `g m (Suc acc)`).
- **The mechanism.** `RefinedWorkerPlan::new`
  (`crates/ken-elaborator/src/elab/refined_recursion.rs` ~:312-316)
  rewrites the original body with `allow_new_projection_redexes =
  false`. The projection arms (~:185, ~:193) return `None` on any
  projection of a literal pair, after `unwrap_pair` strips `Ascript`.
  The recursive call's result ascription instantiates the dependent
  result at the coerced argument `(m, proof)`, so the plan fails at
  ~:412 and admission falls back to the original bodies, which SCT
  refuses. That pair is created by the refined split, not the
  pre-existing projection the ~:312 comment means to exclude.
- **Accepted controls on main.** The five AC-1 rows of the parent WP; a
  refined result that does not mention `n`; a later parameter refined
  on `n`; a `requires` clause on `n`; a plain `Nat` parameter with the
  dependent result.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The refined-worker plan accepts a projection redex created by
instantiating the result type at the coerced recursive argument, while
still refusing a projection of a literal pair already present in the
source. Committed tests for d2, d2s, r2 and r4.

## Acceptance

- **AC-1.** d2, d2s, r2 and r4 are accepted with the open obligations
  they had on `b22a41649`.
- **AC-2 (negative).** `f n` and `f (Suc m)` with the dependent result
  stay K0014. A source-level projection of a literal pair in the body
  still takes the original admission path.
- **AC-3.** The parent WP's tests, the SIGMA subset-Σ tests and its
  AC-2a census vector are unchanged.
- **AC-4 (mutation).** Allowing every new projection redex in the
  pre-check reddens the AC-2 source-pair row; restoring the blanket
  refusal reddens AC-1.

## Stop conditions

- Any kernel or `sct.rs` change, or any `trusted_base()` change.
- A newly refused program in AC-3.

Checks: 4 fired: `unwrap_pair`, the ~:185/~:193 projection arms, the
~:312 pre-check and the ~:412 fallback exist at `664d86ee8`. 7 fired:
AC-1 carries all four refused siblings, and AC-2 keeps a source pair
projection on the conservative path.

## Symptom inventory

1. Worker admission TypeMismatch: open-obligation constants are
   env-typed against the public group member (`g651 : … g648 (m, g2)
   …`), which the worker split cannot rewrite, so the worker pair's
   predicate (at the worker call) disagrees with its proof's type (at
   the public call). Keyed on a non-member constant whose declared type
   mentions a group member (Architect `evt_e23n6wvtsd2s`).
