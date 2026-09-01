---
id: RT-COMPOSED-RETURN-RUNTIME-CLOSURE
title: "Composed-return native repair — TARGETED INVOCATION-OWNED RUNTIME CLOSURE (Architect mechanism ruling evt_3j6vshm83rk5q). ONE atomic runtime-closure repair from the clean held checkpoint ad191d1c2, REPLACING the refuted delayed-SSA route; four internal checkpoints, gated + merged ONCE at the atomic tip (Runtime QA + Architect on the exact SHA). Adds an internal InvocationClosure boundary tag + a one-shot live-closure descriptor and a response-seam apply; NO kernel primitive, NO spec commitment, NO public callable ABI — it implements normative spec 42 §6.2 (Vis e k -> apply k (H e)), 41 §2.1 (ordinary live-domain closures), 45 §3 (native function lowering)."
status: active
owner: runtime
size: L
gate: runtime-qa+architect
tier: T1
depends_on: [RT-COMPOSED-RETURN-DIRECT-ROLE-SPLIT]
blocks: []
github: null
origin: "Architect mechanism ruling evt_3j6vshm83rk5q (2026-09-01), the authoritative byte-level contract for this repair; do NOT re-derive it, fold and cite it. Issued under the operator re-scope: the (a)(i) exclusions were fleet design framing, not operator constraints (operator clarified 2026-09-01), and the operator opened the general approach (compile-time SSA vs targeted lowering of the closure into the runtime) and delegated the way-through to Architect + Research (Steward re-scope evt_452d383h945f9; Research advisory SHA-256 66c858f2e7dc51b089cba26c87725b1c80d1258ec882cd7da29c68670affafb1). The Architect selected targeted runtime lowering and confirmed it opens NO new operator fork (no kernel primitive, spec commitment, public callable ABI, equality, persistence, or cross-domain behavior), so the Steward frames and releases it — not an operator gate. Bound base = the clean held checkpoint ad191d1c29af288b059bbb00c1b573c3c4356ab3, tree 342e3b735 (carries WP1's preserved environment/result role split). Diagnostic only, not production: a9d174ccb2941e3a0d05c5cc158417e46c5077ec (tree fc37564dc), row dump SHA-256 cc09b264dedda250e126cbc3a5dde80c6cb217cea08008171e09ea481121a3fc. This node REPLACES the refuted delayed-SSA route: RT-COMPOSED-RETURN-TAIL-FORWARD-EDGE (WP2, mechanism refuted at HS#5 evt_67hab6csq6mc7) and RT-COMPOSED-RETURN-ATOMIC-CLOSEOUT (WP3, old closeout) are superseded and closed as diagnostic evidence; initial-development replacement applies, no parallel routes. WP1 RT-COMPOSED-RETURN-DIRECT-ROLE-SPLIT is PRESERVED as the base asset in ad191d1c2 and is the depends_on predecessor. Origin funding evt_3met6tbk5wrnd after accepted terminal NO_UNIQUE_EDGE evt_mx6scjje1yjp."
---

## Authoritative contract

The Architect mechanism ruling **`evt_3j6vshm83rk5q`** is the authoritative,
byte-level contract for this repair. This node folds its structure, types, and
acceptance boundary for release; where any detail here is thinner than the
ruling, the ruling governs. Do not re-derive the mechanism.

## The mechanism (why it is determined)

The permanent semantic baseline is an **invocation-owned, one-shot ordinary
closure**, applied in generated code **after the host reply is
validated/materialized and before answer collapse**. HS#5 proved the current ABI
has no SSA owner that joins the future response, the exact continuation K, the
captures, and the target (the closed `RT-LIVE-K-FUTURE-INPUT-OWNERSHIP-D0`
negative). Adding one either specializes every shared response producer per K or
transports a selector plus environment — the latter is closure conversion under
another name. The runtime form is the general mechanism and matches the normative
`Vis e k -> apply k (H e)` rule (spec 42 §6.2), ordinary live-domain closures (41
§2.1), and native function lowering (45 §3). Direct SSA application remains a
lawful LATER optimization only when the planner constructively proves one fixed K
per response-producing Function and carries every capture as an explicit
parameter; it is **not** a second implementation in this repair and receives no
credit here.

## Required representation

Add the invocation tag; **do not revive the durable tombstone**
(`PersistentClosure` stays recognized-and-retired — never admitted, retagged,
adopted, or used as fallback). `RuntimeValue::ClosureRef` is semantic precedent
only (its string symbol and value-domain captures are not this planner-issued
carrier).

```rust
#[repr(u8)]
enum BoundaryTag {
    // existing values 0..=9 unchanged
    InvocationClosure = 10,
}

#[repr(u64)]
enum LiveClosureState { Armed = 0, Running = 1, Consumed = 2, Failed = 3 }

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct LiveClosureCodeId(u64);

struct LiveClosureDescriptor {
    code: LiveClosureCodeId,
    creation: StaticOriginId,
    body: StaticOriginId,
    params: Vec<String>,              // exactly one
    captures: Vec<StaticOriginId>,    // declaration order
    capture_record: AggregateOccurrenceId,
    response_origin: StaticOriginId,
    response_owner: PredeclaredFunctionId,
}

#[derive(Clone, Copy)]
struct LiveClosureOperand { word: CarriedBoundaryWord }
```

`(InvocationClosure, BoundaryClass::Closure)` is the only live closure relation.
Referent owner `InvocationArena`; `NODE_SLOT == NULL_SLOT`; `NODE_TAG_ID ==
code`; `NODE_PAYLOAD == LiveClosureState`; ordered child words are the captures.
`BoundaryClosureEnvironment` is retained as the capture/body/schema authority and
subsumed into the descriptor. At the selected K creation seat the old
environment-only emitted value is replaced by the `InvocationClosure` word:
allocate, write planner-issued code id, write `Armed`, write exactly the
descriptor's capture count/order, seal by exact schema, expose only
`LiveClosureOperand`. No body term, `FuncRef`, route enum, pointer, equality,
hash, or reverse-decoding payload enters the word.

## Transport and application

The planner publishes one exact `creation -> response_owner/response_origin`
relation and adds one explicit closure-word parameter along that generated
Function edge. **Forbidden:** process-global/thread-local pending K, adjacency
lookup, body lookup, EntryAbi replay, seed substitution, static-origin side
table. A dynamic `Vis` activation creates exactly one word; the invocation arena
never reuses its node index before the activation dies.

At the response owner keep the current host sequence through
`lower_process_host_effect` (call `ken_host_dispatch_v1`, check status/tags,
validate resource errors and response shape, construct/materialize the exact
`Lowered::HostResult`). Then, **before** returning it to `RoutedAnswer` or any
source-machine collapse, invoke exactly:

```rust
fn call_live_closure_once(
    &mut self,
    builder: &mut FunctionBuilder<'_>,
    closure: LiveClosureOperand,
    response: CarriedBoundaryWord,
    site: StaticOriginId,
) -> Result<CheckedIhApplicationResult, CraneliftBackendError>;
```

Body constraints:

1. Resolve the descriptor for `site`; require one descriptor, `params.len() ==
   1`, exact `InvocationClosure + Closure`, invocation owner, null slot, exact
   code id, exact capture count.
2. Require state `Armed`, then write `Running` before loading a target or
   entering K. `BoundaryWord` is copyable; this runtime transition, not a Rust
   move, is the affine guard. `Running`/`Consumed`/`Failed` all refuse a second
   application.
3. Load captures by ordinal through the existing boundary field helper.
   Re-declare/select the descriptor's target in this applying Function; never
   transport a Cranelift `FuncRef`.
4. Call the existing route-aware declared-call emitter with operands `[response,
   capture_0, ..., capture_n]` — response is explicit parameter 0, captures the
   ordered suffix. Trap/status checked before Result exactly as today.
5. On returned Trap/error mark `Failed`; on a Trap-checked Result mark `Consumed`
   before routing it. Construct `CheckedIhApplicationResult` only from that
   emitted call and Result. Only its word may reach the exact one-parameter Ret
   sink; the `HostResult` itself may not.

`ken_host_dispatch_v1` must return before step 1 — K is never called from Rust
host dispatch. A nested effect performed by K therefore begins after the prior
mutable `ProcessContext` borrow ended and allocates its own closure node; it
cannot overwrite a singleton pending slot.

## Deliverable — one atomic repair, four internal checkpoints

Built on one branch from the clean held checkpoint `ad191d1c2`. **No QA,
Decision, publication, or merge exists before the atomic tip.** Each checkpoint
is a natural report/hard-stop seam:

1. **Descriptor + `InvocationClosure` ABI** — the types above; tag/relation/owner
   wiring; `PersistentClosure` stays retired.
2. **Creation + explicit transport** — replace the environment-only emitted value
   at the K creation seat with the sealed `InvocationClosure` word; publish the
   `creation -> response_owner/response_origin` relation and the one explicit
   closure-word parameter along the generated Function edge.
3. **Response-seam apply + Ret** — `call_live_closure_once` at the validated
   response seam per the five body constraints; route only its
   `CheckedIhApplicationResult` word to the exact one-parameter Ret sink.
4. **Full controls** — the acceptance boundary and the complete mutation grid
   below. This checkpoint is the atomic tip: it cuts the SOLE production
   candidate (Runtime QA + Architect on the exact SHA, then Steward M1-M4,
   lieutenant M5-M9).

## Acceptance boundary

One dynamic event must pair: exact effect/host call; validated `HostResult`
Inst/value; exact K code id and capture instance; one `Running -> Consumed`
application taking that response as **operand 0**; Trap-checked K Result; exact
Ret argument. **Both fixed products (`rt_parity_fs_read_at_offset_single`,
`rt_parity_fs_write_at_offset_single`) reach `InvalidOffset` with unchanged
effect order** — the base-red `ResourceBodyResult` `PatternMatchFailure` flips to
exact `InvalidOffset`.

## Mutation grid (each negative must reach and red for its OWN claim, with an exact restored positive)

Independently mutate: code id; arity; capture count; capture order; capture
value; owner; slot; activation; response operand; each transport parameter; state
before apply; duplicate apply; stale handle; persistent adoption;
apply-before-validation; apply-after-collapse; Ret sink; nested-effect re-entry.
Direct SSA receives no credit and no implementation in this WP.

## Disposition of held work (preserve as evidence, not production)

Superseded as construction mechanisms: the delayed-Tail call `TailProducerPending
-> call at S` and the WP2 phase disposition. Preserve their measurements and
controls as evidence (`a9d174ccb` diagnostic, the P=14 / 27-row 16/9/2 ledger /
terminal-control discriminator). **Preserve independently sound assets:**
environment/result role separation (WP1 in `ad191d1c2`), planner-issued capture
order / body identity, route-aware call emission, Trap-before-Result, the exact
Ret sink, and the fixed semantic products. Do not keep the runtime closure and
the failed delayed-SSA route in parallel.

## Contention

Single-writer runtime lane, priority lane 1. Touches the Cranelift backend
boundary-carrier/closure lowering and the response seam
(`lower_process_host_effect`) — no overlap with the concurrent doc track
(`library/`, `agent/`) or the language lane's kernel/FO work. No kernel crate
touch (`crates/ken-kernel` byte-unchanged); no `/spec` change. Base is the held
runtime branch `ad191d1c2`, not `main`; the sole candidate merges at the atomic
tip.
