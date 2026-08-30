---
id: RT-HOST-RESPONSE-APPLICATION-RET-BUILD
title: "Production recut on the checked Host-Vis route: prove the causal chain Host response (H e) -> the existing sole apply-k-resp application -> that application's result -> exact Ret, as TWO typed compiler-only edges (HostResponseToApplication + ApplicationResultToRet) that never let one discharge the other. Predeclare one private application-result continuation at the existing Host-Vis Let/application seat, lower the existing body exactly once, deliver every physical successful application return exactly once to that continuation, then consume the existing Tail proof and lower/schedule the exact Ret destination once. Read = the direct Computational head (D1, independently mergeable); write = Active as a distinct exhaustive arm (D2, revalidate the same application/frame/Ret-body/binder relation, deliver to the same logical continuation). Do NOT add or reapply k; do NOT forward the environment-transport word as resp."
status: ready
owner: runtime
size: L
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Architect ruling evt_3jn61sw0w23hy (thr_6vmcjhwxc78sk), 2026-08-30, on exact base 0be25235b188bc67b3f9209d1ff0b6f8fa063258, tree 769c24708fb2052c3d6e719a8adc135423c28192. This ruling BLOCKED the released production recut RT-COMPOSED-RETURN-PRODUCER-SINK-COLOCATION-BUILD as framed and WITHDREW the true-emitter premise it rested on: the prior ruling evt_6vpyaje9a7n85 incorrectly promoted physical call identity (call_checked_ih_environment_transport, an environment-materialization call) to semantic result identity. The prototype digest 1bf9907d604cf7248d007b83c311cf2b56e4032bebd2f770500c47746ead2122 is a valid refutation; the observed Result PatternMatchFailure is exactly the forbidden wrong-word control. The Architect authorizes this fresh recut: Host response -> existing sole application -> application result -> exact Ret, via the two causal edges below. Steward-recut per COORDINATION section 2. Design authority for this production recut only; NO candidate/merge authority before fresh Runtime QA, Architect review, and publisher CI. @steward owed the withdrawal/reframe/release; runtime was parked until this release."
---

> # READY — PRODUCTION recut. Released to the runtime ring (lane 1). Runtime was
> # parked at base `0be25235b`; this IS the release.
>
> This node SUPERSEDES `RT-COMPOSED-RETURN-PRODUCER-SINK-COLOCATION-BUILD`
> (now closed): its true-emitter premise is SPENT.
> `call_checked_ih_environment_transport` is an
> environment-materialization call — its returned SSA is NOT the semantic Host
> response and NOT the result of the one semantic `apply k resp`. **Stop
> repairing environment-transport emitters.** Directly forwarding that word as
> `resp` is the forbidden wrong-word control and reproduces the exact `Result`
> `PatternMatchFailure`.
>
> The shared-predicate answer across the whole chain is YES: the same defect keeps
> recurring — a compiler phase/result with the right *static* identity is used to
> name a different *semantic* value role. The recut fixes that by identifying the
> response and the application from structure already present, adding no tag,
> heuristic, count, or lookup.

## Exact base and coordinates (Architect `evt_3jn61sw0w23hy`)

Binding base `0be25235b188bc67b3f9209d1ff0b6f8fa063258`, tree
`769c24708fb2052c3d6e719a8adc135423c28192`. Relevant blobs at that tree (the
ruling's own identities — **re-measure the exact seams at this base before
touching anything**; these cite functions/regions, not frozen line numbers):

- `crates/ken-elaborator/src/erasure.rs` `8532ced2ce2b4beb0cbe7ee8960b73777f4e540c`
- `.../cranelift_backend/lowering/effects.rs` `efa1c5234766eb61bbba5050e152fee026e7f19c`
- `.../cranelift_backend/lowering/core.rs` `eea98dc6ddb0ae2f7656b16fed7ee461b24de0a1`
- `.../cranelift_backend/lowering/source.rs` `c39f82e7854f626244b4398ba9941ae38b25485e`
- `.../planning/static_transition/continuations.rs` `2f7700d15dd37bb834533ea879425143e2221e90`
- `.../planning/static_transition/aggregates.rs` `e7bc36287fd0557b0670a6e1ab20171be42f6dbd`

### Three distinct values — they must NOT be collapsed

The defect is collapsing these; the recut keeps them separate.

1. **`resp = H e`** — the actual Host response. `erasure.rs:3448-3454` emits the
   Host `Vis` step as `RuntimeExpr::Let { value: RuntimeExpr::Effect { ... }, body:
   continuation_body }`. `effects.rs:2833/2889` emits the real host dispatch;
   `effects.rs:3629-3636` decodes its reply into
   `LoweringOperand::Specialized(Lowered::HostResult { success, error, ok, ... })`.
   That `HostResult` is the actual `H e` response for the governed `Result ...`
   operation. It is NOT the environment-transport word.
2. **`apply k resp`** — the source program's ONE Host-Vis continuation
   application. On the governed checked-IH Host-Vis route, `erasure.rs:3225-3342`
   constructs the exact `CheckedComputationalIHInvocation` and appends
   `RuntimeExpr::Var(0)` to that invocation's call arguments at `:3333` — the
   response injected exactly once. This is NOT a candidate application. For an
   ordinary lambda continuation, the same `Let.body` is the lowered lambda body
   under binder zero.
3. **`next = apply k resp`** — the application RESULT, an `ITree`, whose exact Ret
   arm is then consumed. It is neither `resp` nor the environment-materialization
   word.

### The lowering path already preserves that relation

`core.rs:3518-3537` lowers the `Let.value`, binds the resulting `HostResult` at
environment index zero, and lowers the exact `Let.body` with that binding while
retaining the active eliminator stack. The planner gives this value exact
provenance: `continuations.rs:2894-2917` mints `ProducerLocalKind::HostEffectResult`
at the Effect origin with the Let-body locator at environment index zero, and its
independent source walk/validator re-derives the whole record. **So the response
producer and the sole application are identified without a new tag, heuristic,
count, or lookup** — the recut consumes this existing structure, it does not
reconstruct it.

## Normative floor (keep every genuine spec wall)

The backend is OUTSIDE the type-soundness TCB: a bug here is a wrong value caught
by the differential, never a false proof. What is spec-REQUIRED and must hold
(`36-effects.md` §§2.1/2.2/5.2, `42-evaluation.md` §6.2, `45-native-backend.md`
§§2-4, `41-values.md` §2.1, `44-capacity.md` §§1-3):

- The eliminator receives the ACTUAL `HostResult` response for the governed
  `Result ...` operation — never the carried environment/transport word.
- The exact continuation is applied EXACTLY ONCE, in tail position; the existing
  sole application is neither dropped nor duplicated and no second application is
  introduced.
- Exactly one semantic response/resumption; no duplicated effect; strict CBV and
  the observable effect sequence unchanged.
- No observable closure identity, persistence, forged callable, or Ken-visible
  tag; internal representation stays private.
- Native value/effect observation agrees with the interpreter — differential
  parity on both products.

## Authorized design (Architect `evt_3jn61sw0w23hy`)

**Do NOT add or reapply `k`.** Center the replacement protocol on the existing
Host-Vis `Let` and its exact `CheckedComputationalIHInvocation`.

### Two typed compiler-only edges (neither discharges the other)

- **`HostResponseToApplication`** — proves WHICH value is passed to `k`: the exact
  Effect-origin producer-local binding, the exact Let-body locator, the exact
  checked invocation/call/callee/template, and the exact response argument
  position containing binder zero.
- **`ApplicationResultToRet`** — proves WHERE the result of applying `k` goes: the
  same application identity, its actual returned native result, and the existing
  exact Tail destination `{selected case, active frame, Ret body, ConstructorChild
  field 0, forward/direct delivery}`.

**No field may assert `resp == next`, and no edge may let the other discharge
it.** They are two separate proofs about two separate values.

### The application-result continuation

At the Host-Vis `Let`/application seat, **predeclare ONE private
application-result continuation before lowering the body.** Lower the existing
body exactly once. Every physical successful return of that application must
deliver its actual result exactly once to that continuation. Then consume the
existing Tail proof and lower/schedule the exact Ret destination once.

`RecursiveBackedge` remains a compiler-control marker: it may terminate the old
Rust lowering path **only after** the corresponding emitted CFG edge has delivered
the real application result. It is never itself a response or result value.

### Result representation — block param preferred, private carrier conditionally authorized

**Prefer an in-function Cranelift block parameter and the existing join/result
representation.** If the application result crosses a generated function boundary
and cannot dominate the Ret continuation, a private internal ABI/result carrier is
explicitly authorized, subject to ALL of:

- scoped to one exact checked Host-Vis application and its emitted function;
- carries the actual application result, never the environment-transport output;
- one outstanding instance per physical application emission, one consuming edge,
  a second consume refused, outstanding-at-close refused;
- no durable store, recovery scan, stable-identity lookup, proximity/count
  pairing, Ken-visible tag, closure identity, or observable representation;
- environment materialization remains a separate auxiliary result and can never
  satisfy the application-result disposition.

### Read vs write (the two products / the two deliverables)

- **Read — the direct `Computational` head** may supply the first independently
  mergeable product (D1).
- **Write — `Active` is a DISTINCT EXHAUSTIVE ARM, not a fallback** (D2): it must
  take the exact selected-scope frame authority, revalidate the same
  application/frame/Ret-body/binder relation, and deliver to the SAME logical
  application-result continuation. A generic `Active => accept`, inference from the
  sink, or borrowing the read head is FORBIDDEN.

## Deliverables

**D1 — the two edges + application-result continuation + read product green.**
Introduce, at the existing Host-Vis `Let`/application seat, the predeclared
application-result continuation and the two typed edges
(`HostResponseToApplication` + `ApplicationResultToRet`). For the read product via
the direct `Computational` head: deliver the actual application result exactly once
to the continuation, consume the existing Tail proof, and lower/schedule the exact
Ret destination once. Read differential parity passes. D1 is independently
mergeable (partial-merge policy) if the write product needs a further turn.

**D2 — write product on the `Active` exhaustive arm.** Consume the D1 protocol for
the write product: `Active` takes the exact selected-scope frame authority,
revalidates the same application/frame/Ret-body/binder relation, and delivers to
the same logical application-result continuation — a distinct exhaustive arm, never
a fallback, never `Active => accept`, never inference from the sink or borrowing
the read head. Write differential parity passes. If both products fall out of the
shared protocol in one turn, D1 and D2 may land together; the split exists so read
can land first, not to force two turns.

## Acceptance criteria (each carries its own control; controls are mutations that must red)

Mapped one-to-one from the ruling's seven required discriminating controls.

- **AC-1 — Actual response at the application (control 1).** Replacing only
  binder-zero `resp` at the existing application with the environment word
  reproduces the exact `Result` `PatternMatchFailure`; restoring the `HostResult`,
  the product reaches `InvalidOffset` (i.e. the correct response reaches the
  eliminator). The downstream `Result` wall stays discriminating — it may be
  satisfied only because the correct response arrives, never weakened to accept the
  wrong word.
- **AC-2 — Exactly the one existing application (control 2).** Dropping or
  duplicating the existing checked Host-Vis application makes the exact application
  census refuse; no second application may be introduced by the repair.
- **AC-3 — Application result, not the transport word, to Ret (control 3, distinct
  from AC-1).** Keeping the application but substituting its
  environment-materialization call result for `next` makes the application-result
  disposition refuse BEFORE Ret lowering.
- **AC-4 — One physical application-result edge, one-to-one (control 4).**
  Suppressing or duplicating one physical application-result edge leaves that exact
  instance outstanding or double-consumed and reds; aggregate equality is NOT
  accepted as proof.
- **AC-5 — Continuation installed/scheduled (control 5).** Skipping the
  application-result continuation installation/scheduling reproduces the
  unfilled-body failure; it is restored independently.
- **AC-6 — Every coordinate load-bearing (control 6).** Mutating the Effect origin,
  Let-body locator, response argument, call template, selected member, frame, Ret
  body, binder, direction, or delivery — one at a time — each refuses at its
  natural producer/consumer boundary, with an independent exact positive restored
  afterward.
- **AC-7 — Ordinary routes independently green + differential agreement (control
  7).** Ordinary Host-Vis lambda continuations, Direct routes, and ordinary Ret
  remain independently green. Read and write still require native/interpreter value
  and effect-sequence agreement. A green product alone is NOT the causal proof —
  AC-1..AC-6 controls must accompany it.

## Returns a hard stop (to the Architect) if

- closing the obligations structurally requires a Ken-visible tag, durable closure
  identity, closure identity, or a wrong-word coercion (any genuine spec wall
  above);
- the only available closure needs a durable store, recovery scan, stable-identity
  lookup, or proximity/count pairing (the ruling forbids all of these — they
  cannot prove the exact application/response pairing);
- the application result crosses a generated function boundary AND even a private
  internal ABI/result carrier under all the stated conditions cannot deliver it to
  a point that dominates the Ret continuation — a wider return-protocol question
  for the Architect, not a repair.

On a hard stop, return the exact condition and evidence to the Architect; do not
attempt a remedy or reopen a closed axis unilaterally. Per the ruling there is no
candidate and no QA route from a hard stop.

## Prohibitions

Do NOT add or reapply `k`. Do NOT forward `call_checked_ih_environment_transport`'s
returned word (or any environment-transport/materialization output) as `resp` or as
`next`. Do NOT assert `resp == next` or let one edge discharge the other. Do NOT
introduce a second semantic application, a duplicated effect, observable closure
identity, a Ken-visible tag, or a heuristic/droppable/stable-identity disposition.
Do NOT make `RecursiveBackedge` itself a response or result value. Do NOT close the
objective via Direct-only greenness while the governed Tail route executes with a
wrong value. Do NOT revive the spent `source.rs:4492` / `CarriedEnvironment` /
`InlineNoCall` D3 seat.

## Reviewers, sequencing, contention

- **Reviewers:** independent Runtime QA (all seven AC controls proven, both
  products); Architect (design review of the production increment against this
  frame, the two-edge protocol, and the normative floor — this is where a defect
  delivers a wrong value). Publisher CI is the code gate. A resolved merge Decision
  is required before routing. No Conformance Validator — this is native-backend
  value-correctness, not kernel/conformance. This grants no candidate/merge
  authority before those.
- **Sequencing:** runtime ring (lane 1). D1 (two edges + continuation + read) then
  D2 (write on the `Active` arm); D1 independently mergeable, or both together if
  one turn suffices. Size L, tier T1; hit a releasable increment or a genuine hard
  stop per turn.
- **Contention:** touches `crates/ken-elaborator/src/erasure.rs` and
  `crates/ken-runtime/src/cranelift_backend/{lowering/{effects,core,source}.rs,
  planning/static_transition/{continuations,aggregates}.rs}`, plus the
  `rt_parity_native.rs` evidence wrappers. No crate/catalog contention with the
  concurrent lanes (kernel `conv.rs`, verify CI scripts, language module/import).
  Targeted builds ONLY via `scripts/ken-cargo` scoped to `ken-runtime` /
  `ken-elaborator` / the parity test, never `--workspace`; the full-workspace and
  conformance gates run in CI.
