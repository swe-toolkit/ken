---
id: RT-CLOSURE-BOUNDARY-LANE
title: "Classify which spec clause governs each refused closure crossing -- durable-export, where the refusal is CORRECT, or live-domain cross-artifact exchange, where the spec GRANTS a lane the implementation has not built -- then size the repair"
status: ready
owner: runtime
size: S
gate: none
depends_on: [RT-SRCBODY-BIND-ORDER, RT-PLANNED-CLOSURE-PREEXISTENCE]
blocks: []
github: null
origin: Measured at frozen base 21fd46dc by the RT-SRCBODY-BIND-ORDER D10 differential (evt_2jc88hbzfskpm). All 16 CI failures at aa032cc2 fail at the base too -- ZERO bind-order flips -- so this is pre-existing base debt, not a regression. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> ## FRAMED AND `ready`, 2026-08-15. `size: S` IS THIS INCREMENT, NOT THE REPAIR.
>
> **The frame this node owed is below.** It was owed because nothing measured
> bounded the repair; [[RT-PLANNED-CLOSURE-PREEXISTENCE]] supplied the first
> measurement that does, and this frame is cut against it.
>
> **`size: S` describes `D1`-`D2` — a classification and a routing — and nothing
> else. The repair is still unsized and is a separate cut made from `D1`'s
> result.** The node's standing warning is unchanged and is why this frame does
> not guess: *a guessed size on this campaign has been wrong every time it was
> guessed.* Do not read `S` as a bound on the lane.
>
> It also still exists so that a **skipped CI row has an owner**. A skipped row
> measures nothing; the node that owns it owns **un-skipping** it.

> ## A SECOND POPULATION NOW REACHES THIS SIGNATURE. DO NOT ABSORB IT.
> ## Steward, 2026-08-15.
>
> [[RT-REQUIRED-CONSUMER-REACH-CENSUS]] `D1` measured **row 4 depths 2 and 3 of
> the recursor campaign** refusing at `lowering/mod.rs:11550-11552` with **this
> node's exact sentence** (`evt_6qc0vkzj43c0e`, base `a737d8c9b`).
>
> **That is not a reason to widen this node's rows**, and the reason is
> structural rather than procedural. The site is the
> `Lowered::Closure | Lowered::DeclarationClosure` arm of
> **`boundary_transfer_admissibility`** — a **total, wildcard-free walk over the
> whole value graph**. ⇒ **Every closure-carrying graph that attempts the
> crossing refuses here.** A shared sentence is evidence the gate is total, not
> evidence of a shared production root.
>
> **The upstream fork is with the Architect** (`evt_7rpkfc7awktmb`): for the
> recursor rows, is a closure in the crossing graph **correct** — in which case
> the durable lane this node is about is the shared repair and a subsumption is
> real — or **incorrect**, in which case that chain owns a lowering fix that
> never reaches this gate and the convergence is a coincidence.
>
> **Until that is ruled: do not add the recursor rows here, do not cite this
> node as their owner, and do not treat this node's size as covering them.**
> Whichever way it goes, the frame this node still owes is unchanged.

> ## RETRACTED. THE SUBSUMPTION IS LIVE AGAIN, AND MY
> ## EXCLUSION OF IT WAS VACUOUS. Steward, 2026-08-15.
>
> **A block here previously read "RULED. THE RECURSOR ROWS ARE NOT THIS NODE'S"
> and declared the fork settled against subsumption. That ruling was wrong and
> is withdrawn.** It is restated in full below so the error is legible rather
> than merely deleted.
>
> **What it said.** [[RT-REQUIRED-CONSUMER-REACH-CENSUS]] `D5` measured the
> enabled rows at `(closure_present, crossing_reached) = (true, true)` and both
> suppressed legs at `(false, false)`. The Architect's dispositions named one
> branch under which the two populations could be one defect: closure PRESENT
> under suppression **and the crossing ALSO reached under suppression**. I
> observed the crossing is not reached under suppression and concluded that
> branch was excluded by measurement.
>
> **Why that is vacuous. Suppression removes the crossing by construction — that
> is what suppressing the required-consumer route DOES.** So
> `crossing_reached = false` is guaranteed on every suppressed row **in every
> possible world**, whether or not the closure pre-exists. A conjunct that cannot
> be satisfied under the condition being tested excludes nothing. **I read a
> guaranteed reading as a discriminating one**, and the whole elimination rests
> on that one step.
>
> **It is the same defect I had flagged twice that morning in others' work** —
> the census's own `closure_child_present: false` on suppressed rows was already
> recorded as an artifact of having no observation point. **The second conjunct
> is an artifact for exactly the same reason, and I did not carry the reading
> across.**
>
> ⇒ **The durable-lane branch is live and it is this node's mechanism.** The
> Architect, resolving `dec_6hwh86vdzp2ha` on [[RT-CROSSING-CALLEE-IDENTITY]]:
> closing branch 3' *"leaves TWO branches, and the survivor is the Steward's
> ORIGINAL 'correct' branch: (1) the realization produces a closure-shaped value
> into an intended call-input route ... versus (durable-lane) the source value
> legitimately carries a closure through a source-authored call and Ken has no
> lane => **`RT-CLOSURE-BOUNDARY-LANE`'s mechanism after all**."*
>
> **So the sizing guidance also reverses.** Do **not** size this node as covering
> only the row it already lists. Whether the recursor rows are its rows now turns
> on **closure pre-existence**, which is still open.
>
> **Do not try to settle it by suppression a third time.** The Architect's
> sequencing (`dec_6hwh86vdzp2ha`): *"STOP ASKING THE RUN, ASK THE PLANNER."*
> Without the projection these rows never build the subgraph, so the question has
> no observation point on that path — but it has one on the planning side. Ask
> whether the **planned** occurrence at origin 5 carries a closure-typed field 0
> **by construction**. Planner says closure ⇒ pre-existing ⇒ durable-lane, and
> these rows are this node's. Planner says otherwise while lowering produced
> `Lowered::Closure` ⇒ the realization manufactured the shape ⇒ branch 1, and
> they are not.

> ## ANSWERED. THE PLANNER SAYS CLOSURE. THE ROWS ARE THIS NODE'S.
> ## Steward, 2026-08-15, on [[RT-PLANNED-CLOSURE-PREEXISTENCE]] exact `1cd9947cf`.
>
> **Branch 1 is refuted and the durable-lane branch is selected on measured
> evidence.** Both governed plans give field 0 of origin 12 a closed producer set
> of exactly `[(StaticOriginId(12), LexicalClosure)]`. **The realization did not
> manufacture the closure — the source program authored it.**
>
> **The classification carries its own discriminating power**, which is why it
> settles a question two suppression differentials could not: mutating the real
> source field to an integer flips the classification `LexicalClosure → Other`.
> It is a measurement, not an argued shape.
>
> ⇒ **Row 4 depths 2 and 3 of the recursor campaign are routed here**, and the
> Architect's earlier *"do not fold"* is **superseded by him, explicitly**
> (`dec_55a8p5m2f8gzk`): *"That reasoning still holds; the EVIDENCE changed."*
> Nobody should read the block above this one as still blocking the fold.
>
> ### POOL THEM FOR SIZING. DO NOT ASSUME ONE MECHANISM.
>
> **Architect, non-blocking, and it is the load-bearing caveat on this handoff.**
> The measurement covers **these two rows only**. It does **not** measure the
> escape rows, so *"one population"* remains one step ahead of the evidence, and
> the two sub-shapes differ in **how the closure must survive**:
>
> | sub-shape | rows | the closure must survive |
> |---|---|---|
> | escape-lifetime | this node's original `rt_escape_second_resource_native` row | **outliving its defining frame** |
> | argument-crossing | recursor row 4 depths 2 and 3 | **crossing as a call argument at a unit boundary** |
>
> Both need a representable closure, so **pooling them for sizing is right**.
> **Size against both sub-shapes rather than assuming uniformity** — a node sized
> for escape-lifetime meets the argument-crossing case late.

## Exact signature

```text
Closure: a closure cannot cross the boundary: it is runtime-local and live-domain only, and it has no durable lane
```

> # THAT SENTENCE NAMES TWO SPEC CLAUSES AND OBEYS ONLY ONE OF THEM.
> # Steward, 2026-08-15. This is what `D1` exists to settle.
>
> **This node's title and its `Why this is NOT` section below both call the
> refusal a *gap*. That word was never grounded against the spec, and the spec
> that governs it is merged.** [[SPEC-CLOSURE-BOUNDARY]] landed 2026-07-26 (PR
> #982, exact `0ccca4c5`) and `spec/40-runtime/41-values.md` now states **two
> different rules about a closure meeting a boundary**:
>
> | spec | clause | what it says about a closure crossing |
> |---|---|---|
> | `41-values.md:72-75` | **Transitively non-persistable** | a persistence, canonical-store, Merkle/serialization, or **durable-export** boundary **MUST reject** it, and MUST NOT substitute a pointer, ordinal, digest, or process-local handle |
> | `41-values.md:76-83` | **Live-domain invocation only** | **separately compiled artifacts MAY exchange an ordinary closure** within one live runtime domain while the defining owner and artifact remain live; the receiver **may invoke it at its checked callable type** |
>
> **The refusal sentence asserts the first and denies the second in one breath.**
> *"It is runtime-local and live-domain only"* is clause 2's **grant**, and
> *"it has no durable lane"* is clause 1's **prohibition** — and the code emits
> them together as a single unconditional rejection of every closure reaching
> `boundary_transfer_admissibility`.
>
> ⇒ **Two readings, and they size completely differently:**
>
> - **The boundary these rows cross is a durable-export boundary.** The refusal
>   is then **correct by spec and correct forever**, this node is not a repair,
>   and a durable higher-order value is `41-values.md:92-95`'s **separate
>   `FrozenClosure`-class abstraction** — which the spec deliberately does not
>   define. That is spec work, not a runtime lane.
> - **The boundary these rows cross is a live-domain cross-artifact exchange.**
>   The spec **already grants the lane** and the implementation has not built it.
>   The refusal is then **over-broad**, and the repair is bounded: a typed opaque
>   carrier with explicit owner and lifetime.
>
> **I am not deciding which. The premise is grounded enough to measure and not
> grounded enough to rule** — and the two answers differ by roughly the whole
> size of this node, which is exactly why it must be measured before sizing.
>
> ### The observation point exists and is small
>
> `boundary_transfer_admissibility` is a **total, wildcard-free walk** with its
> closure arm at `lowering/mod.rs:11718-11729`. Its production callers are
> **two**, both screening a whole graph ahead of the first allocation:
> `transfer_into_carrier` (`mod.rs:6551`) and the specialized-child screen
> (`core.rs:15696`). **The gate is the same for both readings; what differs is
> what the caller is doing**, and the caller is enumerable.
>
> **Neither the gate nor its message may be weakened to settle this.** If the
> answer is "durable-export", the refusal stays exactly as it is.
>
> ### BOTH BRANCHES ALREADY HAVE AN OWNER. DO NOT RE-DERIVE EITHER.
>
> **Measured at `boundary_value.rs:1319-1339`, and this is the most useful thing
> on this record for whoever picks the node up.** The ABI already carries a
> dedicated status, `BOUNDARY_ERR_RETIRED_LANE` (`-12`), whose doc settles both
> halves by citation:
>
> | branch | who owns it | status |
> |---|---|---|
> | durable closure lane | **retired by Architect decision** — `RT-FNSPLIT-C1` `D5`, `dec_21aa95jbsznfh` plus addendum `dec_6xffebwj4s347` | the `(PersistentClosure, Closure)` pair is **recognized ABI vocabulary that is never admitted** |
> | callable **cross-owner carrier** | **[[RT-FNSPLIT-B2F]]'s design** (`merged`) — the attribution is `boundary_value.rs`'s own words, *"a callable cross-owner carrier is `B2F`'s design rather than this node's"*, not a claim read off the `B2F` node | exists as a design; whether it is the lawful route for the argument-crossing sub-shape is **`D1`'s to check, against `B2F` itself** |
>
> ⇒ **If `D1` lands on durable-export, the answer is not "build a lane" — a lane
> was built, ruled on, and retired, and the refusal is that ruling being
> enforced by name.** The ABI keeps the retired pair in its vocabulary
> *specifically so it can be refused by name* rather than as an unrecognized
> byte, which is why this is discoverable at all.
>
> ⇒ **If `D1` lands on live-domain cross-artifact, check `B2F`'s carrier BEFORE
> proposing anything.** A cross-owner callable carrier may already be the
> designed route, in which case the repair is wiring rather than design, and
> this node's size collapses accordingly.
>
> **This is why `D1` is a classification and not a repair.** Two decisions and a
> merged design bear on the answer, and none of them was visible from the
> refusal sentence this node was named after.

## Rows it owns

- \`rt_escape_second_resource_native\` \`escaped_resource_used_by_fanning_host_op_matches_interpreter\`
- **recursor row 4 depths 2 and 3** — routed here 2026-08-15 on the planner
  measurement above. Pooled for sizing, **not** assumed to share a mechanism.

## Deliverables

**`D1` — classify each owned row's boundary against the two clauses.** For every
row this node owns, name **which of the two production callers** reaches the
refusal and **what that caller is doing at that moment**: publishing into a
durable/serialized artifact, or handing a value to a separately compiled artifact
inside one live runtime domain.

**Answer per row, not for the node.** The two sub-shapes reached this gate by
different routes and there is no measurement yet that they answer alike. **A
single verdict covering both rows fails `AC-1`** unless it is derived per row and
the rows then agree.

**`D2` — route and size, against the table.** State plainly, per row:

| `D1` reads | then | and this node |
|---|---|---|
| a durable-export / persistence / serialization boundary | the refusal is **correct by spec**, permanently | is **not** a repair for that row; the row's disposition is a spec question about `FrozenClosure`, and it routes to the Architect |
| a live-domain cross-artifact exchange | the spec **grants** the lane and it is unbuilt | **owns** that row, and `D2` states the size against both sub-shapes |

**If the rows split across the two branches, say so and stop.** A split is a
real and useful outcome — it means this node owns one row and not the other, and
that is a cheaper thing to learn now than after a repair is cut.

**`D3` — the un-skip owner.** Whatever `D1` returns, state what it takes to stop
this node's CI row being skipped. A skipped row measures nothing, and that
obligation does not move with the classification.

## The live stop

**Do not build a carrier, a lane, or a `FrozenClosure` in this node.** `D1` is a
classification and `D2` is a routing. **The repair is cut after them, from their
result** — the same sequencing that just worked twice on this lane, and the
reason the fork closed on a measurement instead of a fourth guess.

**If `D1` cannot separate the two clauses from the caller,** say so and stop
rather than picking the likelier one. That is a finding about the boundary's
observability and it routes to the Architect, not to a fourth measurement.

## Acceptance criteria

**`AC-1`.** `D1` classifies **per owned row**, naming the production caller and
citing `41-values.md:72-75` or `:76-83` for each. **A node-level verdict fails
this**, and so does a classification that names the gate without naming what the
caller was doing.

**`AC-2`.** `D2` states the routing and, for any row landing on the live-domain
branch, a size **derived against both sub-shapes** — escape-lifetime and
argument-crossing. **A size taken from the escape row alone fails this**, and
the Architect's caveat is why.

**`AC-3`.** Neither `boundary_transfer_admissibility` nor its refusal message is
weakened, narrowed, or made conditional. **The classification is a reading of the
callers, not an edit to the gate.**

**`AC-4`.** No closure carrier, durable lane, or `FrozenClosure`-class value is
introduced. **`D1`-`D3` are classification and routing work.**

**`AC-5`.** Nothing added to a production build surface, verified by a targeted
`ken-runtime` check rather than asserted.

**`AC-6`.** No-regression, in CI (`COORDINATION §12`).

## Why this is NOT [[RT-CARRIER-BYTESPAN-OBSERVE]]

**Different mechanism.** Closures refused at the boundary transfer walk, not an
effect-seat availability question. That distinction is unaffected by `D1`.

> **This paragraph used to read *"a representation/lane gap"*.** Corrected
> 2026-08-15: **whether it is a gap is `D1`'s question**, and calling it one
> presumed the answer. The word travelled into this node's title and stood
> unchallenged against a merged spec for three weeks.

## Provenance

**Fails at frozen base `21fd46dc`, so it is not caused by the de Bruijn
binding repair.** Measured per row with `--no-fail-fast`; see the hazard note
in the D10 handback -- `cargo test` with several `--test` flags is fail-fast
**per binary**, and a partial run reads as a complete one.
