---
id: RT-INTERP-J-COMPUTATIONAL-REDUCTION
title: "The interpreter has no Term::J arm, so a closed, ground J whose motive is a data type evaluates to Neutral where the kernel gives a constructor. CAT-GRAPHS-DEPENDENCY's public Cycle return walk is such a J. Reduce J in the interpreter as the kernel does"
status: merged
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary M8 finding evt_1be6pqw5j0qwf on 725b73258 (CAT-GRAPHS-DEPENDENCY). Correctness, runtime canonicity, fails closed. Violates spec 42 section 3.6. The gap was recorded only as the label of an ignored row (RT-COMPMATCH-TREE-SCRUTINEE, operator 2026-09-26); no node owned it. Steward-filed per COORDINATION section 2."
---

# The interpreter reduces J

## Objective

A closed, well-typed, ground term of inductive type whose head is `J`
evaluates to the same constructor form in `ken-interp` as the kernel
derives, as spec 42 §3.6 requires.

## Settled inputs (Adversary `evt_1be6pqw5j0qwf`, at `725b73258`)

- **No arm.** `eval_in_erasure_scope` (`crates/ken-interp/src/eval.rs`
  ~:2379) has no `Term::J` case; J falls to the catch-all
  `_ => EvalVal::Neutral` (~:2599).
- **The kernel always reduces a well-typed J.** `conv.rs` ~:433-446 calls
  `obs::j_reduce`: the base on `refl`, otherwise a `Cast` (spec 15 §4).
- **Repro.** `fn tr (x : Bool) (h : Equal Bool x True) : Nat = J (λp _.
  Nat) (Suc Zero) h` with `const c : Nat = tr True Proved`: the
  interpreter gives Neutral, the kernel accepts `Equal Nat c (Suc Zero)`
  and rejects `Suc (Suc Zero)`. Two sibling rows (a value-argument J, and
  `Equal Bool x x` at True with base 2) behave the same way.
- **Catalog exposure.** `Algorithm.Graphs.Dependency`'s
  `walk_transport_end` (~:545) is a computational J, stored by
  `cycle_from_hit` as the public `Cycle` return walk. The interpreter
  gives Neutral for its step count and edges on every measured cycle; the
  kernel gives Bool two_cycle 1, part_cycle 1, tri 2, selfc 0, tri-dup 2.
  `cat_graphs_dependency.rs` ~:669 narrows that row to kernel-only checks.
- **No ignored row remains.** The `ds5b` row labelled "ken-interp has no
  `Term::J` reduction arm" left the suite in `e41f7589e`. On `0568688cb`,
  no `#[ignore]` row under `crates/` cites a missing J arm (D0
  `evt_7swzx84np0qav`; Steward `evt_7nyxbwt3m3vdt` struck AC-3).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

`ken-interp` evaluates a computational J by the kernel's rule: the base on
`refl`, otherwise the transport the kernel derives, evaluated by the
interpreter's existing cast semantics. The Architect rules the mechanism
at D0, including whether a proof-valued (`Omega`) J stays erased.

## Acceptance

- **AC-0 (D0, measure first).** Every J reach in `ken-interp`
  (evaluation, erasure, caches) by mechanism with `file:line`, and what
  the interpreter's cast path does with the kernel's non-`refl` result.
  The Architect rules the arm.
- **AC-1.** Three `eval_checked` rows evaluate to the kernel's
  constructor, each paired with an accepted kernel theorem and a rejected
  wrong twin: `tr True Proved` = 1, `trId True Proved` = 2, and the
  Architect's value-argument row `tv (Suc Zero) Proved` = 1 (`J (λp _. Nat)
  n h` over `Equal Nat (same_nat n) n`). The Adversary's third source was
  unavailable (Steward `evt_42tsz9t8r26dp`).
- **AC-2.** The graph `Cycle` return walk's step count and edges match the
  kernel on four cycles: two_cycle, part_cycle, tri and selfc. Each row
  computes its kernel pairing live from its fixture, and the
  `cat_graphs_dependency.rs` row checks them at runtime, not kernel-only.
  tri_dup is deleted, not ignored: it is tri's graph, its kernel oracle does
  not finish in 1,800 s, and its enumeration axis is not the J arm's
  (Architect `evt_38j28mej8919j`, Steward fork (a) `evt_2pb5r5pjk5tyn`). A
  row is admitted only if its kernel phases finish within 600 s locally.
- **AC-4 (control).** Proof-term J in cast evidence keeps its current
  behaviour unless the Architect rules otherwise, and the `ds5b` suite's
  other rows stay green.
- **AC-5 (mutation, QA).** Removing the new arm reddens AC-1 and AC-2.

## Stop conditions

- The interpreter's result differs from the kernel's on any row.
- The arm needs a kernel, `trusted_base()` or spec change.

## Closeout

Merged `44443145c` from exact `5739d053c` (PR #4717). Runtime QA
`evt_1mzb86tqjpad6`; Architect `evt_3rb4jb9jervtv` with Decision
`dec_6a6v3ezwde0xa`.

- The interpreter reduces a closed, ground `J` as the kernel does. The
  graph `Cycle` return walk now evaluates to the kernel's constructor on
  the four AC-2 cycles. tri_dup is deleted under fork (a).
- The first candidate `16f7bcd12` (PR #4714, closed) was red in CI. The
  inline arm grew the evaluator's frame from 6016 to 8080 bytes, and
  `cc7_argparse_acceptance` overflowed its stack. The respin moves the
  arm into an `#[inline(never)] fn eval_j`, which brings the frame back to
  6064 bytes. `16f7bcd12` is never republished.
