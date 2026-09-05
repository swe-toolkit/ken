---
id: RT-WRITEALL-IO-IDENTITY-COMPLETE
title: "writeAll §1.7.3 mid-stream Io-identity completion on NATIVE — the two host-Io identities A1 left unwitnessed (Unsupported/ENOSYS + BrokenPipe/EPIPE), each FORCED as a GENUINE mid-stream reply after an exact prior Wrote prefix, reified through the REAL derived writeAll, absolute vs LOCKED §38 and co-indexed on both engines. GATE-0 inherited from A1 (same HostResult-value-agnostic K / owner Spec(2), no new construct); small serialized follow-up."
status: closed
owner: runtime
size: S
gate: none
tier: T1
depends_on: [RT-WRITEALL-ERROR-ROUTE-NATIVE]
blocks: [PX8]
github: null
origin: "Steward, 2026-09-05, folding the Architect's PX8 exit-gate completeness flag (evt_5jqte9xgsakyx) into a small serialized follow-up after A1 landed 2168f3ae3. A1 (RT-WRITEALL-ERROR-ROUTE-NATIVE) witnessed the mid-stream Interrupted Io identity on the writeAll route; the two remaining IN-PATH host-Io identities on the positioned WRITE path — Unsupported/ENOSYS and BrokenPipe/EPIPE — stay native-unwitnessed and are REQUIRED for PX8 closure (Architect: they are §1.7 in-path Io identities, inside the bounded exit-gate population). Ruled by the Steward (evt_4ntysm9qqkv5b): land A1 as-is, carry these two as this follow-up rather than voiding A1's votes-of-record. Steward-filed per COORDINATION §2."
---

> # CLOSED — the two mid-stream Io identities WITNESSED on native (Steward,
> # 2026-09-05). Candidate d75a18c23 landed as squash `4b1a0a590` (PR #3333) onto
> # base c9b38b4a2; the touched file crates/ken-verify/tests/px8f_write_partition.rs
> # is blob-identical (7e83322db) between the reviewed candidate and the landed
> # tree, Steward-verified on origin/main. Adversary post-merge NO OBJECTION
> # (evt_2ykzpqe8rc3y5): provenance airtight, zero-TCB (kernel tree 51d04bba6
> # identical base vs landed), §14(5) empty. Unsupported/ENOSYS and
> # BrokenPipe/EPIPE are now each forced as a genuine mid-stream reply after an
> # exact prior Wrote prefix (AC-GENUINE-MIDSTREAM), reified through the real
> # derived writeAll, absolute vs LOCKED §38. This clears the PX8 depends_on edge;
> # PX8 still closes only on the Architect's closure-property re-verification, not
> # on this node going green.
>
> # RELEASED 2026-09-05 to the runtime ring, serialized AFTER A1
> # (RT-WRITEALL-ERROR-ROUTE-NATIVE, landed 2168f3ae3). SAME FILE
> # (crates/ken-verify/tests/px8f_write_partition.rs) -> must serialize behind A1,
> # which is why it is a distinct node and not folded into A1's diff. Base =
> # current main (2168f3ae3). Cheap (S); the last writeAll-route Io identity gap.

## Why this is a serialized follow-up, not part of A1

A1 landed the writeAll §1.7.3 error/termination route and witnessed obs 1/3/4,
including the mid-stream **Interrupted** Io identity that rides that route. The
Architect's completeness flag (evt_5jqte9xgsakyx) is that two further in-path
host-Io identities on the positioned WRITE path stay native-unwitnessed and are
REQUIRED for the PX8 closure property:

- **Unsupported / ENOSYS**, mid-stream, after an exact prior Wrote prefix.
- **BrokenPipe / EPIPE**, mid-stream, after an exact prior Wrote prefix.

These are the SAME derived `writeAll` route A1 closed, at the SAME owner
(ContinuationSpecialization(2)) with the SAME HostResult-value-agnostic
continuation `K`. The B2 separability measurement returned NO
(runtime-implementer evt_1aqmj64zhk9b1, evidence 85d35017): a single positioned
`writeAt` with an injected BrokenPipe/Unsupported refuses at
`StaticResponseDeferred` BEFORE host dispatch, so there is no synchronous
single-op Io identity off the writeAll boundary and no independent B2 node. The
only place these two identities reach reification is mid-stream on the writeAll
route — the same file A1 just touched. Hence: a small serialized follow-up,
same file, behind A1.

## Objective

Extend `crates/ken-verify/tests/px8f_write_partition.rs` so the derived
`writeAll` §1.7.3 route witnesses BOTH remaining in-path host-Io identities on
NATIVE, each reified into checked Ken code, absolute against the LOCKED text of
`spec/30-surface/38-ffi-io.md` and co-indexed (request / span / count / buffer)
to every other value in the same reply, with interpreter agreement:

- +2 interposer modes on the existing LD_PRELOAD pwrite64 path: `brokenpipe ->
  -1/EPIPE` and `unsupported -> -1/ENOSYS`.
- +2 `assert_write_trace` arms, one per identity.

## GATE 0 — inherited from A1, NOT re-run

GATE 0 is DISCHARGED by A1 and is NOT re-measured here: the continuation is the
identical single-shot / handler-bounded error/termination `K`, the owner is the
identical ContinuationSpecialization(2), the drive is the identical current
HostResult in-frame, and `K` is HostResult-VALUE-AGNOSTIC — the two new
identities change only the injected host reply, never the continuation shape,
add no `ir::RuntimeExpr` variant, no frame field, and no value across a cranelift
fn-return. This node is test-only over the landed A1 route. If, contrary to the
inheritance, forcing either identity requires ANY production lowering change,
that is a hard-stop -> Steward -> operator (do not build it under this node).

## Acceptance criteria, each with its control

- **AC-GENUINE-MIDSTREAM (Architect evt_1qrkrxhk15rya, the non-vacuity bar).**
  Each of the +2 modes FORCES its identity as a GENUINE MID-STREAM reply: the
  injected `-1/EPIPE` and `-1/ENOSYS` land on **call 2**, AFTER a real prior
  `Wrote` prefix was durably written, in the obs-4 error-mode shape — NOT a
  first-call refusal (that is the B2 before-dispatch refusal path the
  measurement ruled out). Control: a mutation that moves either injection to
  call 1 (first-call refusal) makes the arm assert against a missing prior
  prefix and REDS; a mutation that drops the prior-prefix accounting REDS.
- **AC-IOIDENTITY-EXACT (obs-4 rigor).** Each new `assert_write_trace` arm
  matches the EXACT `IoErrorIdentityV1` constructor for its identity, the EXACT
  request tuple, the EXACT durable sink prefix, and the EXACT reaching
  syscall-log — not a coarse "some Io error" shape. Control: a mutation that
  swaps the constructor (EPIPE<->ENOSYS, or either to Interrupted), or perturbs
  the request tuple / sink prefix / syscall-log, REDS.
- **AC-COINDEXED-BOTH-ENGINES.** Both identities reify with the reply co-indexed
  (request/span/count/buffer) and interpreter agreement, absolute vs LOCKED §38.
  Control: a co-index-breaking mutation (span.length != count on the prefix)
  REDS on both engines.
- **AC-A1-PRESERVED.** A1's obs 1/3/4 rows, including the mid-stream Interrupted
  identity, stay green and byte-unchanged in mechanism; the +2 modes are
  additive. Control: the existing A1 arms still pass unmodified.
- **AC-NO-REGRESSION.** Re-run the COMPLETE affected-target closure for the
  changed paths (ken-verify writeAll suite + any target that loads a module
  whose closure the added interposer modes touch), scoped by changed PATHS via
  `scripts/ken-cargo`, never `--workspace`. Workspace-green is CI's verdict.

## Gate, reviewer, sequencing, TCB

`gate: none` (zero-TCB — test-only over the landed A1 route; kernel tree stays
`51d04bba…b58b15a8`; no production lowering/dispatch/RuntimeExpr/ABI-wire delta).
Reviewer: **Architect** (REQUIRED — it reviews the candidate, this is NOT a
rubber stamp: the non-vacuity bar AC-GENUINE-MIDSTREAM is the Architect's
evt_1qrkrxhk15rya call and must be verified on the exact SHA) + **runtime-qa** +
CI on the exact SHA, then Steward M1-M4 -> lieutenant. Lane-1 runtime,
serialized behind A1 (same file). EXIT is NOT this node going green — PX8 closes
on the Architect's closure-property re-verification (absolute vs LOCKED §38,
co-indexed, both engines; population re-derived structurally from the closed
effect_v1.rs sums).
