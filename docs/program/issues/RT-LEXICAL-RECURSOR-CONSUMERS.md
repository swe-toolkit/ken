---
id: RT-LEXICAL-RECURSOR-CONSUMERS
title: "Repair the LexicalCallArgumentRecursor consumer population on the functionized lane, activated by B-only exclusion before the retirement removes the seam"
status: active
owner: runtime
size: M
gate: none
depends_on: [RT-MATCH-RECURSOR-CONSUMERS, RT-LEXICAL-R3-FUSION-EMITTER, RT-CONTKEY-CONSUMING-OCCURRENCE, RT-CONTKEY-CONSUMER-DESCENT-CARRY, RT-CONTKEY-ROUTE-CLOSURE-PROBE, RT-REQUIRED-OCCURRENCE-PROJECTION]
blocks: [RT-RECURSOR-TRANSPORT]
github: null
origin: Architect ruling evt_5w09dcwbf7k70 (2026-08-08) on RT-RECURSOR-TRANSPORT hard stop 4, narrowed to rows 1-5 by the re-rule evt_3r4j14fv1jtj2 on the nine-expression census evt_16cmej481q7ns. Campaign docs/program/16-recursive-descent-retirement.md node #6d. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> # THIS NODE IS ROWS 1-5 ONLY. ROW 6 IS NOT ITS WORK.
>
> **Narrowed 2026-08-08 by Architect re-rule `evt_3r4j14fv1jtj2`**, on the
> measured census. An earlier revision of this file claimed all six red rows and
> asserted they shared `host_result_closure_match`. **Both claims are false and
> are withdrawn.**
>
> Row 6 (`d8d`) enumerates exactly `{MatchScrutineeRecursor}` — **it was never in
> this node's population.** It belongs to [[RT-MATCH-RECURSOR-CONSUMERS]].
>
> **Do not fold the two nodes back together.** The exact residual producer, the
> activation hook, the observed boundary and the completion owner all differ. If
> the two `D1` causal partitions later prove one exact shared production root,
> **route a subsumption proposal before coding** — it may not be inferred from
> shared retirement timing or shared syntax.

> # THE ROUTE WORK HAS LEFT THIS NODE. `D2k-1c` IS A WRONG CUT — DO NOT REOPEN IT.
> # Steward, 2026-08-15. Every `depends_on` is now `merged`.
>
> **CORRECTED. This banner previously said all five expressions were stopped on
> one absent relation, and named [[RT-CONTKEY-CONSUMING-OCCURRENCE]] as `ready`
> and awaited.** That relation **merged 2026-08-14 at `a998d3f6`**, and so did
> the carry that followed it. The banner outlived the blocker it described and
> read as current state — which is how a leader asking for the next increment
> got *"the next increment is your cut"* when the honest answer was that this
> node has none.
>
> **`D2k-1c` is a wrong cut, not an unfinished one**, ruled on the second
> boundary. Both ways forward cross this node's own banned scope: one mutates
> the planner-owned `ContinuationTemplate` population, the other needs a
> projection through the excluded continuation-source surface. **A WP that must
> cross its own banned scope to discharge its AC has been cut wrong**, and the
> repair is a new cut, not a wider one.
>
> ### THE ROUTE QUESTION IS [[RT-CONSUMING-OCCURRENCE-ROUTE-WIRE]]
>
> **UPDATED 2026-08-15 — that node is `merged`, and so are the two after it.**
> The live successor is now [[RT-CROSSING-CALL-SITE-ATTRIBUTION]] (`ready`,
> `S`); [[RT-REQUIRED-CONSUMER-REACH-CENSUS]] merged with `D1`-`D5` delivered.
> Route-wire measured the depth-2+ boundary to be a representation problem;
> [[RT-REQUIRED-OCCURRENCE-PROJECTION]] built the lawful surface and advanced
> **row 4 depths 2 and 3** to a `Closure` refusal at `66715f9fb`. **Row 4 depth 1
> is outside that surface by construction** — the projection is minted only where
> `required != source` — so it is a different residual, and the census node is
> what establishes the per-row reach. **Everything below this line describes the
> state before that merge; read the successor for what is live.**
>
> `ready`, `M`, runtime-owned, filed 2026-08-15. The chain's three landed nodes
> each supplied an input and **not one closed a row**:
> `RT-CONTKEY-CONSUMING-OCCURRENCE` gave the source-keyed relation, complete at
> depth 1; `RT-CONTKEY-CONSUMER-DESCENT-CARRY` gave `required(N)` = the consumer
> established at `N-1`; `RT-CONSUMER-CARRY-CONTROL-DEBT` took the five carries
> off that merge.
>
> ⇒ **`required_consuming_occurrence` is production-written and TEST-ONLY-READ.**
> No production path has ever consulted it. Wiring one consumer and measuring
> what the boundary then does is that node, and it deliberately carries **no
> closure AC** — that supplying the relation closes the route is not
> established, ruled twice.
>
> ### WHAT REMAINS HERE
>
> **The landed partials are real and this node is not reset** — `D2k-0`,
> `D2k-1a`, `D2k-1b-i`, eleven `D2f` increments, and `D2k-1e`. **`D2k-1c-0c` is
> stale in the frame and already repaired in the tree**: the six `site` labels
> now carry the prescribed `function@qualifier` form. **`D2k-1c-1a` moved** — it
> needs a rebound field, so it is `D3` of the route-wire node, taken as part of
> that validation rather than after it.
>
> **Do not read `active` as "someone is building this."** It stays `active`
> because increments continue to land against it; the route work does not.
>
> ### ZERO DISPATCHABLE INCREMENTS, and as of 2026-08-15 that is MEASURED
>
> **Every item this file called owed has been checked against the tree and all
> three are discharged** — see the table further down. Combined with `D2k-1c`
> being a wrong cut and `D2k-1c-1a` having moved, **there is nothing a ring can
> start here.** That is not the same as blocked, and it is not framing debt:
> the node's five expressions are genuinely unrepaired, and the reason no slice
> exists is that the repair needs a representation surface that does not exist
> yet.
>
> ⇒ **`depends_on` now names [[RT-REQUIRED-OCCURRENCE-PROJECTION]].** That edge
> was real and lived only in prose across three files. Writing it down makes
> the operator's top-priority chain machine-readable end to end:
>
> ```mermaid
> graph LR
>   PROJ[RT-REQUIRED-OCCURRENCE-PROJECTION<br/>active] --> CONS[RT-LEXICAL-RECURSOR-CONSUMERS<br/>active, zero dispatchable]
>   CONS --> TRANS[RT-RECURSOR-TRANSPORT<br/>active, D3 gated]
>   TRANS --> RET[RT-DESCENT-RETIRE]
> ```
>
> **`RT-RECURSOR-TRANSPORT`'s `D3` gate is unaffected and stays as written** —
> it is keyed on the transport half *actually landing in the tree*, checked
> against `enum RecursiveDescentResidual`, **not** on this node's status. That
> wording is correct and is why the gate survived this correction.

> # THE RESIDUAL IS SIX EXPRESSIONS AT TWO WALLS — updated 2026-08-12
>
> The `draft`-until-the-A-node-merges hold is **spent**:
> [[RT-MATCH-RECURSOR-CONSUMERS]] merged, this node released and is `active`,
> and eleven `D2f` partials have landed.
>
> **Two of the original eight expressions have left this node.** Row 2 went to
> [[RT-LEXICAL-ROW2-MISSING-MINT]] (**merged** 2026-08-12, no production repair
> — the row was not a regression, its assertion was over-specified). Row 5's
> **before**-hole went to [[RT-LEXICAL-R3-FUSION-EMITTER]] together with the
> `D2f` emitter machinery, **released to Runtime 2026-08-12**.
>
> | cell | expressions | where it stopped |
> |---|---|---|
> | rows 1 and 4 | 4 | `StaticWorkerBinding` wall |
> | row 5 **after**-hole | 1 | `StaticWorkerBinding` wall |
> | row 3 | 1 | retained singular-specialization wall |
>
> ⇒ **Five of the six remaining expressions are stopped at one wall**, and
> `#6d` closure gates [[RT-RECURSOR-TRANSPORT]] `D3`, which gates
> [[RT-DESCENT-RETIRE]]. The measured remainder for closure is **closer to a
> week** (runtime-leader `evt_645tm43wf1cne`).
>
> > ### TWO MORE EXPRESSIONS HAVE LEFT. ROW 4 DEPTHS 2 AND 3 ARE THE
> > ### DURABLE LANE'S. Steward, 2026-08-15.
> >
> > **Routed on [[RT-PLANNED-CLOSURE-PREEXISTENCE]] exact `1cd9947cf`,
> > Architect `dec_55a8p5m2f8gzk`.** The planner gives field 0 of origin 12 a
> > closed producer set of exactly `[(StaticOriginId(12), LexicalClosure)]` in
> > both governed plans ⇒ **the source program authored the closure and the
> > realization did not manufacture it.** Branch 1 is refuted, and the repair for
> > these two expressions is [[RT-CLOSURE-BOUNDARY-LANE]]'s, not this node's.
> >
> > **Row 4 depth 1 stays here.** It is outside the projection surface by
> > construction — the projection is minted only where `required != source` — so
> > it is a different residual and was never part of this routing.
> >
> > ⇒ **The table above overstates this node's surface by two.** The residual it
> > owns is **four** expressions: rows 1 and 4-depth-1 and row 5's after-hole at
> > the `StaticWorkerBinding` wall, plus row 3 at the retained
> > singular-specialization wall. **This is the third time expressions have left
> > this node and a heading has kept describing the old population** — row 2,
> > row 5's before-hole, and now these two.
> >
> > #### THE SEQUENCING CONSEQUENCE IS NOT MINE TO SETTLE, AND IT IS LIVE
> >
> > **If these two expressions must be green for this node to close, then
> > [[RT-CLOSURE-BOUNDARY-LANE]] now sits on the critical path to
> > [[RT-DESCENT-RETIRE]]** — this node gates [[RT-RECURSOR-TRANSPORT]] `D3`,
> > which gates the retirement, which is the operator's top-priority work. **That
> > node was framed today and is deliberately unsized past `D1`.**
> >
> > **The alternative is that this node closes with those two expressions routed
> > out**, exactly as row 2 left without a production repair. **Which reading
> > holds is a closure-criterion question for the ring and the Architect**, asked
> > at the durable-lane node's `D1`/`D2` rather than answered here. **Do not
> > assume either while sizing.**
> >
> > ##### ANSWERED, AND MY FRAMING OF IT WAS WRONG IN ONE PLACE
> >
> > **Architect `evt_3emtcx20vjg8s`. The criterion below is his sentence; landing
> > it is mine, and it is landed here.**
> >
> > **No, they need not be green — and *"route them out"* is not the reason.**
> > This node already states that its population is *"the production
> > `LexicalCallArgumentRecursor` predicate"* and that the eight expressions are
> > *"a floor, not a perimeter"*. Row 2 already left without a production repair.
> > **The list was never the criterion.**
> >
> > **But routing them out does NOT make the retirement safe, and that is the
> > step my flagging skipped.** These eight expressions are **previously green** —
> > they compile today on the descent lane and fail closed on the functionized
> > lane. **They are not invariant across the lane change**, so after `D3` retires
> > the class, **two programs that compile today refuse.** Relabelling the owner
> > does not make that disposition acceptable.
> >
> > > **THE CRITERION, as ratified:** this node closes when **every expression in
> > > the production predicate's population carries a recorded disposition** —
> > > **repaired**, or **refused with its spec clause cited and its
> > > pre-retirement behaviour accounted for.**
> >
> > **It is satisfiable under both spec readings.** It keeps the real gate — no
> > silent capability regression — and drops the false one, compile-green.
> >
> > ⇒ **It also decouples [[RT-DESCENT-RETIRE]] from the SIZING of the durable
> > lane**, which is the coupling flagged above. **The lane's size stops being on
> > the critical path; the disposition stays on it.** That is the sequencing
> > answer, and it is better than either reading I offered.
> >
> > **The measurement that settles the disposition is `D0` of
> > [[RT-CLOSURE-BOUNDARY-LANE]]** — does the **descent** lowering of these two
> > expressions perform an equivalent boundary crossing? Nobody has measured it.
> > If it does, the retirement **corrects** a live defect; if it does not, the
> > retirement is a capability regression that must be covered or explicitly
> > recorded as a narrowing.
>
> > ### THE `StaticWorkerBinding` WALL IS `D2k`, INSIDE THIS NODE — 2026-08-12
> >
> > `docs/program/wp/RT-LEXICAL-RECURSOR-CONSUMERS-D2a.md` filed it as
> > *"successor, **unfiled here**"* and **no successor was ever filed**, so five
> > expressions sat behind an unowned repair. **Architect ruling
> > `evt_5wvk3e8k1bjqn`** (on Steward question `evt_27chdjk4xh200`) places them
> > here: **frame as `#6d`'s next `D2` increment, `D2k`. No new node.**
> >
> > **Frame: `docs/program/wp/RT-LEXICAL-RECURSOR-CONSUMERS-D2k.md`**,
> > shovel-ready, fixed inputs at `b2ee3377`. **Sequence after
> > [[RT-LEXICAL-R3-FUSION-EMITTER]]** — same file set, and Runtime runs one
> > node at a time.
> >
> > **[[RT-CONTSRC-CALLABLE-CONTRACT]] is NOT a prerequisite**, and no graph
> > edge is owed. The two artifacts state the same semantic fact on **different
> > component surfaces**: here the callable fact is already expressible and
> > installed as `LoweringEnvironmentBinding::StaticWorker`, and both the direct
> > lowerer and the source machine already have the lawful consumer — a `Call`
> > whose callee is an exact `Var`. **The wall says the binding reached the
> > wrong CONSUMER SHAPE, not that the component lacks vocabulary.** The
> > callable-contract node repairs a separate planner/projection gap in
> > `ContinuationSourceSlotAuthority`; it stays real and `ready` and is not on
> > this retirement path by virtue of these five walls.
> >
> > **The trap `D2k-0` exists to close:** all five report the same refusal
> > sentence, but it is emitted by **one chokepoint every value read funnels
> > through**, so a shared string is equally what five unrelated wrong-consumer
> > routes would produce. **Prove the roots before sizing the repair.**
> >
> > #### `D2k-0` MERGED 2026-08-12 — THE ROOT COUNT IS ONE
> >
> > Exact `67b6b99e`, PR #1969, **CI green**, `origin/main` `d9e717fe`.
> > Declared base `b6c8655c`, one non-merge commit, one test-only path
> > `core/tests/control.rs`, `+128/-0`. Decision `dec_6x992cvp2v0ds`; QA
> > `evt_3ssgrrmvr3kfe`, Architect `evt_2s93e1qy2nxet`. M6 verified by blob
> > identity from the declared merge-base. **Evidence only — no repair.**
> >
> > **What is asserted, exactly:** all five refuse with construct
> > `StaticWorkerBinding` and the **same `edge`**, `a Var in value position`.
> > The committed table also gives a refusing caller and a route per
> > expression. **The frame's scope signal did not fire: no fifth `value_at`
> > caller at that base.**
> >
> > ⇒ **`D2k-1` is provisionally sized as a SINGLE repair.** That sizing is this
> > measurement's only load, and it is the reason the measurement came first.
> >
> > **The bound travels with the number.** The route is **not executable and the
> > candidate says so** — observing which caller fired needs instrumenting
> > `value_at`, which `AC-2` requires byte-unchanged — so the route column is a
> > **measured-and-reverted probe**, and the conclusion stands as *"one root, on
> > this evidence."* **A different `edge` met during `D2k-1` is a SECOND ROOT
> > and re-opens the sizing.** Stop and hand back; do not widen the repair.
> >
> > #### THE SIZING IS PROVISIONAL — the discriminator is ONE-SIDED
> >
> > **Adversary `evt_1mpre0qmhx6s5`, confirmed against `67b6b99e`. Rider on
> > `D2k-1` at `evt_hkhfctkb888h`, and it lands BEFORE the repair.**
> >
> > The frame's discriminator is *"the `edge` argument and the causal consumer
> > owner, **never the message**."* **`wall()` returns `(construct, edge)` with
> > `edge` parsed out of the refusal message via
> > `reason.split(" is a value-producing position")`, and no consumer owner
> > appears in the tuple or in any assertion.** So *"never the message"* is
> > false as written, and the message-independent half is absent.
> >
> > **The direction is what matters: the claim being asserted is SAMENESS**, and
> > a discriminator establishing identity must be able to distinguish. **The
> > candidate's own doc discloses how it might not — `mod.rs:3661` forwards its
> > caller's edge**, so a forwarded route presents `a Var in value position`
> > indistinguishably from a direct `core.rs:14593` route. **Five same edges is
> > consistent with one root and with distinct routes converging through a
> > forward.**
> >
> > **An earlier version of this record said "the same call site." That was my
> > overstatement, corrected here before it landed** — the candidate never
> > claimed the route was executable.
> >
> > **The rider's three outcomes are all good:** the owner is obtainable without
> > touching `value_at` and all five share it (one root established on a
> > discriminator that can distinguish); it resolves to more than one root (the
> > sizing stop fires, late but correctly); or **no message-independent
> > discriminator is constructible under `AC-2`** — a return-to-the-Steward
> > finding meaning *"one root"* can never be established on the evidence this
> > frame permits. **`AC-2` is not to be weakened to resolve the third.**
> >
> > #### RIDER ANSWERED — outcome 1, and the SAME measurement stopped the repair
> >
> > Runtime implementer `evt_6z8xjk3gkh821`; stop report `evt_134atze90gs1m`.
> >
> > **"One root" survives on a discriminator that can actually distinguish.**
> > The causal consumer owner for all five is **`RuntimeExpr::Construct`** —
> > `PX8JTree1::Node`, `PX8JScopeTree::Node` at three depths, and
> > `PX8JHoleOutput::Node`. It is the enclosing `lower_expr` arm, so nothing is
> > parsed out of `reason`. **The forwarding ambiguity is measured absent, not
> > argued away:** all four `value_at` callers were tagged, `mod.rs:3661`
> > **never fired**, and the last tag before every refusal was `core.rs:14593`.
> > **`value_at` was never touched** — the instrumentation sat on its callers
> > and on the `Construct` arm — so `AC-2` is intact and outcome 3 does not
> > arise. **The stronger measurement agrees with the weaker one rather than
> > overturning it.**
> >
> > **It is PROBE-MEASURED AND REVERTED.** The committed `D2k-0` test still
> > asserts only construct and edge, so the **durable** evidence remains the
> > narrower thing. The leader has ordered the owner assertion committed as its
> > own pre-repair checkpoint (`evt_4ggrxzk22e46m`).
> >
> > **DISCHARGED — the redness claim is executable, not inherited.** That
> > sentence read *"still unverified by anyone. Still owed."* until 2026-08-15.
> > `D2k-1a` landed `d2k_0_control_reddens_when_the_wrong_consumer_condition_is_removed`
> > (`control.rs:5994`, `905fd05de`), which runs `D2k-0`'s **own** predicate with
> > the wrong-consumer condition removed and asserts it answers `None`. **The
> > owner assertion the leader ordered as a pre-repair checkpoint
> > (`evt_4ggrxzk22e46m`) is `control.rs:5572+`, also `D2k-1a`.**
> >
> > > ##### SECTION 3'S FRONT-LOADED PREMISE IS MEASURED FALSE FOR THESE FIVE
> > >
> > > **The same measurement that saved "one root" is why `D2k-1` stopped.**
> > > The frame's section 3 says the lawful consumer — *a `Call` whose callee is
> > > an exact `Var` bound to `StaticWorker`* — is **already installed**, so the
> > > wall means the binding reached the wrong consumer *shape*.
> > >
> > > **For these five there is no call.** The static worker is a **constructor
> > > argument**, which is a value-producing position **by construction**, so
> > > the exact-`Var` callee path has nothing to key on. **The root "one root"
> > > names is not the root section 3 assumes.**
> > >
> > > ⇒ **Section 3 is marked *"do not re-derive it"*. Do not act on it for
> > > these five until the Architect's disposition lands** (`evt_78agvsvb0z450`,
> > > grounding static workers as constructor arguments). **The repair scope is
> > > open and must not be broadened by the ring.**
> >
> > **The committed test asserts the edge and refusal per expression and reds if
> > a repair moves any of them.** That is what makes this durable rather than a
> > one-time reading — **a red there is information, not a test to update.**

> # R3 MERGED. THE BLOCKER IS DISCHARGED AND THE STATUS IS NOW `ready`.
> # Steward, 2026-08-14. Nothing was un-landed and nothing is re-ranked.
>
> **`RT-LEXICAL-R3-FUSION-EMITTER` merged as squash `34769380`.** The section
> below says this node is `active` only because R3's fusion disposition was
> incomplete. **That condition no longer holds**, so the word `active` would
> now mean the one thing the section below was written to stop it meaning:
> in progress.
>
> **No seat holds this node.** Deps met, unassigned ⇒ `ready`.
>
> **~~Sequenced second, deliberately.~~ SUPERSEDED BY THE OPERATOR, 2026-08-14
> — see the block immediately below. The reasoning is retained only so the
> override is legible; do not act on it.** It read: Runtime's next assignment
> is [[RT-DYNAMIC-ARM-SCALAR-MERGE]]; that node unblocks `KERNEL-NESTED-IND`
> and behind it three Kernel seats and Foundation's `DS-9`, whereas this node
> blocks `RT-RECURSOR-TRANSPORT` inside Runtime's own lane, so *"six seats idle
> beats one lane's depth."*

> # OPERATOR PRIORITY RULING, 2026-08-14. THIS IS RUNTIME'S PRIORITY WORK.
>
> **Verbatim:** *"that is the priority for the runtime team. prioritize that
> work over other runtime work."* — issued on the Steward's measured answer that
> **nothing in the preceding twelve hours advanced the `RecursiveDescent`
> retirement**: 49 commits on `main`, none touching `RecursiveDescent` in
> `crates/`.
>
> ⇒ **This node outranks every other Runtime node**, including
> [[RT-CHECKED-IH-REALIZATION-AUTHORITY]], which is `ready` and stays unreleased
> until this lands. **A priority call between `ready` WPs is the operator's
> under `ken-steward` §3; the Steward's own contrary call above was surfaced and
> overruled.** Do not re-derive the trade-off — it was made and answered.
>
> **Why this node specifically, and it is the whole reason the ruling is
> narrow:** it is the **single unblocked node** on the retirement path. Both its
> deps are merged ([[RT-MATCH-RECURSOR-CONSUMERS]],
> [[RT-LEXICAL-R3-FUSION-EMITTER]]). It blocks [[RT-RECURSOR-TRANSPORT]] —
> `draft`, whose other four deps are all merged — which blocks
> [[RT-DESCENT-RETIRE]] — `draft`, whose other four deps are all merged.
> **Every other node on both paths is done. This one is the whole remaining
> chain.**
>
> **The debt it is holding, measured at `main` `6c574cdd`:** 92
> `recursive_descent_residual`, 58 `RecursiveDescentResidual`, 28
> `select_body_emission_authority`, 17 `BodyEmissionAuthority::RecursiveDescent`
> (12 outside tests). The selector still evaluates on every compilation and the
> emission lane is still compiled in — **which is exactly the "half-migrated
> state carrying tech debt for no benefit" the original directive named.**
>
> **THE THREE ITEMS THIS BLOCK CALLED OWED ARE ALL DISCHARGED — measured at
> `fa18caec0`, Steward 2026-08-15. Do not frame any of them.**
>
> They were discharged by **this node's own `D2k-1a`**, and the frame never
> caught up. That is the sharpest form of the staleness: the work did not move
> to another node and get lost, it landed here and kept advertising itself as
> owed. Each line below is a file-and-line read, not a status inference.
>
> | called owed | actual state at `fa18caec0` |
> |---|---|
> | the `D2k-0` **redness claim**, *"still unverified by anyone"* | **executable at `control.rs:5994`**, `d2k_0_control_reddens_when_the_wrong_consumer_condition_is_removed`, landed `905fd05de`. It runs `D2k-0`'s **own** predicate with the wrong-consumer condition removed — the source comment says *"not a twin of it"*, which is the failure mode a redness proof usually has. |
> | **(a)** name the two-root population once as a `const`, iterate it in the nine controls | **`R3_TERMINAL_STOP_POPULATION` at `control.rs:2711`**, landed `12cefd5b3`, iterated at **exactly nine** sites (`:3128, :3290, :3424, :3583, :3718, :3871, :4024, :4181, :4350`). |
> | **(b)** add the armed `ProducerArity` row, *"~6 lines, by relaxing its own `assert!(error.is_none(), ...)`"* | **present at `control.rs:4365-4386`.** The `assert!(error.is_none(), ..)` now scopes the population loop only (`:4353`); the `ProducerArity` case captures the error, matches the widened-producer refusal literal, and pins `(validations, consumptions, invocations) == (0, 0, 0)`. |
>
> **NOT re-measured, and still stated as inherited:** the residual at the foot
> of this file — five groups carrying no cardinality assertion. It belongs to
> the next `#6d` slice frame, not to this node. **I checked the three above and
> not this one**; treat it as unverified in either direction.
>
> **Why this mattered enough to correct:** the block prescribed a specific
> remedy with line numbers and a cost argument (*"the harness cost was measured
> false"*), which is exactly the shape a framer picks up and turns into a WP.
> **A repair block reads identically whether it is owed or done**, and the line
> numbers it carried had already shifted — `:4325` now lands on the `arm()` call
> inside the very control the item asked for.

## `active` here means WAITING ON R3, not in progress. Edge declared 2026-08-14

runtime-leader at `evt_5rvv28pt58zge`, answering a Steward sequencing question:

> *"`RT-LEXICAL-RECURSOR-CONSUMERS` has no independently startable repair
> slice. Its `D2k` seed-route work is stopped: the five seed witnesses cannot
> carry the checked-fusion repair, route work is explicitly stopped, and its
> remaining semantic-effect/route obligations were transferred to R3. It
> remains active only because the R3-owned fusion disposition is incomplete."*

⇒ **`depends_on` now names `RT-LEXICAL-R3-FUSION-EMITTER`**, and R3's `blocks`
names this node. **The relationship existed only in this body's prose**, so
`gen-progress.sh` — which reads `depends_on` — showed an `active` node with no
blocker while it was in fact waiting on another node's disposition.

**It failed in the direction that hides a stall.** A reader of the tracker, the
operator included, saw an `active` Runtime node and would conclude the ring was
progressing on it. This is the same defect class `RT-NESTED-IH-NATIVE-REALIZATION`
recorded for `KERNEL-NESTED-IND`: an edge real in prose and absent from the
field the generator reads. Two instances now, in the same lane, found the same
way — by asking a leader whether a node was startable rather than by reading its
status.

**Do not read this edge as a graph tidy.** It is the transferred-obligation
relationship: work that was this node's is now R3's, so this node cannot finish
before R3 disposes of it.

## What it is

**Eight expressions across five test families**, previously green, that fail
closed on the functionized lane once [[RT-RECURSOR-TRANSPORT]]'s `D3` retires
the `LexicalCallArgumentRecursor` residual class:

| row | fixture family | expressions |
|---|---|---|
| 1 | `owned_scope_deletion` | 1 |
| 2 | `all_three_producer_paths` | 1 |
| 3 | `siblings_share_an_origin` | 1 |
| 4 | `scope_segments` depth 1, 2, 3 | 3 |
| 5 | `selected_scope` before / after hole | 2 |

Every one enumerates **exactly `{LexicalCallArgumentRecursor}`**, and every
unexcluded compile returns `Ok` — so each row's red is produced by the lane
change and not by a fixture that was already broken.

This node repairs that consumer population **on the pre-retirement tree**, so
`D3` can then retire the class and prove these rows green with no exclusion hook
and no `#[ignore]`.

## The activation seam, measured rather than assumed

**B-only exclusion is this node's seam and it is proven.** At exact `D2`
`8efdfdb3`, excluding **only** `LexicalCallArgumentRecursor` leaves the residual
set empty for each of the eight expressions, so the selector reaches
`FunctionizedUnits` while production continues selecting `RecursiveDescent`.
Every row carries a real activation denominator — a compile through
`px8j_capture_source_trace` — so a refusal cannot be credited where the harness
never reached the path.

⇒ **The repair can be built and proven before the retirement**, which is what
makes this an independently mergeable node rather than a quarantine.

> **The seam was asserted before it was measured, and it nearly set the
> sequencing.** Both the Architect and the Steward stated that the existing hook
> activates all six fixtures; that was a candidate promoted to a fact. The hook
> removes B from the **complete** residual set and reaches `FunctionizedUnits`
> **only when the remainder is empty** — true for these eight, and inapplicable
> to row 6, whose set never contained B at all. The census
> (`evt_16cmej481q7ns`) is the object of record.

## The population is the production predicate, not this list

**`D0` closes the population from the production `LexicalCallArgumentRecursor`
predicate.** The eight measured expressions are a **floor**, not a perimeter.

Sweep every compilation entry that can supply the predicate. Helper spelling,
snake_case fixture spellings and `BodyEmissionAuthority::RecursiveDescent`
assertions are **candidate selectors, not closure** — a grep tells you which
fixtures might be in the family, never what any one of them enumerates.

## Size

**`M`, provisional.** It is a scoping figure taken from a symptom count, and
five rendered refusal strings are **not** five proven causes.

**`D0`/`D1` are authorized to return a partition instead of a repair.** If the
causal partition finds materially distinct authorities rather than downstream
symptoms of one root, or any repair needs a new planner/ABI population, **return
the partition before coding.** Do not silently turn one symptom-named node into
five repairs.

## The edge that is not in the frontmatter

This node's base is **post-`D2`-correction `main`**, and that correction is a
*partial* merge of [[RT-RECURSOR-TRANSPORT]], not its completion. A `depends_on`
naming that node would be a **cycle** — its `D3` is blocked on this one.
`depends_on` is empty and the base is stated in prose and in the frame.

⇒ **Read the base from the frame, not from the edge.** The machine-checked edge
that matters is `blocks: [RT-RECURSOR-TRANSPORT]`.

## What is ruled and not reopened

- `10369776252861e8b15e613576256a3682c70066` is **held evidence only** — not a
  candidate, not a repair base, not to be continued.
- **Zero new `#[ignore]`.** The Steward ruled these quarantinable at
  `evt_7vhjcstd37a50`; that ruling is **withdrawn** and was not revived by any
  later correction.
- The old-green semantic controls are **not disposable**. Surface-Ken
  reachability is unproved; old-green runtime capability is **proved**, and
  these rows are the only probes for the guards they exercise.

## `D2f` ABI-class accepted partial — MERGED 2026-08-11, PR #1897

Exact `006730d4085a04e95dc6b2ca7bebe19d1fbcb6d4` from declared base `84a8f66d`;
one commit, six paths, `+285/-35`, no added ignores. M6 blob identity 6/6 MATCH.
**This node stays `active`.**

> ### IT CLOSES ZERO OF THE EIGHT EXPRESSIONS, BY DESIGN
>
> **A landed partial on a node with 27 merges reads as progress against the row
> count unless it says otherwise.** This one is a **structural prerequisite**:
> the fail-closed `StaticContinuationFusion` ABI class and
> `ContinuationEmissionOwner::Fusion`, with every consumer disposition refusing.
>
> **No constructor, no emitter, no source-body emission authority, no redirected
> producer edge, and no fusion runtime behaviour.** Verified on the object:
> seven `Err(` sites added and **zero** panic-style macros, so the class refuses
> rather than traps; and the only two matches for redirected-edge or emission
> vocabulary are **doc comments stating their own absence.**

## Second `D2f` partial — identity-plane wiring, MERGED 2026-08-11, PR #1899

Exact `1b362f5ea3201ba4dc54d74f0dc88462e3fa4f19` from declared base `e0e4aeb3`;
one commit, five `ken-runtime` paths, `+123/-4`. M6 blob identity 5/5 MATCH.

**The landed fusion identity plan now reaches the sole production compile
path**, with a causal arrival control. **Empty resolution remains legal.**
Excludes definitions, descriptors, authorities, edge redirection, emitter
behaviour, planner signature widening, and every emitter AC claim.

**This one also closes zero of the eight expressions.** Two partials, two
structural prerequisites, zero rows.

> **What its control proves, CORRECTED 2026-08-11.** *"Empty resolution is
> legal"* is exactly the condition that lets an arrival control pass for free,
> so what the control does and does not establish is worth stating exactly.
>
> **It proves arrival, and that half is real.**
> `NonZeroUsize::new(planes.len()).expect(...)` panics on an empty drain, so
> **a compile that never reached the production builder fails the control
> rather than passing it.** Resolved sizes are **recorded, not pinned** — this
> witness plans no admitted fusion, so a control asserting resolution *success*
> would have been either vacuous or wrong.
>
> **STRUCK — *"the assertion equates the recorded planes with the established
> arrival count as one population."*** **That is false, and it was published
> here and in PR #1899's body.** `planes` is read **once**; `reached` is
> `NonZeroUsize::new(planes.len())`; the assertion then compares `planes.len()`
> to `reached.get()`. Both sides come from that one read, so it is
> `planes.len() == planes.len()` — **a tautology, not a second measurement.**
> Adversary-measured; Steward disposition `evt_7ewdkteptjr8t`.
>
> **An equality is a measurement only if its two sides come from different
> reads.** No count of intervening named bindings changes that, and naming the
> doubt in the merge notification after the fact was not the same as checking
> it.
>
> **The repair is folded into the `D2f` emitter increment, not re-reviewed
> here** — the merged code is not wrong, only weaker than the sentence above
> claimed. The honest form is to drop the `assert_eq!` and keep the `expect`;
> **a second counter manufactured to make the equality look measured is the
> cosmetic repair and is forbidden.** A real equality becomes available only
> once a fusion resolves, at which point builder arrival and resolved-plane
> population are genuinely independent quantities.

**The `D2f` emitter is the next increment**, scoped to the one `R3` before-hole
witness — **not** an eight-row repair. The `R3` after-hole / missing-`Mint` cell
is excluded from `D2f` and owned by [[RT-LEXICAL-ROW2-MISSING-MINT]].

**Measured remainder** (runtime-leader, `evt_645tm43wf1cne`): the `D2f` emitter
plus its review cycle is **closer to one working day**; **`#6d` closure is
closer to a week.** ⇒ **`D2f` completion and `#6d` closure are separate planning
milestones**, and the former does not discharge the latter. The Steward examined
a re-cut on that estimate and **declined** — the remainder is bounded and named,
so cutting further would manufacture nodes rather than reveal them.

**The `D2a` rider below is discharged by this partial** — it is one of the six
paths.

## The `D2f` gating measurement came back EMPTY. 2026-08-11.

**The emitter increment is stopped, and it stopped for the right reason.**
Runtime measured the gate before touching the emitter, which is exactly what
the gate was placed there to do.

Measured on the exact `R3` before-hole `B`-only compile
(`px8j_equal_payload_hole_placement(BeforeReturnHole)` through
`px8j_capture_source_trace`), with temporary instrumentation at the production
call site, since reverted:

```
planes=[0]   oriented_present=[false]
```

One production compile reached the builder. It resolved **zero**. The first
reason is the `oriented` gate at `planning/static_transition.rs:8901` and
`:9058`, which returns an empty plan **before any candidate enumeration runs** —
before checked transport, IH bindings, or the root walk. The `None` originates
at `cranelift_backend/test_objects.rs:70`, where the harness passes a **literal**
`None` for `oriented_subcontinuation_plan`. Production oriented plans are
decoded from a checked package's metadata (`planning.rs:144`); the `px8j`
witness is a **seed-lane** compile, so there is no metadata to decode and
nothing that could supply one.

⇒ **This is structural, not a defect in the enumerator.**

**Every gate below the `oriented` check is UNMEASURED on this witness.** The
probe short-circuits at the first cause, so it measured that cause and not the
set. Nothing here says the enumerator would or would not find a candidate if a
plan were supplied.

**And no control has ever exercised this mechanism on the witness the ACs
name.** Every `D2h`/`D2j` control that reaches a fusion candidate uses its own
synthetic fixture (`d2j_entry()` / `D2J_DECLARATION` with a hand-authored
`d2j_oriented_plan_under(cause)`) and calls the builder directly. **None of them
compiles `px8j`** — so the mechanism is untested against the acceptance fixture
on *both* sides of the gate.

### The frame defect is the Steward's, and it is named here so it is not inherited

**`AC-1` requires the fusion to occur on the exact `px8j` `R3` before-hole
compile. That compile structurally cannot carry an oriented plan today.** The
frame therefore pins acceptance to a witness that cannot carry the mechanism's
required input — **a defect in the frame I wrote, not in the work.** It is
recorded now rather than after the ruling, because the next slice frame would
otherwise inherit the same witness by citation.

**`AC-1` is NOT amended yet, deliberately.** Which amendment is correct depends
on the mechanism ruling below, and amending first would presume its answer.

### The fork is RULED. Architect `evt_6vf66hmwv52y6`, 2026-08-11.

> **There is no lawful plan-supply route for the exact unmarked `px8j` seed
> witness, so `D2f`'s `AC-1`/`AC-2` fixture binding was UNSATISFIABLE.**
>
> This follows from the **landed required-member ruling**, not from the empty
> measurement — the measurement revealed the defect rather than causing it. The
> `px8j` witness was **deliberately preserved as the unmarked negative**: no
> checked frame, no selected-IH-slot, no checked-IH-invocation marker, so it is
> not a fusion candidate. `validate_oriented_subcontinuation_transport` makes
> the boundary structural — unmarked IR with `None` is lawful seed IR producing
> no fusion; unmarked IR with a non-empty plan is a marker/plan mismatch that
> must **reject**; an empty supplied plan carries no checked transport
> coordinate; wrapping changes the occurrence tree.
>
> **Route 2 — making fusion independent of `oriented` — is REJECTED outright.**
> It would reopen `D2h`'s soundness-bearing identity and contradict the
> required-member ruling.
>
> **The lawful positive already exists**: the landed `D2g`/`D2j` checked
> `R3`-shaped fixture and its complete, independently authored
> `OrientedSubcontinuationPlanV1`, consumed through **one hoisted `#[cfg(test)]`
> constructor** and entered through `compile_expr_into_object_module` with
> `Some(oriented)` — never by calling the builder or emitter directly.

**Frame correction landed at `main` `17f68eb1`** (PR from the Steward; Runtime
notified `evt_r775vtj0pqye`). `AC-1` now names the checked `D2j` witness as the
positive full-pipeline baseline; `AC-2` binds to that twin's **own freshly
derived coordinates**, with the origin-23 reference struck as an old `px8j`
coordinate. **`px8j` is retained as the absence / ordinary-refusal comparator**
and must never again be described as the fusion-positive.

**A new `Deliverable 0` gates the emitter**: the old negative at resolved plane
`0`, the checked positive at resolved plane `1` with exactly one
key/ID/descriptor, and a one-marker-stripped exact validator refusal — committed
**before any emitter definition**. **No emitter AC may be credited until the
positive row is non-zero.**

### The two routes as they stood before the ruling, retained

Both crossed lines a leader or the Steward could not cross alone, which is why
this went to the Architect rather than being decided in the ring:

1. **Give the `px8j` witness a lawful oriented plan.** Keeps `AC-1` as written.
   But authoring the plan is authoring the input the key re-derives against, and
   the line between *supplying the witness's real oriented facts* and
   *fabricating a candidate so the emitter has something to emit* is exactly the
   line the ring was told not to cross.
2. **Make fusion not require an oriented plan.** This reopens `D2h`'s key
   re-derivation, which is the soundness-bearing half and is **excluded scope**
   under this frame.

**Neither was started.** The standing risk handed back one turn earlier is now
**confirmed rather than suspected**: an emitter built against this witness would
discharge `AC-4` **vacuously** — a no-activation proof over nothing emitted
passes for free — and `AC-6a`'s refusal controls would assert against a resting
zero, which the frame already warns proves nothing.

⇒ **Do not authorize a synthetic candidate or a zero-definition emitter to
unblock the increment.** That trades a stop for a control that cannot fail,
which is the failure this node has now filed against itself three times.

**The ruling closed both routes and supplied a third the frame had not
considered** — reuse of the already-landed checked fixture. **The forbidden
routes are now named in the frame** so they are not re-derived as options: no
`Some(plan)` to `px8j_capture_source_trace`, no synthesized default plan, no
marker inference from the Runtime shape, and no weakening of the required
checked-transport key member.

## Third `D2f` partial — the arrival-control repair, MERGED 2026-08-11

Exact `aa3b78f8680c9637b754d524012b0d7d48c38834` from declared base `87f6983f`;
one commit, one path (`lowering/core/tests/control.rs`), `+18/-8`. Decision
`dec_17c3zfw5zxwk8` resolved APPROVE — Architect `evt_1y9jzz923ymdd`, QA
`evt_1ry88yshf6629`.

**Deletes the tautological equality corrected above.**
`NonZeroUsize::new(planes.len()).expect(...)` now stands as the whole control.
**No second counter was manufactured.** Causal A/B: replacing the production
observation with a discarded length reds the control at its own arrival message;
restored byte-identically.

**This one also closes zero of the eight expressions.** Three partials, three
structural prerequisites, zero rows — stated here because the merge count is
what a later reader will otherwise use to size this node.

### The arrival proof rests on the CALL SITE, and nothing says so. Fold into `D0`.

Adversary `evt_23zyn4pywy6yg`, measured on `446c3e79`. **The vector-shape
argument holds** — `d2f_note_production_fusion_plane` has one writer
(`core.rs:444`) and one call site (`core.rs:2057`), and **the call is
unconditional**. So a reached compile pushes exactly one element (possibly `0`)
and an unreached compile pushes nothing, which is what makes `planes.len()`
discriminate arrival from non-arrival.

**The durability defect: that reason is not written beside the thing it
protects.** The comment at `:2053-2055` explains why the observation is the
consumer; **it does not say that the unconditional push is what makes `len()`
discriminate.**

⇒ **Pushing a `0` looks pointless in isolation**, so `if !static_continuation_`
`fusion_plan.is_empty() { … }` is a plausible tidy. It would silently convert
the control from *"the path was reached"* into *"the path resolved something"*
and make the `expect` **panic on a legal empty resolution** — which is exactly
the state this witness is in.

**Disposition: one sentence at `:2057`, folded into `D2f` Deliverable 0.** No new
node, no re-review of `aa3b78f8`. Same file family, active ring, and it is
cheaper than the grep that found it.

## `e4531318` — APPROVED, THEN WITHDRAWN UNPUBLISHED. Not a dropped merge.

Deliverable 0's first candidate `e45313180eb6404a309df0d0234a686c2d239405`
(one commit, five `ken-runtime` paths, `+323/-5`) reached a **resolved APPROVE**
— `dec_5x9mfj08wfftt`, Architect `evt_79wp5r4wj64jz`, QA `evt_3gpsy11vbr0pn` —
and was routed to the Steward for publication. **It was withdrawn by
runtime-leader before the publisher ran** (`evt_2yr8wqjknbmvs`), on Architect
ruling `evt_6907h4rv5kq1a`. **It never became a PR and nothing was reverted.**

**Why, and it is not a defect in the candidate.** The Architect's own words: it
**remains a sound identity-plane partial**, but its **bare-root observation
cannot carry into emitter acceptance.** The gate observed the bare
`DeclarationRef`, and root projection stops there at `Unsupported(Closure)` — so
that shape **cannot reach the definition movement Deliverable 0 exists to
prove.** A positive that cannot reach the claimed movement is not a positive.

**`dec_5x9mfj08wfftt` is spent on `e4531318` alone.** The applied-root recut
needs **fresh QA and Architect review**; no coordinate and no vote carries.

⇒ **This is the accepted-partial policy working, not failing.** The candidate was
correct for the claim it made and merging it would have put a bare-root baseline
on `main` under a deliverable whose successor must not use one.

**Also fold: record where the "sole production compile path" claim is
established.** That question has been open across all three `D2f` partials. The
enumeration was performed once by the implementer during grounding and lives
only in a retro, so it currently reads as asserted. **Deliverable 0 measures
arrival counts, which is not the same claim** — arrival-is-one does not establish
that only one production call site exists. Record the caller enumeration beside
the control.

## `D2f` Deliverable 0 — the per-cause applied-root gate, MERGED 2026-08-11

Exact `068bd6bcd7a74fe970460f6dc54c842d7dc9edf0` from declared merge-base
`1585a2e6`; one commit, five `ken-runtime` paths, `+489/-21`. Decision
`dec_1n9rxnp3tbfjc` resolved APPROVE — Architect `evt_3cgg4nab999t6`, QA
`evt_2b6g8xk7mtza3`. PR #1910; `origin/main` is `f81e36f6`. M6 blob identity
5/5 MATCH. The staleness intersection against `main`'s changes since the
merge-base was empty, so no rebase was owed despite the base sitting two merges
back.

**What landed.** One exported `d2j_checked_fixture_under(cause)` with a
**per-cause** root family, every arm spelled out so a new cause is a compile
error rather than an inherited default: the `Exact` family takes an applied
`Call(DeclarationRef(D2J), [Unit, Unit])`; `ReHomed` takes a bare
`DeclarationRef` in its own explicit branch. The planner-only bare-entry helper
is retired with zero callers. All four `D2j` planner controls were rebaselined
on their own cause-selected roots — one had been sharing a single entry across
both causes, so one of its two sides was being measured against a program it
does not describe.

**The node stays `active`.** No emitter definition, authority, edge redirection,
or emitter AC is credited, and none of the eight target expressions closes.
This is the fifth merge on `#6d` and the count still overstates progress
against the node's actual surface.

### Both carried riders above are discharged in this range

The `core.rs` doc now names the **unconditional push** as why arrival *length*
discriminates never-reached from reached-and-empty — a conditional push would
collapse the three-versus-two phase split into an unreadable zero. And the
"sole production compile path" claim is now recorded as what it is: a
**structural** claim about four delegating entries, not a measurement over
program shapes. A reader can now tell which kind of claim it is, which is the
thing the rider asked for.

### Four candidates for one deliverable, and what that bought

`e4531318` approved-then-withdrawn (above); `9d942c4b` and `ce5323ca` QA-blocked.
**Neither block was about code.** The first: replacing a test body from `#[test]`
downward left the previous doc block in place, so the item carried two doc
comments, Rust concatenated them, and the **withdrawn** bare-root contract was
the first durable reading — on a commit whose central claim is that the
withdrawn revision does not extend. The second: a decorative glyph. QA named one
occurrence; the implementer swept the **class** and found five across two files,
leaving pre-existing ones alone on the grounds that copying nearby style is how
it introduced them.

**The mutation-evidence carry is MEASURED, not argued.** The three A/B mutations
were taken on `9d942c4b`'s tree. Each recut step was checked comment-only by its
author, but **nobody had checked the composition end-to-end** — three
individually-comment-only steps is not the same claim as the whole chain being
comment-only, and the raw stat across it is 3 files and 100 changed lines, net
negative, which `--stat` cannot separate into comments and code.

Measured at `f81e36f6`, on both trees present locally:

```sh
git diff -w 9d942c4b 068bd6bc -- crates/ | grep -E '^[+-]' \
  | grep -vE '^(\+\+\+|---)' | grep -vE '^[+-][[:space:]]*(//|/\*|\*)'
```

**Zero non-comment changed lines.** ⇒ The executable tree the mutations were
taken on is byte-identical to the merged one, and the evidence transfers. This
was first written here as a caveat *"it is an argument, not a re-measurement"*;
the adversary's point (`evt_2kn8jtgn64d9s`) was that a one-command conversion
from argument to measurement should never be left as a caveat, and that a decent
prior on the answer is a reason to expect the empty result, not to skip it.

## `D2f` Deliverable 5 — the complete-key redirect selector, MERGED 2026-08-11

**Exact `e89de6674f283b80184acd4228ca8a6ae506f6fb`, PR #1915.** Decision
`dec_svd7p853crep` resolved: Architect `evt_7ey1xa79ef22t`, QA
`evt_1nt2vy2rdh14h`. One non-merge commit from declared merge-base `16d7e467`,
one path — `cranelift_backend/planning/static_transition.rs` — `+198/-0`, clean
`diff --check`. M6 blob identity **1/1 MATCH**, path count equal to declared
scope. Current-main path intersection empty, so no rebase was owed.

**The node stays `active`, and this is the sixth merge on `#6d`.** It creates no
ABI arena, definition, emitter, or redirection, and credits **no emitter AC**.

**This is the emitter turn's first partial, and the split was the ring's own
call.** The turn was released whole; the implementer declined to start it on
capacity, and after a fresh turn the leader cut the selector out and shipped it
rather than stretch the turn or leave a half-built ABI class. **The ABI/emitter
class remains unstarted by construction, so more partials are expected before
`D2f` completes.**

### What it establishes

`fusion_redirect_target` selects the redirect edge **once, from the complete
key** — `invocation_caller`, `invocation_callee`, `invocation_callee_entry` —
and from nothing else. The `StaticBody` edge kind is validated **on the
survivor**, not used to pre-filter. Zero matches and more-than-one are
separately named errors.

**That ordering is the substantive design call.** Pre-filtering by edge kind
applies a criterion the key does not contain: redundant if the three members
already determine one edge, and silently resolving an ambiguity a redirection
may not have if they do not.

### The coordinate this deliverable used to require

The frame previously required redirecting a literal `StaticBody` edge
`0 -> 2`. **No edge of that shape exists on the checked twin** — its invocation
is caller 3, callee 2, and unit 0 is a `SchedulingEntry` that invokes nothing.
`0 -> 2` was measured on the retired `px8j` witness. The frame was amended to
state the derivation (PR #1913), and the candidate writes **no coordinate into
the derivation**: `3 -> 2` lives only in the control, where it is a measurement.
The Architect's resolution confirms it is kept control-only.

⇒ **The general form, since this node will cut more slices:** pin a frame
against the derivation, never against the number the derivation produced on
whichever witness was current. A number outlives its witness and stays
syntactically valid after it stops being true — nothing goes red.

**The scope loss was local to `D2f`.** `D2d-GROUNDING` records the coordinate
under "measured coordinate on this witness" and `D2e` says "do not re-derive
those coordinates; derive the mechanism that produces them". Both upstream
sources were correct; one restatement dropped the qualifier. No sibling frame
edit is owed — checked, not assumed.

### Non-vacuity, and the bound

The control's discriminator is written **before** its positive and is **per
member**: each invocation member is independently repointed at another identity
the same plan really contains — unit 2 a real caller, unit 1 a real callee,
origin 34 a real callee entry — and each repointing must refuse. That is the
right shape for a selector, because one matching on a **subset** of the key
would still pass a whole-key positive.

### The bound, split into its two halves — they cost very differently

**This was first written here as one item, "whether the three members are
jointly sufficient on any witness other than this one." That conflates two
questions** (adversary, `evt_2rzveprrs80p0`), and only one of them is open:

| half | state |
|---|---|
| a **subset** match would still pass a whole-key positive | **closed by construction** at this SHA — all three conjuncts are visibly present in the predicate |
| a proper subset would be **insufficient** to name one edge on some other witness | **open, witness-dependent, not answerable by reading** |

The predicate is the whole of it:

```rust
edge.caller()        == key.invocation_caller
    && edge.callee()        == key.invocation_callee
    && edge.callee_origin() == key.invocation_callee_entry
```

⇒ **The per-member controls are not what excludes a subset match today — the
code is. What they protect against is REGRESSION to a subset**, which is a
different and still-worth-having job. Saying "the controls guard joint
sufficiency" credits them with the wrong half.

**No coordinate leaked into the derivation.** The function body contains no
literal identity — no `PredeclaredFunctionId(N)`, no `StaticOriginId(N)`, no
bare `== N`; the only comparisons in it are those three. That was the specific
thing the frame amendment existed to prevent, and it is measured rather than
assumed.

### The open half and the ordering question need the SAME missing witness

The validate-on-survivor ordering differs from a `StaticBody` pre-filter **only
when the three members fail to determine one edge.** That is the same ambiguous
key the necessity half needs.

⇒ **One fixture with an ambiguous key would settle both**, and if the key is
provably unambiguous, **both dissolve together** rather than needing separate
answers. Do not scope two controls here. This is the adversary's observation and
it is the most useful thing on this record for whoever cuts the next slice.

### A claim chain that looked corroborated and was inherited

The per-member repointing description travelled: implementer's commit message →
my PR body and this node → the adversary, which **explicitly took my description
rather than opening the controls.** Three artifacts agreeing, one source.

**Now read directly at the merged tree:** the controls are as described — a
per-member table repointing `invocation_caller` at `PredeclaredFunctionId(2)`,
`invocation_callee` at `PredeclaredFunctionId(1)`, and the callee entry at
origin 34, each required to refuse. **Confirmed, and it was worth confirming**;
agreement among readers who share a premise is not corroboration of it.

## `D2f` ABI-only accepted partial — the fusion arena, MERGED 2026-08-11, PR #1922

Exact `6e60b3bf`, merge-base `14d410cd`, `origin/main` now `41cd949e`. Two
paths, `+739/-14`: `planning/static_transition.rs` and its new
`planning/static_transition/abi.rs`. Blob identity 2/2 against the declared
base. Decision `dec_3h20vrv3ngmsa`, QA `evt_477w8qsw9560s`, Architect
`evt_5g5d5mz5tmwbm`.

A separate fusion ABI arena and installer, the observable population repointed
to that arena, and the `AC-4` projected-input carrier gate.

### The ruling that let it land: un-wired is not half-wired

The ring asked whether this should go to QA or be held as WIP, and the question
was fair — both the implementer and I had previously named *a half-built ABI
class* as the outcome to avoid. **The hazard we named was a half-wired tree**: a
descriptor without an emitter, an owner without a redirected edge, a state where
some paths believe the fusion exists and others do not.

This cut is not that. It has **no production installer caller, no emitted
definition or body, no source authority, and no redirected edge**, and the
checked applied `Exact` twin still reaches its ordinary `ComputationalMatch`
refusal unchanged. An inert addition cannot create the inconsistent intermediate
state, because nothing consults it. ⇒ Routed to QA as a labelled accepted
partial.

**The second reason, which is the one that decides close calls:** landing it now
puts the `AC-4` carrier gate on `main` **before the emitter exists**. The
implementer's own argument for the ordering — a gate written after a working
emitter can be shaped to fit it, and this one cannot be.

### The un-wired premise was MEASURED, not inspected

Adversary `evt_28ndgecr5a6ms`, on `41cd949e`. My ruling rested on *"no
production installer caller"*, which is an enumeration, so it was checked
directly. `install_static_continuation_fusions` has exactly two occurrences: the
definition at `:13330`, before the test boundary, and its **sole call at
`:17170`, inside `mod tests`** — which opens at `:15223` under `#[cfg(test)]` at
`:15222`.

⇒ **Zero production callers.** The stronger reading is the correct one: not
"nothing calls it yet", but that no production path *can* at this SHA without a
new call site being added. The `AC-4` ordering argument is supported by the same
measurement — there is no emitter the gate could have been shaped around and no
production consumer whose behaviour could have been fitted to it.

**And the property expires by design.** The installer is `pub(in
crate::cranelift_backend)`, so un-wired-ness ends the moment any call appears
anywhere in that module tree, and nothing in the code marks that transition.
That transition **is** the emitter increment, so there is nothing to guard — but
it is worth writing down that this is a property true *at a SHA*, not by
construction. Do not cite this section as evidence about any later tree.

### What it deliberately does NOT do — a handover, not a defect

`install_static_continuation_fusions` reads the producer's declared operand run
via `key.producer_owner` and **enforces none of the three preflight
equalities**. Under the Architect's ruling those belong in the pre-definition
preflight, which is the emitter turn's scope.

The implementer flagged this itself (`evt_42y21cg6655k5`) rather than leaving it
to be found. **Recorded here so the next pass does not spend a turn deciding
whether it inherited a defect.** It did not.

### The emitter mechanism is ruled and unbuilt

Architect ruling `evt_79v3kj4nk2t3g`: an affine, compiler-only, move-only
`FusionRegionClaim` per installed fusion, derived from the complete production
key and immutable static plan — **never from witness coordinates**, which is the
same discipline Deliverable 5 above was amended to state.

The stop that produced it was real and is worth preserving: redirecting the
producer edge alone leaves the consumer suffix live and **executes it twice**,
because unit 3 has already installed the consumer `ComputationalMatchScrutinee`
continuation when it emits the producer call. The resolution is bounded — swap a
checked continuation prefix for its stored `next`, consumed once at the one call
seat. **A generic suppressed-origin or AST-excision facility is explicitly
out.** The implementer had been sizing that larger facility, and it was never
authorized.

**The three equalities hold on the canonical positive**, measured through the
production planner: `invocation_caller` 3 = `consumer_owner` 3,
`invocation_callee` 2 = `producer_owner` 2, `invocation_callee_entry` 37 =
unit 2's `body_occurrence`. So the ruling's load-bearing
`invocation_caller == consumer_owner` is a property this witness **has**, not a
constraint it fails — which is the difference between a preflight written
against a passing witness and one written defensively against a failing one.

## `D2f` claim-facility accepted partial — MERGED 2026-08-11, PR #1925

Exact `877fd731`, merge-base `10d5eda9`, `origin/main` now `cf1b36b4`. Two
paths, `+843/-1`, blob identity 2/2. Decision `dec_6js0bxbx5mqf7`, QA
`evt_609ejeyhcrnw`, Architect `evt_56kx6cvzk5yav`. Steward scope ruling
`evt_2pzeff27crgpz`.

The complete `FusionRegionClaim` facility ruled at `evt_79v3kj4nk2t3g`:
pre-definition preflight, affine ledger, set-equality closeout, controls. The
claim is non-`Clone`/non-`Copy`/non-`PartialEq` and derives only from the
complete production key and immutable static plan.

### The ruling: un-wired is a DIFFERENT AXIS from partial

The leader held before QA because the turn was authorized as *atomic
ABI/emitter construction* and this is not that deliverable — and because its own
release said *"do not leave a partial claim facility."* Both checks were right
to run.

**The instruction guards against a half-built mechanism** — a preflight without
a ledger, a ledger without a closeout, refusal rules present for some rows and
absent for others. What landed has all four pieces. **What is absent is the
wiring, and completeness and wiring are different axes.** This cut is the first
without being the second.

⇒ Landed as a labelled claim-facility accepted partial. Had the facility been
missing a refusal row, the answer would have been hold.

### The un-wired property is carried by DIFF SHAPE, not by a green suite

Three hunks, and only one touches pre-existing code:

| hunk | what |
|---|---|
| `+562` after `fusion_redirect_target` | the facility, contiguous, **zero deletions** |
| `+276` in `mod tests` | the controls |
| `abi.rs:1292` | `fn` → `pub(super) fn` on `fusion_input_carrier_admissibility`, plus a doc comment |

`FusionRegionClaim` occurs **7 times, all 7 inside its own definition block.**

⇒ **Because no existing code path changed, the checked applied `Exact` twin's
behaviour is unchanged by construction.** That is stronger than a green suite,
which is equally consistent with a behaviour change nothing observes. **The
reasoning is only as good as the hunk enumeration**, which is why the
enumeration is recorded here rather than the conclusion alone.

### The `pub(super)` widening is deliberate, and not scope creep

The installer applies the carrier gate before a slot is inserted; the preflight
applies the identical gate before any definition exists. **Two readings of one
function, never two spellings of one rule.** A second copy that drifts from the
first is a defect class this node has already produced. If a later turn needs a
variant, that is a scope question — not a second copy.

### Two things worth keeping

**Closeout is set equality across planned/defined/redirected/consumed, not
counts.** Counts hold vacuously at zero and also survive a swap.

**The preflight deliberately omits `producer_argument_binding.frame_origin ==
consumer_binding.frame_origin`.** It is false by design — 25 vs 10 — and
asserting it would refuse the canonical positive. Recorded as a **deliberate
omission** so a later reader does not "fix" it into a refusal of the very
witness the node is built on. It survived a compaction and a full review cycle
only because it was restated at every handoff.

### QA's evidence was a grid, not a count

Three mutations at three distinct sites, each preserving compilation, each
reddening a **different** control, each restored byte-identically: the
`caller == consumer_owner` conjunct, the three-domain overlap refusal, and the
planned-vs-defined set equality. Each moved exactly one field of the expected
output. **That discriminates which rule is load-bearing** rather than only
showing that something fails.

### The located seam — WITHDRAWN 2026-08-11, and what replaces it

**The coordinate below was wrong about which unit raises the refusal, and it is
withdrawn.** It is kept as written because the next turn was told to inherit it,
and a reader who acts on it must be able to see that it was retracted rather
than merely absent.

> The `Exact` path still reaches `boundary_transfer_admissibility`'s
> `ComputationalRecursorClosure` refusal, so the takeover must be located
> **before that refusal**. The ruling puts the takeover at the producer-call
> return; this says the ordinary path refuses upstream of it. ⇒ Incomplete
> about where, not contradicted.

The Runtime leader measured it (`evt_1xjz1y6qgznv7`): on applied `Exact`,
`transfer_into_carrier` sees **one** refusal at origin 31, and it is raised by
**unit 2's own body**, which constructs `Node{ComputationalRecursorClosure}` and
transfers it across unit 2's own boundary. The claimed edge `3 -> 2 @37` is unit
2's sole incoming invocation, but `plan.executable_units()` returns `[0,1,2,3]`
with entries `[5,41]` — so redirecting the call and taking over the consumer
continuation leaves unit 2 declared, defined, and still refusing.

⇒ **The takeover does not reach this refusal at all.** The seam named above is
real for the double-execution problem the Architect ruling addresses; it is not
the seam for this one. Do not spend a turn making it fit.

### The producer-suppression question — scope ruled, mechanism to the Architect

Steward classification `evt_1qprfdz1h97ys`, in reply to `evt_1xjz1y6qgznv7`.

**Scope: bounded completion of `D2f`. No re-cut, no new node, `#6d` stays
`active`.** The fused definition subsuming both bodies is the deliverable. A
fusion that leaves the producer's body defined and refusing has not fused
anything, so *"after fusion, unit 2 must not receive an independent body"* is
inside the ruled outcome rather than beyond it.

**Mechanism: the Architect's, routed by the ring directly** (`COORDINATION §14`,
any team to Architect for component design). The suppressed-origin prohibition
at `:654` above is the Architect's own sentence, and narrowing another lane's
prohibition is not available from the scope lane; and the answer changes what
gets emitted.

**The question must not be asked on the axis the leader first proposed.** *"May
a producer with a sole claimed emittable invocation cease standalone emission"*
keys the predicate on **call edges**, and the landed code refuses that axis in
writing.

Coordinates below are in `crates/ken-runtime/src/cranelift_backend/planning/`
`static_transition.rs`, read from the **git object**, not a worktree.
`cf1b36b4` and `28bed66a` carry the identical blob `7ba173e7` for that file, so
one set serves the ring's branch and `main` alike.

| coordinate | what is there |
|---|---|
| `:13566` | `executable_units` already narrows `emittable_units` by `template_only_worker_bodies` (`:13473`), probing `unit.body_occurrence()` |
| `:13596` | *"reading it here would ask an executability question with a call-identity key ... executability is a function of the body alone"* — restated at `:14251` |

**Two wrong coordinate sets were published before these, and both resolve to
real unrelated code** (`evt_e6q5z241x98v` corrects them). The Steward's set was
read from the main repo checkout at `f8f8bfbc` and then labelled as measured on
`28bed66a` — a tree qualifier that named neither the tree read nor the reader's.
The leader's set was the `10d5eda9` base, exactly 562 lower, which is the `+562`
D2f hunk. Nothing errored in either case; that is the hazard. **Read a
coordinate from `git show <sha>:<path>`, and state the SHA you actually read.**

These are cited, **not ruled to cover the case** — that measurement is the
ring's. They fix the shape of the question: ask on the **body axis**, and ask
whether a landed narrowing already spans this case or a new one is needed.

That distinction is also what separates the need from the prohibition. What was
ruled out is a **generic** suppressed-origin or AST-excision facility, which is
the larger thing the implementer had been sizing. Using or extending an existing
ruled narrowing is a different object. Whether it is *this* narrowing is the
Architect's to say, and the ask should surface **the need with the vehicle left
open** — a bundled mechanism anchors the owner, and its rejection then reads as
"the need cannot be met" when the owner can usually meet it more cheaply from
inside their own lane.

### The completeness premise is now VERIFIED, not just asserted

The adversary (`evt_4nyse2f1rs30k`) re-measured un-wired-ness rather than
carrying the `6e60b3bf` verdict, and **the evidence moved even though the
conclusion held**: installer occurrences 2 → 3, call sites 1 → 2 (`:17732`,
`:17843`), and the `mod tests` boundary shifted `:15223` → `:15785`. Both call
sites are inside the test module. **A carried verdict would have asserted "one
test call" about a tree with two.**

It then named the load-bearing gap correctly: **the completeness half is the
actual premise of the Steward ruling, and nobody had tested it.** Measured on
`cf1b36b4`:

`FusionClaimRefusal` declares **eight** variants. Seven are constructed in
`fn preflight` (`Identity` at three sites, plus `InvocationTriple`,
`SelfRedirection`, `BinderAgreement`, `InputAvailability`, `ResultLane`,
`OverlappingClaim`). Each ruled row maps to an applied gate.

**`SelectorEdge` is the eighth and production never constructs it** — which is
the "stated but not applied" shape, and it is not that. The preflight delegates:

```rust
// `redirect_target` raises its own absent/ambiguous/declaration-kind refusals
let redirect = view.redirect_target(plan)?;
```

The ruled row — one unique landed `StaticBody` edge — **is enforced**, by the
`D2f` Deliverable 5 selector that landed at `e89de667`, which already refuses on
zero and on multiple. `SelectorEdge` is a **reporting label for a delegated
family**, and both the production comment and the test helper say so.

**The control keys on production's real messages, not on a name production never
emits.** The helper matches `"no edge to redirect"`, `"selects more than one
emittable"`, and `"rather than a static body edge"` — three actual
`fusion_redirect_target` messages — and normalizes them to the label. So it is
not a control whose answer its own helper supplies.

⇒ **No declared-but-unapplied join. The facility is complete, and the ruling's
premise is measured rather than inherited.**

**One residual, direction stated:** that normalization is keyed on message
substrings from another function's prose. If `fusion_redirect_target`'s wording
changes, the helper falls through to `"other planner invariant: {message}"` and
the control **reds**. That is the safe direction — a false negative that shouts,
not a false positive that hides — so it is recorded, not filed as work.

**Still unhunted by everyone:** the individual correctness of the preflight
joins, and whether each refusal names a real identity. Whether
planned/defined/redirected/consumed are the four sets that matter cannot be
settled until the emitter exists, because `consumed` has no content before then.

### This acceptance is NOT precedent for the emitter cut

Un-wired-ness ends at the first production call anywhere in the module tree, and
nothing marks that transition. **The wiring turn is the first `D2f` increment
that changes production behaviour, and it is reviewed on its own terms.**
Neither this partial nor `6e60b3bf` is evidence about the wired tree.

## `D2f` producer-side atomicity partial — MERGED 2026-08-11, PR #1933

Exact `a656fca1`, declared merge-base `cf1b36b4`, `origin/main` now `62f2931b`.
Four non-merge commits, three paths, `+733/-26`, **blob identity 3/3** enumerated
from the declared base. Decision `dec_231jk98fca7kb` verified `resolved` at merge
time by reading the object. Architect `evt_4rq7d58nckemc`, QA
`evt_34465chdhpkyf`. Steward scope ruling `evt_4p4zbdwd34976`.

It implements the Architect's body-axis ruling `evt_6qnwm7qz1a16t`: typed body
ownership keeping `ContinuationTemplate` and `FusionOwned` distinct, atomic
exact-once installation through an affine ledger, and zero-region controls. Two
of the three paths are re-export plumbing only — five inserted lines total
carrying the new types up to the backend module.

### The ruling was on a DIFFERENT SHA, and the premise was re-taken

**The scope ruling `evt_4p4zbdwd34976` was cut against `21455ec4` at `+474/-26`.
What published is `a656fca1` at `+733/-26` across four commits.** The reasoning
transfers — same producer-side body-axis machinery, still un-wired — but the
ruling's load-bearing premise is a claim about a *tree*, so it was re-measured
rather than carried.

Measured at the merged SHA and again on `main`: `install_fusion_owned_bodies` is
defined at `:13726` with **eleven** call sites, `:18168` through `:18689`, and
`mod tests` opens at `:16097`. Every call site is after the boundary.

**That census was the weaker check, and the Steward said so at the time rather
than letting it ride.** Zero production callers of one named function proves the
map is empty only if that function is the *sole* way an entry can enter it — a
constructor, a `Default`, a builder, or any direct field mutation would be
invisible to a caller census.

### CLOSED on the write set, and structurally — adversary `evt_5p4mrr5begwyg`

The complete write set of `fusion_owned_bodies` is **two** sites: `:10864`, the
constructor, initialising it **empty**; and `:13846`, a whole-map assignment
inside the installer. The field is declared at `:2680` and is **private, no
`pub`**.

⇒ **The map cannot acquire a `FusionOwned` entry in production**, so the
disposition filter is the identity and the inertness argument holds — now
established on the property the argument needs rather than on the one first
measured.

**The privacy is what closes it, not the grep.** A private field's write set is
bounded by its module by construction, so this survives edits elsewhere in the
crate; a caller census is silently invalidated by the next call site anyone adds.

**A narrowing on the atomicity question, explicitly a shape fact and not a
verification:** `:13846` is a whole-map *assignment* from `scratch`, not an
incremental insert into `self`. A single move-assignment cannot half-populate, so
a half-install would have to arise inside `scratch`'s construction and then be
assigned wholesale. That is a narrower failure mode than a partially-mutated live
map, and it remains unhunted.

### The acceptance property CHANGED shape, and this is the trap for the next reader

The `877fd731` claim-facility partial rested on *"nothing consults this"* — an
inert addition no production path reads. **That is not the property here, and
citing it would be wrong.** The executable-unit population **does** consult the
disposition map on production paths. What makes this cut inert is only that the
map is never populated, so the filter is the identity.

⇒ **The right question for this partial is "empty map, filter is identity,
behaviour unchanged", not "zero production callers of the consumer".** A caller
census on the *reader* side answers the wrong question and passes. The census
above is on the *writer* side, which is the one that decides.

### Third un-wired partial, and the expectation set for the next one

`6e60b3bf`, `877fd731` and now `a656fca1` have all landed without changing
behaviour. Each was individually right and none is reversed. **The Steward
expectation, stated at `evt_4p4zbdwd34976`, is that the next `#6d` candidate is
behaviour-changing**; a fourth un-wired cut brings its reason to the Steward
*before* the candidate, because that is a sizing question about the node rather
than a scope question about the cut.

The wired shape is measured and its gap is named — see the producer-suppression
section above. Node stays `active`; no emitter AC credited.

## `D2f` emitter — the full chain, INERT. MERGED 2026-08-11, PR #1940.

`822351670a000bea16c4e638c60cce4feec9352f`, declared merge-base `e0246b08`. Two
non-merge commits, seven `ken-runtime` paths, `+1085/-15`. Decision
`dec_8ssptk582gvg` read `resolved` from the object; QA exact approval
`evt_6ka0d4pxmfy7`. Blob identity verified 7/7 against `origin/main` from the
**declared** merge-base, and the arming constant confirmed still `false` **on
the landed tree**, not merely in the candidate's description.

**`bd5961f8` is superseded and must not be merged.** It was the first cut of
this candidate and is referenced below only where the measurements were taken on
it. The second commit is the correction described under "the overclaim" below.

**Read this section before proposing an emitter cut.** It is the fourth
un-wired partial and it is a different object from the first three.

### Why it is inert, and why that is NOT the capacity story again

`6e60b3bf`, `877fd731` and `a656fca1` were each inert because a seat ran out of
capacity. **This one is inert because of an unanswered design question.** The
distinction is load-bearing: capacity-blocked machinery can be regenerated by
another turn of the same kind, and therefore rots when it is kept. This chain
cannot be regenerated that way, and it is the instrument that makes the open
question askable at all.

> ### THE GATE COVERS ONE STAGE, NOT THE CHAIN. Corrected 2026-08-11.
>
> **This section said "arming is one line" and that the ordinary production
> path is behaviourally unchanged. Both overstate what the constant does**, and
> the sentence is the Steward's. Adversary `evt_7v8j1ntwv7rz2`, verified from
> the object at `98702040`.
>
> The `if` at `:2210` **closes at `:2213`.** Running unconditionally on every
> production compile, outside it:
> `FusionRegionClaimLedger::preflight` (`:2214`),
> `install_fusion_owned_bodies` (`:2215`, **`?`-propagated**), and
> `define_static_continuation_fusion_bodies` (`:2486`). There is exactly **one**
> non-comment read of `D2F_EMITTER_ARMED` in the file and it guards the
> installer alone.
>
> ⇒ **The chain is inert by EMPTY INPUT, not by the gate.** The later stages
> execute and iterate nothing, because the one gated call is what would
> populate them.
>
> **This is the same substitution this node refused one cut earlier**, at the
> `a656fca1` acceptance: inertness resting only on a map never being populated
> makes the filter the identity — a weaker guarantee answering a different
> question. It recurred one layer up, and **the presence of a `false` const
> made it read as the stronger guarantee.** Reading the constant is not reading
> its extent.
>
> **The remedy first routed here — "extend the gate to cover `:2214`, `:2215`
> and `:2486`" — is WITHDRAWN, `evt_6htz44wp8pnkx`. Do not do it.**
> `core.rs:2163-2164` records that the arrangement is deliberate: the chain is
> **not** guarded on `is_empty()` because *"a guard would make the zero case
> take a different path from the one the non-zero case exercises."* A
> whole-chain gate destroys exactly that property. The instruction would have
> had the ring undo a correct decision to make the code match a Steward
> summary.
>
> **The code was right at the site the whole time.** `core.rs:2165-2172` states
> it precisely — everything downstream is *"built, compiled and **reachable**,
> and every one of them is inert because `continuation_fusions()` is empty on a
> plan with no installed plane"* — and `control.rs:4074-4079` agrees. Adversary
> `evt_27q1fqr5m66n3` closed the population: three mentions of the constant,
> all read, **two correct**.
>
> ⇒ **The defect was never in the code.** It was in prose written away from the
> site by a reader who did not consult the comment at the site. Reading the
> constant is not reading its extent, and the comment that would have said so
> was in the same screenful.
>
> **What is actually owed, and it is smaller.** `units.rs:2889` is the one
> genuinely wrong statement — it attributes inertness to the gate rather than
> to the empty population, in a comment doing safety work. **Fix that comment;
> change no code.**
>
> **The substantive open question, unchanged by any of the above:** `:2215` is
> `?`-propagated, so refusal paths that were unreachable before this merge are
> reachable on every production compile and merely do not fire. Arming
> therefore changes those stages' *data*, not their *reachability*.
>
> **Still unverified, and cheap:** that the three stages are observable no-ops
> on empty input. CI green on the corpus is not that proof. If they are not,
> this stops being a description defect and becomes a behaviour one — that is a
> stop, and it returns to the Steward.

The arming constant is `const D2F_EMITTER_ARMED: bool = false` at
`lowering/core.rs:2209`, guarding the `if` at `:2210`, with the instruction to
flip it at `:2171`; the control at `lowering/core/tests/control.rs:4075` names
the inertness in place. **Those coordinates are read from `origin/main` at
`98702040`, after the merge** — they were `:2206`/`:2207` on `bd5961f8`, and the
second commit moved them by three. Census rows
moved and are labelled where they moved: `lowering/units.rs` 4 to 5
builders/definitions and 3 to 4 declarations — a fifth *emitting function
class*, not a fifth copy of an existing one.

**Steward scope verdict `evt_1mdw894g79ed6`: the surplus is ACCEPTED and must
not be cut back to the literal fallback.** The pre-authorized fallback was
production export plus bundle map/declaration *only*; what landed is that plus
the fused definition, redirect, input supply, takeover and closeout. Deleting
the surplus would delete the measurement that locates the open question. **This
consumes the fourth inert cut. There is no fifth — the next turn on this node
is a wiring turn, gated on the Architect's answer rather than on capacity.**

### The overclaim, and why the second commit is the better half

**The Architect's ruling did not only answer the open question — it exposed a
false statement in `bd5961f8`'s own durable contracts.** Those contracts
described the causal-authority arrangement as though it were the per-phase one
the ruling requires. It is not: the code binds a **single provisional
`Predeclared(producer_owner)` across the whole combined lowering**, and the
producer-to-consumer-to-producer switch at the claimed suffix seam does not
exist yet.

The second commit corrects that in place rather than leaving it. It states the
provisional authority truthfully, keeps `Fusion` as region/definition identity
only, and **explicitly excludes** the later authority switch and the
fusion-specific checked-frame adoption that ruling `evt_4vqey13cxxjqs`
requires. The direct `bd5961f8..82235167` delta is two paths, `+70/-27`, with
**zero added executable lines and exactly one removed code line** — an unused
`ContinuationEmissionOwner::Fusion(fusion.id)` local. Every other machinery and
control path is blob-identical; the arming gate and installer are untouched and
no ignores were added.

> **This is the failure mode this node has hit before, caught early for once.**
> A comment or contract that describes the mechanism the author *intends* is
> structurally exempt from execution — no test fails when it drifts from the
> code. Landing prose that reads as done, on machinery that is inert and will
> be picked up by a different seat after a compaction, is precisely how the
> next reader arms something that was never wired. **The correction is worth
> more than the 1042 lines it annotates.**

### What arming MEASURES — four refusals cleared, and where it stops

Armed against the `Exact` witness, the chain clears four successive distinct
refusals and stops at a fifth:

1. `ComputationalMatch: a computational recursor closure names an in-flight
   activation` — **the baseline**, in the producer's standalone unit. Body
   ownership removes that unit, so the refusal is gone. This is the 0 to 1
   movement `#6d` asks for.
2. `callee frame is missing a declared input` — the redirect is live and the
   fused frame's continuation inputs are unsupplied. Closed by
   `Lowering::fused_redirect_inputs`.
3. `the claimed continuation target was not declared into this function` — the
   fused body lowers; the producer's causal refs need declaring in it. Closed.
4. `a continuation call token was claimed by a context that is not its emission
   owner` — the ambient emission owner must be the body's own predeclared owner
   per phase. Closed.
5. `OrientedSubcontinuationPlanV1: computational IH slot marker is detached
   from its checked frame` — **where it stops.**

The consumer half is measured correct: the takeover fires **exactly once per
compile**, at `StaticOriginId(10)` in `PredeclaredFunctionId(3)` with seat
`StaticOriginId(37)`, and unit 3's body then completes. Redirect, ownership
install and takeover work *together* — the irreducible core of `:650` is built
and behaves.

The continuation inputs are the consumer's own entry-frame parameters
(`EntryAbi{source_owner: 3, position: 0/1, Parameter}`, both `ValueWord`),
resolved through the landed `verify_predeclared_entry_frame_membership` gate
rather than a rule respelled at the seat.

### The two design questions — BOTH RULED, `evt_4vqey13cxxjqs`

> #### THE ANSWER, and it is what the next turn builds. Durable at `51e6a266`.
>
> **1. Adopt the consumer's checked frame; do not re-home the marker.** The
> fused `Function` opens its **own fresh** `CheckedFrameFunctionScope`, then
> locally re-enters the **same consumer frame identity** already carried by the
> preflighted `FusionRegionClaim.checked_transport`. The original consumer and
> the fused function therefore each have an **independent per-`Function`
> consumption transaction** — this is the `D8n` split-body law, **not** one
> transaction spanning two functions, which is where the question was
> mis-posed. Do not carry `active_subcontinuation_frame` across a function
> boundary, mint a new frame for `Fusion(id)`, or accept a raw frame id from a
> scan. Expose only a fusion-specific adoption seam taking the claim's complete
> resolved transport coordinate; `CheckedFrameFunctionScope::open` and its
> ordinary callers stay unchanged.
>
> **2. `Fusion(id)` is region/definition identity, NOT causal-token authority
> in this cut.** Keep the planner-issued source owners. Producer source lowering
> runs under `Predeclared(producer_owner)`; when the producer dispatcher enters
> the claimed consumer suffix/case body, **that phase runs under
> `Predeclared(consumer_owner)`, then restores the producer phase.** No causal
> token is re-homed or newly issued under `Fusion(id)`. The fused definition's
> `FuncId`, the fusion claim/definition ledger identity, and source causal
> emission authority are **three separate axes**.
>
> Project and declare **only the exact planner-issued token subset that moved
> with each phase** — do not union all consumer tokens, and do not reach for a
> generic suppressed-origin or AST-excision facility. If the existing ledger
> cannot express that exact moved subset, it must **refuse and expose the next
> bounded planner seam**, never silently relabel tokens as `Fusion`-owned.
>
> **Five causal controls are required before arming:** the original consumer and
> fused function each consume the same checked frame once under distinct emitted
> `FuncId`s, with the fused body reaching the checked IH slot/call relation; a
> second activation in the same fused function refuses while the same checked
> frame across functions accepts; wrong consumer, wrong continuation occurrence,
> or any displaced member of the checked-transport coordinate refuses **before**
> definition/redirect/takeover commit; producer phase under consumer owner,
> suffix phase under producer owner, or either under `Fusion(id)` each refuse
> independently while the exact pair succeeds; and ordinary zero-fusion and
> unarmed paths stay byte-for-byte inert.

Both were routed directly by the ring per `COORDINATION §14`; the Steward is
not the relay. **The questions as originally posed are kept below**, because
the ruling corrected the *shape* of the first one and a reader who sees only
the answer will not know that.

1. **Step 5.** A `CheckedFrameFunctionScope` is a per-`Function` transaction,
   and a fused region is the first construct whose checked frame spans two
   `Function`s — the marker is minted where the consumer's frame lives and
   consumed where the fused body lives. Whether the fused body adopts the
   consumer's frame, or the marker is re-homed to the fused region, is an
   Architect call. **Nothing in this node settles it**, and every way to force
   it past is a widening of authority the node rules out.
2. **The ambient emission owner in the fused body.** The implementer chose the
   body's own `Predeclared` owner per phase rather than `Fusion(id)`, and
   flagged it as a decision made rather than found — which is what makes it a
   design question. `Fusion(id)` is what the region *is*, but the planner
   issues no `Fusion`-owned causal tokens, so requesting them returns the empty
   set and the producer's first causal call refuses. Read strictly, the
   alternative is that **the planner owes re-homed tokens**, which is planner
   work that does not exist. Same seam as step 5; ask them together.

### Two measurements that outlive this candidate

**The natural way to write the fused body is the defect.** Lowering the
producer to a value and eliminating afterwards is the obvious shape, and it
reaches the producer's own "names an in-flight activation" refusal *inside* the
fused function — the identical refusal fusion exists to remove, merely
relocated. The producer's body has no value representation to hand anywhere;
that it does not is the whole reason the region is fused rather than called.
The eliminator has to go **on the stack**, with the producer dispatcher
consuming it. This was measured by building the wrong one first.

**The definition pass must run BEFORE the ordinary bodies, and the ordering is
forced rather than stylistic.** The takeover consumes the claim, and the
takeover happens inside the consumer's body — a definition pass placed after it
finds the claim spent and cannot read the region's authorities at all. Unlike
the context and continuation passes, this position is not a readability choice.

## Carried rider — the `D2a` control's durability. Owed, not optional.

**DISCHARGED 2026-08-11** by the `D2f` ABI-class partial above, which carries
`docs/program/wp/RT-LEXICAL-RECURSOR-CONSUMERS-D2a.md` among its six paths.
Retained below as the statement of what was owed and why.

**Land this with the next candidate that touches `control.rs`.** It is a rider,
not a deliverable, and it does not earn its own node. It is recorded here
because a rider stated only in a thread strands.

Adversary `evt_xyj8813ymrad`, Steward disposition `evt_3k5eg1trmw2q9`,
independently re-derived at `origin/main` before it was routed.

In the `D2a` arm, `arrivals` is incremented at `core.rs:6686` and `forwards` at
`:6696`, and **between them there is no branch, no fallible step, and no early
return.** In the non-suppressed leg every arrival forwards by construction, so
`assert_eq!(forwards, arrivals)` is decided by the test's own suppression flag
and by nothing about the mechanism.

⛔ **The hazard is not the tautology, it is that the tautology passes at
`0 == 0`** while looking like the stronger of the pair. A later trim that keeps
the equality and drops `arrivals > 0` leaves the whole control vacuous and
green, with every `!contains(R1)` holding because the marker never arrived —
the exact defect class the control was written to avoid. Both counters are
`#[cfg(test)]`, so a pass removing test-only machinery from production `core.rs`
has a motive to touch this precise block.

> **The property, stated so the route stays with the ring:** a later trim must
> not be able to retain the half that passes at zero. Label it, or make the two
> inseparable so there is no half to keep. Durability by instruction or by
> construction — the Steward leans to construction where it is clean, and that
> is a preference, not a ruling.

**Do not change the predicate to make the equality informative** — no failure
mode is available between those two increments, so it cannot be made
informative, only labelled or fused. **Do not pin a fixed count.**

**The same candidate corrects the `D2a` record's own sentence**, *"asserted as a
relation (`forwards == arrivals`, `arrivals > 0`), never as a fixed count"*,
which lists the two as equals and is where the mis-weighting was taught. Fixing
the code and leaving the record reproduces the finding one layer up, which is
the failure this lane has already had twice.

Severity is **control durability, not correctness.** `D2a` merged correctly at
`41b75c7c`; this reopens nothing.

## Carried rider — `D2j`'s non-degeneracy: two groups assert it, five claim it

**Adversary finding `evt_99agje0m3rx1`, measured on `22fb3a61` after `D2j`
merged. Confirmed. It does not reopen `e2907c5e`.**

`D2j`'s matrix closed seven member groups, and `D2j`'s own `AC-1` requires each
row to rest on a **reaching non-degenerate witness** — "an empty vector, a
single-element set, a `None`, or a value that coincides with its neighbour is
degenerate, and a row resting on one is not discharged."

**Measured across the 72 added assertions: two groups assert a non-degeneracy;
five state one in prose.** Of six cardinality pins, four are `len() == 1`
uniqueness pins, which are a different property. The two real guards are
`:438` (`ih.len() == 2`, "so neither lookup is forced") and `:639`
(`widened_args.len() == 2`), and **those are precisely the two rows where a
degeneracy was already caught** — the IH lookup, and the one-child producer
construct the Architect found.

⇒ **The population was the class and the repair took the instances.** All seven
groups share one witness, so a cardinality that collapses a distinction for one
row can collapse it for another. Nothing about either fix generalises to the
five groups nobody looked at. The prose claim for those five is the exact state
the producer-construct row was in until the Architect caught it.

### This is one read per group, not five assertions

**The Adversary's bound is carried and is load-bearing:** it searched for
cardinality pins only. A group could establish non-degeneracy by an
`assert_ne!` between two candidate positions or by a distinctness check, and
that search would not see it; and a member whose authoritative fact does not
depend on cardinality needs no such guard at all. So the honest claim is *five
groups carry no cardinality assertion*, **not** *five groups are unguarded*.

| finding for a group | action |
|---|---|
| no non-degeneracy established, and the fact is cardinality-sensitive | add the guard in `:438`'s form — the count **plus** what it buys |
| established by a different instrument | record where, and the group is done |
| the fact does not depend on cardinality | say why, and the group is done |

**Adding an assertion to a group in the third case is worse than the gap** — it
is a control that cannot fail, which is the failure this lane has now filed
against itself twice.

**`:438` is the model to copy:** assert the count *and* state the reason in the
message. A bare `len() == 2` with no reason is the next thing to rot.

Severity is **evidence completeness, not correctness.** `D2j` merged correctly
and its three deliverables are discharged; this is inherited by the next `#6d`
slice frame.

## `D2k-1e` — the refusal-phase census, MERGED 2026-08-15 at `00fad9da9`, PR #2284

**An increment inside `#6d`. It does NOT retire a residual class**, and nothing
about the campaign's remaining scope moved. Measured on `main` after the merge:
`enum RecursiveDescentResidual`
(`crates/ken-runtime/src/cranelift_backend/lowering/core.rs:1979`) still carries
**`MatchScrutineeRecursor` and `LexicalCallArgumentRecursor`**. Three classes
are retired; these two are not, and [[RT-RECURSOR-TRANSPORT]] owns them.

Candidate `152a7d13a`, declared base `301b7af20`, two `ken-runtime` paths,
`+77/-35`. Blob identity on both paths against `origin/main`: MATCH. QA
`evt_56jz5wbsmsdrh`; Architect `evt_6tmzxsc6684gc` resolving `dec_6q652b4y857pw`.

**What it establishes.** The three tier-3 rows now assert
`(arrivals, validator_admitted, named authority) == (0, false, true)`, which
separates the **phase** of a refusal from its **reason** — the outer
pre-builder validator and the builder's duplicated validator produce the same
named refusal, and before this the rows could not tell them apart.
`with_match_recursor_census` owns only `MRC_CENSUS` and restores nested prior
state; `compile_cause` drains only `D2F_GATE_ARRIVALS`; the nesting yields one
census row without cross-draining. Production feature-off behaviour is
unchanged — the production delta is one line.

The mutation proof is real and was run in both seats: bypassing only the outer
validator while leaving the builder's own validator intact flipped
`validator_admitted` to `true` and reddened exactly the three rows on the phase
tuple, with `core.rs` restored byte-identical afterwards.

> ### CARRIED, NOT CLOSED — the initialization value conflates two states
>
> **The Architect's non-blocking finding on this candidate, deliberately
> excluded from it and owed by the duplicated-validator node.**
> `validator_admitted` is `false` **as its initialization value**, so `false`
> means *"the outer validator refused"* **and** *"the outer validator never
> ran"* — one bit for two facts.
>
> The rows red if the outer validator admits, and they **pass if it is removed
> entirely.** ⇒ **The control cannot see its own subject being deleted**, which
> is the same shape as
> [[RT-SRCMACHINE-DISPATCH-REACHABILITY-CONTROL]] and is why that node exists.
> Do not treat the three-state repair as folded in here; it is not.
>
> ### CONFIRMED BY MECHANISM, and it re-scopes the repair. Adversary `evt_582zrq09zr1ft`.
>
> **It is not a worry about initialization semantics — it follows from row
> creation and outcome recording being two independent calls:**
>
> ```rust
> rows.push(MatchRecursorCensusRow {
>     … validator_admitted: false, reached_selector: false, authority: None });  // :1236
>
> fn mrc_census_validator(index: Option<usize>, admitted: bool) {                // :1246
>     let Some(index) = index else { return };
>     … row.validator_admitted = admitted;
> }
> ```
>
> Deleting the outer validator deletes **only the `mrc_census_validator(..)`
> call**. The row is still created, the flag stays at its birth `false`,
> `census.as_slice()` still destructures as `[row]`, and the tuple reads
> `(0, false, true)`. **The three rows pass.**
>
> **WIDER THAN ONE FIELD — scope the repair to the row's INITIALIZATION
> CONVENTION.** The row is born with **three** absence-valued fields:
> `validator_admitted: false`, `reached_selector: false`, `authority: None`. The
> tuple reads only the first today, so **any future row asserting
> `reached_selector == false` or `authority.is_none()` inherits the identical
> conflation.** A one-field fix lets the next assertion re-acquire the blindness
> for free.
>
> **A second silent route to `false` that the destructure does not catch.**
> `mrc_census_validator` opens `let Some(index) = index else { return }`, so even
> with the validator present a `None` index makes the outcome call a **no-op**.
> The `[row]` destructure catches *no row*; **nothing catches "row created,
> outcome never recorded."**
>
> **NARROWER IN WHAT IT LEAVES STANDING — and this bounds the repair.**
>
> | member | what it can distinguish |
> |---|---|
> | `arrivals == 0` | **not phase** — the assertion's own message says a zero alone cannot tell which phase fired |
> | `named_authority == true` | **not phase** — both validators emit the same named refusal, so it holds under either |
> | `validator_admitted == false` | **phase, and it is the blind one** |
>
> ⇒ **The tuple has exactly ONE discriminating member, and that member cannot
> see its own subject being deleted.** That is stronger than *"one of three is
> weak"*. **Do not add a fourth member** — the other three are already
> non-discriminating for phase. The repair is to make `validator_admitted`
> separate *ran-and-refused* from *never-ran*, which the birth-value convention
> currently prevents.
>
> **The landed mutation proof is the other half and remains correct.** Bypassing
> the outer validator while leaving the builder's intact flips the flag `true`
> and reds the three rows — **that establishes the ADMIT direction. The DELETE
> direction has no observation, and now has a mechanism for why.**
