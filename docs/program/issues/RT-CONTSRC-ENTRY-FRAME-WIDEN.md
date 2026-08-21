---
id: RT-CONTSRC-ENTRY-FRAME-WIDEN
title: "capture supply for the recursive-position closures is inexpressible under the CURRENT capture plan (RT-CAPTURE-SUPPLY D0: all 16 word-only, the boundary is entry-ABI-origin recoverable vs producer-local-origin not) -- the named route is to EXTEND the entry-source enumeration so a live producer-local value becomes a real entry-ABI member, never RELAX verify_entry_frame's membership guard; D0 measures liveness + soundness feasibility before any implementation"
status: merged
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

# `D0` FEASIBILITY + SOUNDNESS MEASUREMENT at base `32cb6c93a`

**Result, in one line: the route is OPEN — all 25 producer-local captures are
widenable on both axes — and widening every one of them greens ZERO of the 16
witnesses.** Both halves are measured, and the second is the one that decides
what the dependants should do next.

## The population, and the six witnesses that have none

D0's population is the **25 `ProducerLocal` claims** carried by the generated
contexts of the 16 witnesses (RT-CAPTURE-SUPPLY's D0 measured 30 claims: 25
`ProducerLocal`, 5 `EntryAbi`). Those are the claims with a producer-local
coordinate, and therefore the only ones an entry-source extension could seat.

**Six of the sixteen witnesses have an EMPTY population** — `px7l` (both rows),
`px8ta :: public_one_level_bracket_finishes_and_releases`, `px8x`,
`rt_escape :: escape_resource_plus_plain_matches_interpreter`, and
`rt_parity :: buffer_allocate_malformed_capacity_narrows_to_invalid_bounds`.
These are exactly the six with **no generated context owning the closure body**.
They are neither widenable nor not-widenable: there is no frame for the widening
to extend, so the route does not reach them **even in principle**. Recorded as
its own disposition rather than folded into a refusal, because a refusal implies
a capture was examined and declined, and here there is nothing to examine.

## Axis 1 — liveness at the context's generation point: **25 of 25 LIVE**

The operative liveness question is whether the value exists, and is locatable,
at the point the redirect is emitted. **The planner answers it directly and in
its own words:** every one of the 25 `ProducerLocal` claims carries
`availability.direct_emission = Some(CurrentLexical { emission_owner,
producer_result_origin, emission_origin, lexical_environment_origin,
nearest_alias_index })`, with a **concrete** `nearest_alias_index` (15 at index
1, 10 at index 2). Zero carry `direct_emission: None`.

A `CurrentLexical` claim is the planner asserting it has located that value at a
named index in the emitting function's lexical environment **at the emission
origin**. A value the planner can index at the emission point is by construction
computed before it. ⇒ Every producer-local capture in the population is live
where the redirecting caller would have to supply it.

Note what this does *not* say: `resolve_context_capture_claim` **refuses** a
`CurrentLexical` claim, and rightly so — that consumer holds an ABI operand run,
not a semantic environment, so a nearest-alias index is not a frame slot. The
claim is evidence about **liveness**, not about admissibility. Reading it as
admissibility would be exactly the confusion the guard exists to prevent.

## Axis 2 — ABI/identity preservation: **preserving, for all 25**

| question | measured answer |
| --- | --- |
| Does seating a new member change the context's **identity**? | **No.** `ContinuationContextId` is a declared id; `continuation_contexts()` joins descriptors to planned contexts **by identity** (`by_id`), never by position within the capture list. Identity is independent of the capture set. |
| Does it change the context's **ABI shape**? | **Yes, by construction** — `header.captures` gains one and the slot run lengthens. The run is `[Parameter x producer parameters] ++ [Capture x projected inputs] ++ CONVENTION_SLOTS`. |
| Does that shape change land **under an existing consumer**? | **No.** Every consumer of the shape is regenerated from the same descriptor inside the same compilation: the context's own emitted body reads `slots`, and the redirecting call site appends `view.captures()` in declared order. |
| Is the coordination **enforced** or merely hoped for? | **Enforced.** `continuation_contexts()` refuses when `inputs.len() != planned.captures.len()`, and refuses two descriptors declaring one identity. An incoherent extension fails loudly at the planner rather than silently at the ABI. |
| Does any **durable/external** consumer pin the arity? | **No.** `ContinuationContextId` is `pub(in crate::cranelift_backend)`, has no `serde` or schema derive, and appears in no artifact or serialization module. A generated context's ABI is a per-compilation, compiler-internal contract. |

⇒ The extension is ABI-**shape**-changing and identity-**preserving**, and the
shape change is internally coordinated with no consumer outside the plan. That
is the "extend the enumerated membership" side of the discipline, not the
"relax the guard" side: `verify_entry_frame` would be asked to find a member
that genuinely *is* in the enumeration, which is the check passing on its own
terms rather than being loosened.

## Disposition

**All 25 producer-local captures: WIDENABLE** (live, and ABI/identity-preserving).
**Six witnesses: OUT OF REACH** — no owning context, so the route has no frame to
extend for them. **Zero not-widenable.**

The widenable subset is non-empty, so `D1` has a real target and the Architect's
open door is measured **open** rather than closed.

## THE RESULT THAT DECIDES WHAT THE DEPENDANTS DO

**Widening every one of the 25 claims greens zero of the 16 witnesses**, and this
is arithmetic rather than judgement. Per closure, the owning context's capture
plan holds **at most 2** claims against a source capture set of **3 to 5**.
Measured over all 25 recursive-position closures: **the claim count reaches the
capture count in 0 of 25; all 25 remain short even if every claim were widened.**

⇒ A witness needs **every** capture resolvable to be planner-recoverable. Closing
the producer-local axis completely still leaves 1 to 3 captures per closure with
**no planner claim of any kind** — the cardinality gap RT-CAPTURE-SUPPLY's D0
recorded as a residual. That gap is a *different* question from provenance: it
asks why the planner's capture projection is smaller than the closure's capture
set at all, and widening the entry-source enumeration does not answer it.

**This is the third consecutive necessary-but-not-sufficient result in this
chain** — RT-BRANCH's partition, then capture-supply provenance, now the
entry-frame widening. Each was real and none alone moves a witness. Whoever
scopes the next node should treat "does this green a witness?" as a separate
question from "is this route sound and open?", because on this chain the answer
has been no and yes three times running.

## THE ONE THING THAT COULD OVERTURN THE WIDENABLE VERDICT

`ProducerLocal`'s own doc (`continuations.rs:364-370`) says the coordinate
*"deliberately carries no ABI position and is not convertible to one. The value
does not exist at its owner's function entry, so `parameters + captures` has no
position for it, and inventing one is the first of the five exits the Architect
closed at `evt_75k8cydbj5127`."*

**That statement is about the OWNER'S OWN entry run, and my verdict is about the
GENERATED CONTEXT'S run. They are different frames**, and the distinction is the
whole basis of the verdict:

- Seating a producer-local in **its owner's** entry run is impossible, and I
  measured why independently: in **25 of 25** claims `binding_owner ==
  emission_owner`, so the value is computed inside the very function whose frame
  the consumer defines. A function's entry run is filled by its caller, before
  its body computes anything. Inventing a position there is the closed exit.
- Seating it in the **generated context's** run is a different act. That context
  is minted at the producer's continuation point, the redirecting caller holds
  the value there (axis 1), and the context has its own ABI arenas
  (`context_descriptors` / `context_slots` / `context_inputs`) distinct from the
  predeclared function partition.

**Whether `evt_75k8cydbj5127`'s closed exit was scoped to the owner's run alone
or to any entry run is the Architect's to say, not mine.** If it covers the
context's run too, this D0's widenable verdict is wrong and the population is
genuinely not-widenable on soundness. I have not assumed either reading: the
verdict above is stated together with the fact that would overturn it, because a
wrong widenable verdict is precisely the failure this node's tier note names —
it would seat a value that is not truly at an entry-ABI position.

## `AC-0` — the invariant audit

The measurement reads only planner-owned and retained-source state. It reuses the
RT-CAPTURE-SUPPLY D0 probe's captured output plus static reads of the planner
sources; **no capture value is read from the carried word**. That probe's added
lines were audited by grep for `Carried`, `carrier_field`, `emit_carrier` and
`word` and matched nothing, and the three touched files were restored
byte-identical by blob hash. **This D0 touched no production line at all** — its
new evidence is static analysis of the planner plus arithmetic over already-captured
measurements, and the tree carries only this document.

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
