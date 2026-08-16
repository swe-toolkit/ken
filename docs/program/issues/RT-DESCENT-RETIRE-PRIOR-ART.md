---
id: RT-DESCENT-RETIRE-PRIOR-ART
title: "The retirement's discharge is now a research referral -- Ken's own rules cannot settle source-reachability of the retained difference, so ask how other implementations closed the same shape and what that says about retiring the lane"
status: active
owner: research
size: M
gate: none
depends_on: [RT-MATCH-DIFFERENCE-REACHABILITY]
blocks: [RT-DESCENT-RETIRE]
github: null
origin: "Operator directive 2026-08-16, verbatim: 'Prior art indicates that retiring RecursiveDescent is possible and observed resource usage by that implementation makes it desirable. If outcome 2 or 3 is returned by runtime-implementer, that is a clear signal to refer the matter to research for guidance.' RT-MATCH-DIFFERENCE-REACHABILITY D1 returned outcome 3 (PR #2463, Architect dec_5chyprtg9bn7c). Steward-filed per COORDINATION section 2 so the capstone's bar cites an owned node rather than a message."
---

## What this node is

**The referral the operator directed in advance, now triggered.** It is the
only thing between the federation and retiring `RecursiveDescent`.

**Ken's own rules have been asked and cannot answer.**
[[RT-MATCH-DIFFERENCE-REACHABILITY]] settled that much: the runtime ring
consulted the surface grammar, elaborator admission and the kernel gates, and
**no rule in those layers refuses the shape** — so source-unreachability cannot
be claimed, and the retained difference cannot be deleted. That is a complete,
accepted answer, not a failure, and it is what triggers this node.

> ### ONLY THIS SEAT CAN ASK THE PRIOR-ART QUESTION, AND THAT IS THE POINT
>
> `CLEAN-ROOM.md` puts the permissive and copyleft references off-limits to
> every build seat. **The runtime ring could not have consulted prior art if it
> wanted to**, so its outcome 3 is not a shortfall — it is the boundary of what
> that seat may know. This node asks the one question the previous one was
> structurally unable to ask.

## The fixed input — outcome 3's measured rule gap

**Do not re-measure this and do not start from the campaign's history.** The
following is `D1`'s result, QA-verified (`evt_27jq5cpgnvaw4`) and
Architect-accepted (`evt_4vvv46232nxdp`). The coordinates are the ring's, taken
from the node; treat them as a map, and re-read any line you build an argument
on.

1. **The difference is real at the backend.** `MatchScrutineeRecursor` is
   retained exactly when a `Match` scrutinee is a `ComputationalMatch` with some
   case carrying recursive positions **and** the ordinary producer route
   declines it. `D2a` constructed a member, so the set is non-empty as a
   backend-IR shape and no measurement will ever empty it.
2. **Normalization does not remove the shape.** The native driver makes selected
   recursive declarations opaque, but the kernel rebuilds a stuck `Term::Elim`
   for a neutral scrutinee and full normalization preserves it. The earlier
   attempt normalized **because of the program it chose**, not because a rule
   requires it.
3. **The elaborator refusal that looked like a gate has an escape.**
   `StructuralResultOutOfScope` refuses the un-ascribed spelling, but `RAsc`
   routes the same expression through checked elaboration, which admits the same
   recursive-result selectors.
4. **What actually stops a witness today is a compiler-path invariant.** Present
   native checked-value erasure wraps every computational match in a
   `CheckedSubcontinuationFrame` before an enclosing `Match` can see it, so the
   enclosing match never has an immediate `ComputationalMatch` scrutinee.
   **Generic non-plan erasure can emit the bare shape**, so the wrapper is not a
   language or kernel theorem — and treating it as one is the exact
   compiler-behaviour-as-language-invariant error that blocked an earlier
   deliverable in this campaign.
5. **The unavailable proposition, named:** a grammar, elaborator-admission or
   kernel rule requiring every admitted computational scrutinee to carry that
   wrapper. **No such rule exists in the consulted layers.**

## The question

**How do other implementations compile a recursive eliminator whose scrutinee is
itself a recursive computation, without keeping a second, whole-function
recursive-descent lane alive for the residue?**

Ken's `RecursiveDescent` is a migration selector the code itself calls
temporary, and the campaign exists to delete it. The blocker is not a missing
capability — the functionized lane handles everything measured — but an
**unprovable negative** about a shape nobody can currently write.

**Three sub-questions, in the order that changes what we do:**

1. **Is the shape one that mature implementations admit at all?** If comparable
   systems refuse it at elaboration or normalize it away **by rule**, name the
   rule and its enforcement point. That is the shape of argument Ken's method
   gate would accept, arrived at from outside.
2. **If they admit it, what do they emit?** A single uniform lowering, a
   normalization that provably removes the case, a restriction on scrutinee
   form, or something Ken has no analogue for.
3. **Does the wrapper have a principled version?** Point 4 above says a current
   erasure path incidentally closes the route. **If prior art makes that kind of
   framing an enforced invariant rather than an artifact, say so** — converting
   it into a stated, checked property is a route to retirement that Ken can
   build, and it is the bridge outcome 3 leaves dangling.

## What the operator's ground adds, and what it does not

> Prior art indicates that retiring `RecursiveDescent` is possible and observed
> resource usage by that implementation makes it desirable.

**Treat "possible" as a hypothesis to test, not a conclusion to confirm.** If
prior art does not in fact support it, **say that plainly** — the referral is for
guidance, and a negative finding is guidance. The federation has twice paid for
the difference between an argument that holds and one that was reached for.

**The desirability half needs nothing from you.** The campaign's efficiency
ground is already on record (2026-07-29 directive; the per-function code-size
wall). Do not spend the turn re-arguing that the lane should go.

## Deliverables

**`D1` — a written advisory at `docs/program/17-descent-retirement-prior-art.md`.**
Findings against the three sub-questions, each attributed to the system it came
from, with Ken's applicability stated separately from the observation.

**`D2` — a recommendation with its confidence, and the next actionable step.**
One of: an argument shape Ken could adopt to close the route; an engineering
change that would make it closable; or "prior art does not support retirement as
scoped," which is a real answer and routes back to the operator.

## Acceptance criteria

**`AC-1`. The advisory is a durable file, not a thread post.** An in-thread
ruling is not a deliverable, and the capstone's bar must cite something a cold
reader can open.

**`AC-2`. Every finding names its source system and separates observation from
inference about Ken.** *"Lean does X"* and *"therefore Ken could do Y"* are two
sentences with different warrants.

**`AC-3`. Clean-room discipline, and it is load-bearing rather than
ceremonial.** Permissive references may be read to understand; copyleft
references (GPL/AGPL/CeCILL) for **approach and behavior only**, under the
leakage recheck. **Nothing is copied into the repo and no reference source text
is quoted into an artifact a build seat will read.** The AGPLv3 prototype is not
mounted and is not consultable by anyone.

**`AC-4`. A negative finding is written as plainly as a positive one**, and does
not get softened because the operator's stated expectation points the other way.

**`AC-5`. No claim rests on a system's documentation where its behaviour was the
question.** Inherited from the predecessor's method gate.

## Banned scope

- **Any change under `crates/`, and any deletion, narrowing or re-scope.** This
  node is advisory. The capstone's bar lifts on a ruling, not on this file.
- **Re-measuring Ken's rules.** That was `D1` and it is accepted; start from the
  fixed input above.
- **Writing Ken code from a reference.** `CLEAN-ROOM.md` — implementers build
  from `/spec`, and this advisory is read by the Architect and the Steward, not
  transcribed into a lowering.

## Sequencing

**`active`. Released to the research seat at `evt_1nphjvhzs39e0`, `main` = `483d740eb`.**

**Blocks [[RT-DESCENT-RETIRE]], and it is that node's eleventh dependency and
its sole live discharge.** The other ten are `merged`; the capstone is barred by
its own recorded text, not by an open edge.

**This is lane 1 under the operator's directive** — it is the retirement, in the
only seat that can advance it right now. The runtime ring is not blocked on it
and is not waiting: its next lane-1 work is independent of this answer.
