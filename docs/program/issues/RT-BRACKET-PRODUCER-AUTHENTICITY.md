---
id: RT-BRACKET-PRODUCER-AUTHENTICITY
title: "CHILD 1 of 3 of RT-BRACKET-CONTROL-REGION-IR. The checked bracket producer and its AUTHENTICITY boundary. Capture the exact canonical template-call occurrence through checked lowering and wrap THAT occurrence; the compiler marker must be NON-RESOLVABLE FROM SOURCE; and the PRODUCER marks the acquire, acquired-body, final-settlement, outcome-merge and resume roles ITSELF, before inlining. Two shapes are REJECTED ON MEASUREMENT and must not be retried: ordinary prelude `proc` declarations as markers, because a Ken program can name `private_checked_resource_bracket_*` and the claimed compiler-private authority is therefore FORGEABLE (AC-4 unmet); and locating ports by scanning the expanded Runtime expression for an acquisition `HostOpV1` plus every `ResourceRelease`, because a LAWFUL PUBLIC EARLY RELEASE inside the bracket body adds a release and makes the finalizer port AMBIGUOUS. `HostOpV1` may VALIDATE already-marked ports; it may never DISCOVER which expression becomes a region. THIS CHILD DOES NOT LAND ALONE -- it is a held, reviewed input assembled by child 3."
status: draft
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: [RT-BRACKET-SETTLEMENT-PLANE]
github: null
origin: "Steward recut 2026-09-22 on the Architect's second WIP audit verdict evt_3rvns2yxm898r, which ruled RT-BRACKET-CONTROL-REGION-IR MIS-SIZED and directed a retained-work recut into three ordered children. The Steward owns the recut; the mis-sizing was the Steward's, not the ring's. The producer-authenticity boundary alone needs a finished design, controls and review before the planner or lowerer can safely depend on it. Retained work is preserved at evidence checkpoint 81f222b7f012829cd9f8d0f3dc684410a9b2b9ee on wp/RT-BRACKET-CONTROL-REGION-IR, parented on pre-registration commit c22f4861d which stays unchanged. That checkpoint is EVIDENCE AND REUSABLE WORK, NEVER A CANDIDATE."
---

> **NOT STARTABLE (2026-09-24).** Its section 5 STOP followed the kernel
> trace's Ret STOP. The source role is being redesigned under
> `RT-BRACKET-SOURCE-EDGE` (operator 2026-09-24). This child's reviewed
> checkpoint `4b4c8565c` stays a held input: never moved, rebased or landed.

## 1. What is settled. Do not re-derive.

- The representation is not in question. The Architect's second audit says
  outright that these findings **do not invalidate explicit Control IR**. What
  failed was the CUT, and the recut is the Steward's.
- **Retained and NOT to be rebuilt** (measured at `81f222b7f012829cd9f8d0f3dc684410a9b2b9ee`):
  `bracket_templates_by_owner` and `collect_term_constant_ids` are GONE;
  `bracket_release_only_suffix` is GONE; the exact-call transformation idea
  stands. Carry those forward.
- The semantic unit is a **bracket control region**. One region owns one
  acquisition-success lifetime from delayed-body entry through
  settlement-result merge.

## 2. REJECTED ON MEASUREMENT. Do not retry either.

1. **Ordinary prelude `proc` declarations as the marker.** The three
   `private_checked_resource_bracket_*` markers were ordinary prelude procs. A
   Ken program can NAME them, so the claimed compiler-private authority is
   FORGEABLE and AC-4 is not met. A declaration whose spelling says "private"
   is not a privacy mechanism.
2. **Port discovery by uniqueness scan.** Locating ports by scanning the
   expanded Runtime expression for an acquisition `HostOpV1` and every
   `ResourceRelease` breaks on a LAWFUL PUBLIC EARLY RELEASE inside the bracket
   body: the extra release makes the finalizer port ambiguous.

## 3. Deliverable

- **Exact occurrence capture.** Preserve the exact canonical template-call
  occurrence through checked lowering and wrap the result of THAT occurrence.
  If normalization would erase the call, record its exact checked occurrence
  path in the compiler-owned plan BEFORE normalization and consume that path
  exactly once during erasure.
- **A compiler-only marker identity, not resolvable from source.** Injected
  after source elaboration, or carried in a checked compiler side plane. Not a
  public declaration.
- **Producer-marked roles.** The producer marks acquire, acquired body, final
  settlement, outcome merge and resume ITSELF, before inlining. `HostOpV1` may
  validate those marked ports afterward; it may never discover them.
- **Reference evaluation unwraps `body` unchanged.**

## 4. Acceptance criteria

- **AC-1.** A Ken program that names or constructs the marker identity REFUSES.
  Direct forgery is the control, not an argument about visibility.
- **AC-2.** A bracket body containing a LAWFUL PUBLIC EARLY RELEASE still has an
  unambiguous finalizer port, and the early release stays ordinary body work.
- **AC-3.** Nested brackets each carry their own distinct role markers.
- **AC-4.** Bracket-SHAPED but unmarked code gets NO region. The control is ONE
  declaration chain carrying a canonical `withResource` bracket AND the proven
  lookalike declaration, asserting `regions.len() == 1`. RESEMBLANCE, not an
  unrelated acquire. An acquisition outside a canonical producer is
  UNCONSTRUCTIBLE FROM SOURCE -- prelude confinement removes a 41-name roster
  from `elab.globals`, `private_resource_acquire` among them -- so the earlier
  "unrelated acquire/release pair" control demanded a state nothing can reach.
  Do NOT hand-build the Runtime IR here: that bypasses erasure, and erasure's
  selection is the thing AC-4 measures.
  - **What ONE mutation establishes, written as that mutation and not as a
    class.** Widening the selector to key on application SHAPE rather than on
    the injected identity reds FAIL-CLOSED at marker normalization with
    `compiler bracket marker <id> has malformed application`. Measured at
    `daee72575`; located at `compiler_driver.rs:1275-1281`, inside
    `transform_bracket_producer_term` under `NormalizeMarkerPayloads`, firing
    only when `markers.ids().contains(&id)` holds for the application head and
    the spine is malformed. SUPERSEDES the count-of-2 discriminator in ruling
    `evt_7nrdfv7ekvcqt` point 5, which was a published PREDICTION.
  - **This does NOT generalize to over-wrapping.** The guard is keyed on MARKER
    IDENTITY, so a SOURCE LOOKALIKE -- different `GlobalId`, so
    `contains(&id)` is false -- passes normalization untouched and NOTHING
    refuses. `source_lookalike_identity_does_not_forge_bracket_metadata` shows
    exactly that: `regions.is_empty()`, no error. Do not restate refusal as a
    property of over-wrapping in general; that is the same move as the
    prediction it replaced.
  - **THE COUNT IS DISCHARGED ON HORN (b) at `0528c13e1`**, inside the accepted
    tip. Test file only, +35/-3, with `compiler_driver.rs` byte-identical and
    every mutation reverted -- so the guard was NOT relaxed to manufacture a
    reachable failure, which would have been the explicit non-discharge.
  - **The recorded reason is STRONGER than the one this frame predicted, and
    the prediction should not be read as established.** This frame offered
    "the guard is identity-keyed and normalization structurally precedes region
    construction" as the expected horn-(b) reason. Measured, three mutations
    are stopped by TWO barriers, not one. Widening to any 2-argument
    application, and the same restricted to well-typed applications, are
    refused at `NormalizeMarkerPayloads` on the arity guard. But marking a
    SATURATED 2-argument transparent application CLEARS normalization and then
    fails erasure lowering with `normalized HostIO body is neither
    identity-checked Ret nor Vis`. Normalization is therefore not the only
    thing in the way, and a second region still needs a second bracket-shaped
    producer occurrence that source cannot express.
  - **The count assertion is not inert, and that is measured rather than
    argued.** AC-8 runs the byte-identical sweep over a two-producer program
    and reads 2, so the instrument does distinguish 1 from 2. What is
    unconstructible is a FALSE 2 on the AC-4 fixture.
- **AC-5.** No port is selected by `HostOpV1`, name, expression resemblance, or
  a `ResourceRelease` search. Validation by `HostOpV1` is permitted and must be
  shown to be validation, not selection.
- **AC-6.** Reference evaluation unwraps `body` unchanged, shown by a control
  that would fail if it did not.
- **AC-7.** The marker is metadata: no runtime value, no public Ken syntax, no
  effect, no ABI field.
- **AC-8.** Owner disambiguation. Two distinct producers in one chain yield two
  distinct occurrences, each settling exactly once. This is what defeats the
  refuted section 2 selector. It is a SEPARATE claim from AC-4 and does not
  discharge it.

## 5. Stop condition

**STOP and the deliverable is the stop and its evidence if** the marker cannot
be made non-source-resolvable without adding a new public surface, or if exact
occurrence capture cannot survive normalization without a second authority.

Do not answer a stop with a census, a narrowed control set, a carve-out, or a
"private by convention" marker. On any stop branch it comes to the Steward and
then the operator.

## 6. Scope

Checked erasure and prelude production, the compiler-owned occurrence plan,
`ir.rs` for the wrapper, and the reference evaluator unwrap. **No new carrier,
return protocol, KRET lane, host operation, dispatcher reorder, trace sort or
fallback.**

## 7. Symptom inventory

One entry per advancing hard stop on this WP. The Architect owns the count;
the Steward seeds and amends the frame. Counters are per WP and per design
question and do not pool: the umbrella and the parked predecessor keep their
own. Triggers fire at three.

**Entry 1 -- AC-4 demanded an unreachable state.** The prescribed control was a
declaration carrying a canonical bracket plus an UNRELATED acquire/release
pair. Measured: prelude confinement is `elab.globals.remove` over a 41-name
roster, `private_resource_acquire` included, so an acquisition outside a
canonical producer cannot be written from source at all. The two producers
also cannot be siblings -- unnested sibling brackets fail at erasure with
`checked_oriented_marker_location`, which is the pre-existing `RT-AGG-COMPOSE`
and NOT this WP's to repair. The claim AC-4 makes is sound; only its
prescribed control was unbuildable, and it is replaced above by resemblance.
The owner-disambiguation fixture built while discovering this is retained as
AC-8 rather than discarded, because it measures a real and different property.

The replacement's DISCRIMINATOR was then mis-stated in turn, TWICE. First the
count: an over-wrapping selector does not yield 2, because widening the
selector to key on application shape is refused at marker normalization before
any region is built. Then the correction itself OVER-GENERALIZED, replacing a
predicted class with an asserted one -- refusal holds for that named mutation
and NOT for a source lookalike, whose different `GlobalId` passes normalization
untouched with nothing refused. Both are corrected in AC-4 above, which now
names the mutation and its message rather than a property of over-wrapping.
NON-ADVANCING by the Architect's ruling `evt_7m05yqa48pxhk` point 4: a defect
in frame TEXT surfaced by measurement, no mechanism rework, no new structural
wall met. The count stays at 1. The consequence AC-4 carried is now DISCHARGED
on horn (b) at `0528c13e1`, and nothing on this WP is owed to child 3.

The frame was wrong a THIRD time in the same direction and the ring caught it
again: the horn-(b) reason offered above was a single barrier, and there are
two. Every one of these was a general claim stated at a scope that had not been
run. The standing remedy is in AC-4's wording, which now names the mutation and
its message instead of a property.
