---
id: RT-DESCENT-RETIRE
title: "Retire RecursiveDescent — delete the migration selector, the residual enum, the authority variant, and the recursive-descent emission lane"
status: draft
owner: runtime
size: M
gate: none
depends_on: [RT-DECL-CLOSURE-PORT, RT-SEED-CALL-PORT, RT-PRODUCER-MATCH-PORT, RT-RECURSOR-TRANSPORT, RT-FNUNIT-RESULT-TOKEN, RT-LEXICAL-RECURSOR-CONSUMERS, RT-CLOSURE-CROSSING-ELIMINATE, RT-LEXICAL-CALL-ARG-WITNESS-OR-PORT, RT-MATCH-SCRUTINEE-DISPOSITION]
blocks: []
github: null
origin: Operator directive 2026-07-29 — "we should not let it linger in a half-migrated state. That just carries tech debt for no benefit." Campaign docs/program/16-recursive-descent-retirement.md. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> # 2026-08-16 — MEASUREMENTS `A`/`B` RETURNED. BOTH VARIANTS ARE NOW FILED, AND
> # THIS NODE IS BLOCKED BEHIND THEM. Steward.
>
> **The two read-only measurements the Architect required are complete**
> (runtime-implementer `evt_4v0frfza70d2m`, runtime-leader `evt_1d5wb0t98jadx`),
> at exact `3523868afe7cd84b47c7b07281ff7df7c3202d61` — a base whose complete
> `crates/ken-runtime` tree is identical to `dc98f6f84` at
> `17246cb8615e04fd520d646eed60079ea28d06f0`, so the result is current. Suite
> 941 passed / 0 failed / 4 ignored; probe reverted; `core.rs` blob preserved.
>
> **`A`: the original `D1` probe DID read the short-circuiting selector — the
> hazard was real — and the 27/4 split survives it anyway.** The probe printed
> after `select_body_emission_authority` returned, and production selection
> (`core.rs:2409-2415`) short-circuits. The re-read against
> `enumerate_recursive_descent_residuals` found **zero dual-retained
> renderings**: removing `L` leaves exactly the three `M` renderings, removing
> `M` leaves exactly the twelve `L` renderings. **So 27/4 is a true partition and
> 27 is not a lower bound. The population did not move.**
>
> **Do not read that as vindicating the selector.** It was the wrong instrument
> and the number happened to be tight. **The set-valued observation is the
> established currency**, and `d3_the_exact_set_control_still_reds_under_short_circuiting`
> (`lowering/core/tests/control.rs:16959`) exists to keep it that way. A future
> measurement reaching for the selector repeats the defect whatever this one's
> outcome was.
>
> **`B`: all fifteen renderings are SINGLY retained**, so each exclusion result
> is **sole-retainer evidence** — closing the silent blindness in the probe's
> `debug_assert`, which checked only that the excluded variant was *present*.
> **Sole-retention is necessary and NOT sufficient**: an exclusion returning
> `FunctionizedUnits` says the classifier no longer retains the program, not that
> the lane can emit it. **It is not capability evidence.**
>
> ## The two nodes, filed. This is also the `#6d` gate repair.
>
> | node | variant | population |
> |---|---|---|
> | [[RT-LEXICAL-CALL-ARG-WITNESS-OR-PORT]] | `LexicalCallArgumentRecursor` | 12 renderings, 27 compiles |
> | [[RT-MATCH-SCRUTINEE-DISPOSITION]] | `MatchScrutineeRecursor` | 3 renderings, 4 compiles |
>
> **Both are now in this node's `depends_on`, and that edge IS the repair of the
> under-specified gate.** The retirement previously gated on `#6d` closure, which
> is a **rows-1-5** claim, to retire a **variant**. **It now gates on the
> variant's measured population, named by hash in the two frames.** The edge is
> on `depends_on` rather than only on the children's `blocks` because
> `scripts/gen-progress.sh` reads `depends_on` and nothing else.
>
> **Status flipped `active` → `draft`, and `draft` here means BLOCKED, not
> unframed.** The frame is complete, was released once, and its section 7 hard
> stop did its job. Nothing was lost and no work was withdrawn: `D1` delivered
> and `D2`-`D8` were correctly never entered. **`draft` is the only status the
> schema has for a framed node whose dependencies have not landed** — `ready`
> makes `check-issue-schema` warn that *"a team pulling this node will find its
> premise false"*, which is exactly right, and leaving it `active` would
> advertise a ring working this node while the ring works its children. **It
> returns to `active` when both children merge.**
>
> ## `LexicalCallArgumentRecursor` disposition — measured, no port owed
>
> [[RT-LEXICAL-CALL-ARG-WITNESS-OR-PORT]] `D1` measured all twelve fixed
> lexical renderings as fixture-only under the current production source path:
> **zero of twelve renderings and zero of 27 measured compiles are
> source-reachable**. Kernel definition admission rejects the required
> immediate `Application(Lambda, ...)` before native preparation or Runtime
> lowering. This is a **measured conclusion, not a capability decision**: there
> is no source-reachable population for a lexical port to serve, so no port is
> owed and `LexicalCallArgumentRecursor` is re-described as fixture-only with
> the retained lane for this retirement.
>
> The result is conditional on the measured gate. The rejection is a current
> kernel incompleteness: a future completeness change that admits or normalizes
> the ascribed `App(Lam, ...)` shape must re-open the twelve-row disposition
> before this retirement gate can rely on it. This record does not retire the
> lane itself, and it says nothing about `MatchScrutineeRecursor`; that variant
> remains governed only by [[RT-MATCH-SCRUTINEE-DISPOSITION]].

> # 2026-08-16 — `D1` HARD-STOPPED. THE LANE IS NOT DEAD. DO NOT DELETE ANYTHING.
>
> **`D1` fired section 7's trigger at exact `5eb05d6b2`, and that is the correct
> outcome, not a setback.** Over the serial complete `ken-runtime --lib` corpus
> — 941 passed, 0 failed, 4 ignored; 801 compilation entries; 103 non-empty
> full-residual enumerations — **production selected
> `BodyEmissionAuthority::RecursiveDescent` 31 times across 20 tests**: 27
> `LexicalCallArgumentRecursor`, 4 `MatchScrutineeRecursor`. Probe reverted,
> branch range empty, `core.rs` byte-identical, worktree clean.
> **`D2`-`D8` were correctly not entered.** runtime-leader `evt_511w8d94qcp5w`,
> attribution `evt_2vhjfm8ds954f`.
>
> **The certified `FunctionizedUnits` refusals were NOT miscounted as firings.**
> The 72 non-empty rows selecting `FunctionizedUnits` were excluded. Section 7's
> certified-refusal-is-not-a-residual-firing paragraph is what made that
> separable; do not remove it.
>
> ## Architect ruling `evt_620806vfy5kwm` — the disposition is PER-VARIANT
>
> **A selection is not a failure and not a refusal. It is a positive
> classification carrying a typed reason**, so the disposition axis is the
> `RecursiveDescentResidual` variant, not the expression. **Fifteen exact
> renderings cannot carry fifteen dispositions; they carry at most two**, because
> the live enum has exactly two members left. All coordinates below were
> re-verified against the tree by the Steward.
>
> - **`LexicalCallArgumentRecursor` is an INCOMPLETE PORT. Capability is owed,
>   and the variant's own doc says so** (`lowering/core.rs:2005-2011`): *"The
>   recursive result still carries invocation-local scope/return-hole state.
>   Passing it through a separately declared lexical unit is **not one of the
>   completed functionized ports**, so the established recursive descent lane
>   retains the whole call."* That is a statement that the port was never built —
>   not a conservation law and not an invariant.
> - **`MatchScrutineeRecursor` is UNMEASURED. Symmetry is not an argument for
>   it.** Its entire doc is one line (`core.rs:2003`) — *"An ordinary match
>   consuming an active computational recursor."* **No reason is recorded.**
>   Nothing in the tree says whether it is an unbuilt port or a shape the
>   functionized lane would correctly refuse. **Do not rule it by analogy to its
>   neighbour**; each of the four retired variants got a node that built the port
>   first.
>
> **`evt_5h7vzc27mc11j` DOES NOT CARRY HERE, and the reason is stronger than
> selections-versus-refusals.** That ruling was about `FunctionizedUnits`
> refusals grounded in conservation laws, an invariant, a semantic impossibility
> and a structural absence — correct, over-strict, nothing owed. **This is the
> retained lane holding programs because its replacement was never written.
> Opposite disposition.** Applying the earlier ruling here would conclude "no
> capability owed" about the one class where the tree explicitly says it is.
>
> ## The cut: ONE node keyed on the variant, triage FIRST
>
> **Group by residual variant, not by provenance.** All twelve distinct lexical
> expressions enumerate the same variant, so they need the same port: one port,
> one node. Provenance records which census saw them; it does not predict the
> mechanism.
>
> **The homogeneity premise is already false, so the node's FIRST deliverable is
> a per-expression fixture-or-production triage.**
> `rt_lexical_call_argument_recursor_executable()` is `#[cfg(test)]` at
> `lowering/core/tests/control.rs:16166-16167`, doc-commented *"`D1` position B"*
> — **a fixture authored to occupy the position, not a production-reachable
> shape. No production capability is owed for it.** The other three
> out-of-population expressions — ordinary-frame aggregate, dynamic-effect
> host-result, the two `D8` revisit-with-join compiles — **have not been
> opened**, and one fixture out of four does not make the other three fixtures.
> **A port sized against twelve expressions when some are fixtures builds
> capability for a population that cannot arise** — the thing ruled out hours
> earlier.
>
> ## `#6d` IS NOT REFUTED. The defect is the GATE that consumed it.
>
> **Its population statement was accurate about the scope it named.** Campaign
> §4 item 6d says *"rows 1-5 **only**, eight expressions across five test
> families"* — a deliberate narrowing, recorded at
> `RT-LEXICAL-RECURSOR-CONSUMERS.md:11` as *"narrowed to rows 1-5 by the re-rule
> `evt_3r4j14fv1jtj2` on the **nine-expression** census"*. **A nine-expression
> census was known and deliberately cut to eight. `#6d` never claimed to be the
> variant's population and nobody was misled.**
>
> ⇒ **It is a scoped claim used by a consumer as though it were variant-wide, and
> that consumer is the retirement gate** — `#6d` closure was made the
> precondition for retiring `LexicalCallArgumentRecursor`, which requires the
> variant's population, not rows 1-5's.
>
> **This IS the same defect as `RT-RECURSOR-TRANSPORT` `D3`, and the Architect
> affirmed the regularity deliberately after refusing a false one the same
> morning.** `D3` named two executable positions when the governed population was
> broader; `#6d` names rows 1-5 when the variant is broader. **Both are
> under-specified gates with the same repair — name the population the gate
> governs.** That is still a different defect from #2442's unsatisfiable
> condition, whose repair is deletion. **The test is whether the repairs
> coincide, not whether the symptoms rhyme.**
>
> **Sweep bound, and it is a documentary check on gates rather than a
> re-measurement of corpora:** *did a closed node commit a scoped population, and
> does any live gate cite its closure as if it were complete?* No method node.
>
> ## The comparison baseline was stale and under-counts by two
>
> **`#6d`'s live population is SIX, not eight.**
> `RT-LEXICAL-R3-FUSION-EMITTER.md:228`: *"`#6d`'s population drops from eight
> expressions to **six**: rows 1, 3, 4, and row 5's after-hole expression. Row 2
> is [[RT-LEXICAL-ROW2-MISSING-MINT]]'s. Row 5's before-hole is this node's."*
>
> **This does not change the no-live-owner finding** — all four of
> `RT-LEXICAL-RECURSOR-CONSUMERS`, `RT-MATCH-RECURSOR-CONSUMERS`,
> `RT-LEXICAL-R3-FUSION-EMITTER` and `RT-LEXICAL-ROW2-MISSING-MINT` are
> `merged`, verified. **It changes which node's claim each occurrence tests, so
> the new node's frame carries six and names the two spun-out cells
> separately.**
>
> ## TWO MEASUREMENTS BEFORE ANY CAPABILITY WORK IS SCOPED. runtime-leader owns both.
>
> **A. Which function did the `D1` probe read? This can invalidate the 27/4
> split.** `recursive_descent_residual` (`core.rs:2079`) **short-circuits** —
> every arm is `find_map`/`or_else`. In the `Match` arm `MatchScrutineeRecursor`
> is tested **first** and `.or_else`s the rest, so **a program retained by both
> variants under a `Match` reports only `MatchScrutineeRecursor`. Lexical
> retention is masked ⇒ 27 is a LOWER BOUND and 27/4 is not a partition** — if
> the probe read the selector's answer. The non-short-circuiting twin is
> `enumerate_recursive_descent_residuals` (`core.rs:2180`), surfaced as
> `observed_recursive_descent_residuals() -> Option<BTreeSet<...>>`
> (`core.rs:1134`, `#[cfg(test)]`). **The set-valued instrument is the
> established currency** — `#6d` stated its own population in set language.
> The tree already names this hazard:
> `ResidualEnumerationMutation::ShortCircuitLikeTheSelector` (`core.rs:570`,
> injected at `core.rs:2189`), commented *"this is the regression the instrument
> exists to prevent, injected at the instrument itself rather than at a
> convenient downstream point, so a control that stays green under it is
> measuring something else."*
>
> **B. Retention multiplicity of each of the 15.** The `D1` exclusion probe
> returns `FunctionizedUnits` only when the residual set is **empty after
> removing the excluded variant**. For a doubly-retained program, removing one
> leaves the other, so it stays on the retained lane and **the probe observes
> nothing about the functionized lane's capability for it.** Its `debug_assert`
> checks only that the excluded variant **was present**, not that it was the sole
> retainer — **so the blindness is silent.** Until multiplicity is known, an
> exclusion result is not capability evidence for any program that is not singly
> retained.
>
> ## Explicitly NOT ruled
>
> **Whether the port is worth building.** That is scope — Steward's and the
> operator's — and it is decided **after** `A` says whether the population is
> twelve expressions or more, and after the triage says how many are
> production-reachable at all. **No `crates/` work is authorized**; `A` and `B`
> are read-only against existing `#[cfg(test)]` instruments. **The Steward files
> the node once `A` returns, because `A` can move the population.**

> # 2026-08-16 — THIS NODE NOW ABSORBS THE TWO RESIDUAL VARIANTS AND THE
> # CONTROL RE-DESCRIPTION. `RT-RECURSOR-TRANSPORT` IS CLOSED. Steward.
>
> **`RT-RECURSOR-TRANSPORT` merged at PR #2443 and is `merged`.** Its contract
> resolved as follows, and the wording matters because a future reader will
> otherwise mis-read the retirement as blocked:
>
> - **`D0`/`D1` — delivered, a measured negative.** A disposable retirement at
>   `3f95967b8` produced **920 passed / 12 failed / 4 ignored**, reverted. The
>   landed continuation machinery does **not** close either residual class for
>   free.
> - **`D2` — DISCHARGED ON A MEASURED EMPTY PREMISE.** Its text is *"only for a
>   class that does not close for free, add the narrow consumer-port authority
>   its failure proves necessary."* **No class in the governed population
>   requires one** — every failure examined is a correct refusal by a
>   conservation law, an invariant, a semantic impossibility or a structural
>   absence. *"Only for X, do Y"* with no X is satisfied by doing nothing.
>   **This is discharged, not voided** — we asked and the answer was zero, which
>   is the result `D0`/`D1` paid for. An empty-premise obligation cannot be
>   *failed*, so it is **restated**, never marked confirmed.
> - **`D3` — ITS GATE IS MET, NOT UNSATISFIABLE.** The condition is *"only after
>   both executable positions are green."* **The record says both passed**, as
>   did the propagation-disable negative and corrected row 2. **The retirement
>   was stopped by the broader governed population, which `D3`'s gate never
>   named** — so `D3` was **under-specified**, not impossible.
>
> > **Do not file `D3` as another instance of the unsatisfiable-gate defect
> > corrected in #2442.** That one named a condition nothing could satisfy and
> > its repair is *delete the gate*. **This one named two witnesses when the
> > governed population was larger, and its repair is *name the full
> > population*.** Recording them as one pattern would put a false regularity in
> > the corpus and teach the wrong repair. (Architect `evt_13fw3q7j0jma0`.)
>
> ⇒ **THE PATH IS OPEN.** The residual blocker is dispositioned as a control
> **re-description**, and this node owns it.
>
> ## What this node must now do, beyond deleting the lane
>
> **Re-describe the controls for the two established categories** — the
> `StaticWorkerBinding` conservation group and row 1 owned-scope. Architect
> ruling `evt_5h7vzc27mc11j`: **do not repair, do not retire.**
>
> - **Repair is foreclosed and not on cost grounds.** None of these is a
>   capability gap. "Repairing" would mean weakening a conservation law or a
>   planner invariant so it admits inputs violating its own premise — growing
>   the TCB and removing a check in one motion, for a population no source
>   program produces. Row 1's refusal **is** the invariant `RT-REFUSAL-SOURCE-
>   WITNESS-OR-INVARIANT` landed at `f39bdb9ad`; repairing it would undo a
>   disposition ratified the same day.
> - **The new expected values are already measured** — the reverted attempt's
>   per-category first outcomes, in `docs/program/wp/RT-RECURSOR-TRANSPORT.md`.
>   The re-description is specified by measurement, not assertion.
> - **Write the pin as unobserved-by-construction, not as rejected forever.** A
>   pinned refusal can freeze a design defect; if a later node decides one of
>   these shapes *should* lower, the pin must read as a fact about today's
>   reachable population rather than a commitment.
> - **State that it is an expectation change, not a repair**, so the next reader
>   does not read it as a regression papered over.
>
> **Two dispositions are OPEN and this node must settle them** — they were
> deliberately not selected by the record that measured them: **the two-sibling
> rows** (planner structures, one layer farther from source than the direct
> emitter rows) and **corrected row 2**. Do not inherit them silently.
>
> **`d8d` is outside all of this.** The functionized binding site observing two
> installs against the retained lane's zero is a **count divergence, not a
> refusal**. It has a different owner and must not be partitioned into a refusal
> bucket.

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
