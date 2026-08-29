---
id: RT-COMPOSED-RETURN-SHAPE-B-DISCOVERY
title: "DISCOVERY (binary viability verdict only): can the planner statically de-quotient the generated entry into a bounded one-member generated-function identity — so exact transport identity and its input-environment morphism are available by construction, Tail route selection precedes the relocated declared call, and that call's returned SSA value feeds the existing shared Ret-body parameter — WITHOUT a runtime discriminator, store, capture, recovery, or general composed-return result carrier? The shape-(b) prerequisite the Architect ruled must precede any build."
status: ready
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Operator decision 2026-08-29 (FUND THE SEMANTIC REPAIR), then Architect ruling evt_79qhv6m9dj5j4 (binds exact main ba1c92214, tree f606a7483): fund a SCOPED shape-(b) DISCOVERY first — do NOT frame a direct build, because current code deliberately erases the authority and environment facts a relocated call needs, so a direct build is under-specified. Follows the shape-(a) build hard stop (RT-COMPOSED-RETURN-PRODUCER-ORDER-BUILD, held draft; Architect evt_6bq9q76rmzm90) — shape (a) is a viable authority join but insufficient as the Tail semantic repair because the private proof can only refuse earlier and cannot alter the emitted returned word. Steward-filed per COORDINATION section 2."
---

> # RELEASED — lane 1, the new runtime objective. Discovery-only. `ready`.
>
> The operator FUNDED the composed-return semantic repair; the Architect ruled
> (evt_79qhv6m9dj5j4) that the first increment is a scoped, decisive shape-(b)
> discovery, NOT a build. The held shape-(a) build frame
> (`RT-COMPOSED-RETURN-PRODUCER-ORDER-BUILD`) stays `draft` and untouched. This
> is a T1 paper viability verdict; it lights no PX8 witness by itself.

## What shape (b) is, and its load-bearing obligation (Architect)

Shape (b) is the mechanism FAMILY: statically **de-quotient** the generated
entry so one emitted function/edge retains one exact `ContinuationCallIdentity`,
then **relocate** Tail's declared-call producer to a boundary AFTER Tail route
selection. "Producer-result-to-Tail-Ret binding" is its semantic OBLIGATION: the
exact SSA result of that relocated call — not the pre-call seed, not the later
constructed/merged word — must be the sole input to the existing shared Ret body.
These are not competing mechanisms; the binding is shape (b)'s load-bearing
postcondition.

"Post-validation" means after governed validation AND after Tail-route selection.
The existing producer is already after governed validation; merely moving it
inside `source_call_state` repeats shape (a) and stays semantically inert.

**A long-lived `Produced` value threaded through general composed return is NOT
authorized** — it re-enters the closed Produced-transfer / D3 route.

## The binary question this discovery answers

Can the planner create a bounded static ONE-MEMBER generated-function identity
where:

- (a) exact transport identity and its input-environment morphism are available
  BY CONSTRUCTION,
- (b) Tail route selection PRECEDES the relocated call, and
- (c) the returned SSA value can feed the existing Ret-body parameter WITHOUT a
  runtime discriminator, store, capture, recovery, or general composed-return
  result carrier?

## Deliverable — a binary verdict with a FOUR-AXIS census

A **viable** verdict gives the exact type/function split and a CLOSED census on
four axes (Architect):

1. **member/function identity** — the bounded one-member generated-function
   identity and its exact type/function split.
2. **caller closure** — the closed set of callers and emitted call edges.
3. **transport-input environment/ABI** — the environment/ABI reconciliation: the
   transport identity and its input-environment morphism available by
   construction.
4. **exact producer-to-Ret SSA binding** — that the relocated call's returned
   SSA value is exactly the sole argument to the one shared Ret-body parameter.

A **refutation** names the FIRST unavoidable runtime choice or missing
environment operand. **"Probably viable" WITHOUT the environment proof is NOT
decisive** — it is a hard stop, not a verdict.

## Load-bearing property the verdict must establish is achievable

**SOURCE-INDEXED PRODUCER/RET COLOCATION.** For every governed Tail arrival,
exactly one statically selected source member owns exactly one relocated declared
call, and that call's returned SSA value is exactly the sole argument delivered to
the one shared Ret-body parameter. The initial seed, constructed carrier, merge
output, and sibling result cannot satisfy the Tail edge.

## Exact seam (Architect coordinates — RE-MEASURE at the working SHA)

Authority blobs at the ruling's base: `aggregates.rs` `9eb2c118`, `source.rs`
`88fcc401`, `core.rs` `79ec94b7`, `lowering/mod.rs` `d5837de3`, parity tests
`b1adb83d`, spec 42 `69b9d6d2`. The relevant-path diff from the ruling base
`ba1c92214` to current main is empty; still re-measure — coordinates decay.

- **The pre-quotient member EXISTS.** `aggregates.rs:5902-5977`
  (`checked_ih_generated_entry_row`) reopens via `transport.source_call_identity`
  at `:5932`, derives the route, and returns member identity SEPARATELY from
  common projection at `:5974`.
- **The quotient is explicit.** `CheckedIhGeneratedEntryCoordinate`
  (`:414-427`) omits source identity; `CheckedIhGeneratedEntryProjection`
  (`:430-437`) omits identity, transport, and ancestry;
  `CheckedIhGeneratedEntryConfluence` (`:439-445`) keeps identities only in
  `members`; `build_checked_ih_generated_entry_confluences` (`:6123-6257`) merges
  equal projections and accumulates members at `:6211-6237`.
- **Publication ERASES the member.** `build_checked_ih_generated_entry_accesses`
  (`:6323-6543`) installs only `confluence.projection.clone()` at `:6385` and
  publishes `Governed(projection)` at `:6407`; `CheckedIhGeneratedEntryAccess`
  (`:467-477`) makes certificate members and the graph caller unrepresentable.
  **This is the de-quotienting boundary to REPLACE or SPLIT; a sibling identity
  map is FORBIDDEN.**
- **The current early producer** is `source.rs:4309-4374`: transport selection at
  `:4309`, declared transport call at `:4369-4371`, `RoutedAnswer::checked(returned)`
  at `:4373`. Governed arrival validation is already earlier at `:3976-4241`.
- **The result loses top-level authority in the general protocol.**
  `source.rs:1250-1264` destructures `RoutedAnswer`; `:1647` (`ConstructArgument`)
  pushes the operand as constructor-field material; `:1691-1710` constructs the
  ordinary carrier and replaces the top-level value with
  `RoutedAnswer::direct(constructed)`. Retaining the earlier result across that
  seam is the CLOSED general-transfer design — do not.
- **The Tail consumer** is `core.rs:12635-12743`: it selects the checked fallback
  at `:12635-12645` yet still jumps to shared `return_body` with `scrutinee.word`
  at `:12743`. The ordinary Ret arm feeds its projected child into that same block
  at `:12433-12445`. **Shape (b)'s semantic landing point is the checked
  PREDECESSOR of this shared block:** emit the statically member-bound Tail call
  there and pass its returned SSA value as the block argument. **Do NOT lower a
  second Ret body.**
- **The active self-resumption header** at `core.rs:12233-12258` also carries
  `scrutinee.word`; that is topology/control evidence, NOT fresh-result authority.
  The discovery must prove relocation can bypass that stale value at the checked
  Ret predecessor WITHOUT adding a runtime lane.

## Acceptance criteria

- **AC-VERDICT-DECISIVE** — the deliverable returns exactly one of: VIABLE with
  the four-axis closed census (member/function identity, caller closure,
  transport-input environment/ABI, exact producer-to-Ret SSA binding) and the
  exact type/function split; or REFUTED naming the first unavoidable runtime
  choice or missing environment operand. "Probably viable" / "needs more work" /
  a viable claim missing the environment proof is NOT a verdict — it is a HARD
  STOP to the Architect.
- **AC-PROHIBITIONS-HELD** — a viable sketch introduces no runtime
  discriminator, store, capture, recovery, general composed-return result
  carrier, long-lived `Produced` value, sibling identity map, or new runtime
  lane; and it does not retain the earlier result across the
  `source.rs:1250-1710` general-transfer seam. A sketch needing any of these has
  left shape (b) and refutes.
- **AC-DISCOVERY-ONLY** — no production edit, Runtime QA gate, semantic
  candidate, CI run, or PX8 closure. The deliverable is the verdict artifact.

## Prohibitions

No revival of `RT-COMPOSED-RETURN-PRODUCED-TRANSFER`, the D3 chain, Direct-only
salvage, recovery, storage, runtime tags, result carriers, or HS15. No sibling
identity map at the de-quotienting boundary. No second Ret body.

## The build the viable verdict unlocks (recorded, NOT this node's ACs)

If viable, a LATER build is ATOMIC from producer relocation through exact Ret
binding and both product flips. The Architect specified its causality controls in
advance so the discovery anticipates them (they gate the BUILD, not this
discovery):

- Suppress only the relocated call while route selection remains: both exact
  products revert to `ResourceBodyResult` `PatternMatchFailure`, never still
  `InvalidOffset`.
- Keep the call but replace only the Ret-predecessor argument with old
  `scrutinee.word`: both revert likewise — this separates producer existence from
  binding.
- Swap a member with a distinct sibling in a real multi-member class while
  leaving projection equal: static authority rejects before emission, or a
  sentinel product fails. No first/positional member.
- Collapse two split members back to one generated function: the one-member
  closure gate reds (distinct from the binding control).
- Duplicate the relocated call or feed its result twice: exact emission/effect
  census reds; resumption stays single-shot.
- Direct and the ordinary Ret predecessor stay behaviorally unchanged; no
  Direct-only candidate lands.
- Positive: interpreter-agreeing `InvalidOffset` for the exact fs-read-at-offset
  and fs-write-at-offset sources; base negative is the measured
  `PatternMatchFailure`. A failing configuration that can ALSO produce
  `InvalidOffset` for another reason is not a valid control.

## Reviewers

Architect — the verdict's soundness: whether a VIABLE sketch's four-axis census
is genuinely closed and constructive (identity available by construction, route
selection precedes the call, SSA result binds the Ret parameter, no prohibited
form), or a REFUTATION's first unavoidable runtime choice / missing operand is
exact and not a missed route. Runtime QA ONLY if the verdict carries an
executable probe (then: the probe violates no prohibition and its measurement is
reproducible); a pure paper verdict has no QA diff to gate. A design fork inside
the verdict HARD-STOPS to the Architect.

## Capability tier

T1 — a soundness-bearing viability judgment on the de-quotienting and
result-binding argument, reviewed on the argument, not a diff. Size M.

## Sequencing

Lane 1 (runtime), the new objective. Released 2026-08-29 per the operator's
funding decision and the Architect's discovery-first ruling. No `depends_on` —
the seam is at current main and the ruling is deductive. If viable, the Steward
frames the atomic build from the verdict; if refuted, the Steward returns it to
the operator with no fallback to the closed axes. The shape-(a) build frame stays
`draft`, untouched. HS15 stays unspent — it belonged to the closed endpoint
series.
