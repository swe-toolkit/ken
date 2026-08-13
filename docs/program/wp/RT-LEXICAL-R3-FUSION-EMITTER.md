# RT-LEXICAL-R3-FUSION-EMITTER — the one expression that needs fusion

Owner: runtime. Size: M. Node: [[RT-LEXICAL-R3-FUSION-EMITTER]].

**Fixed inputs measured at `origin/main` = `5a794bff`.** Line numbers are
anchors to re-find at your own base, not values to trust. Re-derive your
merge-base from `origin/main`; **do not take a SHA from this frame.**

**Seat tier: T2 build ring.** Architect votes at merge. **No Spec vote** if your
diff stays in `crates/`.

## 1. What this node owns, exactly

**One expression:** row 5's **before-hole** member of the eight-expression
lexical-recursor consumer population — `selected_scope` before the hole.

It is carved out of [[RT-LEXICAL-RECURSOR-CONSUMERS]] (`#6d`) **together with
its repair and its discriminating-control obligations.** That coupling is the
point of the carve-out, not a detail of it: Architect ruling `evt_7knsqyqg72103`
states that moving the machinery while leaving the expression in `#6d` would
give `#6d` an acceptance surface it cannot discharge. **If you find yourself
repairing an expression that is not this one, stop — it belongs to the parent.**

Seven siblings do **not** need fusion and are not yours: rows 1 and 4 are at the
`StaticWorkerBinding` wall after `D2a`; row 3 is at its retained
singular-specialization wall after `D2b`; row 5's **after**-hole expression is
at the `StaticWorkerBinding` wall; row 2 belongs to
[[RT-LEXICAL-ROW2-MISSING-MINT]].

## 2. Why fusion is genuinely required here, and is not an optimization

Under constraints already settled and **not reopened by this node**:

- the producer owner **lacks the downstream call arguments**, so eager forcing
  changes CBV;
- the recursor closure is a **live activation/cursor**, so representing or
  transferring it weakens `#6d`'s `AC-3` guard 2 (a closure is never made
  boundary-transferable);
- the producer and its exact consuming suffix live in **different units**.

⇒ The ruled lawful repair is **one planner-identified producer-plus-suffix
emission region**. Suppressing that fusion must restore the measured refusal —
that is `AC-3` below, and it is the control that distinguishes a fusion that
does work from one that is merely present.

## 3. THE TRAP — read this before you plan, it has already cost this work once

> ### `px8j`'s before-hole compile CANNOT carry an oriented plan
>
> **It is not your positive fixture.**
>
> `px8j` is a **seed-lane** compile deliberately preserved as the **unmarked
> negative**: no checked frame, no selected-IH slot, no checked-IH-invocation
> marker. `test_objects.rs:70` passes a literal `None` for
> `oriented_subcontinuation_plan`, and production oriented plans are decoded
> from a checked package's metadata (`planning.rs:144`) — a seed-lane compile
> has no metadata to decode. The `oriented` gate
> (`planning/static_transition.rs:8901`, `:9058`) then returns an empty plan
> **before any candidate enumeration runs.**
>
> Measured on the exact witness: `planes=[0] oriented_present=[false]`. One
> production compile reached the builder and resolved **zero**.
>
> **An earlier version of the parent frame pinned acceptance to this witness.
> That was my defect, not the ring's**, and it made `AC-1`/`AC-2` unsatisfiable
> — an emitter built against it would discharge its no-activation criterion
> **vacuously**, because a proof over nothing emitted passes for free.
>
> ⇒ **The positive fixture is the landed `D2g`/`D2j` checked `R3`-shaped twin**,
> with its own independently authored `OrientedSubcontinuationPlanV1`, consumed
> through the **one hoisted `#[cfg(test)]` constructor** and entered through
> `compile_expr_into_object_module` with `Some(oriented)` — **never** by calling
> the builder or emitter directly. **`px8j` is retained as the absence /
> ordinary-refusal comparator** and must never again be described as the
> fusion-positive.

**Forbidden routes, already ruled out** (Architect `evt_6vf66hmwv52y6`) and
listed so they are not re-derived as fresh ideas: no `Some(plan)` handed to
`px8j_capture_source_trace`; no synthesized default plan; no marker inference
from the Runtime shape; no weakening of the required checked-transport key
member; and **no making fusion independent of `oriented`**, which would reopen
`D2h`'s soundness-bearing key re-derivation.

## 4. The interior seam, located — do not re-derive it

The per-phase authority switch **has no function-level boundary left to sit
at**, and that is a consequence of a forced design choice, not an oversight.
Collapsing producer and consumer lowering into the single fused dispatcher call
was forced — lowering the producer to a value first relocates its own
"in-flight activation" refusal into the fused function — and the collapse
deleted the boundary the switch occupied.

⇒ Per the Runtime implementer's located answer (`thr_2wp6pehk4ybgk`), which
this frame adopts so the turn starts from it:

- **The switch site is inside the eliminator's case-body lowering**, keyed on
  the frame whose `static_origin` is a claimed fusion's `continuation_origin`.
  That is the only place the consumer's case body is identifiable.
- **The fused body's checked-frame adoption belongs at that same interior
  point**, not at the fused function's entry.

Architect ruling `evt_4vqey13cxxjqs` (durable at `51e6a266`) governs the
semantics: the fused function opens its **own fresh per-`Function` scope** and
re-enters the **consumer** frame identity; authority runs
producer → consumer → producer **per phase, and is NEVER `Fusion`**. `Fusion`
remains region/definition identity only.

**Binding, carried from the parent:** never assert `frame_origin` equality
(`25 != 10`). The carrier gate covers **one** function.

## 5. Deliverables

**`D0` — the gate, committed BEFORE any emitter definition.** Three rows: the
old negative at resolved plane `0`; the checked positive at resolved plane `1`
with **exactly one** key/ID/descriptor; and a one-marker-stripped exact
validator refusal. **No emitter AC may be credited until the positive row is
non-zero.** This deliverable exists because every prior control that reached a
fusion candidate used a synthetic fixture and called the builder directly —
**none of them compiled the acceptance fixture**, so the mechanism was untested
against it on *both* sides of the gate.

> ### RECUT 2026-08-12 — `DP` COMES FIRST, AND NOTHING BELOW IT IS RELEASED
>
> **The cumulative stop fired at `D2`** (Architect `evt_1q7v9fcw5hd87`; full
> statement in the node). `D1` and `D2` are built and **preserved as evidence
> only** at `8063dd67...7166baaa` — not a merge candidate, not routed to QA.
>
> > #### MEASURED CORRECTION — `DP` DOES NOT MINT AN IDENTITY. IT ADDS COVERAGE.
> >
> > **Steward, 2026-08-12, on runtime-implementer's armed probe
> > `evt_5pfgetdgv3bkf`, which was run BEFORE anything was written.** On the
> > `D2j` `Exact` twin the layers came back `(frame=Some(1), invocation=None)`
> > and `(frame=Some(0), invocation=Some(1))`. **The producer layer ALREADY
> > carries its own checked frame id** — the checked source authors it, with its
> > own semantic position, slot and `ParentFrame` witness. **What was absent is
> > invocation-source COVERAGE**, nothing else.
> >
> > **The two struck phrasings below are the defect.** Both read as `DP`
> > creating the producer's identity, which would put `DP` in the business of
> > authoring what the planner already authored — the exact inference the
> > Architect's membership law forbids Runtime from making. **The PRECISION
> > CORRECTION further down was right all along and is now confirmed by
> > measurement**, so this box aligns the leading sentence with it rather than
> > changing any ruling.
> >
> > **Why it had to be corrected here specifically:** a reader who greps for the
> > deliverable finds the headline, not the qualifier eleven lines later. A
> > superseded claim that survives in the leading sentence is invisible to a
> > line-local read.
> >
> > **Second measurement from the same probe, load-bearing for the shape:** one
> > call template is entered **twice in a single compile** — instance 1 from
> > `lower_fused_producer_through_suffix` with a two-layer composed segment,
> > instance 2 from `define_unit_bodies` with a one-layer ordinary one.
> > Widening the shared base covers the first and refuses the second with
> > `expected={0, 1} instantiated={0}` — **the same refusal `89ee005b`
> > produced, reached by a second independent route.** No template-level
> > widening can satisfy both shapes.
>
> ~~**`DP` — give the producer semantic occurrence its own transported checked
> identity.**~~ **`DP` — establish the already-authored producer frame's
> membership in the concrete invocation segment, at the composition splice.**
> **Design class CONFIRMED by the Architect at
> `evt_2qmknsgtmy0rj`; the design-class hold is lifted.** Class 2 would require
> inventing a semantic partition the measured segment does not have, while the
> emission region is intentionally producer plus suffix.
>
> ~~**`DP` populates and transports, from the checked source:** the producer's
> **distinct frame template and marker**, plus its real checked occurrence
> path, semantic position, segment site, input/output interfaces, occurrence
> binding, control witness, and **invocation/parent-edge relation.**~~ **`DP`
> transports, from the checked source, the invocation-source coverage for a
> frame identity the checked source has already authored** — the frame
> template, marker, occurrence path, semantic position, slot and `ParentFrame`
> witness are read, never minted. The **unchanged** validator must then accept
> the complete expected frame set, order, endpoint composition, site, witnesses
> and dynamic ownership.
>
> > #### PRECISION CORRECTION — "its own invocation source" OVER-SPECIFIES
> >
> > **A dynamic semantic-frame identity is the pair `(invocation_instance_id,
> > frame_id)`.** `OrientedSubcontinuationFramePlanV1` supplies the distinct
> > frame occurrence with its position, site, interfaces, binding fingerprint
> > and witness. Invocation templates **separately** supply an ordered
> > `callee_frame_templates` sequence, `instantiate_checked_invocation_segment`
> > transports one affine invocation source/instance onto the exact expected
> > frames, and composition keys by **the pair**.
> >
> > ⇒ **"Its own invocation source" means planner-authored invocation-source
> > COVERAGE for the producer frame.** It does **not** require a gratuitously
> > distinct invocation-template variant, nor a second dynamic invocation.
> >
> > - **If the checked source establishes producer and consumer as frames of
> >   one invocation-local segment**, sharing that invocation source and affine
> >   instance **is lawful** — but only because the plan names **both distinct
> >   frame IDs in the exact expected order.**
> > - **If they are distinct invocations**, the planner must supply **distinct
> >   sources plus the checked dynamic parent edge.**
> >
> > **Which relation holds is a planner/source fact. Lowering and fusion must
> > not choose it, copy it, or infer it.** Read that as the same prohibition
> > that killed the consumer-identity alias, applied one level up.
>
> **RELEASED 2026-08-12 at `main` = `1f578a70`.** The design-class hold was
> already gone; the remaining condition was Runtime sequencing, and it cleared
> when `D2k-1d` merged. **The confirmation still grants no merge, arming,
> `AC-8`, `D4`, or held-range carry-forward credit**, the cumulative stop
> remains fired at later cuts, and `D1`+`D2`+`D3` remain one atomic candidate
> **after** `DP`, on a fresh re-derivation against then-current `main`.
>
> **Why this class and not the other.** The alternative is to redesign
> composition so the inferred producer layer is provably **outside** the checked
> consumer segment, with an explicit validated boundary. Two things rule against
> it. The Architect's own analysis says the producer eliminator **is** a genuine
> semantic participant — `semantic_pending`, *"not a control-only wrapper that
> may be omitted"* — so proving it outside the segment argues against the
> measured fact. And the fused region deliberately puts producer and consuming
> suffix **into one emission region**, so a partition proving them separable
> fights the fusion design this node exists to build. `DP` supplies the missing
> authority instead of arranging not to need it, and it is the only one of the
> two that leaves the segment-wide checked representation untouched.
>
> **`DP` is the thing the stop exists to catch, so it is authorized
> deliberately or not at all.** It is exactly *"new planner population beyond
> what is landed."* It does not slide in as part of an emitter increment.
>
> **What survives unchanged:** `D0` (merged), the located seam in section 4, the
> forbidden routes in section 3, and the atomic-candidate rule below — which
> still governs `D1`+`D2`+`D3` once `DP` makes them buildable.
>
> **What any successor owes on contact:** correct the stale arming comment.
> **After `D2` the live refusal is the mixed checked/inferred semantic-frame
> guard, not the prior step-5 refusal.**

**`D1` — the interior authority switch**, at the seam located in section 4.
**Implemented and committed, then HELD.** Not routed to QA, not merged alone.

**`D2` — the fusion-specific checked-frame adoption**, at that same interior
point. Built on top of the held `D1`, and **held with it.**

**`D3` — arm the emitter.** Flip `D2F_EMITTER_ARMED`. **This is the final
implementation step**, taken only after `D0`'s positive row is non-zero and
`D1` and `D2` are implemented. **Arming is authorized by this frame and by
nothing you inherit from `#6d`.**

> ### `D3` GAINED A SELECTOR, AND ITS FIVE NETS ARE ACCEPTANCE — Steward,
> ### 2026-08-12, recording Architect `evt_4g2hmsr8tb3bm`
>
> **Why `D3` is no longer just the arming flip.** Arming exposed that one fused
> function builds **two** checked segments — composed (`layers=[inner,outer]`,
> needs `{0,1}`) and ordinary (`layers=[outer]`, needs `{0}`) — and `DP`'s
> body-extent selector marks **both** composed, so the ordinary one refuses
> `expected={0,1} instantiated={0}`. **The validator is correct and fail-closed;
> the selector is too coarse.** Measured and eliminated first: Runtime segment
> shape (forbidden, and circular — subset coverage renamed), `D1`'s per-phase
> authority (**identical at both composes**), `RecursorProducerOriginId` (a
> lowering-minted counter, not a plan key).
>
> **THE RULING — an affine capability bound to the concrete edge, NOT to the
> body.** Issued by the already-checked fusion-composition splice and carried on
> **that splice's specific pending semantic edge/eliminator**. The segment that
> actually consumes that edge consumes the capability and is checked `Composed`;
> every other segment, including the second `[outer]` one in the same fused
> function, stays `Ordinary`. Preserve it through the producer-through-suffix
> descent; consume it **at most once**; restore and close the scope on **both**
> success and error.
>
> **The line the ruling draws, and it is the whole point.**
> `evt_2f0nnwtzqy65m` does **not** prohibit this shape: the capability
> contributes **no frame ID and proves no membership**, and the checked plan
> remains sole author of `ordinary_frame_templates` and
> `composed_frame_templates`. It proves only *which dynamic construction is the
> checked splice*. **But a permit whose only provenance is "we are in a fused
> body" or "this is the first compose observed" WOULD violate that ruling** — it
> makes execution order the authority for applicability. **Fail-closed exact
> coverage is a necessary net and does not turn an unbound ordering guess into
> an identity.**
>
> **Explicitly forbidden:** an ambient body flag, a counter, a global or
> body-scoped "next call wins" slot, a search for the first segment whose
> Runtime shape fits, and **retaining `fused_composition_extent`**.
>
> **Why not a planner per-occurrence key now:** the fusion claim plus that
> particular pending edge already distinguish the construction, so a new planner
> key would duplicate authority. **Escalate to one only if implementation proves
> the capability cannot be attached to and recovered from that exact edge
> without falling back to next-event ordering — and that is a NEW HARD STOP, not
> licence to use the coarse permit.**
>
> **`AC-D3-SEL` — the five selector nets, in this same held atomic object:**
>
> 1. The `[inner, outer]` segment consumes exactly one splice capability and
>    selects `Composed`; the `[outer]` segment selects `Ordinary`.
> 2. Deleting the capability restores the existing exact/mixed refusal — it does
>    **not** silently select `Composed` from frame shape.
> 3. Duplicate, replayed, escaped, and unconsumed capabilities all red.
> 4. An unrelated ordinary segment inserted **before** the target cannot steal
>    the capability or be accepted as `Composed`.
> 5. Controls 2 and 3 and the **inner-slot-widening witness** remain owed
>    unchanged — none of them is earned by this selector work.
>
> ### STOP 3 IS RULED — `AC-D3-SELF` — Steward, 2026-08-12, recording
> ### Architect `evt_4x3291v9dx0vb`
>
> **Full ruling is at `evt_4x3291v9dx0vb`; the leader's dispatch is
> `evt_6fzg11hpvfp4w`. Read the ruling, not this box** — what is here is the
> part that binds acceptance.
>
> **The moved suffix's claimed IH invocation is a RECURSIVE CALL TO THE SAME
> `Fusion(id)` DEFINITION.** Not a no-op or current-result substitution, and
> not a call back to the standalone producer. The producer body was lowered
> inline for the **current** invocation, which does not discharge this **child**
> invocation: dropping the call drops recursion, and targeting the fusion-owned
> producer body would run the producer without the moved suffix and violate body
> ownership.
>
> **No new planner population is owed** — the preflighted `FusionRegionClaim`
> already transports every identity needed. This is `D3` emission wiring inside
> the held atomic object.
>
> **The internal self edge is NOT a second external redirect.** The existing
> consumer-to-fusion redirect remains the sole external affine redirect; the
> recursive self edge is a distinct definition-local obligation of the same
> claim and **does not consume the region claim a second time.**
>
> **KEEP CALL-SITE IDENTITY DISTINCT FROM BODY/CALLEE IDENTITY, even though
> this fixture prints `37` for more than one axis.** `call_declared_unit` looks
> up a retained callee/body coordinate; the authorization is the exact
> `consuming_call` occurrence. **Keying both by the numeric coincidence at `37`,
> or by "missing target while in a fused body", is unlawful.** If the exact call
> occurrence is not available at the emission seam, **thread the existing source
> occurrence there — never infer it from lookup failure or body shape.**
>
> **Rejected repairs, named so none is re-proposed:** union with the consumer
> edge table; restoring the standalone producer edge; identity or current-value
> forwarding; a generic self-call fallback on a missing `unit_calls` entry; an
> ambient "inside fusion" switch.
>
> **`AC-D3-SELF` — closeout must require the exact internal recursive edge for
> each claimed consuming call, and reject each of:** omission; duplication; a
> different consuming call; a different `Fusion` target; the standalone-producer
> target; a producer-frame contract. **Plus a discriminator separating call-site
> identity from body/callee identity**, since this witness leaves them
> numerically coincident — a control that passes on the `37` coincidence proves
> nothing.
>
> ### MEASURED — the discriminator's fixed inputs, and the half of the ruled
> ### guard that is NOT verified
>
> **Steward, 2026-08-12, from the implementer's build report
> `evt_6d3gb569n1twy` at exact `fe5c311e`. These are measured numbers, not
> chosen ones.**
>
> **The identity axes as this fixture prints them:**
>
> | axis | value |
> |---|---|
> | `consuming_call` (the authorization) | **17** |
> | `consuming_callee` | 16 |
> | `producer_body` | 37 |
> | seat | 37 |
> | `redirect_callee` | 37 |
>
> **So the coincidence the Architect warned about is real and it is three-way:
> seat, `producer_body`, and `redirect_callee` all print `37`, while the
> authorizing call site is `17`.** Folding call-site into body **type-checks**,
> and a wrong-call-site edge is then indistinguishable from a right one. ⇒ **The
> `AC-D3-SELF` discriminator is satisfied only by a control that separates `17`
> from `37`.** A control whose expected values are all `37` would pass under the
> fold and proves nothing — that is the whole reason the discriminator is an AC.
>
> **THE RULED GUARD IS HALF-BUILT, AND THE COMMENT BESIDE IT IS NOT A
> DISCHARGE.** The ruling requires the checked consuming-callee/binder relation
> to **resolve to** `claim.producer_body()` and the redirect's producer entry.
> The implementer's first attempt compared `consuming_callee` to `producer_body`
> **directly** — `16` against `37` — and refused every lawful region; the
> relation **resolves**, it does not equal. That resolution is **preflight's**
> and is **not re-derivable at the emission seam**: it needs `ih_bindings` and
> `SemanticIr::child_origin`, which is `pub(super)` to the planner. Inventing a
> substitute is exactly the inference the ruling forbids, so the implementer
> did not.
>
> **What IS checked:** the redirect's producer entry against the claim's
> producer body — two fields reaching the claim by different routes, so their
> agreement is a genuine cross-check. **What is NOT checked:** the
> callee-to-body resolution itself. It is named in a code comment rather than
> left looking discharged, which is the right handling — **but a named limit is
> not a met criterion.** ⇒ **`AC-D3-SELF` is not closed by the guard as built.**
> Closing it needs either the planner-side closure the implementer offered to
> the Architect, or an explicit Architect ruling that the cross-check is
> sufficient and the resolution half is not owed. **Neither has happened; do not
> read the comment as either one.**
>
> ### THE EMITTER HALF STOPPED AT A GENUINE BOUNDARY — three premises under it
> ### are refuted, and one of them is a NAME COLLISION WITH LANDED PRODUCTION
> ### CODE. Steward, 2026-08-13, recording `evt_46ka0eqw5rmrb`. **BOTH
> ### QUESTIONS ARE THE ARCHITECT'S AND ARE OPEN.**
>
> **This stop is not the previous turn's reason.** That one was context depth,
> named honestly. This one is a mechanism boundary reached on a **fresh
> context**, found by arming the emitter in a **local reverted probe** rather
> than building the picture from a comment. No emitter code written.
>
> **(1) THE IN-TREE STEP LIST IS OBSOLETE, IN THE GOOD DIRECTION — AND NOTHING
> GOES RED WHEN IT LIES.** `core.rs` says the wired form advances through four
> refusals and stops at step 5 or step 6. **It does neither:** armed, `Exact`
> now runs through **all of lowering** and dies at the very end in `emit_result`
> → `ground_value` → `into_specialized_at`, at the root projection. Steps 1-6
> are all behind it. **That is an advancing refusal and it is the held `D1`/`D2`
> range working** — but **the comment describing it is now false, and its own
> text admits no detector fires when it becomes false.** A reader planning
> against that block would plan against a stop that no longer exists.
>
> **(2) THE FUNNEL CANNOT REACH THE VALUE THAT BLOCKS THE ARMED PATH.** The
> escaping value is a `StaticWorker` constructor field at **owner construct 30,
> position 0, `field_origin` 29, `ctor:fixture::D2gOut::Node`, `funcid43`** —
> the **only** worker field in the entire armed compile. But `Exact`'s **whole
> continuation-call population is two edges**, at construct origins **36** and
> **39**, and **both are composed**. **Construct 30 mints no call identity, so
> no funnel lookup — composed or direct — reaches it.** Building the funnel
> would not move this refusal. ⇒ It also makes this family **wholly composed**:
> there is **no residual direct edge** to be the `DirectCall` half of the
> same-body row.
>
> **(3) `CandidateDisposition::ComposedCall` ALREADY EXISTS IN PRODUCTION WITH
> THE OPPOSITE MEANING.** It is settled at `mod.rs:8381` and documented as *"a
> raw-worker call was emitted AND passed finished-CLIF verification"* — the
> `D8` composed-recursive-argument mechanism, in which **a call IS emitted**.
> The ruled fusion composition **emits no call**. The collision is load-bearing
> because it feeds landed closeouts: `call_obligations = DirectCall ∪
> ComposedCall`, then `emitted ⊎ composed == call_obligations` **and**
> `claimed == call_obligations`. A fusion-composed edge is in **none** of those
> sets, so settling it `ComposedCall` reds immediately. A fourth face is
> structural: `close` also asserts `resolved == planned` and `declared ==
> planned`, yet a target with no emitted `Function` has no `FuncId` to resolve
> and no `FuncRef` to declare while its identity is still `planned`.
>
> **WHAT WOULD HAVE HIDDEN ALL OF IT, and this is the reason it was reported
> from the code rather than from a red:** the claim/candidate closeout **is
> never reached on the `d2j` witnesses, armed or unarmed.** The collision
> **cannot be measured on the witness this unit is built around** — it would
> have landed green here and red somewhere else.
>
> **THE FORK, and the implementer explicitly declined to choose:** (a) is the
> construct-30 worker field in **this node's scope at all**, or does it belong
> to the `DP` transported-identity work — the funnel was ruled expecting to
> consume it and that expectation is **measurably false**; and (b) how does a
> fusion-composed edge satisfy the two closeout laws while preserving `D8`'s
> landed meaning of `ComposedCall`? **`InlineNoCall` is the available spelling
> and is the silent-wrong-answer option** — it would make a fusion-composed edge
> indistinguishable from an inlined non-call, and the enum's own doc says
> `InlineNoCall` is *"deliberately not a third arm."*

> ### THE FORK IS ANSWERED BY REFUSING BOTH OF ITS OPTIONS — the guard moves
> ### from the EMITTER to the PLANNER as a bijection law. Steward, 2026-08-13,
> ### recording Architect `evt_7akh94dvqeqap` (durable `cc37116f`).
>
> **Branch (b), but explicitly NOT another heuristic** — and (a) was rejected in
> the terms this frame already anticipated: *"documenting it as unreachable
> would leave the load-bearing claim in prose. Neither is acceptable."*
>
> **THE ACTUAL FINDING, and it is a real hole rather than a re-siting.**
> `validate_continuation_specialization_closure` today proves key↔unit
> bijection, unique call tokens, token/target agreement, and surjective
> reachability of units. **It does NOT prove injectivity of call target: two
> distinct call tokens may name one unit.** ⇒ the emitter's incoming-domain scan
> was defending an **invalid planner state, late, with a guard no lawful source
> can make fail.** The fixtures-first result did not merely show the guard was
> vacuous — it showed the guard was **at the wrong layer**.
>
> **THE REPLACEMENT CONTRACT.** (1) Planner closure makes calls and
> specialization units **bijective**: no two exact planned calls share a
> `ContinuationSpecializationId`, and a repeated target **refuses in planner
> validation** — before generated-context interning, fusion preflight,
> disposition, declaration, or any lowering mutation. (2) **Target liveness is
> the outcome of its unique call identity**, settling exactly one of
> `ComposedCall` or `DirectCall`; **there is no complete-incoming-domain scan in
> the emitter and no partial-coverage state there.** (3) **Body equality is
> never liveness authority** — distinct full keys give distinct IDs and distinct
> call identities even with equal worker body/provenance, so composing one
> identity cannot suppress a sibling. Do not query body, origin, owner, target
> coincidence, constructor spelling, or arity. (4) The two ruled composition
> layers are unchanged.
>
> **THE SAME-BODY SIBLING FIXTURE PROBLEM IS DISSOLVED RATHER THAN WAIVED.** It
> becomes a **planner-level** discriminator: two exact, closure-valid full keys
> sharing worker body/provenance but differing on one legitimate identity
> coordinate must intern to distinct units with distinct call identities; mark
> one composed and one direct, and only the composed unit may leave the
> executable population. **It need not be a `D2j` source program** — it tests
> the planner relation directly — **but it must use the exact path, never a
> coarsening mutation, and must not be described as source reachability.**
>
> **THE OWED RESIDUAL-DIRECT-CALLER SOURCE FIXTURE IS REMOVED**, because that
> state is now **rejected as a planner alias** rather than preserved as lawful
> execution. **It is explicitly NOT replaced by a comment-only
> "structurally unreachable" claim.**
>
> **THE FIRST BOUNDED UNIT, before any lowering or body change:** strengthen the
> closure validator with the unique-target law; show Exact and ReHomed stay
> green at two units / two unique call identities / two unique targets; add a
> **mutation-backed negative that reaches the new check** — an `OmitProjection`
> row **whose omitted projection is not present in the call token** is the
> preferred discriminator, because a mutation that already fails token/target
> agreement **does not discharge this row**; and keep the duplicate-token
> refusal separate, since duplicate identity and distinct-identities/same-target
> are **different defects**.
>
> **THE FALSIFIER IS NAMED, which is what makes this rulable without a further
> round:** if the new bijection check **rejects any current exact lawful
> fixture**, stop and report that population — **that refutes the ruling's own
> premise.** If exact planning stays green and the projection-coarsening
> mutation reds at the new check, the ring **continues to the already-ruled
> composition relation with no further Architect round.**

> ### THE FIXTURES CANNOT BE BUILT, AND THAT IS THE RESULT — the partition
> ### guard may be defending a state the planner already makes unreachable.
> ### Steward, 2026-08-13, recording `evt_1j3h4asztzp7n`. **THE FORK IS THE
> ### ARCHITECT'S AND IS OPEN.**
>
> **`FIXTURE A` — a specialization with a residual direct caller — IS NOT
> CONSTRUCTIBLE BY ANY LAWFUL PROGRAM, and the reason is structural.**
> `intern_specialization` keys on the **whole** `ContinuationSpecializationKey`,
> which already contains the fields distinguishing one call identity from
> another (`producer_result_origin`, `producer_construct_origin`,
> `producer_alternative`, `recursive_position`, `consumer_owner`,
> `continuation_origin`, `worker`). Two distinct call identities agree on every
> key field only if they are the same tuple ⇒ **distinct identities imply
> distinct targets.**
>
> **The confirmation is that the tree already contains machinery for this
> population, and it is a MUTATION.** `ContinuationInternMutation::{OmitProjection,
> PrefixOnly}` exist precisely to **coarsen** that key so two keys intern to one
> unit, and existing controls use them as **injected defects**. ⇒ A fixture
> exhibiting a residual direct caller would be a program **plus a planner
> defect** — the opposite of a durable preservation fixture, and it was
> correctly not built under the mutation knob.
>
> **`FIXTURE B` — a same-body sibling — was NOT constructed and is NOT claimed
> impossible.** Interning does not exclude it: two specializations may differ in
> continuation origin or alternative while sharing a worker body occurrence. But
> worker provenance is occurrence-keyed and each recursive field is its own
> closure occurrence, so **ten measured configurations** — all eight `D2j`
> causes plus a twice-applied declaration and an applied call wrapped in a
> constructor — give ten sets of pairwise-distinct worker bodies. Duplicating
> the call site does not help and says why: each application mints **its own**
> specialization rather than a second edge into a shared one.
>
> **WHAT THE SEQUENCING BOUGHT, stated plainly.** The fixtures-first reasoning
> is unchanged — on a singleton domain the partition cannot fail and would land
> green carrying no information. **What changed is that the fixture cannot
> supply the non-degenerate population either.** ⇒ The unit's escape hatch
> ("stop and report; do not reshape the witness") fired exactly as bounded, and
> it surfaced this **before** a mechanism was built on top of the guard rather
> than after.
>
> **THE FORK, and the implementer explicitly declined to choose it:**
>
> - **(a) The guard is correct and deliberate defence in depth**, in which case
>   the two controls are **structurally unreachable rather than owed**. **This
>   repo already has that category and a precedent:**
>   `FusionClaimRefusal::SelfRedirection`, whose doc says no witness reaches it
>   and instructs that **no control claim one** — one sentence in a doc comment,
>   not a fixture.
> - **(b) The partition should be keyed on something other than incoming-call
>   multiplicity.**
>
> **Do not read (a) as the cheap answer.** It converts two owed controls into a
> documented unreachability claim, which is a claim that must itself be true —
> see the standing rule that a documented limit is not a discharged claim.

> ### THE ROUTE-C MECHANISM IS RULED — A PLANNER-AUTHORED, CALL-EDGE-LOCAL
> ### CONTINUATION COMPOSITION. Steward, 2026-08-13, recording Architect
> ### `evt_1t3f4e8100rb5`. **Read the ruling; this box is the frame
> ### consequence and the sequencing.**
>
> **The class, in one sentence:** compose the exact continuation specialization
> **at each planner-authorized call edge** — its selected body lowered in the
> caller with the same complete phase-bearing operand run the direct call would
> have received, flowing straight into the caller's existing computational
> eliminator. **No intermediate `emit_result`, `ground_value`, result slot, call
> ABI, or returned SSA word for that composed edge.** The existing redirect,
> self-edge rule, takeover seat and `FusionRegionClaim` consumption are
> unchanged — **the composition relation is not permission to spend the fusion
> claim early.**
>
> **What the trace ruled out, so it is not re-proposed:** downstream forwarding
> has no source value to forward; a specialization-local join would move the
> outer eliminator, redirect and claim spend away from their measured seats and
> need a second skip relation in the caller. And `StaticWorkerBinding` still
> gets **no** runtime representation, carrier, slot, descriptor, token or ABI
> member.
>
> **THIS IS NOT THE WITHDRAWN SUBSUMPTION AND NOT GENERIC INLINING.** The
> authority is **one opaque exact `ContinuationCallIdentity`**, never a
> target/body/origin/owner coincidence. The join is re-derived from existing
> planner relations and **may not** search by constructor spelling, body, target
> alone, shared origin, arity, ambient `defining_unit`, or whichever candidate
> exists. Zero, ambiguity, duplicate domain, a call claimed by two fusions, a
> fusion edge claimed by two incompatible calls, wrong emission owner, or
> disagreement with either source relation **refuses before mutation**.
>
> **THE POPULATION RULE IS THE LOAD-BEARING CORRECTION.** For every
> specialization, preflight enumerates its **complete** incoming
> `ContinuationCallIdentity` domain and partitions it into composed and ordinary
> direct edges. A specialization leaves declaration, definition and target
> resolution **only when every incoming identity is proved composed at its own
> call edge.** One residual direct caller keeps it live; a same-body sibling is
> independent. **No fusion-body ownership fact, `enclosing_specialization`
> match, or absence at one caller is sufficient to remove a target** — that
> inference is exactly what was withdrawn.
>
> ### THE DERIVATION CLOSES, AND THE RELATION WAS ALREADY IN THE FILE
>
> **Grounded at `evt_62e7r7herf08c`: no new source fact is needed.** The two
> composed edges are named exactly by the fusion key's two `CheckedIhBinding`s —
> `consumer_binding.frame_origin` selects the OUTER layer, and
> `producer_argument_binding.frame_origin` the INNER (Exact 10/25, ReHomed
> 6/21). Each conjoined with the claim's `consumer_owner` / `producer_owner`
> selects exactly one specialization, disjointly, from already-checked planner
> facts in hand at preflight. **Mutation proof:** swapping the two bindings
> takes both selections to zero, so the row is keyed on which binding names
> which layer.
>
> **The preflight comment already recorded these bindings as "different checked
> frames by design (measured 25 and 10)", warning that asserting them EQUAL
> would refuse the very witness the class exists for. They differ because they
> are the two composition layers** ⇒ **the fact that forbids one check is what
> supplies this one.**
>
> **A structural substitute was found and rejected, and that was the harder
> call:** `child_origins(producer_body)` is a one-element run holding the inner
> frame on both witnesses. **It agrees and would have passed both witnesses** —
> and it reads the body's shape rather than a checked relation, so it stops
> agreeing the moment a producer body carries more than one child.
>
> ### STEWARD SEQUENCING — FIXTURES FIRST (`evt_4gq61qp9cg5xa`)
>
> **Every specialization's incoming domain is a SINGLETON on both witnesses, and
> that singleton is the composed edge.** ⇒ the partition is satisfied
> **vacuously**: *"every incoming identity is proved composed"* holds for **any**
> implementation that composes the one edge, **including one that would remove a
> target with a residual direct caller.** Green would carry no information, and
> the vacuous thing would be the safeguard itself.
>
> **So the two blocked controls get their fixtures first** — a witness with a
> residual non-fusion direct caller, and one with a same-body sibling. **The
> objection that this means authoring against a relation that does not exist
> dissolves once the fixtures are seen as programs rather than as tests of the
> relation:** their correct behaviour today (specialization declared, defined,
> target-resolved, called) **is exactly what must still be true afterwards.**
> They land as ordinary positive coverage on the current tree and become the
> partition's discriminators without being rewritten. If a fixture cannot be
> built in the `D2j` family without a new capability, **stop and report** — do
> not reshape the witness to make it authorable.

> ### THE RECUT IS ACCEPTED AS EXACT NEGATIVE EVIDENCE, AND THE NEXT ROUTE-C
> ### MECHANISM IS HELD FOR ONE MORE BOUNDED TRACE. Steward, 2026-08-13,
> ### recording Architect `evt_mt5zxq1qd6bt` on tip `e30d33b9`.
>
> **The recut landed exactly** (`evt_6qgv6q1sbq88m`): both withdrawn mechanisms
> removed, `continuation_units()` restored across all four projections, the
> specialization declared/defined/target-resolved/called once and settled
> `DirectCall`. **`git diff <rebased-pre-withdrawal> -- crates/` is EMPTY** —
> the implementer proved the revert byte-exact rather than asserting it, and
> that check caught two stray blank lines. Suite 909, down from 911 by exactly
> the two controls that pinned withdrawn machinery. **The control that pinned
> the subsumption was DELETED, not retargeted** — it asserted the thing the
> ruling reverses, and a retargeted version would have been a test written to
> survive its own refutation.
>
> **THE CLASSIFICATION IS RE-AFFIRMED AND IS STILL R3-OWNED.** A
> `StaticWorkerBinding` is **compiler-only transport** from an exact constructor
> template to its exact static eliminator and call. **It may not acquire a
> runtime representation, carrier, slot, descriptor, token, or ABI member, and
> `ground_value` must continue to refuse it.**
>
> **WHY NO SIXTH MECHANISM YET, and this is the sharpest finding of the arc.**
> The prior one-counter trace labelled its seats with **ambient
> `defining_unit` / fused authority** — but `define_continuation_bodies`
> deliberately binds the semantic consumer owner into that ambient state **while
> emitting a distinct generated Cranelift `Function`.** ⇒ **Those labels do not
> prove which emitted function owns the continuation call, the refusal, or the
> redirected call.** Nor does the trace prove what the direct call's returned
> SSA value does next. Four outcomes are open — exact operand of the redirected
> call; only the carried scrutinee/control predecessor; consumed wholly inside
> the specialization; or absent from the redirected operand run — **and they
> require different lawful joins.**
>
> **AUTHORIZED: exactly one measure-only armed trace** over both witnesses,
> recording the **actual emitted Cranelift function/emission owner** rather than
> ambient `defining_unit`; minting an opaque probe identity for the direct
> call's returned SSA value; tracing every use of it; classifying into exactly
> one of the four cases; and reporting whether the specialization frame already
> holds every planner-authorized continuation input the redirect needs. **No
> repair, no retarget, no new relation or representation, no ABI change, no
> substitute operand, no grounding the worker, no control changes.**
>
> **THE MEASURE-FIRST DISCIPLINE IS NOW THE ARCHITECT'S OWN PRACTICE, not a
> Steward intervention.** It ruled this hold unprompted. ⇒ **A future Steward
> does not need to re-rule sequencing on this seam each cycle** — raise it only
> if a mechanism is prescribed with an unmeasured premise. The reason it stuck
> is in the implementer's own words: the single-run trace *"did not merely
> choose between my two branches, it eliminated the whole class."*

> ### `FusionForward` IS WITHDRAWN, AND SO IS CONTINUATION-SPECIALIZATION
> ### SUBSUMPTION. Steward, 2026-08-13, recording Architect
> ### `evt_27qhdnnmv4h4z` (durable state `fd62eb5d`).
>
> **Read the ruling, not this box.** What follows is the frame consequence,
> which is mine to record: **this changes what the node's deliverables are, so
> anything below describing a forward or a subsumption is superseded.**
>
> **THE ARCHITECT WITHDREW ITS OWN PRIOR RULING, on the stated ground that it
> rested on a false temporal premise.** `evt_713gc922d1d7g` introduced an exact
> call-identity-to-fusion forward. The measured order is the other way: at the
> proposed forward seat the fused result **does not exist**, the direct
> continuation-specialization call is emitted next, and only after that does the
> consumer emit the redirected producer-body call into the fusion. ⇒ **The
> direct call is a causal INPUT to the fusion invocation, not a residual
> duplicate of it.** A source relation cannot repair this — it would name a
> future value.
>
> **TO REMOVE:** `CandidateDisposition::FusionForward`, the
> `fusion_forwarded_calls` relation, its target-resolution skip, and the operand
> guard. **TO RESTORE:** the exact continuation specialization stays
> **executable** — declared, defined, target-resolved, called once at its
> existing seat, settled as `DirectCall`. `install_fusion_owned_bodies` may own
> the claimed producer body but **must not infer from that ownership that the
> matched specialization is dead**; the measured direct caller proves the
> opposite.
>
> **THE DURABLE INVARIANT IS A DATA-FLOW JOIN, NOT A FORWARDING RELATION.** The
> direct call's result must be the actual SSA/data-flow predecessor consumed by
> the redirect. **Do not reconstruct it from a constructor, select it by operand
> shape, copy it across functions, or settle the candidate before the direct
> call has returned.** The redirect does not move either: before the direct call
> returns it lacks the value its existing ABI path consumes.
>
> **`a3c25dae` IS NEGATIVE EVIDENCE, AND THAT IS ITS VALUE.** The mechanism
> built there is withdrawn; **the measurements are not.** The one-run ordering
> trace and the guard's operand finding are what retired an entire branch, and
> they are recorded here and in the thread precisely so a recut that deletes the
> code does not strand them.
>
> **THE `StaticWorkerBinding` REFUSAL RETURNS, AND IS NOT A REGRESSION.** With
> the live specialization restored it is again the next honest boundary inside
> route C. The ruling forbids grounding or transferring `StaticWorker`, adding
> an ABI member, or guessing a new routing mechanism in this recut: rebase onto
> `main`, remove the invalid subsumption and forward, restore the direct-call
> path, re-run the bounded armed witnesses, **then stop and report.**
>
> ### THE STANDING DISCIPLINE FOR THIS SEAM — Steward, and it is sequencing
>
> **Five mechanisms have now been prescribed here and five premises have been
> refuted by measurement:** terminalization (`evt_2m62086x60c94`), the generic
> funnel (`evt_5rze80e6w9qz8`), receipt ordering (`evt_26ganh9p86xw8`), the
> forward's operand premise (`evt_6e58apgyy09dc`), and the forward's temporal
> premise (`evt_7ydbavjbtxx97`). **The classification has survived all five
> untouched.** What fails is the layer beneath it, the same way every time:
> prescribed against the emitter's documented intent, refuted by the measured
> control flow.
>
> ⇒ **No further mechanism is prescribed for this seam without its premise
> measured first.** Where a residual is a question of fact whose branches have
> different repairs, the bounded probe runs before the ruling — my sequencing
> calls `evt_41remx6xn6pda` and `evt_6j04882rsx096`, both of which paid for
> themselves in minutes against a full ruling-and-dispatch cycle. **A guard that
> refuses on its own stated premise is the mechanism working, not a failed
> turn**; do not let a recut delete such a guard to make a path go green.

> ### STOP 4 IS RULED — `AC-D3-ROUTE` — Steward, 2026-08-12, recording
> ### Architect `evt_2m62086x60c94`. THE ATOMIC BOUNDARY DOES NOT MOVE.
>
> **Full ruling is at `evt_2m62086x60c94`; the leader's dispatch is
> `evt_5e6agmqhwq4d2`. Read the ruling, not this box.**
>
> **The `StaticWorkerBinding` stop is an R3 ROUTING DEFECT, not an external
> value-representation gap.** I routed this as a scope fork at
> `evt_7r043m92mz7fb` and supplied the measurement that R3's range does not
> touch the refusal site. **The ruling uses that fact and rejects the inference
> from it**, in terms worth quoting because it is the trap: *"the ownership of
> the refusal site is not the ownership of the defect"*, and explicitly **"do
> not declare the stop external merely because R3 did not edit the refusal
> site."** ⇒ **My measurement was the right fact to supply and would have been
> the wrong basis to rule on. The frame's earlier statement that it "settles
> neither reading" stands as written.**
>
> **Why the refusal is correct and stays.** `emit_result -> ground_value` is the
> terminal-result boundary and `RuntimeGroundValue` is an intentionally **closed**
> domain of observable compile-time values. `ConstructorField::StaticWorker` is
> intentionally compiler-only — lawful only while transported through a
> constructor template to the kind-preserving static `Match` binder, then
> consumed by the exact-`Var` call. **The guard is a conservation guard, and it
> is doing its job.**
>
> **What the armed path is actually proving when it stops there.** The inner
> selected producer builds the intermediate `D2gOut::Node([Var(0)])` whose field
> is the recursive worker; the outer selected case eliminates that same
> constructor and calls the rebound field; the leaf produces the observable
> terminal `Result::Ok(Unit)`. **A worker-bearing `D2gOut` is not the program
> result — it is the intermediate whose outer consumer suffix R3 claimed and
> moved.** ⇒ ~~Reaching `ground_value` with that intermediate intact **proves
> the fused path terminalizes the producer result before completing the claimed
> consumer suffix.**~~ **MEASURED FALSE — see the box below.** R3 changed the
> flow into an unchanged site, and the flow is unlawful.
>
> **`AC-D3-ROUTE` — the repair stays inside the held atomic object:** ~~the
> fused path carries the intermediate through the already-authorized outer
> elimination, kind-preserving rebind, and exact worker call, **and only then**
> emits the resulting ground value.~~ **THE REPAIR IS UNDER RE-RULING — DO NOT
> BUILD THE STRUCK TEXT.** The existing conservation relation stays
> authoritative and **must close on the armed fused path**.
>
> > #### THE CLASSIFICATION HOLDS; THE CAUSAL SENTENCE UNDER IT IS MEASURED
> > #### FALSE. Steward, 2026-08-12, from `evt_6bg3en6yy4dgz` at `766cbdf0`.
> >
> > **This is my correction to make because I wrote the mechanism into
> > `AC-D3-ROUTE` above.** The struck text was a faithful transcription of the
> > ruling; the ruling's *causal story* is what four probes contradict.
> >
> > **What was measured under a temporary arm:** the fused suffix lowering
> > returns **`Carried`** — a runtime call result, **not** a compile-time
> > constant, so it terminalizes nothing; the consumer takeover forwards that
> > `Carried` result, so **the redirect RAN and the claimed suffix WAS
> > consumed**; the escape's backtrace is `emit_result` called straight from the
> > **root projection**, not from inside the fused function; and
> > `require_complete_static_worker_disposition()` **passes** immediately
> > before it.
> >
> > ⇒ **The worker-bearing constructor reaching `ground_value` is a DIFFERENT
> > OCCURRENCE from the one the suffix consumed**, arriving as the **root
> > answer** under `FunctionizedUnits`. Field origin **29** on `Exact`, **25**
> > on `ReHomed`.
> >
> > **WHY THE STRUCK REPAIR WOULD HAVE BEEN WRONG, and this is the whole point
> > of the strike.** Read against the stated mechanism, *"carry the intermediate
> > through the outer elimination and only then emit"* says the takeover
> > forwards too early, so the obvious implementation is **to stop forwarding**.
> > Measured, **the takeover forwards a call result and is CORRECT** — changing
> > it breaks working code and **reintroduces the double-suffix defect at `:650`
> > that the forward exists to prevent.** ⇒ **A repair that would have looked
> > ruled and been wrong.** The implementer stopped rather than build it; that
> > is the correct handback and I am recording it as such.
> >
> > ~~**THE OPEN QUESTION, back with the Architect at `evt_2fanpwder54a0`, and
> > deliberately not answered here:** is the second root occurrence **wrongly
> > produced**, or **wrongly selected as the answer**?~~ **ANSWERED — it is
> > WRONGLY SELECTED. `AC-D3-ANSWER` below.** The implementer did not guess and
> > neither did I; the fork was real and the Architect resolved it.
> >
> > **CORRECT MY "DIFFERENT OCCURRENCE" ABOVE BEFORE YOU ACT ON IT — the
> > Architect called this precision load-bearing** (`evt_5rze80e6w9qz8`).
> > **`29`/`25` is NOT a second same-spelling source constructor.** It is **the
> > claimed producer source occurrence ITSELF, entered again by a distinct
> > lowering traversal / dynamic construction.** ⇒ *"different occurrence"* is
> > true **only at the dynamic construction / route-instance axis**, and **false
> > at the planner source-origin axis.** The planner coordinates on `766cbdf0`:
> >
> > | case | claimed producer | the OTHER same-spelling construct |
> > |---|---|---|
> > | `Exact` | construct **30** / field **29** | construct 39 / field 38 |
> > | `ReHomed` | construct **26** / field **25** | construct 35 / field 34 |
> >
> > **Constructor spelling and origin equality remain non-selectors**, and
> > **repeated entry of one static field origin stays legal and
> > instance-scoped.** Anyone reading my earlier sentence as "hunt for a second
> > source constructor" is hunting the wrong thing.
> >
> > **UNAFFECTED EITHER WAY, so do not re-open them:** the classification (an R3
> > routing defect, not a representation gap), the closed `RuntimeGroundValue`
> > domain, the conservation guard itself, and **the atomic boundary** — nets
> > 2/4, controls 2/3, the inner-slot witness, the routing discriminator and the
> > self-edge closeout all remain owed **unshrunk**.
>
> ### DO NOT BUILD `AC-D3-ANSWER` AS WRITTEN — ITS MECHANISM IS MEASURED
> ### INERT AND ITS ROWS 3/4 ARE INVERTED. Steward, 2026-08-12, from
> ### `evt_1twk028k03mxe`; re-ruling requested at `evt_7snn4fr8b4yea`.
>
> **This box sits ABOVE the AC because the AC below reads as buildable and is
> not.** The implementer grounded the ruling **before** implementing, and the
> grounding refuted the prescribed mechanism. **No code was written; the held
> object is unchanged at `766cbdf0`.** Stopping there is the condition the
> leader authorized.
>
> **MEASUREMENT 1 — THE RULED FUNNEL HAS NOTHING TO RESOLVE FOR THE CLAIMED
> PRODUCER.** `claim_and_call_continuation` resolves through
> `continuation_call_binding_for(producer_construct_origin, ...)`. Armed, on
> `Exact`, the planner's continuation-call projection contains **exactly two
> entries and neither names construct 30**:
>
> ```
> (producer_construct=36, continuation=25, alt=0, pos=0)
> (producer_construct=39, continuation=10, alt=0, pos=0)
> ```
>
> ⇒ For construct **30** the funnel returns `None` at the binding lookup, the
> repair falls through to *"retain byte-identical ordinary direct
> construction"*, **and the direct template still reaches root projection.**
> **The prescribed fix is INERT on the very witness its own discriminator row 1
> names.**
>
> **MEASUREMENT 2 — THE ROLES ARE INVERTED; two DIFFERENT relations name the
> two constructs and each is authoritative for its own.**
>
> | relation | names |
> |---|---|
> | **`Fusion` claim** | `producer_construct_origin = 30`, `consuming_call = 17`, `producer_body = 37` |
> | **continuation call** — *the relation the ruled funnel keys on* | `producer_construct_origin = 39`, `continuation_origin = 10` |
>
> `ReHomed` is the same shape: fusion-claim producer **26**, continuation-call
> producers **35** and **32**. ⇒ **Row 4 asks me to prove non-authoritative the
> one construct the prescribed funnel IS authoritative for**, and **row 3's "no
> exact binding" case is the CLAIMED producer**, not the other one. **Rows 3 and
> 4 must be restated against these coordinates before either can be built.**
>
> **MEASUREMENT 3 — WHERE THE ESCAPING TEMPLATE IS ACTUALLY BUILT, which
> narrows the defect.** The construct-30 completion at that seam runs with
> `fused_authority = Some((StaticOriginId(10), PredeclaredFunctionId(3)))` and
> `defining_unit = Some(2)` — **inside the `Fusion(0)` definition, on the
> producer half, with the region claim still outstanding.** So this is **not a
> stray traversal outside the fused object**: the fused body itself completes
> the producer's constructor through the source-machine route and hands back a
> direct specialized template, while the suffix separately returns `Carried`.
> **Construct 39 never reaches this seam on the armed path at all** — it is the
> consumer's scrutinee the fused call replaced — **which is why a row keyed on
> 39 cannot discriminate anything here.**
>
> **WHAT THE IMPLEMENTER DECLINED TO DO, and it was right to.** Substituting the
> fusion claim for the continuation-call relation as the funnel key is the
> obvious move and is **exactly the choice the ruling reserves**: it would make
> the source-machine constructor completion consult `FusionRegionClaim` and
> select an answer from it — **a new answer-selection authority inside the fused
> body**, not the *"existing funnel"* the ruling authorized reusing. It also has
> to interact with the **affine claim, which is outstanding at that point and
> must not be spent twice**, and with the takeover that already forwards the
> correct `Carried`. **Getting it wrong reintroduces the double-suffix defect at
> `:650` or spends the claim at the wrong seat — this node has already produced
> each of those once.**
>
> **THE FORK, stated as a fork and not a recommendation, with the Architect:**
>
> 1. **Should the fused body's producer half reach the source-machine
>    constructor route AT ALL**, or should the producer construct inside
>    `Fusion(0)` lower through the **generic eliminator route** that already has
>    the claim/call funnel and the eliminator frame?
>    `lower_fused_producer_through_suffix` invokes the generic route, **so a
>    producer construct arriving at the source-machine seam may itself be the
>    routing defect, one level up from answer selection.**
> 2. **If the source-machine seam is the right place, which relation keys it**,
>    given the continuation-call relation demonstrably does not name the claimed
>    producer?
>
> **Different repairs, different discriminators, both consistent with every
> measurement above.**
>
> ### THE ATOMIC BOUNDARY IS RE-PRICED AND DOES NOT SHRINK — SETTLED.
> ### Architect `evt_6xb2jnracee7x`, re-priced against `main` = `689dabd7`.
>
> **The Steward asked (`evt_2v4sdvr74yme2`) whether in-tree unarmed controls had
> satisfied `evt_4m0q1m4zn4k79`'s "may not merge unexercised" precondition for
> part of the object. The answer is no. This is CLOSED — do not re-raise it.**
>
> **THE CRITERION, and it is the reusable part: a unit-tested component law is
> NOT the positive for the behavior-changing production slice that depends on
> it.** Every control cited exercises an invariant **directly**, not the
> production route that invariant governs:
>
> | control | what it actually exercises |
> |---|---|
> | control 1 | a full-pipeline **negative baseline** — never exercises the composed positive |
> | `dp_composition_time_membership...` | validates and round-trips a **manually populated** `composed_frame_templates`; **`DP`'s production positive is still behind the arm** |
> | selector net 3 | the ledger's ordinary/one-spend/replay/forged/monotone laws — its own comment says the **unconsumed refusal** lives in armed splice closeout |
> | `AC-D3-SELF` | installs a plane, preflights, calls `fusion_self_edge_identities` — **emits no fusion definition, exercises no recursive self edge** |
> | binder-to-body | installs its plane directly; production runs `preflight` over an **empty** installed population while `D2F_EMITTER_ARMED` is false |
>
> **STILL-UNEXERCISED PRODUCTION OBLIGATIONS, as ruled:** `DP` source-authored
> membership on the live composed segment; `D1` authority switching; `D2`
> checked-frame adoption; production fusion installation, claim issuance,
> ownership, definition, redirect, takeover and closeout; selector nets 1/2/4
> plus the armed half of net 3 and the inner-slot witness; the emitted recursive
> self edge; the exact route-C continuation-specialization disposition;
> `AC-8`/`AC-9`/`AC-10`; and `D4` under the necessary-green rule.
>
> **No partial cut from `766cbdf0` is authorized.** The first test-only baseline
> commit could be copied out harmlessly, but it **shrinks no executable
> dependency and does not warrant another candidate or review hop.**
>
> **RING INSTRUCTION:** continue the ruled route-C implementation; **rebase the
> whole held range onto current `main` at the next natural seam; ARM LAST.**
>
> ### RESEARCH ADVISORY — CONSUMPTION AND REACHABILITY ARE TWO DISTINCT PROOF
> ### OBLIGATIONS. Steward, 2026-08-12, carrying `evt_77kkd21bnnekj` (full
> ### text there) to the thread at `evt_33vnf0f1pmvgj`.
>
> **Prior-art framing only. Research proposed no repair, did not read
> `crates/`, and states that it does not select Ken's mechanism. It is
> advisory and non-blocking, and it must not be read as a ruling.**
>
> **The bottom line:** *"DEAD is a property of the residual executable program,
> not a consequence of having consumed one fused edge."* Prior art establishes
> producer deadness three ways — a rewrite replacing the whole redex so the
> occurrence is **structurally absent**; a use-edge fusion that **retains** the
> producer while any uses remain and erases it only when the use set is empty;
> or a later **reachability/DCE pass** over the residual graph. **An ownership
> model can make the first true by construction, but the fusion claim is not
> normally the liveness certificate — the residual graph is.**
>
> **The discriminator that maps onto our measurements.** In GHC, a let-bound
> producer used by both `foldr` and another consumer **does not fire the fusion
> rule**, because fusing one use would duplicate the producer's work. ⇒ **one
> consumer can consume a producer's result while the producer binding stays
> live through another occurrence.** GHC keeps **fusion rewrite and binding
> deadness as separate pipeline facts**; MLIR Linalg and OpenXLA split them the
> same way, erasing a producer only when it is known to have no uses.
>
> **The named failure shape, and it is ours:** *"Incorrect residual ownership:
> the original remains reachable even though no legitimate surviving use
> requires it... **This is not an internally incomplete fused object. It is an
> extra live node or root in the residual executable population.**"*
>
> **TRANSFER.** Our affine claim establishes **dynamic consumption** — fused
> edge and suffix used exactly once. It does **not** establish **residual
> unreachability** — that no executable root, standalone specialization or
> export still reaches the original producer occurrence. **Prior art treats
> these as two distinct proof obligations.** ⇒ The terminal refusal is
> consistent with **an orphan residual caught after a CORRECT fusion**, rather
> than with answer selection inside the fused object being wrong — **which is
> why all three mechanisms aimed at answer selection had nowhere to attach.**
>
> **A disposition prior art contains that our option set does NOT, flagged and
> explicitly not proposed:** GHC's answer to a producer with a surviving second
> use is **to decline the fusion**. Our three candidate readings — route C runs
> / its result is selected / ownership left it reachable — do not include it.
> **Recorded so the space is closed deliberately rather than by omission.**
>
> **Research found no dependently-typed eliminator-fusion precedent in this
> bounded pass that changes the conclusion, and said so rather than padding.**
>
> ### THE TRACE LANDED, AND IT FOUND A THIRD CONSTRUCTOR ROUTE NOBODY HAD
> ### BEEN RULING ABOUT. Steward, 2026-08-12, from `evt_1a9t81p0j7h77` +
> ### `evt_338x5qd0hf6fx`, under bounds `evt_41remx6xn6pda` + `evt_7qkp9qd7a2381`.
>
> **Read this box before any of the stop-4 boxes below it. It retires the
> question they were all built on.**
>
> **THE ONE-MACHINE-OR-TWO QUESTION IS ANSWERED AND RETIRED: one machine,
> joined, and CORRECT.** The fused body starts one machine for the producer's
> selected case body; construct **30** completes inside it with
> `next=["Terminal"]` — **and that `Terminal` IS the `ResumeOuter` the machine
> was started with**, so it resumes the pending outer eliminator, which runs the
> suffix as nested machines under the **consumer's** defining unit. The
> producer's machine then exits carrying the suffix's result and the fused body
> exits `Carried`. ⇒ **The join exists, it is the `Terminal`/`ResumeOuter`, and
> nothing about it is broken.**
>
> ⇒ **AND IT RETIRES THE SHARED PREMISE OF EVERY REPAIR SO FAR:** there is **no
> point in that nesting at which two candidate answers coexist and one is
> selected.** **There is no answer selection to repair at any fused seat** —
> which is exactly why a funnel and a receipt each **had nowhere to attach.**
> Three mechanisms failed for one reason.
>
> **THE ESCAPE IS A THIRD PRODUCTION OF THE SAME SOURCE OCCURRENCE, BY A ROUTE
> NOBODY HAS RULED ON.** The three constructor-completion routes:
>
> | route | what it is | verdict |
> |---|---|---|
> | **A** | `lower_computational_producer_expr`'s `Construct` arm — the generic funnel with `claim_and_call_continuation` | every ruling so far aimed here |
> | **B** | the owned source-machine `ConstructArgument` completion | ...or here |
> | **C** | **`lower_expr`'s `RuntimeExpr::Construct` arm** — documented in place as *"the ONE armed producer"*, **the sole builder of the `StaticWorker` arm** via `static_worker_constructor_template` | **this is the one that escapes** |
>
> **Route C is the only one of the three that can produce a `StaticWorker` field
> at all** — its own comment records that route A's two `Construct` producers
> stay fail-closed at `value_at`.
>
> **WHERE AND WHEN IT FIRES, which is what makes it a separate object.** Route C
> emits at `machine_depth=0`, `fused_auth=None`, in **root-projection entry
> `block3`**, with the enclosing emission identified by backtrace as
> **`compile_expr_into_module_with_root_projection`** — **not a unit body and
> not the fused definition.** It runs **after** the fused body exited `Carried`
> and **after** the takeover forwarded `Carried`. **At that point the affine
> claim is already SPENT and the fused definition is already complete**, so
> route C is **downstream of the entire affine lifecycle rather than racing
> it.**
>
> **Answer sinks, both traced:** the fused definition's result-store input is
> `Carried` on **both** witnesses; **route B's direct template is consumed
> inside the machine and never reaches a sink at all**; route C's template goes
> straight to `emit_result`/`ground_value` with **no other consumer**, never
> entering the fused definition. ⇒ **Independent emissions in DIFFERENT
> FUNCTIONS — not alternative CFG arms, not sequential values on one lineage.**
>
> **A TRAP THE IMPLEMENTER FLAGGED BEFORE ANYONE COULD FALL INTO IT:** at the
> route-C emission `defining=2` is **ambient carrier state, NOT the enclosing
> emission** — **the backtrace is the authority and the two disagree.** ⇒ **Do
> not key anything at that seat on `defining_unit`.**
>
> **WHAT IS STILL OPEN, and it is narrower than anything before it:** why does
> the root projection lower the claimed producer occurrence **at all** after
> body ownership and fused consumption — and is the defect that **route C
> runs**, that **its result is selected as the program answer**, or that
> **ownership left the occurrence reachable from the root**? **Different
> repairs. Nobody has probed or picked between them.** The Architect is
> grounding the root projection's ownership/selection contract
> (`evt_7z2jq0k1g42np`).
>
> **THE SEQUENCING RULING IS VINDICATED AND SHOULD BE REUSED.** The Architect
> accepted evidence-first without argument (`evt_5mj370jt0ax14`: *"no fourth
> repair ruling will issue before the causal trace"*), **tightened the trace's
> bounds instead of ruling** (`evt_7qkp9qd7a2381`), and **one trace in two cuts
> retired a question three full ruling cycles could not settle** — and surfaced
> a route that was never in anyone's field of view. **Cost: one implementer turn.**
>
> ### THIRD MECHANISM, THIRD REFUTATION — and my sequencing ruling
> ### `evt_41remx6xn6pda`: THE CAUSAL TRACE RUNS BEFORE THE NEXT MECHANISM
>
> **Steward, 2026-08-12, from `evt_7kke2mm0sncgw`. The re-ruling
> `evt_26ganh9p86xw8` replaced the inert funnel with a fusion-scoped,
> instance-paired ANSWER RECEIPT. Its relation separation is MEASURED CORRECT
> and stands** — `30`/`26` authoritative only through the fusion projection,
> `39`/`35` and `36`/`32` only through `continuation_call_binding_for`, neither
> substitutable. **What fails is step 2 against step 3.**
>
> **THE RECEIPT CANNOT EXIST AT THE SEAT RULED TO CONSUME IT.** Step 2 mints it
> *"when the exact claimed `Var` consumer call completes"*; step 3 has the owned
> source-machine completion select it. **That requires the call to PRECEDE the
> constructor completion. Measured, it is strictly the other way** — one armed
> monotone counter, both witnesses:
>
> ```
> [0] srcmachine-construct-complete origin=30 fused_auth=Some((10, unit 3))
> [1] [2] [3] ih-call-mint
> [4] srcmachine-construct-complete origin=26 fused_auth=Some((6, unit 1))
> [5] [6] [7] ih-call-mint
> ```
>
> There is **exactly ONE** source-machine completion of the claimed producer
> inside the fused body per compile, and **every IH call mint follows it.** ⇒ At
> the only seat step 3 can act **no receipt has been minted**, the "no matching
> receipt" fallback is **the branch that always runs**, the reconstructed
> template still reaches root emission, **and all six controls pass vacuously.**
> **The inert-fix shape again, one layer in — and this time the controls would
> not catch it, because they are written against a mechanism that never fires.**
>
> **WHY, and it is not an ordering accident.** At that completion the
> continuation chain is `origin=30, fused body: next=["Terminal"]` — **no
> pending `ComputationalMatchScrutinee` on that machine's stack.** The outer
> eliminator is pending as *authority* (`fused_auth` is set) but is **not on this
> machine's stack**, so the constructed template **is that machine's final
> answer**, and nothing downstream can consume it — which is also why no later
> receipt could be applied retroactively. Contrast a genuinely consumed
> construct: `PX8J construct 28: next=["ComputationalMatchScrutinee",
> "ApplyRecursorSelection", "Terminal"]`.
>
> **THE CONTROL THAT STOPS THE OVER-CLAIM, produced against the implementer's
> own tidier story and worth more than the finding:** the **same** construct 30,
> same fixture, **no installed fusion**, is **also** `next=["Terminal"]` **and
> that compile does not escape.** ⇒ **"Terminal at the producer's constructor"
> is NORMAL and is not by itself the defect.** What differs under fusion is that
> this machine's terminal answer becomes the fused function's answer while the
> suffix's correct `Carried` is produced on a **different route** — the one the
> takeover forwards, which remains correct.
>
> **THE QUESTION THE THIRD REFUTATION LEAVES STANDING, and it is the one to
> rule:** at that seat, **are the producer machine and the suffix ONE machine,
> or TWO with a defined answer join?** **Every repair proposed so far — funnel
> reuse and receipt selection alike — presumes they are already one machine with
> a selection point, and measured they are not.**
>
> **The two adaptations the implementer declined, both correctly, both the
> reserved decision:** deferring the constructor's answer so a later receipt can
> replace it — a **promise-and-patch** mechanism, not the *"select that returned
> answer"* that was authorized, and it settles the fused body's answer **after
> its own machine terminated**; or pushing the outer eliminator onto the producer
> machine's stack, which **changes which route runs the suffix** and risks the
> `:650` double-suffix defect and a **second spend of the affine claim** — this
> node has produced each once already.
>
> **MY SEQUENCING RULING (`evt_41remx6xn6pda`), and it is sequencing only.** The
> implementer runs and posts a **bounded causal trace of the fused body's
> machine structure BEFORE the next mechanism is chosen**: scoped to the
> one-machine-or-two question, both witnesses, carrying the consumed-construct
> contrast **and** the no-fusion control, **measuring only — no proposed
> repair.** The Architect rules **against the trace rather than ahead of it**,
> and may re-bound the trace instead. **This reorders evidence and decision. It
> shrinks no scope, adds no party, and adds no review hop.**
>
> ### THIS IS THE SECOND CONSECUTIVE RULING REFUTED BY GROUNDING BEFORE
> ### IMPLEMENTATION — and both times the frame carried the refuted mechanism
>
> **Steward, 2026-08-12. Recording it because the sizing consequence is mine.**
>
> Stop 4 has now been ruled twice and refuted twice: `evt_2m62086x60c94`'s
> causal sentence fell to four probes, and `evt_5rze80e6w9qz8`'s prescribed
> funnel is inert on its own witness. **Both times I transcribed the mechanism
> into this frame as an AC, and both times a seat reading my box would have
> built the wrong thing.** The classification has survived both refutations
> unchanged; **it is the causal/mechanism layer beneath a correct
> classification that keeps failing.**
>
> ⇒ **Two standing consequences.** For the ring: **ground a prescribed mechanism
> against the armed coordinates before writing code, and report the refutation
> rather than substituting a mechanism of your own** — that is now twice
> vindicated and is not over-caution. For me: **an AC transcribed from a ruling
> inherits that ruling's mechanism risk**, so a mechanism I did not measure gets
> a box like this one **above** it, not silent inclusion beside criteria that
> are measured.
>
> ### `AC-D3-ANSWER` — STOP 4 RE-RULED. Steward, 2026-08-12, recording
> ### Architect `evt_5rze80e6w9qz8`, durable at `9fd0731e`.
> ### SUPERSEDED IN ITS MECHANISM — read the box above first.
>
> **Leader's dispatch is `evt_5s5hkcjr0e2c`. This SUPERSEDES the struck
> `AC-D3-ROUTE` repair; the classification and the atomic boundary are
> unchanged.**
>
> **The defect is ANSWER ROUTING at owned source-machine constructor
> completion** — not unlawful source production, and **not a fault in the fused
> takeover.** The Architect withdrew its own causal sentence: the fused call
> returning `Carried`, the takeover forwarding it, suffix consumption and the
> worker-disposition close are **all correct and must remain unchanged.**
>
> **The two construction routes, which is the whole mechanism:**
>
> 1. **Generic `lower_expr`** lowers the constructor fields, attempts the exact
>    continuation claim/call, constructs the ordinary fallback, then selects
>    `continuation_result.unwrap_or_else(|| RoutedAnswer::direct(produced))`.
>    ⇒ **a successful claimed call OWNS the answer.**
> 2. **The owned source machine** lowers the same `Construct` through
>    `SourceContinuation::ConstructArgument` and then **unconditionally** returns
>    `RoutedAnswer::direct(finish_source_constructor(...))`. ⇒ **it has no
>    equivalent exact-call answer choice at all.**
>
> Under `FunctionizedUnits`, **route 2 is the direct specialized value the root
> projection sees.** Entering the producer occurrence, lowering its fields, and
> having an ordinary fallback are all lawful. **What is wrong is that fallback
> remaining the SELECTED answer when the exact plan-authored claim resolves and
> its call result exists.**
>
> **`AC-D3-ANSWER` — the lawful repair boundary.** Close the owned
> source-machine constructor-completion seam with **the same authoritative
> claim/call decision generic `lower_expr` already uses**: after all fields are
> lowered, resolve the exact continuation identity **from the planner relation**
> and pass through the **existing** claim/call/settlement funnel. If the exact
> call succeeds, its returned `RoutedAnswer` — **including the `Carried` result
> and its checked route** — replaces the direct constructor fallback. If no
> exact binding exists, ordinary source-machine construction stays
> **byte-for-byte** the direct fallback. **Prefer one shared constructor-answer
> funnel over two independently maintained spellings, if that can be done
> without widening the surface.**
>
> **This is selection AFTER lawful evaluation, not "do not produce origin
> 29/25."** Keep source-occurrence entry, field lowering, recognition, rebind,
> exact consumption, and the ordinary fallback.
>
> **Forbidden, and the first five are the tempting ones:** suppressing the
> constructor by source origin, constructor name, root status, or fusion-body
> context; stopping or delaying consumer takeover; replaying the suffix;
> changing `FunctionizedUnits` root pairing; turning `StaticWorker` into a value
> or carrier; weakening `ground_value`.
>
> **WHY THE LEDGER PASSING IS NOT A CONTRADICTION, and why it must not be made
> to carry this.** `require_complete_static_worker_disposition()` succeeding
> before the escape is **consistent** with the diagnosis: that ledger proves
> **recognition → transition → consumption**. It does **not** prove that a
> compiler template containing the now-consumed field was not **selected later
> as an answer**. ⇒ **R3 adds the answer-exclusivity control; it does not
> burden the conservation ledger with a routing fact.**
>
> **THE SIX REQUIRED DISCRIMINATORS, in this same held atomic object:**
>
> 1. On armed `Exact` **and** `ReHomed`, the owned source-machine completion for
>    the claimed producer **reaches the exact claim/call funnel and selects its
>    returned `Carried` answer**; the direct producer template **does not reach
>    root emission**.
> 2. **Mutating only the final choice** to prefer the direct constructed
>    template despite a successful call **must red**, while the existing
>    worker-disposition close **stays green**. This is what separates answer
>    routing from conservation.
> 3. A row with **no exact continuation binding** still returns the ordinary
>    direct constructor — proving this is **not global suppression**.
> 4. The control **pins the planner claim coordinates** above and keeps the
>    other same-spelling construct (**39/38** or **35/34**) **non-authoritative**.
> 5. **Repeated entry of one static field origin stays permitted and
>    instance-scoped** — **no origin-level "already handled" bit** may discharge
>    a later construction.
> 6. The final post-consumer ground result **remains accepted**, and a
>    worker-bearing producer fallback at root **remains refused by the unchanged
>    terminal guard**.
>
> **Its control is a discriminator, not a green run:** it must **fail** if the
> producer intermediate reaches terminal emission before its claimed outer
> consumer, **while the final post-consumer ground result stays accepted.** A
> control that only shows the armed path green does not discharge this.
>
> **Rejected dispositions, named so none is re-proposed:** a `StaticWorkerBinding`
> arm on `RuntimeGroundValue`; a materialized carrier, slot, descriptor, closure
> value or opaque token for the worker; conversion to `Specialized`, erasure, or
> dropping it; any relaxation of `ground_value`; **declaring the stop external on
> the strength of R3 not editing the site**; and **shrinking or deferring
> selector nets 2/4, controls 2/3, or the inner-slot witness.**
>
> ### `AC-D3-SELF`'s OPEN HALF IS NOW AN OWED PLANNER OBLIGATION, and the
> ### frame's "not closed" reading was confirmed in the tree
>
> **Steward, 2026-08-12, from `evt_2z63k4vb5rk7k` at `5d322edf`.**
>
> The implementer took a **mandatory correction as its own commit**: it had
> written that the callee-to-body resolution *"is established at preflight"*, and
> **that sentence was false in the tree** — `BinderAgreement` proves only the
> marginal facts. The comment now says so, names the ruling, and records the
> relation as an **owed planner-side obligation**, with the constraint that
> lowering's independent cross-check **must not grow into a reconstruction of the
> binder relation.**
>
> **The shape of the owed guard, scoped and deliberately not built** (so the next
> turn starts from it instead of re-deriving it): `CheckedIhBinding` carries
> `frame_origin` and `recursive_position` and **no body**, and
> `SemanticIr::static_body_call_edges` returns `(caller, callee, callee_origin)`
> with **no call-site axis** — `callee_origin` is the callee function's planned
> entry node, not tied to a call occurrence. ⇒ **The resolution cannot key on the
> consuming call.** It must key on the binder's **owning function**: take
> `occurrence_authority(frame_origin).owner`, require a **unique** `StaticBody`
> edge out of it, and require that edge's callee entry to equal
> `key.invocation_callee_entry`, the redirect callee entry, and the claim's
> producer body — **refusing before claim issuance.**
>
> > ##### CLOSED at `766cbdf0` — and the route above is NOT the one that
> > ##### works. Steward, 2026-08-12, from `evt_6bg3en6yy4dgz`.
> >
> > **`AC-D3-SELF`'s open half is DISCHARGED.**
> > `FusionClaimRefusal::BinderBodyResolution` refuses **before claim
> > issuance**, in two steps: the consuming callee is **re-resolved through the
> > planner's own binding authority** rather than taken from the key, and that
> > binder is then resolved to a body required to be the one the claim redirects
> > into.
> >
> > **THE SCOPED ROUTE ABOVE WAS NEARLY A RESTATEMENT, measured.** The redirect
> > edge is **already unique by callee** and carries `caller == consumer_owner`,
> > so the by-caller lookup returns **the same edge** and **the comparison
> > cannot fail.** The working route uses the binder's **own members** — frame
> > scrutinee, then the argument at its recursive position, then that argument's
> > body — so **`recursive_position` is USED rather than compared to the key's
> > copy of itself**, which two equal numbers satisfy without naming an argument
> > that exists.
> >
> > **Discriminating evidence, not construction:** on all three `D2j` causes
> > that install a key — `Exact (10,0) -> 37`, `ReHomed (6,0) -> 33`,
> > `ProducerArity (10,0) -> 38` — each resolves to exactly its own
> > `invocation_callee_entry`; **and the same resolution on each key's PRODUCER
> > argument binding gives 34, 30 and 35**, the producer's own outgoing edge. A
> > derivation that just returned the redirect target could not do that.
> >
> > **ONE comparison is written, not three**, because `InvocationTriple` already
> > forces `redirect.callee_origin() == invocation_callee_entry` and the claim
> > is built with `producer_body: invocation_callee_entry`. Two of the three
> > would be **branches that cannot fail** — which read as checks while being
> > none.
> >
> > **THE CONTROL'S FIRST SHAPE WAS KILLED BY ITS OWN MUTATION PROOF, and the
> > reason generalizes.** Perturbing the binder's frame refused with the right
> > cause and looked correct — but tautologising step 2 left the row **green**:
> > moving the frame also desynchronised `consuming_callee`, so **step 1
> > answered first and the resolution never ran.** A two-step rule **shadows
> > itself**, and the classified cause cannot see it because both halves report
> > the same cause. The perturbation now relabels callee, binding and admitted
> > frame **together** — internally consistent and still wrong. **One mutation
> > proof per STEP, not per rule.**
>
> ~~**Its control must perturb the resolved relation while frame,
> recursive-position, result-root and redirect marginal checks stay green.**~~
> That is what makes it independent of `BinderAgreement` rather than a second
> spelling of it, and it needs a test-only planner switch on the
> `DUPLICATE_STATIC_BODY_TRIPLE` pattern. **The implementer declined to author it
> at the end of a long turn on the explicit ground that a quickly-written
> control here is the unexercised-control error the atomic rule exists to
> prevent. That is the correct call and I am not treating it as a miss.**
>
> ### THREE STOPS, ONE SHAPE — a premise about the fused function written
> ### BEFORE the suffix moved into it
>
> **Steward, 2026-08-12, recording a pattern the implementer named at
> `evt_2ffgnktmm34ta` rather than treating its third instance as a one-off.**
>
> | # | the stale premise | how it surfaced |
> |---|---|---|
> | 1 | *"the takeover cannot run before the body it sits in is defined"* | `D3` defect 1 — a fused region took over its own suffix and consumed its own claim |
> | 2 | the redirect's ordering clause | `claim_present=false` at every caller |
> | 3 | *"Not the consumer's edges — the consumer's redirected invocation is the edge that reaches this function, not one inside it"* | `call_declared_unit(StaticOriginId(37))` against `unit_calls = [34]` |
>
> **`D1`/`D2` moving the suffix inside the fused function falsifies all three,
> and each was discovered by ARMING and hitting it.** That is a serial
> discovery process: one stop, one turn, one repair, repeat. Nothing goes red
> until the armed compile reaches the site, so the count of remaining instances
> is **unknown and unbounded by anything measured so far**.
>
> **STOP 4 IS NOT A FOURTH INSTANCE OF THIS SHAPE — do not recount it as one.**
> The `StaticWorkerBinding` stop (`AC-D3-ROUTE` above) is a **routing** defect:
> the fused path terminalizes the producer intermediate before completing the
> claimed suffix. No stale written premise is involved, and the refusal site was
> correct all along. **The count behind the census offer below is still three**,
> and stop 4 neither strengthens nor weakens it.
>
> **The option, and it is an option rather than a deliverable I am adding:**
> whoever takes the next stop may census the premises written about the fused
> function before the suffix moved in, instead of waiting for the next one to
> fire. Three for three is enough to suspect a fourth. **I am not mandating it
> and not making it an AC** — it may be cheaper to keep taking the stops, and
> that judgment needs the code in front of you. **What I am fixing is that the
> third instance should not read as bad luck.**
>
> **Do not read the two measured facts on stop 3 as a repair route.** The
> consumer's own static-body edge set is **empty** — body ownership removed seat
> 37's edge, which is the hole the redirect exists to fill — so declaring
> consumer edges into the fused function supplies nothing. And reaching seat 37
> inside the fused body means the suffix is invoking a producer the dispatcher
> already lowered inline, which is **a question about what the suffix lowers
> to**, not about edge tables. The implementer measured both and declined to
> guess; that is the correct handback shape.
>
> **Routing note, mine.** The implementer named this fork and declined to settle
> it, having just been burned by a false ordering premise on the same turn
> (`D3` defect 1: a fused region taking over its own suffix and consuming its
> own claim). **That was correct.** I ruled only the routing — soundness/design
> goes to the Architect — and flagged the `evt_2f0nnwtzqy65m` tension **without
> resolving it**. The ruling came back neither "forbidden" nor "fine" but
> "the raw formulation is forbidden, here is the lawful correction," which is
> the answer a pre-emptive reading of my own would have destroyed.

**`D4` — the before-hole expression is repaired and green** under `B`-only
exclusion at the pre-retirement base. **If arming plus `D2` necessarily makes
`D4` green, it belongs in the same candidate** — do not cut another inert
increment to carry it.

> ### `D1`+`D2`+`D3` MERGE AS ONE ATOMIC CANDIDATE — 2026-08-12
>
> **Architect ruling `evt_4m0q1m4zn4k79`, on exact `33a77bd4`. `D1` may not
> merge unexercised, and a pre-arm decline is not its control.**
>
> The `D1` mechanism as built is structurally coherent — the claim supplies
> `(continuation_origin, consumer_owner)`, the selected composed case-body seam
> matches `eliminator.static_origin`, `AmbientBodyAuthority` installs the
> consumer's `Predeclared` owner and unit, both `Ok` and `Err` restore the
> producer facts, `Fusion` is never authority, and there is no signature,
> planner or ABI expansion. **But with the installer unarmed the production
> population is empty, the field stays `None`, and none of that behaviour
> fires. Green compilation of that state proves only that inert scaffolding
> compiles.**
>
> **Binding ordering.** Implement `D1`, hold it. Build `D2` on top, hold it.
> Make `D3` arming the last implementation step. Then route **one** review
> candidate spanning `D1`+`D2`+`D3` (plus `D4` if arming makes it green), whose
> controls exercise the real checked `D0` positive through
> `compile_expr_into_object_module`.
>
> **Forbidden:** routing `33a77bd4` alone to QA or merging it; any standalone
> inert `D1` or `D2` merge; a direct-builder fixture; a test-only setter for
> `fused_consumer_authority`. A pre-arm observation that the real seam is
> reached and declines is lawful **as diagnostic evidence only** and earns no
> `D1` or AC credit.
>
> **This changes merge granularity, not semantic order.** `D1` and `D2` are
> still implemented before the arm, no `main` state ever carries the arm
> without both, and the control becomes constructible only inside the armed
> range.
>
> **What was superseded is this frame's own `D3` clause** — *"`D1`/`D2` are
> landed with their controls"*. It was **infeasible as written**, because
> `D1`'s real control needs `D3`'s non-empty population. Do not reconstruct it
> from a memory of this frame.
>
> **Section 9's sizing target still applies per increment.** The atomic unit is
> the **review candidate**, not the turn: `D1`, `D2` and `D3` remain separate
> commits with separate handbacks to the leader. This adds no review hop and no
> new party.

> ### `DP-0` — THE RETIREMENT INSTRUCTION SCHEDULES A LIVE PROHIBITION FOR DELETION
>
> **Added 2026-08-12 from Adversary `evt_h0mzz2y4666b`. All three findings
> verified at the landed source before framing, not accepted on report.**
> Comment-only, one file, `lowering/core.rs`. Released ahead of `DP`, which is
> stopped on an open Architect relation ruling. **Do not fold this into `DP`,
> and do not touch `DP`'s population.**
>
> **This repairs my own ask.** The retirement clause exists because I required
> one on the previous partial: a tree-state claim must name its falsifier and
> its owner, since nothing reddens when it expires. That was right. **The
> scoping words I accepted were not.**
>
> **FINDING 1 — the deletion region is over-broad, and the excess is the safety
> content.** The clause says `D1`/`D2` landing makes *"every sentence above
> false"* and that this node owns *"deleting this block"*. `D1`/`D2` falsify
> **the measurement** — the guard stops being unreached. They falsify neither
> of these, both of which sit above the clause and therefore inside the region
> marked for deletion:
>
> - **`core.rs:2234`** — *"Architect `evt_1q7v9fcw5hd87`; the answer is the
>   node's `DP`"*
> - **`core.rs:2238`** — *"A fusion-only admission of the guard, and copying or
>   inferring the consumer's identity onto the producer, are both ruled
>   **unlawful** rather than merely out of scope."*
>
> **Measured at `8b142d01`: `git grep evt_1q7v9fcw5hd87` over `crates/` returns
> exactly one hit, line 2234, and the `unlawful` prohibition appears once, at
> 2238. This block is the only record of both *in the source tree*.**
>
> > **CORRECTION 2026-08-12, and the false claim was mine.** This block first
> > read *"the only **in-tree** record of both"*. **That is false**, and
> > Runtime QA blocked `49072fb8` on the same sentence after it was carried
> > into `core.rs`. Measured at `db265561`, `evt_1q7v9fcw5hd87` appears in
> > **five files, eight occurrences**: `core.rs`, this frame (4), the node,
> > the `D2k` frame, and the briefing. **The node at
> > `docs/program/issues/RT-LEXICAL-R3-FUSION-EMITTER.md:15-50` records the
> > same ruling and both unlawful shortcuts.**
> >
> > **The defect is the same shape as Finding 1 itself: a correct measurement
> > with an over-broad conclusion bolted on.** The grep was scoped `-- crates/`
> > and said so; the conclusion silently widened `crates/` to *"in-tree"*. I
> > had already found four of those durable citations an hour earlier doing
> > the `M3` check on `112c07f5` and cited them in `evt_3ad99t706j226` — **the
> > disconfirming evidence was already mine when I wrote the claim.**
> >
> > **What survives, and it is why `DP-0` is still right.** Retiring the
> > prohibition from `core.rs` still removes it from **the source tree**,
> > where the successor who meets the mixed-frame refusal is reading. Losing
> > the in-code copy is a real loss even though `docs/` retains it. **The
> > severity is lower than I framed it; the repair is unchanged.** State it
> > non-exclusively — the fence is a provenance boundary, and a false
> > uniqueness claim misdirects the next successor's source of authority.
>
> ⇒ **The prohibition is scheduled for deletion at precisely the moment it
> becomes relevant.** `D1`/`D2` landing is what makes the guard reachable,
> which is when a successor meets the mixed-frame refusal and reaches for one
> of the two shortcuts — and the sentence saying those shortcuts are *ruled
> unlawful rather than merely out of scope* goes out in the same candidate, on
> the instruction of the block it was written into. **Fail-open, and what
> survives is the guard while what is lost is the record that two ways around
> it were considered and refused.**
>
> **The cut to make is one the block already draws elsewhere**, between *"where
> it stops now"* and *"ANSWERED, not open"*: **a measurement of a tree state
> expires; a ruling about what is lawful does not.** Retire the first, keep the
> second. If you instead keep a single region, it must name its boundary
> explicitly — *"every sentence above"* and *"this block"* are the two phrases
> doing the scoping today and **neither names one**.
>
> **FINDING 2 — a labelling convention applied to one of two neighbouring
> claims.** Step 6 carries `MEASURED on the D0 gate's own compiles at
> `21307d7f`` plus *"do not read a green suite as evidence either way"*. Step 5
> carries nothing, is present tense, and reads as mechanism — but was written
> from the held commit message and the node rather than from a compile. With
> nothing labelled a reader is uniformly cautious; **with `MEASURED` four lines
> away, the unlabelled neighbour reads as *this one did not need saying*.**
> Establishing the convention raised the cost of the one omission. The existing
> qualifier does not cover it: it opens a paragraph whose heading, body and
> entire measured content are step 6. **One clause on the step-5 sentence
> itself — held off `main`, not re-derived here — closes it.**
>
> **FINDING 3 — a stale population inside the one sentence this candidate
> rewrote.** *"Arming it makes the `Exact` and `ReHomed` roots refuse at step
> 5, which reds `d2f_0`."* **`d2f_0` has had three positive roots since
> `7a018ef6`, three merges ago** — `control.rs:2859-2861` reads *"the three
> positives, each on its own root"*, and `ProducerArity` sits in the same
> assertion tuple. The candidate **edited this sentence** and carried the
> two-root population forward unexamined.
>
> **Answer it by measurement, both branches informative:** does arming make
> `ProducerArity`'s root refuse at step 5? **Yes** ⇒ the warning under-counts
> the reds and the fix is one word. **No** ⇒ the third positive behaves
> differently at step 5 from its two tuple-mates, which is a more interesting
> fact than the sentence and belongs in it. **The sentence as written is the
> only one of the three answers not supported.** Low severity — it errs toward
> under-promising a red — but the same population is now cited from two files.
>
> **Refuted by the same pass, do not re-raise:** the `MEASURED` block's own
> currency **holds**. Every changed line in the merged range is a `//` line, so
> nothing executable moved under the pin between `21307d7f` and `8b142d01`.
> Pinning a measurement to a SHA and then moving no code under it is the right
> shape and it survived the check.
>
> **Excluded:** `DP`'s population, the relation ruling, mechanism, planner,
> ABI, arming, AC/node credit, and the held `D1`/`D2`/`D3` range.

> ### `DP` IS RULED (a) — Architect `evt_w4nvsmrs1qhk`. **(a)-vs-(b) STANDS; its
> ### POPULATION AND ITS NETS DO NOT — see the two withdrawal boxes inside.**
>
> **Added 2026-08-12. This supersedes `DP`'s open relation question; do not
> re-derive the fork.** (b) is **rejected for this source**, and **no
> `AC-10`-shaped source cut is owed** — the current checked source is
> sufficient to decide the relation.
>
> **The decisive fact is the checked invocation census, not segment-site
> equality.** The source has two distinct checked frame occurrences but exactly
> **one** checked computational-IH invocation occurrence, `D2G_CALL`, rooted at
> `D2G_OUTER_SLOT`. The producer carries `D2G_INNER_SLOT`, which is a checked
> IH **binder template** — not a call marker and not an invocation source, so a
> second dynamic invocation cannot be minted from it. **(b) would invent a
> second checked call occurrence, invocation template and dynamic splice edge
> that the source does not contain.**
>
> > # ITEMS 1 AND 2 ARE WITHDRAWN BY THE AUTHOR — Architect `evt_2f0nnwtzqy65m`,
> > # 2026-08-12. `89ee005b` IS THE FALSIFIER.
> >
> > **`D2G_OUTER_SLOT.frame_templates` must NOT be expanded**, and in particular
> > must not be populated by the transitive `ParentFrame` closure. The base
> > uncomposed plan stays **exact**: `outer slot`/`call = [outer frame]`, `inner
> > slot = [inner frame]`.
> >
> > **The unarmed `ReHomed` refusal is CORRECT.** `instantiate_checked_invocation
> > _segment` is not too strict and **must not** be weakened, taught a fusion
> > exception, or changed to accept subset coverage.
> >
> > **Why the withdrawn items were wrong.** `ParentFrame(inner, outer)` proves
> > static checked control nesting. It does **not** prove that the outer IH
> > call's *current dynamic carrier* contains an inner recursor layer — the
> > checked source puts `D2G_INNER_FRAME`/`D2G_INNER_SLOT` inside the lexical
> > closure in the outer match's **scrutinee**, while `D2G_OUTER_SLOT`/`D2G_CALL`
> > are in the outer selected **case body**. `callee_frame_templates` is the
> > exact sequence carried by *that* checked invocation occurrence, and the
> > instantiator enforces equality against the frames actually instantiated.
> >
> > ## THE CORRECTED MEMBERSHIP LAW — this is what items 1 and 2 become
> >
> > > **Checked occurrence identity is planner-authored; membership in a concrete
> > > invocation segment is established at the checked event that actually adds
> > > that semantic layer.**
> >
> > For the base IH marker the event carries **one** outer layer, so membership
> > is one frame. When checked fusion composition later splices the producer
> > semantic layer into that same invocation-local segment, **the composition
> > relation transports the producer's independently authored frame identity and
> > qualifies that actual layer with the already-minted invocation source/
> > instance.** Only then do `(instance, outer_frame)` and `(instance,
> > inner_frame)` coexist.
> >
> > **Runtime may VALIDATE that relation. It may not DISCOVER it** from shape,
> > from `ParentFrame`, from segment-site equality, or from a fusion label.
> >
> > ⇒ **What changed is WHEN the producer frame becomes a member, not whether.**
> > The base call template does not promise a layer that is absent; the checked
> > composition plan promises and transports the added member **at the splice
> > that creates it**.
> >
> > **The (a)-versus-(b) result is PRESERVED and is not reopened:** still one
> > checked call marker and one invocation instance, **no fabricated second
> > source**. Items 3 and 4 below stand unchanged.
>
> ~~**The authorized population, exactly:**~~
>
> 1. ~~Expand `D2G_OUTER_SLOT.frame_templates` from the singleton outer frame to
>    the complete checked invocation-local sequence containing **both** frame
>    IDs, ordered by their authoritative `semantic_position`.~~ **WITHDRAWN.**
> 2. ~~Let `D2G_CALL.callee_frame_templates` **inherit** that sequence from the
>    slot, as production already does via
>    `callee_frame_templates = slot.frame_templates.clone()`. **Do not patch the
>    call vector independently.**~~ **WITHDRAWN.**
> 3. Keep `frame_template_id = D2G_OUTER_FRAME` as the exact slot/binder frame,
>    and keep the call's existing `parent_frame_template_id` and
>    `parent_segment_site_id` as the dynamic call-to-open-scope edge.
> 4. Keep the producer as its own frame — own frame ID, marker, occurrence path,
>    semantic position, interfaces, fingerprint, and `ParentFrame(D2G_OUTER_FRAME)`
>    witness. **One** call marker mints **one** fresh affine invocation instance,
>    and `instantiate_checked_invocation_segment` transports that single
>    source/instance onto both expected frame IDs. Dynamic identity stays the two
>    distinct pairs `(instance, outer_frame)` and `(instance, inner_frame)`.
>    **Nothing is aliased.**
>
> **This does NOT read `ParentFrame` as an invocation identity.** The checked
> frame relation supplies the static rooted segment and its endpoint/order
> facts; the sole checked call occurrence supplies the dynamic invocation
> source; **the planner joins those two checked facts** by authoring the slot's
> reusable callee sequence. **Lowering and fusion still infer nothing** — they
> may only validate and instantiate the sequence the plan transported. The
> original prohibition is intact.
>
> **The adjacent `erasure.rs:1469-1476` prohibition is NOT triggered**, and the
> implementer's reported distinction holds: it forbids manufacturing callee
> membership from the caller/enclosing-parent edge, and the `DP` producer frame
> is not an enclosing caller endpoint but the distinct checked child frame
> already inside the outer slot's rooted segment. **Do not generalize this** to
> *"every frame with the same `segment_site_id` belongs to every slot"* — the
> population is the exact source-derived sequence rooted at this slot, and
> `D2G_INNER_SLOT` does **not** acquire an outer-frame invocation by site
> coincidence.
>
> **This is a represented mechanism, not a new Runtime shape.**
> `callee_frame_templates` and slot `frame_templates` are already vectors, both
> invocation kinds pass through the same exact-frame instantiator, and the
> existing two-frame RTFP control proves one affine invocation can lawfully
> instantiate multiple ordered frame templates. **`DP` repairs planner
> population, not the Runtime identity model.**
>
> > # THE FIVE NETS BELOW ARE SUPERSEDED — they were premised on the withdrawn
> > # population. USE THIS LIST. Architect `evt_2f0nnwtzqy65m`.
> >
> > **These controls must be load-bearing in the recut:**
> >
> > 1. The unarmed, **uncomposed** `Exact` / `ReHomed` / `ProducerArity`
> >    baselines retain their **pre-WIP** behaviour. In particular **`ReHomed`
> >    must not acquire an expected absent inner frame** — that regression is
> >    exactly what `89ee005b` demonstrated.
> > 2. On the **actual armed composed path**, the producer layer carries its own
> >    exact checked frame ID **before** the mixed-frame guard, while both frame
> >    keys carry the **same** checked invocation source/instance.
> > 3. **Deleting the composition-time producer membership** restores the current
> >    mixed checked/inferred refusal.
> > 4. Each of these stays **red**: copying the consumer frame ID; deriving
> >    membership from Runtime shape or from `ParentFrame`; inventing a second
> >    invocation source; accepting subset coverage; widening the inner slot.
> > 5. **`89ee005b` remains preserved WIP negative evidence** that unconditional
> >    rooted-closure population is unlawful. It is **not a candidate**, earns no
> >    credit, and takes no QA route.
> >
> > **Note what moved in control 1.** The superseded net 2 asked that deleting
> > the producer frame from the *slot/call sequence* restore a refusal. The new
> > control 3 asks the same of the *composition-time membership*. The old form
> > would now pass against a population that must never exist.
>
> ~~**Required nets — these are the controls, not a wish list:**~~
>
> - ~~the positive plan carries **both** frame IDs in semantic-position order, the
>   call sequence **equals** the slot sequence, both dynamic frame keys carry the
>   **same** invocation source/instance, and the full endpoint chain composes;~~
> - ~~**deleting** the producer frame from the slot/call sequence restores the
>   current missing-coverage/mixed-frame refusal;~~
> - ~~**permuting** the transported frame occurrences is refused by the
>   planned-order check;~~
> - ~~a purported **second source** without its own checked invocation marker and
>   checked dynamic parent edge is refused;~~
> - ~~the **inner slot stays independently exact** rather than widened by site
>   coincidence.~~
>
> **SCOPE AND CONTENTION — read before assigning.** This authorizes the
> **elaborator-side slot population**, so it reaches
> `crates/ken-elaborator/src/erasure.rs`, which is **outside this frame's
> section 9 contention set and on another crate's review surface**. That is the
> reason the Architect routed the re-cut to the Steward rather than back to the
> ring. **Sizing and the contention re-check are owed by the Steward before this
> is released**; it is not startable on the strength of this block alone.
>
> **Still excluded:** arming, `AC-8`, `D4`, and the held `D1`/`D2`/`D3` range,
> which remain one atomic candidate **after** `DP`. This ruling grants no
> lowering/fusion inference, arming, or `D1`/`D2`/`D3` credit.

> ### THE RECUT: `DP` FOLDS INTO THE ATOMIC HELD CANDIDATE — Steward, 2026-08-12
>
> **This supersedes the two-cut block below in full.** Read this; the block
> below is kept only for the withdrawn argument and its falsifier.
>
> **Architect `evt_2f0nnwtzqy65m` settles the repair class and the atomic
> boundary.** `DP`'s positive **is** the live composed segment, so there is no
> population change that can be validated on its own.
>
> ⇒ **`DP` IS NOT SEPARABLE. The review object is `DP` + `D1` + `D2` + `D3`**,
> one atomic candidate — with `D4` only under the already-recorded
> necessary-green rule.
>
> **Three things this forbids, each of which was previously in play:**
>
> - **No standalone `DP` population merge.** The thing I released this morning
>   as `DP-1` cannot exist in any slicing.
> - **No standalone held `D1`/`D2` merge.** That direction is closed too.
> - **No QA route on `89ee005b`.** It is preserved negative evidence, not a
>   candidate, and earns no credit.
>
> **The cumulative planner/transport stop governs the running total.** This is
> the same stop that fired at `D2` and produced the `DP`-first recut in the
> first place; it now governs a single larger object rather than a chain of
> partials, and the running total keeps accumulating across it.
>
> **What this ruling does NOT grant:** arming, `AC-8`, `D4`, node or AC credit,
> and approval of any held commit. It settles class and boundary only.
>
> **Sizing consequence, and I am not pretending otherwise.** This is materially
> larger than anything released on this node so far, and the one-hour turn
> target cannot be met by it — expect a sequence of hard stops inside one
> atomic object rather than a sequence of merges. **That is the correct shape
> here and not a sizing failure**: the alternative is a partial that cannot
> carry its own positive control, which is precisely what was just measured.
> Hand back at each stop; do not push through to keep the object whole.
>
> **Contention: the measurement below still holds** — `erasure.rs` is
> uncontended and `KERNEL-NESTED-IND` remains structurally excluded for the
> duration. The **surface finding also still holds and matters more now**:
> section 9 is a floor, and the atomic object spans strictly more than `DP`
> did.

> ### `DP` WAS SIZED AND RELEASED AS TWO CUTS — Steward, 2026-08-12. SUPERSEDED.
>
> **This discharges the sizing and contention re-check the block above says are
> owed, and it is what releases `DP`.** Measured against `main`
> `e48c2f90`. Do not re-derive the fork above; this block decides only the cut,
> the surface, and the order.
>
> #### The contention re-check, and the block above understates the surface
>
> **The Architect named the elaborator. It is not the only path outside section
> 9.** Measured by enumerating every site that constructs or compares a slot
> `frame_templates` or a `callee_frame_templates` — 24 sites, six files:
>
> | file | in section 9? | why `DP` reaches it |
> |---|---|---|
> | `ken-elaborator/src/erasure.rs` | **no** — other crate | the producer; `:1453` `frame_templates: vec![frame_id]` is the singleton this replaces |
> | `ken-runtime/src/oriented_subcontinuation_plan.rs` | **no** | both binding fingerprints consume the sequence, and the `call == slot` equality check lives here |
> | `ken-runtime/src/cranelift_backend/lowering/mod.rs` | **no** | the transport-side sequence comparison |
> | `ken-runtime/src/cranelift_backend/planning/static_transition.rs` | **no** | the entire `D2G_*` fixture, 49 references, all in this one file |
> | `ken-runtime/src/cranelift_backend/lowering/core/tests/control.rs` | **yes**, under `core*` | where the five nets land |
> | `ken-runtime/src/cranelift_backend/test_objects.rs` | **no** | one constructed slot |
>
> ⇒ **Section 9 is stated as `lowering/core*`, `units.rs`, and the eliminator
> case-body path, and four of those six files are outside it.** Read section 9
> as a floor for this node, not as its surface. Re-derive the intersection at
> candidate time as section 9 already instructs.
>
> **Live contention is empty, measured rather than asserted.** No worktree in
> the repo holds an uncommitted edit to `ken-elaborator/src/erasure.rs`. Two
> `.claude/worktrees/agent-*` scratch trees hold uncommitted elaborator edits —
> `ast.rs`, `elab.rs`, `lexer.rs`, `parser.rs`, `resolve.rs`, `extract.rs`,
> `lib.rs`, and an untracked `numbers.rs` — and **none of them is `erasure.rs`**.
> They are abandoned subagent trees, not seat trees.
>
> **The one latent collision, named so it is not rediscovered.**
> [[KERNEL-NESTED-IND]] `D5` claims `ken-elaborator/src/erasure.rs` explicitly,
> and its lane surface is defined as *"every path an `AC-K12` stage traverses,
> minus `crates/ken-runtime`"* — which contains the elaborator by construction.
> **Kernel cannot collide today:** it is idle and its node is blocked behind
> [[RT-DYNAMIC-ARM-SCALAR-MERGE]] (`ready`, unstarted) and
> [[RT-NESTED-IH-NATIVE-REALIZATION]] (`draft`). Both are Runtime nodes, and
> **Runtime runs one node at a time**, so neither can start while `DP` holds the
> lane. The collision is therefore structurally excluded for `DP`'s duration
> rather than merely absent right now. If `DP` outlives that, it is my problem,
> not the ring's.
>
> #### Sizing: the mechanism is small, the nets are the work
>
> The population edit is **one site**. `erasure.rs:1453` is a literal
> `vec![frame_id]`, and the derivation it becomes — the frames rooted at the
> slot frame by `ParentFrame` witness, ordered by `semantic_position` — is a
> mirror of a traversal `oriented_subcontinuation_plan.rs` **already performs
> and already validates** (the `by_id` / parent-chain-to-`DistinguishedRoot`
> walk, and the existing `frames.sort_by_key(semantic_position)`). Both fields
> are already on the frame. **This is not new machinery.**
>
> The ripple is mechanical and bounded: `slot.frame_templates` is a fingerprint
> input, so both the slot and the call binding fingerprints move, and the
> `call.callee_frame_templates != slot.frame_templates` checks follow the clone
> automatically.
>
> **The five required nets are the bulk of the work, not the mechanism.** That
> is what puts `DP` past the one-hour turn and why it is cut.
>
> #### The cut — WITHDRAWN 2026-08-12, see the box below before using any of it
>
> **`DP-1` — the population, plus nets 1 and 2.** The derivation at `:1453`,
> the positive net (both frame IDs in semantic-position order, call sequence
> equals slot sequence, both dynamic frame keys carrying the same invocation
> source/instance, full endpoint chain composing), and the deletion net
> (removing the producer frame from the sequence restores the current
> missing-coverage/mixed-frame refusal).
>
> **`DP-2` — nets 3, 4, and 5.** Permutation refused by the planned-order
> check; a purported second source without its own checked invocation marker
> and checked dynamic parent edge refused; the inner slot staying independently
> exact rather than widened by site coincidence.
>
> **Nets 1 and 2 ship together or not at all.** Net 1 alone is a population
> that is present and proves nothing — the exact shape `AC-2` exists to catch,
> one level down. A positive with no deletion control passes for any reason,
> including the mechanism never being reached.
>
> > # THE SPLIT IS WITHDRAWN. ITS PREMISE WAS MEASURED FALSE AT `89ee005b`.
> >
> > **Steward, 2026-08-12, on runtime-leader's hard stop `evt_3ccqdyp9tkk33`.**
> > Everything in the two paragraphs below this box is **struck**. It is left
> > standing rather than deleted because the argument it makes is the one a
> > successor would otherwise rebuild from scratch, and its falsifier is worth
> > more than its absence.
> >
> > **What I claimed:** `D2F_EMITTER_ARMED` is `false`, therefore `DP-1` lands
> > an inert population and nothing executes the transported sequence between
> > the cuts.
> >
> > **What was measured:** with the widened outer-slot/call sequence, **unarmed
> > `ReHomed` reaches `instantiate_checked_invocation_segment` and refuses** —
> > expected frames `{0,1}`, instantiated `{0}`. The arming flag gates the
> > **fusion emitter**; it does not gate every consumer of the transported
> > sequence. **`DP-1` alone is a regression, not an inert change.**
> >
> > **The error shape, and it is one I hold a lesson on:** I read a single named
> > gate as bounding the whole population that reaches a mechanism. `D2F_EMITTER_
> > ARMED: false` was evidence about *one* path and I used it as evidence about
> > *all* paths. The census I never ran is *which unarmed consumers reach the
> > instantiator* — and the answer is at least one.
> >
> > **The second failure is larger and is NOT about the split.** Net 1 requires
> > the producer's second **dynamic** layer, which arises only under the held
> > `D1`/`D2`; net 2 therefore fires on the proposed positive. ⇒ **`DP` may not
> > be able to carry its own positive control at all**, which is a question
> > about `DP`'s separability from `D1`/`D2`, not about how `DP` is cut. It is
> > with the Architect.
> >
> > **No re-cut until the Architect rules** the static-versus-dynamic membership
> > question, because the answer determines whether the authorized population is
> > right — and a cut authored against a population that may move is waste.
> > `89ee005b` is **WIP evidence, not a candidate**; do not route it.

> ~~**Why splitting is safe here, and it is a real argument rather than a
> convenience.** `D2F_EMITTER_ARMED` is `false` at `core.rs:2304` and the guard
> is measured unreached, so `DP-1` lands an **inert** widened population:
> nothing in production executes the transported sequence between the two cuts.
> What `DP-2` adds is proof that the population is *exact* rather than merely
> *sufficient*, and net 5 is specifically the guard against the site-coincidence
> over-generalization the ruling above forbids.~~
>
> ⇒ ~~**`DP-2` is released at the same moment as `DP-1` and takes the lane
> directly after it.** No other node comes between them. If `DP-1` hands back
> and `DP-2` does not follow, that is a stall to report to me, not a stopping
> point.~~
>
> **A `DP-1` hard stop inside the hour is a good outcome.** Say so and hand
> back. The sizing target is not an acceptance criterion and no AC is derived
> from it.
>
> **Unchanged by this block:** the authorized population and the five nets are
> the Architect's and are quoted, not reinterpreted; arming, `AC-8`, `D4`, and
> the held `D1`/`D2`/`D3` range stay excluded; neither cut credits an AC or
> closes the node.

> ### ADVERSARY TRIAGE on `e48c2f90` — Steward, 2026-08-12 (`evt_2933sm5wnh2je`)
>
> Two confirmed defects, folded into `DP-1` rather than given a node. One
> proposal, taken, and it lands on `DP-2`. **I reproduced both defects at the
> shipping tree before folding them; neither is taken on report.**
>
> #### `DP-1` also carries two comment repairs in `core.rs`, both comment-only
>
> **1. The census is falsified by the sentence that states it.** `core.rs:2252`
> pins *"MEASURED at `49072fb8`: `evt_1q7v9fcw5hd87` appears once under
> `crates/`"* — and **spells the id it is counting**. Measured: `1` at
> `49072fb8`, `2` at the shipping tree, where the second occurrence *is that
> sentence*. The pin was true when taken and was falsified by its own
> publication. A successor re-running the stated probe gets `2` and cannot
> distinguish real drift from self-reference, which is the exact ambiguity the
> pin exists to remove. **Second defect in the same sentence:** the trailing
> *"— this line —"* resolves to `:2252`, while the record it means is at
> `:2242`.
>
> **Repair, and prefer this form over re-pinning.** Refer to the id
> positionally instead of spelling it — *"the Architect id named above appears
> exactly once under `crates/`, at the head of this fence, and in four files
> under `docs/program/`."* That fixes the deictic and makes the census
> **self-stabilizing**: the sentence stops counting itself, so the number stays
> correct without a re-pin at every future edit. Re-pinning to the shipping tree
> and saying *twice* is also correct but needs maintaining forever.
>
> **2. The step-5 block states an instrument that cannot produce its result.**
> It discloses *"the panic surfaces only the first"* and then reports where all
> three roots land. Honest, and the gap is visible — but a reader re-running it
> hits the same wall. One clause naming how the three were separated (three
> runs, per-root isolation) makes it reproducible. Low severity.
>
> #### `AC-DP` — the two unlawful shapes get a control, on `DP-2`
>
> **The prohibition at `core.rs:2242` has never been falsifiable.** Both shapes
> it rules unlawful are code someone must author — a fusion-only admission in
> the mixed-frame validator, and a producer identity obtained by copying or
> inferring rather than transporting — and **a control can red on exactly
> those**. Until now it could not be written, because there was no producer
> transported identity to compare against. `DP-1` lands one.
>
> ⇒ **`DP-2` carries a control that reds if the producer's checked identity is
> ever obtained by copy from the consumer's rather than transported, and reds
> if the mixed-frame guard admits under a fusion-only condition.** Grounded in
> Architect `evt_1q7v9fcw5hd87`, which ruled both shapes unlawful; this adds no
> ruling and reinterprets none.
>
> **This is why there is no follow-on node for the comment fence.** The only
> mechanism that could protect comment text is a source-text oracle, which is
> in the standing operator-disfavoured class. And after `DP-0` the fence's cost
> of being crossed is **proximity, not loss** — the ruling is carried in full by
> the node at `:39-51` and the frame at `:566-568`, both independently verified
> by the Adversary this pass. Put the mechanism where the ruling can fail, not
> around the prose. **Keep the fence anyway**; it costs nothing to maintain.
>
> **STOP CONDITION, and do not engineer around it.** If this control cannot be
> written without inferring or comparing identity in a way the ruling itself
> forbids, that is a **hard stop back to me** — not a weakened control and not a
> workaround. An unfalsifiable prohibition is better than a control that
> launders the prohibited operation into the test suite.
>
> #### Ledger correction, and it is the Adversary's to hold
>
> The false uniqueness claim **originated in the Adversary's finding**
> (`evt_h0mzz2y4666b`) and was quoted verbatim into this frame. I wrote *"that
> claim was mine"* in the `DP-0` handoff; the Adversary has put it back on its
> own ledger, correctly, on the grounds that a scoped probe carrying an unscoped
> conclusion is a defect it must keep. **What is mine is separate and stands:**
> I adopted it without re-measuring, and my own `M3` check an hour earlier had
> already surfaced four of the durable citations that disprove it. Both entries
> are real and neither cancels the other.

## 6. Acceptance criteria

**AC-1 — the positive is a real full-pipeline compile.** The checked `D2j`
`R3`-shaped twin compiles through `compile_expr_into_object_module` with
`Some(oriented)` and resolves **plane 1** with exactly one key/ID/descriptor.
*Control:* the committed row, with its own freshly derived coordinates. **Do
not cite origin-23 or any other old `px8j` coordinate** — those were struck.

**AC-2 — suppressing the fusion restores the measured refusal.** *Control:* a
committed mutation that disables the fusion for this expression and reproduces
the exact refusal, with evidence the detector was reached. **This is the AC the
node exists for.** A fusion that is present but doing nothing passes every
criterion except this one.

**AC-3 — the five parent guards are intact**, unchanged from `#6d`: 
`RecursiveBackedge` stays protocol-only; a closure is never made
boundary-transferable; an actual non-constructor computational scrutinee still
refuses; source-join closeout still rejects an un-emitted/unselected join; a
missing recursive-IH authority still refuses. *Control:* a committed negative
witness per guard, **each with a positive control proving its path is reached.**

**AC-4 — `px8j` still refuses ordinarily**, as the absence comparator, and its
refusal is asserted as such rather than as an incidental pass.

**AC-5 — no banned mechanism.** No fallback to `RecursiveDescent`, no
`BoundaryUse`, no `PlannedEffectSeat` widening, no lowering-minted token, no
invocation-local activation/resume/return-hole state in ABI data. *Control:*
name the ABI payload at each new crossing and show ordinary typed fields.

**AC-6 — zero new `#[ignore]`**, and no tracker `status:` change in the
candidate. *Control:* `git diff`.

**AC-7 — CI green** on the merge. Not a local `--workspace` run
(`COORDINATION §12`).

**AC-8 — the live authority control, on the atomic candidate.** Architect
`evt_4m0q1m4zn4k79`; this is the AC that the atomic ordering exists to make
constructible, and **it must run from the actual armed production compile**,
not from a fixture.

- **One installed fusion definition and one region-keyed switch** are asserted
  from that compile.
- **The live authority fields are observed in three positions:** producer
  `Predeclared` **before** the consumer phase, consumer `Predeclared` **during**
  the exact selected case body, producer **restored after** it. **No `Fusion`
  authority at any of the three.**
- **The error control forces an error *after* the switch, through that same
  production path**, and observes producer restoration **before the error
  propagates.** An error injected anywhere else does not discharge this.
- **`D0`'s plane and refusal rows are retained**, along with this frame's guard
  controls. `AC-8` is added to them, not substituted for them.

> ### ROUTED IN FROM `D2k` — 2026-08-12, Steward split under `evt_290zp8kxn9jbs`
>
> The Architect rebound `D2k`'s `AC-1` and delegated the resulting split. **Two
> obligations land here**, in the frame rather than as a cross-post, because
> this node is where the mechanism that can discharge them lives. **This node
> was carved out of `RT-LEXICAL-RECURSOR-CONSUMERS` for exactly this reason** —
> its own title says leaving the expression in the parent while moving the
> machinery *"would give the parent an AC it cannot discharge."* That is what
> happened anyway, and this is the correction.
>
> **`AC-9` — the semantic effect, which `D2k` cannot prove.** `D2k`'s rebound
> criterion asks a checked positive to exhibit **one recognition-to-rebind
> transition, one exact consumer, and one suffix execution**. All three are
> properties of an **installed** fusion, and installation is gated at
> `lowering/core.rs:2231-2235` behind `D2F_EMITTER_ARMED: false` — which this
> node owns and `D2k` may not touch. `AC-2` already carries the fourth
> obligation (the restored refusal on suppression); these three join it.
> *Control:* asserted from the armed production compile alongside `AC-8`, not
> from a fixture, and each transition named rather than inferred from a
> non-zero count.
>
> **`AC-10` — route coverage on BOTH hole placements, or the route stays open.**
> The ruling requires acceptance to exercise every structural route the campaign
> claims, **before-hole and after-hole**, and forbids citing a checked twin for
> a route it does not execute.
>
> **Measured at `dec2e0c7`, and it is why this cannot be discharged by
> citation:** `D2jCause` has eight variants — `Exact`, `Frame`, `SelectedSlot`,
> `Invocation`, `ExactSuffix`, `CallIdentity`, `ReHomed`, `ProducerArity` — and
> **none of them is a hole placement**. `d2j_entry_under` builds a
> `DeclarationRef` to `D2J_DECLARATION` applied to two `MkUnit` arguments, and
> the whole `D2j` fixture region contains no `Px8jSelectedScopePlacement` token.
> The hole axis exists **only** on the `px8j` **seed** family in
> `lowering/core/tests/control.rs` — the unmarked lane that resolves zero.
>
> ⇒ **Neither hole route has a checked positive today — not one missing, both.**
> `AC-1`'s `D2j` twin is qualified to the route it actually executes and stands
> in for neither. Authoring lawful checked positives for the two placements is
> **new fixture population and is in scope for this node**; a route with no
> lawful positive **remains open** rather than passing by citation. *Control:*
> for each placement, the positive resolves its own key and the suppression
> control reproduces its own refusal — a shared control across placements
> discharges one of them at most.
>
> **Not blocking, flagged to the Architect:** that his requirement 1 presupposes
> an armed emitter is the Steward's reading. If he intends the semantic effect
> to be provable without installation, `AC-9` returns to `D2k` and this node
> keeps `AC-10` alone.

## 7. Excluded scope

- **Retirement of the residual class and any lane deletion.** That is
  [[RT-RECURSOR-TRANSPORT]] and [[RT-DESCENT-RETIRE]].
- **The other seven expressions**, including row 5's after-hole member.
- **`D2h`'s key re-derivation.** Soundness-bearing and not reopened.
- **Unwinding any of the eleven landed `D2f` partials.** They are the
  substrate; they are inert and correctly labelled.
- **A direct-builder fixture, and a test-only setter for
  `fused_consumer_authority`** (Architect `evt_4m0q1m4zn4k79`). The first is
  the exact defect `D0` exists to correct; the second manufactures the state
  whose real arrival is the thing under test. **If `AC-8` seems to need
  either, the candidate is not yet armed** — that is a sequencing answer, not a
  fixture problem.
- **A standalone inert `D1` or `D2` merge.** See the atomic-candidate ruling in
  section 5.
- **A fusion-only admission in the mixed-frame validator**, and **copying or
  inferring the consumer's frame or invocation identity onto the producer**
  (Architect `evt_1q7v9fcw5hd87`). Both are ruled **unlawful**, not merely out
  of scope, and the reasons are in the node. The short forms worth carrying:
  segment checkedness is **segment-wide**, so no existing proof boundary can
  confine an exception to the consumer suffix; and this boundary accepts
  **transported** identities only, so deriving one from body shape, origin
  coincidence, the sole remaining plan row, or the fusion claim is inference.
  **Calling the case "fusion" does not supply the missing authority.**

## 8. Stop conditions — return to me, do not decide

- **`D0`'s positive row will not go non-zero** through the lawful route. That
  is this frame's central premise failing, and it is the same wall that stopped
  the work before — **do not repair it by supplying the plan.**
- **The interior seam turns out not to be where section 4 says**, or the switch
  cannot be expressed there without a signature change rippling beyond the
  eliminator.
- **A guard in `AC-3` cannot be preserved** under the fused lowering.
- **The repair needs a new planner/ABI population beyond what is landed.**
  This stop is written against the *cumulative* state, not against a single
  increment — that is how the parent's identical stop failed to fire across
  eleven partials, each of which was individually small enough not to trigger
  it. **Ask it at every cut, about the running total.**

  > **THIS STOP HAS FIRED — 2026-08-12, at `D2`.** It is answered, not armed.
  > `D0` needed no new population and `D1` needed none; `D2`'s adoption exposed
  > a producer-identity gap that requires it, and `DP` in section 5 is the
  > response. **Do not re-read this bullet as an open question**, and do not
  > treat `DP` as having already discharged it for later cuts — the running
  > total keeps accumulating, and `DP` is the largest single addition to it.
  >
  > **It fired the way the frame said it would.** Each increment was
  > individually clean; only the total was not. The instrument that caught it
  > was the implementer stating the **running total** in the handback rather
  > than the increment's own delta.

## 9. Contention and sizing

`crates/ken-runtime/src/cranelift_backend/lowering/core*`, `.../units.rs`, and
the eliminator's case-body lowering path.

**Runtime runs one node at a time.** [[RT-LEXICAL-ROW2-MISSING-MINT]] held this
file set and **merged 2026-08-12** at `main` `741f66c3`. Re-derive the
intersection at candidate time anyway: a merge-base goes stale without your
branch moving, and row 2's arc landed nine PRs into `core/tests/control.rs`.

> **THIS NODE NO LONGER HOLDS THE RING — 2026-08-12.** It is stopped, and
> `#6d` `D2k` took the file set at `evt_9tx4kt0k8epm`. **The held
> `8063dd67...7166baaa` range will not rebase cleanly once `D2k` lands, and
> that is accepted** — it is preserved as evidence, not as a merge candidate,
> and `DP` rebuilds `D1`/`D2` regardless. **Whoever resumes this node
> re-derives everything from the `main` of that day**, including section 4's
> seam and section 2's fixed inputs. Do not carry a coordinate across the gap.

`scripts/ken-cargo test -p ken-runtime --lib` plus your focused suite.
**Never `--workspace`**; that is CI's gate.

**Sizing note.** The parent measured this emitter increment plus its review
cycle at **closer to one working day**. That estimate predates the interior-seam
discovery, so treat it as a floor. If you reach a hard stop inside an hour, that
is a good outcome — say so and hand back.
