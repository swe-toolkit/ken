---
id: LANG-LATE-LEVEL-SOLVE-SIBLING-SITES
title: "LANG-MATCH-MOTIVE-LATE-LEVEL-SOLVE kept the level open only in the check-mode dependent-motive builders, so an if, an inferred-position match, a let-bound match and a nested-pattern matrix whose motive sort is solved later still store it at Type 0 and reject. Close the class at every motive-sort site"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-MATCH-MOTIVE-LATE-LEVEL-SOLVE]
blocks: []
github: null
origin: "Adversary finding F1 evt_2akanmfydmqgz on 440b216f1: the landed fix does not close its class (spec 39 §5.7). Correctness, false reject, fails closed, pre-existing. The Architect's carry evt_6009epx8k4rfg named :3498/:3515/:3565 and required a new WP with its own AC-0. Steward-filed per COORDINATION section 2."
---

# Every motive sort waits for its level

## Objective

As spec 39 §5.7 requires, a motive whose sort depends on a level solved
later in the declaration is stored at the solved level, at every site that
infers a motive sort.

## Settled inputs (Adversary, measured at `440b216f1`)

- The open query `kernel_infer_in_context_open` covers elab.rs:5295, :6976,
  :3849 and the reverting branch at :20597.
- Defaulted queries remain at :1473 (`make_if_elim`), :20599, :21180
  (`build_ctor_buckets`), :22247 (`infer_match_with_predicates`) and
  :3498/:3515/:3565 (`build_index_equation_convoy_body`).
- **Repro.** Prelude plus `fn two (y : Nat) (c : Type 1) : Nat = y`, then
  `fn m1 (a : Type) ... : Nat = two (<E>) a` for `<E>` among:
  `h (if b then x else x)`; `(match n { Zero ↦ h; Suc k ↦ h }) x`;
  `let y = match n { Zero ↦ x; Suc k ↦ x } in h y`;
  `h (match n { Zero ↦ x; Suc Zero ↦ x; Suc (Suc k) ↦ x })`;
  `(match n { Zero ↦ h; Suc Zero ↦ h; Suc (Suc k) ↦ h }) x`. Each gives
  `expected: Type 0, found: Type suc 0`; each `(a : Type 1)` control is Ok.
- Switching only :1473 to the open query flips the `if` row alone. No
  single site flips the inferred-match or matrix rows.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

Every motive-sort inference that stores a level reads it open, with the
landed `.or_else` fallback, so each elaboration decision equals the base
decision and admission re-checks with the default. The Architect names the
site set at AC-0.

## Acceptance

- **AC-0 (D0, measure only).** Every motive-sort query site by mechanism,
  with `file:line`, and for each repro row the site set that decides it.
  The Architect rules the closure.
- **AC-1.** The five repro rows check, and their `(a : Type 1)` controls
  stay Ok.
- **AC-2.** The landed `lang_match_motive_late_level_solve` rows stay
  green, and the catalog census has no verdict change. Timing within 10%.
- **AC-3 (mutation).** Reverting each ruled site to the defaulted query
  reddens the row that site decides.

## Stop conditions

- A site whose stored result is not re-checked at admission: stop to the
  Architect.
- Any kernel, `trusted_base()` or spec change.
