---
id: KERNEL-CONV-IOTA-DISCHARGE-DESCENT
title: "Conversion never halts on two distinct recursive heads whose neutral elimination sits under a closed ι-redex: the no-progress ledger discharges the head pair on any ι-progress, so a closed `elim_Bool true` resets it every lap. Discharge only on ι-progress that witnesses descent toward the pair's recurrence"
status: merged
owner: kernel
size: M
tier: T1
gate: architect
depends_on: [CHECK-REFL-CLOSED-STEP-FOLD-DIVERGENCE]
blocks: []
github: null
origin: "Architect disposition evt_7jp6sqvgp6kcx and carry in Decision dec_6t8hwnnghxv18 (CHECK-REFL-CLOSED-STEP-FOLD-DIVERGENCE), on the kernel-implementer's measurement evt_51cp4yaag19de. The residual predates the CHECK-REFL repair. Steward-filed per COORDINATION section 2."
---

# ι-discharge must witness descent

## Objective

Conversion halts on two separately declared recursive definitions that
recurse beneath a stuck eliminator, even when a closed ι-redex surrounds
that eliminator. It returns false at the distinct recursive-identity
boundary, as spec 17 §3.5 already requires. This closes a conformance gap;
it does not amend the spec.

## Fixed inputs (read at `08ff8431f`)

- **The fixture** (`evt_51cp4yaag19de`). Two separately SCT-admitted
  self-recursive `Nat → Bool` declarations. Each wraps its neutral-`Nat`
  elimination in `elim_Bool true` with two identical methods. Both sides
  apply their head to the same open `n`, and the heads are distinct.
- **Observed.** Under a temporary depth-90 conversion guard, the check
  aborts with exit 73 after 60 ι events, instead of returning. The fixture
  was scratch-only. Commit it as the first control.
- **The mechanism** (Architect `evt_7jp6sqvgp6kcx`). The ledger discharges
  a head pair on ι-progress anywhere in a side's whnf. The closed `true`
  ι discharges the `(c, d)` pair on every lap. The result is
  non-termination, never a false "equal".
- **The site.** `conv_struct_path` in `crates/ken-kernel/src/conv.rs`:
  `delta_origin_pair` captures the pair, and the `Some(p) if iota_progress`
  arm discharges it (`:719-746` at `08ff8431f`).
  `CHECK-REFL-CLOSED-STEP-FOLD-DIVERGENCE` restructures this function, so
  re-read it on the landed main.
- **Also carried.** Any `(recursive, non-recursive)` pair still refused
  after `CHECK-REFL-CLOSED-STEP-FOLD-DIVERGENCE` increment 2 keys the ledger
  on recursion (Architect `evt_7bkmcjy3a77z9` §3c). That under-acceptance
  predates this node.
- **The spec.** Spec 17 §3.5 (`spec/10-kernel/17-conversion.md:343-352`):
  "Conversion MUST halt with **false** at that boundary."

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

A discharge rule in kernel conversion that counts only ι-progress
witnessing descent toward the pair's recurrence. The fixture then returns
false at the default stack.

## Acceptance

- **AC-0 (design stop to the Architect; no build).** Propose the discharge
  rule with its failing counterexamples:
  - the closed-scrutinee pair above, which must halt with false;
  - closed computations such as `isEven 3` against a distinct same-shape
    copy, which must still converge;
  - the CHECK-REFL repro and its controls, which must keep their verdicts.
  The Architect rules the rule before any build.
- **AC-1.** The fixture returns false promptly at the default stack, with
  no depth guard. The converging counterexamples still converge.
- **AC-2 (controls).** Restoring the any-ι discharge brings back the
  divergence under a bounded timeout. The whole-catalog census shows zero
  verdict changes, or each change is named and ruled.

## Stop conditions

- Any fuel, or a depth cutoff that returns "equal". Neither is a repair.
- Any spec change: an operator question.
- Any change to what the kernel accepts beyond halting at this boundary:
  stop to the Architect.
- A `trusted_base()` change.

## Closeout

Merged as `9301e09e1` (PR #4398).
- **AC-1.** Conversion returns false at the distinct recursive-identity
  boundary beneath a stuck eliminator, as spec 17 §3.5 requires, with no
  fuel or depth guard. Nested δ is deferred below eliminator descent
  (`whnf_nested_component`), and the hard refusal keys on SCT diagonals.
- **AC-2.** The 57-package census shows zero verdict changes. The pin
  `stuck_nested_components_take_linear_reducer_entries` bounds all six
  nested shapes at k = 16, 32 and 64, and QA's mutation turns it red.
  Public `whnf` mode is unchanged. There is no `trusted_base()` change.
- **Carried.** Nested `Cast` types stay exponential in public `whnf`
  through `obs::cast_reduce` (Architect `evt_1fyxdasdg2g79`). It predates
  this WP, and its placement is an operator question.
