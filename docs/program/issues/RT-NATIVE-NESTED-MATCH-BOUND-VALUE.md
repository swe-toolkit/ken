---
id: RT-NATIVE-NESTED-MATCH-BOUND-VALUE
title: "A native build refuses at ObjectEmission when a value bound by one computational match's pattern is used inside the arm of a later, nested computational match, with 'a computational recursor closure names an in-flight activation, not a transferable value'. Every native pipe write needs that shape, so ResourceStreamWrite cannot be promoted until it lowers. Repair the continuation lowering so the write-free and write-bearing counterexamples build and match the interpreter, with the unchanged live-child control still green"
status: merged
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: [RT-NATIVE-RESOURCE-STREAM-WRITE-PROMOTION]
github: null
origin: "Architect evt_26bftsyspq7gn (R1) on RT-NATIVE-RESOURCE-STREAM-WRITE-PROMOTION Increment 2, measured by runtime-implementer evt_7a9k2sxwf995 (Q) and evt_8zs5xyr15t34 (S1-S3 ladder). The repair belongs to the RT-LEXICAL recursor-consumer lowering lane, not to a label promotion. Steward-filed per COORDINATION section 2."
---

# A match-bound value used in a nested match arm lowers natively

## Objective

A Process program that binds a value in one computational match's arm and
uses it inside the arm of a nested computational match builds natively, and
its native run matches the interpreter event for event. After this node,
`RT-NATIVE-RESOURCE-STREAM-WRITE-PROMOTION` Increment 2 resumes on its
landed fixture as ruled in `evt_2bvvckavp77xm`.

## Settled inputs (main `cd9f3f907`)

- **The refusal.** `Lowered::ComputationalRecursorClosure`'s arm of the
  boundary-transfer walk refuses
  (`crates/ken-runtime/src/cranelift_backend/lowering/boundary.rs:1052-1056`,
  with its disposition twin at about `:1287`), at stage `ObjectEmission`,
  field `checked_process_object`, `source_cause: Some(Fragment)`. It fires
  when a match continuation that names an in-flight activation of its
  recursor is lowered to a value and handed across a unit boundary. The
  RT-LEXICAL fusion (`lowering/core.rs`, `D2f`, near `:4495`) removes this
  for the producer and consumer pairs it already claims.
- **The write-free counterexample Q.** `px10_live_child.ken` with
  `close_pipe_read` imported and one insertion: in the
  `Ok (ReadProgress.ReadSome _ _)` arm, bind `close_pipe_read AFull pipe`
  before the unchanged `wait_child` bind. `pipe` is bound by the outer
  `match taken`. The reference run exits 0 with 17 canonical events; the
  native build refuses as above. The source is at
  `/workspaces/ken/tmp/rt-f2-i2-Q-evidence/Q.ken` (sha256 `2982ed1e...`).
- **The write-bearing counterexample S3.** The landed
  `lang_process_child_pipe_write_home.ken` with all four procs inlined
  (`/workspaces/ken/tmp/rt-f2-i2-shape-ladder/S3.ken`, sha256
  `24286c27...`). `span` from `ReadSome` is used by `write_pipe` inside the
  arm of `match stdin_result`, and `writable` is used by `close_pipe_write`
  inside the arm of `match written`. Its reference equals the landed
  fixture's 19 events, and its native build refuses the same way, as do
  S1 and S2.
- **The control.** `px10_live_child.ken` unchanged builds and matches
  natively (`rt_native_settlement_placement_key.rs`
  `f_live_child_matches_fresh_interpreter`, same RootExecution substitution
  and 80-node profile).
- **Why no fixture routes around it.** `write_pipe` needs a `BufferSpan`,
  which only a `ReadSome` arm mints, and a `PipeWrite`, which only
  `take_stdin`'s `Ok` arm yields (Architect `evt_26bftsyspq7gn`, check 4).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The continuation lowering for the nested-match-bound-value shape and its
native rows. No availability label change (`ResourceStreamWrite` stays
`RepresentedUnavailable`), and no spec, catalog, ABI, authority or trust
change.

## Deliverable

One runtime candidate.

## Acceptance

- **AC-0 (D0, Architect, before lowering code).** On `cd9f3f907`, name
  which lowering site materializes the outer arm's continuation when the
  nested match's arm uses its bound value, and rule the repair: an
  extension of the existing fusion's claimed pairs, or a distinct route.
  The D0 also states whether the population is wider than these
  counterexamples, measured by a census over the native Process fixtures,
  not by reading shapes.
- **AC-1.** Q, committed as a fixture, builds natively, and its native run
  equals its reference run event for event, with the same exit status and
  terminal outcome.
- **AC-2.** S3, committed as a fixture with `ResourceStreamWrite` still
  unavailable, builds natively, and its native run stops at the write with
  `NativeEffectRunErrorV1::OperationUnavailable { operation:
  ResourceStreamWrite }`, not with a lowering refusal.
- **AC-3.** The unchanged live-child control and the landed early-close
  read and write rows stay green. A disposable revert of the repair turns
  AC-1 and AC-2 back to the measured `ObjectEmission` refusal; restore it
  byte-identically.
- **AC-4.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The repair needs an ABI, availability-label, planner-authority or trust
  change.
- A native run differs from its reference at any event.

Checks: 8 fired. Q and its control differ by one native-tested close of an
outer-match-bound value inside a nested arm, with no write and no span, so
AC-1 fails exactly when the nesting still refuses. AC-2 requires the run to
reach the write's unavailability, so a build that never lowers the write
cannot pass. 6 fired. The shape claim is measured over four landed Process
fixtures plus Q and S1-S3, which is why AC-0 censuses the population before
the repair.
