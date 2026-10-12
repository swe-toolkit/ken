---
id: LANG-EMBEDDED-CONVOY-WITH-CONTEXT-CONVOY
title: "The embedded-method equation convoy runs only when no context convoy is present, so a true Equal at a single-constructor type is still refused once the goal names a second value at the matched index (two lists at one index). Let the equation convoy coexist with the context convoy at the coherent-frame gate"
status: active
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-EMBEDDED-METHOD-EQUATION-CONVOY]
blocks: []
github: null
origin: "Adversary report evt_6dmkns8jbbay0 on c6008b54c (LANG-EMBEDDED-METHOD-EQUATION-CONVOY): the merged fix closes T1 but not the class when a context convoy is present. Distinct from the T2/T3 recursive-step shape in LANG-RECURSIVE-STEP-COORDINATE-COHERENCE. Steward-filed per COORDINATION section 2."
---

# The equation convoy beside a context convoy

## Objective

A true `Equal` proof at a single-constructor type in a coherent-frame
nested indexed match elaborates when its goal also mentions another value
at the matched index, as T1 does without one.

## Settled inputs (Adversary `evt_6dmkns8jbbay0`, at `c6008b54c`)

- **Repro.** Fixture `env()` from
  `lang_embedded_method_equation_convoy.rs`, plus
  `data DualPair (a : Type) : Type where { DP : a -> a -> DualPair a }`.
  - R1 has no outer match. With `xs` and `ys : DualEnv a (Suc m)`, the goal
    is `Equal (DualPair a)` of a `DP` pair of two `dual_lookup`s at
    `DualFZ m`, and the body matches `xs` then `ys` and ends in `Refl`.
    It gives `KernelRejected TypeMismatch { expected: (Dg647 @3), found:
    (Dg647 (cg72 @6)) }` at `match xs`. The same goal with body `Refl` is
    Ok.
  - Q1 is T1's shape with a second list, refused at the DualFZ arm's
    `match xs`, identically on base. Swapping the inner order or wrapping
    in Option also refuses.
- **Locus.** `plan_coherent_frame_motive` (elab.rs ~:4533-4536) enters
  the embedded convoy only when
  `!(defer_coupled_expansion && has_nontrivial_coupled_sides) &&
  has_nontrivial_coupled_sides && probe_context_convoy.is_empty()`. At
  R1's `match xs` frame it reads `context_convoy=1, coupled=true,
  defer=true`, so the planner is never entered. T1's frames read
  `context_convoy=0, defer=false, plan=2`. The gate predates the convoy WP
  (`ab55f525c`).
- **Not regressed.** Every probe row Ok on `8823a47a6` is still Ok. Eight
  T1-class rows went from Err to Ok.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

D0 first: the Architect rules how the equation convoy and the context
convoy coexist at that gate. Then the change in the coherent-frame motive
planner.

## Acceptance

- **AC-1.** R1 and Q1, and the swapped-order and Option variants,
  elaborate and check.
- **AC-2 (discriminating).** A false twin of R1 is refused at its false
  step, with an observation the true row does not produce.
- **AC-3 (control).** The convoy WP's suite and its 61-row set, and the
  72-file catalog census, are unchanged.
- **AC-4 (mutation, QA).** Restoring the `probe_context_convoy.is_empty()`
  conjunct reddens AC-1.

## Stop conditions

- Any kernel rule, conversion change or `trusted_base()` change.
- The repair is the recursive-step coordinate split: stop to the
  Architect, since that is `LANG-RECURSIVE-STEP-COORDINATE-COHERENCE`.
