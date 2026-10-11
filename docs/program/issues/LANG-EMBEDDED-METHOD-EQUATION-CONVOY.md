---
id: LANG-EMBEDDED-METHOD-EQUATION-CONVOY
title: "The coherent-frame motive planner generalizes a constructor-headed index of an embedded recursive method to a fresh variable and drops the equation that relates them, so a true Equal proof at a single-constructor type in a nested indexed match is refused exactly like a false one. Generalize by equation: convoy the index equation and supply the IH evidence from it"
status: ready
owner: language
size: L
tier: T1
gate: architect
depends_on: [LANG-MATCH-ARM-REFL-PAIR-GOAL]
blocks: []
github: null
origin: "Architect fork evt_7w6zenn2rez46, option (i), taken by the Steward: it supersedes LANG-COHERENT-FRAME-SINGLE-CONSTRUCTOR-EQUAL, carrying over its AC-1, AC-2 and measurements (evt_7ksak22xar0xj). Steward-filed per COORDINATION section 2."
---

# Generalize embedded method indices by equation

## Objective

A true proof of `Equal` at a single-constructor data type, inside a
coherent-frame nested indexed match, elaborates. A false one is refused
with an observation that a true one does not produce.

## Settled inputs (on `55a0bee93`)

- **Locus** (Architect `evt_74sfj4fef0vya`). T1, T2 and T3 (true) and
  `dual_recursive_false` all reject at the `DualFS` inner `match xs` with
  `TypeMismatch { expected: (Dg647 @2), found: (Dg647 (cg72 @6)) }`. F2
  and F4 are the false controls.
- **Mechanism** (`evt_7ksak22xar0xj`, `evt_7w6zenn2rez46`).
  - In the declined `dual_lookup` method, the ambient cast's evidence is
    valid only because its binder `Eq Nat (Suc k) (Suc m)` reduces to
    `Eq Nat k m`.
  - The rebase generalizes `Suc m` to a neutral `@6`. That leaves
    `Eq Nat (Suc k) @6`, and no binder relates `@6` to `Suc m`.
  - This is the same for every row.
- **Falsified repairs** (`evt_7w6zenn2rez46`).
  - Companion-carrying every unrepairable method: T1, T2 and T3 still fail,
    and a coherent-frame row panics.
  - The index-equation convoy at the outer frame (elab.rs ~:4091): an
    `Internal` overlap with the context convoy, which reddens 7 existing
    rows.
- **The needed shape.** When a constructor-headed index is generalized,
  keep `Eq Nat @6 (Suc m)` as a convoyed binder. Supply the IH evidence as
  `trans` of the two equations, then the observational reduct. No kernel
  rule is needed. Prior art: Coq Equations `generalize_eqs` and depelim,
  and Lean `cases` index generalization with `noConfusion`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

D0 first: the Architect rules the design, with a research prior-art pull on
index generalization by equations requested at kickoff. Then the convoy in
the coherent-frame motive planner, coexisting with the context convoy.

## Acceptance

- **AC-1.** T1 and T3 elaborate and check. T2 as the D0 ruling places it.
- **AC-2 (discriminating).** The named row and F4 are refused at their
  false step, with an assertion that a true twin does not satisfy.
- **AC-3 (control).** The coherent_frame, context_telescope, motive_rebase,
  index_convoy_impossible_arm_level and other suites in the Architect's
  61-row set, and the 72-file catalog census, are unchanged.
- **AC-4 (mutation, QA).** Dropping the convoyed equation reddens T1 and
  T3.

## Stop conditions

- Any kernel rule, conversion change or `trusted_base()` change.
- A suite or census delta outside the named rows: stop to the Architect.
