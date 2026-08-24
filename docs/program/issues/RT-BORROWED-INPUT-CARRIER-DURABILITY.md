---
id: RT-BORROWED-INPUT-CARRIER-DURABILITY
title: "Execution-parity successor — give a borrowed process input (BorrowedOpaque) a durable carrier representation on the generic carried-value path, so a capture that crosses the closure boundary does not trap at run with `malformed borrowed process input` (object_linker_packaging.rs:2221, native stub value -1)"
status: draft
owner: runtime
size: M
gate: none
depends_on: [RT-CLOSURE-BOUNDARY-RESIDUAL]
blocks: []
github: null
origin: "Steward, 2026-08-24, from the Architect's M4-stop ruling (evt_2dnst700ynbeh) on WIP checkpoint 422310a32. M4's closure-crossing contract is discharged; the post-crossing execution trap is a DISTINCT execution-parity seam, not unfinished M4. Steward framing call (Architect left own-node-vs-fold open): a DEDICATED node, because the nearest family member RT-ENTRY-TRAP-254 is closed and specific (value 254) and CI-SKIPPED-NATIVE-TESTS is CI-coverage tracking, not a mechanism owner. Steward-filed per COORDINATION section 2."
---

> # Execution-parity successor — gated behind M4's crossing

## Objective

A `BorrowedOpaque` is a borrowed process input: runtime-local and live-domain
only, with no durable carrier representation. When such a value is captured and
routed across the closure boundary (now that M4's crossing works), it survives
lowering/construction but TRAPS at execution. Give the borrowed process input a
durable carrier representation on the generic carried-value path so the capture
runs correctly end-to-end.

This is the SAME "no durable lane" root cause as the original closure-boundary
refusal, but for a DIFFERENT object: M4 gave the closure its durable lane; the
borrowed process VALUE the closure captured still has none. Building that
representation is a new mechanism on the shared carried-value path, not a member
M4's world already has — the exact M6→M3 pattern (the crossing succeeds and
EXPOSES a deeper, structurally-distinct seam).

## Fixed inputs (measured @ `011bf2a95` / WIP `422310a32`, verified)

- The trap is EXECUTION-layer, owned by the native C stub at
  `crates/ken-runtime/src/object_linker_packaging.rs:2221`:
  `if (value == -1) fputs("ken native trap: malformed borrowed process input\n",
  stderr);`. It is a sibling of the RT-ENTRY-TRAP-254 malformed-payload family
  (that one value 254; the ExitCode family value -3), on the same execution
  layer the Architect named as a distinct residual in the post-M6-landing
  assessment (evt_1vcwzkd3g0s1r). Empty effect trace + RuntimeTrap(1) confirm it
  fails after object emission, at run.
- The surface is a GENERIC SHARED mechanism, not M4's code: `emit_carrier_tag`
  is defined at `crates/ken-runtime/src/cranelift_backend/lowering/mod.rs:7346`
  and called from effects/joins/source and the carried-match at
  `core.rs:12270`. M4's closure transfer merely ROUTES a capture into it — a fix
  here changes a path many non-M4 flows depend on, which is why it is not M4's
  same-contract work.
- Measured trigger (fixed input): a `BorrowedOpaque` capture → the generic
  carried-match `emit_carrier_tag` → the native wrapper value `-1`. First
  observed on the two direct-result `px8l_recursive_decl_native` rows at WIP
  `422310a32`, which cross the boundary honestly (M4 works) then red only here.
  The static-dispatch capture run projected faithfully (`Int, Constructor,
  BorrowedOpaque`, not collapsed/reordered), so the fault is the borrowed-input
  durability, not a capture-projection defect.
- Test assertion sits at
  `crates/ken-runtime/src/cranelift_backend/lowering/core/tests/effects.rs:1112`.

## Relationship to M4 and the re-pointed rows

M4 (`RT-CLOSURE-BOUNDARY-RESIDUAL`) re-points the ignore string of any censused
row that crosses but reds ONLY at this borrowed-input execution seam to THIS
node as owner (per the Architect disposition, exactly as M6 re-pointed
`rt_write_writable_stage` to M3). Those rows carry forward as honest advancing
refusals, not regressions; they green here once this seam lands. As of the M4
stop, at least the two direct-result `px8l_recursive_decl_native` rows re-point
here; M4's widening measurement (its Deliverable 2) determines the full set.

## Sequencing and caution

Draft, execution-parity family. Its resolution greens the re-pointed rows, so it
follows M4's crossing landing (depends_on). Distinct from M3
(`RT-CARRIED-IH-DISPATCH-SITEOP`, the CarriedWord/ConstructorTag lowering seam)
and from M4 (the closure-boundary crossing): this is the shared carried-value
EXECUTION layer. Do not collapse the three because they neighbour on the
carried-value path — they are different objects and different layers. Whether it
stays standalone or later joins the execution-parity family umbrella is a
post-framing call; the Architect reviews the WP when it is framed and released.
