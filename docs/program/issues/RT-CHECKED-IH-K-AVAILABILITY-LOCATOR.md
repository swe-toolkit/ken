---
id: RT-CHECKED-IH-K-AVAILABILITY-LOCATOR
title: "RT checked-IH immediate K-availability locator — the planner-only predecessor EXTENSION that completes the operational witness of the existing K-inheritance proof: enrich the forward binder-resolution walk so that where derive_checked_ih_bindings sees RuntimeExpr::Var(index) resolve to CheckedBinderProvenance::InductionHypothesis(binding) it RETAINS an immediate K-binding locator instead of discarding index; the locator is a dedicated non-interchangeable checked-IH type naming the exact descendant invocation_origin, the exact callee_origin whose immediate environment is indexed, an explicit immediate-invocation-environment domain tag, and the environment index (consumer + environment identity travel with the number), stored on the same CheckedIhSelfResumptionStep that already carries invocation_origin/call_origin/callee_origin/callee_binding, derived only when the zero-argument callee's exact binder resolution yields that step's CheckedIhBinding, re-derived and exact-equality-checked by the planner validator. It exposes semantic K identity and immediate availability SEPARATELY (as inheritance and fresh-R2 destination already remain separate), adds NO standalone search accessor, changes NO emitted call/ABI/artifact/result/runtime behavior, and is INDEPENDENTLY LANDABLE. It is HS6 predecessor incompleteness, not a third semantic successor; the atomic D3A+D3B consumer is explicitly RE-RELEASED against this extended base AFTER it lands. Architect HS6 component ruling evt_bqyqcvn0ng1d incorporating Research advisory evt_1t84ypm156mqh."
status: merged
owner: runtime
size: M
gate: none
depends_on: [RT-ITREE-CHECKED-IH-RESULT-SUCCESSOR]
blocks: [RT-RESULT-CONTINUATION-BINDING-PROVENANCE]
github: null
origin: "Runtime hard stop 6 on RT-RESULT-CONTINUATION-BINDING-PROVENANCE (runtime-implementer evt_3e4qm2nhq4pzx, thread thr_396kgnc2fanq2): the landed predecessor RT-ITREE-CHECKED-IH-RESULT-SUCCESSOR (7a3f6935e) proves WHICH semantic capability K survives and WHICH invocation uses it, but its consumer projection exposes only owner/body/frame/recursive-position — NOT the domain-tagged runtime coordinate at which the inherited continuation capability K is immediately AVAILABLE at the final CheckedIhBinding. Two consumer guesses (source recursive slot; final recursor residual) each selected a non-K word and returned runtime failure -1; branch reverted clean at c1945c6fb, no candidate. HS6 is this chain's designated sustained-Research trigger (Steward evt_2gndchjj9q73n; Research triggered evt_7hv3grzarrja2, advisory evt_1t84ypm156mqh). Architect HS6 component ruling evt_bqyqcvn0ng1d (incorporating the advisory) rules the closure is a planner-owned predecessor EXTENSION, not a lowering fallback and not another authorized consumer route (none exists). Steward-owned durable predecessor-extension/erratum frame; the atomic D3A+D3B consumer contract is unchanged and RE-RELEASED explicitly after this lands through ordinary gates."
---

> # PLANNER-ONLY PREDECESSOR EXTENSION — immediate K-availability locator (Architect HS6 ruling evt_bqyqcvn0ng1d)
>
> Grounded on `origin/main@1fe35740590cf4beb7009065afd6e15c01765603`; the four
> relevant runtime blobs are UNCHANGED from the RT-RESULT consumer base
> `c1945c6fbbd7b0d8422123904fc6f7138fc85df9`:
> `cranelift_backend/planning/static_transition/aggregates.rs d0c1f30a`,
> `.../static_transition/continuations.rs 69af3177`,
> `cranelift_backend/lowering/source.rs 4fe3b73a`,
> `cranelift_backend/lowering/core.rs 18a25d1f`.
>
> **What HS6 established.** The merged predecessor
> [[RT-ITREE-CHECKED-IH-RESULT-SUCCESSOR]] proves the exact captured continuation
> capability `K` is INHERITED to the exact recursively-exposed zero-argument
> checked self-resumption invocation, and that the fresh `R2` it would yield is
> destined for the ordinary Ret capture. It retained the IDENTITY half —
> `CheckedIhBinding { frame_origin, recursive_position }` — and discarded the
> ACCESS coordinate. `CheckedIhCapabilityInheritance` exposes owner, body, final
> frame, and recursive position, but NO locator into the `env` that
> `RuntimeExpr::Var(index)` actually reads. Semantic binding identity alone cannot
> select a runtime word (Research advisory `evt_1t84ypm156mqh`): the binder walker
> `derive_checked_ih_bindings` HAS the index at the trustworthy point and throws
> it away; lowering only indexes its concrete `env` after seeing the exact
> `Var(index)`. Capability identity/in-scope and immediate runtime availability
> are DISTINCT facts. Ken already separates them for continuation-input
> (`ContinuationEnvironmentClaim`'s closed `CurrentLexical` vs `EntryFrame` domain
> sum) — a CONCEPTUAL precedent for the domain-tagged locator, NOT a type to reuse
> mechanically (its arms carry a continuation-input-specific contract).
>
> **This node completes that operational witness.** It is HS6 predecessor
> INCOMPLETENESS, not "another successor after the successor," and NOT a fourth
> semantic axis. It is planner-only, behaviorally inert, and INDEPENDENTLY
> LANDABLE. RT-RESULT's atomic D3A+D3B consumer stays frozen/non-landable and the
> runtime consumer branch stays clean and held at `c1945c6fb`; after this lands,
> the consumer is explicitly RE-RELEASED against the extended base and consumes
> ONLY this locator through the existing accessor.

## Objective

Enrich the EXISTING forward binder-resolution walk so the planner retains, and
the planner validator re-derives, an immediate K-binding LOCATOR at the final
governed `CheckedIhBinding`: the exact typed runtime coordinate at which the
inherited continuation capability `K` is immediately available to the consumer
that reads it. The node exposes semantic `K` identity and immediate availability
SEPARATELY, adds no standalone search accessor, and changes no emitted behavior.
Its correctness is that the locator RE-DERIVES from the forward binder/layout
walk rather than freezing a number, carries its consumer/environment identity
with the index, and fails closed on any ambiguity.

## Authorized component shape (Architect evt_bqyqcvn0ng1d + advisory evt_1t84ypm156mqh)

1. **Enrich the existing forward walk — do not add a layer.** At
   `derive_checked_ih_bindings`, where `RuntimeExpr::Var(index)` indexes the exact
   threaded environment and resolves to
   `CheckedBinderProvenance::InductionHypothesis(binding)`, RETAIN an immediate
   K-binding locator instead of discarding `index`. The derivation is the SAME
   forward binder-resolution walk; no reverse search, no second binder catalog, no
   `CheckedCaseBinderLayout` re-run.
2. **Dedicated non-interchangeable type.** The locator is a dedicated checked-IH
   type — NOT a bare `u32`, and NOT a mechanical reuse of
   `ContinuationEnvironmentClaim`, `ProducerLocalLocator`, or
   `CheckedIhTransportInputDestination` (their contracts name different
   source/transport consumers). Its positive form names ALL of: the exact
   descendant `invocation_origin`; the exact `callee_origin` whose immediate
   environment is indexed; an explicit immediate-invocation-environment domain
   tag; and the environment index. The consumer identity and the environment
   identity TRAVEL WITH the number — a bare integer is never authority.
3. **Store on the existing step; derive conditionally.** Store the locator on the
   same `CheckedIhSelfResumptionStep` that already carries `invocation_origin`,
   `call_origin`, `callee_origin`, and `callee_binding`. Derive it ONLY when the
   zero-argument callee's exact binder resolution yields that step's
   `CheckedIhBinding`. The final capability view exposes the FINAL step's locator
   together with semantic `K` identity; it adds NO standalone search accessor.
4. **Planner validator re-derives with exact equality.** The planner validator
   re-runs the SAME forward binder/layout derivation and requires exact equality
   over the binding PLUS the full typed locator for EVERY retained step. Zero
   matches return no inheritance; duplicate bindings/locators, wrong
   consumer/environment, or ambiguity REJECT.
5. **Consumer stays out of this node.** D3A later consumes ONLY
   `checked_ih_continuation_inheritance_for_invocation` under its existing full
   source-call/owner/body/frame/recursive-position key, obtains the returned
   capability locator, and indexes only the named current invocation `env`,
   requiring the located binding to be `Value(Carried(_))` and failing closed on
   `StaticWorker`, `Specialized`, missing, wrong domain, or out-of-range. THAT
   consumer is the RE-RELEASED RT-RESULT D3A+D3B, NOT this node.
6. **Refines an existing axis; mints nothing.** `CheckedIhFreshResultDestination`
   is unchanged. The locator refines the inheritance suppression / at-most-once
   axis; it does NOT create a fourth semantic axis, a second binder catalog, a
   second ABI lane, a new continuation identity, or a new result path.

**Forbidden** (Architect): the lowerer reading the `Var` index, re-running
`CheckedCaseBinderLayout`, scanning `env`, borrowing the earlier frame-470
transport morphism, deriving from recursive position / residual / body shape, or
comparing runtime words. **If the exact locator CANNOT be obtained by enriching
this existing forward walk** — if it needs reverse search, alias guessing, a
second catalog, or lowering reconstruction — **STOP: do not add another metadata
layer; route to an explicit IR continuation/environment parameter** (the prior
Research stop condition fires; hold and surface to the Architect/Steward).

## Deliverables

- The dedicated checked-IH immediate K-binding locator type (item 2), retained by
  the enriched forward walk at `derive_checked_ih_bindings` (item 1), stored on
  `CheckedIhSelfResumptionStep` and surfaced on the final capability view (item
  3), on the planner side only.
- The planner validator's re-derivation and exact-equality check over binding +
  typed locator (item 4), and the discriminator controls below.
- No lowering edit, no `CheckedIhFreshResultDestination` change, no
  emitted-behavior change. D3A remains ABSENT on `main`.

## Acceptance criteria

- **AC-LOC-DERIVE** (retained at the trustworthy point, read/write separate, one
  per arrival) — on BOTH unchanged admitted programs, the enriched forward walk
  retains EXACTLY ONE immediate K-binding locator for each governed
  recursively-exposed zero-argument arrival, derived where `Var(index)` resolves
  to `InductionHypothesis(binding)` and ONLY when that resolution yields the
  step's `CheckedIhBinding`. Read and write are derived INDEPENDENTLY from their
  own planner facts, never cross-used.
- **AC-LOC-TYPE** (dedicated, identity travels with the number) — the locator is a
  dedicated checked-IH type carrying all four coordinates (descendant
  `invocation_origin`, `callee_origin`, immediate-invocation-environment domain
  tag, environment index); it is neither a bare `u32` nor an instance of
  `ContinuationEnvironmentClaim` / `ProducerLocalLocator` /
  `CheckedIhTransportInputDestination`. A control that strips the consumer or
  environment identity down to the bare index REDDENS.
- **AC-LOC-STORE** (stored on the existing step; no new accessor) — the locator is
  stored on the same `CheckedIhSelfResumptionStep`; the final capability view
  exposes the final step's locator together with semantic `K` identity; NO
  standalone search accessor is added. Semantic identity and immediate
  availability are exposed as SEPARATE facts.
- **AC-LOC-REDERIVE** (validator exact equality) — the planner validator re-runs
  the same forward binder/layout derivation and requires exact equality over the
  binding PLUS the full typed locator for every retained step. Zero matches return
  no inheritance.
- **AC-LOC-DOMAIN** (wrong domain rejects) — the SAME integer read from a
  different runtime domain REJECTS; the domain tag is load-bearing, not
  decorative.
- **AC-LOC-CONSUMER** (wrong callee/invocation rejects) — the RIGHT index paired
  with a different `callee_origin` / descendant `invocation_origin` REJECTS.
- **AC-LOC-BINDER-SHIFT** (re-derives, does not freeze) — inserting ONE
  intervening binder shifts the derived immediate index WHILE the semantic
  `CheckedIhBinding`, the transport/call identity, and `K` inheritance remain
  unchanged. This proves the locator re-derives rather than freezing a number.
- **AC-LOC-SUBST** (source-slot / final-residual reject) — substituting the source
  recursive-slot coordinate, OR the final-recursor residual coordinate, for the
  derived locator each INDEPENDENTLY REJECTS. (These are exactly the two consumer
  guesses that returned runtime failure at the HS6 stop.)
- **AC-LOC-MUTATIONS** (removal + duplication, read/write independent, byte-clean
  restore) — INDEPENDENTLY: remove the locator for one governed arrival; duplicate
  it; each must FAIL at the validator with read and write derived independently,
  and restore byte-identically.
- **AC-LOC-INERT** (behaviorally inert) — the planner-only extension changes no
  behavior: D3A is still ABSENT on current main, both read/write products stay at
  their existing fail-closed defaults, the emitted ABI / call / artifact surfaces
  are BYTE-IDENTICAL, `CheckedIhFreshResultDestination` is unchanged, and D1 plus
  the erasure blob are unchanged. It mints no fourth semantic axis, second binder
  catalog, second ABI lane, continuation identity, or result path.
- **AC-LOC-FORWARD-OR-STOP** (forward-derivability is the gate) — the COMPLETE
  locator for every governed arrival must be obtained by enriching the EXISTING
  forward binder/layout walk. If it cannot — if the derivation needs a reverse
  search, alias guessing, a second catalog, or lowering reconstruction — STOP: do
  NOT add another metadata layer or weaken the locator; HOLD and route to an
  explicit IR continuation/environment parameter (surface to Architect/Steward).
- **AC-NO-REGRESSION** — whole-suite green in CI; local targeted `-p ken-runtime`
  / `-p ken-cli` / `-p ken-verify` only, never `--workspace`.

## Reviewers

Architect (the extension enriches the existing forward binder-resolution walk at
`derive_checked_ih_bindings` to retain a dedicated non-interchangeable checked-IH
locator naming descendant invocation / callee / domain tag / index, stored on the
existing `CheckedIhSelfResumptionStep`, derived only on the zero-argument callee's
binder resolution to the step's `CheckedIhBinding`, re-derived with exact
validator equality; no standalone search accessor, no
`CheckedIhFreshResultDestination` change, no lowering consumer, no new
axis/catalog/ABI-lane/identity; the domain tag and consumer identity travel with
the index; the forbidden derivations are absent and AC-LOC-FORWARD-OR-STOP is the
gate) + runtime-qa (AC-LOC-DERIVE retains one locator per arrival read AND write
from their own facts; AC-LOC-REDERIVE validator equality; AC-LOC-DOMAIN /
AC-LOC-CONSUMER / AC-LOC-SUBST reject wrong domain, wrong callee/invocation, and
the source-slot/final-residual substitutions; AC-LOC-BINDER-SHIFT shifts only the
index while semantic identity holds; AC-LOC-MUTATIONS removal and duplication each
bite with byte-clean restore; AC-LOC-INERT holds — byte-identical
ABI/call/artifact, D3A absent, fresh-result destination and D1/erasure blob
intact). If the complete locator cannot be forward-derived, that is
AC-LOC-FORWARD-OR-STOP: HOLD and route to the Architect/Steward (explicit IR
parameter), do not weaken it.

## Capability tier

T1 — a soundness-bearing planner extension reviewed on the provenance argument
(a domain-tagged immediate-availability locator re-derived from the existing
forward binder walk, carrying consumer/environment identity with the index,
validated by exact re-derivation, distinguished from every existing
source/transport locator type), not a differential diff. Size M.

## Sequencing

Lane-1 (runtime, priority). This is the durable HS6 predecessor EXTENSION for
[[RT-RESULT-CONTINUATION-BINDING-PROVENANCE]], downstream of the merged
[[RT-ITREE-CHECKED-IH-RESULT-SUCCESSOR]] whose `CheckedIhSelfResumptionStep` and
capability view it enriches. It lands FIRST (independently landable, behaviorally
inert). Then the RT-RESULT D3 branch — held clean at `c1945c6fb` — is explicitly
RE-RELEASED against this extended base and builds the ATOMIC D3A+D3B consumer,
whose D3A obtains the located `K` through the existing
`checked_ih_continuation_inheritance_for_invocation` accessor and indexes only
the named current invocation `env`. The prior atomic D3A+D3B contract and its
three suppression axes are UNCHANGED. Runtime stays HELD on RT-RESULT (consumer
branch clean, no fallback) until this lands and the consumer is re-released; the
Architect reviews the exact candidate before any gate. Single Runtime lane object
at a time; PX8 stays blocked until the whole native carried-value program lands.
