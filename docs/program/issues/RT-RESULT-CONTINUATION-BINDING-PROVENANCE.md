---
id: RT-RESULT-CONTINUATION-BINDING-PROVENANCE
title: "RT-ITREE D2/D3 — the checked ITree Ret carried arm (call_checked_ih_transport_from_case_environment, core.rs:7699-7714) settles InlineNoCall and returns the transported CheckedIhCapturedEnvironment word WITHOUT applying the source continuation, so the expected ResourceBodyResult is never minted and the admitted read/write programs reach their fail-closed default. D2 localization is ACCEPTED as evidence (ac1ebdacb; no merge, no QA); D3 repairs the carried arm to apply the source continuation ONCE and green InvalidOffset. Option (c) phase separation (Architect evt_1hren6zm8mgxv): D2 measures natural reachability only; D3 owns the same-path application positive + the causal application-removal mutation."
status: ready
owner: runtime
size: M
gate: none
depends_on: [RT-ITREE-DEFAULT-SELECTION-PROVENANCE]
blocks: []
github: null
origin: "Architect hard-stop-2 ruling evt_5w03f4zbg02ry, 2026-08-26, splitting RT-ITREE-DEFAULT-SELECTION-PROVENANCE; then hard-stop-3 ruling evt_1hren6zm8mgxv, 2026-08-26 (option (c), D2/D3 phase separation, Research advisory evt_4cbecpkg2e0gs accepted). D1's route slice landed independently (21d62130); this node localizes the ResourceBodyResult continuation-binding boundary observed on top of it, then repairs it. Steward-owned recut per the ruling; the final-product ACs (AC-5 / AC-D1-PRODUCT / final InvalidOffset witnesses) live here."
---

> # HARD STOP 3 DISCHARGED 2026-08-26 — option (c), D2/D3 phase separation (Architect evt_1hren6zm8mgxv)
>
> The Architect accepted Research advisory `evt_4cbecpkg2e0gs`: there is no
> principled unmodified same-path positive for an application ABSENT from the
> only governed branch. The prior demand for a fresh pre-repair application
> positive is WITHDRAWN as impossible. Phase separation is the structural
> closure. This discharges hard stop 3; the shared predicate across all three
> stops is recorded (a downstream semantic result claimed before the
> graph-authorized predecessor operation that produces it). Durable inventory
> fold: `7e5d54b9839451d8d31d76070934af84516e7cf8` over current main.
>
> **D2 disposition — localization ACCEPTED as EVIDENCE ONLY, not a merge
> candidate.** At evidence object `ac1ebdacb8fefa79e264656c029c84fb6a69a69d`,
> `call_checked_ih_transport_from_case_environment` classifies the selected
> binding at `core.rs:7699`: the `StaticWorker` arm continues through
> capture/envelope assembly to `call_declared_unit_target` (`:7840-7846`); the
> `Value(Carried(word))` arm settles `InlineNoCall` and returns the word
> (`:7701-7713`) — NO application instruction exists on that CFG arm. The
> complete test population takes zero calls to either arm; the admitted
> read/write programs reach ONLY the carried arm with exact planner descriptors
> read `608`/`662`/`939`/spec 1 and write `720`/`1238`/`1257`/spec 3. Accepted
> classification: the expected `ResourceBodyResult` was never minted because the
> source continuation was not applied; slot 1 faithfully carries a planner-typed
> `CheckedIhCapturedEnvironment` (not a result) through the correct
> parameter-plus-seven-capture mapping. **`ac1ebdacb` stays FROZEN evidence; QA
> is NOT requested on it. No fresh pre-repair positive object is required.** The
> committed one-case Direct positive
> `d2_checked_ret_result_reaches_the_exact_continuation_capture` remains VOID —
> it must not be credited or merged under its present name/prose.
>
> **Frame correction (this recut) — three changes the ruling required.**
> (1) AC-D2-4 is REPLACED with exact natural reachability (below): D2 measures
> only that the unchanged admitted programs reach the carried branch with their
> exact identities, emit no application there, the typed result is absent, and
> each reaches its exact fail-closed default; an entry-marker/refusal mutation
> may prove reachability but must NOT apply the worker or inject a result.
> (2) The same-path application positive and the application-removal mutation
> MOVE to D3. (3) The old instruction that D3 waits for a corrected pre-repair
> positive is DELETED — D3 waits for THIS recut to land and a fresh Runtime
> release. **Runtime is HELD until this recut lands and is re-released.**
>
> Process note (Architect): the next Research trigger on this chain is hard
> stop 6 — hard stops 4 and 5 do not re-consult Research.
>
> ## Lineage (compact)
> D1 [[RT-ITREE-DEFAULT-SELECTION-PROVENANCE]] route-transport slice landed at
> origin/main `21d62130` (PR #2948), advancing the admitted programs
> monotonically to this later fail-closed boundary. This node is the D2/D3 half
> of the RT-ITREE hard-stop-2 split (Architect evt_5w03f4zbg02ry).

## Symptom inventory

Append one line per hard stop; never rewrite history.

1. Forcing the localized outer ITree carried match through an origin-only
   checked-return bypass progressed into value-deduplicated
   `ResourceBodyResult` defaults instead of `InvalidOffset` — keyed on an
   artificial predecessor-route bypass, not a planner-authorized route.
2. After D1's route repair the admitted programs naturally terminate at the
   ordinary `ResourceBodyResult` match, but the expected result is absent from
   the entire eight-entry receiving environment — keyed on a producer-to-binding
   chain that never places the source continuation's result into environment
   slot 1.
3. The pre-repair localization phase's required AC-D2-4 positive must apply the
   exact source continuation at the carried seam, but the complete runtime-test
   census reaches neither arm and the admitted read program reaches only the
   carried early-return arm — keyed on requiring the repaired operation as its
   own pre-repair control.
4. Exact WIP `7199330550f9eae611b417c30b289722cd8057b1` makes the governed
   carried call execute and return a new value, but source control then runs
   `CheckedComputationalIHInvocationReturn -> ConstructArgument(476) ->
   TerminalResumeOuter -> Computational(301)`; the later Ret-case closure 460
   still captures the prior transported environment at capture 0, and final
   `Var(1)` reaches the same default — keyed on claiming a call result before
   the intervening recursive ITree computation binds it to the later capture.

## Objective

Localize the first unresolved authority, which is UPSTREAM of ordinary
`ResourceBodyResult` selection: the checked ITree `Ret` carried arm that should
apply the source continuation and place its `ResourceBodyResult` into the
receiving environment, but instead returns the transported captured-environment
word unapplied. Then repair that arm so the admitted read-offset and
write-offset full programs green the exact `InvalidOffset` observation. Runtime
must NOT repair the default or search the environment at runtime.

## Phase structure (option (c), Architect evt_1hren6zm8mgxv)

- **D2 — localization. ACCEPTED as evidence (`ac1ebdacb`); NOT a merge
  candidate; NO QA.** Its residual is exactly D3: the lawful application
  mechanism. Its ACs are the census (AC-D2-1/2/3, satisfied by the accepted
  evidence) plus the reworded natural-reachability AC-D2-4.
- **D3 — repair. The MERGE candidate.** Runtime builds it after this recut
  lands and a fresh release. It owns the same-path application positive, the
  causal application-removal mutation, the authorized component shape, the
  conjunctive controls, and the final `InvalidOffset` product.

## Evidence objects (Architect probe-verified; evidence ONLY, not candidates)

- Localization object `ac1ebdacb8fefa79e264656c029c84fb6a69a69d` — ACCEPTED as
  D2 localization evidence per the ruling. Structural split at
  `lowering/core.rs:7699-7714`: `StaticWorker` -> capture/envelope ->
  `call_declared_unit_target` (`:7840-7846`); `Value(Carried(word))` ->
  `InlineNoCall` -> returns the word (`:7701-7713`), no application. Complete
  suite: zero calls to either arm; admitted read/write reach only the carried
  arm with read `608`/`662`/`939`/spec 1, write `720`/`1238`/`1257`/spec 3.
  Durable inventory anchor `7e5d54b9839451d8d31d76070934af84516e7cf8` over
  current main. STAYS FROZEN — do not edit, do not promote, no QA.
- Production-only parent `cc7dc7c021be67bb94f3d68de5aef8e93ffc3255` (base/current
  main `de304429c`): read naturally terminates at planned identity `36` /
  `decl:rt_parity_fs_read_at_offset_single::ResourceBodyResult`; write at `37` /
  `decl:rt_parity_fs_write_at_offset_single::ResourceBodyResult`. No
  force-origin or route bypass.
- Instrumentation object `e701eaeb972505097371761807f5dd8fa18a1522` (tree
  `d2ee1aaa8`): evidence ONLY, must NOT be promoted — its observation-trap
  pre-interning shifts diagnostic identities to 78/79, so 36/37 are correctly
  bound from the production-only parent, not the instrumented object.
- Terminal facts: the read terminal ordinary `Match` is origin `451`, owner
  `main`, path `[0,1,1,0,0,2,1,0,0,1,1,0,0]`; write is origin `464` at the same
  path. Scrutinees are origins `450`/`463` at path + final child `0`, both
  syntactic `Var(1)`. This is the ordinary `RuntimeExpr::Match` path in
  `lowering/core.rs` (`producer_route=false`); `lower_expr(Var(1))` reads env
  slot 1, obtains a `Carried` word, and calls `lower_carried_match(..., None)`.
- The complete local environment has eight entries, all `Carried`; the closed
  `env.iter()` × two-case scan returns `EnvironmentHasNoReceivingIdentity` on
  both programs — the expected result is not present in ANY of the eight slots.
  Fail-closed behavior and pre-dispatch effects are preserved: read `FsOpen ->
  BufferAllocate -> ResourceRelease(FsHandle) -> ResourceRelease(Buffer)`, no
  `FsReadAt`; write `FsOpen(source) -> FsOpen(sink) -> ResourceRelease(source)
  -> ResourceRelease(sink)`, no `FsWriteAt`, empty sink already created; both
  exit via a controlled `PatternMatchFailure`. `erasure.rs` blob `8532ced2...`
  unchanged across base/parent/object.

## Deliverables

- **D2 (localization — ACCEPTED, no new object required).** The census in
  AC-D2-1/2/3 is satisfied by the accepted evidence `ac1ebdacb`. The only
  outstanding localization AC is the reworded natural-reachability AC-D2-4,
  provable on the unchanged programs plus an entry-marker/refusal mutation at
  the carried branch that does NOT apply the worker or inject a result. NO
  repair site, NO QA on the evidence object.
- **D3 (repair — the MERGE candidate). Authorized component shape (Architect
  evt_1hren6zm8mgxv); Runtime builds to exactly this after the recut lands and
  a fresh release:**
  - Keep the exact `CheckedIhEnvironmentTransport` as the sole two-endpoint
    authority. In the `Carried(word)` branch, validate its planner record as the
    exact `CheckedIhCapturedEnvironment` for the transport's source owner and
    seat, and validate the runtime field count against the planner-declared
    capture count. The word is a capture vector — NOT code identity, NOT a
    semantic answer.
  - Project capture ordinal `i` from that word with the existing positional
    carrier projection, governed by the transport's exact source record and
    `checked_ih_capture_origin`. NEVER inspect a runtime tag, family, spelling,
    body word, or field-count coincidence to choose the path.
  - Assemble the existing `ContinuationOrdinaryEnvelopeRole` ONCE: nonrecursive
    fields still come from their ruled case-environment coordinates;
    `WorkerCapture` fields come from the exact projected carried-environment
    ordinals; continuation inputs still come from the existing transport
    morphism. Do NOT synthesize a `StaticWorkerBinding` or redirect into the
    neighboring `StaticWorker` branch.
  - Resolve only
    `function_local.continuation_calls[transport.source_call_identity()]`,
    emit ONE declared call through the existing call authority, record it under
    the exact transport, settle through the existing candidate discipline, and
    return that call's result. Factor the `StaticWorker` and carried-capture
    sources into one downstream envelope/call path rather than duplicating the
    continuation body or creating a second call lane.
  - If the source record cannot be validated, its captures cannot be projected
    from existing planner facts, or the exact continuation target was not
    already declared into this destination function — HARD-STOP. Do NOT add a
    second identity catalog, ABI lane, raw cast, environment search, or
    family-specific fallback.

## Acceptance criteria

- AC-D2-1 (environment census — accepted) — the eight-binding environment is
  fully censused, each slot bound to its exact producer / insertion op / source
  origin / binder-capture role / carried identity; slot 1 is traced
  producer-to-`Var(1)`-read through every join/continuation. Satisfied by
  evidence `ac1ebdacb`.
- AC-D2-2 (producer census + single classification — accepted) — every
  planner-authorized producer of the two receiving `ResourceBodyResult`
  identities is censused; classified as EXACTLY ONE arm: never-minted (the
  source continuation was not applied). Satisfied by evidence `ac1ebdacb`.
- AC-D2-3 (typed carrier — accepted) — the live carrier is bound to a typed
  producer identity (`CheckedIhCapturedEnvironment`, not a result) and the
  source value it represents is stated; no spelling / ABI / family / trap /
  field-count / `Var(1)`-index authority is introduced. Satisfied by evidence
  `ac1ebdacb`.
- AC-D2-4 (natural reachability ONLY — REPLACES the withdrawn pre-repair
  positive, Architect evt_1hren6zm8mgxv) — on the UNCHANGED admitted read/write
  programs: each reaches the exact graph-authorized carried branch with its
  exact transport / source-record / worker-body / result identities (read
  `608`/`662`/`939`/spec 1, write `720`/`1238`/`1257`/spec 3); NO continuation
  application is emitted on that branch; the typed result is ABSENT from the
  closed eight-entry receiving environment; each program reaches its exact
  downstream fail-closed default. An entry-marker/refusal mutation at THIS exact
  carried branch may prove both programs reach it (then byte-restore), but it
  MUST NOT apply the worker or inject a result. A neighboring `StaticWorker`
  test may remain as instrument/regression health only and MUST NOT be credited
  as same-path evidence. This AC is discharged by the accepted evidence plus the
  reachability-marker mutation; it does NOT require a post-repair positive.
- AC-D3-POSITIVE (same-path application positive — MOVED here from D2) — on the
  UNCHANGED admitted read/write programs, the unmutated D3 candidate makes each
  governed carried arrival apply its exact source continuation and proceed
  through exact `ResourceBodyOk` / `ResourceBodyErr` selection to the
  independently specified `InvalidOffset` observation and effect prefix.
- AC-D3-PAIRING (one application per arrival) — pair EVERY governed
  carried-branch arrival with EXACTLY ONE application event carrying the same
  transport identity, source record, worker body, source result, and
  destination owner. Unpaired scalar totals are INSUFFICIENT — the programs may
  legitimately reach the seam more than once.
- AC-D3-CAUSAL (application-removal mutation — MOVED here from D2) — suppress
  ONLY that production application while keeping entry, descriptor, and detector
  live; require BOTH unchanged admitted programs to return to the localized sole
  default; restore byte-identically and recover the exact positive.
- AC-D3-ATMOSTONCE — prove at-most-once STRUCTURALLY, or add the opposite
  duplicate-application mutation. A removal mutation proves at-least-once only.
- AC-D3-CHECKED-TRACE — retain a SEPARATE exact checked-route trace through
  `CheckedSelectedRecursor`, checked `CarriedEliminationEntered`, and
  `CarriedFallbackEmitted`, but do NOT substitute that trace for the
  carried-application pairing.
- AC-D3-INDEPENDENT-ORACLE — keep expected `InvalidOffset` and the effect
  prefixes INDEPENDENT of the new lowering logic. No result derived from the
  repair mechanism may serve as its own oracle.
- AC-5 / AC-D1-PRODUCT (final product, GATED on D3 — relocated from RT-ITREE) —
  on BOTH admitted programs SUCCESS is the exact `InvalidOffset` SemanticErrorV1
  observation with the preserved effect prefixes (read `FsOpen -> BufferAllocate
  -> ResourceRelease(FsHandle) -> ResourceRelease(Buffer)`, no `FsReadAt`; write
  `FsOpen(source) -> FsOpen(sink) -> ResourceRelease(source) ->
  ResourceRelease(sink)`, no `FsWriteAt`) — not merely the absence of the trap.
  The transitional route/frontier witness left by the D1 slice is replaced by
  the durable nonignored `InvalidOffset` witnesses here.
- AC-D3-SCOPE — both fail-closed defaults, `erasure.rs`, D1's private route
  lane, ordinary-case precedence, the checked-answer caller population, and the
  respective read/write effect prefixes are all preserved. PROHIBITED: scanning
  environment tags in production to "find" a matching value; family-specific
  routing; raw casts; reminting the checked answer as a `ResourceBodyResult`;
  duplicating the continuation body; bypassing the default; altering the
  already-correct parameter/capture mapping; changing D1.
- AC-NO-REGRESSION — whole-suite green in CI; local targeted `-p ken-runtime` /
  `-p ken-cli` / `-p ken-verify` only, never `--workspace`.

## Reviewers

Architect (D2 reworded reachability introduces no application/result and no
spelling/ABI/family/trap/field-count/index authority; the D3 repair is the
authorized `CheckedIhEnvironmentTransport` single-application shape — sole
two-endpoint authority, projected capture ordinals, single envelope/call path,
no `StaticWorkerBinding` synthesis, no second identity catalog/ABI lane; both
fail-closed defaults and `erasure.rs` intact) + runtime-qa (AC-D2-4 proves
natural reachability WITHOUT applying/injecting; the D3 pairing is
one-application-per-arrival with the causal removal mutation reddening and a
structural at-most-once or duplicate-application control; the checked-route
trace is retained but NOT substituted for the pairing; the final `InvalidOffset`
product holds on both programs with the exact effect prefixes, independent of
the lowering logic). QA is requested on the D3 candidate only — NOT on the
frozen `ac1ebdacb` evidence.

## Capability tier

T1 — a graph/claim continuation-binding repair reviewed on the provenance
argument (which transport, which projected capture ordinals, one application per
arrival), not a differential diff; the pre-repair localization is already an
accepted object. Size M.

## Sequencing

Lane-1 (runtime, priority). D2 localization is ACCEPTED (evidence `ac1ebdacb`,
no merge, no QA). Runtime is HELD until this recut lands and is re-released;
then the Runtime ring builds the D3 repair candidate (the merge). After this
node greens `InvalidOffset`, [[RT-RETAINED-UNIT-CALL-TARGET-DERIVATION]]
(ReadSome/Wrote) and the final four-value closure fold follow; the D1 follow-up
[[RT-CHECKED-SUCCESSOR-EMIT-REACHABILITY]] is sequenced after this node on the
single Runtime ring (ring contention, no logical dependency). PX8 stays blocked
until the whole native carried-value program lands. Single Runtime lane object
at a time.
