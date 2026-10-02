---
id: KERNEL-OBS-NESTED-CAST-LINEAR
title: "Reducing a nested Cast in public whnf repeats conversion work at each level, because cast_reduce's regularity test converts the two type endpoints before any structural step. Make public whnf of a Cast nested k deep cost work linear in k"
status: active
owner: kernel
size: S
tier: T1
gate: architect
depends_on: [KERNEL-CONV-SPINE-RETRY-LINEAR, KERNEL-OBS-REDUCT-WITNESS-TYPING]
blocks: []
github: null
origin: "Architect carry evt_1fyxdasdg2g79 in the KERNEL-CONV-IOTA-DISCHARGE-DESCENT TCB review: nested Cast types stay exponential in public whnf through obs::cast_reduce, pre-existing and outside that WP. Operator 2026-09-30 ('concur with recs'): file it after KERNEL-CONV-SPINE-RETRY-LINEAR. Steward-filed per COORDINATION section 2."
---

# Linear public whnf over nested Cast

## Objective

Public `whnf` of a `Cast` nested k deep costs work linear in k, with the
same result it gives today.

## Settled inputs (read at `a0c194ee8`)

- **The carry.** The Architect's IOTA review (`evt_1fyxdasdg2g79`) found
  that nested `Cast` *types* remain exponential in public `whnf` through
  `obs::cast_reduce`. The IOTA pin's GAP note records it
  (`conv.rs:1265-1268`: "`Cast^k` itself already repeats conversion within
  public `whnf`").
- **The site.** `cast_reduce` (`obs.rs:342`) first tests regularity with
  `convert_type(env, ctx, a, b)` (`:353`), then dispatches on the endpoint
  heads. `cast_at_inductive` and the `Sigma` arm convert again (`:492`,
  `:516`, `:527`, `:543`, `:608`).
- **Measure on the repaired reducer.** `KERNEL-OBS-REDUCT-WITNESS-TYPING`
  rewrites the witnesses at `:614` and at the sibling sites, which changes
  the cost this node measures.
- **Unmeasured:** the growth rate, and which conversion repeats. No timing
  or entry count exists yet.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

Public `whnf` over the measured nested-`Cast` shapes costs work linear in
the depth. The Architect rules the repair.

## Acceptance

- **AC-0 (measure, then design; no build).**
  - Measure public `whnf` on `Cast` nested at k = 8, 16 and 20, with the
    types of both endpoints nested as well. Use a reducer-entry or
    `convert` entry count, not wall time.
  - If growth is not super-linear, stop: the Steward closes the node.
  - Otherwise name the repeated conversion and propose the repair. The
    Architect rules before any edit.
- **Ruled (Architect `evt_1k4yc761p6rty`).** The site D0 is disposition
  (a), a different mechanism. N0–N2 are this WP's deliverable, and the S
  residual goes to `KERNEL-J-NONREFL-ENDPOINT-SHARING`.
- **AC-1.**
  - T1 entries ≤ 4k and T2 ≤ 8k at k = 8, 16 and 20. Base T1 at k=8 is
    1,021 and N1-only T2 at k=16 is 579, so both bounds discriminate.
  - Each T returns its input unchanged.
  - S at k=4 is a verdict pin (`check` accepts), with 560,397 in a comment
    and not asserted, named as the successor's fixture.
- **AC-2.**
  - Reverting N1 reddens T1, and reverting N2 reddens T2.
  - `cast A A refl a ⇝ a` and a stuck neutral cast keep their results.
  - The 57-package census shows no verdict change, and `trusted_base()` is
    equal.
- **Before QA.** The N2 premise runs with the assertion on for T1 and T2 at
  k=8 and S at k=4. The handoff lists the default-progress callers of
  `conv_struct_deferred`: exactly three, N1 once and N2 twice.
- **Trust delta.** N0–N2 skip one idempotent `whnf_defer_head_delta` at
  those entries and add no representation, cache, fuel, cutoff or rule, so
  this is not an operator question (`evt_7rt1154qwdgfb`).
- **Gates.** Kernel QA, then the Architect.

## Symptom inventory (§1b, Architect)

1. Source-derived S multiplies per level outside the cast-regularity and
   Cast/Cast type-component re-reduction removed by N0–N2 (S k=4 573,169 →
   560,397). Keyed on re-reduction of a non-refl J reduct's endpoints
   across the sites of one J reduction (j_endpoints, the type-equality
   witness, the Cast arm's components). The j_nonrefl fan-out is about 8
   per level, and the per-site cost is constant (`evt_3maz6hzwmq32r`, §1a 1).

## Stop conditions

- Any fuel, or a depth cutoff.
- Any change to what the kernel accepts: stop to the Architect.
- A `trusted_base()` or spec change: an operator question.
