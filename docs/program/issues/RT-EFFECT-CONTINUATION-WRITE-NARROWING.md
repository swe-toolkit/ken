---
id: RT-EFFECT-CONTINUATION-WRITE-NARROWING
title: "Build the general cross-context effect-result channel: carry an effect's full result (the discriminated ResourceBody{Ok/Err} Ken value, including narrowed error products) from the inner effect-execution response-specialization cranelift function, across the function-return boundary, to the outer effect-tail exit sink — replacing the source-machine backward reconstruction (the lossy middle) whose PatternMatchFailure trap today masks the exact error product. First consumer: the composed read-then-write recursor's write-narrowing (write reaches exit as ResourceBodyErr(InvalidOffset), not PatternMatchFailure). The carrier + join + exit-routing are effect-agnostic and shared; the socket/network effect paths are the next consumers."
status: active
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Operator ruling 2026-09-04 (this session, verbatim: \"Fund (ii).\") FUNDED the option-(b) cross-context effect-continuation capability after the inc3-HS5 escalation (Architect evt_11fw2641pyk59). Operator grounding: the native-ABI campaign is less than half done (sockets/networking still ahead), the cross-context effect-continuation pattern is common and will recur, and accurate error products are critical agent-facing guidance — so this is funded as a GENERAL reusable capability, not a two-test point fix (write-narrowing is only its first consumer). Mechanism + ABI/spec answer ruled by the Architect evt_fevj1ay5cbwh (grounded from ea0c04eec): ABI/spec = NO, gate=none, backend-only, zero TCB. Steward-filed and released per COORDINATION section 2. This is the WRITE HALF (ii) of the D3-RECUT terminal deliverable; the (i) soundness-hardening within-lane increment on RT-COMPOSED-RETURN-FORWARD-RET-EDGE is its clean base."
---

> # ACTIVE — RELEASED. Lane 1 (runtime), operator-funded (ii).
> The (i) soundness-hardening increment LANDED as squash 553d99a42
> (blob-verified) — it established the write tail is non-collapsible and
> falls to base, which is the clean base this channel operates on. Base for
> this build = current main (553d99a42). RE-MEASURE every coordinate at that
> base before editing: the line numbers in the Mechanism section are from
> ea0c04eec and decay; the semantic anchors (AbiCarrier::ResultWord, the
> CarrierWord join, format_final_export, dead_arm_effect_trap) are stable.
>
> Mechanism front-loaded by the Architect (evt_fevj1ay5cbwh); the ring
> builds, the Architect reviews. gate=none, backend-only, zero TCB — the
> standing runtime hard-stop protocol still applies (a design fork or any
> newly-surfaced ABI/spec/TCB surface HARD-STOPS to the Architect, and per
> the caveat below a multi-word future effect result re-flags the boundary).

## Why this is funded as a capability, not a point fix

Operator grounding (2026-09-04): the native-ABI campaign is less than
halfway; sockets and networking are still ahead; the cross-context
effect-continuation pattern recurs across them; and accurate error products
(the exact `ResourceBodyErr(InvalidOffset)` rather than a malformed
`PatternMatchFailure` trap) are critical guidance for agents consuming Ken.
So the deliverable is the GENERAL effect-result channel. The write-narrowing
is its first consumer; the carrier, join, and exit-routing are shared and
effect-agnostic (subsume-don't-proliferate, reflect-don't-extend).

Orthogonal-axes note (Architect, on record): option-(b) — needing
cross-context state across a function-return boundary — is why operator
funding was correct and necessary. ABI/spec-touch — needing a new
kernel-visible/wire/spec construct — is a DIFFERENT axis, and this mechanism
does NOT cross it. The capability funding is sufficient; there is no further
specific construct to re-confirm and no spec-enclave round.

## ABI/spec answer: NO (backend-only, zero TCB)

Verified by the Architect from ea0c04eec:

- `RuntimeExpr` (the elaborator->runtime contract, ir.rs:619+) already has
  `Effect` and has NO `ResourceBody` node. `ResourceBody{Ok/Err}` is a Ken
  VALUE (surface type, spec/30-surface/38-ffi-io.md), built in-backend by the
  existing `synthesized_dynamic_alternative(ResourceInvalidOffset)` lane. No
  new erased-IR node is forced — the TCB surface flagged at HS5 stays
  untouched.
- The activation-frame schema (`AbiSlot`/`AbiFrameHeader`/`AbiCarrier`,
  abi.rs) is `pub(in crate::cranelift_backend)`, inert, with `frame_bytes`
  derived from the slot run; it does not leak outside the backend.
  `AbiFrameHeader` is `#[repr(C)]` only for deterministic offsets between the
  program's OWN functions (the internal `(frame_ptr, services_ptr) -> i64`
  convention) — it does not cross to external C. The C ABI
  (crates/ken-runtime/src/activation_abi.rs) carries handles + the resource
  profile, not the activation layout. A frame-layout edit is backend-internal.
- The process exit is a RENDERED existing Ken value via `format_final_export`
  — not a new wire encoding. ir.rs:161 (host-spine role-record wire) is
  unrelated and untouched.

## Mechanism (Architect-ruled evt_fevj1ay5cbwh)

A general cross-context effect-result channel. Three parts, all reuse:

1. CARRIER = reuse `AbiCarrier::ResultWord`. The inner effect-specialization
   already stores its result word to the caller-provided frame Result slot
   (units.rs ~3453). Extend it to carry the effect's FULL result as the
   discriminated `ResourceBody{Ok/Err}` Ken value (the narrowed InvalidOffset
   product on the Err arm). The value is self-discriminating, so NO new frame
   field, NO side table, and NO continuation-identity object is added — the
   existing one-word carrier suffices.
2. ROUTING = reuse the outward-join machinery: `JoinPlanToken`'s closed
   `JoinResultRepresentation::CarrierWord` (joins_traps.rs:69-71, "lowering
   cannot add a third representation") + `SourceJoinTarget`. Route the inner
   specialization's frame-carried `ResourceBody` result to the outer
   effect-tail sink through the planned CarrierWord join, instead of the
   source-machine backward reconstruction. The outer effect tail's transport
   today is InlineNoCall returning the PRE-effect carried word; the channel
   makes the outer sink consume the POST-effect frame-carried value.
3. OUTER SINK = read the frame-carried `ResourceBody{Ok/Err}` and route it to
   the process exit VERBATIM via `format_final_export`, REPLACING
   `dead_arm_effect_trap` (the `RuntimeTrapCode::PatternMatchFailure` lossy
   middle, joins_traps.rs ~54). The trap is replaced for the
   effect-result-carrying path, not bypassed.

GENERALITY: the channel is effect-AGNOSTIC. The only per-effect part is the
specific `ResourceBodyErr` constructor (`ResourceInvalidOffset` for the write;
sockets/network get their own error products via the same
`synthesized_dynamic_alternative` pattern). The carrier + join + exit-routing
are shared — that is the reusable capability.

## Dependency and sequencing

The (i) soundness-hardening increment (fail-closed reject-set repair on the
forward-edge guard) LANDED as squash 553d99a42 (blob-verified). (i)
established the write tail is NON-collapsible = it falls to base. (ii)'s
channel operates on that BASE path (where the effect actually executes and
the frame-carried result lives), routing it to exit — it does NOT resurrect
the forward-edge collapse for effect tails (which stays correctly off them).
So (i) is the clean base (ii) builds on, now on main. This node is RELEASED to
the runtime ring at base 553d99a42.

## Acceptance criteria

- AC-WRITE-NARROWS. The 2 write tests
  (`fs_write_at_malformed_offset_narrows_to_invalid_offset` + the
  without-write-right sibling) un-ignore and go green: the write reaches exit
  as `ResourceBodyErr(InvalidOffset)`, NOT `PatternMatchFailure`.
- AC-EDGE-CONTROL-REKEY. The 8 deferred AC-EDGE-CONTROL-REKEY controls re-key
  onto this increment (the increment that actually un-ignores them), and the
  census-exemption is removed. (Re-keying a #[ignore] to an increment that
  does not un-ignore it is inert — the deferral-marker-inert trap — so these
  ride the increment that lands the write narrowing, per the Steward
  WP-boundary ruling.)
- AC-REUSE-NOT-PROLIFERATE (control). No new `RuntimeExpr` variant; the
  carrier is `AbiCarrier::ResultWord`; the routing is
  `JoinResultRepresentation::CarrierWord`. Assert NO new frame field, side
  table, or continuation-identity object was introduced.
- AC-LOSSY-MIDDLE-REPLACED (control). `dead_arm_effect_trap` /
  `PatternMatchFailure` no longer fires for the write program's
  effect-result path.
- AC-GENERALITY (note/control). The write-specific surface is only the
  `ResourceInvalidOffset` constructor; the carrier/join/exit-routing are
  shared and effect-agnostic, ready for the socket/network consumers.
- AC-NO-REGRESSION. The inc1 read collapse and all non-effect tails are
  unchanged; byte-inertness where applicable is green in CI.
- gate=none, backend-only, zero TCB.

## Honest boundary caveat (carry it)

The ABI = NO answer holds because every current effect result fits a single
carrier word (`ResourceBody{Ok/Err}`). If a FUTURE socket/network effect
result needs multi-word transport, a dedicated frame field is then warranted
— still backend-only, still ABI = NO, but re-flag it to the Architect at that
point so the boundary is re-confirmed. Not needed now.

## Reviewers

Builder: runtime-implementer (option-(b) mechanism, reasoning-dense =
capability-tier T1). Reviewer: Architect (front-loaded the mechanism;
reviews the reuse-not-proliferate controls, the lossy-middle replacement, the
write tests, no-regression, and re-verifies no new erased-IR/ABI construct
crept in). Independent mechanics reviewer: runtime-qa. Plus CI green on the
exact SHA (CODE = full CI; byte-inertness binding). Merge via Steward
M1-M4 -> lieutenant.
