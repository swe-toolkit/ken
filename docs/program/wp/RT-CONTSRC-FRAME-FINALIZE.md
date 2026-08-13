# RT-CONTSRC-FRAME-FINALIZE — finish the two-stage continuation availability lifecycle

**Owner: runtime. Size: M. Gate: none.**

**Base: re-derive `origin/main` at cut time.** Fixed inputs measured at
`origin/main` = `37bc8464`, all in
`crates/ken-runtime/src/cranelift_backend/planning/static_transition.rs`.

## Fixed inputs

| fact | site |
|---|---|
| `ContinuationInputProjection` carries an **`availability`** field — *"where this value is IMMEDIATELY available to the emitter"*, distinguished from root provenance | `:950-992` |
| the two-stage types: `ContinuationAvailabilityDraft` over `ContinuationFrameRequirement`, `ContinuationAvailabilityViews` over `ContinuationFrameIdentity` — *"Both consumers read this form"* | `:1445-1451` |
| the publication gate that refuses an unfinalized claim | `:2377-2392` |
| why the gap persists — *"the context ids that would resolve it are minted **after** this record is interned"* | the projection's own comment |
| the blocked consumer population | `RT-LEXICAL-RECURSOR-CONSUMERS` `D2k-1b`, five governed rows |

## D1 — establish the resolution point

**Find where the context ids are minted, relative to where the projection is
interned.** Report both sites and the ordering. The whole node turns on this:
if the ids are available at some later point that still precedes both
consumers' reads, Stage 2 can run there and this is a sequencing fix. If they
are not, say so — that is a different and larger finding.

**Do not assume the interning order is fixed.** Report whether it is a
structural requirement or an incidental ordering.

## D2 — resolve the requirement to an identity for the five governed rows

Run Stage 2 for them: `ContinuationFrameRequirement` → `ContinuationFrameIdentity`,
so `continuation_input_view` publishes a finalized form both consumers read.

**Constraints, and they are the design rather than fences around it:**

- **Never publish the draft** — a half-stamped claim.
- **Never publish a default** — an invented one. If an id cannot be resolved,
  the gate must still refuse. **Making the gate pass is not the deliverable;
  making the claim true is.**
- **The root-vs-immediate separation stays structural.** It is enforced by the
  two-stage types and it is soundness-grounded (`evt_609am4v7cdt5b`). Do not
  collapse the stages to make resolution easier.

## D3 — show the five rows become joinable

With availability finalized, demonstrate that `D2k-1b`'s five governed rows can
name both operands. **You are not required to land `D2k-1b`'s repair here** —
that is its own slice and returns to its own frame. What is required is
evidence that the blocker is gone.

## Acceptance criteria

- **AC-1 — the publication gate passes for the five governed rows because their
  requirement RESOLVED**, and a control shows it still **refuses** when
  resolution is genuinely unavailable. A gate that stops refusing entirely has
  been broken, not satisfied.
- **AC-2 — no draft and no default is ever published.** Assert the published
  form's identity, not merely that publication succeeded.
- **AC-3 — the resolution point from D1 is named**, with the two sites and their
  ordering.
- **AC-4 — `D2k-1b`'s five rows can name both operands**, shown against the same
  rows, not a proxy.
- **AC-5 — no new relation, carrier, or cross-`Function` transport exists in the
  diff.** This is a lifecycle completion inside a landed design; if the repair
  starts requiring one, that is a hard stop, not a widening.

## Banned scope

- New continuation-source/planner relation, ABI carrier, cross-`Function`
  worker/template transport, a second recognition writer, or arming
  `finish_source_constructor`. **Each of those was enumerated and refused: they
  join the operands while the frame identity is unresolved, which is the
  invented default the gate exists to refuse. Relaxing one buys an unsound
  join.**
- Weakening, bypassing, or default-filling the publication gate.
- Row 3's retained singular-specialization wall.
- Classifier, checker, marker, enumeration, fusion-candidate, representation,
  ledger, closure-boundary repair. Gates 5-6 held; production unarmed.

## Hard stops — return to the Steward

- **The context ids cannot be minted before both consumers read.** That is the
  larger finding the Architect's ruling did not foreclose, and it changes the
  node.
- **Resolution would require any banned item above.**
- **Finalizing changes what a consumer emits.** The lifecycle should change when
  a claim is publishable, not what the plan contains. If the produced artifact
  differs, stop — same discipline as gate 4b's observation gate.

## Sequencing and contention

**Runtime, after gate 4b.** One implementer lane; 4b is in flight and
unaffected by this node. `D2k-1b` stays parked until this lands — it is blocked
on exactly this, so working it first would repeat the stop.

Touches `planning/static_transition.rs`. **Gate 4b touches
`lowering/core.rs`** — different files, but the same seat, so they run in
sequence rather than together.
