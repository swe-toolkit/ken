---
id: LANG-EFFECT-ROW-MATCH-BINDER-LATENT
title: "A match pattern binder releases nothing in the let consumer, so a callback pulled out of a constructor and applied in a let right-hand side has its action dropped. `match b { MkBox f ↦ let ignored = f MkUnit in host_program (print_line \"b\") }`, with `b = MkBox action` and `action` a Console proc, prints only `b`. Give a pattern binder its scrutinee's latent, read the same way by the gate and the let consumer"
status: merged
owner: verify
size: S
tier: T1
gate: architect
depends_on: [LANG-EFFECT-ROW-CLOSURE-BODY]
blocks: []
github: null
origin: "Adversary evt_50j4zgh3vg8ja (b) against landed LANG-EFFECT-ROW-LOCAL-LATENT 3aaea53e4. Architect AC-0 evt_1sygjr8yf6cgd on LANG-EFFECT-ROW-CLOSURE-BODY split it to its own node, sequenced after that WP, with the ruled direction below. Steward-filed per COORDINATION section 2."
---

# A match binder releases what its scrutinee releases

## Objective

A pattern binder carries the latent its scrutinee releases when named,
so a let whose right-hand side applies the binder sequences the action.
The probe prints `ab` and a newline.

## Settled inputs (main `df3089aed`)

- **The probe** (Architect D0,
  `/workspaces/ken/local/architect-shared/closure-body-d0/probes/match_let.ken`).
  `run_with (b : Box)` matches `MkBox f` and binds `let ignored = f
  MkUnit`. Base prints `b` only. CLOSURE-BODY's patch leaves it
  unchanged, so this node owns it.
- **As built.** `ElabCtx::surface_latent` (`elab.rs:1279`) resolves only
  declaration parameters and let binders, so `let_may_perform`
  (`:2171`) reads `f MkUnit` as pure. Match binders are pushed across the
  matrix, split and virtual-surface paths, and none of them registers a
  latent.
- **The gate already pays the scrutinee.** In `infer_expr_row_type`
  (`:17767`), the `RMatch` arm (`:17869`) joins the scrutinee's row and
  enters each arm's binders as releasing nothing. This is correctness,
  not soundness (Architect AC-0).
- **The ruled direction** (AC-0). A pattern binder releases its
  scrutinee's `bound_value_latent`, as landed by CLOSURE-BODY. The
  gate's `RMatch` arm and the let consumer read it the same way.
- **The census sentinel moves** on any new `.ken` fixture under
  `crates/*/tests`. Inline Rust sources leave it unmoved.

Treat anchors as perishable. CLOSURE-BODY lands first and moves these
lines. If a settled input is false on the landed base, stop and report
the mismatch; do not build around it.

## Deliverable

One candidate in which a match pattern binder carries its scrutinee's
latent at every binder site, read by both consumers, with the pins below.

## Acceptance

- **AC-0 (D0, Architect).** Rule the binder-site registration across
  the matrix, split and virtual-surface paths, and the consumer sweep
  with a disposition per consumer.
- **AC-1.** `match_let` `run` prints `ab` and a newline. A pure-field
  `Box` control prints `b` and a newline.
- **AC-2, a mutation per pin.** Removing the binder's latent makes AC-1
  print `b` only (red).
- **AC-3, no regression.** Every program the sweep names checks as
  before or changes as AC-0 rules. `trusted_base()` is unchanged.

## Stop conditions

- A kernel or trust change, or a change to a row typing rule.
- The corrected consumer changes a program that AC-0 did not name.

Checks: 2 fired: the choke point is binder registration for
`surface_latent`, and its fan-in is the three match binder paths that
AC-0 names. 8 fired: the base prints `b` for the probe and `ab` is
required, and the pure-field control separates the binder's latent from
the match form.
