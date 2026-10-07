---
id: LANG-MATCH-MOTIVE-LATE-LEVEL-SOLVE
title: "A match whose motive's sort depends on a bare-Type level that is solved only later in the declaration is rejected at admission: the motive's sort is inferred over a zonked context, which reads the unsolved level as Zero, so a later solve to Suc Zero leaves the stored motive at Type 0. Keep the level open or defer the query, as spec 39 requires"
status: active
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-MATCH-ARM-LEVEL-META-KERNEL-CHECK]
blocks: []
github: null
origin: "Adversary hunt evt_5gf6s6krtxft on abc35cbcf (LANG-MATCH-ARM-LEVEL-META-KERNEL-CHECK): correctness, false reject, fails closed, pre-existing (identical at abd0b7621). Named by that WP's stop condition and the Architect's carry evt_6nshs7j2we7zv. Reachable from well-formed source. Steward-filed per COORDINATION section 2."
---

# A late level solve does not invalidate a match motive

## Objective

A match inside a declaration whose bare-`Type` level is solved after the
match elaborates, with the motive at the solved level. Spec 39 §5.7
(`39-elaboration.md:873-876`): `?u` is solved during elaboration, and the
kernel never receives a level metavariable.

## Settled inputs (Adversary `evt_5gf6s6krtxft`, read at `abc35cbcf`)

- **Repro.** With the prelude and `fn two (y : Nat) (c : Type 1) : Nat = y`:
  `fn m1 (a : Type) (n : Nat) (x : a) (h : a → Nat) : Nat = two (h (match n
  { Zero ↦ x; Suc k ↦ x })) a` gives `KernelRejected TypeMismatch {
  expected: Type 0, found: Type suc 0 }`. The same holds for a `Bool`
  scrutinee and for an ascribed `(match … : a)`.
- **Controls, all accepted:** `(a : Type 1)`; the level solved before the
  match; no later solve; a later solve with no match.
- **Cause.** The motive's sort is a kernel inference over the zonked context
  (`elab.rs:6058`, `:4454`, through `kernel_infer_in_context_current` at
  `:8457-8467`). `MetaCtx::zonk_level` (`:137-140`) reads an unsolved meta as
  `Zero` without recording it, so the motive is ascribed at `Type 0`.
- **Not a regression.** All seven rows give the same verdicts at
  `abd0b7621`.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## AC-0 (Architect, at kickoff)

The Architect rules the repair: keep the motive's level as the open meta, or
defer the sort query until the level is solved, or commit the `Zero` read in
the store so a later conflicting solve is a surface diagnostic.

## Acceptance

- **AC-1.** The repro and its two variants check, and the motive's sort is
  `Type 1` in the elaborated core.
- **AC-2 (controls).**
  - The four controls keep their verdicts.
  - A declaration whose later use genuinely conflicts with the match's level
    is still rejected.
  - Reverting the repair reddens the repro row.
  - The `LANG-MATCH-ARM-LEVEL-META-KERNEL-CHECK` rows stay green.
- **AC-3.** The targeted match suites stay green, the catalog census is
  byte-identical, and `trusted_base()` is unchanged.

## Stop conditions

- The repair needs a kernel change or reordering elaboration across
  declarations: stop to the Architect.
