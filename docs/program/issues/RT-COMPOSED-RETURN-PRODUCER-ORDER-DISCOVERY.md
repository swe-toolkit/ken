---
id: RT-COMPOSED-RETURN-PRODUCER-ORDER-DISCOVERY
title: "DISCOVERY (viability verdict only): can source-specific validation authority be established BEFORE the existing producer — shape (a) — so the tail-resumptive composed-return fresh-R2 transfer becomes realizable without a store, captured continuation, runtime tag, or recovery. Scoped to shape (a); a refutation returns the shape (b) de-quotienting cost decision to the operator."
status: ready
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Operator decision 2026-08-29, concurring with the Steward recommendation after Research advisory evt_774v5fjnxcfcw and Architect disposition evt_5te99temrdcty. RELEASED 2026-08-29 after the native-carried-value campaign front was determined DRAINED on current main 863bf0fbf by the Architect (evt_1xndnw1dp1r6v) and returned by the runtime-leader (evt_6bmd84zd6yzg2) — the sequencing precondition the operator set (queue after the campaign) is met. The composed-return wall (closed node RT-COMPOSED-RETURN-PRODUCED-TRANSFER, D0b=NO) is a partial-order contradiction on the Tail lowering route. Research confirmed Q2 POSITIVE: the shape is reached by well-formed Ken SOURCE (two complete SourceFormat::Ken programs — fs-read-at-offset and fs-write-at-offset — through the real build_native_program pipeline; the 48 Tail / 3 Direct arrivals are compiler arrivals within them, not fixtures), so the wall is NOT moot. Research Q1: no surveyed family (Interaction Trees, Koka, CPS/SSA) preserves Ken's current order (emit R2 -> collapse -> quotient away source identity -> validate later) and still delivers R2 without a store/capture/tag/recovery; the Architect's two shapes are the only known families and there is no local patch. Operator: fund shape (a); the language has no users so a wrong-value-vs-refuse failure-mode check is unnecessary."
---

> # RELEASED — OPERATOR-FUNDED DISCOVERY, lane 1. Deliverable is a VIABILITY
> # VERDICT, not a landed mechanism.
>
> The native-carried-value campaign front is DRAINED (Architect determination
> `evt_1xndnw1dp1r6v`, grounded on `origin/main` `863bf0fbf`, tree
> `67089b7b22d36ca8ac04b21ce88856e23e6ada32`; runtime-leader return
> `evt_6bmd84zd6yzg2`). This node is the first eligible semantic object on lane
> 1. `depends_on` is empty: the queue-after-the-campaign ordering was ring
> contention, and the ring is now free.

## The question, and why it is real work and not fixture-cleanup

The composed-return wall is closed (`RT-COMPOSED-RETURN-PRODUCED-TRANSFER`,
D0b=NO): on the Tail lowering route the fresh result `R2` cannot be delivered
into the handler's `Ret` capture because the producer has already emitted before
the validated `Selected` owner exists — a partial-order contradiction, not a
missing carrier. Research settled the two questions that gate funding:

- **Reachable from Ken source (Q2 POSITIVE).** The D0b fixture compiles two
  complete `SourceFormat::Ken` programs (fs-read-at-offset, fs-write-at-offset)
  through the real parser/elaborator/native-build pipeline; "Tail" is derived by
  the compiler from checked source, not assigned by the test. One source witness
  (semantic core `bind t (\x. Ret (f x))` under resource bracketing) suffices to
  refute source-unreachable. This is bread-and-butter effectful I/O, so the gap
  sits on the ABI/native-completeness critical path.
- **No cheap or local fix (Q1).** Interaction Trees, Koka, and CPS/SSA all
  establish continuation/handler identity BEFORE producing the response, or make
  identity the selected function/block. None preserve Ken's current order without
  a store, captured continuation, runtime tag, or recovery — all forbidden here.
  The Architect's two shapes are the only known families; there is no third.

## Grounding — current main, re-measured at release (Architect coordinates)

Binds `origin/main` `863bf0fbf38df33dd6d0f2a9582f7df1055da5c0`, tree
`67089b7b22d36ca8ac04b21ce88856e23e6ada32`. The two producer-order authorities
are byte-identical to the accepted D0b base `6c2b6a18f`:

- **Producer** — `crates/ken-runtime/src/cranelift_backend/lowering/source.rs`
  (blob `88fcc401b0e078f78298a0998d09364b22e64a27`), `:4369-4374`:
  `call_checked_ih_transport_from_case_environment` produces
  `RoutedAnswer::checked(returned)` and returns it as the source-machine value,
  BEFORE the later Tail selection.
- **Route authority** —
  `crates/ken-runtime/src/cranelift_backend/planning/static_transition/aggregates.rs`
  (blob `9eb2c118e227c3a7db2849e03046db02d93a48eb`),
  `fn checked_ih_fresh_result_route` at `:5461-5582`: selects the exact
  Direct/Tail route from planner facts.
- The later landings do NOT invert this order or supply fresh `R2`.
  `09faf638e` (checked-successor) single-lowers the Ret successor behind a shared
  join (`core.rs:12668-12743` feeds the shared Ret block with `scrutinee.word`
  unchanged); `634d5faf0` (pairing-leg controls) only strengthens route-observation
  evidence. Neither moves the source producer, de-quotients source identity, or
  authorizes an earlier Tail mint (Architect `evt_1xndnw1dp1r6v`).
- D0b partition, still structurally decisive: Direct 3/3 because its producer
  remains AFTER the authority point; Tail 0/48, with 47/48 reaching `ResumeOuter`
  only after the exact producer has already emitted and the last row also failing
  the pointwise partition. Re-measure the exact line coordinates at the working
  SHA before acting — they decay.

## Scope — shape (a) ONLY

Prove or refute whether **source-specific validation authority can be established
before the existing producer**, so the mint occurs only after it. Ken already
derives a fresh route per exact pre-quotient inheritance
(`checked_ih_fresh_result_route`, above) BEFORE folding equal projections into
the generated-entry confluence class, so source-specific route authority exists
pre-quotient. **What is unproven — and is exactly this node's question — is
whether that authority can be exposed AT the producer under the current
confluence and caller-closure guarantees without becoming a second catalog or a
positional guess.**

Shape (b) (relocate the producer to a post-validation boundary via a static
de-quotienting / function-identity design) is **NOT in scope.** A shape-(a)
refutation is a clean, valuable outcome: it returns the shape-(b) cost decision
(static de-quotienting plus closure/code-size/environment reconciliation) to the
operator as a distinct, larger call.

## Deliverable — a VIABILITY VERDICT, not a landed mechanism

A discovery verdict: **shape (a) is at least probably viable** (with the
constructive sketch that makes it so), or **shape (a) is refuted** (with the
exact obstruction). It is not a build. If probably-viable, a follow-on build WP
is framed from the verdict; if refuted, the operator decides on shape (b).

## Acceptance criteria

- **AC-VERDICT-DECISIVE** — the deliverable returns one of exactly two verdicts:
  probably-viable WITH a constructive sketch, or refuted WITH the exact
  obstruction. "Unknown / needs more work" is not a verdict — it is a hard stop
  to the Architect, not a landing.
- **AC-GROUNDED-CURRENT-MAIN** — the sketch or the obstruction is grounded in the
  current-main executable coordinates (producer `source.rs:4369-4374`; route
  authority `aggregates.rs:5461-5582`), re-measured at the working SHA, not in
  prose and not solely in the closed D0b record.
- **AC-PROHIBITIONS-HELD** — a probably-viable sketch introduces no store,
  captured continuation, runtime tag/discriminator, or recovery, and no backward
  token move across the emit-then-validate order. A sketch that needs any of
  these IS a refutation of shape (a) and is reported as one, not as a solution.
- **AC-SCOPE-SHAPE-A** — shape (b) is not explored or begun. A shape-(a)
  refutation names the shape-(b) cost decision and returns it to the operator as
  a distinct call.

## Prohibitions (from the wall — a candidate needing any of these has refuted shape (a))

No store, captured continuation, runtime tag/discriminator, or recovery; no
backward token move across the emit-then-validate order; SSA permits only a
forward producer-to-resumption edge. Manufacturing the missing edge by any of
these is the forbidden route, not a solution.

## Reviewers

Architect — the verdict's soundness: whether a probably-viable sketch is
genuinely constructive and prohibition-clean, or whether a refutation's
obstruction is exact and not a missed route. Runtime QA only if the viable
verdict carries an executable probe/spike (then: the probe violates no
prohibition and its measurement is reproducible); a pure paper verdict has no QA
diff to gate. A design fork inside the verdict HARD-STOPS to the Architect.

## Capability tier

T1 — a design-discovery judgment on a soundness-bearing ordering, reviewed on the
argument, not a diff. Size M.

## Sequencing

Lane 1 (runtime). RELEASED 2026-08-29: the campaign front is drained, which is the
operator's stated sequencing precondition. No `depends_on`. **No Research advisory
is owed on release — `evt_774v5fjnxcfcw` discharges it; HS15 stays unspent and is
NOT to be spent on this axis.** Keep `RT-COMPOSED-RETURN-PRODUCED-TRANSFER` and the
D3 chain closed; no Direct-only salvage.

## Inputs (decay — re-measure at the working SHA)

Research advisory `evt_774v5fjnxcfcw` (source-coordinate ledger SHA-256
`9c4f7d45f7353ec2f9bcd28977ec88d62577cd758b902164e23bc64474900bb3`); Architect
disposition `evt_5te99temrdcty` and drain determination `evt_1xndnw1dp1r6v`
(current main `863bf0fbf`, tree `67089b7b22d36ca8ac04b21ce88856e23e6ada32`); closed
node `RT-COMPOSED-RETURN-PRODUCED-TRANSFER` (D0b report SHA-256 `fdd5e859...`); spec
§6.2.

## Discovery verdict — probably viable

**Verdict: shape (a) is probably viable.** At working SHA
`751e35e03d475fd15dfd9234d47695f9ef1884fd`, source-specific planner
membership and the sanitized governed-arrival projection can be joined before
the existing producer. The join needs no second catalog, source-order guess,
runtime discriminator, store, captured continuation, recovery, or backward
token movement.

This is a constructive paper verdict, not an implementation or executable
probe. It does not trigger a Runtime QA gate.

### Current executable order

The release coordinates remain exact at the working SHA:

- `lowering/source.rs` is blob
  `88fcc401b0e078f78298a0998d09364b22e64a27`.
  `validate_checked_ih_generated_entry_governed_arrival` starts at line 3976.
  It checks the exact invocation/call/callee triple against
  `projection.fresh_result_route()` at lines 4102–4113. Its caller,
  `source_call_state`, starts at line 4119 and performs that total admission
  and validation before callee dispatch.
- The same function selects the existing exact environment `transport` at line
  4309. Only afterwards does it call
  `call_checked_ih_transport_from_case_environment` at line 4369 and mint
  `RoutedAnswer::checked(returned)` at line 4373.
- `planning/static_transition/aggregates.rs` is blob
  `9eb2c118e227c3a7db2849e03046db02d93a48eb`.
  `checked_ih_fresh_result_route` remains at line 5461. It derives one exact
  Direct or Tail route from the governed invocation, immediate-K locator,
  binding, typed transport partition, active frame, Ret binder, and forward
  destination.

The required order therefore already has an unused forward interval:

1. validate the governed arrival and its exact route;
2. select the exact source transport;
3. emit the transport call;
4. mint the checked routed answer.

The missing operation is an exact join between steps 2 and 3, not a movement
of either endpoint.

### Constructive shape

A follow-on build can make the interval authoritative as follows.

1. Make the existing governed-arrival validator return an opaque, compiler-only
   proof instead of returning only `CheckedIhBinding` and discarding the fresh
   route. The proof carries the already-validated access identity, exact
   invocation/call/callee key, binding, and common projection. It is a Rust
   lowering value and emits no runtime field or SSA value.
2. After line 4309 has selected the existing `CheckedIhEnvironmentTransport`,
   but before line 4369 emits it, join that proof to the transport through the
   planner's existing `checked_ih_generated_entry_confluences` relation. Build
   the exact `CheckedIhGeneratedEntryCoordinate` from the access context,
   enclosing specialization, worker body, binding, and governed call key; use
   exact map lookup, require projection equality, and require
   `confluence.members.contains(transport.source_call_identity())`.
3. Return a private, opaque producer-authority proof only from that exact join.
   Require that proof at the line-4373 fresh-result mint. A governed admission
   paired with the wrong transport, a non-governed admission reaching this
   producer, an absent class, or a non-member transport refuses before call
   emission. The producer-to-mint flow remains forward.

This does not recover source identity from the generated-entry quotient. The
exact transport already carries `source_call_identity`; the join only verifies
that identity is a member of the exact pre-existing certificate class. The
identity is neither copied into `CheckedIhGeneratedEntryAccess` nor emitted at
runtime.

### Why this is one authority, not a second catalog

The planner already retains both sides of the join:

- `CheckedIhGeneratedEntryConfluence` stores the source-specific member set and
  common projection at lines 439–443;
- `CheckedIhGeneratedEntryAccess` publishes the sanitized call-key projection
  at lines 467–475;
- `build_checked_ih_generated_entry_confluences` at line 6123 refuses
  projection disagreement among colliding source identities;
- plan validation proves source-specific governed pairs equal certificate
  members at lines 6656–6659, and certificate keys equal installed sanitized
  keys at lines 6681–6684.

The new operation is therefore an exact accessor over one already-validated
relation. It neither copies the member set into the installed access nor scans
transports and chooses a first match. Multi-member confluence is not ambiguous:
each member is allowed only because every member has the same typed projection,
and the producer's already-selected transport supplies the exact member tested.

### Prohibition and scope audit

- No store or captured continuation: the proof lives only in the compiler's
  current Rust call stack between validation and emission.
- No runtime tag or discriminator: neither the route nor source identity enters
  the generated ABI or carrier word.
- No recovery: every absent or mismatched exact relation refuses before the
  producer.
- No backward token move: validation and membership join precede the existing
  call and mint in source order.
- No second catalog or positional guess: the join uses the existing exact
  confluence key and set membership, never iteration order or numeric
  proximity.
- Shape (b), Produced-transfer, the D3 chain, Direct-only salvage, and HS15
  remain untouched. A build WP for the opaque proof and exact join is a
  separate successor; this discovery does not begin it.
