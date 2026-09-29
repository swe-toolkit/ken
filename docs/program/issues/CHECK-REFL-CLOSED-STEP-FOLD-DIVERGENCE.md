---
id: CHECK-REFL-CLOSED-STEP-FOLD-DIVERGENCE
title: "The checker exhausts stack and memory on a true 669-byte Refl goal: a Node equation for a fold wrapper with a closed step lambda. Classify the phase (elaboration whnf or kernel convert) first, then repair it in its owner, with no stack increase as the repair"
status: active
owner: kernel
size: M
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Architect ruling 2026-09-29 (evt_65j62s92avcsx) on CAT-REL-CLOSURE-LAWS's second converse stop (foundation-implementer evt_1g9334q9f4wyg, foundation-leader evt_37mg32nppxr2g): a measured checker defect found in L3 proof backfill. The Architect asked the Steward to file it. Steward-filed per COORDINATION section 2."
---

# Refl on a closed-step fold wrapper diverges

## Objective

The checker decides the goal below promptly: it accepts or rejects it with
bounded stack and memory. The equation is true, so the expected result is
acceptance.

## Fixed inputs (read at `30e9f2081`)

- **The repro.** This is 682 bytes as a file (the fence dedented by two
  spaces), SHA-256 `a97ea96fd974f081…`, as minimized by the foundation
  implementer:

  ```ken
  data Tree k v = Leaf | Node (Tree k v) k v (Tree k v)

  fn fold (k : Type) (v : Type) (b : Type) (f : k → v → b → b) (z : b) (m : Tree k v) : b =
    match m {
      Leaf ↦ z;
      Node l key val r ↦ fold k v b f (f key val (fold k v b f z l)) r
    }

  fn wrapped_fixed (k : Type) (v : Type) (b : Type) (m : Tree k v) (acc : b) : b =
    fold k v b (λkey. λval. λbefore. before) acc m

  theorem wrapped_fixed_node
        (k : Type) (v : Type) (b : Type)
        (left : Tree k v) (key : k) (val : v) (right : Tree k v) (acc : b)
      : Equal b
          (wrapped_fixed k v b (Node k v left key val right) acc)
          (wrapped_fixed k v b right (wrapped_fixed k v b left acc)) =
    Refl
  ```

- **Observed.** The run exits 134 at the default and at 16 MiB main-thread
  stacks. At 256 MiB under a 3 GiB memory cap, it ends in allocation
  failure. That is divergent or exponential behaviour on a tiny term.
- **Controls (measured).**
  - The bare `fold` Node equation checks.
  - The same wrapper with the step passed as a parameter checks.
  - `Proved` in place of `Refl` rejects promptly with `TypeMismatch`.
  The fault is therefore keyed on ι-reducing `fold` on a `Node` under a
  closed step lambda.
- **The site.** `synth_refl_proof` (`crates/ken-elaborator/src/elab.rs`
  `:1678-1692`) calls `whnf` on the goal, then the kernel's `convert` on
  the two sides. Either call may be the one that diverges. The phase has
  not been observed.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

A repair in the owning phase, so that the repro checks with default stack
and bounded memory. The three controls stay as they are.

## Acceptance

- **AC-0 (done): kernel `convert`.** Measured at `22093cfcd`
  (`evt_40sdhwvwtjhbg`): `whnf` returns, then `convert` never returns and
  the 8 MiB stack overflows. The growing term inside `convert` is not yet
  observed. The kernel-convert stop fired. Operator 2026-09-29: "one wp for
  checker crash, yes"; the Kernel ring owns the repair (kickoff
  `evt_32bdq2a3gjj33`).
- **AC-0 as framed (classify, then ruling; no edit).** Run the repro instrumented and
  name the diverging call: elaboration `whnf` or kernel `convert`. Give the
  term shape that grows, for example an unfolding that re-exposes the same
  redex. The Architect rules the repair before any build.
- **AC-1.** The repro checks at default stack with bounded memory, and the
  wrapper's Node equation is accepted.
- **AC-2 (controls).**
  - The `Proved` variant still rejects with `TypeMismatch`.
  - A false variant of the Node equation still rejects promptly. For
    example, swap `left` and `right` on the right-hand side under a step
    that does not ignore its accumulator.
  - Reverting the repair brings back the divergence, under a bounded
    timeout.

## Stop conditions

- **The fault is in kernel `convert`.** Stop to the Steward. The repair then
  belongs to the Kernel ring, which is outside the current lane roster (an
  operator question).
- A repair that raises a stack or fuel limit instead of removing the
  growth. That is not a repair.
- Any change to what the kernel accepts beyond deciding this goal promptly:
  stop to the Architect.

## Increments

- **Increment 1, landed `b9840913e`** (PR #4375; candidate `f9ee9d4b7`;
  Kernel QA `evt_g7cka5dp2x5v`, Architect APPROVE and Decision
  `dec_6t8hwnnghxv18`). Head δ is deferred in kernel conversion. The
  682-byte repro checks at the default stack, and all 57 catalog packages keep
  their verdicts.
- **Increment 2, released `evt_kab4esnr78a6`.** Increment 1 records
  same-head pairs for every transparent constant, so three nested calls to a
  non-recursive wrapper are refused (Adversary `evt_9s2j10ap3pxc`). The
  Architect ruled the repair (`evt_7bkmcjy3a77z9`): the ledger records a pair
  only when a head is recursive, meaning on a cycle of the transparent-body
  reference graph. That also closes the distinct non-recursive sibling.
- **Filed residual.** Two distinct recursive heads under a closed ι-redex
  still do not halt. That predates increment 1 (`evt_7jp6sqvgp6kcx`) and is
  `KERNEL-CONV-IOTA-DISCHARGE-DESCENT`.
