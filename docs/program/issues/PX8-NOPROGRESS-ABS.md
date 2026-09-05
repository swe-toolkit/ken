---
id: PX8-NOPROGRESS-ABS
title: "NoProgress absolute oracle on the INTERPRETER — a component-boundary test asserting a reified ResourceError::NoProgress against LOCKED §1.7.2, the PX8-WROTE-ABS shape. Closes PX8 clause-(b) both-engines for NoProgress (native + host-shared already cover it; the interpreter side has no absolute assertion)."
status: closed
owner: runtime
size: S
gate: none
tier: T1
depends_on: []
blocks: [PX8]
github: null
origin: "Steward, 2026-09-05, filing B1 of the Architect's PX8 closure-property residual decomposition (evt_4dr93v8qdv3tv, verdict evt_56ssrfbr4tt37). Gap G3: the interp reifier maps NoProgress (eval.rs:5077) but no interpreter test asserts a reified NoProgress absolutely; native + host-shared cover it, so clause-(b) both-engines fails on interp. INDEPENDENT of the native carried-value residual (Lane A) — production unchanged, no seam. Steward-filed per COORDINATION §2."
---

> # CLOSED — B1 landed squash d03957492 (Steward, 2026-09-05), blob-identical to
> # the approved candidate d14a3de5. The interpreter NoProgress absolute oracle
> # asserts the reified constructor against LOCKED §1.7.2; runtime-qa APPROVED and
> # Decision dec_38ftemdcpc0qj resolved, exact CI gate complete (runtime-leader
> # evt_2m93nccpsw813). Production byte-unchanged (test-only, #[cfg(test)]). This
> # closes PX8 clause-(b) both-engines for NoProgress. PX8 itself still closes only
> # on the Architect's closure-property re-verification, routed now.
>
> # RELEASED 2026-09-05 to the runtime ring (A1 landed 2168f3ae3; the ring has
> # bandwidth off the A1 critical path). Independent of Lane A — does NOT gate on
> # RT-WRITEALL-ERROR-ROUTE-NATIVE, and touches a DIFFERENT file (interp
> # eval.rs:5077 reify path), so it runs parallel to the critical-path follow-up
> # RT-WRITEALL-IO-IDENTITY-COMPLETE without contention. Base = current main.

## Objective

Add the interpreter half of the NoProgress absolute oracle, exactly the
PX8-WROTE-ABS component-boundary shape, so the closure property's clause-(b)
(both engines) holds for `ResourceError::NoProgress`:

- a test-local zero-write `HostEffectBackendV1`
- -> the real `dispatch_host_op_v1`
- -> the real minted `ResourceErrorV1::NoProgress`
- -> the existing interp reify path (`eval.rs:5077`)
- -> assert the exact `no_progress_id` checked constructor ABSOLUTELY against
  LOCKED §1.7.2 ("write returns zero -> NoProgress, never Wrote").

Production is unchanged; no production seam is added. This is the same
component-boundary evidence the clause-(a) scope ruling (PX8 evt_5h884g6xhtts3)
established for capped-short Wrote.

## Acceptance criteria, each with its control

- **AC-NOPROGRESS-ABS.** The interpreter component-boundary test asserts the
  reified NoProgress constructor absolutely against the §1.7.2 literal. Control:
  a reifier mutation that returns Wrote(0) instead of NoProgress REDS.
- **AC-PRODUCTION-UNCHANGED.** No production lowering / dispatch code changes;
  the test drives the existing reify path only. Control: a differential shows
  the production reifier byte-unchanged.
- **AC-NO-REGRESSION.** Re-run the affected interpreter/eval targets scoped by
  changed PATHS via `scripts/ken-cargo`, never `--workspace`.

## Gate, reviewer, sequencing

`gate: none` (zero-TCB — a test-only component-boundary oracle, no production
change). Reviewer: **runtime-qa** + CI on the exact SHA, then Steward M1-M4 ->
lieutenant. Architect review OPTIONAL (the shape is a landed precedent,
PX8-WROTE-ABS; loop the Architect only if the oracle needs a §1.7.2
interpretation call). Lane-1 runtime, off the critical path. EXIT is the
Architect's PX8 closure-property re-verification, not this node going green.
