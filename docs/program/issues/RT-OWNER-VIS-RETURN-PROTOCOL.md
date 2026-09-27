---
id: RT-OWNER-VIS-RETURN-PROTOCOL
title: "Carry a grafted continuation's Vis and its K back across a generated response-owner boundary, so the owner accepts a conforming non-Ret result instead of trapping at its Ret-tag check"
status: active
owner: runtime
size: L
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Architect scheduling fact 2026-09-27 (evt_6f0ms4dg8c99k): three L1 ignored rows wait on the one representation decision RT-PLANNER-KRET-GRAFTED-SPINE parked at structural stop 16 (evt_6freqarew56yh), which said reopening requires a separately framed architecture decision grounded on current main. Operator L1 directive 2026-09-17 (clear the ignored tests). Steward-filed per COORDINATION section 2."
---

# Carry the owner's Vis result across the generated boundary

## Objective

Clear the three L1 rows that stop at the response-owner Ret-tag check.

## Settled inputs -- measured at `6fed125f5` and partial `129ecd207`

- **The rows** (Runtime leader `evt_4ecz57wqx1w9k`; an exact-site marker
  changed each row's `-1` to the check's own marker):
  - `crates/ken-cli/tests/px7f_resource_native.rs`
    `linked_public_right_denial_preserves_exact_masks` (owner Vis 578);
  - same file, `linked_public_second_release_is_closed_and_the_handle_closes_once`
    (owner Vis 609);
  - `rt_escape_second_resource_native.rs`
    `r2_cross_buffer_freeze_fails_closed_with_invalid_bounds` (owner Vis
    1298; reached only with the BufferFreeze carried-seat partial landed).
- **The wall.** `require_i64(ret_tag, expected_ret)` at
  `lowering/units.rs:3663-3671`: the owner expects an immediate `Ret`, and
  the conforming grafted continuation returns a `Vis`. Spec 36 §2.2 (bind
  grafts) and 42 §6.4 fix that tree shape; the runtime must follow it.
- **Denied routes** (stop 16): the four existing representation-shaped exits.
  Full reconstruction adds unprovisioned carrier state, and each
  allocation-free word erases either the `Vis` or its K. The persistent
  closure lane stays withheld (`dec_21aa95jbsznfh`, `dec_6xffebwj4s347`).

## Deliverable

A return protocol for generated response owners, designed at D0, ruled by the
Architect, and built, that un-ignores all three rows.

## Acceptance

- **AC-0 (D0, design; no build).** The ring proposes the carrier, grounded on
  current `main`: what state crosses the boundary, where it lives and for how
  long, how the owner resumes the carried K, and why it is none of the
  denied routes. Cite the spec clauses it implements. The Architect rules on
  it before any build.
- **AC-1.** All three rows run green and un-ignored, with both engines
  agreeing.
- **AC-2 (control).** Removing the carrier re-reddens each row at the Ret-tag
  check; then it is restored.
- **AC-3.** No other row changes colour. Targeted suites only, through
  `scripts/ken-cargo`; no-regression means green in CI.

## Stop conditions

- Any kernel, `trusted_base()` or spec change is an operator question.
- A design that reopens the withheld closure lane is an Architect stop.
- If a row needs a second, independent repair after the protocol, land the
  rows the protocol clears.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.
