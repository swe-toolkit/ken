---
id: RT-PX8DS-REPRESENTED-CROSSING-GATE
title: "Restore the retired-flat rejection-witness gate on the represented result-edge crossing (transfer_represented_boundary_value entry), which commit 5ec0ab42f bypassed when it rerouted the specialized-join consumer off transfer_into_carrier. Single-locus, entry-not-site, additive fail-closed, keyed on the test-only px8ds_retired_flat_order flag (strict no-op in production). Restores the exact boundary.rs:1047 raw-closure refusal the px8ds negative control asserts, without touching the legitimate D3 represented result-edge crossing."
status: merged
owner: runtime
size: S
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "Steward-filed 2026-09-08 on the Architect PX8DS ruling (evt_1x24nftyc43xz, thr_30nsxsykvcsm2), which concurs with runtime-leader's classification (evt_cj27hbm7e9j2): NARROW CONSUMER-SIDE REGRESSION, not stale. Surfaced when the px8ds negative control (px8ds_retired_flat_order_does_not_gain_m4_representation) stopped refusing at boundary.rs:1047 after first-bad-commit 5ec0ab42f. The Architect grounded the mechanism, the repair locus, the not-a-soundness-hole argument, and four guardrails at exact 5a6c84638, and directs building the fix from that message without further lookup. Explicitly NOT a reopen of the closed RT-NESTED-IH-NATIVE-REALIZATION WP (the D3 realization is sound; one consumer change failed to carry a test-control gate). Explicitly DISJOINT from RT-PARITY-NATIVE-GOLDEN-REBASELINE: px8ds is not stale, it caught this — only the two rt_parity_native absolute-id goldens are the stale members of that node."
---

> # Narrow within-lane runtime fix: restore one gate on the represented crossing.
> # Additive, fail-closed, single-locus, entry-not-site; keyed on the test-only
> # px8ds flag so it is a strict no-op in production. Do NOT rebaseline or weaken
> # the px8ds test — the test is correct and caught this.
>
> The design is the Architect's PX8DS ruling (evt_1x24nftyc43xz). This node
> transcribes it; the exact expression is runtime's to write. Build from the
> ruling; the mechanism/repair/guardrails below are its verbatim substance.

## Not a production soundness hole (read before treating this as a chain reopen)

The two consumers differ ONLY under `px8ds_retired_flat_order_enabled()`, which
is `cfg(any(test, feature="px8-ds-test-support"))` and const `false` in
production. The Architect confirmed the production paths coincide where it
matters: a `Closure { boundary_environment: None }` (and DeclarationClosure /
ComputationalRecursorClosure / every non-represented variant) falls through
`represented_boundary_admissibility` to `boundary_transfer_admissibility()` — the
SAME `boundary.rs:1047` refusal (`aggregates.rs:1042-1060`). So a no-authority
closure refuses on BOTH paths in production; nothing unauthorized crosses. The
RT-NESTED-IH D3 realization is sound and is NOT reopened. The defect is purely
that a negative control lost its subject.

## Mechanism (Architect, grounded from 5a6c84638)

- The retired-flat rejection witness IS a gate inside `transfer_into_carrier`
  (`lowering/mod.rs:7038-7040`): when
  `value.contains_boundary_closure_environment()? && !px8ds_retired_flat_order_enabled()`
  it returns `transfer_bind_continuation_boundary_value(...)`; otherwise
  `value.boundary_transfer_admissibility()?` reaches the raw-closure refusal at
  `boundary.rs:1047`. Its own comment states the intent: the test-only retired
  flat-order control intentionally receives no M4 representation — it is a
  rejection witness, not another bind edge.
- Commit `5ec0ab42f` rerouted the specialized-join consumer
  (`lowering/joins.rs:320`) from `transfer_into_carrier` to
  `transfer_represented_boundary_value` (`aggregates.rs:1165`). That function is
  `represented_boundary_admissibility -> source_aggregate_preflight ->
  emit_carrier_transfer` and consults `px8ds_retired_flat_order_enabled()`
  NOWHERE. So the specialized-join path bypasses the rejection-witness gate: a
  retired-flat closure with a valid `boundary_environment` (record119/120 —
  schema matches, so `represented_boundary_admissibility` admits it) now
  materializes the represented record and proceeds to `units.rs:3250` instead of
  refusing at `boundary.rs:1047`. That is exactly the record-becomes-regression
  branch (PFI4/seat545/body536 and PFI6/seat592/body584 in the runtime-leader
  trace).
- The one production difference the reroute makes is the LANE for an
  environment-bearing closure — the represented result-edge lane
  (`transfer_represented_boundary_value`) is the D3-intended lane, reviewed and
  parity-green (`rt_nested_ih_native_realization`). That is intended, not the
  defect.

## Repair ruling (Architect): restore the gate at the represented-crossing entry

The retired-flat rejection-witness gate is a property of the represented-crossing
DECISION, not of one caller. `transfer_into_carrier` gates its BIND lane;
`transfer_represented_boundary_value` (the RESULT-EDGE lane) must gate
symmetrically at its OWN entry.

- Place the gate at `transfer_represented_boundary_value` entry
  (`aggregates.rs:1165`), NOT at the `joins.rs:320` call site: when
  `value.contains_boundary_closure_environment()? && px8ds_retired_flat_order_enabled()`,
  do NOT take the represented path — reach
  `value.boundary_transfer_admissibility()?` (the raw-closure refusal,
  `boundary.rs:1047`), identically to
  `transfer_into_carrier`'s retired-flat fall-through.
- Why entry-locus, not site-locus: the exact failure was "a new consumer reached
  the represented transfer without the gate." Gating the entry closes that
  class; gating one call site leaves the next new consumer free to reintroduce
  the bypass. The symmetry with `transfer_into_carrier`'s gate is the tell that
  the check belongs with the decision. This is the single structural closure,
  not a point patch.
- Required behavior (exact expression is runtime's to write): flag ON +
  `boundary_environment` present -> `boundary.rs:1047` refusal; flag OFF ->
  represented result-edge path exactly as today (a strict no-op in production, a
  dead branch under const `false`).

## Deliverables

- **D1 — the single-locus entry gate.** Add the retired-flat gate at
  `transfer_represented_boundary_value`'s entry per the repair ruling, mirroring
  `transfer_into_carrier`'s existing fall-through. Additive; no other consumer
  widened; no new refusal string.

## Acceptance criteria (the Architect's four guardrails, each with its control)

- **AC-D3-GREEN (the critical non-regression).** `rt_nested_ih_native_realization`
  stays GREEN (native == interp == Nat 3). The gate keys on the test-only flag
  and MUST be a strict no-op when the flag is off, so the legitimate D3
  represented result-edge crossing is untouched. Control: the D3 parity test
  passes unchanged; the flag-off path is byte-equivalent to today.
- **AC-PX8DS-REFUSES.** `px8ds_retired_flat_order_does_not_gain_m4_representation`
  returns to the exact `boundary.rs:1047` refusal, count == 1. The test is
  CORRECT and stays as-is — do NOT rebaseline or weaken it. Control: with the new
  gate removed, px8ds reddens again (the control controls) — a load-bearing
  mutation on the gate must red px8ds.
- **AC-DEAD-IN-PRODUCTION.** Confirm the gate is dead under
  `px8ds_retired_flat_order_enabled() == const false` in production artifacts (no
  new production path).
- **AC-ADDITIVE-FAIL-CLOSED.** Additive, fail-closed, ONE gate; no new refusal
  string, no other consumer widened.

## No-regression closure (targeted; CI is the workspace verdict)

Re-run the affected-target closure for the runtime lowering path (every target
loading the changed lowering module, plus `rt_nested_ih_native_realization` and
the `px8ds` control), scoped by changed PATHS via `scripts/ken-cargo`, NEVER
`--workspace`. Green in CI is the workspace verdict. Do NOT run `rt_parity_native`
foreground — it is the ~24-min D5-ceiling test that holds the shared build lock
(COORDINATION §12); if it must run, run it BACKGROUND and targeted.

## Gate, reviewer, sequencing

`gate: none` node field (the runtime convention). The change is in the
boundary-transfer lowering path, so the MERGE carries **M4 TCB** (boundary
transfer path) at Steward routing. On the candidate: **runtime-leader owns the
repair + merge Decision**; **Architect required review** (the repair is the
Architect's ruling — verify the gate is entry-locus, symmetric, and a strict
production no-op) + **Runtime QA** on the exact SHA, then Steward M1-M4 ->
lieutenant. No Decision is required to RELEASE this node (the design is the
Architect's deductive ruling); the merge Decision is assembled from the Architect
+ Runtime QA votes on the candidate.

## Disjoint from RT-PARITY-NATIVE-GOLDEN-REBASELINE

The Architect ruled px8ds OUT of the golden-rebaseline node: px8ds is not stale,
it caught this. `RT-PARITY-NATIVE-GOLDEN-REBASELINE` is scoped to exactly the two
`rt_parity_native` absolute-id checked_ih goldens (already correct — px8ds is not
a member). This node is px8ds's disposition; the two nodes do not overlap.

## Contention

`crates/ken-runtime` lowering path (`aggregates.rs`, adjacent to `boundary.rs` /
`mod.rs` / `joins.rs`). Re-measure at pickup. The concurrent doc track touches
`library/` and `agent/`, disjoint. Lane 3 (foundation) is on `catalog/`,
disjoint. This is the runtime ring's work.

## Hard stop

Route to the Steward if the fix CANNOT be a strict production no-op (flag-off
path not byte-equivalent to today), if it appears to require touching any
production path, or if restoring the entry gate perturbs
`rt_nested_ih_native_realization`. Any of those means the repair as ruled is not
what the tree needs, not that the scope should bend.
