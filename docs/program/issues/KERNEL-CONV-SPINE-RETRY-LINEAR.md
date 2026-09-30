---
id: KERNEL-CONV-SPINE-RETRY-LINEAR
title: "Since the iota-discharge change, a false conversion between nested transparent eliminator wrappers takes time exponential in the nesting depth, because the same-head spine comparison and the δ retry each compare the same folded component. Conversion is linear in the depth on empty and hard δ-ledger paths, and quadratic at worst across soft→hard transitions"
status: merged
owner: kernel
size: S
tier: T1
gate: architect
depends_on: [KERNEL-CONV-IOTA-DISCHARGE-DESCENT]
blocks: []
github: null
origin: "Adversary M8 finding evt_2gmkfy1cdfe9y on 9301e09e1 (KERNEL-CONV-IOTA-DISCHARGE-DESCENT): a kernel liveness regression, reachable on success-path conversion branches. Steward-filed per COORDINATION section 2; sequenced before KERNEL-ENV-RAW-INSTALL-CRATE-PRIVATE as a regression of a landed kernel change."
---

# Linear conversion across the δ retry

## Objective

A conversion between nested transparent eliminator wrappers costs time
linear in the nesting depth on empty and hard δ-ledger paths, as it did
before `9301e09e1`, and at worst quadratic across soft→hard transitions
(Architect `evt_37g2qxwesq2zz`). It keeps the halting that change
delivered.

## Settled inputs (Adversary `evt_2gmkfy1cdfe9y`, read at `9301e09e1`)

- **Repro.** Take a non-recursive transparent
  `pred n = Elim n [zero; \m ih. m]` in the context `[x y : Nat]`. Then
  `convert(Nat, pred^k x, pred^k y)` returns false on both builds. The
  release timings, landed build then parent `111442ba8`:

  | k | landed | parent |
  |---|---|---|
  | 12 | 74 ms | 4.0 ms |
  | 16 | 1.19 s | 10.5 ms |
  | 20 | 18.97 s | 23.2 ms |

- **Mechanism.** A stuck consumer is rebuilt from its deferred component
  (`conv.rs:306`, `scrut: Box::new(s_d)`), so its scrutinee stays folded as
  `pred^(k-1) x`. In `conv_struct_path` (`:899`), the same-head spine
  congruence (`:922-945`) compares the folded pair and fails. The δ retry
  (`:963-971`) then exposes `Elim` over the same folded pair, and the
  structural arm compares it again: C(k) = 2·C(k-1). `pred` is
  non-recursive, so no δ-origin pair forms and the ledger never refuses.
- **Siblings rebuilt the same way, by source only:** `Proj1`/`Proj2`
  (`:232-261`), `Eq` (`:342`), `Cast` (`:364`) and `QuotElim` (`:415`).
- **Success-path reach.** A false result is a normal branch at `obs.rs:353`,
  `elab.rs:1364`/`:1370` and `elab.rs:2254`. A check can stall without
  reporting an error.
- **The landed pin cannot see it.**
  `stuck_nested_components_take_linear_reducer_entries` (`conv.rs:1270`)
  counts reducer entries in `whnf` and never calls `convert`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

Conversion over the six nested shapes is linear in k on empty and hard
paths, with the repair ruled by the Architect. The `9301e09e1` halting
fixture and its converging counterexamples keep their verdicts.

## Acceptance

- **AC-0 (design stop to the Architect; no build).** Reproduce the `pred^k`
  timings on the landed base. Name the recomputed comparison and propose
  the repair. The Architect rules before any edit.
- **AC-1.** A committed `convert` pin over all six nested shapes, at three
  depths including k = 20, bounds a measure that is linear in k. It
  returns the expected verdict for each shape.
  - A linear row on a hard, non-empty ledger path.
  - A seeded soft-path row bounded by `c·k²` at k = 8, 16 and 20, with
    the constant stated and the expected `false` verdict. It is the
    accepted polynomial residual, and its promise comment cites
    `evt_37g2qxwesq2zz`.
- **AC-2 (controls).**
  - Reverting the repair reddens the new pin.
  - The `9301e09e1` halting fixture still returns false promptly, and its
    converging counterexamples still converge.
  - The 57-package census shows no verdict change.

## Closeout

Merged `aa51bf9d7` (PR #4415), exact `b19da6f73`: Kernel QA
`evt_2x4kqhzhr31wm`, Architect `evt_2zv3nfepqqd6s`, Decision
`dec_2d03cmm74wkjy`.

- A retry-scoped failed-spine memo, keyed on context depth, δ path and pair,
  returns false only when that exact comparison replays.
- Conversion is linear on the empty and hard ledger paths, pinned across six
  nested shapes and a hard-ledger row. The soft→hard worst case is about
  3k² and is pinned (Architect `evt_37g2qxwesq2zz`).
- Both memo lookup sites have their own controls. Scope is `conv.rs` only,
  with no trust or verdict change.

## Stop conditions

- Any fuel, or a depth cutoff that returns "equal".
- Any change to what the kernel accepts: stop to the Architect.
- A `trusted_base()` or spec change: an operator question.
- The pre-existing nested `Cast` cost in public `whnf` (`obs::cast_reduce`)
  is not this WP. It is a separate operator question.

## Hard-stop inventory (§1b)

§1a count: 1 (Architect `evt_37g2qxwesq2zz`). The audit row falsified
linearity on a soft incoming path. The residual is accepted and pinned, and
the memo is not redesigned.
