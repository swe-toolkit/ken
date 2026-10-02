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
- **AC-1.** A committed pin bounds the count by `c·k` at those depths, and
  each result equals today's public `whnf` result.
- **AC-2 (controls).**
  - Reverting the repair reddens the pin.
  - Cast regularity (`cast A A refl a ⇝ a`) and a stuck neutral cast keep
    their results.
  - The 57-package census shows no verdict change.

## Symptom inventory (§1b, Architect)

1. Source-derived S multiplies per level outside the cast-regularity and
   Cast/Cast type-component re-reduction removed by N0–N2 (S k=4 573,169 →
   560,397). Keyed on: pending the D0 attribution (`evt_47jxn48dmreze`,
   §1a 1).

## S-residual D0 and dispositions (Architect `evt_7rt1154qwdgfb`)

The D0 is non-advancing and makes no source edit beyond N0–N2. A disposable
`cfg(test)` site stack charges each reducer entry to sites A–G (A: the whnf
Cast arm's components; B: cast_reduce regularity; C: `cast_at_inductive`;
D: `j_endpoints` from `j_nonrefl`; E: `type_eq_by_j` and its base; F: other
`check::infer`; G: untagged), with S at k = 2–5 and per-site k/(k−1)
ratios. The leading hypothesis is j_nonrefl witness typing, and it is not yet
measured.

- **(a) D, E, F or δ alone multiplies, with A–C linear.** N0–N2 land as their
  own increment, and the S residual becomes a successor node framed on the
  named site. AC-1 becomes:
  - T1 and T2 ≤ c·k at 8, 16 and 20, with results as ruled;
  - S at k=4 as a verdict pin with no bound, named as the successor's
    fixture, with its count in the comment.

  AC-2 becomes:
  - reverting N1 reddens T1, and reverting N2 reddens T2;
  - regularity and a stuck neutral cast keep their results;
  - the census is unchanged and `trusted_base()` is equal.
- **(b) A, B or C multiplies.** S stays in this WP, and the Architect rules
  the next entry from the named site.
- **Before QA, in either case:**
  - the N2 premise runs with the assertion on for T1 and T2 at k=8 and S at
    k=4;
  - the handoff lists every default-progress caller of
    `conv_struct_deferred`, which must be exactly N1's and N2's.
- **Trust delta.** N0–N2 skip one idempotent `whnf_defer_head_delta` at two
  entries and add no representation, cache, fuel, cutoff or rule, so this is
  not an operator question. A third default-progress caller reopens it to the
  Architect.

## Stop conditions

- Any fuel, or a depth cutoff.
- Any change to what the kernel accepts: stop to the Architect.
- A `trusted_base()` or spec change: an operator question.
