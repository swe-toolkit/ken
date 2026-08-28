---
id: RT-CHECKED-IH-SELF-RESUMPTION-RESULT-PROVENANCE
title: "RT-ITREE checked-IH self-resumption RESULT-PRODUCER predecessor — the THIRD sibling proof the landed architecture lacks. It has (1) exact inherited K authority plus its governed application coordinate, and (2) the exact fresh-result DESTINATION; it does NOT have the proof of WHICH EMITTED CONTROL EDGE PRODUCES the fresh dynamic result of that application. CheckedIhFreshResultDestination names a destination, never a producer. This node derives a planner-owned, typed, sealed fresh-result PRODUCER relation over the COMPLETE governed population (conceptually DirectInvocationResult / CarriedLoopExitResult; absence is NOT a default), keyed from the existing governed call coordinate plus typed active-frame/eliminator graph facts, rebuilt and validated for totality and functional agreement before publication, and extends the sanitized compile-time projection with ONLY the producer proof. A MANDATORY D0 measures the four marker-producing governed keys through the emitted carried header and owning merge FIRST: a NO is a SUCCESSFUL D0 that stops and returns coordinates to the Architect, and choosing a recursive call / frame morphism / explicit continuation is NOT pre-authorized. The predecessor neither applies K nor binds R2, so it lands BEHAVIORALLY INERT. Architect hard-stop-8 ruling evt_54efxydhb3n6w."
status: merged
owner: runtime
size: M
gate: none
depends_on: [RT-ITREE-DEFAULT-SELECTION-PROVENANCE, RT-ITREE-CHECKED-IH-RESULT-SUCCESSOR, RT-CHECKED-IH-K-AVAILABILITY-LOCATOR, RT-CHECKED-IH-GENERATED-ENTRY-ACCESS]
blocks: [RT-RESULT-CONTINUATION-BINDING-PROVENANCE]
github: null
origin: "Architect hard-stop-8 ruling evt_54efxydhb3n6w, 2026-08-27 (thr_2g0w05my2d5ym), verified on exact base 00e66312b4ef617eb658a2e75db9f99ff2c56492 / tree e286949f8fe5053e4719e54d0cc66adbe073dcdf with ambient RUST_MIN_STACK unset. Runtime hard-stopped an eighth time on the same mechanism chain (implementer observation accepted; Architect independently reproduced, log SHA-256 401c52398b2f221bfff2987f36c6f29b003318d6ecd78b8f7c53bf0c605d5352, source bytes restored to blob 4ea7b32e2295ae98ce53906a4f1941bb33fce421). On four of five governed coordinates the exact governed K application exposes NO LOCAL LoweringOperand result at CheckedComputationalIHInvocationReturn: the active self-resumption arm jumps to the already-active loop header, switches the builder to an unreachable block, and returns Lowered::RecursiveBackedge, which source.rs defines in its own comment as a PROTOCOL MARKER, not a value. This is an absence of a LOCAL result, NOT a proof that no fresh dynamic result is eventually produced — the owning carried merge may still produce it, and that is exactly what D0 measures. The ruling establishes that the missing piece is a COMPONENT BOUNDARY — a third sibling proof — and NOT one more D3B field, because adding it inside the atomic consumer would make lowering invent the very edge whose authority is absent and would repeat this chain's decomposition failure. It deliberately does NOT authorize a recursive call or frame morphism: spec/40-runtime/42-evaluation.md section 6.2 specifies the ITree driver as tail-resumptive and therefore realizable as a loop without a suspended-resumption stack, so the first candidate producer to MEASURE is the existing carried-loop exit. Steward-owned frame; the Architect supplied the provisional node name. RT-RESULT-CONTINUATION-BINDING-PROVENANCE is frozen and recut to depend_on this node; its atomic D3A+D3B consumer needs a SECOND explicit Steward release after this lands. Architect PRE-RELEASE FRAME REVIEW evt_33ajd0hmezn2c, 2026-08-27, on landed blob 94a1d4f74a6b559738ee171f8fa571f9515ab10f (origin/main f868f43c1, crates tree ae84c42092f3c1233878a8b9772b8f80fb4b6d69): mechanism, component boundary, D0 fork and its YES/NO dispositions, total sealed producer relation, typed planner-to-emission provenance, sanitized projection, three-proof separation, mutation families, absence of an arrival-count pin, tail-loop preservation, stop-9 trigger, and two-stage explicit release all APPROVED as faithful to evt_54efxydhb3n6w, with the D0 NO arm confirmed correctly written as a successful delivered result. Release was withheld pending four narrow text-only corrections, folded here: (1) this node's depends_on stated the real proof inputs instead of an empty list; (2) the local-result absence is no longer overclaimed as 'does not yield a fresh R2' in either origin or the consumer banner; (3) the Objective and Deliverables no longer authorize a second keyed read — the producer proof is a field of Governed(projection), reached through the existing single admission lookup; (4) AC-PRODUCER-KEY now pins the DirectInvocationResult variant by typed static provenance of the governed CheckedComputationalIHInvocationReturn result edge, and AC-PRODUCER-MUTATIONS splits wrong-direct-edge and wrong-loop-edge into independent mutations each with its own same-shape positive, so a neighbouring carried invocation result is not admissible as the governed producer. No mechanism, scope, fork, or sequencing change was made beyond that text, so the approval carries and no new Architect round is required."
---

> # MERGED, BUT ITS CENTRAL CLAIM IS FALSIFIED — HS9 ruling `evt_7wbxwxa74cdnr` (2026-08-28)
>
> **This node is `merged` and stays merged. Everything below it is the frame it
> was built to, and that frame's loop arm is now known to name the WRONG
> SEMANTIC OBJECT.** Read this block before treating anything below as current.
>
> **What was falsified.** Its `CarriedLoopExitResult` arm named the Ret body's
> OUTPUT — the carried elimination's merge parameter. The consumer needs that
> body's INPUT. Normative `spec/40-runtime/42-evaluation.md §6.2` is `Ret r -> r`,
> so the result is `r`; and the emitted order (header input, then Ret /
> checked-fallback input, then case environment and capture, then body
> evaluation, then merge) puts the merge parameter causally DOWNSTREAM of the
> capture. Under SSA dominance it cannot flow backward into a capture evaluated
> in its own predecessor body. The edge is not merely unrepresented — **it is
> backwards in the emitted CFG.**
>
> **Its D0 `YES` was an INSTRUMENTATION ERROR.** The observation co-emitted
> header, Ret body, merge predecessor, and merge parameter and never paired ONE
> dynamic result across them. **Co-emission is not a value-flow edge.** The
> correct answer to that D0 is **NO**. Both gates and both approvals were given
> in good faith against evidence that did not measure what it was read as
> measuring.
>
> **Nothing in `main` is behaviorally regressed and nothing is reverted.** Both
> landed fields are compile-time validation-only, the destination is discarded at
> `source.rs:4108`, and the false loop arm drives no emitted value or block. The
> node is not being reopened as a product defect.
>
> **BUT `CarriedLoopExitResult` IS LATENT FALSE AUTHORITY. D3 MUST NOT CONSUME
> IT**, and this node no longer establishes predecessor sufficiency for D3 (the
> Architect withdrew that specific consequence of its approval of `c8ddfb896`,
> landed as `830aa0952`). The replacement is
> [[RT-CHECKED-IH-FRESH-RESULT-ROUTE]], which REPLACES the
> `CheckedIhFreshResultProducer` abstraction rather than extending it. The
> `DirectInvocationResult` arm was NOT falsified and is preserved there as
> `DirectInvocationReturn`.
>
> # THIRD SIBLING PROOF — a PRODUCER, not a destination (evt_54efxydhb3n6w)
>
> Read `evt_54efxydhb3n6w` in full before building. This frame is faithful to it,
> but the ruling is the law.
>
> **The gap, stated once.** The landed architecture carries two sibling proofs:
>
> 1. **`K` inheritance** — the exact inherited continuation capability `K` and its
>    governed application coordinate
>    (`CheckedIhImmediateKBindingLocator`, `aggregates.rs:247`).
> 2. **Fresh-result destination** — where the fresh `R2` must land
>    (`CheckedIhFreshResultDestination`, `aggregates.rs:310`).
>
> It lacks a third: **which emitted control edge PRODUCES the fresh dynamic
> result of that application.** A destination names where a value goes. It does
> not name what makes the value. The active-elimination stack stores only
> `(static origin, header block)`, and the generated-entry projection carries no
> typed relation from the governed application to either a direct invocation
> return or an owning carried-loop exit.
>
> **Why this is a component and not a D3B field.** Putting the producer inside
> the atomic consumer would make lowering invent the very edge whose authority is
> missing — which is precisely the decomposition failure this chain has already
> been ruled on. The three proofs stay distinct: `K` inheritance, producer,
> destination. There is no transitive `R1 -> capture` claim.
>
> **What the HS8 evidence does and does not establish.** It establishes that
> `RecursiveBackedge` cannot be relabeled as `R2`: on active re-entry,
> `lower_carried_computational_match` (`core.rs:12194`) emits an unconditional
> jump to the already-active loop header, switches the builder to an unreachable
> block, and returns the marker only so compile-time lowering can unwind, so
> `CheckedComputationalIHInvocationReturn` has no local `LoweringOperand` result
> on four governed coordinates. The capsule residual and the earlier `R1` remain
> equally invalid substitutes. It does **not** establish that the emitted
> computation has no eventual fresh result: the same carried elimination owns a
> one-word merge block, and `lower_carried_computational_match_inner`
> (`core.rs:12267`) returns that merge parameter as `LoweringOperand::Carried`.
> The active re-entry is tail control — at runtime it jumps to the header, and a
> later `Ret` / checked-answer exit may reach the owning merge even though the
> abandoned local compiler continuation sees only the marker.
>
> **That distinction is NORMATIVE, not a convenience.**
> `spec/40-runtime/42-evaluation.md §6.2` specifies the ITree driver as
> tail-resumptive and therefore realizable as a loop without a suspended-
> resumption stack. Replacing the loop with a recursive call or an explicit
> continuation stack **before measuring the existing loop exit** would pre-empt
> the design question and could make the native mechanism LESS faithful to the
> specified operational shape. This is why D0 below is mandatory and why its
> mechanism arm is not pre-authorized.
>
> **This node lands BEHAVIORALLY INERT.** It neither applies `K` nor binds `R2`.

## Objective

Derive and validate a planner-owned, typed, sealed **fresh-result producer**
relation over the COMPLETE governed population: for each governed key, the exact
emitted control edge that produces the fresh dynamic result of the governed `K`
application. Extend the existing sanitized compile-time projection with only that
producer proof. Because the producer proof is a FIELD of `Governed(projection)`,
it is **available through the existing single admission lookup, with no
additional keyed or authority read** — there is no producer map and no producer
accessor.

The node produces the relation, its validation, its projection extension, and its
controls. It emits nothing, changes no runtime behavior, and is independently
landable.

## Fixed inputs (measured 2026-08-27)

Measured at `origin/main` **`ed1ecb121ef435c47c44863360d6e668fa877a07`**.

**`crates/` at `ed1ecb121` is BYTE-IDENTICAL to the Architect's HS8 verification
base `00e66312b4ef617eb658a2e75db9f99ff2c56492`** — both carry `crates` subtree
`ae84c42092f3c1233878a8b9772b8f80fb4b6d69`; the only paths that moved between
them are `docs/` and `agent/`. So building on current `main` builds on exactly
the code the Architect verified, and no rebase question arises. Re-measure before
you start; `main` moves.

Landed surface this node extends, all in
`crates/ken-runtime/src/cranelift_backend/planning/static_transition/aggregates.rs`:

- `CheckedIhImmediateKBindingLocator` — `:247`, `pub(in crate::cranelift_backend)`
- `CheckedIhFreshResultDestination` — `:310`, `pub(in crate::cranelift_backend)`
- `CheckedIhGeneratedEntryProjection` — `:354` (the sanitized projection)
- `CheckedIhGeneratedEntryAdmission` — `:384`, the TOTAL
  `Governed(CheckedIhGeneratedEntryProjection)` / `NonGoverned` admission map;
  **this single admission lookup remains the ONLY applicability/authority read.**

Lowering coordinates:

- `lower_carried_computational_match` — `lowering/core.rs:12194`
- `lower_carried_computational_match_inner` — `lowering/core.rs:12267` (returns
  the one-word merge parameter as `LoweringOperand::Carried`)
- the `RecursiveBackedge` protocol-marker comment — `lowering/source.rs:1969`

The existing mutation harness to copy for controls is the landed
`with_checked_ih_generated_entry_*_mutation` / `*_mutation_is_exact` pattern
(`aggregates.rs:880`, `:904`, `:933`).

**The present one-direct / four-loop split is a MEASURED TRANSITION WITNESS, not
a durable numeric law.** Do not pin arrival counts, and do not write the numbers
4 or 5 into an acceptance criterion as an expected total.

## Authorized component shape (Architect evt_54efxydhb3n6w, obligations 1-5)

1. **Planner-owned typed producer relation over the COMPLETE governed
   population.** Sealed variants, conceptually `DirectInvocationResult` and
   `CarriedLoopExitResult` (names may follow local style). **Absence is NOT a
   default** — a governed key with no derived producer is a failure, not a
   `None`.
2. **Key it from the existing governed call coordinate plus exact typed
   active-frame / eliminator graph facts.** For a loop-exit variant it must
   identify, through typed static provenance, the owning carried elimination AND
   the exact merge/result edge that receives the tail-resumed computation.
   **Emission-local Cranelift block numbers, dense numeric origins, template
   numbers, runtime words, tags, and debug strings are DIAGNOSTICS ONLY, never
   authority.**
3. **Rebuild and validate the COMPLETE relation before publication.** Every
   governed key has exactly ONE producer variant; projection from every member of
   a quotient class is functional and agrees by typed equality. No arrival-count
   pin (see Fixed inputs).
4. **Extend the sanitized compile-time projection with ONLY the producer proof**
   needed by lowering. It carries no source identity, transport ancestry, runtime
   discriminator, call-stack token, result value, or destination capture. The
   already-landed single admission lookup remains the only applicability/
   authority read.
5. **Keep the three proofs DISTINCT** — `K` inheritance, fresh-result producer,
   fresh-result destination. Do not turn them into a transitive `R1 -> capture`
   claim. This predecessor neither applies `K` nor binds `R2`.

## D0 — MANDATORY, and a NO is a SUCCESSFUL D0

**Do D0 before any mechanism implementation. It is a measurement, not a
formality.**

Trace the four marker-producing governed keys through the emitted carried header
and the owning merge, and answer one question:

> Does that existing merge produce the dynamic result of the exact governed `K`
> application, with a typed planner-to-emission morphism sufficient to hand it to
> the already-proved destination?

**YES** — the component is the typed loop-exit producer relation. Build items 1-5
above. **PRESERVE tail resumption. Do NOT add a recursive call or a stack.**

**NO** — **STOP and return** the measured missing coordinates and the ABI/control
facts to the Architect, through the runtime leader, and wait.

> ### THE NO ARM IS A DELIVERED RESULT. READ THIS BEFORE YOU TREAT IT AS A SETBACK.
>
> **A D0 that answers NO with measured coordinates has SUCCEEDED and is
> COMPLETE.** It is not a failure to deliver, not a blocked turn, and not
> something to work around, absorb, weaken, or convert into a partial YES. It is
> the deliverable this node was framed to be able to produce.
>
> The reason is in the ruling and it is normative: choosing among a static
> recursive-call / frame morphism, an explicit continuation mechanism, or another
> result-returning representation **is a genuine technical fork, may require a
> Decision, and is NOT pre-authorized by the HS8 ruling.** Only the Architect
> opens it, and only on your measurement. A ring that reaches NO and then picks a
> mechanism anyway has taken an unauthorized design decision — which is a worse
> outcome than the stop, and is exactly the failure this chain keeps producing.
>
> **What a successful NO looks like:** the four governed keys traced; the exact
> point at which the morphism is missing named in typed terms; the ABI/control
> facts that make it missing; no product edit; no fallback; no relabeling; no
> commit of a workaround. Report it and stop. **This is what "done" is on that
> arm.**

## Deliverables

- The planner-owned typed sealed fresh-result producer relation (item 1), keyed
  per item 2, over the complete governed population.
- Its rebuild-and-validate pass (item 3): totality, exactly one variant per
  governed key, functional agreement by typed equality across every quotient
  class.
- The sanitized-projection extension carrying ONLY the producer proof (item 4),
  **available through the existing single admission lookup, with no additional
  keyed or authority read.** The producer proof is a field of
  `Governed(projection)`; do NOT add a producer map, a producer accessor, or a
  second lookup of any kind.
- The acceptance controls below, using the landed mutation-harness pattern.
- **On the D0 NO arm, the deliverable is the measured coordinate report instead**
  — and the node is complete at that point pending the Architect.

No lowering behavior change. No emitted-behavior change.

## Acceptance criteria

- **AC-D0-FORK** — D0 is executed and its answer recorded before any mechanism
  work. On YES, the loop-exit producer relation is built and tail resumption is
  preserved with no recursive call and no stack. **On NO, the node STOPS with the
  measured missing coordinates and ABI/control facts returned to the Architect;
  that is a PASS of this AC, not a failure of it.** Selecting any mechanism on a
  NO without a new Architect ruling FAILS this AC.
- **AC-PRODUCER-TOTAL** — every governed key in the complete population has
  exactly ONE producer variant. Absence is a failure, never a default. Projection
  from every member of a quotient class is functional and agrees by typed
  equality. Re-derived in the validator and required EXACTLY equal to the
  planner-issued relation.
- **AC-PRODUCER-KEY** — the relation is keyed from the existing governed call
  coordinate plus typed active-frame / eliminator graph facts. **BOTH variants
  are pinned independently, by typed static provenance:**
  - a **loop-exit** variant identifies the owning carried elimination AND the
    exact merge/result edge;
  - a **direct** variant identifies the exact governed application's
    `CheckedComputationalIHInvocationReturn` result edge — **never by `Carried`
    shape, never by template number, and never by "there was a local result
    here."** A NEIGHBOURING carried invocation result must NOT be admissible as
    the governed producer.

  Substituting any Cranelift block number, dense numeric origin, template
  number, runtime word, tag, or debug string as AUTHORITY (as opposed to a
  diagnostic) FAILS.
- **AC-PRODUCER-POSITIVES** — consumer-adjacent positives independently cover
  BOTH producer variants, and show the loop rows reach an exact carried loop-exit
  result while the direct row remains direct. **No hard-coded arrival totals**;
  the present one-direct/four-loop split is a transition witness, not a law.
- **AC-PRODUCER-MUTATIONS** — INDEPENDENTLY: producer removal; producer
  duplication; cross-variant collapse; wrong active frame/eliminator; **wrong
  DIRECT invocation/result edge**; **wrong LOOP merge/result edge**; wrong
  governed key. **Each must reject at its OWN NAMED arm**, and restore
  byte-identically. **The two wrong-edge mutations are SEPARATE mutations, each
  with its OWN SAME-SHAPE POSITIVE** — a single merged "wrong result edge"
  mutation does not satisfy this AC, because it cannot distinguish a direct
  variant that is merely observing some local `Carried` outcome from one that
  has actually proved the governed result edge.
- **AC-PRODUCER-DISAGREE** — a population mutation making two source members at
  one generated-entry class DISAGREE on the producer must REJECT. It may not
  select the first member and may not split at runtime.
- **AC-PROJECTION-SANITIZED** — the projection extension carries only the
  producer proof: no source identity, transport ancestry, runtime discriminator,
  call-stack token, result value, or destination capture. The landed single
  admission lookup remains the ONLY applicability/authority read; adding a second
  authority read FAILS.
- **AC-BACKEDGE-UNCHANGED** — existing tail `RecursiveBackedge` behavior is
  UNCHANGED for non-result-demanding self-resumptions. **A global "marker becomes
  value" repair is FORBIDDEN** and fails this AC on sight.
- **AC-THREE-PROOFS** — `K` inheritance, producer, and destination stay distinct,
  exposed distinctly, with no transitive `R1 -> capture` claim. In the eventual
  atomic D3, every governed arrival pairs one `K` application to exactly ONE
  producer event, and then SEPARATELY pairs that produced `R2` to the
  destination. **The producer proof does NOT become a fourth suppression axis** —
  suppression and at-most-once remain the existing THREE semantic axes
  (inheritance, application, fresh-result binding).
- **AC-INERT** — behaviorally inert: this node applies no `K`, binds no `R2`,
  changes no emitted call / ABI / artifact / runtime behavior, and mints no
  transport, call identity, aggregate, binder catalog, ABI lane, or numeric
  identity. Emitted surfaces byte-identical.
- **AC-HS9-ADVISORY** — **if this node hard-stops, that is hard stop 9, and it
  MECHANICALLY triggers the mandatory bounded Research advisory BEFORE any
  Architect ruling — including a stop that occurs during D0.** This is a
  frame-level obligation, not a Steward judgment call and not a ring judgment
  call. Hold, trigger Research, then rule. **A D0 NO is NOT a hard stop** — it is
  the D0 success arm above and routes to the Architect without an advisory.
- **AC-NO-REGRESSION** — whole-suite green in CI. Local targeted
  `scripts/ken-cargo -p ken-runtime` / `-p ken-cli` only, never `--workspace`.

## Forbidden

Unchanged from the standing chain list, restated because HS8 re-tested it:
marker / residual / `R1` relabeling; direct capture writes; env-index-to-ABI
guesses; source-identity recovery or scans; singleton selection; runtime carrier
or runtime discrimination; new lowering-side reverse search. **Static splitting
and cloning remain forbidden-or-unclassified exactly as before** — nobody has
measured availability, and repetition does not turn "unclassified" into "ruled
out."

The landed generated-entry predecessor and the result-successor destination proof
**remain valid and are NOT reopened** by this node.

## Reviewers

**Architect** — the producer relation is a genuine third sibling proof and not a
D3B field; it is planner-owned, typed and sealed, total over the complete
governed population with absence rejected; keyed from the governed call
coordinate plus typed active-frame/eliminator graph facts with numeric/emission
coordinates as diagnostics only; the loop-exit variant identifies the owning
carried elimination and exact merge/result edge by typed static provenance;
rebuild-and-validate proves one variant per key and functional typed agreement
across quotient classes with no arrival-count pin; the projection extension is
sanitized and the single admission lookup stays the only authority read; the
three proofs remain distinct with no transitive `R1 -> capture`; D0 was executed
first and, on YES, tail resumption was preserved with no recursive call or stack.

**runtime-qa** — AC-PRODUCER-TOTAL/KEY hold and the validator equality is exact;
AC-PRODUCER-POSITIVES covers both variants with no hard-coded totals;
AC-PRODUCER-MUTATIONS each bite at their OWN named arm with near-identical
positives and byte-clean restore; AC-PRODUCER-DISAGREE rejects rather than
selecting a first member; AC-PROJECTION-SANITIZED holds with no second authority
read; AC-BACKEDGE-UNCHANGED holds and no global marker-to-value repair was made;
AC-INERT holds with byte-identical emitted surfaces.

**On the D0 NO arm there is no candidate to review.** The runtime leader routes
the measured coordinate report to the Architect; QA is not owed a verdict and
should not be asked for one.

## Capability tier

**T1.** This is soundness-bearing planner-relation design reviewed on a
provenance argument — deriving a typed producer edge from static graph facts,
proving totality and functional agreement over a quotient, and executing a
measurement fork whose NO arm requires recognizing an unauthorized design
decision and refusing it. It is not a differential or byte-faithfulness diff.
Size M.

## Sequencing

Lane-1 (runtime, priority). This is the independently-landable, behaviorally
inert predecessor for [[RT-RESULT-CONTINUATION-BINDING-PROVENANCE]], which is
FROZEN and now `depends_on` this node.

Order: this node passes its own Architect/QA gates and lands → **the Steward then
issues a SECOND EXPLICIT RELEASE** of the atomic D3A+D3B consumer, which at that
point consumes ALL THREE sibling proofs. Neither this frame landing nor this
node's landing authorizes the consumer; the consumer starts on the explicit
release and nothing else.

The merge of D3A+D3B stays ATOMIC — there is no application-only checkpoint.

Single runtime lane object at a time; PX8 stays blocked until the whole native
carried-value program lands.
