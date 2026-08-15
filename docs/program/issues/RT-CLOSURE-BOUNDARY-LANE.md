---
id: RT-CLOSURE-BOUNDARY-LANE
title: "Admit the source-authored closure crossing on clause 2's liveness-and-domain predicate, routed through B2F's cross-owner carrier -- attempt the repair, and measure only if it fails"
status: ready
owner: runtime
size: M
gate: none
depends_on: [RT-SRCBODY-BIND-ORDER, RT-PLANNED-CLOSURE-PREEXISTENCE]
blocks: []
github: null
origin: Measured at frozen base 21fd46dc by the RT-SRCBODY-BIND-ORDER D10 differential (evt_2jc88hbzfskpm). All 16 CI failures at aa032cc2 fail at the base too -- ZERO bind-order flips -- so this is pre-existing base debt, not a regression. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> ## FRAMED AND `ready`, 2026-08-15. IT IS A REPAIR ATTEMPT, `size: M`.
>
> **The frame this node owed is below.** It was owed because nothing measured
> bounded the repair; [[RT-PLANNED-CLOSURE-PREEXISTENCE]] supplied the
> measurement that does, and this frame is cut against it.
>
> **This node was briefly framed as a classification with the repair cut after
> it. That structure is withdrawn** — operator ruling, 2026-08-15, recorded at
> the deliverables. **Make the best guess from what is known, build it, and let
> the attempt correct the guess.** The guess is stated explicitly so a reviewer
> can attack it directly rather than inferring it from the work.
>
> **`size: M` is the attempt.** The node's standing warning still applies to
> anything beyond it — *a guessed size on this campaign has been wrong every time
> it was guessed* — which is why the size covers **one honest attempt and a
> handback**, not an open-ended lane build.
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
>   The refusal is then **over-broad** — but see the correction immediately
>   below, because "over-broad" does not mean "delete the arm".
>
> ### CLAUSE 2 IS A RESTRICTION WEARING A PERMISSION'S CLOTHES
>
> **Architect `evt_3emtcx20vjg8s`, from the spec text rather than this frame's
> summary. This corrects the sizing this section originally implied.**
>
> The clause reads *"may exchange an ordinary closure **only** within one live
> runtime domain, **while the defining owner and artifact remain live**"*, and
> *"a wrong-domain, expired, or forged representation **MUST refuse before
> invocation**."*
>
> ⇒ **Even on the live-domain branch the repair is not "stop refusing" — it is
> "refuse on the RIGHT PREDICATE", carrying a liveness and domain check.** That
> is **materially larger than deleting a match arm**, and it is the sizing input
> `AC-2` needs. **Check `B2F` first.**
>
> ### THE THIRD CLAUSE FORBIDS THE FIRST THING ANYONE WILL REACH FOR
>
> **`41-values.md:84-91`, and it binds on BOTH branches.** A stable serializable
> callable MUST be exposed as an explicit `StaticCallableRef`-class value, with
> identity qualified by package/artifact, callable unit/export, and ABI
> signature, and **no captured environment** — and *"empty-capture optimization
> MUST NOT silently convert an ordinary closure"* into one.
>
> ⇒ **Defunctionalizing the zero-capture case into a static reference is
> spec-forbidden as a silent conversion.** It is the cheap-looking repair, and it
> is not available. **Know this before `D2` sizes anything.**
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

> # REFRAMED. ATTEMPT THE REPAIR. DO NOT MEASURE FIRST.
> # Operator ruling, 2026-08-15. This supersedes the classification-first
> # structure this node carried for one turn.
>
> **Verbatim:** *"Roughly 50% of design discovery happens in implementation, and
> there's often no way to understand the structure of a problem without trying to
> solve it directly. The measure node, then repair node runs directly against
> that. Measure after a failed repair, but first make the a priori best guess of
> what the repair should be with the information you have."*
>
> **The Steward's error, named so it is not re-inherited:** a measure-then-repair
> split was the correct reaction to **one** earlier failure on this campaign, and
> it was promoted into standard process and written into every frame. **QA and
> Architect review already catch mis-framings** — the split was buying a
> guarantee those reviews already provide, at the cost of a turn each time.
>
> ⇒ **This node is now a REPAIR ATTEMPT with a stated best guess.** The
> classification below is retained **as the fallback**, to be run only if the
> attempt fails, and as the vocabulary for recording the disposition.

## The a priori best guess — build this

**Steward's call on the information in this node. State it as an assumption,
attempt it, and let the attempt correct it.**

> **The crossing is a LIVE-DOMAIN CROSS-ARTIFACT EXCHANGE, and the refusal is
> over-broad.**

**Why this is the best guess and not a coin flip.** The measured facts point one
way: the closure is **source-authored** (predecessor, measured); it crosses **as
a call argument at a unit boundary between separately compiled artifacts**; and
`41-values.md:76-83` describes exactly that scenario as **permitted**. The gate
that refuses it is `transfer_into_carrier` — *"transfer a compile-time `Lowered`
into the operational carrier"* — which is the value-carrying ABI between units,
not a persistence or serialization boundary.

**So build the repair that guess implies:**

1. **Admit the crossing under clause 2's predicate, not by deleting the arm.**
   Exchange is lawful **only** within one live runtime domain and **while the
   defining owner and artifact remain live**; a wrong-domain, expired, or forged
   representation **MUST refuse before invocation**. The repair is **refuse on
   the right predicate**.
2. **Route it through `B2F`'s cross-owner callable carrier** rather than
   designing a new one. `boundary_value.rs:1319-1339` already names that as its
   design.
3. **Do not defunctionalize the zero-capture case into a `StaticCallableRef`.**
   `41-values.md:84-91` forbids it as a silent conversion. It is the
   cheap-looking route and it is closed.

**If the attempt fails, that failure is the measurement — hand it back.** A
carrier that cannot represent the value, or a boundary that turns out to be
durable-export, tells us more than the classification would have, and it tells us
from the inside. **Do not grind: one honest attempt, then the fallback below.**

> # THE GUESS HAS TWO UNSOUND JOINTS. ARCHITECT `evt_69vj8ye0qcdg9`,
> # RE-VERIFIED BY THE STEWARD. Amend before building.
>
> **This is the attack I asked for at review, and it lands. The conclusion
> survives — the crossing is a call-argument exchange inside a live runtime, not
> durable publication, and he verified `D1` himself. Two things I asserted around
> it were not established.**
>
> ## 1. DO NOT ADMIT AT THE GATE. The gate is SHARED and mostly unclassified.
>
> **Measured independently by the Steward on `main`: `transfer_into_carrier` has
> exactly EIGHT non-test call sites** — `mod.rs:7086`, `:7660`, `:8231`, and
> `core.rs:4250`, `:4457`, `:15398`, `:15716`, `:18569` — and **every one of them
> funnels through the single `boundary_transfer_admissibility` call at
> `mod.rs:6613`.** `D1` classified **two**.
>
> ⇒ **"Admit on clause 2's predicate" AT THE GATE is `refuse less` on six routes
> nobody has classified**, including any that is in fact a durable-export
> boundary — which is the one thing clause 1 forbids absolutely. **It breaks this
> frame's own constraint 1** (*"refuse on the right predicate, never refuse
> less"*), and it would do so invisibly, because the gate cannot tell which
> boundary it is standing at.
>
> **The real fork the attempt hits, and it is where to look FIRST:**
>
> | option | what it costs |
> |---|---|
> | **admit per-route, at the call site** | the two classified routes carry the admission; the other six keep today's refusal untouched |
> | **give the gate a boundary-kind parameter** | the gate learns to discriminate, and every one of the eight callers must supply its kind |
>
> **If that discrimination has no home, THAT is the "stop and report what blocked
> it" case** — and it is a better handback than a carrier built on the wrong
> seam. **Do not write a carrier before this is settled.**
>
> ## 2. "Between separately compiled artifacts" is UNVERIFIED, and it sizes it
>
> **My assertion, not a measurement.** Functionization splits a recursor into
> generated units; whether those are **separately compiled artifacts** or units
> **within one compilation output** is measured nowhere.
>
> ⇒ **If they are intra-artifact, clause 2's *"defining owner and artifact remain
> live"* is satisfied BY CONSTRUCTION on the argument-crossing shape**, the
> liveness/domain predicate has no work to do there, and the repair is
> **materially smaller** than this frame states.
>
> **And the two sub-shapes come apart here, which is the deeper error.** I
> bundled them under one predicate; `D1` measured that they reach the gate by
> **different routes**. Liveness genuinely bites on the **escape** shape — a
> captured environment outliving its lexical frame — and may bite on nothing at
> all in the argument-crossing shape. **Measure the artifact question before
> sizing the predicate.**

## Deliverables

**`D1` — the repair, per the guess above.** Both owned rows.

**`D2` — the disposition record.** Whatever `D1` produces, every expression in
the population carries a **recorded disposition** — repaired, or refused with its
spec clause cited and its **pre-retirement behaviour accounted for**. That is the
ratified closure criterion (Architect `evt_3emtcx20vjg8s`), and it is what
[[RT-LEXICAL-RECURSOR-CONSUMERS]] and [[RT-DESCENT-RETIRE]] actually need.

**Part of accounting for pre-retirement behaviour is the Architect's question:
does the DESCENT lowering perform an equivalent crossing today?** If it does, the
retirement **corrects** a live violation; if it does not, retiring the class
converts two compiling programs into refusals and that narrowing must be
**explicitly recorded**. **Answer it while dispositioning — it is not a gate in
front of the repair.**

**`D3` — the un-skip owner.** State what it takes to stop this node's CI row
being skipped. A skipped row measures nothing.

## FALLBACK ONLY — the classification, if the repair attempt fails

**Do not run this first.** It is the vocabulary for the handback and the route to
take if the guess is wrong.

Classify **per row** which production caller reaches the refusal and what it is
doing: `transfer_into_carrier` (`mod.rs:6551`) or the specialized-child screen
(`core.rs:15696`); publishing into a durable artifact, or handing a value to a
separately compiled artifact in one live domain.

| the boundary is | then | and this node |
|---|---|---|
| durable-export / persistence / serialization | the refusal is **correct by spec**, permanently | is **not** a repair for that row; the disposition is a spec question about `FrozenClosure` and routes to the Architect |
| live-domain cross-artifact exchange | the spec **grants** the lane | **owns** that row — which is the guess above, already attempted |

**`D4` — the missing-child mutation counts a hit when it changed nothing.**
Adversary `evt_1rdqcjzy9p4h8`, **verified on `main` `ddeb200a3`**. It lands in
the machinery [[RT-PLANNED-CLOSURE-PREEXISTENCE]] merged one hour earlier, and
it is a trap for whoever arms that mutation next.

**Symptom 1 — `generated_unit_call_entry_callee` (`mod.rs:7697-7714`).** At the
source-machine entry route the **unmutated lookup already fails**. That is not
an inference: `Entry(_)` is reachable **only** through the unmutated `Err` arm,
and `constructors.rs:8217` asserts every callee on that route is `Entry(_)` —
*"a declared-unit scheduling entry with no child zero must state its level"*.

⇒ **Arming the mutation there redirects a lookup that was already failing,
changes no outcome, and still counts a hit.**

**Both neighbouring mutations state this hazard and defend against it. These two
resolvers are the only ones that do not:**

> `mod.rs:7768` — *"**The hit is counted only when the coordinate actually
> CHANGES.** A call already made at the root would otherwise report a
> substitution that substituted nothing, **which is indistinguishable from a
> well-defended one**."* The occurrence twin says the same at `:7796`, and
> `constructors.rs:7863` enforces it with `assert_ne!(passed_in, used, "… a
> no-op wearing a hit count")`.

**Symptom 2, same root — `generated_unit_call_body_callee` (`mod.rs:7671-7688`)
returns `MissingBodyChild` for an unmutated `Err` WITHOUT counting a hit.** So
the variant means *either* "the mutation removed child zero" *or* "this plan node
genuinely has none", and the only separator is a **global** counter that cannot
be attributed to an event.

**Severity: masked today, live for the next row.** The existing control's weight
is carried by the tag flip `Body(StaticOriginId(49)) → MissingBodyChild`, which
genuinely discriminates because child zero exists at that route; the
`assert!(missing.1 > 0)` beside it is redundant **there**. **The moment a row
arms this mutation and rests on `hits > 0` alone — the shape `constructors.rs`
already uses for `D7` — it can be satisfied entirely by entry-route hits that
moved nothing.**

**Direction, not a prescription: neither resolver compares against its own
unmutated outcome, and one repair covers both.** The comparison is available —
the unmutated outcome is just `child_static_origin(entry, 0)`. Count only when
the two results disagree, which is exactly what `assert_ne!(passed_in, used)`
does one file over.

> **The `D3` exclusion of the five production `child_static_origin(...)?` calls
> was attacked and held.** A production `?` behaves identically in both profiles,
> so it cannot produce the test-profile-only divergence the predecessor existed
> to close. **The criterion is the argument; it needs no independent control.**
> Recorded so it is not re-litigated.

## The live stop

**Building the carrier IS this node's work now** — the earlier ban on it was the
measure-first structure and is withdrawn with it. What remains banned is
narrower and still real:

- **No `FrozenClosure`-class value.** The spec defines none, and inventing one
  here is a spec change, not a lowering repair.
- **No silent zero-capture conversion to `StaticCallableRef`** (`:84-91`).
- **No weakening of the refusal to make the rows pass.** Admitting the crossing
  means **refusing on the right predicate**, never refusing less.

**One honest attempt, then hand back.** If the repair does not come together —
the carrier cannot represent the value, the boundary proves to be durable-export,
or the liveness predicate has no home — **stop and report what blocked it.** That
report is the measurement, and it is worth more than the classification would
have been. **Do not grind, and do not fall back to classifying without saying so.**

## Acceptance criteria

**`AC-1`.** `D1` attempts the repair on the stated guess: the crossing is
admitted under clause 2's **liveness and domain predicate**, routed through
`B2F`'s carrier. **A candidate that merely classifies, with no attempt, fails
this** unless the handback states what blocked the attempt.

**`AC-2`.** Every expression in the population carries a **recorded
disposition** — repaired, or refused with its spec clause cited and its
pre-retirement behaviour accounted for, including whether the descent lane
crosses today. **This is the gate `RT-DESCENT-RETIRE` reads.**

**`AC-3`.** The refusal is **not weakened**. Admitting a lawful crossing is a
predicate change; a wrong-domain, expired, or forged representation must still
refuse before invocation, and the durable-export case must still refuse
outright.

**`AC-3a`.** **No admission is installed at the shared gate.** The six
unclassified `transfer_into_carrier` call sites must reach **today's refusal,
unchanged**, demonstrated rather than asserted. **A candidate that relaxes
`boundary_transfer_admissibility` itself fails this**, however the predicate is
written, because the gate cannot tell which boundary it is standing at.

**`AC-3b`.** If the repair needs the gate to discriminate boundary kinds and
there is no home for that discrimination, **stop and report it** — that is a
finding about the seam, and it discharges the attempt.

**`AC-4`.** No `FrozenClosure`-class value, and no silent `StaticCallableRef`
conversion of the zero-capture case. **`D4` is diagnostic-correctness work and
does not breach this.**

**`AC-4a`.** `D4` lands: both resolvers count a hit **only when the mutated and
unmutated lookups disagree**, and `MissingBodyChild` distinguishes a mutated
removal from an honest absence. **Demonstrated** — arm the mutation at the
**entry** route and show the hit count **unchanged**. **A control that only arms
it at a closure route does not discharge this**, because that is the route where
the defect is already masked.

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
