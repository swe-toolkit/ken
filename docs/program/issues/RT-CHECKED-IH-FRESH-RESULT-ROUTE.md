---
id: RT-CHECKED-IH-FRESH-RESULT-ROUTE
title: "RT-ITREE checked-IH fresh-result ROUTE relation — REPLACES the falsified CheckedIhFreshResultProducer. HS9 ruling evt_7wbxwxa74cdnr determined that for the loop rows fresh R2 is the governed K application result AS DELIVERED INTO THE RET CASE'S INPUT BINDER, not the result of evaluating that Ret body, so it is NOT the carried elimination's merge parameter: normative spec/40-runtime/42-evaluation.md section 6.2 is Ret r -> r, and the emitted order (header input -> Ret/checked-fallback input -> case environment/capture -> body evaluation -> merge) puts the merge parameter causally DOWNSTREAM of the capture, unreachable backward under SSA dominance. The landed CarriedLoopExitResult named the Ret body's OUTPUT where the consumer needs its INPUT, so it is latent false authority and D3 must not consume it. This node replaces the enum with a planner-owned typed fresh-result ROUTE relation over two variants, DirectInvocationReturn (preserving the accepted body-refined governed invocation-return relation) and TailResumedRetInput (the forward route through the existing active-self-resumption header and checked-answer/Ret input edge into the exact Ret-case binder consumed by CheckedIhFreshResultDestination). It lands BEHAVIORALLY INERT: no K application, no R2 binding. Replace, do not extend -- the old enum and arm are not retained in parallel."
status: active
owner: runtime
size: M
gate: none
tier: T1
depends_on: [RT-ITREE-DEFAULT-SELECTION-PROVENANCE, RT-ITREE-CHECKED-IH-RESULT-SUCCESSOR, RT-CHECKED-IH-K-AVAILABILITY-LOCATOR, RT-CHECKED-IH-GENERATED-ENTRY-ACCESS, RT-CHECKED-IH-SELF-RESUMPTION-RESULT-PROVENANCE]
blocks: [RT-RESULT-CONTINUATION-BINDING-PROVENANCE]
github: null
origin: "Architect HS9 ruling evt_7wbxwxa74cdnr, 2026-08-28 (thr_146dz84k4bq1q), grounded on exact origin/main@830aa0952c425684fef539d73dcb90ab3b53ae24 / tree 41193dd086e780d5311668f30703c41f8f1c4815, accepting mandatory Research advisory evt_58t039yrevmsk. Hard stop 9 was taken cleanly by runtime-implementer evt_5p5mknw26g4qq and routed through the mandatory advisory by runtime-leader evt_1g79zjszzvbx7 before any Architect ruling, as the HS8 banner evt_54efxydhb3n6w required. The ruling is deductive from the spec and emitted causality, so NO Decision object is required. Steward-owned recut per the ruling's closing instruction."
---

> # OPERATIVE — HS9 RULING `evt_7wbxwxa74cdnr` (2026-08-28). READ IT IN FULL FIRST.
>
> **This node exists because the D0 that authorized its predecessor answered the
> right question WRONG.** The prior `YES` was an instrumentation error: the
> observation co-emitted header, Ret body, merge predecessor, and merge
> parameter, and never paired ONE dynamic result across them. Co-emission is not
> a value-flow edge. **The correct answer to that D0 is NO.**
>
> **Nothing in `main` is behaviorally regressed and nothing needs reverting.**
> Both landed fields are compile-time validation-only, the destination is
> discarded at `source.rs:4108`, and the false loop arm drives no emitted value
> or block. **But `CarriedLoopExitResult` is LATENT FALSE AUTHORITY and D3 must
> not consume it.**
>
> **THE SHARED PREDICATE BEHIND ALL NINE STOPS, and the reason this is a replace
> and not a fourth field:** static endpoint facts have repeatedly been treated as
> a directed dynamic value-flow edge. HS6, HS7 and HS8 each added one more
> endpoint — availability, then access, then a "producer" — and HS9 shows the
> last endpoint was still **on the wrong side of the consumer**. **Do NOT add a
> fourth local field or another endpoint predecessor.** The structural closure is
> ONE typed, directed fresh-`R2` route whose source, intermediate tail edge, and
> sink COMPOSE.
>
> **REPLACE, DO NOT EXTEND.** The `CheckedIhFreshResultProducer` enum and its
> `CarriedLoopExitResult` arm are not retained in parallel. Code spelling may
> follow local style; the abstraction is what is replaced.

## Objective

Replace the falsified producer abstraction with a planner-owned typed
**fresh-result route** relation over the complete governed population, keyed from
the existing governed call coordinate plus typed active-frame / eliminator /
Ret-binder facts, and extend the sanitized compile-time projection with ONLY that
route proof. Land behaviorally inert.

## The two variants

- **`DirectInvocationReturn`** — PRESERVE the accepted body-refined exact
  governed invocation-return relation. This arm was not falsified; do not
  redesign it. Its landed Direct-authority controls stay.
- **`TailResumedRetInput`** — identify the exact governed call result's FORWARD
  route through the existing active-self-resumption header and the
  checked-answer/Ret input edge, into the exact Ret-case binder consumed by
  `CheckedIhFreshResultDestination`. For the four measured rows the relevant
  emitted route is `ActiveSelfResumption` with the exact checked selected-recursor
  route into the unique Ret body; **the raw checked answer occupies that body's
  one Ret-input slot. It is not the body's merge output.**

**Composition is the point, and it is what the old object lacked.** The route's
SOURCE must compose with the governed `K` application result. Its SINK must
compose with the destination's exact logical
`ConstructorChild { frame_origin, field_position: 0 }` binder and its capture
provenance.

**Preserve the ordinary/fallback distinction.** Ordinary Ret field projection and
the checked-answer fallback's DIRECT occupation of the same logical binder are
different facts. **Do not pretend the fallback dynamically projected a
constructor child.**

## Sanitization, unchanged from the landed architecture

The generated-entry capsule carries only this static route proof, through the
sole existing `admission_for` lookup. **None of these may enter it:** a Cranelift
block or value number, a source-call identity, transport ancestry, a result
value, a runtime discriminator, an ABI/frame/environment lane, a reverse search,
or a second authority read. Emission-local block/SSA identities are diagnostic
observations ONLY.

**Behaviorally inert: no `K` application and no `R2` binding happens here.** That
is D3's work, after this lands.

## Deliverables

- **D1 — the route relation.** Rebuild the COMPLETE governed relation and require
  totality, uniqueness, and quotient agreement as before, but **over routes**
  rather than endpoints.
- **D2 — the controls below**, including the one that tests the previously
  prescribed fix itself.
- **D3-obs — the emitted observation.** It must establish the FORWARD ORDER AND
  PAIRING: governed application result into the active header, exact checked
  Ret-input edge, exact destination binder. **Reporting that all three sites were
  emitted is exactly the error that produced the false D0 YES** — co-emission is
  not pairing.

## Acceptance criteria, each with its control

- **AC-ROUTE-TOTAL.** Totality, uniqueness, and quotient agreement over the
  complete governed population, rebuilt and validated before publication.
- **AC-CAUSAL-DIRECTION.** A compile-preserving mutation substituting the OLD
  `CarriedLoopExitResult` merge/output edge for `TailResumedRetInput` **must
  redden at a named causal-direction arm.** This is the required test of the
  previously prescribed fix, and it is the AC that would have caught HS9 before
  the predecessor landed. Two-sided: apply the substitution, show the build still
  succeeds, show the named arm reddens.
- **AC-NEIGHBOUR-MUTATIONS.** Independently mutate each of: the active
  frame/header relation, the checked route kind, the Ret body/input binder, the
  governed call key, and cross-variant classification — using REAL LEGAL
  NEIGHBOURS. **Each needs its own same-shape positive**; a mutation with no
  paired positive proves nothing about discrimination.
- **AC-PAIRED-OBSERVATION.** The emitted observation establishes forward order
  and pairing, not co-emission. Control: an observation that would pass on
  co-emission alone must FAIL this AC.
- **AC-REVERSAL-REJECTED.** Reversing source and sink, or using the body merge
  parameter, **must be REJECTED even when the same Ret body and frame
  coordinates remain unchanged.** The unchanged coordinates are the point: that
  is precisely the configuration the falsified object accepted.
- **AC-INERT.** Tail/backedge behavior preserved; the three dynamic semantic axes
  (inheritance, application, binding) unchanged. **The static route certificate
  is NOT a fourth suppression axis.** No `K` application, no `R2` binding, no
  capture write, no ABI/runtime carrier, no recursive call, no production stack.

## Sequencing and the stop rule

`ready`. Runtime is HELD on D3 until this corrected predecessor is explicitly
released, gated, and landed. After it passes FRESH Architect and Runtime QA gates
and lands, the Steward issues the second explicit release of the same atomic
D3A+D3B consumer — **that release is the Steward's, and landing authorizes
nothing by itself.**

**No Decision object is required** — the ruling is deductive from the spec and
emitted causality.

**IF RUNTIME CANNOT DERIVE the exact forward checked-answer-to-Ret-binder route
without prohibited authority, STOP CLEANLY AS HS10.** Do not fall back to the
merge and do not select a new mechanism. A clean stop is the correct outcome, not
a failure of the turn.
