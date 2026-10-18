---
id: KERNEL-CONV-DELTA-RETRY-CONVERTED-SIBLINGS
title: "When a same-constant spine fails at its last argument pair, the kernel's δ retry unfolds the constant and converts the already-converted sibling arguments again, because SpineFailure memoizes only the failed pair. A swapped nested H (b : Type 1) (a : Bool) with True against False costs 2^K in conversion alone: 13.8s at K=20. Avoid re-converting arguments that already converted"
status: draft
owner: kernel
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect attribution evt_bq2fze6dbp15 of the LANG-TYPE-POSITION-SORT-SPINE-GATE R1-swap stop (verify-implementer evt_4evecavcpxby6): pre-existing kernel behaviour, not a CONGRUENCE-FIRST regression. A kernel change: draft until the operator approves. Steward-filed per COORDINATION section 2."
---

# The δ retry does not re-convert converted siblings

## Objective

Kernel conversion of two nested same-constant spines whose failing
argument pair comes last costs time linear in the nesting depth, as it
already does when the failing pair comes first.

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
- **Precedent.** `KERNEL-CONV-SPINE-RETRY-LINEAR` (merged) made the δ
  retry linear for stuck eliminator wrappers. This is the same doubling
  through a different recomputed comparison.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The repair in `crates/ken-kernel/src/conv.rs`, ruled by the Architect at
AC-0, with a pin over the swapped row.

## Acceptance

- **AC-0 (design stop to the Architect; no build).** Reproduce the
  swapped row on the landed base, name the recomputed comparison and
  propose the repair. The Architect rules before any edit.
- **AC-1.** A committed `convert` pin over the swapped row at three depths
  including K=20 bounds a measure linear in K and returns false.
- **AC-2 (controls).** Reverting the repair reddens the pin. R1, R2 and
  the `KERNEL-CONV-SPINE-RETRY-LINEAR` pins keep their verdicts and
  bounds. No package changes verdict.

## Stop conditions

- Any change to what converts, as opposed to how fast, or any
  `trusted_base()` change.
