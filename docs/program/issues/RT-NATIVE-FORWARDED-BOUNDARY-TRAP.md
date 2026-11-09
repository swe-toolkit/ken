---
id: RT-NATIVE-FORWARDED-BOUNDARY-TRAP
title: "A pure-forward carried clause at its function's outermost carried position lowers its IH in place, jumping its own header with k, so F's body-tail request never reaches the enclosing genuine handler and the checked-answer fallback releases Buffer1 early. Until the re-entry continuation exists, replace that one edge with a named trap, so F refuses natively by name at the boundary clause and every control stays green"
status: draft
owner: runtime
size: M
tier: T1
gate: architect
depends_on: [RT-NATIVE-SETTLEMENT-PLACEMENT-KEY]
blocks: [RT-NATIVE-FORWARDED-CONTINUATION-REENTRY]
github: null
origin: "Steward recut 2026-11-09 20:45Z (evt_1g0fjah621vx3) on the Architect's bisection ruling (evt_3jatzr9kk7e11): the fail-closed trap leaves RT-NATIVE-SETTLEMENT-PLACEMENT-KEY and returns as its own node, re-based on the placement authority. The trap's design is the Steward's 14:50Z recut (evt_6rkw3hdx6m2jn) on S0b RE-ENTRY-ABSENT (evt_6cww96v9b0vf2). L1 objective: the Linux ABI program. Steward-filed per COORDINATION section 2."
---

# Trap the boundary edge

## Objective

F's three rows refuse natively by the trap's name at the boundary
clause, after the inner pure-forward Vis case has run. Every control
stays as green as the placement authority left it.
`RT-NATIVE-FORWARDED-CONTINUATION-REENTRY` later replaces the trap with
the lawful forward.

## Settled inputs

- **The defect** (D2, `evt_6t5mkjt1y7znx`). F's body-tail request
  enters a pure-forward carried clause whose IH lowers in place. It jumps
  the header with k under the CHECKED control, both tags miss, and the
  checked-answer fallback releases Buffer1 early.
- **No re-entry unit exists** (S0b, `evt_6cww96v9b0vf2`). The chain is
  two clauses: S1165, then C986 in argv; S606, then C427 in live.
- **The trap is already written** at `91e7e83a0`, carried on the
  authority's base: `forwarding_boundary_trap` and
  `is_boundary_forwarding_arm` (`brackets.rs`). Today the decision is
  lowering-only (`core.rs:16479`), and the trap is interned
  unconditionally (`construction.rs:1661`).
- **The Process gate** refuses F before object emission on any
  production base (`compiler_driver.rs:3001`-`3024`), so AC-1 is
  measured under the uncommitted C-28 overlay (`evt_5vsaybryes08b`;
  Steward `evt_1kpgr76vwez5g`).

Treat anchors as perishable. If a settled input is false on the landed
authority, stop and report the mismatch.

## Deliverable

One candidate on the landed `RT-NATIVE-SETTLEMENT-PLACEMENT-KEY`,
stacked under B1a. The boundary arm's in-place IH edge, jump header(k,
1), becomes the named trap. The boundary arm is the pure-forward arm
(`brackets.rs:429` `forwarding_vis_arm`) whose clause is the outermost
carried ComputationalMatch of its emitting function: C986 in argv, C427
in live. Inner pure-forward clauses keep their by-value forward, and the
checked-answer fallback stays untrapped. The boundary decision is read
from the placement authority's record, never re-derived in a plane, and
the trap is interned only when a program has a trapped arm.

## Acceptance

- **AC-1 (measured under the overlay).** Each F row refuses natively by
  the trap's name at the boundary clause, after the inner S1165 or S606
  Vis case, in S0b M4's order, shown by ordered markers. The fallback
  count is 0. The overlay is named by sha256. QA re-runs it at the exact
  tip.
- **AC-1c (committed).** The F fixtures, with a committed native test
  whose expectation is today's production refusal naming the seven
  unavailable operations, for B2 to promote.
- **AC-2.** The authority's AC-1 controls stay green on the committed
  product, without the overlay. The trapped-arm count is 1 for each F
  row and is reported for every control; a control with a non-zero count
  still passes, and its arm is named.
- **AC-3.** Each mutation compiles and moves AC-1 off the trap's name:
  - M-T removes the trap and restores the early Buffer1 release;
  - M-P places the trap at the inner clause, and the ordered markers show
    the wrong placement.
- **AC-4.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A plane other than the lowerer needs to change for F to reach the
  trap. Post it as a stop, with its log, before repairing it.
- A kernel or trust change, or a change to the Process gate,
  `HostOpV1::availability` or AC-2g (B2's).

Checks: 2 fired: the trap's one decision site and its interning site
were read on `91e7e83a0` (`core.rs:16479`, `construction.rs:1661`); the
fan-in to the boundary decision is the planes the authority switches to
its record. 8 fired: AC-2 runs without the overlay, because the F-only
gates were green while the controls were red throughout the earlier
chain.
