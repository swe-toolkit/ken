---
id: RT-WRITEALL-ERROR-ROUTE-NATIVE
title: "writeAll §1.7.3 error/termination route on NATIVE — reify obs 1 (all-full), obs 3 (write-zero -> NoProgress), obs 4 (mid-stream Io error after an exact prefix) through the REAL derived writeAll, absolute vs LOCKED §38 and co-indexed on both engines. The error/termination continuation is DISTINCT from the success carry the FOLD closed; GATE-0-FIRST, option-(b) fork-ready."
status: active
owner: runtime
size: M
gate: none
tier: T1
depends_on: [RT-NATIVE-WRITEALL-SUCCESS-FOLD]
blocks: [PX8]
github: null
origin: "Steward, 2026-09-05, filing A1 of the Architect's PX8 closure-property residual decomposition (evt_4dr93v8qdv3tv, verdict evt_56ssrfbr4tt37) after the PX8 property re-verified NO at main febce9a10. The writeAll error/termination route (px8f_write_partition.rs:355, ken-verify) is IGNORED under a label naming RT-CLOSURE-BOUNDARY-RESIDUAL, which is MERGED — that node discharged the SUCCESS-carry population (px8f_buffer_native). Per the Architect the error/termination continuation is a DIFFERENT continuation than the success carry the FOLD closed, so this is DISTINCT work, not a reopen. Steward-filed per COORDINATION §2."
---

> # RELEASED 2026-09-05 to the runtime ring (idle on pi; runtime-implementer
> # gpt-5.6-sol/high = T1, correct provisioning). Lane-1 forward path and the
> # PX8-closing critical sub-item. Base = current main (1ae3c1675).

## Why this is a new node, not a reopen of RT-CLOSURE-BOUNDARY-RESIDUAL

RT-CLOSURE-BOUNDARY-RESIDUAL is `merged`: it discharged the residual
checked-**success-carry** closure population (`px8f_buffer_native`, the row the
RT-NATIVE-WRITEALL-SUCCESS-FOLD ultimately greened). The writeAll ERROR/
TERMINATION route was left ignored under that same label
(`px8f_write_partition.rs:355`, ken-verify: "a closure cannot cross the
boundary … no durable lane", boundary.rs:1044), and a live `#[ignore]` naming a
merged owner is the tell that the row was never discharged. Per the Architect
(evt_4dr93v8qdv3tv), the error/termination continuation is a DIFFERENT
continuation than the success carry the FOLD closed — so it must NOT default
within-lane on the FOLD's success-path precedent. This node owns it and
re-homes the label.

## Objective

Witness the derived `writeAll` §1.7.3 route on the NATIVE backend across its
three unwitnessed observations, each reified into checked Ken code absolute
against the LOCKED text of `spec/30-surface/38-ffi-io.md` and co-indexed
(request/span/count/buffer) to every other value in the same reply, on BOTH
engines:

- obs 1 — all-full: every chunk writes its full request -> `Wrote(full)`, exact
  running prefix accounting to completion.
- obs 3 — write-zero -> NoProgress: a mid-loop zero-length write reifies
  `ResourceError::NoProgress` (never `Wrote 0`), per §1.7.2 ("write returns 0 ->
  NoProgress").
- obs 4 — mid-stream Io error after an exact prefix: an `IoErrorIdentityV1`
  (the mid-stream Interrupted identity rides this route) reifies AFTER an exact
  byte prefix has been durably written, with the prefix accounted exactly.

Liveness: un-ignore `crates/ken-verify/tests/px8f_write_partition.rs:355`
(`checked_write_all_reaches_full_short_zero_progress_flip_and_error_prefixes`) —
all four observations green on native, interpreter agreement — and re-home its
`#[ignore]` label from the merged RT-CLOSURE-BOUNDARY-RESIDUAL to this node.

## GATE 0 — MANDATORY FIRST, decides within-lane vs operator fork

This is the option-(b) surface the success FOLD's GATE 0 + contingent hard-stop
guarded across three hard stops. Before building recut-A-style, STRUCTURALLY
measure the ERROR/TERMINATION continuation for single-shot / tail-resumptive /
handler-bounded shape — NEVER inferred from a RecursiveBackedge marker, always
from the real WRITE_ALL error-route compile:

- Does the error/termination continuation return, store, indirect-call, or
  escape as a runtime object, or does every live tail stay Ret / one self-call
  within the handler subtree?
- Is there a single static owner of the whole error/termination subtree, with
  the response consumed once per path, no duplication / nested capture?
- Does supplying the error route need a value/handle across a cranelift
  fn-return, a new `ir::RuntimeExpr` variant, a new frame field, or a durable
  closure lane?

WITHIN-LANE (build recut-A: single static owner of the error/termination
subtree, in-frame, error-route effect dispatched locally in that owner with the
current HostResult, zero new construct, no closure/continuation identity across
a return, no piecemeal target/settlement/lexical import) is authorized ONLY if
the measurement holds. FALSIFIED, or the route genuinely needs a durable
closure lane / a new construct -> HARD-STOP -> Steward -> operator. The
Steward queues the operator fork STATED: fund B2F (durable closure lane) OR a
sanctioned continuation-ownership-model extension OR a structural recut of the
write/error-plane decomposition. Treat "a closure cannot cross the boundary /
no durable lane" (the row's own label) as the LIVE hypothesis, not a solved
case.

## Acceptance criteria, each with its control

- **AC-GATE0-ERRROUTE-SINGLE-SHOT.** A structural measurement over the real
  WRITE_ALL error-route compile establishes the error/termination continuation
  is single-shot / tail-resumptive / handler-bounded (or falsifies it and
  triggers the hard-stop). Evidence logged and hashed, not inferred from
  RecursiveBackedge. This AC gates every AC below — do not build the witness
  until it holds.
- **AC-WRITEALL-OBS-1-3-4.** All three observations reify on native through the
  REAL derived writeAll, each asserted ABSOLUTELY against LOCKED §1.7.3/§1.7.2,
  co-indexed, with exact-prefix byte accounting; interpreter agreement. Control:
  a mutation that mis-accounts the prefix (e.g. reports Wrote on a zero write,
  or drops the mid-stream error) REDS.
- **AC-LABEL-REHOMED.** `px8f_write_partition.rs:355` is un-ignored and green;
  no live `#[ignore]` naming the merged RT-CLOSURE-BOUNDARY-RESIDUAL remains for
  this row. Control: reverting the fix restores the exact boundary refusal.
- **AC-BACKBONE-PRESERVED.** P1 (254,1229,FsWriteAt,NoContinuationUnit) stays
  Deferred / main-lowered and OFF the admission ledger; 267/279 specialized;
  the plane-close backbone holds; the arm-3 owner-escape mutation STILL REDS; no
  StaticResponseDeferred escapes its owner. Control: the arm-3 mutation reds.
- **AC-NO-REGRESSION.** Re-run the COMPLETE affected-target closure for the
  changed paths (ken-runtime cranelift lowering + ken-verify writeAll suite +
  any module whose closure the error-route lowering changes), scoped by changed
  PATHS via `scripts/ken-cargo`, never `--workspace`. Workspace-green is CI's
  verdict.

## Gate, reviewer, sequencing, TCB

`gate: none` **contingent** — zero-TCB IF GATE 0 holds and recut-A stays
in-frame (no kernel/spec/conformance/RuntimeExpr/ABI-wire delta; kernel tree
stays `51d04bba…b58b15a8`). If GATE 0 falsifies or the route needs a new
construct / durable closure lane, that is a TCB/scope fork -> HARD-STOP ->
Steward -> operator (NOT built under this node). Reviewer: **Architect**
(required — self-contained re-review of the GATE-0 result + the installed
single-static-owner error-route candidate, exactly as the success FOLD) +
**runtime-qa** + CI on the exact SHA, then Steward M1-M4 -> lieutenant. Lane-1
runtime; the critical sub-item of the PX8 closure residual and the one most
likely to fork. EXIT is NOT this node going green — PX8 closes on the
Architect's closure-property re-verification (absolute vs LOCKED §38,
co-indexed, both engines; the population re-derived structurally from the closed
effect_v1.rs sums).
