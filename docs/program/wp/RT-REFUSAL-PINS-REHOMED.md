# RT-REFUSAL-PINS-REHOMED

**Re-home the asserted-refusal pin for constructs 1 and 2 onto a mechanism that
survives `D3`.**

Frame. Steward-authored 2026-08-16. Node:
`docs/program/issues/RT-REFUSAL-PINS-REHOMED.md`.

**Treat every anchor as perishable.** If a fixed input is false against the
landed code, say so and stop — do not build around it.

## 1. Objective

`D1` ruled constructs 1 and 2 **CORRECT SEMANTICS** (Architect
`evt_5cxzxp4b6q31v`). A correct-semantics verdict owes a **detector**: an
asserted-refusal pin that **reds if the behaviour changes**.

The only such assertion today rides `RecursiveDescentResidual`, which `D3` of
[[RT-DESCENT-RETIRE]] deletes.

⇒ **Deliver an assertion of the same two refusals that does not reference the
retiring enum, and land it before `D3`.**

## 2. Fixed inputs

Measured at `origin/main` by the Steward. Cite; do not re-derive.

| input | value |
|---|---|
| existing pin | `d2k_0_the_five_no_longer_reach_a_static_worker_value_read`, present at `main` |
| the file | `crates/ken-runtime/src/cranelift_backend/lowering/core/tests/control.rs` |
| forcing mechanism it uses | `set_selector_variant_exclusion(Some(RecursiveDescentResidual::…))` |
| what `D3` deletes | `RecursiveDescentResidual` — **15** occurrences in `lowering/core.rs`, **42** in `control.rs` |
| construct 1 | `ComputationalMatch` / in-flight non-transferable activation, **4** programs |
| construct 2 | `StaticWorkerBinding`, **2** programs |
| held candidate | `D2c` `036e8ee916844fb91a4f42f2a2b04ebaea0dde2f`, **untouched, unpublished, NOT rebased** |

**The exact program names for constructs 1 and 2 are in
`docs/program/wp/RT-DESCENT-LANE-COMPLETENESS.md` section 3.** Find each by
name, never by line.

## 3. Deliverables

### D1 — Can the refusal be asserted WITHOUT the retiring enum? Answer first.

**This is a question, not an assumption, and the answer decides the node.**

Both lanes exist today, so a compile does not automatically take the
functionized route; `RecursiveDescentResidual` is how the current pin forces it.
After `D3` there is nothing to force away from.

**Determine whether an assertion exists that holds in BOTH worlds.** Report the
mechanism you found, or report that none exists **with the specific blocker**.

> **Do not silently pick the easy arm.** "Land it with the exclusion now and
> rewrite it at `D3`" is **not** a discharge — it recreates the exact defect
> `AC-9` exists to prevent, one commit later. If you conclude that is the only
> option, **say so explicitly and stop**; that is a finding for the Steward and
> the Architect, not a decision to take inside the increment.

### D2 — Land the pin.

Assert **both** constructs' refusals. **One pin per construct is acceptable and
so is one pin covering both**, provided the arms are independently checkable —
constructs 1 and 2 are **independently mutable**, and a shared reason has never
justified shared coverage.

**State in the commit which observable each arm keys on.**

### D3 — Prove it is a detector, not a decoration.

**A pin that has never been seen to red is not known to be a detector.**
Demonstrate that each arm reds when the behaviour it asserts is perturbed, in a
**disposable** tree, and report the perturbation and the observed failure.
**Reset the perturbation before any candidate is offered.**

## 4. Acceptance criteria

**AC-1.** The delivered assertion contains **no reference to
`RecursiveDescentResidual`**, and none to `set_selector_variant_exclusion`
carrying it. Control: grep the delivered test.

**AC-2.** Both construct 1 and construct 2 are asserted, with **independently
checkable arms**. Control: `AC-2` fails if perturbing one construct's behaviour
leaves every arm green.

**AC-3.** Each arm has been **observed to red** under a named perturbation
(`D3`), with the perturbation and failure reported and reverted. **A pin
asserted to work but never seen to fail does not discharge this.**

**AC-4.** The pin holds **at current `main`, with both lanes present.** It is
not permitted to depend on `D3` having already run.

**AC-5. `D2c` is untouched.** `036e8ee91` is not applied, rebased, published or
edited. Control: `git rev-parse` it unchanged.

**AC-6.** No production behaviour changes. This node lands a test artifact and
**deletes nothing**.

**AC-7.** No-regression, green **in CI** (`COORDINATION §12`). **Local runs are
targeted only** — `-p ken-runtime` or `--test`, never `--workspace`.

**AC-8.** If `D1` concludes no exclusion-free assertion exists, the node stops
with that finding **stated with its blocker**, and `AC-1`-`AC-4` are reported
as not attempted rather than as met. **A hard stop here is a legitimate
outcome, not a failure.**

## 5. Banned scope

- **Touching the five programs with no refusing construct.** They are `D6`
  rewrites in the predecessor.
- **Delivering constructs 3 or 4's obligations.** Those are recorded at
  [[RT-FNUNIT-MULTI-WORKER-CONTINUATION]] and
  [[RT-FNUNIT-CHECKED-ROOT-AUTHORITY-ROUTING]] and are not pins.
- **Any `D3`-`D8` retirement work**, including deleting the retiring enum,
  which this node exists to become independent of.
- **Widening to the other exclusion sites in `control.rs`.**

## 6. Why this is cut separately

[[RT-DESCENT-LANE-COMPLETENESS]] **authorizes no implementation and can never
reach `merged`** — it is measurement and adjudication, and its terminal state is
`closed`. **A pin is code and needs a node that merges.** Folding it back would
make a real deliverable ride a record.
