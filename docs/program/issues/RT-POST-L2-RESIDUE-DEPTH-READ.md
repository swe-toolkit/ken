---
id: RT-POST-L2-RESIDUE-DEPTH-READ
title: "How much refusal residue stands between the four RT-CONTEXT-FRAME-LABEL-CORRECTION rows and a passing lowering, measured as a STATIC PATH-SCOPED READ under ONE NAMED hypothesized repair -- not by forcing, which has no terminating observation, and not by enumerating refusals, which over-collects by two orders of magnitude. Answers a question that is NOT a property of the rows: `frame` has no value until a repair supplies one, so reachability behind L2 is a property of (rows + repair) and the repair must be named or the count has no subject."
status: ready
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-18. Commissioned after RT-CONTEXT-FRAME-LABEL-CORRECTION's D0/D1 established that no repair at core.rs:1230 makes any of the four rows pass -- all four stop immediately behind it at core.rs:9558, whose owner RT-CONTSRC-PRODUCER-LOCAL is `merged` and cannot close them. Instrument class and terminating observation handed over by the Architect (evt_1g71815y1kyya); the operand pruning and its one borrowed premise checked and confirmed by the Steward (evt_9h58fp9q3z7c); the trace-versus-static-read correction and the missing-parameter objection are the Architect's (evt_3jrjm7hxg9rp1). Scope is the Steward's."
---

> # THE TWO INSTRUMENTS THAT LOOK RIGHT AND ARE NOT. READ THIS BEFORE SIZING.
>
> Both were proposed in this node's own history, by people who had just argued
> against the failure each one instances. **Neither is available to this WP.**
>
> **1. FORCING PAST EACH REFUSAL IN TURN.** A forcing run is *existential*: one
> run, one witness, one next stop. No number of them composes into the universal
> this node needs. **It has no terminating observation** -- every run yields a
> layer, a layer is always available, so it ends by budget and the budget is the
> only thing the number describes.
>
> **2. ENUMERATING THE REFUSALS.** `grep -c 'unsupported(' core.rs` returns
> **342**. That is a *presence oracle* and says nothing about reachability on
> this path. A flat enumeration over-collects by two orders of magnitude.
>
> ⇒ **The instrument is a STATIC PATH-SCOPED READ.** Refusals are static code at
> named sites, so the inventory question is call-graph reachability and the tool
> for it is reading. **Do not write the word "trace" in any deliverable of this
> WP** -- see the next block for why that word specifically is a trap.

# "TRACE" IS UNBUILDABLE HERE, BY CONSTRUCTION

The Steward's first scoping proposal was *"scope by the four rows' actual
lowering trace."* The Architect refuted it and the refutation is worth keeping,
because the phrase will occur to whoever takes this node too:

    The rows STOP AT L2 TODAY. A real lowering trace of any of the four
    therefore contains ZERO of the ten sites behind it. It cannot -- execution
    ends at core.rs:9558. To obtain a trace that reaches verify_entry_frame at
    all you must force past :9558, which is instrument 1 above.

**"Actual lowering trace" is ambiguous between a dynamic reading that cannot be
built and a static reading that is exactly right.** The static reading: *read
the code path under the constraint of the operands the rows are known to carry.*
That prunes hard and needs no run. **Say STATIC PATH-SCOPED READ.**

# THE PARAMETER WITHOUT WHICH THIS QUESTION HAS NO TRUTH VALUE

`verify_entry_frame` matches on the pair `(frame, defining_owner)`. Which arm a
row takes is decided by `frame`. **`frame` comes from the claim, and the claim
is precisely what is absent** -- `views.context_capture == None` *is* the L2
refusal at `core.rs:9558`.

⇒ **`frame` has no value today. It acquires one only from a repair.**

> **SO REACHABILITY BEHIND L2 IS NOT A PROPERTY OF THE FOUR ROWS. It is a
> property of (rows + a hypothesized repair), and this WP names the repair it
> measures against.** Left unnamed, *"the sites the rows never reach"* reads as
> a fact about the rows when it is a fact about an unstated assumption -- **a
> premise carried as a finding**, which is the exact shape that `D1` refuted in
> `RT-CONTEXT-FRAME-LABEL-CORRECTION` hours before this node was written.

## THE NAMED REPAIR IS NOT YET NAMEABLE. THAT IS THIS WP'S FIRST DELIVERABLE.

**A first draft of this node named a specific repair `H` and was wrong to.**
`H` was selected by eliminating one branch from a surviving PAIR, and the pair
came from a pruning whose input has never been measured. See the next section.
**Selecting `H` is `D2`, after `D0` measures the operand that decides it.**

Whatever `H` turns out to be:

    Every count this WP reports is attributable to H and to nothing else.
    A different repair is a different residue and a different depth answer.

### WHAT PLAUSIBLY SITS AT THE END OF A PREDECLARED-FRAME REPAIR

`continuations.rs:4279`'s own comment says a mid-body value is simply **absent
from the entry ABI run**. That is `RT-CONTSRC-PRODUCER-LOCAL`'s actual subject
-- *a mid-body value is a third availability class with no ABI seat* -- and that
node is `merged`.

**The pass may well report that the residue terminates in the thing whose owner
cannot close these rows.** That is a FINDING, not a failure of this WP, and it
is written here in advance so it is not met as a disappointment and quietly
re-framed into something that sounds like progress.

# THE PRUNING IS REAL AND ITS INPUT IS NOT MEASURED. `D0` MEASURES IT.

`ContinuationFrameIdentity` has two variants in play and
`ContinuationEmissionOwner` has three (`continuations.rs:92-106`). The match is
**fully enumerated with no catch-all**, so `defining_owner` alone prunes six
arms to two:

    defining_owner              surviving arms
    Predeclared(_)              9604 (Predeclared,Pre) + 9759 (GenCtx,Pre)
    Specialization(_)           9629 (GenCtx,Spec)     + 9747 (Pre,Spec)
    Fusion(_)                   9785 (Pre,Fus)         + 9797 (GenCtx,Fus)

**That is the whole pruning and it needs no run. What it needs is
`defining_owner`, and nobody has measured it.**

> ## THE ERROR THAT PUT A WRONG PRUNING IN THIS NODE'S FIRST DRAFT
>
> `runtime-implementer` published, per row, `owner PredeclaredFunctionId(5)` /
> `binding 395` / `environment 391`. The Architect read `owner` as
> `defining_owner`, flagged the reading as **borrowed rather than measured**, and
> asked to be corrected. The Steward checked it and reported it **confirmed.**
>
> **The check was the wrong instrument.** It established that
> `ContinuationEmissionOwner::Predeclared` carries a `PredeclaredFunctionId` --
> a **type** fact -- and concluded the operand therefore **is** a
> `defining_owner`. It also asserted that type belongs to no other field. **That
> is false: `PredeclaredFunctionId` is the field type of at least ten distinct
> roles in `continuations.rs` alone** -- `raw_owner` (`:224`), `binding_owner`
> (`:388`), `source_owner` (`:436`), `producer_owner` (`:511`, `:1070`),
> `consumer_owner` (`:512`, `:1073`), `emission_owner` (`:647`), `owner`
> (`:1136`).
>
> **A type-compatibility check cannot distinguish two fields that share a type.**
> It confirmed the operand *could* be a `defining_owner` and was read as
> confirming that it *is* one.
>
> **It is not.** The published triple is
> `ProducerLocalBinding::binding_owner` / `binding_origin` /
> `ProducerLocalLocator::environment_origin` (`continuations.rs:386-410`), all
> three fields of `ContinuationSourceCoordinate::ProducerLocal` (`:450-453`).
> And the refusal proves it: `core.rs:9557-9567` interpolates **`{coordinate:?}`
> and nothing else.** `defining_owner` is a separate parameter of
> `resolve_context_capture_claim` (`core.rs:9555`) and is not printed there.
>
> ⇒ `binding_owner` names who owns **the binding being looked up**.
> `defining_owner` names who owns **the frame being defined**. **Two different
> facts sharing a newtype.**
>
> **A hedge got a check, the check was the wrong instrument, and the result then
> read as measured -- which is worse than leaving the hedge flagged.**

## `D0` COSTS ONE OPERAND AND ZERO FORCING

**The four rows already stop at `core.rs:9558` today, unforced.** So printing
`defining_owner` beside `{coordinate:?}` at that site is **not a forcing step
and not a rung** -- it is one more operand at a refusal the rows reach on the
run that is already being made.

**Do not infer `defining_owner` from the refusal's prose.** *"A generated
context capture..."* is a fixed string in the format literal, identical on every
execution. Inferring an operand's value from surrounding prose is the same error
as inferring it from a shared type.

# THE TERMINATING OBSERVATION -- THE PROPERTY THAT MAKES THIS WP FINISHABLE

    forcing            terminates when? No answer exists. Every run yields a
                       layer, which is always available. Ends by BUDGET.
    static path read   terminates when the PATH ends -- lowering returns Ok, or
                       reaches the row's terminal. The path is finite, so the
                       observation that ends it is one the WORLD produces.

**Write that second line into the method section beside the method.** It is the
reason this instrument is admissible where the other two are not.

> ## THE ESCAPE HATCH IS LOAD-BEARING. IT IS NOT A HEDGE.
>
> **If the path-scoped residue under `H` comes back too large to work by hand,
> THAT SIZE IS THE FINDING: the pass reports it and stops.**
>
> Without this clause the instrument becomes the thing it replaces. *"Keep
> reading until the path ends"* has no more terminating observation than *"keep
> forcing"* once the path is large -- the ending stops being reachable and the
> pass ends by budget again, wearing a different name. **The operand pruning
> above is what keeps the path small enough for the ending to stay reachable;
> if it turns out not to, say so and stop.**

# What must not happen

- **No production change.** This WP measures. It does not repair `:9558`, does
  not implement `H`, and does not touch `core.rs:1230`.
- **No claim that the residue is bounded below what was read.** Report what the
  read covered and what it did not. An unread branch is unread, not empty.
- **No row-closure claim.** Nothing in this WP closes an ignored row, and no
  deliverable may be reported as progress against the fifteen.
- **No successor framed as "the last blocker."** A framing that cannot be wrong
  is the tell; `RT-SITEOP-RETAINED-ROWS-ADVANCED-PAST-LABEL` opens by refusing
  its own.

# Related

- `[[RT-CONTEXT-FRAME-LABEL-CORRECTION]]` — the node whose `D0`/`D1` produced
  this question. Its `D3` remains worth doing **on its own merits** and is not
  row work.
- `[[RT-CONTSRC-PRODUCER-LOCAL]]` — `merged`, and the owner of the L2 refusal
  all four rows now stop at. Cannot close them as it stands.
- `[[RT-SITEOP-CARRIED-WITNESS]]`, `[[RT-CARRIED-RESIDUAL-IH-ARITY]]` — the two
  earlier instances of the same shape: a row's blocking refusal owned by a node
  that cannot close it.
