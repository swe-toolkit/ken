---
id: RT-COMPOSED-RETURN-PRODUCED-TRANSFER
title: "Carry the closed predecessor's FULL route-specific objective: add Direct's ruled declared call whose local return is fresh R2, replace the general composed-return protocol with one exhaustive affine state machine carrying Tail's produced result to the exact Tail consume, and bind the delivered R2 from BOTH variants through each independently derived Ret capture. Atomic over Direct application + Tail produced transfer + both-arm D3B + products."
status: active
owner: runtime
size: L
gate: none
tier: T1
depends_on: [RT-COMPOSED-RETURN-TRAMPOLINE-EXHAUSTIVE]
blocks: []
github: null
origin: "Architect HS14 component-design ruling evt_7gnw8s9k7rh6, 2026-08-29, answering Steward disposition request evt_6dvw8j96w2sdx. Arm 1 selected: widen the composed return protocol. Arm 2 collapses into arm 1 (an earlier coupling still needs a carrier, and the only candidates are the already-rejected runtime data / capture / control stores, or arm 1). Arm 3 is true only of the predecessor's scope, not of the objective. This node is the Steward's recut of that ruling; the behaviour-preserving trampoline prefix is split out as RT-COMPOSED-RETURN-TRAMPOLINE-EXHAUSTIVE. Steward-owned cut per COORDINATION section 2."
---

> # OPERATIVE AND TOPMOST — LOCAL-RETENTION REFUTED. D1-D4 HELD. ONLY D0b RUNS.
>
> **Architect ruling `evt_1nadp56v2k4rd`, 2026-08-29, on the runtime ring's
> constructibility stop `evt_6dhkkaejn7tnd`.** Binds base
> `cc070e7c9c4b3e782041afc2c6596357a92c9491`, tree
> `00c143e2ba30c356490f2e7797add705daaae990`.
>
> **What was refuted is a sentence THIS FRAME asserted**, not a claim the ring
> invented. The old point 2 said *"Retain that projection locally through call
> dispatch."* Measured evidence falsifies it: the governed read row is
> reached/validated 29 times with Direct/Tail dispatch **0/0** (log SHA-256
> `a4eb5a16ae67ff83583e6ffa897b0f6c514c9df8cd51ddeef9193bf012bf708f`), and the
> move-only diagnostic reaches `ResumeOuter` before the exact dispatch/mint and
> fails with *"an unconsumed governed projection reached source ResumeOuter"*
> (log SHA-256
> `4f87109f19faf2005be0782e752115c64a766eaf777bf750ce419d9da0283eea`).
> Eight-path evidence diff SHA-256
> `31b9db53dbd11c18c8790f43f5c2a81c0d90a88abd4caefdb25686a3351fa2ac`.
>
> ⇒ **`ResumeOuter` is a real ownership boundary, not an ignorable
> implementation detail.** A frame sentence that reads as a design convenience
> ("just keep it local") was doing load-bearing work, and the measurement is what
> exposed it. **This is the D0 hard stop paying off exactly as its HARD STOP
> clause intended — it is a complete deliverable, not a failure.**
>
> **THE EVIDENCE DRAFT IS NOT A CANDIDATE AND MUST NOT BECOME THE REPAIR.** Its
> `SourceControl.pending_checked_ih_projection: Option<_>` is **exactly the
> prohibited sibling control lane**, and its late `ConstructArgument` handling
> must not become a substitute mint. Do not edit the preserved evidence worktree.
>
> **THE VARIANT COUNT WAS NEVER THE INVARIANT.** The number of enum states was
> contingent on the refuted locality premise. **The invariant is one exhaustive
> affine owner — not "three states."** Do not defend a count.
>
> **STATUS: D1-D4 HELD. NOT DISPOSITIONED.** The node is not refuted; one
> permitted in-place protocol correction exists, and it is written into "Required
> protocol shape" below. **Only the corrected D0b may run, and only after the
> Steward separately releases it** — this amendment is not that release. No HS15,
> no fallback, no candidate, no D3 partial, no new predecessor. **The HS15
> Research trigger is untouched.**

> # ATOMIC FROM THE FIRST PRODUCTION MINT. NO PARTIAL LANDING.
>
> Architect, verbatim: *"From the first production mint onward, producer ->
> construction -> composed propagation -> exact Tail consume -> D3B Ret-input
> binding -> read/write `InvalidOffset` controls is one semantic candidate.
> **Nothing may land with a live `Produced` state dropped or merely reported.**"*
>
> **A dropped-or-reported `Produced` is the precise failure this node exists to
> prevent**, so it can never be an acceptable intermediate state. There is no
> application-only checkpoint and no evidence-only partial.
>
> ### THE ATOMIC OBJECT IS FOUR THINGS, NOT TWO (Architect `evt_3vpt967507xw3`)
>
> **Direct application + Tail produced transfer + BOTH-ARM D3B + products.**
>
> **This corrects a Steward omission in the first recut.** That version scoped
> the node to the Tail transfer and D3B alone, and said of Direct only that
> `DirectInvocationReturn` "remains an explicit ordinary/direct arm" — which is
> the ruled PROTOCOL SHAPE, not a deliverable. **No production Direct call ever
> landed: Direct D3A was FROZEN with Tail, never discharged.** So in this tree
> "Direct remains ordinary" **preserves the MISSING Direct application** instead
> of carrying the objective forward. **A protocol-shape constraint is not a
> substitute for a scope item, and a sentence that reads as preservation can
> authorize an omission.**
>
> **RELEASED by the Steward, 2026-08-29 06:30 UTC. `active`.**
>
> The prefix is confirmed `merged` **by blob identity, not ancestry** — the
> publisher squashes. At `origin/main`
> `fdbf35686104b527d9eb74f15ac67a4eaa1436c5` (PR #3083, merged 06:21:52 UTC),
> `crates/ken-runtime/src/cranelift_backend/lowering/core.rs` is blob
> `bde7db36c8190875492b87f2839bd3a20420d5b5`, identical to the gated candidate
> `aab371f951746ef3ce922185922fa14f060d925a`.
>
> **The prefix landed INERT, and that is what makes this node's baseline
> trustworthy.** Its `AC-PRODUCTS-UNCHANGED` was discharged as a differential,
> not a green suite: baseline and candidate each emitted 122 finalized objects
> over the same 993-pass population, and the sorted
> `(label, length, SHA-256(bytes))` ledgers are byte-identical at
> `da4739dc20e67ea8d75c849c1461b205c9fc27d6e478a9c82f958e86bf3cc48d`. **So every
> product this node changes is a product THIS node changed** — there is no
> inherited drift to disentangle from your own.
>
> Two facts from the prefix that constrain the work here:
>
> - **Both `ProducerTrampolineStep` payloads are boxed, and that is
>   load-bearing.** Direct payloads and boxing only `Continue` each reproduce a
>   cold-path stack overflow in `rt_cold_lowering_path_enumeration`; boxing both
>   restores it without touching a test stack. Do not "simplify" it. The reason
>   is stated in the enum's own comment — keep it there.
> - **`ComposedReturn` currently has only `Ordinary`.** This node is what adds
>   the pending states, so the exhaustive match that is trivially total today
>   becomes the real obligation. `AC-EXHAUSTIVE-PROPAGATION` bites here, not in
>   the prefix.

## What this preserves, and what was refuted

[[RT-RESULT-CONTINUATION-BINDING-PROVENANCE]] is **closed as structurally
refuted at HS14** — not abandoned. **The semantic objective is NOT refuted:**
`spec/40-runtime/42-evaluation.md §6.2` still requires perform -> observe ->
single tail resume, and the two admitted products still distinguish the wrong
seed from the fresh `R2`. What was refuted is delivering that edge **while
forbidden to change the general composed-return protocol.** This node's scope
explicitly includes changing it.

**Do not create another predecessor.** The Architect ruled it directly: no HS15
and no further predecessor on this axis. Fourteen stops established the
obstruction; the remaining work is construction, not localization.

## Required protocol shape (CORRECTED by `evt_1nadp56v2k4rd` — do NOT redesign it)

**One exhaustive affine state machine, NOT `Option` beside `LoweringOperand`.**
The correction adds **one pre-mint logical state, `Selected`**. It does **not**
move the mint earlier and does **not** add a second carrier.

1. **`Ordinary`** owns ordinary evaluation/return work.
2. **`Selected`** owns the one already-selected, already-validated governed
   projection **together with the entire source-machine/composed work that must
   reach its exact producer.** It owns **no** produced operand, and therefore
   **does not move the mint earlier.** This is what replaces the refuted "retain
   the projection locally through dispatch" — the projection is carried by an
   owner that survives `ResumeOuter`, not by locality.
3. **At the exact ruled producer**, consuming `Selected` **and** the exact
   returned operand **together** performs the route-specific transition:
   **Direct** consumes it into the ruled ordinary/direct fresh-`R2` result;
   **Tail** consumes it into **`Produced`**, carrying the exact source-call
   identity and the projection-narrowed **opaque Tail destination certificate**.
   The exact Tail mint remains the return of
   `call_checked_ih_transport_from_case_environment` at the ruled source
   producer.
4. **`Produced`, `ConstructedWithProduced`, and `Routed` retain the already-ruled
   affine transitions**, unchanged by this correction. At `ConstructArgument` a
   `Produced` field is **borrowed** to emit/store the constructor field, then the
   **same Rust owner is moved out** — **no operand clone**; the transition
   becomes `ConstructedWithProduced { current: constructed, produced }`. A second
   live `Produced` **fails closed**. **`ConstructArgument` neither selects a
   projection nor mints anything** — it only borrows/moves an already-minted
   `Produced`.
5. Every composed/source return boundary propagates the state **exhaustively**.
   **No helper may call `into_value` on a pending state.** At a boundary that
   genuinely accepts only ordinary output, extraction is an exhaustive `Ordinary`
   match and **any pending state is a compiler error.**
6. At the exact later Tail route, match the current destination coordinates
   against the carried opaque destination certificate, consume `Produced` to form
   `Routed`, and have the active jump consume `Routed` **exactly once**. The
   certificate uses the landed Tail route for **retained topology/destination
   facts only — it must NOT resurrect `TailResumedRetInput` as value authority.**
7. **`DirectInvocationReturn` remains an explicit separate ordinary/direct arm**
   in the protocol. **That is a shape constraint and NOT the Direct deliverable —
   see D-DIRECT below, which is what actually discharges the obligation.**

### Where `Selected` may live, and where it may NOT

**`Selected` is permitted ONLY as a variant of the same exhaustive
compiler-owned state machine.** It is **not** permitted as an `Option` field on
`SourceControl`, an environment/capture binding, a side table, a function-local
receipt, an ABI/runtime carrier, or a recoverable identity. **It must be
non-`Clone` and non-`Copy`.** The state variant owns the whole next
evaluation/return work, **so projection and control cannot exist
independently** — that inseparability is the mechanism, not a style preference.

**Two forms, ONE owner.** The logical `Selected` state may need an
**evaluation-form** and a **return-boundary form**, because no `LoweringOperand`
exists while an expression is still being evaluated. **Those are two
representations of one moved owner, not two tokens**, and **every conversion
must consume the prior form.**

**At `ResumeOuter`**, the selected return crosses through the general composed
protocol and the exact active successor **without `into_ordinary`**. If the
successor enters source evaluation, ownership **moves back** into the selected
evaluation form.

**Fan-out before the exact producer must prove a sole successor or fail
closed.** Either prove one selected successor and move the state into that sole
successor, or fail closed. **Cloning the work into branches is forbidden.** An
ordinary terminal, a join, an unrelated constructor, a second governed
admission, or a mismatched producer **reached while `Selected` is live is a
named error.**

## Deliverables

- **D0 — DISCHARGED, and it returned a refutation.** The mint-point and boundary
  census ran and **falsified this frame's own local-retention sentence** (see the
  topmost banner for the bound logs). That is D0 working, not D0 failing. Its
  evidence worktree is **preserved and must not be edited**.
- **D0b — CORRECTED FEASIBILITY MEASUREMENT. Measurement only; SEPARATELY
  RELEASED.** Establish whether the `Selected` owner is **constructible** on the
  exact read and write routes, before any production redesign resumes:
  1. Mint **one scratch-only, non-clone `Selected` token** immediately after the
     existing governed validation.
  2. Move **that same token** across **every actually reached** source/composed
     boundary, **including `ResumeOuter` and the active successor.**
  3. Require it to **arrive at the exact ruled producer** with the same
     projection source coordinates and the exact `ContinuationCallIdentity`, then
     record whether the route is **Direct or Tail**.
  4. Establish **pointwise** `governed reached == Selected minted == exact
     matching dispatch`, with the **Direct/Tail counts PARTITIONING the governed
     count**. **No aggregate-only total** — the refuted premise survived because
     0/0 dispatch sat under a healthy-looking governed count of 29.
  5. Controls, each failing **at a named boundary BEFORE any call or jump**:
     suppressed `ResumeOuter` forwarding; a duplicated token; an independently
     wrong source identity; a wrong route variant.
  6. Compile the **complete nominal caller closure, `cfg(test)` included**, then
     **byte-restore the diagnostic.**

  **D0b lands no production, creates no candidate, and routes no QA.**

  **Both outcomes are complete deliverables.** If the same token **cannot** reach
  the exact producer without fan-out, recovery, or another forbidden store,
  **return NO and the node is dispositioned — do NOT invent another mechanism.**
  If **YES**, the Architect reviews the exact report and **the Steward
  separately releases the amended D1-D4 build.** Neither outcome authorizes
  starting D1.
- **D1 — the affine state machine**, points 1-7 above.
- **D2 — the exact Tail consume**, point 5, consumed exactly once.
- **D-DIRECT — add Direct's ruled declared call.** Retain the body-refined
  `CheckedIhEnvironmentTransport`; **validate its source record and capture
  projection**; resolve **only**
  `continuation_calls[transport.source_call_identity()]`; emit **exactly one**
  ruled declared call; **treat its local return as fresh `R2`**; and pair it to
  the exact governed Direct arrival. **This is the
  predecessor's still-live D3A obligation for the Direct variant** — it was
  frozen alongside Tail and never discharged, so it is added here, not
  preserved by leaving Direct "ordinary".
- **D3 — the Ret-input binding** (the surviving D3B): bind the delivered fresh
  `R2` **from BOTH variants** through ordinary Ret-case/capture semantics, with
  the read and write analogues **derived independently** from their own planner
  facts. **Both arms, not Tail alone.**
- **D4 — the read/write `InvalidOffset` products** and the fresh-`R2`/seed
  inequality proof.

**D1-D4 are ONE candidate.** D0 may be reported before the rest.

## Acceptance criteria, each with its control

### D0b ACs — these gate the MEASUREMENT, and nothing else

- **AC-D0b-POINTWISE-PARTITION.** Report **pointwise** `governed reached ==
  Selected minted == exact matching dispatch`, with **Direct and Tail counts
  PARTITIONING the governed count.** Control: an **aggregate-only total does not
  satisfy this AC and must be rejected.** **This AC exists because the refuted
  premise survived behind exactly that gap** — a governed count of 29 read as
  healthy while Direct/Tail dispatch was 0/0. A total that cannot expose a 0/0
  split certifies the defect it is meant to catch.
- **AC-D0b-SAME-TOKEN-CROSSES-RESUMEOUTER.** The **same** non-clone token, minted
  once immediately after the existing governed validation, arrives at the exact
  ruled producer with the same projection source coordinates and the exact
  `ContinuationCallIdentity`. Control: the traversal must cross **every actually
  reached** boundary including `ResumeOuter` and the active successor — **a route
  that is not reached is not evidence**, and a token that arrives by any path
  other than being moved fails this AC.
- **AC-D0b-NEGATIVE-CONTROLS-FAIL-EARLY.** Each of suppressed `ResumeOuter`
  forwarding, a duplicated token, an independently wrong source identity, and a
  wrong route variant fails **at a named boundary BEFORE any call or jump.**
  Control: exhibit each failure and its boundary name. **Vary them
  independently** — a shared failure proves only that something in the union
  matters.
- **AC-D0b-NO-PRODUCTION.** D0b lands no production, creates no candidate, and
  routes no QA. Control: the diagnostic is **byte-restored**, and the complete
  nominal caller closure compiles with `cfg(test)` included.
- **AC-D0b-NO-IS-COMPLETE.** A NO verdict — the token cannot reach the exact
  producer without fan-out, recovery, or a forbidden store — **is a complete
  deliverable and must be returned as one.** Control: none needed; this AC exists
  to stop a NO being treated as a failed turn worth retrying with a new
  mechanism.

### D1-D4 ACs — HELD. These gate the build, which is not released.

- **AC-EXHAUSTIVE-PROPAGATION.** Removing any propagation arm is a **compile
  error** or a named fail-closed error. Control: delete an arm, show the exact
  failure, byte-restore.
- **AC-NO-PENDING-ESCAPE.** No helper calls `into_value` on a pending state, and
  a pending state at an ordinary-only boundary is a compiler error. Control:
  suppress only the final consume while **preserving** the mint, and observe a
  **pending-state escape error — NOT `Unavailable`**, and not a silent default.
- **AC-CERTIFICATE-DISCRIMINATES.** Mutate the carried destination certificate
  and the source identity **independently**, and observe refusal **before** the
  active jump. Keep an ordinary/direct positive control. **Vary each
  independently — a stated arm count is both wrong and narrowing**, since it
  turns an open family into a checklist somebody can finish.
- **AC-SUPPRESSION-RETAINS-OLD-PRODUCTS.** Suppress **only** the `Produced`
  propagation while **preserving construction**, and observe both products
  retain the **old wrong seed/default**. **This is the two-sided proof the whole
  chain turns on** — a suppression that reds for any other reason proves
  nothing.
- **AC-DIRECT-APPLICATION.** Exactly ONE ruled declared call is emitted per
  governed Direct arrival, resolved **only** through
  `continuation_calls[transport.source_call_identity()]`, with its source record
  and capture projection validated and its local return treated as fresh `R2`.
  Control: mutate the resolved identity and observe refusal; a Direct arrival
  that silently emits zero calls must FAIL, not pass quietly. **Zero emitted
  calls is the pre-existing state, so an AC that cannot distinguish zero from
  one certifies the defect.**
- **AC-ROUTE-SUPPRESSION-INDEPENDENT.** Direct application must be
  mutable/suppressible **independently** of Tail production, Tail propagation,
  Tail consume, and D3B binding — five separately movable axes, and the parent's
  route-variant discriminator is preserved. Control: suppress each alone and
  observe a distinct, named failure. **A shared suppression proves only that
  something in the union matters**, which is how a two-arm defect hides behind a
  one-arm control.
- **AC-BOTH-ARM-BINDING.** D3B binds the delivered fresh `R2` for **both**
  variants, each through its own independently derived Ret capture. Control: an
  arm-specific mutation reds only its own arm; keep ordinary/direct positive
  controls alongside.
- **AC-AFFINE-ONCE.** Duplicate mint and duplicate consume must **fail closed**.
  Control: attempt each and show the named refusal.
- **AC-NO-CLONE.** The constructor field is emitted from a **borrow** and the
  same owner is moved out — no operand clone anywhere on the path. Control:
  assert at the natural producer, not at the consumer.
- **AC-PRODUCTS-EXACT.** The successful candidate proves fresh-`R2`/seed
  **inequality** and the independently derived read and write Ret bindings, with
  the exact `InvalidOffset` products — **exact text, never merely a changed
  default.**
- **AC-CALLER-CLOSURE-COMPILES.** The complete nominal caller closure compiles,
  `cfg(test)` callers included. Control: name the closure; show the build log
  covering every named caller. **A `-p <crate> --lib` build does not compile a
  `cfg(test)` caller.**
- **AC-AFFECTED-CLOSURE.** Cover every target that loads any module whose
  closure this increment changes, whether or not it touches that target's file.
  **Scope by which PATHS changed, never which VALUES changed.** Targeted via
  `scripts/ken-cargo`, never `--workspace`.
- **AC-NO-REGRESSION.** Whole-suite green in CI.

## FORBIDDEN

No empty Tail state, `Unavailable`, or fallback. No cursor proximity or any
recovery by liveness/number/proximity. No runtime carrier, ABI/header field,
capture write, side table, post-emission rewrite, or `answer_route` promotion.
**No resurrection of `TailResumedRetInput` as value authority.** No `Option`
beside `LoweringOperand` in place of the exhaustive state. No Vec/stack for
multiplicity.

**Added by the correction `evt_1nadp56v2k4rd`:**

- **No sibling control lane.** `SourceControl.pending_checked_ih_projection:
  Option<_>` — the shape in the preserved evidence draft — is **prohibited**, as
  is any environment/capture binding, side table, function-local receipt, or
  recoverable identity holding the selection.
- **`Selected` must be non-`Clone` and non-`Copy`**, and must exist only as a
  variant of the one exhaustive compiler-owned state machine.
- **No fan-out cloning.** Branching before the exact producer either proves a
  sole selected successor and moves the state into it, or fails closed.
- **`ConstructArgument` may not select a projection or mint anything.** The late
  `ConstructArgument` handling in the evidence draft **must not become a
  substitute mint.**
- **Do not edit the preserved evidence worktree**, and do not promote the
  evidence draft into a candidate.
- **Do not defend a variant count.** The invariant is one exhaustive affine
  owner; "three states" was contingent on the refuted premise.

## Trust boundary — settled, no operator escalation

Architect, on the Steward's TCB question: this does **not** add to the proof TCB
defined by `spec/60-security/64-trust-model.md §1` — no kernel code, primitive
declaration, postulate, FFI signature, or ABI surface is added. It changes code
inside the **already-existing** native-runtime correctness assumption (§4.3) and
its audit surface, and replacing the clone/out-parameter escape with an
exhaustive affine enum makes that surface **more** explicit. **There is no
operator escalation on TCB-growth grounds** — do not re-raise it.

## Capability

**T1, size L.** Novel design realization with a soundness-bearing affine
discipline, an open family of discriminating mutations, and a two-sided
suppression proof. This is the axis on which the predecessor chain took fourteen
hard stops.

## Contention check

Touches `source.rs`, `core.rs`, and the planner route surface.
[[RT-COMPOSED-RETURN-TRAMPOLINE-EXHAUSTIVE]] touches `core.rs` and **must land
first** — that is why it is `depends_on` and not a parallel node.
[[RT-RESULT-CLOSURE-LIFETIME-CONTAINMENT-CONTROL]] is `active` over
`aggregates.rs`; **it yields to this node only on an explicit Steward release,
never automatically.** Re-measure at release.

## Sequencing

**`active`.** The prefix [[RT-COMPOSED-RETURN-TRAMPOLINE-EXHAUSTIVE]] is
`merged`; this node was released 2026-08-29 06:30 UTC, and D0 has since run and
returned a refutation.

**CURRENT SEQUENCING STATE, 2026-08-29:**

- **D1-D4 are HELD.** Not refuted, not dispositioned. No candidate, no partial,
  no D3 increment is authorized.
- **D0b requires its own explicit Steward release.** This amendment is **not**
  that release — amending a frame and starting a turn are two separate acts, and
  the ring is correct to wait for the second. **No Runtime release follows from
  a ruling itself**, here or at HS14.
- **On D0b returning YES:** the Architect reviews the exact report, and **the
  Steward separately releases the amended D1-D4 build.** D0b's own YES does not
  start D1.
- **On D0b returning NO:** the node is dispositioned by the Steward. **Do not
  invent another mechanism** to keep it alive.

**No HS15, no fallback, no new predecessor. The HS15 Research trigger is
untouched** — this is a correction of the HS14 realization frame after a measured
false premise, not a new hard stop on the axis.
