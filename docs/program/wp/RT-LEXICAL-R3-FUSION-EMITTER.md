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
> **`DP` — give the producer semantic occurrence its own transported checked
> identity.** **Design class CONFIRMED by the Architect at
> `evt_2qmknsgtmy0rj`; the design-class hold is lifted.** Class 2 would require
> inventing a semantic partition the measured segment does not have, while the
> emission region is intentionally producer plus suffix.
>
> **`DP` populates and transports, from the checked source:** the producer's
> **distinct frame template and marker**, plus its real checked occurrence
> path, semantic position, segment site, input/output interfaces, occurrence
> binding, control witness, and **invocation/parent-edge relation.** The
> **unchanged** validator must then accept the complete expected frame set,
> order, endpoint composition, site, witnesses and dynamic ownership.
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

> ### `DP` IS RULED (a). THE RE-CUT IS AUTHORIZED — Architect `evt_w4nvsmrs1qhk`
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
> **The authorized population, exactly:**
>
> 1. Expand `D2G_OUTER_SLOT.frame_templates` from the singleton outer frame to
>    the complete checked invocation-local sequence containing **both** frame
>    IDs, ordered by their authoritative `semantic_position`.
> 2. Let `D2G_CALL.callee_frame_templates` **inherit** that sequence from the
>    slot, as production already does via
>    `callee_frame_templates = slot.frame_templates.clone()`. **Do not patch the
>    call vector independently.**
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
> **Required nets — these are the controls, not a wish list:**
>
> - the positive plan carries **both** frame IDs in semantic-position order, the
>   call sequence **equals** the slot sequence, both dynamic frame keys carry the
>   **same** invocation source/instance, and the full endpoint chain composes;
> - **deleting** the producer frame from the slot/call sequence restores the
>   current missing-coverage/mixed-frame refusal;
> - **permuting** the transported frame occurrences is refused by the
>   planned-order check;
> - a purported **second source** without its own checked invocation marker and
>   checked dynamic parent edge is refused;
> - the **inner slot stays independently exact** rather than widened by site
>   coincidence.
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
