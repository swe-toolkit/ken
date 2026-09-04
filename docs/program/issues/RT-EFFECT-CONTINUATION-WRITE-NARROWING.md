---
id: RT-EFFECT-CONTINUATION-WRITE-NARROWING
title: "Materialize closed effect-response planes through existing response owners: execute each host effect synchronously, resume its exact continuation once through the existing activation frame and Result slot, and preserve deferred ordinary lowering for open planes. First consumer: composed read-then-write narrowing returns ResourceBodyErr(InvalidOffset) instead of trapping at ResourceBodyResult."
status: active
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Operator-funded option (ii), corrected by the Architect's HS6 ruling on 2026-09-04. The initial ResultWord-routing premise was falsified by measurement: the available InlineNoCall word is the seven-field pre-effect environment, not a post-effect ResourceBody. Gates A and B then established exactly-once synchronous resumption and an existing-slots-only implementation. ABI/spec = NO, backend-only, zero TCB."
---

> # ACTIVE — IMPLEMENTED, AWAITING REVIEW
>
> The soundness-hardening predecessor landed as `553d99a42`; this work was
> measured and built on that exact base. The original post-effect ResultWord
> routing premise was falsified before implementation. The corrected mechanism
> is execute-then-resume through existing response-owner functions, activation
> frames, Parameter/Capture inputs, and the existing Result slot.
>
> No `RuntimeExpr` variant, frame field, side table, reified continuation,
> continuation-identity object, ABI lane, wire form, or spec surface is added.

## Why this is a capability, not a point fix

The native-ABI campaign still has sockets and networking ahead, and nested
host-effect continuations recur across those families. Accurate error products
are critical agent-facing guidance: the runtime must return the exact
`ResourceBodyErr(InvalidOffset)` rather than a secondary
`PatternMatchFailure` caused by skipping the continuation that performs the
narrowing.

The implementation therefore classifies response planes structurally. It does
not list `FsReadAt`, `FsWriteAt`, resources, or any other operation. A plane gets
execute-then-resume only when it is closed and at least two producer groups have
exclusively predeclared transport sources. An open plane remains
conservatively Deferred rather than receiving an unsound partial specialization;
a single-stage plane retains the already-proven forward-Ret path.

## Corrected HS6 measurement

### Gate A: resumption is single-shot and structured

Finished CLIF for `verify_static_response_finished_body` contains exactly one
direct call to the outer `FsOpen` response owner's exact K context. The call is
after host-response validation and before Ret validation, the Result store, and
function return. Existing verifier mutations red on zero K calls, two K calls,
a call before validation, and a call after answer collapse.

`lower_process_host_effect` and `call_declared_unit_target` use synchronous
direct Cranelift calls. There is no indirect callback, future, waker, yield, or
post-return resume path. `SourceControl` and `SourceContinuation` are move-only;
branch construction clones only a terminal-removed prefix and creates distinct
non-`Clone` predecessor edges for mutually exclusive CFG arms. Function-local
slots stay live through the nested call, so ordinary return gives LIFO teardown
before the caller reads Result and returns.

### Gate B: existing frames and slots are sufficient

The generated continuation contexts already exist in the same function plane.
Their ordinary arguments plus worker captures exactly fill their planned
Parameter/Capture headers. The relevant instances measured at the released base
were:

| function / specialization / body | ordinary inputs | captures | context header |
|---|---:|---:|---:|
| fn45 / 0 / 1075 | 12 | 9 | 12 / 9 |
| fn46 / 1 / 979 | 14 | 9 | 14 / 9 |
| fn47 / 2 / 888 | 11 | 8 | 11 / 8 |
| fn48 / 3 / 1238 | 8 | 6 | 8 / 6 |
| fn49 / 4 / 676 | 10 | 6 | 10 / 6 |

Each ordinary envelope is its constructor word followed by `WorkerCapture`
ordinals, and `retargeted_worker_body` is the exact body. Existing
response-owner lowering already assembles those Parameter/Capture inputs, calls
the declared context synchronously, and returns through the existing Result
word. Nothing must persist across a return beyond the ordinary callee Result
slot.

## As-built mechanism

The implementation is a planning correction, not a new lowering construct.

1. Phase A continues to build a context demand for every response with a K unit
   and records unit-less responses as P1 Deferred rows.
2. After the first aggregate-ownership and checked-IH transport derivation,
   phase B identifies transport-source K identities. It admits
   execute-then-resume only when the plane has no P1 row and at least two
   producer groups have exclusively predeclared transport sources. A
   producer that also has a specialization- or fusion-owned source is mixed-owner
   fan-out, not an exclusively ordinary stage. All former P2 demands in that
   eligible plane become
   Specialized. Their existing checked-IH
   transport emissions are consequently real incoming response-owner calls
   rather than dormant pre-effect transports.
3. Response-owner assignment can change which closure environments cross an
   emitted boundary. Phase B therefore rebuilds aggregate ownership and
   checked-IH environment transports before inheritance is derived, validates
   both planes, and requires the transport-source identity population to remain
   unchanged. A changed population fails closed as a circular derivation.
4. Existing response-owner lowering performs the host effect, validates its
   response, invokes the exact K context once, stores the returned word in the
   existing Result slot, and returns synchronously. For the first consumer this
   materializes buffer allocation and `FsReadAt`, then enters the `FsWriteAt`
   continuation and narrows the malformed offset before host write dispatch.
5. If any response is unit-less P1, phase B retains all transport sources as P2.
   This prevents a `StaticResponseDeferred` placeholder from escaping a
   partially specialized mixed plane. A single-stage closed plane also retains
   P2, preserving inc1's forward-Ret route. The existing `writeAll` fixture is
   the open-plane control, the read fixture is the single-stage control, and the
   two read-then-write fixtures are the eligible composed controls. The
   discriminator is structural and
   operation-agnostic.

The lowerer, activation-frame schema, join representations, runtime IR, host ABI,
and wire formats are byte-unchanged. `dead_arm_effect_trap` remains in place for
genuinely unreachable arms; it simply no longer receives the skipped
read-then-write path.

## Acceptance criteria and evidence

- **AC-WRITE-NARROWS.** Both write parity tests are live and green. Native and
  interpreted execution return exact `InvalidOffset`; native has no terminal
  trap. The malformed request does not dispatch `FsWriteAt`.
- **AC-MATERIALIZATION.** Native and interpreted traces each contain exactly one
  `BufferAllocate` and one `FsReadAt`. Suppressing the P2-to-Specialized
  classification restores the exact pre-effect `ResourceBodyResult` trap and
  removes both prefix effects, proving the planner change is causal.
- **AC-SINGLE-SHOT.** Existing finished-body controls red on omitted K,
  duplicated K, K before host validation, and K after answer collapse. The live
  parity trace observes the ordered prefix exactly once.
- **AC-REUSE-NOT-PROLIFERATE.** The code diff adds no runtime expression,
  activation-frame field, side table, continuation object, join representation,
  ABI, wire, or spec construct. It changes response classification and rebuilds
  only existing derived planning planes.
- **AC-GENERALITY.** The eligible read-then-write plane specializes all has-K
  transport sources. The single-stage read plane retains the inc1 forward-Ret
  path, while mixed `writeAll` contains both P1 and retained P2 and still
  compiles. No classifier names an effect operation.
- **AC-EDGE-CONTROL-REKEY.** All eight historical generated-entry capsule
  controls now mutate distinct execute-then-resume response-edge facts and run
  live. Their eight ignored-test exemptions are removed.
- **AC-NO-REGRESSION.** The inc1 read collapse, inc3(i) fail-closed purity guard,
  response-owner mutation suite, byte-inertness controls, runtime library suite,
  and CI shard-union inventory remain green.

## Honest boundary

The current admissible subset is plan-wide: every response must have a
continuation unit, and at least two producer groups must have exclusively
predeclared transport sources. A program with any unit-less response retains its
transport
sources as P2 and uses ordinary lowering. This is a conservative residual, not a
claim that unrelated open and closed components are already separated.
Component-local closure can subsume this guard later if a real consumer requires
it; it must preserve inc1's pure forward-Ret path, the no-partial-specialization
failure mode, and the operation-agnostic classifier.

## Reviewers

Builder: runtime-implementer. The Architect reviews the corrected existing-slots
mechanism, composed-plane guard, single-shot and no-new-construct controls,
write-narrowing parity, and no regression. Runtime QA independently verifies the
mechanics and mutation controls. Full CI runs on the exact candidate SHA before
merge through the Steward and lieutenant path.
