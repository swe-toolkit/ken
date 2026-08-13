---
id: RT-CONTSRC-FRAME-FINALIZE
title: "The continuation availability lifecycle stops one stage short -- Stage 1 interns a `ContinuationFrameRequirement` before the context ids that would resolve it are minted, so Stage 2's `ContinuationFrameIdentity` never runs for the five governed rows and the publication gate correctly refuses to publish an unfinalized claim; the storage-independent view both consumers read is designed, typed and landed, it is simply never finalized"
status: ready
owner: runtime
size: M
gate: none
depends_on: []
blocks: [RT-LEXICAL-RECURSOR-CONSUMERS]
github: null
origin: Architect evt_713gsajzg3hmf, ruling on the RT-LEXICAL-RECURSOR-CONSUMERS D2k-1b hard stop routed by the Steward at evt_3547g5q20t0bc. Named by the Architect as the lawful successor and deliberately NOT authorized in passing; scope, ownership and sequencing taken by the Steward 2026-08-13.
---

## What this is

`D2k-1b` stopped because its five governed rows split the required operands
across two Cranelift `Function`s. **That split is a consequence, not the cause.**

The cause is a **lifecycle gap inside a landed design**:

- **Stage 1** carries `ContinuationAvailabilityDraft` =
  `ContinuationAvailabilityOver<ContinuationFrameRequirement>`.
- **Stage 2** is the published, immutable form both consumers read:
  `ContinuationAvailabilityViews` =
  `ContinuationAvailabilityOver<ContinuationFrameIdentity>`
  (`crates/ken-runtime/src/cranelift_backend/planning/static_transition.rs:1445-1451`).

The projection's own comment states why the gap persists: *"a generated frame is
still a structural requirement, because the context ids that would resolve it
are minted **after** this record is interned."*

⇒ **This node resolves the generated frame requirement to an exact context
identity for the five governed rows, so Stage 2 runs and
`continuation_input_view` publishes a finalized form.**

## Two claims that were wrong, and correcting them is what sized this node

**Runtime reported that `ContinuationInputProjection` "supplies
provenance/owners, not a value or pending consumer expression."** The Architect
measured otherwise: at `static_transition.rs:950-992` it also carries an
**`availability`** field whose own doc calls it *"where this value is
IMMEDIATELY available to the emitter"*, explicitly distinguished from root
provenance — *"the function that actually emits the call is the key's
`emission_owner`... This field is the position in the emitting context's own
environment where the value sits."*

⇒ **A per-emitter availability view is not missing. Neither is the shared
both-consumer form. What the five rows lack is a FINALIZED one.**

## Why no prohibition may be relaxed to get there

Runtime enumerated five join routes — a second recognition writer, arming
`finish_source_constructor`, cross-`Function` worker/template transport, a new
continuation-source/planner relation, an ABI carrier — and discarded them on
frame grounds. **The disposition was right for a better reason than the one
given: the binding constraint is soundness, not scope.**

Each route puts both operands in one place **while the frame identity is still
unresolved**, which is exactly the invented default the publication gate exists
to refuse (`:2377-2392`):

> *"a continuation input has no finalized availability, so its generated frame
> requirement was never resolved to an exact context identity; RT-CONTSRC-
> PRODUCER-LOCAL D3b refuses to publish an unfinalized claim"*

— never the draft, *"which would be a half-stamped claim,"* and never a default,
*"which would be an invented one."*

⇒ **Relaxing any of the five buys an UNSOUND join, not a lawful one.** The
root-vs-immediate separation (`evt_609am4v7cdt5b`) is enforced structurally by
the two-stage types; that is the design working, not an obstacle in it.

## Stated at its own strength

**The operands cannot be joined while their availability is a draft. Nothing
measured says they cannot be joined once it is finalized.** That distinction is
the whole difference between "this needs a new carrier" and "this needs an
existing two-stage lifecycle finished" — and it is why this node is materially
smaller and safer than the new relation or carrier the stop implied.

## The pattern this is the second instance of

Both today, in this arc:

- On gate 4b, a handback described where a seam would go and was **silent about
  the observer three lines below it**.
- Here, a handback reports that no existing view carries the operand, when the
  view **exists but is unfinalized**.

Each report is accurate about its own subject and each reads as a **negative
claim about the mechanism**. ⇒ **"No existing view does X", from the side that
cannot reach X, is a statement about REACHABILITY, not about EXISTENCE.** Both
times the real fix was smaller than the report implied. Recorded so Runtime
knows it is a pattern rather than two coincidences.

## Not this node

- **Any new continuation-source/planner relation, ABI carrier, or cross-
  `Function` transport.** The whole point is that none is needed.
- **Weakening or bypassing the publication gate**, which is the thing keeping
  the claim honest.
- **Row 3's retained singular-specialization wall** — a separate increment.
- Classifier, checker, marker, enumeration, fusion-candidate, representation,
  ledger or closure-boundary repair. Gates 5 and 6 stay held; production stays
  unarmed.
