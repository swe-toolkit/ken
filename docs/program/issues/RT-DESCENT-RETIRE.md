---
id: RT-DESCENT-RETIRE
title: "Retire RecursiveDescent — delete the migration selector, the residual enum, the authority variant, and the recursive-descent emission lane"
status: active
owner: runtime
size: M
gate: none
depends_on: [RT-DECL-CLOSURE-PORT, RT-SEED-CALL-PORT, RT-PRODUCER-MATCH-PORT, RT-RECURSOR-TRANSPORT, RT-FNUNIT-RESULT-TOKEN, RT-LEXICAL-RECURSOR-CONSUMERS, RT-CLOSURE-CROSSING-ELIMINATE, RT-LEXICAL-CALL-ARG-WITNESS-OR-PORT, RT-MATCH-SCRUTINEE-DISPOSITION, RT-MATCH-DIFFERENCE-REACHABILITY, RT-DESCENT-RETIRE-PRIOR-ART, RT-MATCH-SCRUTINEE-PORT, RT-DESCENT-LANE-COMPLETENESS, RT-REFUSAL-PINS-REHOMED, RT-REFUSAL-PIN-ABSENCE-CLAUSE]
blocks: []
github: null
origin: Operator directive 2026-07-29 — "we should not let it linger in a half-migrated state. That just carries tech debt for no benefit." Campaign docs/program/16-recursive-descent-retirement.md. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> # 2026-08-17 — EVERY GATE IS SATISFIED. UNBLOCKED FOR THE FIRST TIME.
>
> **All fourteen `depends_on` entries are `merged` or `closed`**, checked field
> by field rather than inferred from the tracker. The last one was
> [[RT-REFUSAL-PINS-REHOMED]], merged at exact `d6a9760a9`, which discharged
> `AC-9`'s remaining half by pinning constructs 1 and 2 without the enum `D3`
> deletes.
>
> **What that means and what it does not.** The cost question is answered — the
> `D5` census found **zero `RecursiveDescent` selections across 60 selector
> arrivals** from real source — and the record obligation is discharged at both
> the correct-semantics and missing-port ends. **Nothing capability-shaped is
> known to block the deletion.** It does not mean `D1`'s hard stop is void:
> `D1` stopped because production still selected the lane, and **whether that is
> still true is a measurement at current `main`, not an inference from these
> flips.**
>
> ### ONE GATE WAS ADDED BACK THE SAME DAY. IT DOES NOT STOP THE `D2c` INCREMENT.
>
> **[[RT-REFUSAL-PIN-ABSENCE-CLAUSE]] joined `depends_on` after the Adversary
> hunt** (`evt_3t7y5zwng8aba`) found that pin 2 of the re-homed pins asserts two
> `contains` clauses — **what the refusal must say, and nothing about what it
> must not say** — so the temporal phrasing `D1c` refuted can be re-added beside
> both pinned clauses with every assertion still green.
>
> **It gates the DELETION, not the reroute.** `D2c` routes to nothing and
> **deletes nothing**, so the pin's strength is not load-bearing for it. **The
> in-flight `D2c` increment proceeds unchanged** — do not read this entry as a
> stop.
>
> ⇒ **What it gates is `D3`**, which deletes the enum and leaves pin 2 as the
> only assertion of that refusal's text anywhere. **A pin that survives the
> deletion but cannot detect the regression it was written for survives in name
> only.**

> ### THE HELD `D2c` CANDIDATE IS NOT THE `D2c` LANDING. Cut a fresh one.
>
> **`036e8ee91` stays frozen at base `c98f72ba8` — it is EVIDENCE, and rebasing
> it destroys the base its numbers are stated against.** The frame's *"do not
> rebase it"* protected exactly that.
>
> ⇒ **The reroute lands as a NEW candidate cut at current `main`.** That keeps
> the single-commit-revert property the frame requires of `D2c`, keeps the
> evidence artifact intact, and rebases nothing. **These were never in tension —
> the held object's job is to be read, not to be merged.**
>
> **Read `D2c`'s numbers against `c98f72ba8` and never against `main`**; the
> suite population has moved in both directions since. Measured, with the
> vocabulary half of that concern refuted, in the frame at `D2c`.

> # 2026-08-16 — `D1` IS IN: TWO CORRECT SEMANTICS, TWO MISSING PORTS.
> # The functionized lane is NOT a complete replacement. One question remains.
>
> **Architect `evt_5cxzxp4b6q31v`.** The four constructs **do not answer alike.**
> The discriminator was **denotation versus the compiler's own bookkeeping**, not
> the severity of the message.
>
> | # | construct | verdict |
> |---|---|---|
> | 1 | `ComputationalMatch` in-flight activation (4) | **CORRECT SEMANTICS** — control state live only for that activation |
> | 2 | `StaticWorkerBinding` (2) | **CORRECT SEMANTICS** — same law, second callable kind. **Porting it would give closures a durable lane: a semantic change to Ken, not a port.** |
> | 3 | Backend `Module`, no recursive-position-1 worker (2) | **MISSING PORT** ⇒ [[RT-FNUNIT-MULTI-WORKER-CONTINUATION]] |
> | 4 | `PlannerInvariant`, no checked-root authority (1) | **MISSING PORT** ⇒ [[RT-FNUNIT-CHECKED-ROOT-AUTHORITY-ROUTING]] |
>
> **`D6` MUST RE-HOME THE REFUSAL PIN, NOT CITE IT.**
> `d2k_0_the_five_no_longer_reach_a_static_worker_value_read` pins constructs 1
> and 2 **through `set_selector_variant_exclusion(Some(RecursiveDescentResidual::…))`
> — and `D3` DELETES `RecursiveDescentResidual`.** The pin rides the mechanism
> this node removes. **`nc22` reasoning, third occurrence in one node: check what
> a pin is built ON, not just what it asserts.**
>
> **The re-home is now its own node: [[RT-REFUSAL-PINS-REHOMED]], named in this
> node's `depends_on`.** [[RT-DESCENT-LANE-COMPLETENESS]] closed 2026-08-17
> having delivered every verdict; **its closure discharged constructs 3 and 4's
> obligations and NOT constructs 1 and 2's pin.**
>
> #### BEFORE RUNNING `D6`: A `closed` PIN NODE IS NOT AUTOMATICALLY A DISCHARGE.
>
> **`closed` satisfies a `depends_on`** — `scripts/check-issue-schema.sh:189`,
> *"closed means resolved-without-landing."* [[RT-REFUSAL-PINS-REHOMED]]'s own
> `AC-8` makes a **hard stop a legitimate outcome**: it may resolve by reporting
> that no exclusion-free assertion exists, land no pin, and still close.
>
> ⇒ **In that case this node's dependency reads satisfied while constructs 1 and
> 2 are unpinned — the exact defect `AC-9` exists to prevent, arriving through
> the dependency mechanism instead of through an increment.** Do not read the
> gate; **read whether a pin landed.** If none did, `D6` proceeds only on an
> explicit Steward-and-Architect ruling that it may run unpinned.
>
> **STILL OPEN — successor `D5`, routed to the Architect.** `0/12` does **not**
> bound construct 3: that measurement was over the twelve
> `LexicalCallArgumentRecursor` renderings, and construct 3's mechanism is a
> different shape — a `ComputationalMatch` case with **two recursive positions**,
> i.e. a binary-tree fold. **Whether any source-admissible program has one was
> never asked, and it is the one input that could still flip this node from
> recorded-gap to BLOCKED.**
>
> ---
>
> # PRIOR: NO LONGER CAPABILITY-BLOCKED. THE RECORD IS THE GATE.
>
> **Architect `evt_3bkkjpps1bcpe` supersedes the BLOCKED disposition below.**
> All nine refusing programs map to five hash-tagged lexical fixture renderings,
> **all within the twelve, ZERO source-reachable** — hashes independently
> re-checked against the merged node's own table. ⇒ **The retirement loses no
> user-facing capability. `D2` settles COST, not CORRECTNESS.**
>
> **`D3` is NEGATIVE:** no merged completeness claim is falsified, no erratum,
> ownership does not move. The Architect spot-checked the largest exposure
> itself *"because this campaign has form"* — and found it **records refusals as
> refusals.**
>
> **What now gates `D3`-`D8` is not capability. It is the RECORD.** After
> deletion the lane is gone and these tests are retired or rewritten, so **the
> record is all that remains of four known representability facts** — and *"a gap
> the lane must someday close"* is the **wrong** record for a refusal that is
> correct semantics. ⇒ **`D1` precedes the RECORD, and each construct leaves a
> PIN, not prose** (successor `AC-9`). **This is the `nc22` reasoning applied
> forward: do not retire a fact and its only detector in the same commit.**
>
> **The helper-evidence defect is NOT this node's** — it is live on `main` and
> cut separately as [[RT-TRACE-HELPER-ABORTED-COMPILE-EVIDENCE]].
>
> ---
>
> # SUPERSEDED: THE RETIREMENT IS BLOCKED, `D2c` FOUND A REAL CAPABILITY LOSS.
>
> **Architect `evt_35hwm50tas8kp`.** `D2c`'s sentinel failed on **assertion 1
> verbatim** — `must retain its compiling RecursiveDescent baseline` — which
> **precedes** the sentinel assertion, so the discrimination is complete.
>
> **`RecursiveDescent` compiled row 4 depth 2 at base; the functionized lane
> REFUSES it at `StaticWorkerBinding`. Same program, two worlds, different
> behaviour.** ⇒ **A regression, differentially established on a REAL PRODUCTION
> COMPILE** rather than on hand-built IR with no preimage. **That is exactly the
> population `D2c` existed to reach, and it found one on the first required
> row.**
>
> **No `D6` re-home is lawful. `D3`-`D8` stay gated. `D2c` stays UNPUBLISHED.**
>
> > ### THE STAGING IS WHY THIS COST NOTHING. `AC-7` earned its keep here.
> >
> > `D2c` is **one revertible commit that never landed** — nothing to undo, no
> > evidence destroyed, and the blocker arrived as a **concrete failing program
> > instead of an argument.** A single candidate that rerouted and deleted
> > together could not have told this regression from a compile error.
>
> ### THE QUESTION IS NOT ONE CONSTRUCT. IT IS FOUR, AND THAT CHANGED THE NODE.
>
> **Superseded 2026-08-16 by Architect `evt_7qtgrtwv76vke`, on the ring's
> construct inventory `evt_6bvnv6t4teech`.** This block previously stated the
> successor's question as the single `StaticWorkerBinding` fork below. **The
> inventory REFUTED the one-mechanism hypothesis the Architect had offered.**
>
> **Nine of the fourteen in-set reds are the surviving lane refusing a program
> the retiring lane compiles, across FOUR independent constructs** —
> `ComputationalMatch` / in-flight non-transferable activation (4),
> `StaticWorkerBinding` (2), backend `Module` / missing recursive-position-1
> worker projection (2), backend `PlannerInvariant` / missing affine
> checked-root authority (1).
>
> ⇒ **Four separate representability gaps is a PATTERN, not an omission. The
> successor is not a missing port — it is a LANE-COMPLETENESS question:** is the
> functionized lane a complete replacement for `RecursiveDescent`, or has it
> been carrying only the ported subset? **Framed any narrower it gets scoped as
> one port and comes back.**
>
> **The fork below still governs, now per construct rather than for the node:
> FOUR verdicts, and they may not answer alike.** A principled representability
> refusal and an unported case can sit side by side. It remains a **soundness
> question and it remains the Architect's** — it is `D1` of
> [[RT-DESCENT-LANE-COMPLETENESS]], **not decided by the ring as engineering.**
>
> | answer, per construct | consequence |
> |---|---|
> | **correct semantics** | `RecursiveDescent` was compiling a shape with **no runtime denotation**. Retirement REMOVES that hole; the gap is recorded, nothing is owed. |
> | **missing port** | The functionized lane owes that case and **the retirement waits on it.** |
>
> **Whether this node is BLOCKED or merely incurs RECORDED GAPS is decided by
> the successor's `D2`** — the source-reachability of the nine. `0/12` was
> measured over **renderings**; these are **test names**, and the mapping is
> established for the sentinel alone. **Do not inherit that number here.**
>
> **Two shortcuts are foreclosed** (Architect, who noted being the seat that
> benefits from the first and checked the rendering identity rather than
> reasoning about it):
>
> - **"It is fixture-only, so it does not count."** Row 4 depth 2 **is** one of
>   the twelve — rendering 5, hash `de31e8ed184a5754`, `{L}`, `#6d` live. So
>   `0/12` holds and **no user program reaches this shape today.** **That bounds
>   the BLAST RADIUS, not the MECHANISM.** The refusal is general — *"a
>   constructor carrying an unconsumed static worker denotes a value containing
>   the callable and has no runtime representation"* — a statement about **the
>   functionized lane**, not about one fixture. **The fixture is how it was
>   found, not the extent of what was found.**
> - **"`RecursiveDescent` compiled it, so port it."** Equally unproven and
>   **possibly backwards.** The refusal reads as **principled**: the shape
>   denotes a value containing the callable and has no runtime representation.
>   The functionized lane may be **correctly** refusing what `RecursiveDescent`
>   compiled only because its monolithic structure let the callable stay
>   implicit — on which reading the retirement **removes a latent
>   representability hole.**
>
> ### CLASSIFY THE OTHER 13 — DISCHARGED, and it REFUTED the sizing hypothesis.
>
> **Delivered `evt_6bvnv6t4teech`.** The prediction below was that three more
> `StaticWorkerBinding` refusals would mean one mechanism and one node. **It came
> back FOUR constructs across nine programs, plus five with no refusing construct
> at all** — so the successor is a lane-completeness node, not a port. **The
> instruction was worth running precisely because it could refute the guess that
> motivated it.** Original text, kept for the record:
>
> **Stop the `D6` engineering — no re-homing, retiring, or test edits. Do NOT
> stop reading the other 13 failure messages:** that run already happened and
> the output is in hand. **If three more are `StaticWorkerBinding` refusals it
> is ONE mechanism and one node; if they name different constructs the node is
> several.** Cutting the node on one blocker when the same completed run holds
> the answer for all fourteen buys a second round trip for nothing.
>
> ### ONE CORROBORATION — DISCHARGED. THE ARTIFACT HYPOTHESIS IS CLOSED.
>
> **Delivered `evt_6bvnv6t4teech`, disposable patch removed, tree clean.** At
> untouched base `c98f72ba8`, asserting the pre-existing excluded
> `FunctionizedUnits` result is `Ok` inside the sentinel **fails at row 4 depth 2
> with the IDENTICAL refusal** — same constructor origin 36, same static worker
> field 0, same origin 35, same recognition 2.
>
> ⇒ **Two independent instruments, one of which does not involve `D2c`'s edit at
> all. The finding is about the LANE, not the reroute.** That was the one way it
> could have been an artifact and it is now excluded. **The evidence therefore
> predates `D2c` entirely: the exclusion mechanism was a complete differential
> instrument the whole time, and the sentinel discarded its answer.** How far
> that shape spread is the successor's `D4`. Original instruction, for the record:
>
> **Assert `_excluded_result.is_ok()` at BASE `c98f72ba8`, disposable, green
> tree.** The test already had this answer and threw it away — its first leg
> captures `_excluded_result` at `control.rs:6499` and **discards it**, while
> `control.rs:1717` and `:2057` assert exactly that in the same file. **It
> departed from its own file's convention at precisely the point where the
> evidence was**, which is why its capability claim is `CLAIMED` rather than
> measured.
>
> **Why this and not the single reading:** `D2`'s positive control validated the
> **enumerator**. **Nothing yet validates that `D2c`'s always-`FunctionizedUnits`
> edit is behaviour-equivalent to the pre-existing exclusion mechanism.** The
> base-SHA probe closes that on a tree where nothing was rerouted. **If the two
> disagree, the finding is about `D2c`'s EDIT and not about the lane — and that
> is a different node entirely.**
>
> ---
>
> # PRIOR: 2026-08-16 — RELEASED, TWELVE OF TWELVE DISCHARGED, BAR LIFTED.
>
> **Do the retirement in TWO STEPS and do not collapse them** — `D2c` reroutes
> without deleting and lets the whole corpus in CI be the differential; `D3`
> through `D8` delete only after it is green. Frame:
> `docs/program/wp/RT-DESCENT-RETIRE.md`. **The lift, the evidence and the
> reason the staging exists are in the bar section below — read it before the
> frame.**
>
> **The release gate is satisfied:** [[RT-FNUNIT-RESULT-TOKEN]] is `merged` and
> no `nc22` test carries `#[ignore]`, so the one witness for the shape only this
> lane supported is live rather than suppressed. **Verified by the Steward at
> release, not inherited from the node's status.**
>
> ## `D1` RAN AND FIRED THE HARD STOP: 28 LIVE SELECTIONS. RULED THROUGH.
>
> **At exact `97b963ac4`, `D1` found 28 selector arrivals choosing
> `RecursiveDescent` — 27 lexical-call-argument, 1 match-scrutinee, across 18
> named tests** (runtime-leader `evt_10v6y6m8jq49a`). Section 7's carve-out
> *"if `D1` does surface a program that selects the authority, the hard stop
> applies in full"* is literal, post-dates both dispositions, and fired.
>
> **Architect ruled against it on substance, `evt_98zg6sbqh7ej`: `D2c` PROCEEDS.
> No node is cut ahead of it. The stop is RE-AIMED at deletion, not retired.**
> Its live form is now *"a red from a program outside the pinned set."*
>
> **The decisive leg is not staleness — it is that the stop already fired on
> this exact population and its remedy was already performed.**
> [[RT-LEXICAL-CALL-ARG-WITNESS-OR-PORT]] exists **because** an earlier `D1`
> found 31 selections across 20 tests; the campaign named the class and handed
> it back, the node was built, and its ratified disposition says the variant
> **"retires when `RT-DESCENT-RETIRE` removes that lane."** ⇒ **Zero selections
> was never reachable before this node acts.** Firing again is double jeopardy.
>
> **The 18 test names and 28 sites are PINNED in the frame as `D2b`**, frozen
> before the `D2c` run so the expected-red set is falsifiable. `AC-8` and `AC-9`
> enforce it: **a set widened after seeing CI is a null oracle, and every red
> inside the set is adjudicated per test, never excused per set.**
>
> **Everything below this block is the historical record of how the bar rose and
> fell.** It is retained because the campaign paid for it; it is not live work.
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
> established currency**, and
> `d3_the_exact_set_control_still_reds_under_short_circuiting`
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
> advertise a ring working this node while the ring works its children.
>
> **The sentence that used to sit here — *"it returns to `active` when both
> children merge"* — IS NOW FALSE, and it was removed rather than left to
> mislead.** Both children merged (PRs #2454 and #2458) and **all nine original
> `depends_on` entries are `merged`**, yet this node is still barred: the
> narrowing left `MatchScrutineeRecursor` load-bearing on a difference whose
> source-reachability is unmeasured. **A graph reading "every dependency landed"
> would have advertised this capstone as dispatchable while its own text forbids
> it to delete anything.** [[RT-MATCH-DIFFERENCE-REACHABILITY]] was filed as the
> tenth dependency to carry that discharge. **It merged on 2026-08-16 having
> settled the question in the one direction that lifts nothing** — outcome 3, not
> settled under the method gate — so the discharge passed to the eleventh
> dependency, [[RT-DESCENT-RETIRE-PRIOR-ART]]. **That referral merged the same
> evening and did not lift the bar either** — it closed the unreachability route
> from outside Ken and returned conditional support. **The Architect then ruled
> the route (`evt_nb12nmhd2zzk`), and this node returns to `active` when
> [[RT-MATCH-SCRUTINEE-PORT]] supplies its evidence** — which, after that node's
> `D1` came back non-total, may be a **disposition** rather than a port. See the
> bar below.
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
> > **SUPERSEDED 2026-08-16 — the bullet above is the record of what was ruled
> > then, and its present tense is no longer true.**
> > [[RT-MATCH-SCRUTINEE-DISPOSITION]] `D1` measured the variant at exact
> > `f24ad5242`, and the answer is **neither** of the two outcomes that bullet
> > poses. All three governed renderings compile on the functionized lane, so the
> > port is **not** missing; and the retention rule is not a correct refusal
> > either. Architect, `evt_29rrwtbh48n8z`, naming a **third** class: *"a
> > retention guard whose condition is broader than the capability boundary it
> > was standing in for."* Retention is **existential** over per-case
> > `recursive_positions` (`lowering/core.rs:2104-2118`); the ordinary route is
> > **universal** over case bodies (`lowering/mod.rs:16688-16697`). **The two
> > cannot coincide by construction**, only contingently.
> >
> > ⇒ **THE RETIREMENT SHOULD STOP EXPECTING ONLY TWO ANSWERS.** A variant can
> > also be one whose guard was never load-bearing across its full extent —
> > *"build the port, then delete the variant"* is not the only shape a
> > retirement takes. **The lawful repair here may be a NARROWING, in which case
> > `MatchScrutineeRecursor` survives in reduced form** and this node's premise
> > of deleting the enum outright does not hold as written.
> >
> > **This marker is not the disposition.** The child's `D3` records the settled
> > outcome here, and it is the authority; this exists so a reader between now
> > and then is not told the variant is unmeasured.
>
> **`RT-MATCH-SCRUTINEE-DISPOSITION` `D2`/`D3-narrow` settles the gate:** a
> concrete backend-IR expression occupies the difference. Its immediate,
> non-empty `ComputationalMatch` has a recursive case whose scalar body fails
> `produces_deforestable_aggregate_with_ih`; the residual selector retains it,
> while the ordinary producer route declines it and reaches the exact
> `"scrutinee is not a constructor value"` refusal. Inverting the shared route
> operand makes the difference control red, so the control is sensitive to the
> routing contract rather than only to the syntactic shape.
>
> The bounded source attempt does **not** establish source unreachability. A
> natural nested recursive Ken program normalized before the runtime classifier
> and arrived with residuals `[]` and authority `FunctionizedUnits`; failed
> search is not the required grammar/admission/kernel argument. The
> pre-authorized narrowing therefore applies: `MatchScrutineeRecursor` is
> retained **if and only if** the ordinary producer route declines its immediate
> computational scrutinee. The three measured intersection renderings leave the
> residual set and still compile; the executable row remains
> `Returned(Int(Small(7)))`.
>
> **Retirement consequence:** `MatchScrutineeRecursor` survives in reduced,
> load-bearing form. This capstone may not delete the residual enum, selector,
> `RecursiveDescent` authority, or emission lane on the present evidence. `D2a`
> proves that the retained difference is non-empty as a backend-IR shape, so
> emptiness is not the discharge condition; `D2b` stopped short and made no
> source-unreachability claim. The bar lifts only through
> [[RT-MATCH-DIFFERENCE-REACHABILITY]]: a source-unreachability argument for the
> difference satisfying the method gate — surface grammar, elaborator admission,
> and kernel gates, not failed-attempt sampling — and accepted by a fresh
> Architect ruling. Until then the capstone is barred pending that named
> measurement, not permanently.
>
> **The named measurement returned outcome 3 on 2026-08-16: NOT SETTLED within
> the method gate.** [[RT-MATCH-DIFFERENCE-REACHABILITY]] established that
> normalization is not total: neutral eliminators survive kernel normalization,
> and an ascription admits the recursive-result spelling that the first
> inference-mode attempt refused. Present native erasure then wraps every such
> computational match in a `CheckedSubcontinuationFrame`, so the exact backend
> difference receives no source witness today. That wrapper is a current
> compiler-path invariant, not a surface grammar, elaborator-admission, or
> kernel refusal; generic non-plan erasure can emit the bare shape. The method
> gate therefore does not permit a source-unreachability claim. **This capstone
> remains barred.** No deletion, re-narrowing, or retirement re-scope follows;
> the Steward refers the measured rule gap to research under the operator's
> 2026-08-16 directive.
>
> **THE REFERRAL RETURNED THE SAME EVENING (PR #2467), AND IT CLOSED THE
> UNREACHABILITY ROUTE FOR GOOD.** [[RT-DESCENT-RETIRE-PRIOR-ART]] found that
> **Lean and Agda both admit the exact shape**, each with a passing regression
> case, so no argument from outside Ken supplies the negative either. **What it
> also found is that no surveyed system carries a second whole-function descent
> lane for this shape** — GHC lowers an expression-valued case scrutinee through
> the ordinary expression translator. **The objective survives; the argument for
> it changes from a proof about source to an enforced invariant about IR.**
>
> **ELEVEN OF ELEVEN DEPENDENCIES ARE NOW `merged` AND THIS NODE IS STILL
> BARRED. The current discharge is an Architect ruling, routed at
> `evt_v7w99zp81cqm`** — which argument family the retirement is built on (an
> enforced pre-classifier IR invariant, of the kind Lean's LCNF and Agda's
> Treeless maintain, versus GHC-style uniform lowering), and whether the
> advisory's statement of that invariant is right for Ken. **On the invariant
> route, deletion additionally waits on a demonstrated closure over every
> producer of the relevant IR** — Ken has the wrapper on the checked route and a
> generic non-plan escape, so it has the fact and not the invariant.
>
> **THE ARCHITECT RULED THE SAME EVENING (`evt_nb12nmhd2zzk`): build the
> retirement on UNIFORM LOWERING (family B), declining the advisory's global IR
> invariant as the primary route.** The decisive fact is on the Ken side and not
> in the prior art — **Ken's ordinary `Match` lowering already has the GHC
> shape** at `core.rs:17734-17756`, and the scrutinee form has a general arm in
> the same translator. **So family B is a totality question about code that
> exists, and family A would be a new permanent invariant over a public IR built
> to retire a selector the code itself calls temporary.** The invariant survives
> as a fallback only, boundary-scoped, on an uncloseable refusal plus a fresh
> ruling.
>
> ⇒ **[[RT-MATCH-SCRUTINEE-PORT]] is the twelfth dependency and the first link in
> this chain whose SUCCESS would license deletion** — every predecessor could
> only fail to. Its bar is **totality or a named refusal**, and a **silent**
> conservative fail-closed arm is barred.
>
> ### THE BAR IS LIFTED. `D1d` COMPLETED THE DIFFERENTIAL 5-OF-5 AND THE
> ### ARCHITECT DISCHARGED THE BLOCKER.
>
> **`D1c` (`b7f65ad0c`, scalar rows 1 and 3) and `D1d` (`77c52dd0a`, aggregate
> rows 2, 4 and 5) together measured production against excluded on every row of
> the `D1` quotient.**
>
> | rows | production leg | excluded leg |
> |---|---|---|
> | 1, 3 | `UnsupportedLowering { construct: "Match", reason: "scrutinee is not a constructor value" }` | **identical** |
> | 2, 4, 5 | `OK Returned(Int(Small(7)))` | **identical decoded value** |
>
> **Every row: pre-exclusion residual exactly `{MatchScrutineeRecursor}`, live
> `debug_assert!(was_present)` passing.** That probe is why the result means
> anything — *"both legs agree"* is vacuous if the exclusion never activated.
>
> ⇒ **Exclusion is behaviour-preserving across the measured population.** The
> aggregate rows carry more than the scalar ones: the excluded run **executed
> and produced the value**, which is capability evidence rather than merely
> classifier evidence.
>
> **Architect discharge, `evt_5f4jvs4f6pbdt`: not a port, `D2` correctly
> unentered — a zero differential leaves no mechanism to build — and the
> semantic question blocking this capstone is CLOSED.** Release is the
> Steward's call, not the Architect's; the ruling clears the blocker rather
> than scheduling the work.
>
> ### THE VARIANT IS ROUTING-LOAD-BEARING AND CAPABILITY-INERT.
>
> **The two nodes compose without contradiction — read them together.**
>
> | node | claim | result |
> |---|---|---|
> | [[RT-MATCH-SCRUTINEE-DISPOSITION]] `D2a` | **routing** — is the guard's difference from the producer route non-empty? | **non-empty** |
> | [[RT-MATCH-SCRUTINEE-PORT]] `D1c`+`D1d` | **capability** — does that difference cost observable behaviour? | **none, 5 of 5** |
>
> ⇒ Retiring the variant **changes routing for a non-empty set and changes
> observable behaviour for none of the measured population.** The caveat this
> node recorded above — *"an exclusion returning `FunctionizedUnits` says the
> classifier no longer retains the program, not that the lane can emit it; it is
> not capability evidence"* — **is exactly what the differential closed.** It
> stands as written about `D1`-era evidence and no longer bars deletion.
>
> ### WHAT FIVE ROWS CANNOT SETTLE: THE POPULATION. DO NOT PRETEND OTHERWISE.
>
> The differential covers **five hand-built `RuntimeExpr` with no kernel `Term`
> preimage.** The retirement acts on **every program the narrowed guard
> retains.** Those are not the same set, and treating a five-cell quotient as an
> enumeration repeats the predicate-reachability-is-not-population error this
> campaign has already paid for twice.
>
> **No census is owed. The retirement is STAGED so the corpus proves it** —
> `D2c` in the frame reroutes without deleting and puts the **whole corpus**
> through CI as the differential, and `D3`-`D8` delete only after it is green.
> **The two steps must not be collapsed:** one commit that reroutes and deletes
> cannot tell a routing regression from a compile error.
>
> ### THE ID STILL SAYS "PORT" AND THE ANSWER WAS NOT ONE.
>
> **An ID cannot carry a disjunction.** `RT-MATCH-SCRUTINEE-PORT` asserts
> "port" and travels un-qualified through `depends_on`, this bar, and every
> cross-reference — while the node it names concluded **no port is owed and no
> mechanism is built.** Renaming was declined (Architect `evt_3v1zp1g315vxz`:
> the churn exceeds the harm). **Read that node's title, never its ID.**
>
> **The earlier warrant here — *"the difference population compiles today via
> the retained lane, so a fail-closed arm is a capability regression"* — was
> WITHDRAWN, and the differential then refuted it outright.** The scalar rows do
> not compile under the retained lane either.
>
> **A measurement that comes back the other way does NOT close this node.**
> Operator, 2026-08-16: *"Prior art indicates that retiring `RecursiveDescent` is
> possible and observed resource usage by that implementation makes it
> desirable."* A witness, or an honest "cannot settle", **refers the matter to
> research for guidance** — it does not retire the objective and it does not
> trigger a re-scope. The Steward offered partial retirement as a fallback and
> the operator declined it. Full statement:
> `docs/program/16-recursive-descent-retirement.md`.
>
> **The discharge MOVED, 2026-08-16 (Steward), and the move is the point.** It
> was written as `RT-MATCH-SCRUTINEE-DISPOSITION` `D3-delete` while that node was
> live. **That node merged at PR #2458 without taking the delete branch**, so the
> bar would have cited an un-taken deliverable of a closed node — **no owner, no
> dispatchable increment, and a gate that reads as pending forever.** The
> successor is filed, `ready`, and in this node's `depends_on`, so
> `gen-progress.sh` shows the block.
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
