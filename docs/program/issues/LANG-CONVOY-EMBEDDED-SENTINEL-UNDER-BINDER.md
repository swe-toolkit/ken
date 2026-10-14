---
id: LANG-CONVOY-EMBEDDED-SENTINEL-UNDER-BINDER
title: "Since 295ff612e a planned embedded eliminator found under a binder gets its refinement sentinel installed at the wrong depth, so the raw sentinel index reaches the kernel as VarOutOfScope 2^40+k. Make the sentinel and companion installers depth-aware"
status: ready
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-CONVOY-C1-AFTER-C2-DEPENDENCY]
blocks: []
github: null
origin: "Adversary M8 hunt of ed58ab85f (evt_ssgyr1b7y3ds), finding F1: a fail-closed regression of 295ff612e, whose Objective is that the elaborator never emits core the kernel rejects. Steward-filed per COORDINATION section 2."
---

# No refinement sentinel reaches the kernel

## Objective

A convoy row whose embedded eliminator sits under a binder in a premise
or the goal either checks or gets a typed refusal. No raw refinement
sentinel index reaches the kernel.

## Settled inputs (Adversary `evt_ssgyr1b7y3ds`, on `ed58ab85f`)

- **Fixture.** `env()` and the row shape of
  `crates/ken-elaborator/tests/lang_convoy_c1_after_c2_dependency.rs`,
  with L = `dual_lookup a (Suc m) xs (DualFZ m)`.
- **Repros.** All fail at the `match xs` span:
  - k9: goal `Nat → Equal a (L) (L)`, leaf `λn. Refl`:
    `VarOutOfScope 2^40+3`;
  - k7: `(g : Nat → Equal a (L) y)`, leaf `g Zero`: `2^40+2`;
  - k8: `(h : Equal a (L) y)`, goal `Nat → Equal a (L) y`, leaf `λn. h`:
    `2^40+3`;
  - u4, u5: the C1-after-C2 shape with `g : phi h → Equal a (L) y`.
- **Controls.** Each source checks as a direct body without the
  `match`. d3 (goal `Equal a (L) (L)`, leaf `Refl`, no binder) checks.
- **History.** The sentinel first appears at `295ff612e`. Before it, k7
  was a plain TypeMismatch and k8 and k9 the AC-3b Internal.
- **Probable locus.**
  - `collect_outer_embedded_elims_at` (elab.rs ~:2934) now plans
    eliminators under local binders.
  - `install_embedded_method_sentinels` (~:3333) and
    `install_embedded_index_equation_companions` (~:3774) recurse
    through Pi, Lam, Sigma and Let at an unchanged depth.
  - `finalize_refined_body` reads `canonical = v - depth` (~:9667). A
    sentinel under d binders lands below `region_start` and is shifted as
    an ordinary variable. It also slips past `ed58ab85f`'s escape guard,
    which checks only in-region slots.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

D0 first: the Architect confirms the locus and rules the depth
discipline for the installers (weaken by binder depth, or refuse the
plan under a binder). Then the repair and an acceptance target.

## Acceptance

- **AC-1.** k6 to k10, u4 and u5 each check or give a typed refusal
  named by the D0. None reaches the kernel with an out-of-scope index.
- **AC-2.** The C1-after-C2 rows, d3, P2 and the 295ff612e acceptance
  rows are unchanged; targeted runs only.
- **AC-3 (mutation, QA).** Removing the depth adjustment returns k9 to
  `VarOutOfScope`.

## Stop conditions

- Any kernel, conversion or `trusted_base()` change.
- A previously checking convoy row changes outcome.

## Finding outside this WP

F2 in the same report: a C2 premise that repeats a planned eliminator
(d1, d2, k1, k3) is refused even when unused. It predates `295ff612e`
and has a separate locus (`plan_coherent_frame_motive` ~:4649 and
`rewrite_c2_premise` ~:10800).
