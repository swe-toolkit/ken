---
id: RT-NATIVE-SEQUENTIAL-BRACKETS
title: "Native execution of two sequential resource brackets: a program that uses two withBuffer brackets in sequence builds and runs natively with the observation the interpreter gives, instead of refusing in object emission at the BoundaryCarrier carried-recursive-hypothesis arity check"
status: active
owner: runtime
size: M
gate: architect
tier: T1
depends_on: [RT-IGNORED-ROWS-NEXT-GROUP]
blocks: []
github: null
origin: "Architect ruling 2026-09-28 (evt_1ys064x0c11be) on RT-IGNORED-ROWS-NEXT-GROUP's A1 blocker (runtime-implementer evt_5nmkjfcnemamh): native execution of two sequential brackets is the successor, framed from the verbatim refusal and the one-bracket control. Steward-filed per COORDINATION section 2."
---

# Native sequential resource brackets

## Objective

A native program can use two resource brackets one after the other.

## Settled inputs (runtime-implementer `evt_5nmkjfcnemamh`, on the NEXT-GROUP repair at `c569eef94`)

- The AC-0b witness (`withBuffer 1 ; withBuffer 1`) gets past both
  host-response refusals once the occurrence-key repair is in. It then
  refuses in object emission:
  `unsupported runtime-IR lowering: BoundaryCarrier: a carried recursive
  hypothesis is an eliminated value, not a callable, so it takes no
  arguments, but the call provides 1`.
- The message comes from `reject_carried_residual_arguments` in
  `crates/ken-runtime/src/cranelift_backend/lowering/core.rs` (`:3128`).
- **Controls.** One bracket with the same body compiles (exit 0). The same
  refusal appears when the second bracket is moved into a separately named
  `proc` with capacity 6.
- **Ruled INDEPENDENT of the route selection** (Architect
  `evt_20tgpkchtnck2`). The refusal persists under the "last by origin"
  selection, in which no route hands the refusing call to any Vis.
- **The site, pinned.** It is the source-machine guard at
  `crates/ken-runtime/src/cranelift_backend/lowering/source.rs:5015`
  (arguments 1, funcid 44, owner `PredeclaredFunctionId(3)`,
  `pending_application` None), reached from call `StaticOriginId(187)`.
  - In the witness plan, 187 is CM12's Vis-case dispatch continuation call:
    IHInvocation188 -> Let191 -> leaf Match309 -> root Match312 ->
    IHSlots313 -> CM12 Vis.
  - The refusal needs the second bracket's CM318 nested in CM12's Ret case,
    which is why one bracket compiles.
  - Logs: `/tmp/rt-ignored-build/a1-{repaired-instrumented2,first,last}.log`.
- **Possible overlap.** Both `rt_escape` rows also use two brackets
  (`withResource` then `withBuffer`). `RT-NATIVE-TREE-MATCH-RUNTIME-SCRUTINEE`
  may reach this refusal next. AC-0 checks that, so that the two nodes do not
  each repair the same site.

## Deliverable

The witness, with its two brackets distinguishable at runtime, builds and
runs natively. Its exit code or output encodes both brackets' outcomes, and
it matches the interpreter. The repair is ruled by the Architect.

## Acceptance

- **AC-0 (probe, then D0; no build).**
  - Re-measure the witness and the one-bracket control on the landed
    NEXT-GROUP repair (Check 4).
  - At the refusal, report the call's origin, its callee, and why lowering
    treats that callee as a carried recursive hypothesis.
  - Say whether either `rt_escape` row reaches the same site.
  - Propose the repair. The Architect rules before any build.
- **AC-1.** The witness runs natively with the expected observation. The
  NEXT-GROUP A2 fixed-order mutations change that observation rather than
  refusing.
- **AC-2 (control).** Reverting the repair returns the witness to the AC-0
  refusal. The one-bracket control stays green.

## Stop conditions

- Any kernel, `trusted_base()` or spec change (an operator question).
- Relaxing the arity check for a value that really is an eliminated
  hypothesis is an Architect stop.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.

## Hard-stop inventory (§1b)

§1a count: 1 (Architect `evt_2htrfrdpq3wy`).

1. constructed context frame keyed on planner coordinates (continuation,
   position, body) — keyed on static coordinates, but one function
   constructs the same coordinates more than once with different operands.
