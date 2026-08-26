---
id: RT-RESULT-CONTINUATION-BINDING-PROVENANCE
title: "RT-ITREE D2 successor — after D1's route repair advances the admitted programs to a later fail-closed boundary, the expected ResourceBodyResult is ABSENT from the entire eight-entry receiving environment (EnvironmentHasNoReceivingIdentity, read id 36 / write id 37). Localize the missing authority in the producer-to-binding chain that should place the continuation's ResourceBodyResult into environment slot 1 after the checked ITree Ret route, then repair it so the read/write programs green InvalidOffset. Localization FIRST (object read), repair gated on the object ruling."
status: ready
owner: runtime
size: M
gate: none
depends_on: [RT-ITREE-DEFAULT-SELECTION-PROVENANCE]
blocks: []
github: null
origin: "Architect hard-stop-2 ruling evt_5w03f4zbg02ry, 2026-08-26, splitting RT-ITREE-DEFAULT-SELECTION-PROVENANCE. D1's two-parameter route transport is independently landable and advances the admitted programs MONOTONICALLY to a later natural fail-closed boundary: at production-only cc7dc7c02 read terminates at planned identity 36 / ResourceBodyResult (origin 451), write at 37 (origin 464), no bypass; the ordinary RuntimeExpr::Match reads env slot 1 (Var(1)) and the expected ResourceBodyResult is absent from the whole eight-entry receiving environment (EnvironmentHasNoReceivingIdentity). Hard-stop count 2. Steward-owned split per the ruling; the final-product ACs (AC-5 / AC-D1-PRODUCT / final InvalidOffset witnesses) move here from RT-ITREE."
---

> # D2 OBJECT 1 BLOCKED 2026-08-26 — AC-D2-4 evidence failure (Architect evt_3d1rkw99dmkpj)
>
> First D2 object `ac1ebdacb` is EVIDENCE, not a candidate, and is BLOCKED for a
> corrected control (NOT hard stop 3 — an AC-D2-4 failure within hard-stop-2, no
> new downstream boundary). The localization is SUBSTANTIALLY ACCEPTED: the
> producer/result-root census (classification (i) never-minted — the source
> continuation was not applied), slot-1 typed transport (`CheckedIhCapturedEnvironment`,
> read record `608`/body `662`/result `939`; write `720`/`1238`/`1257`), and the
> eight-slot plan/ABI census all hold. What FAILS is AC-D2-4: the committed
> positive `d2_checked_ret_result_reaches_the_exact_continuation_capture` is a
> one-case Direct `ITree::Ret` whose route trace never hits `CheckedSelectedRecursor`
> — it proves ordinary static-worker capture ordering, not a checked Ret
> continuation; its name/prose overclaim. The Architect NARROWED the first causal
> authority to `call_checked_ih_transport_from_case_environment` (`core.rs:7701-7713`)
> — see the sharpened AC-D2-4 and the conditional D3 scope below. Runtime returns a
> fresh object-control correction ONLY; no QA, no repair, no D3 until the Architect
> authorizes it on the corrected control. Held evidence stays frozen.
>
> # UNBLOCKED 2026-08-26 — D1 route slice landed at origin/main `21d62130`
>
> The blocking [[RT-ITREE-DEFAULT-SELECTION-PROVENANCE]] D1 route-transport
> slice landed (PR #2948), so the `ResourceBodyResult` continuation-binding
> boundary is now reachable and this node flips `draft` -> `ready`. Held
> evidence stays frozen and NOT promoted: production-only `cc7dc7c02` and
> instrumentation `e701eaeb9`. This is D2 in the RT-ITREE object-read
> discipline — localization FIRST (return the object, then HARD-STOP for the
> Architect ruling); NO repair site before that ruling.
>
> # FRESH D2 SUCCESSOR 2026-08-26 — RT-ITREE hard-stop-2 split (Architect evt_5w03f4zbg02ry)
>
> The D2 half of the split. [[RT-ITREE-DEFAULT-SELECTION-PROVENANCE]] keeps
> the INDEPENDENTLY-LANDABLE D1 route-transport slice and closes when that
> lands; THIS node localizes the ResourceBodyResult continuation-binding
> boundary observed on top of the landed slice, then repairs it to green the
> final `InvalidOffset`. Held `draft` until the D1 slice lands (the boundary
> is only reachable once D1's route repair is in place). Runtime starts NO
> localization until the D1 slice lands and this frame is picked up.
> Localization FIRST — no repair site before the D2 object ruling. This is
> D2 in the RT-ITREE object-read discipline (like D0/D0b), not a
> repair-authorized node.

## Objective

Localize the first unresolved authority, which is UPSTREAM of ordinary
`ResourceBodyResult` selection: the producer-to-binding chain that should
place the continuation's `ResourceBodyResult` into environment slot 1 after
the checked ITree `Ret` route. Then repair that chain so the admitted
read-offset and write-offset full programs green the exact `InvalidOffset`
observation. Runtime must NOT repair the default or search the environment
at runtime.

## Evidence objects (Architect probe-verified; evidence ONLY, not candidates)

- Production-only parent `cc7dc7c021be67bb94f3d68de5aef8e93ffc3255` (base/current main
  `de304429c`): read naturally terminates at planned identity `36` /
  `decl:rt_parity_fs_read_at_offset_single::ResourceBodyResult`; write at
  `37` / `decl:rt_parity_fs_write_at_offset_single::ResourceBodyResult`. No
  force-origin or route bypass.
- Instrumentation object `e701eaeb972505097371761807f5dd8fa18a1522` (tree
  `d2ee1aaa8`): evidence ONLY, must NOT be promoted — its observation-trap
  pre-interning shifts diagnostic identities to 78/79, so 36/37 are correctly bound
  from the production-only parent, not the instrumented object.
- Terminal facts: the read terminal ordinary `Match` is origin `451`, owner
  `main`, path `[0,1,1,0,0,2,1,0,0,1,1,0,0]`; write is origin `464` at the
  same path. Scrutinees are origins `450`/`463` at path + final child `0`,
  both syntactic `Var(1)`. This is the ordinary `RuntimeExpr::Match` path in
  `lowering/core.rs` (`producer_route=false`); `lower_expr(Var(1))` reads
  env slot 1, obtains a `Carried` word, and calls
  `lower_carried_match(..., None)`. No `RoutedAnswer`, checked route,
  `EliminatorRole`, or ResourceBodyResult-specific route authority exists
  at this receiving consumer.
- The live carrier is `BoundaryClass::Constructor` but OUTSIDE the receiving
  constructor authority. Read receiving identities `DenseRange { start: 4155, len: 75 }`
  and `{ start: 4230, len: 76 }`; write `{ start: 4471, len: 76 }` and
  `{ start: 4547, len: 77 }`. Live field counts read `8` / write `7`; both receiving
  cases bind one field. The value is not a receiving `ResourceBodyResult` constructor;
  these facts do not identify what constructor it is.
- The complete local environment has eight entries, all `Carried`; the
  closed `env.iter()` × two-case scan returns
  `EnvironmentHasNoReceivingIdentity` on both programs — the expected result
  is not present in ANY of the eight slots. Fail-closed behavior and
  pre-dispatch effects are preserved: read `FsOpen -> BufferAllocate ->
  ResourceRelease(FsHandle) -> ResourceRelease(Buffer)`, no `FsReadAt`;
  write `FsOpen(source) -> FsOpen(sink) -> ResourceRelease(source) ->
  ResourceRelease(sink)`, no `FsWriteAt`, empty sink already created; both
  exit via a controlled `PatternMatchFailure`. `erasure.rs` blob `8532ced2...`
  unchanged across base/parent/ object.

## Deliverables

- D2 (localization object — the missing binding authority; NO repair) —
  return an object that binds the following, then HARD-STOP for an
  Architect object ruling:
  1. Census the complete eight-binding environment: bind EACH slot to its
     exact graph/claim producer, insertion operation, source origin,
     binder/capture role, and carried constructor identity. Slot 1 must be
     traced from its producer through EVERY join/continuation application to
     this `Var(1)` read.
  2. Census every planner-authorized producer of the two receiving
     `ResourceBodyResult` identities on the governed continuation, then
     classify EXACTLY ONE: (i) the expected result was never minted because
     the source continuation was not applied; (ii) it was minted and
     dropped/replaced before environment construction; or (iii) it was
     inserted under an incorrect binder/capture mapping. Present evidence
     already excludes "already present in another of the eight slots".
  3. Bind the actual live carrier to a TYPED producer identity (not merely
     class / field count) and state which source value it represents. No
     spelling, ABI number, family, trap, field count, or `Var(1)` numeric
     index may become authority.
  4. Supply a same-mechanism POSITIVE control where an ITree checked `Ret`
     continuation produces a `ResourceBodyResult` and the downstream
     ordinary match receives one of its exact case identities. A mutation at
     the REAL continuation-application or environment-insertion producer
     must reproduce the default while the positive control flips;
     post-default injection does NOT count.
- D2-repair / D3 (CONDITIONAL scope, per Architect evt_3d1rkw99dmkpj, pending
  the corrected AC-D2-4 control) — the first causal authority is localized to
  `call_checked_ih_transport_from_case_environment` (`core.rs:7701-7713`): the
  `Carried` selected-recursive-field arm treats the transported captured-
  environment word as the semantic force result, settles the continuation
  candidate `InlineNoCall`, and returns the word WITHOUT applying the source
  worker (read `ContinuationSpecializationId(1)` / result `939` / record `608`
  / body `662`; write specialization `3` / result `1257` / record `720` / body
  `1238`). Repair ONLY that carried-environment early-return/application
  authority: the carried word is an ENVIRONMENT, not a semantic answer — use the
  exact `CheckedIhEnvironmentTransport`, source call identity, source record,
  worker-body provenance, capture ordinals, and declared function-local target
  to apply the source continuation ONCE and return its result. PROHIBITED: alter
  the already-correct parameter/capture mapping; scan runtime tags; infer from
  family/origin/index; remint `ResourceBodyResult`; duplicate the continuation
  body; bypass a default; change D1. If the exact source worker cannot be
  declared/applied from those existing planner facts, HARD-STOP rather than
  invent a second identity or ABI lane. NO repair site before the corrected
  object control lands and the Architect authorizes D3.

## Acceptance criteria

- AC-D2-1 (environment census) — the eight-binding environment is fully
  censused, each slot bound to its exact producer / insertion op / source
  origin / binder-capture role / carried identity; slot 1 is traced
  producer-to-`Var(1)`-read through every join/continuation.
- AC-D2-2 (producer census + single classification) — every planner-authorized
  producer of the two receiving `ResourceBodyResult` identities is censused and the
  boundary is classified as EXACTLY ONE of never-minted / dropped-before-construction /
  incorrect-binder-mapping, before any repair site is named.
- AC-D2-3 (typed carrier) — the live carrier is bound to a typed producer
  identity and the source value it represents is stated; no spelling / ABI /
  family / trap / field-count / `Var(1)`-index authority is introduced.
- AC-D2-4 (positive control + real-producer mutation — SHARPENED by Architect
  evt_3d1rkw99dmkpj after the first object's positive overclaimed) — the
  positive must be an actual TWO-CASE Ret/Vis fixture whose route trace proves
  an exact planner-authorized `CheckedSelectedRecursor` predecessor, a checked
  `CarriedEliminationEntered`, and `CarriedFallbackEmitted`, followed by
  downstream exact `ResourceBodyOk`/`ResourceBodyErr` selection. A one-case
  Direct `ITree::Ret` program whose trace is
  `DirectScrutinee -> CarriedEliminationEntered{Direct} -> CarriedDefaultSealed`
  (no `CheckedSelectedRecursor`, no `CarriedFallbackEmitted`) does NOT satisfy
  this AC — it proves only ordinary static-worker capture ordering, not a
  checked Ret continuation. The mutation must act on the SOURCE-CONTINUATION
  application/transport seam reached by that checked fixture (a generic
  static-worker capture-order mutation on a Direct one-case program is
  insufficient), reproduce the sole downstream default, and carry an
  application witness; restore recovers the exact positive. The positive's
  transported value must be bound to its exact planner source record and
  worker-body identity — read `608`/`662`, write `720`/`1238` — so it is the
  SAME mechanism as the governed programs, not a neighboring call path.
  Post-default injection does not count.
- AC-5 / AC-D1-PRODUCT (final product, GATED on D2-repair — relocated from
  RT-ITREE) — on BOTH admitted programs SUCCESS is the exact `InvalidOffset`
  SemanticErrorV1 observation with the preserved effect prefixes (read
  `FsOpen -> BufferAllocate -> ResourceRelease(FsHandle) ->
  ResourceRelease(Buffer)`, no `FsReadAt`; write `FsOpen(source) ->
  FsOpen(sink) -> ResourceRelease(source) -> ResourceRelease(sink)`, no
  `FsWriteAt`) — not merely the absence of the trap. The transitional
  route/frontier witness left by the D1 slice is replaced by the durable
  nonignored `InvalidOffset` witnesses here.
- AC-D2-SCOPE — both fail-closed defaults, `erasure.rs`, D1's private route lane,
  ordinary-case precedence, the checked-answer caller population, and the respective
  read/write effect prefixes are all preserved. PROHIBITED: scanning environment tags
  in production to "find" a matching value; family-specific routing; raw casts;
  reminting the checked answer as a `ResourceBodyResult`; duplicating the continuation
  body; bypassing the default.
- AC-NO-REGRESSION — whole-suite green in CI; local targeted `-p ken-runtime` /
  `-p ken-cli` / `-p ken-verify` only, never `--workspace`.

## Reviewers

Architect (the missing binding authority is localized by graph/claim
producer, not by spelling / ABI / family / trap / field-count / index; the
classification is exactly one arm; the positive control mutates the real
producer; both fail-closed defaults and `erasure.rs` are intact) + runtime-qa
(the D2-4 control discriminates; the final `InvalidOffset` product holds on
both programs with the exact effect prefixes). The D2 object read HARD-STOPS
into an Architect ruling before any repair.

## Capability tier

T1 — an environment/producer localization against a value-deduplicated
identity and a graph/claim continuation-binding repair reviewed on the
provenance argument (which producer, which insertion, which binder mapping),
not a differential diff. Size M.

## Sequencing

Lane-1 (runtime, priority). The D2 half of the RT-ITREE hard-stop-2 split;
observed on top of the landed [[RT-ITREE-DEFAULT-SELECTION-PROVENANCE]] D1
route slice (the ResourceBodyResult boundary is only reachable once the
route repair is in place), so it is `draft` until that slice lands.
Localization first (the D2 object read), repair gated on the Architect
object ruling. After this greens `InvalidOffset`,
[[RT-RETAINED-UNIT-CALL-TARGET-DERIVATION]] (ReadSome/Wrote) and the final
four-value closure fold follow; PX8 stays blocked until the whole native
carried-value program lands. Single Runtime lane object at a time — does not
co-run with the D1 slice.
