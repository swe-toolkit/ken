---
id: KERNEL-CONV-DELTA-RETRY-CONVERTED-SIBLINGS
title: "Within one δ retry the kernel re-decides spine argument pairs it already decided. Converted siblings are converted again because SpineFailure memoizes only the failed pair (swapped H, 13.8s at K=20), and a failed pair presented one binder deeper misses the memo on its depth equality (codomain-position Hc, 3.3s at K=18). Decide each original pair at most once, weakening-correctly"
status: active
owner: kernel
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect attribution evt_bq2fze6dbp15 of the LANG-TYPE-POSITION-SORT-SPINE-GATE R1-swap stop (verify-implementer evt_4evecavcpxby6): pre-existing kernel behaviour, not a CONGRUENCE-FIRST regression. A kernel change, approved by the operator 2026-10-20: \"approve KERNEL-CONV-DELTA-RETRY-CONVERTED-SIBLINGS\"; kicked evt_575g2hsdngz78. Amended per Architect evt_1crshmhjk9ay3 to fold in Adversary M8 finding evt_yvvjeg89jay1 (the depth half). Steward-filed per COORDINATION section 2."
---

# The δ retry decides each spine argument pair once

## Objective

Within one δ retry, each original spine argument pair is decided at most
once among re-encounters under an unchanged ledger path, whatever binder
depth presents it (weakening-correct). Re-encounters under a changed path
(recursive-head origin pushed, ι-discharged or hardened) are decided
afresh. That is the verdict-preserving residual, stated in the code
comment and not claimed linear (Architect AC-0 ruling
`evt_1y9c55a0198na`). Kernel conversion of the swapped row and of R2c is
linear in K.

## Settled inputs (Architect `evt_bq2fze6dbp15`, on `eac1c51a6` and `f04abbfee`)

- **The mechanism.** The same-constant spine (conv.rs ~:1184-1230)
  converts argument pairs in order and stops at the first mismatch. With
  the failing pair last, the successful sibling `t` against `t'` is
  converted at full cost before `True` against `False` fails. The δ retry
  then unfolds `H` to `t -> Int` against `t' -> Int` and converts `t`
  against `t'` again. `SpineFailure` records only the failed pair, so
  T(d) = 2·T(d−1).
- **Measured, kernel-only.** `convert_type(&env, &Context::new(), Const
  LL, Const RR)` on the two towers declared as consts, debug build:

  | Row | K=12 | K=14 | K=16 | K=18 | K=20 |
  |---|---|---|---|---|---|
  | swapped (failing pair last) | 0.052s | 0.219s | 0.877s | 3.449s | 13.837s |
  | R1 (failing pair first) | 0.6ms | 0.9ms | 1.1ms | | |
  | R2 (`Type` against `Type -> Type`) | 0.7ms | 0.9ms | 1.1ms | | 1.7ms |

  The conv.rs blob is identical on both trees, and the swapped rows agree
  within 4%.
- **The depth half (Architect `evt_1crshmhjk9ay3`, on `3282eadd8`).**
  `SpineFailure` is recorded at conv.rs:1213-1222 with `depth: ctx.len()`
  and the failed pair only. `seen_spine_failure` (conv.rs:669-688) hits
  only when `failure.depth == ctx.len() && failure.path == path`, at two
  lookup sites (conv.rs:1132, :1162). The Pi, Lam and Sigma codomain
  recursion passes the same `retry_memo` under an extended context
  (conv.rs:1363-1380). With `Hc (a : Bool) (b : Type 1) : Type 1 = Int ->
  b`, the failed closed pair returns one binder deeper and misses.
- **Measured (Adversary `evt_yvvjeg89jay1`), debug build:**

  | Row | K=10 | K=12 | K=14 | K=16 | K=18 |
  |---|---|---|---|---|---|
  | R2c kernel convert | 0.012s | 0.050s | 0.203s | 0.82-1.09s | 3.32s |
  | R2c full elaboration | 0.112s | 0.457s | 1.98s | 8.0-9.4s | 33.9s |
  | R2 (`b -> Int`) kernel | 0.000s | 0.001s | 0.001s | 0.001s | |

  A measurement-only control ignoring the depth moved R2c from 1.085s to
  0.001s at K=16. It is unsound for open terms and is not a repair.
- **One defect.** Both rows are the retry re-deciding, inside the unfolded
  body, a comparison of the original spine arguments it already decided:
  a positive result for the swapped row, a negative one for R2c. Both
  halves edit the same memo.
- **Precedent.** `KERNEL-CONV-SPINE-RETRY-LINEAR` (merged) made the δ
  retry linear for stuck eliminator wrappers. This is the same doubling
  through a different recomputed comparison.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The repair in `crates/ken-kernel/src/conv.rs`, ruled by the Architect at
AC-0, with pins over the swapped row and R2c.

## Acceptance

- **AC-0 (design stop to the Architect; no build).** Reproduce both rows
  on the landed base, name both recomputed comparisons and propose the
  repair for each. The Architect rules before any edit.
  - The depth half is weakening-correct: a failure stored at depth d
    matches at depth d' ≥ d only against its pair lifted by d' − d. A
    closed pair is the case where the lift is the identity.
  - Show that the ledger's depth-relative tests (`symbolic_since`,
    `mentions_var_below(term, ctx.len() - entry.depth)`, conv.rs ~:690)
    stay invariant under that lift, or restrict the relaxation to where
    they do.
  - Dropping the depth check is excluded.
- **AC-1.** Committed `convert` pins over the swapped row and R2c, each at
  three depths including K=20, each bound a measure linear in K, and
  each asserting its measured base verdict: swapped true, R2c false
  (Architect `evt_1y9c55a0198na`). Unit pins on the memo lookup cover a
  same-path hit, a swapped-orientation miss, a different-path miss, and a
  lifted open pair that hits only as its weakening.
- **AC-2 (controls, one per half).**
  - Reverting only the positive half reddens the swapped pin and leaves
    R2c green; reverting only the depth half reddens R2c and leaves the
    swapped pin green.
  - R1, R2, R1c (`Int -> b`, True/False, accepted) and the
    `KERNEL-CONV-SPINE-RETRY-LINEAR` pins keep their verdicts and bounds.
    No package changes verdict.
  - An open-term row: a failed pair that mentions a context variable,
    presented one binder deeper, keeps its verdict.

## Stop conditions

- Any change to what converts, as opposed to how fast, or any
  `trusted_base()` change.
