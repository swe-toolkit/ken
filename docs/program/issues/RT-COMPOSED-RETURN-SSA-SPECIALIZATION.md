---
id: RT-COMPOSED-RETURN-SSA-SPECIALIZATION
title: "Composed-return native repair — PRIMARY: polyvariant compile-time response-owner specialization (Architect mechanism ruling evt_29jfzzw9j5xjz), the operator-preferred compile-time SSA path OVER the runtime closure. For every statically attributable (response producer, K) pair, emit one Function whose code identity fixes K, whose frame carries all K captures + enclosing continuation inputs as explicit ABI slots, and whose body does host dispatch/validation then directly calls that K with the exact response as operand 0. NO K tag, closure word, apply dispatcher, environment aggregate, code pointer, process-global slot, or runtime selector. Checkpoint 1 is a FEASIBILITY LEDGER ONLY; an irreducibly-multiple or dynamic-K reached edge is a typed SsaInfeasible finding that STOPS and routes to the operator before the fallback is selected. No public ABI change (unit_signature unchanged); no kernel primitive; no spec commitment."
status: active
owner: runtime
size: L
gate: runtime-qa+architect
tier: T1
depends_on: [RT-COMPOSED-RETURN-DIRECT-ROLE-SPLIT]
blocks: []
github: null
origin: "Architect mechanism ruling evt_29jfzzw9j5xjz (2026-09-01), the authoritative byte-level contract; do NOT re-derive it, fold and cite it. Issued under the operator preference for compile-time SSA handling OVER the runtime closure (2026-09-01, correcting a Steward mis-scope of the approach fork; Steward direction correction evt_10dfspc3ssk5). Research extension advisory SHA-256 19bc67e5dada7cbac4445875cdcfd5ab079aecb3bc56b6df712696bcd296f3c1. The Architect confirmed this SSA path opens NO new operator fork (no public ABI change — unit_signature stays (frame_ptr, services_ptr) -> i64; no kernel primitive; no spec commitment), so the Steward frames and releases it. Bound base = the clean held checkpoint ad191d1c29af288b059bbb00c1b573c3c4356ab3, tree 342e3b735 (carries WP1's preserved environment/result role split, and BoundaryClosureEnvironment / ContinuationCallIdentity.worker as the body/arity/capture-schema authorities). The invocation-owned runtime closure (RT-COMPOSED-RETURN-RUNTIME-CLOSURE, ruling evt_3j6vshm83rk5q) remains the FALLBACK, held draft, selected ONLY if this SSA path returns SsaInfeasible and the operator so rules; it is NOT built in parallel. Halted runtime-closure scratch aee8c9408c986bb946d228069a5104c70db84ea4 is evidence only. WP1 RT-COMPOSED-RETURN-DIRECT-ROLE-SPLIT preserved as the base asset and depends_on predecessor; the delayed-SSA WP2/WP3 (RT-COMPOSED-RETURN-TAIL-FORWARD-EDGE, RT-COMPOSED-RETURN-ATOMIC-CLOSEOUT) stay closed. Origin funding evt_3met6tbk5wrnd after accepted terminal NO_UNIQUE_EDGE evt_mx6scjje1yjp."
---

## Authoritative contract

The Architect mechanism ruling **`evt_29jfzzw9j5xjz`** is the authoritative,
byte-level contract. This node folds its structure, types, and controls for
release; where any detail here is thinner, the ruling governs. Do not re-derive.

## AMENDMENT — context-demand extension (Architect evt_4ta6cchxvjrrt); CP1 CORRECTED

The CP1 `SsaInfeasible` on both FsReadAt rows was a PHASE-ORDERING artifact, not
genuine infeasibility, and does NOT go to the operator. `intern_generated_contexts`
at the held checkpoint interned contexts only from the pre-existing
`PlannedContinuationSpecializationCall` (old-caller) population; the response-owner
call does not exist yet, so `continuation_context_for(...)` answered "was some old
caller already enough to mint this target?" — the zero is over the old caller
population, not over K or its ABI inputs. The two FsReadAt rows are SINGLETON-K
rows; the already-issued `PlannedContinuationContext` contract is exactly the
missing target shape (Parameter run = raw K arity + K captures; Capture run = the
K specialization's ordered continuation inputs). No new runtime representation or
ABI family.

Extend the SSA planner:

1. Split response planning into a PRE-CONTEXT DEMAND phase and a POST-CONTEXT
   RESOLUTION phase. Pre-context derives every response producer/K row from the
   semantic graph + the just-built continuation specialization/call population and
   finishes capture/source validation (a count is not enough).
2. Add a typed `StaticResponseContextDemand` keyed by the existing pair
   `(K ContinuationSpecializationId, k_body_origin)`, carrying the response row
   identity for closure checking — NOT a second context-identity domain or a
   response-specific ABI kind.
3. Intern ordinary `PlannedContinuationContext`s from the UNION of existing
   causal-call demands + response demands: intern the existing call population
   FIRST (preserving existing context IDs), then append new unique response keys
   in deterministic response-row order. Same key reuses one context; same key with
   disagreeing worker/input schema is a planner error.
4. Build/install/finalize the existing context ABI from the K specialization's own
   `worker` + `continuation_inputs` authorities — do NOT reconstruct schema from
   response syntax.
5. Resolve each response demand by exact key to the now-issued
   `ContinuationContextId`, then publish `StaticResponseContinuation`. A missing
   context is now a planner population-closure ERROR, not `SsaInfeasible`.
6. Keep `SsaInfeasible` ONLY for the real semantic arms: an incoming edge carries
   multiple/opaque K values, or a K capture/input cannot be expressed as one
   explicit static frame source. Continue the all-producer walk PAST the FsReadAt
   rows (later rows are not assumed feasible).

Still compile-time SSA/lambda lifting: Function identity fixes K; response +
captures/inputs stay explicit slots; the call target stays an ordinary
`ContinuationContextId`. No K tag / closure word / environment aggregate / apply
dispatcher / code pointer / runtime selector; no public ABI change; no kernel
change; no spec commitment — NO new operator fork.

Checkpoint correction (supersedes the "Deliverable" list below where they differ):

- **CP1** completes only when the full read/write all-producer population has
  EITHER a fully-validated context demand for every singleton-K row OR a real
  typed dynamic/non-expressible `SsaInfeasible`. "No context existed before the
  new edge" is NO LONGER an infeasible arm.
- **CP2** interns the union context population, installs its existing ABI,
  forward-declares response owners, statically retargets exact callers.
- **CP3** defines the response owner and emits the exact context call after
  validation.
- No emitted context discharges reachability by declaration; at the atomic tip
  every newly demand-issued context must have >=1 selected response-owner call
  (delete it -> population-closure gate reds).

Added controls (with the existing mutation grid): READ demand key
`Specialization(0)` / body `766`, WRITE `Specialization(0)` / body `1075` — both
resolve to planner-issued contexts and the all-producer walk continues; delete
only the response demand -> FsReadAt row reds before emission; duplicate demand ->
one context, not two; vary body / K identity / capture source / continuation-input
source -> reject the disagreement; prove existing causal-call context IDs +
descriptors unchanged when response demands are appended; remove/retarget the sole
response-owner call -> reject declared-but-unentered; call raw worker -> reject
wrong ABI; RETAIN a genuine dynamic-K row that returns typed `SsaInfeasible` (so
this extension does not make the fallback arm unreachable).

Held checkpoint `48fa6c9d6` remains evidence, not a candidate; `dac8edab`
diagnostic only.

## The mechanism

Select **polyvariant, compile-time response-owner specialization**. For every
statically attributable `(response producer, K)` pair, emit one Function whose
code identity fixes K, whose frame carries all K captures and enclosing
continuation inputs as **explicit ABI slots**, and whose body performs host
dispatch/validation and directly calls that K with the exact response as
**operand 0**. No K tag, closure word, apply dispatcher, environment aggregate,
code pointer, process-global slot, or runtime selector exists.

**Why the terminal D0 negative does not refute this.** D0 proved the
*unspecialized* graph has no future owner joining response, K, captures, and
target. The missing object is a **statically keyed response-owner Function plus
its retargeted caller** — not an index into the existing operand run. Concretely:
WRITE `Vis` 1250 has K closure 1246 / body 1238, arity one, seven captures,
target context 0; READ application 138 and WRITE application 175 each select one
live K. Do not patch context 0 (unentered), context 1 (parameter 0 is the prior
response), or app486 (precedes the future response).

## Required planner representation

```rust
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct StaticResponseContinuationId(u32);

#[derive(Clone, Debug, Eq, PartialEq)]
struct StaticResponseCapture {
    ordinal: u32,
    origin: StaticOriginId,
    source: ContinuationSourceCoordinate,
    producer_abi_slot: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct StaticResponseContinuation {
    id: StaticResponseContinuationId,
    base_owner: ContinuationEmissionOwner,
    producer_call_origin: StaticOriginId,
    response_origin: StaticOriginId,
    vis_origin: StaticOriginId,
    k_identity: ContinuationCallIdentity,
    k_closure_origin: StaticOriginId,
    k_body_origin: StaticOriginId,
    k_context: ContinuationContextId,
    captures: Vec<StaticResponseCapture>,
    continuation_inputs: Vec<(u32, ContinuationSourceCoordinate, u32)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct StaticResponseProducerSpecialization {
    base_owner: ContinuationEmissionOwner,
    continuation: StaticResponseContinuationId, // singular by construction
    header: AbiFrameHeader,
    slots: Vec<AbiSlot>,
}
```

`BoundaryClosureEnvironment` and `ContinuationCallIdentity.worker` remain the
independent body/arity/capture-schema authorities; their runtime environment
record is **not** emitted. `ContinuationContextId` is the K target (it preserves
the K parameter/capture run plus enclosing inputs); calling a raw worker is a
wrong ABI. "Explicit parameter" means a named `AbiSlotKind::Parameter` or
`Capture` and a mapped operand on every direct edge. `unit_signature` stays
`(frame_ptr, services_ptr) -> i64` — no public ABI change.

## Feasibility and specialization algorithm (the load-bearing trichotomy)

Build the relation independently from both ends: host-response producer/caller
edges from the closed semantic graph, and K binder/body/context/capture schema
from the exact `Vis` recursive field plus `ContinuationCallIdentity`. Group by
unspecialized producer P.

- **`|K(P)| = 1`** → emit one specialization.
- **finite `|K(P)| > 1` with a singleton K on every incoming edge** → emit one
  specialization per `(P,K)` and statically retarget each caller. At a join,
  split critical edges or sink the wrapper into predecessor arms. This is
  ordinary polyvariant SSA / lambda lifting, **not** closure conversion.
- **an incoming edge still carries multiple Ks as data, an opaque/higher-order
  parameter supplies K, or capture sources cannot be expressed as explicit frame
  slots** → return a typed **`SsaInfeasible`** record naming that exact edge and
  STOP. Do not choose the first K; do not add a selector-plus-environment. The
  Steward routes that finding to the operator **before** the fallback is
  selected.

Use a memoized worklist keyed by `(base owner, producer edge, static K
identity/schema)`, inserting before descent. Clone an SCC once per static key;
capture values never enter it. A numeric clone cap is a resource refusal, never
permission to merge keys.

## Required emission seam

Forward-declare every response specialization with the existing unit bundle
before defining bodies. Each generated Function declares its own `FuncRef` for
`k_context`. The selected Function contains `lower_process_host_effect` through
`ken_host_dispatch_v1`, status/tag/resource-error validation, and exact
`Lowered::HostResult` materialization. **Do not** factor HostResult across a new
helper ABI in this repair — clone the lowering IR so response and K coexist in
one Function (a later shared-H optimization needs its own explicit checked
response ABI). After validation and before returning HostResult or entering
answer collapse, invoke:

```rust
fn call_static_response_continuation(
    &mut self,
    builder: &mut FunctionBuilder<'_>,
    route: &StaticResponseContinuation,
    response: LoweringOperand,
) -> Result<CheckedIhApplicationResult, CraneliftBackendError>;
```

It requires the specialized owner and exact `response_origin`; reads captures
only from mapped current-Function ABI slots; assembles `[response, capture_0,
..., capture_n, continuation_inputs...]`; calls the Function-local context
target; checks Trap/status before Result; constructs `CheckedIhApplicationResult`
only from that call. Raw `HostResult` may not leave; only K Result reaches Ret.

## Deliverable — one atomic SSA repair, four checkpoints

Built on one branch from the clean held checkpoint `ad191d1c2`. **No QA,
Decision, publication, or merge before the atomic tip.**

1. **Static feasibility ledger + context-demand validation (CORRECTED — see the
   AMENDMENT above).** Publish every producer/caller-edge → exact K/schema row for
   both fixed products. For every SINGLETON-K row, derive and fully validate its
   typed context DEMAND (capture/source validation, not a count). **No production
   emission yet.** CP1 completes only when the full all-producer population has
   either a validated context demand for every singleton-K row OR a real typed
   dynamic/non-expressible `SsaInfeasible` (multiple/opaque K, or a capture/input
   not expressible as one explicit static frame source). "No context existed
   before the new edge" is NOT infeasible — it is a demand to intern at CP2. ONLY
   a real dynamic/non-expressible `SsaInfeasible` is the hard stop the Steward
   routes to the operator.
2. **Typed specialization population** — fixed-point/SCC closure, explicit ABI
   slots, forward declarations, caller retargeting. Prove every emitted
   specialization has at least one selected incoming caller; a
   declared-but-unentered Function is non-discharge.
3. **Response-local K application** at the validated host seam
   (`call_static_response_continuation`), target-context call, Trap-before-Result,
   exact Ret route.
4. **Full controls and the sole atomic candidate** — the acceptance boundary and
   mutation grid below. The tip cuts the SOLE candidate (Runtime QA + Architect
   on the exact SHA, then Steward M1-M4, lieutenant M5-M9).

## Acceptance boundary

Both `rt_parity_fs_read_at_offset_single` and `rt_parity_fs_write_at_offset_single`
reach `InvalidOffset` with unchanged effect order. One dynamic row must join:
selected incoming caller Inst, specialized Function, exact host dispatch,
validated response Value, ordered explicit captures, local K `FuncRef` / call
Inst, Trap-checked K Result, and exact Ret argument.

## Mutation grid (each negative must reach and red for its OWN claim, then restore exactly)

Independently: drop / duplicate / vary a producer-to-K row; merge two K keys;
restore the shared unspecialized producer; remove or retarget an incoming caller;
substitute context 0 without a caller; replace response with operation, prior
response, or app486 environment; drop / permute / vary every capture and
continuation input; call raw worker instead of context; omit / duplicate the K
call; move it before validation or after collapse; bypass Trap-before-Result;
vary Ret. **Include one statically shared two-K producer whose callers split into
two direct specializations, and one genuinely dynamic-K negative that yields
`SsaInfeasible`.**

## Fallback disposition

The invocation-owned runtime closure (`RT-COMPOSED-RETURN-RUNTIME-CLOSURE`,
ruling `evt_3j6vshm83rk5q`) is the FALLBACK, held draft. It is selected ONLY if
checkpoint 1 returns `SsaInfeasible` and the operator so rules. It is NOT built
in parallel; halted scratch `aee8c9408` is evidence only.

## Contention

Single-writer runtime lane, priority lane 1. Touches the Cranelift backend
specialization/emission and the response seam (`lower_process_host_effect`) — no
overlap with the doc track (`library/`, `agent/`) or the language lane's FO
adequacy work. No kernel crate touch (`crates/ken-kernel` byte-unchanged); no
`/spec` change; no public ABI change. Base is the held runtime branch
`ad191d1c2`, not `main`; the sole candidate merges at the atomic tip.
