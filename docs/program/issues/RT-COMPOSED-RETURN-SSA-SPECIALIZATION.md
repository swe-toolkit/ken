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
origin: "RECUT 2026-09-02 (HS3 structural closure, Architect ruling evt_5yjjsrhpmt204 on research advisory evt_3z83vwpenscft): the absence-based decline is replaced by a first-class Deferred residual (classify->Disposition=Specialized|Deferred{payload}, total-match every stage, §7 sealed-enum no catch-all); the proved Specialized path is retained; base cut fresh from origin/main 4a088d8aa; ACs 1-7 in the RECUT body section; HS3 discharged. AMENDED 2026-09-02 (Architect evt_4ar3rxzrra5v4, on implementer pre-impl hard stop evt_33teszvwarz6 = HS4, not a CI-red): Deferred = P1 UNION P2 (P1 absent-residual + P2 present-but-unconsumed placeholder = the HS3-b leak); discriminator = caller-consumption (Specialized IFF specializable AND caller consumed), retargetability hoisted to planning (answer b, D0 preserved); added AC-7 pins classify vs lowering-time CandidateDisposition. Next re-trigger HS6. Prior origin: Architect mechanism ruling evt_29jfzzw9j5xjz (2026-09-01), the authoritative byte-level contract for the Specialized path; do NOT re-derive it, fold and cite it. Issued under the operator preference for compile-time SSA handling OVER the runtime closure (2026-09-01, correcting a Steward mis-scope of the approach fork; Steward direction correction evt_10dfspc3ssk5). Research extension advisory SHA-256 19bc67e5dada7cbac4445875cdcfd5ab079aecb3bc56b6df712696bcd296f3c1. The Architect confirmed this SSA path opens NO new operator fork (no public ABI change — unit_signature stays (frame_ptr, services_ptr) -> i64; no kernel primitive; no spec commitment), so the Steward frames and releases it. Bound base = the clean held checkpoint ad191d1c29af288b059bbb00c1b573c3c4356ab3, tree 342e3b735 (carries WP1's preserved environment/result role split, and BoundaryClosureEnvironment / ContinuationCallIdentity.worker as the body/arity/capture-schema authorities). The invocation-owned runtime closure (RT-COMPOSED-RETURN-RUNTIME-CLOSURE, ruling evt_3j6vshm83rk5q) remains the FALLBACK, held draft, selected ONLY if this SSA path returns SsaInfeasible and the operator so rules; it is NOT built in parallel. Halted runtime-closure scratch aee8c9408c986bb946d228069a5104c70db84ea4 is evidence only. WP1 RT-COMPOSED-RETURN-DIRECT-ROLE-SPLIT preserved as the base asset and depends_on predecessor; the delayed-SSA WP2/WP3 (RT-COMPOSED-RETURN-TAIL-FORWARD-EDGE, RT-COMPOSED-RETURN-ATOMIC-CLOSEOUT) stay closed. Origin funding evt_3met6tbk5wrnd after accepted terminal NO_UNIQUE_EDGE evt_mx6scjje1yjp."
---

## RECUT — HS3 structural closure: first-class Deferred residual (Architect ruling evt_5yjjsrhpmt204)

**This section is the CURRENT governing contract and supersedes the mechanism
below wherever they conflict.** It is the Architect's HS3 structural-closure
ruling (`evt_5yjjsrhpmt204`, 2026-09-02), issued on the research prior-art
advisory (`evt_3z83vwpenscft`, SHA-256
370795b09f783f52d3650a2888e96c2ee4b14c7f2ce7e6039bf726b0af3b576e — advisory
only, the call is the Architect's). It RETIRES the point-fix chain: no further
patch to the absence-based decline. The everything-below-here detail on the
**Specialized** path is RETAINED and remains authoritative for that path; only
the **decline / residual** handling is replaced.

### AMENDMENT (Architect ruling evt_4ar3rxzrra5v4) — Deferred = P1 UNION P2; discriminator = caller-consumption. READ FIRST.

A runtime-implementer pre-implementation hard stop (evt_6cp1w4mac9jaa /
evt_33teszvwarz6; HS4 in the chain, NOT a CI-red and NOT held against the chain —
stopping before a runtime-unverifiable blind push is the correct move) CORRECTED
the premise of this recut, and the Architect adopted the correction. The recut
scope stands, amended as follows; this block governs the Mechanism section below
wherever they differ.

**Premise correction.** The original recut said "the ledger computes the
Specialized side soundly; the defect is the absent residual." The second half is
refuted by provenance: `StaticResponseDeferred` is produced ONLY at
`core.rs:13922` (Construct) and `effects.rs:2205` (Effect), both gated on the
ledger-SPECIALIZED set (`is_static_response_operation_root` /
`is_static_response_effect` reading `static_response_continuations`). So HS3-b's
leaking response is INSIDE the specialized set — a present-but-unconsumed
placeholder — not the absent `1229` residual. **Deferred therefore has TWO
sub-cases, both -> residual:**

- **P1** — no continuation unit (`matching.is_empty()`, the `1229` residual /
  absent complement that Q1 declined). The original Deferred captured only this.
- **P2** — has a unit + owner + `StaticResponseDeferred` placeholder, but its
  selected caller is never retargeted/consumed (HS3-b; same root as HS3-a
  `disposition=None`). Capturing only P1 would compile clean and still leak =
  HS4.

Complete Deferred = **P1 UNION P2**.

**The discriminator (answer (b), with rigor).** `classify` emits **Specialized
IFF (has a unit) AND (its selected caller will be consumed — retargeted to a real
`DirectCall`/`ComposedCall`)**; otherwise **Deferred**. The consumption fact is
today settled at lowering as `CandidateDisposition`
(`DirectCall`/`ComposedCall` vs `InlineNoCall`/`TransportDormant`); it must be
HOISTED to planning so `classify` decides it ONCE (R2). Hoisting changes WHEN the
fact is computed, not the emitted code, so **D0 holds** — Specialized still emits
direct calls, no selector, no environment transport.

Do NOT classify on a syntactic proxy (e.g. the D3 `CheckedIhCapturedEnvironment`
shape) UNLESS that shape is PROVEN equal to caller-non-consumption. A proxy that
merely correlates recurs as HS4 (fix the class, not the instance). If the
retargetability predicate turns out to BE a nameable static shape, that is the
concrete form of (b) and (a)/(b) coincide — but only with the equality proof,
never assumed. Answer (c) is REFUTED by the provenance above (both production
sites are specialized-gated). Do NOT block on the OOM'd native trace for this
decision; the static provenance settles it. Cheap confirmation short of the full
native suite: a planning-time classification log on the `writeAll` fixture — does
its response have a unit, and is its caller `InlineNoCall`/`TransportDormant`? —
confirms P2 directly.

**GUARDRAIL (the one thing that could still obstruct (b)).** The ledger already
inspects the caller edge (rejecting a non-ordinary-callable continuation edge) —
NECESSARY but not SUFFICIENT for consumption; the missing piece is whether that
ordinary caller is retargeted to a real call vs stays
`InlineNoCall`/`TransportDormant`. **VERIFY that distinction is derivable from
PLANNING-available facts** (the caller edge's shape; `CandidateDisposition` keys
on caller shape the ledger already has) BEFORE threading. If it genuinely depends
on an emission-time-only decision not derivable at planning, **HARD-STOP back to
the Architect** — that is a real obstruction to R2 (the partition cannot be
classified once) and the Architect re-rules the structure, not the implementer.

**Integration.** `classify` (extended ledger) emits Specialized only for
**P0 = specializable-AND-consumed**; Deferred for **P1 UNION P2**. The Deferred
verdict gates ALL downstream: no owner forward-declaration for a Deferred response
(closes HS3-a), no `StaticResponseDeferred` placeholder emitted for it (closes
HS3-b), lowers to main's pre-WP path carrying its payload (R3). The proved
Specialized path (feasibility-ledger specialized computation,
`verify_static_response_finished_body`, Q1/Q2) is retained unchanged for P0.

**ADDED AC (AC-7 below; sharpens AC-5).** Pin `classify`'s agreement with
lowering-time `CandidateDisposition`: a response classified Specialized MUST have
its caller consumed at lowering (`DirectCall`/`ComposedCall`), and a Deferred
response MUST NOT acquire an owner or placeholder. A control that REDS if
`classify` says Specialized while lowering finds `InlineNoCall`/`TransportDormant`
(the HS3-b leak reintroduced) — the soundness pin that makes `classify` a
faithful planning-time predictor of the lowering fact.

### Why the recut (the hard-stop chain, one predicate)

Three CI-reds on the px8f/rt_parity native population, each a real distinct
defect, all one predicate: the deferred/declined response `Vis` was modeled as
an **absence** (no demand, no owner, empty set, a bare `continue`) that each
downstream stage had to independently reconstruct and route to main's lowering.
Each point-fix un-masked the next consumer:

- HS1 (Q1): demand filter ABORTS a declined deferred-frontier `Vis`
  (`SsaInfeasible` -> fatal backend abort).
- HS2 (found 0): the px8-ds mutation helper requires an owner and finds zero for
  a fully-fallback program (test-support-only, production-inert).
- HS3-a (`rt_resource_release_carried_observe`): a forward-declared response
  owner has no verified selected incoming call (`disposition=None`).
- HS3-b (`writeAll`): `unsupported runtime-IR lowering: StaticResponseDeferred`
  — a deferred host response is compiler control with no supported lowering arm.

A census/grep cannot close this — it failed twice. The tell: HS3-b names
`StaticResponseDeferred`, a runtime-IR variant that **already exists** — so
"deferred" is partially first-class but not exhaustively handled.

### Mechanism — reify the specialize/residualize partition, classify ONCE

Compute a positive two-valued classification on the response IR, once, consumed
by total matches everywhere:

```rust
classify : ResponseVis -> Disposition          // one pass; positive verdict
lower    : Disposition  -> RuntimeIR            // total match, NO `_ =>`

enum Disposition {
    Specialized { owner, captures, k_route, .. }, // the proved path, unchanged
    Deferred    { payload },                      // routes to main's pre-WP lowering
}
```

- `classify` is the EXISTING feasibility ledger EXTENDED to emit a populated
  `Deferred{..}` instead of an empty complement / no-owner / `None`. The ledger
  already computes the Specialized side soundly; the whole defect is that its
  residual output today is the absent complement rather than a constructed object.
- `Deferred` is a constructor of the SAME sealed sum the already-half-born
  `StaticResponseDeferred` (HS3-b) belongs to. Promote it to a FULL peer at every
  stage: planning/classify, forward-declaration, caller-edge verification,
  retained-unit declaration, runtime-IR lowering. Each stage matches
  `Disposition` with **no catch-all**, so an unhandled stage is a Rust COMPILE
  error (COORDINATION §7 sealed-enum), not a CI-red. This converts "a census must
  find every consumer" into "the type enumerates the consumers for you" — the
  reason the census failed twice and a sealed variant cannot.

### Three binding requirements (each kills one symptom face)

- **R1 — REACHABILITY IS ORTHOGONAL TO COLOR.** Demand/reachability and
  Specialized-vs-Deferred are separate analyses; a `Vis` can be reached AND
  Deferred (the normal residual). HS1 conflated "no static demand" with "not
  present" and aborted. A reached-but-Deferred `Vis` is expected and MUST pass to
  residual lowering, never abort.
- **R2 — CLASSIFY ONCE, ON THE OBJECT.** The verdict lives on the IR object as a
  constructor, not re-derived from local negative evidence at each consumer. This
  removes the reconstruct-obligation from every stage simultaneously — it is the
  closure, and it is why HS2 (found 0) and HS3-a (`disposition=None`) both vanish
  rather than getting a third and fourth patch.
- **R3 — THE RESIDUAL CARRIES ITS PAYLOAD.** HS3-b is a tag with no
  payload-carrying case, so lowering has nowhere to send it. `Deferred{payload}`
  carries exactly the data to route to main's existing pre-WP lowering (the path
  that compiled and ran at `4a088d8aa`), so its lowering arm is a real
  translation, not an unsupported stub.

### Boundary — DO NOT VIOLATE D0 (evt_29jfzzw9j5xjz)

The sealed-tag + total-match STRUCTURE is for the **Deferred** case ONLY. The
Specialized population stays exactly as proved (the mechanism below): direct
calls to the exact K context, the finished-CLIF read-back
(`verify_static_response_finished_body`), NO runtime selector / K tag /
environment aggregate / closure word / shared apply. A tagged variant carrying an
environment is the closure-conversion form D0 excluded; it must never touch the
Specialized side. Only `Deferred` is tagged, and `Deferred` lowers to the
pre-existing main path, which introduces no selector.

### Retain vs replace

- **RETAIN** everything proved: the feasibility ledger's Specialized-side
  computation, the finished-CLIF `Ret`/`Trap` read-back, and the Q1/Q2
  Specialized logic — all correctly reviewed, none is the defect. The entire
  Specialized-path detail below this section is retained and authoritative.
- **REPLACE** only the absence-based decline: Q1's `continue`, HS2's
  empty-substitute, and every stage's implicit "no owner => fall through" become
  one populated `Deferred` threaded by total matches.

### Acceptance criteria — the totality proof (carry ALL SIX)

1. **Congruence before the passes run.** Every `ResponseVis` receives exactly one
   of Specialized/Deferred; no third "unclassified" leak. Assert exhaustiveness of
   `classify`.
2. **Per-stage §7 control.** At EACH stage, deleting/adding a `Disposition`
   variant reddens the Rust build — a compile-time pin per stage, not a runtime
   test. This is the closure's own proof that no stage silently drops the residual.
3. **Positive Deferred program.** The `writeAll` deferred-frontier fixture (the
   `4a088d8aa` shape) COMPILES and RUNS through main lowering with UNCHANGED
   effect order — a real "still compiles+runs" positive control.
4. **Mixed program.** One unit carrying BOTH colors (a Specialized response and a
   Deferred response together). Polyvariance is only real when both coexist; a
   single-color program cannot discriminate genuine threading from a flag.
5. **Specialized-through-Deferred-arm control.** A test that REDS if a Specialized
   response ever flows through the Deferred lowering arm — proving the proved path
   is untouched and the D0 boundary holds.
6. **CI-native whole-binary population green on the exact SHA:**
   `px8f_buffer_native`, `rt_parity_native` all shards,
   `rt_resource_release_carried_observe`, all 8 test shards — the authoritative
   close that caught HS1/HS2/HS3.
7. **classify/lowering agreement pin (AMENDMENT AC, evt_4ar3rxzrra5v4).** A
   response classified Specialized MUST have its caller consumed at lowering
   (`DirectCall`/`ComposedCall`); a Deferred response MUST NOT acquire an owner or
   `StaticResponseDeferred` placeholder. A control REDS if `classify` says
   Specialized while lowering finds `InlineNoCall`/`TransportDormant` (the HS3-b
   leak reintroduced). This is the soundness pin proving `classify` is a faithful
   planning-time predictor of the lowering-time `CandidateDisposition` fact.

Also fold in the **deferred option-2 coverage fixture** (`evt_55jt2yydg0661`)
while the response-IR is being restructured — it is the same surface.

### Scope, gates, base

- **Scope:** elaboration/backend only (`crates/ken-runtime`). NO kernel, TCB,
  `/spec`, or `/conformance` change; research confirmed no operator escalation on
  the mechanism.
- **Base / fixed inputs:** cut fresh from `origin/main` **`4a088d8aa`** (the
  pre-WP baseline where the node is held and the `writeAll` deferred-frontier
  program compiled and ran). This supersedes the stale `ad191d1c2` base cited
  below.
- **Candidate is NEW with fresh gates:** Architect soundness + runtime-QA +
  CI-native on the exact SHA. NO prior approval carries. On a gated candidate:
  Steward M1-M4 -> lieutenant M5-M9. Node stays HELD at the pre-WP baseline until
  the recut lands.
- **§1a bookkeeping:** HS3 is DISCHARGED by this ruling; the next re-trigger is
  HS6.

## Authoritative contract (Specialized-path detail — RETAINED under the recut above)

The Architect mechanism ruling **`evt_29jfzzw9j5xjz`** is the authoritative,
byte-level contract **for the Specialized path**. This node folds its structure,
types, and controls for release; where any detail here is thinner, the ruling
governs. Do not re-derive. The **decline / residual** handling in the sections
below is SUPERSEDED by the RECUT above (first-class `Deferred`); the Specialized
mechanism, representation, feasibility trichotomy, emission seam, and Specialized
controls remain in force.

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
