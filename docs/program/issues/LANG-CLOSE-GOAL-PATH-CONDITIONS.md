---
id: LANG-CLOSE-GOAL-PATH-CONDITIONS
title: "close_goal closes call-site requires obligations and the partial-primitive NoOvf and nonzero obligations over the context alone, never reading path_conditions, so they are posed without the if, match and let equations that hold on their path. Spec 22 §3 says each obligation is discharged under exactly the facts on its path. Close every obligation over its path facts"
status: ready
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-LET-BINDING-EQUATION]
blocks: []
github: null
origin: "Architect finding in the LANG-LET-BINDING-EQUATION AC-0 ruling evt_50qx9dbgqhwfe on 4ae2f60cf. Fails closed (Γ too weak, never too strong), no kernel impact; outside that WP's result-position deliverable. Steward-filed per COORDINATION section 2."
---

# Every obligation is closed over its path facts

## Objective

An obligation posed inside an `if` arm, `match` arm or `let` body carries
that path's equations, whichever closure poses it. Spec 22 §3: "each
obligation is therefore discharged under exactly the facts that hold on its
path".

## Settled inputs (Architect `evt_50qx9dbgqhwfe`, on `4ae2f60cf`)

- **Two closures.** `close_refinement_goal_with` (`elab.rs:11461`) reads
  `cx.path_conditions`. `close_goal` (`:11233`) takes an assumption slice,
  and every caller passes `&[]`.
- **The callers of `close_goal`** at `4ae2f60cf`: `precondition_proof`
  (`:4841`, call-site `requires`, spec §2.3), the `elab_binop` NoOvf and
  nonzero obligations (`:11144`, `:11204`), and two further sites the
  ruling did not name (`:16140`, a space-state postcondition; `:17128`, a
  recursive-view ensures). D0 classifies the last two.
- **After LANG-LET-BINDING-EQUATION** the channel also holds let
  equations, so the gap covers `if`, `match` and `let` facts alike.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **D0 (measure only).** For each `close_goal` caller: whether it can run
   with a non-empty `path_conditions`, one row per reachable caller showing
   the path fact missing from its goal, and the live catalog, library and
   examples population of each. The Architect rules the closure.
2. **The ruled closure**, sharing the install-depth handling of
   `close_refinement_goal_with` rather than a second copy.

## Acceptance

- **AC-1.** Each reachable caller's row closes with the path equation in its
  goal, and a kernel-checked certificate that needs that equation checks
  against the hole type.
- **AC-2 (controls).** Path-free obligations' goals are unchanged, emitted
  core is byte-identical, and no hole closed on base is open on the
  candidate across catalog, library and examples.
- **AC-3 (mutation, QA).** Passing `&[]` again reddens the rows.

## Stop conditions

- Any kernel, `trusted_base()` or spec change.
- A checked catalog, library or example program gains an obligation it
  cannot discharge.
