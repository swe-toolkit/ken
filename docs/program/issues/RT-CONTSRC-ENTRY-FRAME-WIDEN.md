---
id: RT-CONTSRC-ENTRY-FRAME-WIDEN
title: "capture supply for the recursive-position closures is inexpressible under the CURRENT capture plan (RT-CAPTURE-SUPPLY D0: all 16 word-only, the boundary is entry-ABI-origin recoverable vs producer-local-origin not) -- the named route is to EXTEND the entry-source enumeration so a live producer-local value becomes a real entry-ABI member, never RELAX verify_entry_frame's membership guard; D0 measures liveness + soundness feasibility before any implementation"
status: ready
owner: runtime
size: L
gate: none
depends_on: [RT-CAPTURE-SUPPLY-DECLARED-INPUTS]
blocks: [NATIVE-HANDLE-CARRIER, PX8-F-CAP-41]
github: null
origin: "Architect open-door ruling on RT-CAPTURE-SUPPLY-DECLARED-INPUTS D0 (evt_6f4708amnwr4p). That D0 (e8fd40787) found capture supply inexpressible UNDER THE CURRENT CAPTURE PLAN, not under the invariant -- the closing deliverable (extending the entry-source enumeration so a producer-local value becomes a real entry-ABI member) was never attempted, so the node is non-total, not uncloseable. The Architect ruled the widening route genuinely open (not proven closed), named it, and ruled it a SUCCESSOR node beside the merged RT-CONTSRC-PRODUCER-LOCAL campaign -- NOT RT-CAPTURE-SUPPLY's D1. Steward-filed per COORDINATION section 2; framed but the feasibility/soundness fork is the D0's to measure, routed to the Architect."
---

# WHY THIS NODE EXISTS

[[RT-CAPTURE-SUPPLY-DECLARED-INPUTS]]'s D0 (e8fd40787, Architect-approved) measured
that **zero of the 16 recursive-position witnesses is planner-recoverable under
the current capture plan** -- all word-only. The measured boundary is sharp:

> A capture whose value originates as an **entry-ABI position** is
> planner-recoverable; a capture whose value originates as a **producer-local**
> (mid-body) binding is not. Of 30 planner claims: 25 are `ProducerLocal` with
> `context_capture: None`, which `resolve_context_capture_claim` (core.rs:7064)
> refuses outright; 5 are `EntryAbi`/`EntryFrame` claims that resolve.

`verify_entry_frame`'s own membership guard names the exact gap (Architect read
it independently): *"A ProducerLocal member cannot be invented here: the entry
source enumeration produces exactly the entry ABI run, so a mid-body value is
simply absent and this refuses."*

⇒ The non-closure is **under the current capture plan**, NOT under the invariant.
The closing deliverable -- making a producer-local value a real entry-ABI member
-- was never attempted, so this is **non-total, not uncloseable**. This node is
that closing deliverable, and it is **the real remaining gate** for
[[NATIVE-HANDLE-CARRIER]] and [[PX8-F-CAP-41]] for this population (which is why
the `blocks` edge for those two moves here from RT-CAPTURE-SUPPLY on its closure).

# THE NAMED ROUTE (Architect ruling evt_6f4708amnwr4p)

The `EntryAbi`/`ProducerLocal` correspondence shows the planner **already**
threads a value into a context's entry ABI at context entry, sourcing from the
producer side with **zero word read** -- for parameter-origin captures. The route
is to do the same for producer-local (mid-body) bindings: **EXTEND the
entry-source enumeration so a live producer-local value becomes a real entry-ABI
member.** This is sourced at context entry, not from the carried word, so it is
not obviously an invariant breach.

> ### THE SOUNDNESS DISCIPLINE. This is the whole risk, and it is one line.
>
> A widening must **EXTEND the enumerated entry-frame MEMBERSHIP** -- make the
> producer-local a real member of the entry-source enumeration -- and must
> **NEVER RELAX** `verify_entry_frame`'s membership check. The guard is
> fail-closed for a reason: relaxing it would admit a value that is not actually
> at an entry-ABI position, which reintroduces exactly the crossing the
> invariant bars. This is the same "widen the justified set, never blanket-open
> the guard" discipline the Architect ruled for RT-CAPTURE-SUPPLY's Sub-Q2, one
> layer down in the entry-frame enumeration.

# THE OPEN FORK -- the D0 measures it, the Architect rules it

The Architect explicitly did **not** pre-decide feasibility or soundness. The D0
must ground two questions per witness:

1. **Liveness at context-generation time.** The generated context is minted at
   the producer's continuation point, so a mid-body value computed **before**
   that point is a candidate; one computed after is not. For each of the 16
   witnesses' producer-local captures, is the value live at the context's
   generation point?
2. **ABI/identity preservation.** Does extending the entry-source enumeration to
   carry that value preserve the context's ABI and identity, and the recursor
   frame contract? A widening that changes the context's ABI shape or identity
   under an existing consumer is out of scope.

# DELIVERABLES

**`D0` -- the feasibility+soundness measurement, no implementation.** Per witness,
per producer-local capture: classify (a) live-at-context-entry vs not, and (b)
whether extending the enumeration to seat it preserves ABI/identity + the
recursor frame contract. The disposition splits: **widenable** (live and
ABI-preserving -- a D1 target) vs **not widenable** (dead at entry, or the
extension would break ABI/identity -- intended refusal, and if ALL witnesses land
there the whole population is genuinely inexpressible and the dependants need a
different route after all). Route the closed D0 to the Architect.

**`D1` -- the entry-source enumeration extension** (conditioned on D0's widenable
subset). Extend the enumeration so the widenable producer-local values become
real entry-ABI members; `resolve_context_capture_claim` then resolves them at
context entry. NEVER relax `verify_entry_frame`'s membership check. Deliverables
and ACs are the D0's to fix once the fork is measured.

# ACCEPTANCE CRITERIA

**`AC-0` (D0)** -- every producer-local capture across the 16 witnesses is
classified widenable vs not-widenable on both axes (liveness, ABI/identity), with
the not-widenable rows named by which axis fails. The measurement reads only
planner-owned + retained-source state; an audit shows zero capture-value reads
from the carried word (the inherited inviolable line).

**`AC-1` (D1, conditioned)** -- a widenable producer-local capture resolves to a
real entry-ABI member through the extended enumeration, greening a seam-property
fixture; the membership check is extended, not relaxed (a fixture shows a
non-member producer-local still refuses fail-closed).

**`AC-2`** -- the discriminating control: a producer-local value that is NOT live
at context entry, or whose seating would break ABI/identity, still refuses
fail-closed, unchanged.

**`AC-3`** -- conformance for the widenable accept case and the not-widenable
refusal case, if D0 finds a non-empty widenable subset.

**`AC-4`** -- no-regression in CI.

# BANNED SCOPE

- **Relaxing `verify_entry_frame`'s membership check.** The one inviolable line;
  the widening must extend membership, never open the guard.
- **Reading any capture value from the carried word** -- inherited from
  RT-CAPTURE-SUPPLY / RT-BRANCH; still barred.
- **Changing a context's ABI shape or identity under an existing consumer.** The
  extension must be ABI/identity-preserving; a breaking change is a different,
  larger question and out of scope here.
- **Implementing before D0.** The feasibility+soundness fork decides the shape.

# SEQUENCING

**Framed, not released until RT-CAPTURE-SUPPLY closes.** `depends_on:
[RT-CAPTURE-SUPPLY-DECLARED-INPUTS]` -- this node is that node's closing
deliverable; it opens once RT-CAPTURE-SUPPLY closes at D0 as a bounded
non-closure. Sits **beside** the merged [[RT-CONTSRC-PRODUCER-LOCAL]] campaign
(the producer-local-vs-entry-frame boundary is that campaign's subject), not
inside it. `gate: none` -- runtime lowering, no TCB or trusted-reduction change;
the feasibility/soundness question is a design fork the Architect rules on the
D0, not an operator gate. Tier **T1** (soundness-adjacent: a wrong widenable
verdict would seat a value that is not truly at an entry-ABI position). Review:
**Architect** (author is not the reviewer), who reviews the D0 and any D1.
