---
id: RT-ORIENTED-PARENT-EDGE-COMPOSITION
title: "Oriented subcontinuation plan validation checks that each frame's output interface equals the input of the frame after it in semantic order. In a chain those pairs are the parent edges, but in a tree post-order puts siblings side by side, so the check compares two unrelated answer types. PX11's exchange main has such a segment and refuses with E0079 checked_oriented_marker_location. Check child against parent along each ParentFrame edge, refuse sibling pairs by name in the invocation-local splice, and leave the retired flat order as it is"
status: ready
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: [PX11-NATIVE-STREAM-SOCKET-PARITY]
github: null
origin: "Architect ruling evt_5z0179ybnz8bd (arm E-tree) on PX11 Inc2's moved refusal (runtime-implementer evt_48drym1z9c0gw), after probe P-EP evt_7g3egfs47bkfj. The ruling scopes it to its own node, as for ELAB and INDEXED. L1 Linux ABI program, Steward order."
---

# Oriented frames compose along parent edges

## Objective

PX11's exchange main builds natively past plan validation. Every
oriented plan is checked child against parent along each `ParentFrame`
edge, and siblings are never compared.

## Settled inputs (main `3300cf185`; Architect `evt_5z0179ybnz8bd`)

- **The witness.** On Inc2 rebased onto `3300cf185` (`f770a5e6e`),
  exchange main's segment 0 has 14 frames. Frame 11 has the children 12
  and 13, so the order is `[12, 13, 11, ...]`. The window 12 to 13
  compares the `ITree` answer type `Result ResourceError Bytes` with
  `Result ResourceError ReadProgress`. All 13 parent edges compose.
- **Plane 1.** `OrientedSubcontinuationPlanV1::validate`
  (`oriented_subcontinuation_plan.rs` `:718`, and the by-segment loop at
  `:746`-`:763`).
- **Plane 2.** The invocation-local splice (`cranelift_backend/lowering/
  mod.rs`, the `frame_ids.windows(2)` loop at ~`:10945`).
- **Plane 3.** The retired flat order (~`:10963`), behind
  `px8ds_retired_flat_order_enabled`, stays unchanged by ruling.
- **The code.** The ruling gives the exact replacement for planes 1 and
  2, including the per-segment orientation rule. It keeps the error text
  "checked endpoints do not compose" byte for byte. Build it as written.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Planes 1 and 2 as ruled, the existing pins reconciled
(`occurrence_exact_but_semantic_order_reversed_rejects`, `control.rs`
~`:1510`, `host_call_carrier.rs` ~`:4168`/`:4175`, the `erasure.rs`
~`:9476` message), and the rows below. No change to the exchange
fixture or profile, to interface construction, to plane 3, or to the
synthetic fixtures' orientation.

## Acceptance

The ruling's rows, built as written:
- **AC-1.** Plane-1 units on the root-with-two-children shape: (i)
  validates, (ii) refuses on a bad edge, (iii) refuses a mixed
  orientation, (iv) `reversed_rejects` stays red. m1 (the old windows
  loop) reddens (i), and m2 (no edge compare) reddens (ii).
- **AC-2.** A plane-2 unit with siblings where `A.out == B.in` refuses
  with "pairs frames with no parent edge". m3 (no `parent_edge`)
  reddens it.
- **AC-3.** The HS7 two-arm recursive branch compares native against
  the interpreter. Report whether
  `assert_option2_unrelated_owner_child` still reaches its degenerate.
  Do not edit that test's retirement.
- **AC-4.** The existing `ken-runtime` oriented tests and the targeted
  `rt_parity_native` rows pass unchanged.
- **AC-5.** Exchange main builds to an artifact on Inc2 rebased onto the
  candidate, as a measurement that is not committed. Report its arm.
- **AC-6.** Full-workspace CI is green on the exact candidate.

## Stop conditions

The ruling's arms N-sib, N-skip, N-owner, N-mixed and N-other go to the
Architect. Do not relax plane 2, migrate fixtures, or restore the
retired arm.

Checks: 2 fired. The ruling names all three planes, and the frame scopes
the two that change. 8 fired. AC-2's siblings are built so that the old
check passes, so the row separates the edge guard from the interface
compare. 4 fired. I read the plane-1 and plane-2 anchors on `3300cf185`.
