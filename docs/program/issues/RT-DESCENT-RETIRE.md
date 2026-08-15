---
id: RT-DESCENT-RETIRE
title: "Retire RecursiveDescent — delete the migration selector, the residual enum, the authority variant, and the recursive-descent emission lane"
status: draft
owner: runtime
size: M
gate: none
depends_on: [RT-DECL-CLOSURE-PORT, RT-SEED-CALL-PORT, RT-PRODUCER-MATCH-PORT, RT-RECURSOR-TRANSPORT, RT-FNUNIT-RESULT-TOKEN, RT-LEXICAL-RECURSOR-CONSUMERS, RT-CLOSURE-CROSSING-ELIMINATE]
blocks: []
github: null
origin: Operator directive 2026-07-29 — "we should not let it linger in a half-migrated state. That just carries tech debt for no benefit." Campaign docs/program/16-recursive-descent-retirement.md. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> # `draft`, NOT unframed — the frame is written and the premise is not yet true
>
> **Set back to `draft` 2026-08-13 by the Steward. It was mis-flagged `ready`
> while `RT-RECURSOR-TRANSPORT` — a `depends_on` and a genuine mechanism
> dependency — was itself still `ready`.** Nothing had landed, so the node's
> premise ("all residual classes retired") was false the whole time it sat on
> the frontier.
>
> **Runtime paid for that.** `RT-DESCENT-RETIRE` `D1` was pulled and
> hard-stopped at fresh base `c1b9a1e8`: the exhaustive unarmed enumeration
> found **89 intact non-empty residual rows** (74 `LexicalCallArgumentRecursor`,
> 15 `MatchScrutineeRecursor`) and production still selecting
> `BodyEmissionAuthority::RecursiveDescent` **29 times**. Both residual variants
> and both classifier arms remain in production. The observer was fully
> reverted; no candidate exists; `D2`-`D7` were never entered.
>
> **That measurement is worth keeping.** It is a clean unarmed census at a named
> base, and it is exactly the dead-code oracle this node's `D1` is supposed to
> capture. When the node is genuinely released, `D1` re-runs against a base where
> the answer should be zero — and the `c1b9a1e8` numbers above are the control
> that makes a zero meaningful rather than vacuous.
>
> **Flip to `ready` when `RT-RECURSOR-TRANSPORT` is `merged`**, not before, and
> not on a partial: this node's dependency is the mechanism, and `D1` re-run
> must find the residual population empty.
>
> **Two `depends_on` edges added 2026-08-15 to stop that same mis-flip recurring
> for the other residual class.** The census above found **74
> `LexicalCallArgumentRecursor`** rows, and nothing in the original dependency
> list governed them — so once the five listed nodes merged, `gen-progress.sh`
> would have shown this node flip-eligible with the larger residual class fully
> intact. **The prose guard alone was not enough; it is the same shape that put
> this node wrongly on the frontier the first time**, and `depends_on` is what
> the generator actually reads.
>
> ⇒ Added **[[RT-LEXICAL-RECURSOR-CONSUMERS]]** (the lexical class) and
> **[[RT-CLOSURE-CROSSING-ELIMINATE]]** (where that node's remaining population
> is now dispositioned). **The second edge is the load-bearing one**: retirement
> cannot be assessed while it is still open whether the remaining expressions get
> a repair or a recorded refusal, because a refusal makes retirement a
> **narrowing of a presently-compiling capability** rather than debt removal.
> That is a product call, and this node is where it lands.
>
> **AMENDED 2026-08-15 — "presently-compiling" meant "compiling under
> `RecursiveDescent`", and that baseline is retired.** See the oracle ruling
> below. The edge stands and the reason changes: what must be known before
> retirement is assessed is **what the interpreter does**, not what
> `RecursiveDescent` did.

> # `D0`'s promise-class sentence WILL BLOCK YOU, and it is worded wrong
>
> **Read this before you touch
> `recursive_descent_recursors_compile_without_a_boundary_crossing`.** Its
> promise class reads:
>
> > *"Promise class: transition sentinel. Retirement or an authorized boundary
> > repair must rewrite this route comparison rather than preserve its current
> > exact outcomes."*
>
> **That sentence names this node explicitly, and as written it forbids the
> hardening edit rather than the thing it means to forbid.** An arm-order
> inversion — or any added arm, or a stronger anchor — **preserves both outcomes
> exactly** and changes only how they are established. So it lands on the
> *"preserve its current exact outcomes"* side, which is the side the sentence
> exists to prohibit.
>
> ⇒ **The sentence states a requirement on future SEMANTIC change and omits the
> LICENCE underneath it.** The licence is that the promise is the **route
> comparison**, not the exact outcomes — so edits that preserve both outcomes
> while strengthening how they are established are permitted and expected. That
> is implied by the class name and never written, so an author who checks the
> promise class before hardening the control finds what reads as a ban.
>
> **Fix the clause when you get here.** A promise class that has to be
> interpreted is not governing. Adversary `evt_2ka6ngwcm5r44`.
>
> **`D0`'s own non-vacuity is settled by MEASUREMENT, not by reading — use it.**
> The Architect enumerated the shared mutable state and found no memoization
> hazard, and was explicit that this was a reading rather than a measurement. The
> Adversary then ran the inversion on both depths in one process:
>
> ```
> depth=2 FIRST=unexcluded crossings=0   SECOND=excluded crossings=2
> depth=3 FIRST=unexcluded crossings=0   SECOND=excluded crossings=2
> ```
>
> ⇒ **The empty `RecursiveDescent` observation survives being first**, on a
> compile nothing could have memoized, **and the non-empty anchor survives being
> second.** Both orders, both depths. `D1`'s residual census leans on `D0`, so
> this is the evidence that makes a zero meaningful — landing the inversion is now
> optional, not load-bearing.
>
> **One gap left open deliberately:** `_excluded_result` is discarded, so the
> excluded arm's *compile outcome* is unasserted. Correct as written — the
> crossing is recorded before the refusal, and pinning it there would duplicate
> the `D5` control — but **this control would not notice if the excluded arm
> changed from refusing to compiling.** One line if you want it; not a defect.

> # OPERATOR RULING: THE ORACLE IS THE INTERPRETER, NOT `RecursiveDescent`
> # 2026-08-15. This governs every "narrowing" sentence in this node.
>
> Verbatim:
>
> > `RecursiveDescent` should not be taken as de facto spec. It was a failed
> > implementation attempt that needs to be replaced. The key oracle is not
> > `RecursiveDescent`, but the interpreter.
>
> **"Accept and record the narrowing" is no longer an available decision, because
> the thing it would narrow relative to is not a specification.** Every option in
> the fork below was priced against `RecursiveDescent`'s accepted set. A failed
> implementation attempt does not define the target it failed to hit.
>
> ### What SURVIVES this ruling, stated precisely so it is not over-read
>
> **The `41-values.md` argument below is NOT killed by it.** That argument says
> local dispatch machinery is permitted *"only when it cannot affect
> program-observable results"*, so a `RecursiveDescent`/`FunctionizedUnits`
> disagreement about which programs compile is a defect **regardless of which
> backend is right**. That still holds.
>
> **What the ruling supplies is the tie-breaker the argument lacked.** A
> disagreement between two backends is resolved by the oracle:
>
> | interpreter | resolution |
> |---|---|
> | **runs the governed rows** | `FunctionizedUnits` must too — the refusal is a compiler defect and repairing it is convergence |
> | **refuses them** | `RecursiveDescent` was over-accepting; its behaviour is a bug to drop, not a capability to preserve |
>
> ⇒ **Retirement never had to wait on a product decision about narrowing. It
> waited on a measurement**, which was [[RT-SYNTHESIZED-ENV-RECORD-OCCURRENCE]]
> `D3`.
>
> **`D3` REPORTED 2026-08-15 AT 19:04 AND THIS CLAUSE IS DISCHARGED.** The
> governed rows have **no source-level witness**: the shape is constructible
> only as hand-authored `RuntimeExpr`, and both surface routes fail before
> checked-artifact emission — naming the W-style recursive result is
> `Elaboration(StructuralResultOutOfScope)`, recomputing it by self-call is
> `KernelRejected(NotTerminating)`. **No program a user could write reaches
> them**, so there is no capability question in either direction and the fork
> `evt_3yvhf3hz59eb8` is void rather than answered.
>
> ⇒ **Retirement no longer waits on this node at all.** What it waits on is its
> own `depends_on` edge, where the live members are
> [[RT-LEXICAL-RECURSOR-CONSUMERS]] and [[RT-RECURSOR-TRANSPORT]].
>
> **Do not read this as "retire now".** The predictability defect is real until
> the two backends agree with the oracle; the ruling changes what agreement means,
> not whether it is owed.

> # DO NOT RULE ON THE FORK: ITS SIZING IS WITHDRAWN AND A SUCCESSOR IS OPEN
> # Steward, 2026-08-15, second correction of the day, same direction.
>
> **The fork's "cover it" option was priced as inventing a cross-unit
> representation. That was not substantiated and is withdrawn.** The mechanism
> for a compiler-created aggregate to carry planner authority is production code:
> `AggregateOccurrenceProducer::SynthesizedUse`
> (`planning/static_transition.rs:3956`), populated at `:5754`, whose own doc
> says a synthesized aggregate *"is named by the closed compiler role that builds
> it."* What is missing is that its vocabulary is host-result-shaped, which is an
> extension of a closed mechanism rather than an invention.
>
> ⇒ **[[RT-SYNTHESIZED-ENV-RECORD-OCCURRENCE]] attacked it and REPORTED.** The
> question "are these rows repairable?" was dissolved rather than answered:
> **nothing reaches them from source**, so there is nothing to repair and
> nothing to narrow. **"Accept the narrowing" is not a decision anyone needs to
> make**, and no operator ruling is owed for `row4-depth-2/3`.
>
> **Two of the Steward's three lane-1 statements today were wrong in the same
> direction — toward this campaign being more finished than it is.** Treat a
> claim that lane 1 has no next step as suspect until it carries its measurement.

> # THE PRODUCT FORK IN FRONT OF THIS NODE WAS POSED ON A FALSE PREMISE
> # Steward, 2026-08-15. Do not act on the fork as previously stated.
>
> The fork put to the operator was: **cover the refused rows (requires inventing
> a cross-unit representation, large node) / accept and record the narrowing /
> stop the campaign.** The first option was priced against a prohibition that
> does not exist.
>
> **`spec/40-runtime/41-values.md:76-83` specifies live-domain cross-artifact
> closure exchange**, and `:116-118` explicitly declines to constrain its
> mechanism. What the chapter forbids is the **durable** lane (`:73-76`), which
> is the retired `(PersistentClosure, Closure)` pair — a different pair from the
> one a live-domain crossing would need. The "no `(tag, class)` admission"
> criterion that this was read out of is a **scope boundary on
> [[RT-CLOSURE-CROSSING-ELIMINATE]]**, phrased as a handback trigger, not a
> statement about the ABI.
>
> ⇒ **ANSWERED by the Architect at `evt_1ra9asrda1t94`, and the answer voids
> both branches it was posed with.**
>
> **The live-domain clause does not reach this boundary — its subject is
> "separately compiled artifacts", and there is only one artifact.** Every
> generated unit is `Linkage::Local` (`units.rs:940`, `:967`, `:983`, `:1005`;
> zero non-`Local` linkages in the file) and all are declared into one
> `ObjectModule` per compile (`artifact/mod.rs:186`), emitted to one object with
> one hash. The unit boundary is a local call between two module-local functions.
> Re-verified by the Steward.
>
> **So the rows are not refusing a granted lane — and retirement is also not a
> narrowing of an unspecified convenience.** What the spec does reach is
> `41-values.md`'s removed-constraints paragraph: local machinery for dispatch is
> permitted *"only when it cannot affect program-observable results."*
> Functionized units are dispatch machinery, and the governed rows **compile
> under `RecursiveDescent` and refuse under `FunctionizedUnits`.**
>
> ⇒ **Retiring `RecursiveDescent` today would ship a compile-time behavioural
> difference attributable to nothing in the program** — principle 10,
> predictability. That is the defect, and it is not a closure-lane question.
>
> **The obligation this creates is narrower and cheaper than the cover option as
> previously priced.** No owner/lifetime encoding and no refuse-before-invocation
> check are owed. What is owed is: **the unit split must not change which
> programs compile.**
>
> **The inference is attackable and the Architect said so.** The sentence's plain
> subject is closure-representation machinery; reading it to cover unit splitting
> is the step under weight. **If it is bounded to closure representation, this
> whole paragraph falls and the boundary is unregulated** — the negative answer
> above survives either way.

> # RETIRING ALL FIVE RESIDUAL CLASSES IS NOT THE FINISH LINE
>
> With every class retired, the selector still exists, still evaluates on every
> compilation, and the `RecursiveDescent` emission lane is still compiled in —
> **dead**. **That residue IS the tech debt the directive names.** So this is
> a required node, not a tidy-up, and it is the node that actually banks the
> efficiency win.
>
> **Done is:** `select_body_emission_authority`, `RecursiveDescentResidual`,
> `declaration_recursive_descent_residual`, `recursive_descent_residual`,
> `BodyEmissionAuthority::RecursiveDescent` and the recursive-descent emission
> lane are **deleted**, and every program compiles through `FunctionizedUnits`.

## Why it is its own node and not a coda on the last migration

**Because a deletion this wide has a different risk profile than a port**, and
folding it into [[RT-RECURSOR-TRANSPORT]] would let "the last class is retired"
be reported as "the lane is gone." Those are different claims, and only the
second is the directive.

The lane's surface at `origin/main = 14c3c5f7` spans **five production files** —
`lowering/core.rs`, `lowering/mod.rs`, `planning/static_transition.rs`,
`object_linker_packaging.rs`, and the `core/tests/` control modules. A
deletion that misses a file leaves a dead branch that still compiles.

## The dead-code oracle is spent by the commit that clears it

Once the last residual class is retired, **nothing in the tree can any longer
distinguish "the lane is unreachable" from "the lane was deleted."** The
evidence that the lane is dead exists only *before* this node lands.

⇒ **`D1` captures that evidence first**, while it is still capturable, and the
acceptance criteria are written against it. Do not start deleting and then try
to prove the lane was dead.

## Sequencing

**Last** in the campaign, gated on the four migration nodes **and on
[[RT-FNUNIT-RESULT-TOKEN]]**. This is the only node here whose `depends_on`
list is a genuine mechanism dependency rather than file contention — it cannot
land until every class is retired.

**The fifth edge is a different kind of dependency and was added 2026-08-08
by the Steward** (sequencing call; the node was filed that morning, after this
list and the campaign DAG were written). The four migration edges say *the lane
is no longer selected*. `RT-FNUNIT-RESULT-TOKEN` says *the lane is no longer
needed* — it owns `nc22`, currently the only program exercising a shape that
**only the `RecursiveDescent` lane supports**.

**Landing this node first would silently narrow what Ken can compile, and
nothing would fail.** `nc22` is `#[ignore]`d under that node's own quarantine,
so the one witness is already suppressed; deleting the lane under a skipped row
retires the fallback and the detector together. Un-skipping `nc22` green on the
functionized lane is that node's closure condition, and it is this node's
release gate.

## THE FRAME IS WRITTEN

`docs/program/wp/RT-DESCENT-RETIRE.md`. Campaign context, the binding traps that
bind every node in this arc, and the full schedule:
`docs/program/16-recursive-descent-retirement.md` — **read it before the frame.**
