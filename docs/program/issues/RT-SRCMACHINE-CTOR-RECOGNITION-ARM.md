---
id: RT-SRCMACHINE-CTOR-RECOGNITION-ARM
title: "Arm static-worker recognition on the source-machine Construct arm, which never dispatches the classifier, after a bounded check that every eligible-field state can enter the template"
status: merged
owner: runtime
size: M
gate: none
depends_on: [RT-CTOR-TRANSPORT-RECOGNITION-ASYMMETRY]
blocks: [RT-SRCMACHINE-DISPATCH-REACHABILITY-CONTROL]
github: null
origin: "Architect mechanism disposition evt_rdks1pn7cpb (2026-08-14) on RT-CTOR-TRANSPORT-RECOGNITION-ASYMMETRY's D1. Recognition-gap branch confirmed and not close; he also refuted his own route-diversion hypothesis and refined his own one-need ruling against himself. Steward-filed (agents cannot create tracked work per COORDINATION §2), and he asked explicitly that this disposition, the advancing-refusal prediction, and the recognition-versus-disposition split live in the frame rather than in the thread."
---

> # AN ADVANCING REFUSAL IS THE SUCCESS SIGNAL. READ THIS BEFORE YOU MEASURE.
>
> **Arming recognition should move row 4 deep from its `value_at` refusal to
> its DISPOSITION-CORRECT OWNER-GUARD. For a transfer-bound field that is
> `boundary_transfer_admissibility` — NOT `close()`. That is what success
> looks like. It does not close row 4.**
>
> The Architect stated the *shape* of this in advance, and said why: otherwise
> the ring arms the classifier, sees a refusal, and reads a working increment
> as a failed one. Only once row 4 deep advances to its owner-guard is the
> rebinding question properly posed — and that is where a vehicle question
> would finally arise, on a properly stated need, **coming back to him. Not
> now.**
>
> **CORRECTED 2026-08-15 (`evt_692wak3h89gv6`). This paragraph named the wrong
> guard, and it is the first thing you read.** It said the target was *"a
> `close()` conservation refusal — the same one row 5 already has."* **A
> transfer-bound field's owner-guard is `boundary_transfer_admissibility`;
> row 5's close seat is unchanged and row 4 does not join it.** Reaching a
> `close()` refusal here would have satisfied the old wording while being the
> wrong seat. See `AC-4`, which carries the full correction and the separate
> handling depth 2 now requires.
>
> **Both dissolved-fork vehicles stay untaken.** `ContinuationTemplate` is not
> populated and no continuation-source projection surface is added.

## What the predecessor settled

**The recognition-gap branch is confirmed and it is not close.**

| fact | site |
|---|---|
| the classifier | `core.rs:15301-15318` |
| its **only** production dispatch | `lower_expr`'s direct `Construct` arm, `core.rs:17416` |
| the source-machine `Construct` arm | `core.rs:7494-7534` — **never calls it** |
| with the route applied, at constructor 26 field 0 | the classifier's own two conditions **both hold** |

**Production simply never asks.** The comment at `core.rs:17410-17415` names
the other producers as fail-closed *"for now"* — **implementation scope, not a
semantic exclusion.** That is the whole disposition: nothing decided that this
field should be excluded; nothing ever put the question to the classifier.

## The Architect corrected himself twice, and both corrections are load-bearing

**1. His route-diversion hypothesis is REFUTED.** *"Is the route what diverts
it?"* was his own framing of the successor question, and the measurement
separates the two cleanly: the route only flips the classifier's **input** from
`[false]` to `[true]` by supplying a real `StaticWorker` binding. It cannot
move an already-recognized field off the recognition path, **because there is
no recognition dispatch on that arm to move it off.**

⇒ **The route exposes the gap; it does not create it.** Do not carry any
remnant of "the route diverts it" into this node's reasoning.

**2. His "row 4 deep and row 5 are ONE need" ruling is SHARPENED, not
reversed.** Row 5's evidence shows owner 22 classifies `[true]` at the direct
arm **with or without the route** and still reaches its conservation refusal.
So the two are not the same defect:

| residual | defect |
|---|---|
| row 4 deep | **recognition gap** — never enters the ledger at all |
| row 5 | **disposition gap** — enters the ledger; nothing rebinds it |

They share a **law** — `close()`'s *erased before construction, or consumed at
an exact-`Var` call* — but not a **defect**. **They become one need after
recognition is armed, not before.** A report or repair that treats them as one
need now is working from the superseded version of that ruling.

## Deliverables

**D1 — the bounded population check, and it is not optional. Run it BEFORE
arming anything.**

The measured occurrence had `pending_remaining=0, lowered_fields=0` — field 0,
nothing lowered, the easiest possible state. **One occurrence does not
establish that every source-machine `Construct` state carrying an eligible
field arrives that way.**

Enumerate the source-machine `Construct` states that can carry an eligible
field, and report whether any arrive with **`lowered_fields > 0`** or a
**non-empty pending continuation**.

> **A repair that works for field 0 and breaks for field 1 is the failure mode
> here, and it is cheap to rule out first.** This is the deliverable that
> decides whether `D2` is a small arming change or a hard stop.

**D2 — arm recognition on the source-machine `Construct` arm**, so the
classifier is dispatched where its conditions already hold.

## Acceptance criteria

**AC-1. It may only ADD recognition — never remove or weaken a refusal.**
Today's behaviour **over-rejects; nothing unsound ships right now.** This is a
**completeness** repair and it must not convert into a soundness regression on
the way. State the direction of every behaviour change you make.

**AC-2. This is the real gate.**
`d2k_1b_i_every_recognized_static_worker_reaches_a_disposition` stays green,
**and a new control must red if a source-machine-recognized field is recognized
but not transitioned.**

> **SEAT THE CONTROL AT DEPTH 3 OR ROW 5 — NOT AT DEPTH 2.** Amended
> 2026-08-15 by the same Architect ruling (`evt_692wak3h89gv6`). Depth 2 is
> subject to the existing-trace discriminator in `AC-4` and can be legitimately
> masked by [[RT-CLOSURE-BOUNDARY-LANE]], so **a control seated there cannot
> distinguish "recognized but not transitioned" from "masked, as expected"** —
> it would be green for a reason unrelated to the property it is meant to
> guard.

> `close()`'s ledger is what makes *constructed-then-forgotten* impossible, so
> **a recognition that does not record its obligation at `core.rs:15368` is
> strictly worse than the refusal it replaces.** Recognition without
> disposition converts an over-rejection into a silent hole. The new control is
> what makes that failure visible, and a delivery without it has not met this
> node.

**AC-3.** Both banned surfaces stay untaken — no `ContinuationTemplate`
population, no continuation-source projection surface.

**AC-4 — AMENDED 2026-08-15 by Architect ruling, relayed at `evt_692wak3h89gv6`.
The original prediction named the wrong guard and is corrected below.**

Row 4 deep **advances** from its `value_at` refusal to its **disposition-correct
owner-guard**. For a **transfer-bound** field that is
`boundary_transfer_admissibility` — **never `close()`**. Row 5's close seat is
**unchanged**.

**Reporting "still refuses" without naming which law refused does not satisfy
this** — the whole prediction is about *which* refusal fires, and an advancing
refusal is the success signal, not a failure.

> **What this criterion originally said, retained because the correction is the
> useful part:** *"The expected result is a `close()` conservation refusal, not
> a green."* **That named the wrong guard.** A transfer-bound field's owner-guard
> is `boundary_transfer_admissibility`; routing its disposition through `close()`
> would be the wrong seat, not a stricter one. A candidate that reached a
> `close()` refusal here would have satisfied the old wording while being wrong.

**Depth 2 is recorded SEPARATELY and is subject to the existing-trace
discriminator.** Do not fold it into the row-4 result:

- **Recognition recorded AND the closure trace names an enclosing variant** ⇒
  **masked by [[RT-CLOSURE-BOUNDARY-LANE]]**. That is an explained outcome, not
  a gap.
- **No recognition** ⇒ **a bounded residual, and it returns to the Architect.**
  Do not repair it here and do not widen this node to cover it.

**AC-5.** No repair is retained beyond what the outcome requires. Blob identity
on any file this node does not intend to change.

## The reachability residual routes to a successor — RECORD it, do not build it

**Steward scope disposition 2026-08-15, answering `evt_48cjhd4pdeey4`.**

**A governed source-machine route fixture warrants a successor, and it is filed:
[[RT-SRCMACHINE-DISPATCH-REACHABILITY-CONTROL]] (`ready`, `S`).** It does **not**
belong in this node and it does **not** hold this candidate.

**The successor does NOT ask for the D2k-1c harness.** The Architect's refusal
stands and is inherited: no test-only field on a production type. The successor
asks the prior question — *is there an acceptable-cost observation of
source-machine dispatch at all?* — and **"no, and here is the limit" is a
complete outcome there.**

**This node's obligation is unchanged and is exactly the Architect's two
disclosures:** the recording half is unrepresentable, so the committed control's
discriminating value is its dispatch assertion and it cannot see the arm ceasing
to dispatch; and the depth-3 governed transition `value_at` →
`StaticWorkerBinding` is **MEASURED-BUT-UNPINNED**. **Write both where the
control lives.** The successor starts from that measured fact — losing it here
costs the successor its starting point.

**The depth-2 discriminator is still owed** and gates closing this node. It is
not affected by any of the above.

## Stop condition — return to the Architect, do not decide

**If `static_worker_constructor_template` cannot be entered from a
source-machine state without restructuring it** — pending continuation,
partially lowered fields — **that is a hard stop.**

**Do not restructure the template on the ring's authority.** That is the same
class of call the last two stops correctly refused, and refusing it is what
earned this lane two rulings on measurements rather than on guesses.

## Why the predecessor earned this disposition

Recorded because it is the standard for the report this node owes. The frame
asked for the tree named beside every observation, and the ring produced
**five separately labelled measurements** — including the two that distinguish
*route supplies a `StaticWorker` binding* from *route diverts an
already-recognized field*. **The Architect had flagged that only as a risk; the
ring turned it into a measurement**, which is what refuted his own lead.

And reporting that **row 5's recognition class did not change** is what
corrected the one-need ruling. A report covering only row 4 would have left it
standing, and this node would have been framed on it.
