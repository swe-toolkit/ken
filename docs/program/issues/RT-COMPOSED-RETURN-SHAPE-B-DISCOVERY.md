---
id: RT-COMPOSED-RETURN-SHAPE-B-DISCOVERY
title: "DISCOVERY (binary viability verdict only): can the planner statically de-quotient the generated entry into a bounded one-member generated-function identity — so exact transport identity and its input-environment morphism are available by construction, Tail route selection precedes the relocated declared call, and that call's returned SSA value feeds the existing shared Ret-body parameter — WITHOUT a runtime discriminator, store, capture, recovery, or general composed-return result carrier? The shape-(b) prerequisite the Architect ruled must precede any build."
status: active
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

## Discovery verdict — refuted

**Verdict: REFUTED.** At exact working SHA
`68e25cd471bb115918b8b6e92791e8e683cbe2ce`, shape (b) cannot supply the
relocated Tail call's ordinary operand run at the checked predecessor of the
shared Ret body. The first unavoidable missing operands are the source
constructor's nonrecursive fields. They exist in the earlier selected-case
environment, but the active self-resumption backedge carries only the new
scrutinee word and route-control word. Static de-quotienting can retain the
exact call identity; it cannot make those runtime field values dominate a later
loop iteration.

This is a pure paper refutation. No production source, executable probe, Runtime
QA route, CI run, semantic candidate, or PX8 state was changed.

### Exact object and re-measured seam

The working tree is `80a10ae14e494b1ac6d7b7cc8bd9f7ec7e5a4068`.
All six authority blobs in the released frame remain exact:

- `aggregates.rs`
  `9eb2c118e227c3a7db2849e03046db02d93a48eb`;
- `source.rs` `88fcc401b0e078f78298a0998d09364b22e64a27`;
- `core.rs` `79ec94b749836a6e1747d6b6da0b572f919105cd`;
- `lowering/mod.rs`
  `d5837de3c5df3258e5db316fe901a1eb9cb8271a`;
- `rt_parity_native.rs`
  `b1adb83de3f97864d3b81da735eb759361bc962b`;
- `spec/40-runtime/42-evaluation.md`
  `69b9d6d267ba20235f42972865c2b20504531d62`.

The member is still present before quotienting at
`aggregates.rs:5902-5977`. Confluence construction still accumulates members at
`:6123-6257`, and publication still discards them at `:6323-6543`, specifically
`:6385` and `:6407`. The current producer remains at
`source.rs:4309-4374`. The carried-loop header and active backedge remain at
`core.rs:12225-12282`; the shared ordinary Ret predecessor is at
`:12433-12445`; the checked predecessor and stale-word jump are at
`:12634-12743`.

### Four-axis census

#### 1. Member and emitted-edge identity

This axis is representable without a sibling map. The pre-quotient row already
returns `(coordinate, member, retarget_caller, projection)`, and the member is
the transport's exact `source_call_identity`. A hypothetical split publication
could therefore replace
`CheckedIhGeneratedEntryAdmission::Governed(projection)` with a private
member-bound record containing the exact identity, transport, and projection.
The identity need not be recovered from a body, position, or projection.

That type split does not solve the discovery. The fixed products prove that
function identity alone is not already one-member: the read product has two
entry coordinates in one context; the write product has three coordinates in
two contexts and four total members; its W0/W1 coordinate has two distinct
members. These are asserted at `rt_parity_native.rs:959-1118`. A split must
therefore preserve a member on an exact emitted edge, not merely rename the
existing context function.

#### 2. Caller and declared-call closure

The static populations are bounded. `generated_entry_retarget_caller` at
`aggregates.rs:5215-5254` requires exactly one incoming retarget caller for an
enclosing specialization. Generated context definition declares its own
continuation-call targets from the exact emission owner, and
`declare_owned_in_func` includes identities transported into that owner. Thus,
even granting a private member-bound Tail edge, the target `FuncRef` can be
declared in the destination function without copying one from another
function.

This axis also does not supply operands. A declared call target proves which
function may be called; it says nothing about which runtime values fill its
frame.

#### 3. Transport-input environment and ABI — the refuting axis

The call has two independently derived operand runs:

1. its ordinary `Parameter` envelope; and
2. its continuation inputs.

The transport morphism covers only the second run. Its type at
`aggregates.rs:191-211` is a vector of continuation-input ordinal, source
coordinate, and either `LexicalEnvironment(index)` or `EntryFrame(slot)`.
Construction at `:4401-4463` iterates only
`source.continuation_inputs()`. It has no entry for an ordinary constructor
field or worker capture.

The existing call assembler makes the missing division explicit.
`call_checked_ih_transport_from_case_environment` at `core.rs:7642-7847`
constructs the ordinary run before consulting the morphism:

- `NonrecursiveConstructorField` reads the earlier selected-case environment at
  `recursive_count + source_position` (`:7692-7768`);
- `WorkerCapture` reads the compiler-only `StaticWorkerBinding` selected from
  that same environment (`:7713-7791`);
- only then do continuation inputs use the transport morphism at
  `:7801-7837`.

The checked Ret predecessor does not hold that selected-case environment. In
`lower_carried_computational_match_inner`, the ordinary selected arm creates
`case_env = IHs ++ children ++ frame_env` at `core.rs:12513-12532`. That value
is scoped to the selected arm. When its induction hypothesis resumes the same
carried eliminator, the active backedge at `:12225-12258` jumps to the header
with exactly two arguments: `scrutinee.word` and the route-control word. The
header at `:12260-12282` has exactly those two parameters. The later checked
fallback begins after the ordinary case loop at `:12634` and therefore has
neither the prior `case_env` nor a block parameter for any of its children.

The generated context's entry ABI is not a substitute. It carries raw-worker
arguments, raw-worker captures, and continuation-input captures. The target's
ordinary envelope instead names nonrecursive fields of a producer `Construct`
plus selected-worker captures. The code states this separation at
`continuations.rs:1722-1737`: the ordinary envelope is a role projection, not a
worker-body environment map; the continuation descriptor and worker
`arity + captures` contracts are distinct, and no slot-to-lexical-position
relation exists between them. Treating an entry argument as a source
constructor field would therefore be occurrence-blind authority synthesis,
even when two runtime words happened to agree.

The exact Tail family makes the difference semantic as well as positional.
`spec/30-surface/36-effects.md:489-491` (blob
`d43477191c66419f75cda394ce15c7eddd33ef4a`) defines `Vis` with nonrecursive field
`e : E.Op` and recursive worker `E.Resp e → ITree E R`. The generated context's
raw worker argument is the response; the continuation ordinary envelope's
nonrecursive source field is the operation. They are distinct typed values, not
two spellings of one slot. The exact source products make the absence concrete:
`rt_parity_native.rs:158-175` constructs `readAt` and gives `bind` a
response-only `\outcome` continuation; `:222-236` does the same for `writeAt`.
Neither continuation captures the prior operation value. The active backedge
carries the later answer word and cannot supply that earlier operation field.

A private type for the missing piece would have to look like a total
`CheckedIhTailOrdinaryOperandPlan`, with one exact destination for every
`ContinuationOrdinaryEnvelopeRole`. The current plan cannot inhabit its
`NonrecursiveConstructorField` rows at the checked predecessor. Identity
splitting changes no SSA dominance and supplies no value for those rows.

Every way to make the type inhabitable is prohibited:

- add the prior children to the active header's block-parameter run — a new
  runtime lane;
- retain the earlier selected-case environment through general composed return
  — the closed Produced-transfer/D3 design;
- add fields to the generated context frame — runtime capture/ABI transport;
- store the fields and reload them — storage;
- recover or project them from `scrutinee.word` — runtime recovery from the
  stale topology carrier; or
- emit the call while the selected-case environment is still live — before
  Tail route selection, which is shape (a), not shape (b).

This is an operand absence, not merely a missing validator. A new planner
relation could name the old occurrences, but naming an occurrence does not
recreate its SSA value on a later loop iteration.

#### 4. Exact producer-result-to-Ret SSA binding

If a lawful relocated call result existed, the final binding itself would be
local and exact: replace only the checked predecessor's current
`jump(return_body, &[scrutinee.word])` at `core.rs:12743` with the call's
returned SSA word. The ordinary predecessor would continue to feed its projected
child at `:12441`, and the single block parameter at `:12444` would still own the
only Ret-body lowering.

That local edge cannot rescue axis 3. There is no lawful relocated call result
to bind because the call frame cannot be assembled after self-resumption.
Passing a different word to `return_body` without first producing it would only
rename the stale-value defect.

### Decisive obstruction

The first unavoidable missing environment operand is the ordinary
`NonrecursiveConstructorField` run from the source-specific selected case. It
is available before self-resumption and absent after the two-argument active
backedge. A one-member static identity closes source authority but not runtime
value availability. Closing both requires one of the expressly forbidden
runtime transport forms above.

Therefore **SOURCE-INDEXED PRODUCER/RET COLOCATION is not achievable by the
released shape (b)**. The semantic goal in runtime spec section 6.2 remains
open, but this discovery authorizes no fallback to Produced-transfer, D3,
Direct-only salvage, recovery, storage, tags, carriers, a second Ret body, or
HS15.
