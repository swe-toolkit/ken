---
id: RT-NATIVE-READEOF-WITNESS-FOLD
title: "Two-quarter native carried-value fold: un-ignore the ReadEof full-program row via a new zero-TCB rt_cap41 executing native-closure witness, and confirm the already-green SemanticErrorV1 (InvalidOffset) row stays rearmed. Honest progress toward RT-NATIVE-CARRIED-VALUE's four-value PX8 closure; Wrote and ReadSome remain #[ignore]d under the composed-return arc and this node does NOT close the parent."
status: active
owner: runtime
size: S
gate: none
tier: T1
depends_on: [RT-EFFECT-CONTINUATION-WRITE-NARROWING]
blocks: []
github: null
origin: "Steward-framed 2026-09-05 from the Architect's closure-fold decomposition evt_5hwps9bch4zzn (grounded at origin/main f16eb0de8), which reshaped the Steward's committed 'final four-value closure fold' sequencing into a TWO-quarter fold. Correction absorbed (Architect, verified from SHA): the write-narrowing greened SemanticErrorV1 (the exact InvalidOffset error) but did NOT green a successful Wrote(n); the Wrote rows are still #[ignore]d and sit on the composed-return arc, the same arc as ReadSome. So Wrote gates WITH ReadSome, not with the ready set. RT-NATIVE-CARRIED-VALUE stays active/PX8-blocking; this is a step toward it, not its closure."
---

> # ACTIVE — RELEASED. Lane 1 (runtime). Two-quarter fold, tier T1, size S,
> gate=none, backend-only, zero TCB. Base = current main f16eb0de8. RE-MEASURE
> every line coordinate at that base before editing (the numbers below are the
> Architect's at f16eb0de8 and decay; the semantic anchors — rt_cap41_endpoint_stage,
> rt_cap41_expect_eof, rt_read_offset_stage, the differential() harness,
> assert_narrowed_alike, operation_events — are stable).
>
> Mechanism front-loaded by the Architect (evt_5hwps9bch4zzn); the ring builds,
> the Architect reviews the candidate. The standing runtime hard-stop protocol
> applies: the D0 contingent below is a real gate — a native/interpreter
> divergence at effective=0 is a runtime seam and a HARD-STOP, not something to
> paper over.

## Objective

Un-ignore TWO of RT-NATIVE-CARRIED-VALUE's four native full-program rows and
lock them in:

- ReadEof — NEW work. It has a fixture (rt_cap41_endpoint_stage) but no
  executing witness; add one.
- SemanticErrorV1 (the exact InvalidOffset error) — ALREADY green from the
  write-narrowing (RT-EFFECT-CONTINUATION-WRITE-NARROWING); confirm it stays
  rearmed as this fold lands.

Wrote and ReadSome are explicitly OUT of scope: both are #[ignore]d and both sit
on the composed-return arc (Wrote: px8f_buffer_native.rs:345 keyed to
RT-RESULT-CONTINUATION-BINDING-PROVENANCE, and px8f_write_partition.rs:354 keyed
to RT-CLOSURE-BOUNDARY-RESIDUAL; ReadSome: the same RT-RESULT-CONTINUATION-
BINDING-PROVENANCE key). They ride that arc separately and are not touched here.

This node does NOT close RT-NATIVE-CARRIED-VALUE (whose closure condition is all
FOUR native rows green + un-ignored) and does NOT unblock PX8. It is honest
progress that locks in and guards the two ready values while naming the held
pair.

## Why ReadEof is separable now (Architect, verified from SHA)

ReadEof rides the single readAt + (\outcome. ...) path: its value is
ResourceBodyOk carrying NO BufferSpan, no count, and no retained result closure,
so it never enters the "CheckedIhCapturedEnvironment where a fresh R2 belongs"
case that the composed-return arc is about. That is why it can land ahead of
Wrote/ReadSome without touching that arc. It is also categorically cheaper than
the write-narrowing: it needs NO lowering change (cold-lowering already reports
rt_cap41_endpoint_stage Completes, and the structurally-isomorphic green read
stage rt_read_offset_stage executes green), so there is no responses.rs /
construction.rs edit here.

## Mechanism (Architect-ruled evt_5hwps9bch4zzn)

The fixture rt_cap41_endpoint_stage already exists in RT_PARITY_SOURCE
(rt_parity_native.rs:432): readAt file 0 buffer (MkBufferWindow 8 4) on a
capacity-8 buffer, so effective = min(4, 8 - 8) = 0, which yields Ok ReadEof,
matched by rt_cap41_expect_eof (:341). It is UNSELECTED — no differential() /
#[test] runs it (every read differential points at the green,
structurally-isomorphic rt_read_offset_stage :180).

Add ONE executing #[test] that selects rt_cap41_endpoint_stage through the
existing differential(case, entry) harness (build_native_program +
run_bound_process_effect_observation vs the PosixHost interpreter), asserting —
adapting assert_narrowed_alike for a SUCCESS value — that:

- both engines exit 0, with no terminal_error;
- effect-trace parity: non_release_events equal AND release_set equal;
- the spec-load-bearing axis: ZERO FsReadAt events. spec/30-surface/38-ffi-io.md
  step 4 (§1.7, contract-pinned) says effective=0 returns ReadEof "without
  emitting a private read operation or visiting the host." The existing
  operation_events(...).is_empty() axis encodes exactly that no-host-visit
  requirement.
- co-indexed to the same request / span / buffer as the other native rows.

## The one contingent (measure, do not assume)

Cold-lowering proves rt_cap41_endpoint_stage BUILDS, not that it RUNS producing
exactly Ok ReadEof with no host visit. The ring MUST add the test and run it
targeted (scripts/ken-cargo -p ken-cli --test rt_parity_native, filtered) before
claiming the quarter. If native diverges at effective=0 — emits an FsReadAt, or
the value differs from the interpreter — that is a real runtime seam and a
HARD-STOP to the Architect -> Steward -> (if it needs a runtime lowering change)
the runtime ring, and it leaves the zero-TCB fold. All current evidence predicts
clean green (the isomorphic read stage is green, the spec step-4 admission ladder
is implemented, the interpreter half landed). D0 below IS this contingent gate.

## Deliverables (one-hour-turn atomic increments)

- D0 MEASURE (also the contingent gate). Run rt_cap41_endpoint_stage on both the
  native engine and the interpreter; confirm Ok ReadEof + zero FsReadAt +
  effect-trace parity. A divergence here fires the hard-stop above.
- D1 WITNESS. The ReadEof witness #[test] (mechanism above) plus a discriminating
  control: a sibling stage (rt_cap41_out_of_range_stage) must red / InvalidBounds
  so the witness distinguishes ReadEof from the error variants — the positive
  assertion is non-vacuous.
- D2 UN-IGNORE + CI-REARM. Un-ignore the two rows: ReadEof newly green; confirm
  SemanticErrorV1 (InvalidOffset) stays rearmed. Wrote and ReadSome stay
  #[ignore]d under their named composed-return-arc keys (px8f_buffer_native.rs:345
  and px8f_write_partition.rs:354).

## Acceptance criteria

- AC-READEOF-GREEN. rt_cap41_endpoint_stage is green: Ok ReadEof, exit 0, zero
  FsReadAt events, effect-trace parity (non_release_events + release_set), and
  co-indexed to the same request/span/buffer.
- AC-NECESSITY-CONTROL. The discriminating control reds on the out-of-range
  sibling (InvalidBounds), so the ReadEof witness is proved non-vacuous — a
  neutered witness that also passed the error case would fail this AC.
- AC-SEMANTICERR-REARMED. The SemanticErrorV1 (InvalidOffset) row stays green /
  rearmed across this fold; the write-narrowing result is not regressed.
- AC-HELD-PAIR-NAMED. Wrote and ReadSome remain #[ignore]d under their named
  composed-return-arc keys; the node names the held pair and makes NO "closeable"
  claim on PX8.
- AC-NODE-STAYS-OPEN. RT-NATIVE-CARRIED-VALUE stays active and PX8-blocking; its
  closure condition (all four native rows green + un-ignored) is NOT met by this
  node.
- AC-ZERO-TCB-NO-LOWERING. Zero TCB and no-lowering-change, asserted: kernel tree
  hash unchanged; no responses.rs / construction.rs (or other lowering) edit; the
  change is a test-only selection of an existing fixture through existing native
  execution and existing ReadEof / ResourceBodyOk / MkBufferWindow constructors
  against the locked spec.
- gate=none, backend-only, zero TCB.

## Reviewers

Builder: runtime-implementer (T1 seat; the D0 contingent judgment is the
reasoning content). Reviewer: Architect (front-loaded the mechanism; reviews the
ReadEof witness, the necessity control, zero-TCB / no-lowering-change, the
held-pair naming, and the contingent gate). Independent mechanics reviewer:
runtime-qa. Plus CI green on the exact SHA. Merge via Steward M1-M4 -> lieutenant
M5-M9.
