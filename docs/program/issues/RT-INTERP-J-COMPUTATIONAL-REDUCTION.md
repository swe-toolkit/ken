---
id: RT-INTERP-J-COMPUTATIONAL-REDUCTION
title: "The interpreter has no Term::J arm, so a closed, ground J whose motive is a data type evaluates to Neutral where the kernel gives a constructor. CAT-GRAPHS-DEPENDENCY's public Cycle return walk is such a J. Reduce J in the interpreter as the kernel does"
status: ready
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
- **Ignored row.** `ds5b_dependent_match_refinement_acceptance.rs` carries
  an ignored row labelled "ken-interp has no `Term::J` reduction arm".

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
- **AC-1.** The three minimal rows evaluate to the kernel's constructor
  (1, 1, 2).
- **AC-2.** The graph `Cycle` return walk's step count and edges match the
  kernel on the five measured cycles, and the
  `cat_graphs_dependency.rs` row checks them at runtime, not kernel-only.
- **AC-3.** The ignored `ds5b` J row is run: un-ignored if it now passes,
  otherwise its remaining blocker is named and it stays ignored.
- **AC-4 (control).** Proof-term J in cast evidence keeps its current
  behaviour unless the Architect rules otherwise, and the `ds5b` suite's
  other rows stay green.
- **AC-5 (mutation, QA).** Removing the new arm reddens AC-1 and AC-2.

## Stop conditions

- The interpreter's result differs from the kernel's on any row.
- The arm needs a kernel, `trusted_base()` or spec change.
