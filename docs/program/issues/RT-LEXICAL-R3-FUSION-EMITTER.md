---
id: RT-LEXICAL-R3-FUSION-EMITTER
title: "Row 5's before-hole expression is the one member of the eight-expression lexical-recursor population whose lawful repair requires static-continuation fusion -- it is carved out of RT-LEXICAL-RECURSOR-CONSUMERS together with its repair and discriminating-control obligations, because leaving the expression in the parent while moving the machinery would give the parent an AC it cannot discharge"
status: merged
owner: runtime
size: M
gate: none
depends_on: []
blocks: [RT-RECURSOR-TRANSPORT, RT-LEXICAL-RECURSOR-CONSUMERS]
github: null
origin: Steward re-cut of RT-LEXICAL-RECURSOR-CONSUMERS after its AC reconciliation returned 0 of 8 discharged across eleven merged D2f partials (runtime-leader, evt_d9n0twj6y5sp). Separability ruled by the Architect at evt_7knsqyqg72103 on a Steward question (evt_2vbhz9kgp0b44). Steward-filed (agents cannot create tracked work per COORDINATION 2).
---

> # THE CUMULATIVE STOP FIRED AT `D2` — RECUT BEHIND `DP`, NOW RELEASED
>
> **Architect ruling `evt_1q7v9fcw5hd87`, and the recut is landed.** Runtime is
> on this node as of 2026-08-12, `main` = `1f578a70`; the ring was held on
> `#6d` `D2k` until `D2k-1d` merged. **Resume from the recut frame, whose entry
> point is `DP` in section 5**, never from a memory of the pre-stop deliverable
> order.
>
> **What `D2` proved, and it is a real result.** The A/B is valid causal
> evidence: with `D2` off the suffix is outside its checked frame and the IH
> marker refuses as detached; with `D2` on the existing consumer re-enters and
> consumes the transported consumer frame, so lowering advances to the next
> independent guard. **`D2` closes the gap it was designed to close.** It did
> not cause the newly exposed gap and does not settle it.
>
> **What it exposed.** `compose_oriented_subcontinuation` builds
> `semantic_layers` from the selected producer layer plus pending unwind
> layers. Once **any** pending semantic layer is checked, **every** pending
> semantic layer must carry both an exact checked frame and an invocation
> identity. The producer eliminator here is still `semantic_pending` — **it is
> not a control-only wrapper that may be omitted** — so the new refusal is
> correct for the current representation.
>
> **Two repairs are ruled UNLAWFUL and are not to be re-proposed.**
>
> - **A fusion-only validator admission.** It would report one segment as
>   checked while one of its semantic frames has no plan row, no invocation
>   identity and no checked interface or witness. Segment checkedness is
>   **segment-wide**; there is no existing proof boundary that could confine
>   the exception to the consumer suffix. It would also weaken the same
>   invariant that correctly refuses an ordinary inferred IH layer nested in a
>   checked application. **Calling the case "fusion" does not supply the
>   missing authority.**
> - **Copying or inferring the consumer identity onto the producer.** They are
>   distinct semantic occurrences; aliasing them makes one checked identity
>   answer for two frames. Deriving a producer identity from body shape, origin
>   coincidence, sole remaining plan row, or the fusion claim is **inference**,
>   and this boundary deliberately accepts **transported** identities only.
>
> **Operational disposition.** `8063dd67...7166baaa` is preserved as held
> `D1`+`D2` **evidence only**. Not a merge candidate, must not route to QA. **No
> arming commit, no `AC-8` credit, no `D4` credit, no standalone `D1`/`D2`
> merge is authorized.**
>
> **Owed by any successor:** correct the stale arming comment. **After `D2` the
> live refusal is the mixed checked/inferred semantic-frame guard, not the
> prior step-5 refusal.**
>
> **The recut is in section 5 of the frame.** The retirement still needs this
> expression — `RT-RECURSOR-TRANSPORT` `D3` must prove all six rows green
> **without** exclusion, and new `#[ignore]`, fixture reshaping and refusal
> absorption are each separately ruled out for exactly these rows. **Descoping
> it is not available.**
>
> **RELEASED AND `active` — 2026-08-12, `main` = `1f578a70`.** It was briefly
> `draft` while the one unmet dep was the Architect's confirming gate on `DP`;
> **that gate passed at `evt_2qmknsgtmy0rj`**, and the last thing between it and
> a seat was Runtime sequencing, which cleared when `D2k-1d` merged. The ring is
> on this node now.
>
> **Do not read `active` as "start anywhere in it."** `DP` is the entry point
> and everything below it in section 5 is behind `DP`.

> # `D0` MERGED 2026-08-12 — THE POSITIVE ROW IS NON-ZERO
>
> Exact `54f10ca6`, PR #1962, **CI green**, `origin/main` `36848e08`. Declared
> base `b2ee3377`, one non-merge commit, one test-only path
> `core/tests/control.rs`, `+222/-0`. Decision `dec_7fm7rrj76hw58` resolved by
> Architect `evt_59npt60pcy2vq`; QA `evt_71p5wdk23kxnz`. M6 verified by blob
> identity from the declared merge-base.
>
> **The frame's central premise held on the first attempt.** `D0`'s gate is
> committed before any emitter definition, and the checked `D2j` `R3`-shaped
> twin reaches **resolved plane 1** with exactly one key, id and descriptor
> through the production entry `compile_expr_into_object_module` with
> `Some(oriented)`.
>
> | row | subject | resolved plane |
> |---|---|---|
> | absence | seed `px8j` before-hole, no oriented plan | 0 |
> | positive | the checked `D2j` twin via the production entry | 1, one key / id / descriptor |
> | refusal | that twin, frame marker stripped | never planned — validator refuses first |
>
> **Both planes are read by the same call** —
> `build_static_continuation_fusion_plan`, differing only in `Some(oriented)`
> against `None`. That makes `0` and `1` one
> currency rather than an arrival field compared against a plane length, and it
> doubles as the non-constancy proof: the same instrument answers both ways
> inside one test.
>
> **Two things recorded because they are easy to misread later.** The positive
> **still refuses** with `ComputationalMatch` / *"names an in-flight
> activation"* — correct at this deliverable, and it is the baseline the later
> `0 → 1` movement is measured against, which is what will make that movement
> attributable rather than asserted. And `fusion_definitions` is **`0` in every
> row**, deliberately: a non-zero count here would mean something was armed
> ahead of its gate.
>
> **No credit beyond `D0`.** No emitter body, no arming, no interior seam, no
> work on the other seven expressions. **`D1` is next** — the interior seam of
> the frame's section 4, tested against reality.
>
> > #### `D0` CARRIES A CONFIRMED DEFECT — repaired as a `D1` rider
> >
> > Adversary `evt_7gpcg8359ear`, **confirmed by the Steward against the
> > source**. The doc at `control.rs:2963` says the two plane readings differ
> > *"only in the argument that is genuinely different between the two lanes —
> > `Some(oriented)` against `None`."* **They differ in all four arguments:**
> > `:3042` passes `&planner, &entry, &declarations, Some(&oriented)`; `:3073`
> > passes `&seed_planner, &seed_expr, &seed_declarations, None`.
> >
> > **The halves fail separately, and only one fails.**
> >
> > | claim | status |
> > |---|---|
> > | **one currency** — same function, same return type, so `0` and `1` are the same quantity rather than an arrival field vs a plane length | **survives**, and is a property of the signature |
> > | the instrument is **not a constant** | survives — it demonstrably returns both |
> > | **attribution**: that `Some(oriented)` vs `None` is what moves it | **FAILS** — with four arguments differing, the difference is equally explained by the subject |
> >
> > **The design is right and the licence is what overstates.** The absence row
> > is *supposed* to be the seed `px8j` before-hole with no oriented plan — the
> > corrected comparator inherited from `17f68eb1`. **A one-variable comparison
> > was never available between these two rows**, so this is a false sentence
> > about a sound structure, not a bad structure.
> >
> > **`D1` MUST CARRY BOTH REPAIRS:**
> >
> > 1. **Correct `control.rs:2963`** to state what is established and what is
> >    not. Do not simply delete the rider — say that the shared call buys the
> >    currency, and that attribution to `oriented` is **not** established by
> >    these two rows.
> > 2. **Add the missing one-variable cell:** the checked `D2j` twin with the
> >    oriented plan **withheld** — same planner, same expression, same
> >    declarations, `None`. **This is not decoration.** The frame's excluded
> >    scope forbids *"making fusion independent of `oriented`"* because that
> >    reopens `D2h`'s soundness-bearing identity (Architect
> >    `evt_6vf66hmwv52y6`), and this cell is the only thing here that measures
> >    that dependence. **If it resolves `0`**, attribution holds and the
> >    sentence becomes true as originally written. **If it resolves `1`**, the
> >    oriented plan is not what drives the plane and the positive row means
> >    something other than what `D0` claims — **a finding that must reach the
> >    Architect before any arming.**
> >
> > ### THE CELL ANSWERED NEITHER FORK BRANCH — resolved 2026-08-12
> >
> > **Measured: it refuses.** `OrientedSubcontinuationPlanV1` / *"checked
> > subcontinuation markers have no checked plan metadata"*. Not plane `0`, not
> > plane `1`. **The two-way fork above was wrong about the shape of the
> > answer**, and the implementer asserted the refusal as a refusal rather than
> > re-mapping it onto the zero branch — which is what kept the mechanism
> > visible instead of reading as a clean plane-`0` confirmation.
> >
> > **Architect disposition `evt_4m0q1m4zn4k79`: this is NOT the `D2h` stop.**
> > `build_static_continuation_fusion_plan` enters
> > `enumerate_live_fusion_candidates`, whose first operation is
> > `validate_oriented_subcontinuation_transport(entry, declarations,
> > oriented)`. The checked twin still carries its markers, so with `None` that
> > validation refuses **before transport construction, candidate enumeration,
> > key derivation, interning, ID, or descriptor.** No fusion can form through a
> > lawful checked input without oriented metadata.
> >
> > **The bound on the claim, and it is narrow.** This is stronger than plane
> > `0` **on the fail-closed plan-dependence axis only.** It is **not** a
> > plane-cardinality attribution result, because **no plane is returned.**
> > Prose saying "more strongly than plane `0`" without naming that axis
> > overstates it.
> >
> > **Preserve it as a refusal; never relabel it plane `0`.** `px8j` remains the
> > separate, valid, unmarked `None` → plane `0` comparator.
>
> > #### `D1` DOES NOT MERGE ALONE — atomic `D1`+`D2`+`D3` candidate
> >
> > Architect `evt_4m0q1m4zn4k79` on exact `33a77bd4`, which **remains held.**
> > The mechanism is structurally coherent, but with the installer unarmed the
> > production population is empty and none of it fires — **green compilation of
> > inert scaffolding is not a discharge.** `D1` is held, `D2` builds on it,
> > `D3` arming is the last implementation step, and one candidate spanning all
> > three (plus `D4` if arming greens it) goes to review with a live armed
> > control. Frame amended at `docs/program/wp/RT-LEXICAL-R3-FUSION-EMITTER.md`
> > section 5 and `AC-8`.

## What this node is, and why it exists as a node

`RT-LEXICAL-RECURSOR-CONSUMERS` (`#6d`) repairs an **eight-expression**
consumer population so `RT-RECURSOR-TRANSPORT`'s `D3` can retire the
`LexicalCallArgumentRecursor` residual class. **Seven of those eight do not need
static-continuation fusion. One does.** This node owns that one.

**Architect ruling `evt_7knsqyqg72103`**, which is the partition and is exact:

| cell | needs fusion? | why |
|---|---|---|
| rows 1 and 4 (4 expressions) | no | `D2a` removed their `R1` `ComputationalMatch` refusal; all advanced to the distinct `StaticWorkerBinding` wall |
| row 3 (1 expression) | no | `D2b` removed the `R2` closure/ordinary-ABI misclassification; advanced to its retained singular-specialization wall |
| row 5 **after**-hole (1 expression) | no | at the `StaticWorkerBinding` wall |
| row 5 **before**-hole (1 expression) | **yes** | this node |
| row 2 (1 expression) | no | already carved out to [[RT-LEXICAL-ROW2-MISSING-MINT]] |

**Why fusion is genuinely required for that one**, under constraints already
settled and not reopened here: the producer owner lacks the downstream call
arguments, so eager forcing changes CBV; the recursor closure is a live
activation/cursor, so representing or transferring it weakens `#6d`'s `AC-3`;
and the producer and its exact consuming suffix live in **different units**. The
ruled lawful repair is **one planner-identified producer-plus-suffix emission
region**.

## The carve-out rule that makes this node correct

**The expression moves WITH its repair and its discriminating-control
obligations.** Moving the machinery while leaving the expression in `#6d` would
give `#6d` an acceptance surface it cannot discharge — an impossible parent AC.
That is the Architect's load-bearing caveat and it is the reason this node is
scoped to a *cell of the population* rather than to *a pile of mechanism*.

⇒ `#6d`'s population drops from eight expressions to **six**: rows 1, 3, 4, and
row 5's after-hole expression. Row 2 is `RT-LEXICAL-ROW2-MISSING-MINT`'s. Row
5's before-hole is this node's.

## What is already landed, and it is substantial

**Eleven `D2f` partials merged into `main` under `#6d` between 2026-08-11 and
2026-08-12**, every one honestly labelled as inert and every one gated. They are
this node's substrate and **are not to be unwound**: the ABI class, the identity
plane, the arrival-control repair, the per-cause applied-root gate, the
complete-key redirect selector, the fusion arena, the claim facility, the
producer-side atomicity partial, and the full emitter chain (PR #1940) with its
two subsequent prose corrections (PRs #1942, #1943) and the empty-population
attribution repair (PR #1945).

**The whole chain is present and running on nothing.** `D2F_EMITTER_ARMED` is
`false` and gates exactly one call — `install_static_continuation_fusions`.
Everything else runs unconditionally on every production compile and is inert
**by empty population, not by the gate**. Read `core.rs:2163` onward before
forming any view about what arming would change; the comment there is current as
of PR #1945 and was corrected twice to get that right.

## The trap this node must not walk into, recorded before it is framed

**`px8j`'s `R3` before-hole compile structurally cannot carry an oriented
plan.** It is a **seed-lane** compile deliberately preserved as the *unmarked
negative* — no checked frame, no selected-IH slot, no checked-IH-invocation
marker — and `test_objects.rs:70` passes a literal `None` for
`oriented_subcontinuation_plan`. Production oriented plans decode from a checked
package's metadata (`planning.rs:144`), and a seed-lane compile has no metadata
to decode.

⇒ **The acceptance fixture is the checked `D2g`/`D2j` `R3`-shaped twin**, with
its own independently authored `OrientedSubcontinuationPlanV1`, entered through
`compile_expr_into_object_module` with `Some(oriented)`. **`px8j` is the
absence / ordinary-refusal comparator and must never again be described as the
fusion-positive.** That correction landed at `main` `17f68eb1`; this node
inherits it deliberately rather than by citation, because inheriting the old
witness by citation is exactly how the defect would propagate into a successor
frame.

**Forbidden and already ruled out** (Architect `evt_6vf66hmwv52y6`): no
`Some(plan)` handed to `px8j_capture_source_trace`; no synthesized default plan;
no marker inference from the Runtime shape; no weakening of the required
checked-transport key member; and **no making fusion independent of `oriented`**,
which would reopen `D2h`'s soundness-bearing identity.

## Frame

`docs/program/wp/RT-LEXICAL-R3-FUSION-EMITTER.md` — the interior seam, the
arming gate and its five causal controls, deliverables, acceptance criteria with
their controls, excluded scope, and stop conditions.

## Not this node

Retirement of the residual class, lane deletion, and the other seven
expressions. Row 2's missing-`Mint` cell. `#6d`'s six remaining expressions and
their `StaticWorkerBinding` and singular-specialization walls.

## Merged 2026-08-14

**Candidate `cd19957db3e0d1fdeeb8ebe97c0a4b872446b12d`, landed as squash
`34769380`** (PR #2154, CI green; Decision `dec_33chk0zyjkcz4`, Architect; QA
`evt_47g61za1yzbgn`). Merge-base `5a0874a4`, eleven commits, five
`crates/ken-runtime/**` paths, `+2186/-24`; 5/5 blobs verified identical after
landing. **Both SHAs are recorded because the candidate is not an ancestor of
`main`** — a squash rewrites it. Ask content, not ancestry.

**Production remains unarmed** — `const D2F_EMITTER_ARMED: bool = false`,
verified on `main` after landing.

### How the last increment resolved, because the shape is reusable

The ring reported a hard stop while widening the validator control to
`ProducerArity`, and asked to either route a new mechanism to the Architect or
narrow to the two selectors it could measure. **Neither was right.**

`core.rs` already recorded, as a quoted diagnostic rather than a derivation,
that `ProducerArity` **never reaches** the terminal stop — it refuses earlier at
its own widened producer construct meeting the one-argument case, *"which is
the whole reason the cause exists"* — and concluded **the terminal-stop
population is two roots, not three.** The reported refusal matched that text
character for character.

⇒ **The refusal was the cause working as designed, and the frame had
pre-authorized this exact outcome**, assigning the `No` branch a destination:
the fact belongs in the sentence that carries the population claim.

**Recorded because it nearly went the other way:** narrowing to two selectors
was the option on offer, and taking it would have written a correct population
into the record as a concession made under time pressure — which the next
author widens back. **Excluding a non-member is not a scope cut.**

**The Architect retracted the premise of their own block** (`evt_dapr4c8kdcwn`):
they had written `ProducerArity`'s reach was *"unmeasured"*, and the
measurement sat ten lines above a line number they cited in the same review, in
the file they were reading. The block still bought two things — a stale `d2f_0`
comment now distinguishes the three **planning** positives from the
terminal-stop population, and the population is **named** (`Exact`, `ReHomed`)
rather than described, so the next reader checks it instead of re-deriving it.

### Residual: the boundary that scopes the control is prose, asserted nowhere

Architect finding, non-blocking, at `evt_dapr4c8kdcwn`. The control's coverage
is correct **because** `ProducerArity` refuses at the `ComputationalMatch` arity
check — a fact recorded in a comment with a quoted diagnostic, while the
executable assertions measure only planning/build arrival and the unarmed
baseline.

**If the fixture's case arity changes, or `ProducerArity`'s widening moves, that
cause rejoins the terminal-stop population, the control silently covers two of
three again, and nothing reds.** A constraint that never becomes an assertion is
one the build cannot fail, and it is usually the load-bearing one.

**Not filed as a node, deliberately.** `d2f_0` runs unarmed to pin the baseline,
so pinning the armed refusal is a **new armed assertion** rather than a line,
and the arming discipline (`D2fEmitterTestArm`, RAII, one block) is exactly what
not to loosen casually. **It belongs with the emitter's real arming** — at which
point the refusal either still holds and costs one assertion, or has moved and
you want to know.

> #### AMENDED 2026-08-14 by Adversary hunt `evt_28n873ahnq6z7`, run at
> #### `bc62216a`. The blast radius above is wrong by 9x and the cost
> #### argument prices an instrument that already exists.
>
> **The prose fact itself is CONFIRMED, measured armed.** The Adversary armed
> via the sanctioned `D2fEmitterTestArm` RAII and compiled all three roots:
> `Exact => None`, `ReHomed => None`, `ProducerArity => Some(Unsupported{
> construct: "ComputationalMatch", reason: "case ctor:fixture::D2gOut::Node
> expects 1 constructor arguments but value has 2" })` — byte-identical to the
> string quoted at `core.rs:2936-2937`. Probe reverted, `control.rs`
> byte-identical after. **The disposition recorded above is right; only its
> scoping and its costing are not.**
>
> **1. This paragraph is not one control's completeness argument. It is the
> population definition for the candidate's ENTIRE control set.** The literal
> `[(D2jCause::Exact, "exact"), (D2jCause::ReHomed, "rehomed")]` is hard-coded
> at **nine** sites in `control.rs` — `3109`, `3271`, `3405`, `3564`, `3699`,
> `3852`, `4005`, `4117`, and the validator's own pair at `4330` — behind nine
> `r3_fused_*` controls whose `MEASURED:` blocks all read *"on both armed
> roots"* / *"both real selectors"*: `parameter_projection`, `worker_body`,
> `wrong_consuming_call`, `nonempty_producer_captures`,
> `late_call_build_refusal`, `post_field_direct_call_reintroduction`,
> `outer_selector_escaped_claim`, `capture_projection`,
> `target_authority_validator`. **The residual as I filed it points a reader at
> one.** If `ProducerArity` rejoins, nine controls silently cover two of three.
>
> **2. The cost argument reaches the instrument I named and not this one.**
> *"It needs a new armed assertion"* prices building an armed harness. **The
> harness exists and is cause-parameterized:** `D2fEmitterTestArm::arm()` is
> used at twelve sites in this same file, nine of them added by this candidate.
> The Adversary wrote the assertion — 34 lines standalone including boilerplate,
> one build, 36 seconds, exact refusal string, **loosening nothing**. As a third
> row on the existing validator control it is smaller: that control's inner
> `compile(cause, symbol)` already takes a `D2jCause`, already arms, already
> builds through `d2j_checked_fixture_under`. The only obstacle is its own
> `assert!(error.is_none(), ...)` at `control.rs:4325` — relax that to return
> the error and the `ProducerArity` row is a `contains` on the arity sentence,
> roughly six lines. *"Do not loosen the arming discipline"* rules out arming
> production or widening the const; **it does not reach a `#[cfg(test)]` RAII
> arm that nine of this candidate's own controls already use.**
>
> **3. The population has no name, so a third root costs nine edits and a pin
> has nothing to attach to.** There is no `const` naming the terminal-stop
> population; it is nine anonymous array literals agreeing by discipline.
> Missing one on a future edit yields a control that silently covers a subset —
> the same defect one layer down. **And the tree already has the right shape one
> plane over:** `ken-r3-base` (`control.rs:34788`, `:34866`) expresses the
> three-root population **as data**, the literal `[Exact, ReHomed,
> ProducerArity]`, written twice, unarmed. **The degradation from data to prose
> happened at exactly one plane boundary** — base/planning names it, the armed
> plane narrates it.
>
> **Disposition — the repair, in the order it should land, and still not a
> node.** Two pieces, both inside `control.rs`, both `#[cfg(test)]`:
>
> 1. **Name the population once** as a `const` and iterate it in the nine
>    controls. **Needs no arming at all**, so it is not the repair that was
>    costed. It does not detect `ProducerArity` rejoining; it is what makes the
>    detection a one-line addition instead of a tenth literal.
> 2. **Add the armed `ProducerArity` row** to the target-authority validator
>    control, as the ~6-line third row described above.
>
> **Route: the next Runtime candidate that touches `control.rs`** — folded, per
> `ken-steward` §4c, rather than lengthening the critical path with a node for
> two `#[cfg(test)]` edits. **This is also flagged on
> [[RT-LEXICAL-RECURSOR-CONSUMERS]]**, which owns the next `#6d` slice frame,
> because a residual on a merged node is read by nobody by default.
>
> **Explicitly unswept, and named as such rather than implied clean:** the
> Adversary scoped to lines this candidate added and did **not** sweep the
> unchanged bulk of these five files (28k+ lines in `control.rs`), where phrases
> of the same family are frequent.
