---
id: RT-SRCMACHINE-DISPATCH-REACHABILITY-CONTROL
title: "Nothing observes that the source-machine Construct arm still dispatches to the classifier -- the one regression that silently restores the predecessor's defect is the one its control cannot see"
status: ready
owner: runtime
size: S
gate: none
depends_on: [RT-SRCMACHINE-CTOR-RECOGNITION-ARM]
blocks: []
github: null
origin: "Architect ruling evt_9qn4nm8gg0ye section 4 on RT-SRCMACHINE-CTOR-RECOGNITION-ARM's AC-2 seat: the unit control is accepted and the D2k-1c route harness is refused, leaving a reachability residual he explicitly declined to authorize inside that node and routed to the Steward as a scope question (runtime-leader evt_48cjhd4pdeey4). Steward-filed per COORDINATION §2."
---

> # READ THE SCOPE BEFORE THE PROBLEM. THIS IS NOT A LICENCE TO RESTORE THE HARNESS.
>
> **The Architect refused the D2k-1c route apparatus, and the refusal is
> grounded, not provisional.** It would put a **test-only
> required-consuming-occurrence field on production planned units** and reinstate
> a six-file route helper — production surface bent to serve a test.
>
> **A frame that said "build the governed route fixture" would presuppose the
> answer he just refused.** So this node does not ask for it. It asks the prior
> question: **is there an acceptable-cost observation of source-machine dispatch
> at all?**
>
> **"No, and here is the limit" is a COMPLETE outcome of this node** — recorded
> permanently, not a failure to deliver.

## The gap, exactly

The predecessor armed recognition on the source-machine `Construct` arm, and its
committed unit control **drives the dispatcher directly.**

| regression | governed route | the committed control |
|---|---|---|
| delete the source-machine `Construct` arm's dispatch call | falls back to `value_at` | **stays green** |

⇒ **The one regression that silently restores the defect the predecessor exists
to repair is the one regression its control cannot see.** The predecessor's own
`D1` finding was that the arm *"never calls it — production simply never
asks."* Nothing today would notice production going back to not asking.

**This is a reachability gap, not a recording gap.** Do not carry any remnant of
the recording question into this node: *"a `StaticWorker` field exists whose
obligation was never recorded"* is **unrepresentable**, enforced by the required
`recognition` field plus a private constructor whose sole mint site is
`RecognitionIdIssuer::mint()` (`mod.rs:4316`, `:4345`). That was measured and
settled. **The residual is reachability alone.**

## What is already measured — start here, do not re-derive

**The governed depth-3 transition was OBSERVED.** Under the since-reverted
probe, row 4 deep advanced `value_at` → `StaticWorkerBinding`. It is recorded in
the predecessor as **MEASURED-BUT-UNPINNED**.

⇒ **The fact is not in doubt. Only its pinning is.** This node is about whether
that observation can be made durable at acceptable cost — **not** about whether
it is true.

## Deliverables

**`D1` — enumerate the candidate observations, and cost each. This is the
deliverable; `D2` is conditional on it.**

For each candidate, state what it would observe, what it would cost in
**production surface**, and whether it discriminates the regression above.
At minimum consider:

- **An already-existing governed compile** whose output would change if the arm
  stopped dispatching. **Check this first** — the cheapest acceptable answer is
  one that needs no new machinery, and nobody has looked.
- A test-only observation that reads existing state rather than adding a field
  (the predecessor's `d5a_trace` shape is the local precedent).
- A structural argument that makes the dispatch **unremovable** rather than
  observed — a call site the type system requires. **Prefer this if it exists:**
  `COORDINATION §7`'s exhaustive-by-construction beats any control.
- The barred D2k-1c apparatus, **costed and listed for completeness only.** It
  is not selectable here; naming it keeps the comparison honest.

**`D2` — implement the cheapest candidate that passes `AC-1`, if one exists.**
With a mutation proof: delete the dispatch call, show the control reds.

**`D3` — if none passes, record the limit where the control lives.** In the
shape the repo already uses (`control.rs:4388-4430`): name what the control
does discriminate, name the regression it cannot see, and say why no observation
was affordable. **A future reader must not have to rediscover this.**

## Acceptance criteria

**`AC-1` — the bar, and it is the whole node.** No mechanism may add a
**test-only field, variant, or parameter to a production type.** A candidate
that does fails regardless of how well it discriminates. This is the Architect's
standing refusal, restated as a criterion.

**`AC-2`.** If `D2` lands, its control **reds when the source-machine
`Construct` arm's dispatch call is deleted** — demonstrated, not argued. A
control asserted to discriminate without the mutation run does not satisfy this.

**`AC-3`.** The predecessor's committed unit seat is **unchanged**. This node
adds an observation; it does not rework what was accepted.

**`AC-4`.** Banned surfaces stay untaken: no `ContinuationTemplate` population,
no continuation-source projection surface, no template restructure, no guard
weakening.

**`AC-5`.** `D3`'s record lands **even if `D2` succeeds** — if a candidate was
rejected on cost, the next reader needs to know it was considered and why it
lost. **Blob identity on any file this node does not intend to change.**

**`AC-6`.** No-regression, in CI (`COORDINATION §12`).

## Stop condition — return to the Architect

**If the only workable observation requires a production-surface change**, that
is a hard stop and his call, not the ring's. Report the candidate, its cost, and
what it buys. **Do not take it on the ring's authority** — that is exactly the
call the predecessor's implementer correctly refused to make silently, and
refusing it is what produced this node.

## Why this earns a slot

**A completeness repair whose only control cannot see it being reverted has no
regression barrier on its own subject matter.** The predecessor closed a hole
that existed because production never asked a question; nothing now notices
production going back to not asking.

**It is small, and its cheapest outcome may be free** — `D1`'s first candidate
is whether a governed compile already discriminates, which nobody has checked.
**Its most expensive honest outcome is a paragraph.** Neither is a reason to
carry the gap unrecorded.
