---
name: stated-stacks
description: The governing standard for tests that provision or pin their thread stack. Classifies masking a regression, provisioning a baseline, and pinning as an instrument; defines statedness; and distinguishes RUST_MIN_STACK from Builder::stack_size.
scope: tools
---

## Stated stacks

A test may set its own thread stack size. **The governing property is that the
stack is STATED — not that it is large, and not that it is small.**

A test whose outcome depends on the ambient stack (`ulimit -s`, the harness
default, `RUST_MIN_STACK`) is asserting a property of the machine it ran on.
Stating the stack at the site makes the result a property of the code under
test.

**Three acts. Only the first is forbidden.**

1. **Masking a regression — FORBIDDEN.** A test that a base-versus-candidate
   A/B shows **newly failing** may not be repaired by changing its stack. This
   holds **regardless of the reason recorded**: the change functions as the
   repair whatever the author intended. The condition is objective and
   reviewer-checkable — **no open measured regression on that test** — so this
   standard can never be used to unblock a candidate.
2. **Provisioning a baseline — PERMITTED, stated.** A test driving a
   legitimately deep workload may state a stack adequate for it.
3. **Pinning as an instrument — REQUIRED to be stated.** A test whose claim is
   about depth or stack behaviour must state its stack, **often BELOW the
   ambient default**, because the stated bound is the claim.

**"Stated" requires, at every site:**
- **The number** — at the site or in a named constant.
- **What the number is derived from**, written down: for (2) a measured peak
  and the headroom applied, **both as numbers**; for (3) the property being
  controlled and why a *fixed* size makes the control deterministic instead of
  machine-dependent.
- **The ambient environment neutralized** wherever the claim is about depth or
  stack: a spawned child must `env_remove("RUST_MIN_STACK")`, so the stated
  stack is the operative one.

**`RUST_MIN_STACK` and `Builder::stack_size` get DIFFERENT rulings, and the
difference derives from statedness.** `RUST_MIN_STACK` is ambient, invisible
at the site, and fleet-wide: **forbidden as a repair, and neutralized where it
could leak in.** `stack_size` is local and reviewable: **permitted under the
statedness requirement above.**

**The reference implementation is in-tree.**
`crates/ken-runtime/tests/value_depth_totality.rs` states its stack in a named
constant, records why the size is fixed, reports the thresholds it was
measured against at two sizes, and says explicitly that the point is a stated
stack rather than the ambient `ulimit -s`. **A site that reads like that has
discharged this standard.**

## Current act 1 application

`crates/ken-cli/tests/px4b_native_production.rs` has an open measured
regression, so changing its stack would be act 1. It becomes eligible for
provisioning only after that regression closes on its own terms, in a separate
node with its own Decision. This standard does not authorize an edit to that
file.
