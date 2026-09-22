---
id: RT-BRACKET-PRODUCER-AUTHENTICITY
title: "CHILD 1 of 3 of RT-BRACKET-CONTROL-REGION-IR. The checked bracket producer and its AUTHENTICITY boundary. Capture the exact canonical template-call occurrence through checked lowering and wrap THAT occurrence; the compiler marker must be NON-RESOLVABLE FROM SOURCE; and the PRODUCER marks the acquire, acquired-body, final-settlement, outcome-merge and resume roles ITSELF, before inlining. Two shapes are REJECTED ON MEASUREMENT and must not be retried: ordinary prelude `proc` declarations as markers, because a Ken program can name `private_checked_resource_bracket_*` and the claimed compiler-private authority is therefore FORGEABLE (AC-4 unmet); and locating ports by scanning the expanded Runtime expression for an acquisition `HostOpV1` plus every `ResourceRelease`, because a LAWFUL PUBLIC EARLY RELEASE inside the bracket body adds a release and makes the finalizer port AMBIGUOUS. `HostOpV1` may VALIDATE already-marked ports; it may never DISCOVER which expression becomes a region. THIS CHILD DOES NOT LAND ALONE -- it is a held, reviewed input assembled by child 3."
status: ready
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: [RT-BRACKET-SETTLEMENT-PLANE]
github: null
origin: "Steward recut 2026-09-22 on the Architect's second WIP audit verdict evt_3rvns2yxm898r, which ruled RT-BRACKET-CONTROL-REGION-IR MIS-SIZED and directed a retained-work recut into three ordered children. The Steward owns the recut; the mis-sizing was the Steward's, not the ring's. The producer-authenticity boundary alone needs a finished design, controls and review before the planner or lowerer can safely depend on it. Retained work is preserved at evidence checkpoint 81f222b7f012829cd9f8d0f3dc684410a9b2b9ee on wp/RT-BRACKET-CONTROL-REGION-IR, parented on pre-registration commit c22f4861d which stays unchanged. That checkpoint is EVIDENCE AND REUSABLE WORK, NEVER A CANDIDATE."
---

> # AUTHORIZED AND STARTABLE. Build it.
> #
> # **This child does NOT land on its own.** It produces a reviewed checkpoint
> # that child 3 assembles. No QA handoff as a candidate, no publication, no
> # partial landing.
> #
> # **Two shapes are already refuted. Do not re-derive them and do not retry
> # them.** They are named in section 2.

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
- **AC-4.** Bracket-SHAPED but unmarked code gets NO region. The control is a
  declaration containing a canonical bracket AND an unrelated acquire/release
  pair -- the mixed-owner case the previous lookalike control would have missed.
- **AC-5.** No port is selected by `HostOpV1`, name, expression resemblance, or
  a `ResourceRelease` search. Validation by `HostOpV1` is permitted and must be
  shown to be validation, not selection.
- **AC-6.** Reference evaluation unwraps `body` unchanged, shown by a control
  that would fail if it did not.
- **AC-7.** The marker is metadata: no runtime value, no public Ken syntax, no
  effect, no ABI field.

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
