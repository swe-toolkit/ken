---
id: RT-CHECKED-IH-TAIL-RESULT-PRODUCER-ROUTE
title: "Replace the Tail variant of the fresh-result route with one whose SOURCE endpoint is the actual result-producing operation after the checked computation returns. HS11 ruling evt_79trx05xee0dj established that the landed TailResumedRetInput certifies the INITIAL CARRIED SEED as though it were fresh R2: source.rs:4512-4515 returns the carried residual unchanged, source.rs:2148-2150 records that same word.word as a source result, and core.rs:12218-12233 then :12615-12621/:12636-12640 carry the identical scrutinee into the active header, the Ret case environment and the Ret input. The certified pairing is therefore exact SSA identity from seed to sink, not a value edge -- the observer renamed its input. This node starts with D0 ONLY: name the actual producer on the natural emitted path, prove its provenance is DISTINCT from the seed, and pair it forward. Preserve DirectInvocationReturn unchanged; the initial CheckedIhCapturedEnvironment word becomes an explicit NEGATIVE CONTROL, never the source. Replace, do not extend -- no sibling authority."
status: active
owner: runtime
size: M
gate: none
tier: T1
depends_on: [RT-CHECKED-IH-FRESH-RESULT-ROUTE]
blocks: [RT-RESULT-CONTINUATION-BINDING-PROVENANCE]
github: null
origin: "Architect hard-stop-11 ruling evt_79trx05xee0dj, 2026-08-28 (thr_725bmt2m6sv9s), bound to amended release 24c14f4dacbcdf6789952f7a9d3f75155b310e64 / tree 36abd930d5acd2a8ab84141f422bd7bc795a5074 / frame blob 492235777030f4f12083f0de883efe008e1aa0af / spec-42 blob 69b9d6d267ba20235f42972865c2b20504531d62. The ruling accepted the HS11 D0 measurements (four Tail rows, one Direct row; the static/emitted Tail source-active-header-Ret-input pairing still passes; the Direct-only authorized implementation compiles; both admitted products remain the exact ResourceBodyResult defaults) and the linked-artifact observation that the final unit frame carries 0x0e09 while 0x1109/0x1209 are separately live. It then ruled the sharper conclusion: the landed Tail route certifies the carried seed as though it were fresh R2, so TailResumedRetInput is WITHDRAWN as fresh-result value authority while its destination topology, active-header identity and Ret binder identity remain useful facts. Explicitly NOT a sibling authority: the existing fresh-result-route predecessor is recut by replacement. No Decision object is required -- the ruling follows deductively from the exact source path and the runtime value observation. Steward-owned recut and release sequencing per the ruling's closing instruction. Steward-filed per COORDINATION section 2."
---

> # THIS NODE EXISTS BECAUSE AN ENDPOINT PAIRING WAS REAL AND PAIRED THE WRONG
> # VALUE. Read that sentence before designing anything.
>
> The landed `TailResumedRetInput` is not broken in the way a missing relation is
> broken. It certifies a genuine, exact, forward pairing — and both of its
> endpoints hold the SAME value. `CoEmissionOnly` already proved the pairing is
> not mere co-emission; the HS9 causal control proved exact SSA identity from
> seed to sink. **Exact identity does not turn a seed into a result.**
>
> **⇒ A stronger pairing proof cannot fix this, and building one is the trap.**
> The defect is the choice of SOURCE, not the strength of the edge.

## What is already measured, and is not yours to re-derive

Accepted by the Architect at `evt_79trx05xee0dj`; treat as fixed input and cite
rather than re-measure:

- Four real Tail rows and one body-refined Direct row.
- The static and emitted Tail source -> active-header -> Ret-input pairing still
  PASSES. It is passing on identical values.
- The Direct-only authorized implementation compiles.
- Both admitted products remain the exact `ResourceBodyResult` defaults.
- The final unit frame carries `0x0e09`; `0x1109` and `0x1209` are separately
  live.
- The disposable evidence diff was restored; its SHA-256 is
  `24ba840cccc4de3ca3ab3a5223ee31091d92b3c4b773306a090be1b5b443a879`.

**The exact producer read, from the ruling:**

1. `source.rs:4512-4515` — Tail application installs the invocation and returns
   the initial carried residual UNCHANGED:
   `RoutedAnswer::direct(LoweringOperand::Carried(word))`. **The seed.**
2. `source.rs:2148-2150` — the supposed Tail source-result is recorded from that
   SAME `word.word`, immediately before `lower_carried_computational_match`.
   **Nothing there produced a fresh result; the observer renamed its input.**
3. `core.rs:12218-12233` — active self-resumption records and jumps that same
   `scrutinee.word` into the header.
4. `core.rs:12615-12621` — the checked-answer fallback binds the SAME scrutinee
   into the Ret case environment; `:12636-12640` records it as the Ret input.

The route type states the boundary itself at `aggregates.rs:373-375`: it carries
no result value, no emission-local value number, and no runtime carrier. The
LLDB `0x0e09` observation is the dynamic confirmation of that static read.

## D0 — and this node is D0 ONLY until D0 answers

**Do not write a route constructor, a validator, or a control before D0 lands an
answer.** The ruling starts this node at D0 deliberately: every previous stop in
this chain began by naming a mechanism.

On BOTH admitted programs, from the NATURAL emitted path:

1. **Identify the actual producer** of the value returned on
   `CheckedSelectedRecursor`.
2. **Prove that value is DISTINCT IN PROVENANCE from the initial seed.** Not
   distinct in shape, not distinct in word, not merely live at the same time —
   distinct in provenance.
3. **Pair that exact producer value forward** through the active header, the Ret
   input binder, the ordinary capture occurrence, and the body read.

## Acceptance criteria

- **`AC-PRODUCER-NOT-RENAME`.** The named source must be an operation that
  PRODUCES a value. A destination-side observation, a matching tag, a live
  neighbouring SSA word, and a consumer-side rename are each INSUFFICIENT and
  none of them may be presented as the producer. This AC is the whole node; an
  answer that satisfies everything else and fails this one is HS12, not a
  landing.
- **`AC-SEED-IS-A-NEGATIVE-CONTROL`.** The initial `CheckedIhCapturedEnvironment`
  word must appear in the deliverable as an EXPLICIT NEGATIVE CONTROL. It may
  never be the source. A tree in which the seed could still be selected as the
  source has not met this criterion, whatever it asserts.
- **`AC-SEED-SUBSTITUTION-REDDENS`.** A mutation that substitutes the SEED while
  leaving the actual result producer INTACT must REDDEN the producer-to-sink
  claim. This is the discriminating control and it is stated as a required
  observation, not as a roster of mutation names. **A control that cannot fail is
  not weaker evidence; it is none** — and this chain has now twice certified a
  pairing that could not distinguish the values it paired.
- **`AC-DIRECT-PRESERVED`.** `DirectInvocationReturn` is preserved unchanged for
  its one-row population, and its blob is proved identical across the increment.
- **`AC-REPLACE-NOT-EXTEND`.** The corrected Tail variant REPLACES the withdrawn
  one. The old source-value claim is not retained in parallel, and no sibling
  authority is added. Two authorities over one value is the shape this chain
  keeps producing.
- **`AC-AFFECTED-CLOSURE`.** Cover every target that loads any module whose
  CLOSURE this increment changes, diff-touched or not. This is not a relaxation
  of the targeted-build rule: what changes is which targets count as affected,
  never how many crates build at once. An untouched consumer breaking on a
  closure change has now cost three lanes a red merge.

## Banned scope — these are the HS12 tripwires, not preferences

A producer edge may NOT be manufactured by any of: a persistent receipt, a
runtime lane, a direct capture write, a second lookup, a result search, a
clone or stack, an ABI carrier, or target synthesis.

- **Do not consume `0x1109` or `0x1209` by observation.** Shape and liveness do
  not establish which word is `R2`. Selecting either IS the forbidden result
  search, and it will look like progress.
- **Do not promote the compiler-only `answer_route` preservation experiment.** It
  propagates control metadata through `resume_active_continuation` and reaches
  frame 301, but carries NO value provenance and leaves the same seed in the Ret
  input and the final capture. Promoting it creates an unframed receipt while
  preserving the defect.
- **Do not add a fallback**, and do not fall back to the merge.

## Hard stop rule

**If D0 cannot name an existing compiler-visible producer/value edge without one
of the banned mechanisms, that is HARD STOP 12. Stop cleanly and route to the
Steward and the Architect.** Do not select the least-bad mechanism, and do not
weaken `AC-PRODUCER-NOT-RENAME` to make an answer reachable.

**HS12 mechanically triggers the mandatory Research advisory**: the Architect
will hold and call Research with the exact new fork before ruling. That is the
ARCHITECT's procedure to run, not this ring's, and **no Research call is due
before HS12** — the HS11 ruling says so explicitly. Do not manufacture one, and
do not read this clause as applying to any stop other than 12.

## Reviewers

Architect (required — this is the M-series) and runtime-qa, both on the exact
candidate SHA.

## Capability tier

**T1.** The size is M and the deliverable is small, but the entire question is a
provenance judgment that eleven stops have now gotten wrong in the same
direction. This is the axis `§4h` calls reasoning-dense regardless of diff size.

## Sequencing

`RT-RESULT-CONTINUATION-BINDING-PROVENANCE` (atomic D3A+D3B) is FROZEN and stays
frozen until this node lands through fresh exact-object gates **and the Steward
issues another explicit release**. The release `evt_98vzwa6e9qv1` is SPENT.
**Landing alone releases nothing; the HS11 ruling alone releases nothing.**

## The predicate this node must not repeat

Eleven stops share ONE predicate: **a static or local endpoint treated as a
complete directed dynamic value edge.** Each stop added another endpoint —
availability, access, a producer, a destination, a route — and none of them added
a VALUE. The inventory in
[[RT-RESULT-CONTINUATION-BINDING-PROVENANCE]] entries 1-11 is the full list.
**If the answer to D0 is another endpoint, it is the twelfth stop, not a
deliverable.**
