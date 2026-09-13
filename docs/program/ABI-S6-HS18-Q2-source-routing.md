# ABI-S6 HS18 Q2 source-routing determination

Architect, 2026-09-13. This is a component-design ruling on checkpoint
`5d977ac7968dff3763d330690a9b4df530925d79`. It is not a release vote, not a
merge approval, and not a QA authorization. The checkpoint stays held.

## What this rules

Two things were routed together and they get separate answers.

1. The compiler-private verifier repair **meets the repair floor** set by the
   [protocol audit](ABI-S6-HS18-Q2-protocol-audit.md), with named residuals.
2. The reported nested-bracket classification is **not adopted**. The returned
   ledger contains a semantic contradiction that places the first wrong value
   potentially *inside* `Context4`, upstream of the transport chain the
   classification names. A repair written against that classification could
   correct a transport that is faithfully carrying an already-wrong value.

A bounded observation closing that contradiction is authorized and required
before any source repair is written.

## Verifier repair — meets the floor

Reviewed the audit delta against `01d2ccb117151c468cb8a7f06f6e3a5fd8667e33`:
7 paths, +2405/-700, as reported.

The floor items are discharged structurally, not by assertion:

- **All-path traversal is a real product-state walk.** `verify_all_paths_guarded`
  starts `Absent` at the finalized entry, reaches `Guarded` only across the
  exact guard branch's success ordinal and target, and resets to `Absent` on
  every other edge out of that branch. A dependent reached in any other state
  refuses.
- **The stale-loop-guard hole is closed at the state transition, not by a
  special case.** Re-executing the producer sets `Produced`, which discards an
  older `Guarded`. A later iteration therefore cannot inherit an earlier
  iteration's successful guard.
- **Vacuity refuses.** A dependent never observed on any path is an error, not
  a pass. This is the correct polarity and it is the one most often written the
  other way.
- **Layout positions are gone.** `ReachingMemoryState` over `StackMemoryLocation`
  replaces the textual initialization and clobber tests. Overlap is computed
  from slot, offset and width rather than from instruction order.
- **Unknown is conservative in the sound direction.** `resolve_stack_address`
  returns `None` rather than guessing; an unresolved store becomes
  `StackWriteEffect::Unknown`, which routes to `unknown_write_may_alias`, which
  returns `true` for any address form it does not recognize. `successor_edges`
  refuses an unsupported non-terminator instead of enumerating zero successors.
- **The emitter's join authority is gone.** `generated_function_result_contract`
  and `register_generated_constructor_join` are deleted outright, not bypassed,
  so finalized all-input predecessor replay is the sole join proof. This closes
  the declared-metadata-as-finished-proof objection from the path-proof ruling
  at its source rather than guarding it.

The four malformed forms I measured at `01d2ccb11` now have named controls
(`all_path_call_guards_reject_status_and_trap_bypasses`,
`actual_header_and_reaching_initializer_are_independent_obligations`).

### Residual 1 — the path-proof grid weakened its discrimination

`generated_result_path_proof_rejects_each_certificate_corruption` widened its
accepted refusal from two alternatives to four, so fifteen distinct mutations
now share one disjunction. The grid still pins that each mutation reaches
exactly one certified cut and that the exact control still emits an object, so
this is not vacuous. But a disjunction cannot distinguish *refused because the
corrupted dependency was missing* from *refused for an unrelated protocol
reason*, and a later verifier change that makes a guard fire earlier will keep
this grid green while silently retiring the dependency it was written to pin.

The stronger form is already in use twelve lines below it:
`generated_result_owner_certificate_rejects_each_finished_body_corruption`
pairs each mutation with its own expected refusal substring. Convert the
path-proof grid to the same pairing. This is a should-fix on the eventual
candidate, not a blocker on the checkpoint.

### Residual 2 — escape analysis does not see a container address passed to a call

`location_access` derives `escaped` only from store instructions: a value that
resolves to a container slot, stored to a destination that does not resolve.
Passing a container address as a *call argument* does not set `escaped`.

The immediate call is still covered, because `call_may_access_location` checks
each argument against the container set. What is not covered is a *later*
write performed through a pointer an earlier callee retained. For such a call,
`call_may_access_location` returns `false` and the reaching state stays `Exact`
where it should become `Unknown`.

The `UnitCallFrame` header stack address is passed to the direct callee, so
this shape is in the population the verifier is about, not outside it. I have
not measured whether any emitted callee retains such a pointer past return, so
this is a **named residual, not a proven defect** — but it is the one place in
the new memory analysis where the default is permissive rather than
conservative, and it should be either closed or explicitly argued unreachable
before this lands. Absence of a measurement is not a measurement.

## The Mapping classification is not adopted

### The contradiction

The returned ledger states both of the following about the same function:

- `funcid56` / `Context4` / match `origin733` observes tag `ResourceBodyOk`
  (`0x00000d9d0000003d`), arity 1, and selects it.
- `funcid56` / `Context4` / `origin594` constructs `Result::Ok` at `v1591`
  whose field 0 (`v1559`) is `ResourceBracketBodyAndReleaseError` `origin593`
  (`2497/83`).

`Context4` is the inner `withMapping` bracket's settlement. Per
`catalog/packages/Capability/System/Resource.ken.md`,
`ResourceBracketBodyAndReleaseError` carries a body error *and* a release
error. It is not derivable from a `ResourceBodyOk` body under any release
outcome. A settlement that observed `ResourceBodyOk` and published
`ResourceBracketBodyAndReleaseError` has already produced a wrong value before
anything is transported.

The observed effects agree with the source, not with the published value. The
native run emits `MappingAcquireFile`, `MappingWriteView`, `MappingReadView`,
`ResourceRelease`, `ResourceRelease` and then traps at the
`ResourceBodyResult` elimination before `FsReadFile`. Both releases occur and
neither reports an error. For the differential's expected semantics the inner
settlement must be `Result::Ok (ResourceBracketOk MkUnit)`, which is what
`mapping_bracket_body`'s `ResourceBracketOk value` arm consumes to reach
`body_ok_io`.

### Why this changes the repair site

The classification names the transport `Context4` `Ret` -> `Context1` ->
`ResponseOwner2` -> `Context3` as the defect. The transport evidence is sound
as far as it goes: `Context3` exists, loads `v11` from its payload, and fails
its first tag comparison. Nothing in that chain is disputed.

But the ledger's own disjunction — *"native consumes the inner one once and
correctly, then mis-feeds/omits the outer source consumption"* — is unclosed,
and the two arms select different repairs:

- **Mis-feed.** The outer consumption exists and executes; the value reaching
  it is wrong. Repair is in what feeds `Context3`.
- **Omit.** `file_body`'s `bind` continuation
  (`\outcome. mapping_bracket_body outcome`) is never applied, and `Context4`'s
  `Ret` publishes the bind's *left* operand — the inner bracket outcome of type
  `Result ResourceError (ResourceBracketResult Unit Unit)` — directly as
  `file_body`'s body result. Repair is in continuation-seat construction.

The contradiction above adds a **third** arm that neither names:

- **Wrong arm published inside `Context4`.** The settlement selects the
  `ResourceBodyOk` tag correctly but publishes a value constructed by a
  different arm. Under this arm the transport is faithful, `Context3` is
  correctly wired, and a transport or capture repair fixes nothing.

Three arms, one of which the classification does not consider, and the evidence
currently distinguishes none of them. That is why the classification is not yet
a finding.

### What must be measured, and nothing more

Observer-only, on the unchanged checkpoint. No source repair, no ABI, schema,
frame, owner key, tag, route, stack or bound change, no weakening of the
pre-object refusal.

1. **Resolve the contradiction first.** Determine whether `v1559`'s
   `2497/83` attribution is a decode of the value that actually reached the
   arena, or a static read of one arm's construction site. If it is the value:
   the first wrong value is inside `Context4` and the whole transport story is
   downstream. If it is a static read: withdraw it from the ledger, because it
   is currently load-bearing for the classification.
2. **Decide mis-feed against omit by presence, not by inference.** State
   whether the emitted program contains an executed elimination of
   `ResourceBracketResult` — `mapping_bracket_body`'s four-arm match. If that
   match is never emitted or never executed, the continuation is omitted and
   `Context3`'s input is the bind-left value by construction. This is a
   presence question with a yes/no answer and it closes the disjunction the
   handoff left open.
3. **Report the inner settlement's published identity against the expected
   `ResourceBracketOk`.** Name both identities. If they differ, the defect is
   at the settlement; if they agree, the defect is downstream and the transport
   classification survives.

Return those three answers. Do not return a repair proposal with them; the
repair site is not yet determined and naming one now would fix the site before
the measurement that chooses it.

## Scope

Inventory entry 18 closed with *"this is an entry 18 follow-up; no new
source-routing design is authorized."* That fence held correctly while the
localization was unproven. It is **narrowed, not lifted**: a bounded
observation into source routing and capture is authorized for the three
questions above and for nothing else. No source-routing redesign, no
continuation-seat representation change, and no capture-protocol change is
authorized by this ruling. The design authority for whichever site the
measurement selects returns here.

## Preserved

- Q1 `source.rs` stays blob `38ec787dac261f02d46463ee1e9fca555c76d694`,
  byte-identical to `30d35f625`.
- The honest px8f refusal stays: actual `3380/38` against demanded `4442/38`,
  with the five non-promoted possible cuts unchanged.
- Every grid and invariant standing at `01d2ccb11` stays standing.
- No QA request, no merge Decision, no candidate status.
