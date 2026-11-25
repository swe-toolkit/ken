---
id: RT-NATIVE-RESULT-JOIN-PHASE
title: "A Result-returning function whose Ok payload is a borrowed Bytes argument, built under nested matches on specialized scalars, refuses at native-build when an outer match consumes it: the planner plans the outer join's lanes as native scalar and lowering produces a carried boundary word. The interpreter runs it, and builds older than RT-NATIVE-BYTES-TO-LIST refuse it the same way. Name from an AC-0 which side is wrong at the join, repair that side, and land the socket_unix_path parity rows that this refusal blocks"
status: merged
owner: runtime
size: M
tier: T1
gate: architect
depends_on: [RT-NATIVE-BYTES-TO-LIST]
blocks: [PX11-NATIVE-STREAM-SOCKET-PARITY]
github: null
origin: "Architect ruling evt_73g3jcqvh0a2s on RT-NATIVE-BYTES-TO-LIST's p2 join stop (language-implementer evt_2kbee0a827zvr): a pre-existing defect outside that WP's lowering, to its own issue. Steward split evt_74tcgq02wyxyk: the socket_unix_path rows and p2 move here; the Runtime ring runs it while PX11 Inc2 holds. L1 Linux ABI program, Steward order."
---

# Native result join over a borrowed payload

## Objective

A Ken program in which a `Result IOError Bytes` function returns its
borrowed `Bytes` argument as the `Ok` payload, from inside nested matches
on specialized scalars, builds natively when an outer match consumes that
result. Its exit status agrees with the interpreter. That covers the
prelude's `socket_unix_path` as written.

## Settled inputs (main `097517427`; Architect probes under `/workspaces/ken/tmp/arch-subst-coinc-wt/zz-arch-u8probe/`)

- **The refusal.** `jump_planned_join_arm`
  (`crates/ken-runtime/src/cranelift_backend/lowering/joins.rs` ~`:381`)
  refuses with PlannerInvariant "planned native scalar lanes but lowering
  produced a carried boundary word" when a `NativeScalarPair` join receives
  a `LoweringOperand::Carried` arm. The planner computes the result phase in
  `summarize_result_phase` (`planning/static_transition/joins_traps.rs`
  ~`:286`). In p2's build the refusing join is `StaticOriginId(101)`.
- **Witnesses.** Each has the same `main`: match `argv`, and in the `Cons`
  arm `match check path { Err _ ↦ Failure 75; Ok _ ↦ Success }`. Only
  `check`'s body varies, and it always returns `Result IOError Bytes`.
  - q9, the minimal one: two nested `leq_int` matches on
    `bytes_length path`, then `Ok IOError Bytes path`. It has no
    `bytes_to_list`, no list, and no `uint8_to_int`.
  - q8: a `bytes_at` `Option` match, then `eq_int` against 47, then
    `Ok ... path`.
  - q6: `socket_unix_path`'s body copied locally, without the NUL walk.
    Three binaries built from main before RT-NATIVE-BYTES-TO-LIST refuse it
    identically.
  - p2: the prelude's `socket_unix_path` itself. It reaches this join only
    once RT-NATIVE-BYTES-TO-LIST removes the earlier refusals.
- **Controls, which build.** q7 is q6 with a static payload
  (`bytes_encode "k"`). q1 and q2 have one match on a unit-call `Bool`,
  then either `Ok path` or a static payload. So the trigger is the borrowed
  payload under nested scalar matches.
- **Prior repairs at this refusal.** `RT-JOIN-PHASE-CASE-BINDER-CARRIED`
  (closed) planned a carried case binder's join so that it was not
  `NativeScalarPair`. `RT-JOIN-SCALAR-PAIR-NONSCALAR-RESULT` (merged)
  planned a constructor-with-fields result from its shape as well as its
  phase. Neither covers a borrowed argument as a constructor payload.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The planner's result-phase computation or lowering's operand for the inner
`Result` join, whichever AC-0 names, and the parity rows. No prelude change,
no fixture rewrite to avoid the join, no new primitive, and no change to the
refusal's text or to the PlannerInvariant arm.

## Acceptance

- **AC-0 (Architect, before code).** Name which side is wrong at the join,
  using p2's `StaticOriginId(101)` and q9:
  - Is the planner's scalar plan for the outer join wrong?
  - Or is lowering wrong to give the inner `Result` join a carried result?

  Then rule the repair on that side. Check whether the two prior repairs
  above missed this sibling, and sweep that repair's other call sites by
  the same criterion. Do not presume a fix before the side is named.
- **AC-1.** Parity rows, interpreter against native, each with a known exit
  status:
  - q9, q8 and q6;
  - `socket_unix_path` on an absolute path, a relative path, an over-length
    path and a NUL-bearing path, each giving its ruled result. These rows
    moved here from RT-NATIVE-BYTES-TO-LIST, and the NUL row needs its G2;
  - q7, q1 and q2 as controls that still build and agree.
- **AC-2.** Revert the repair: q9 refuses again at native-build with the
  same PlannerInvariant, and the controls still build. Restore it
  byte-identically.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The repair needs a prelude change, a new primitive or a TCB change.
- AC-0 finds both sides consistent with their own contracts. Then the
  contract between them is unspecified: report it to the Architect and do
  not choose one.

Checks: 3 fired. I grepped `docs/program/issues/` for the refusal text and
for join-phase nodes. Two earlier repairs sit at this refusal, one closed
and one merged, and neither names a borrowed payload, so AC-0 checks them
as siblings. 1 fired. "Pre-existing" rests on three binaries built from
older main trees, independent of the candidate. 8 fired. The controls
differ from the witnesses only in the payload or in the scalar nesting, so
a repair that just removes the refusal shows up in AC-2's revert.
