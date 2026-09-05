---
id: ABI-REVOKE-D1
title: "authority-side lineage + admission lease + the path-side revoked error identity — wires RevocationDomain into the shared host dispatcher and turns the revoked-path oracle case green"
status: draft
owner: runtime
size: M
gate: none
depends_on: [ABI-REVOKE-D0]
blocks: [ABI-REVOKE-D2]
github: null
origin: "Architect decomposition of ABI-REVOKE, evt_30z9y81yxvdyv (2026-09-05). Second of three increments (D0 S + D1 M + D2 M). Queued behind ABI-REVOKE-D0; the Steward RE-RELEASES it explicitly once D0 lands — a satisfied dependency is not itself a release. Cut per COORDINATION §2."
---

> # QUEUED — NOT YET RELEASED
>
> This node is `draft` and behind `ABI-REVOKE-D0`. The landing of D0 discharges
> the dependency; it does not authorize a start. The Steward flips this
> `ready`/`active` and kicks the ring after D0 lands. Re-measure every anchor
> against the landed tree at release.

## Objective

Wire the `RevocationDomain` (built in D0) into the single shared host dispatcher
so that guarded operations are admitted against live authority lineage, and give
the path-side denial its distinct `revoked` identity. This is the increment that
first makes revocation observable to a Ken program — it turns the
`revoked-path-operation-is-distinct-fileerror` oracle case green.

## Deliverables (Architect D1, verbatim in substance)

**`D1.1` — authority lineage on the capability slots.** `CapabilityGrantV1`
slots retain a `RevocationNodeId`; `insert` threads the root/attenuated node;
the **host** runtime `attenuate` creates a child, `copy` preserves the node.
**Do NOT reopen the elaborator static contract** (`RevocationHandle` /
`check_revocation_transitive` / `discharge_attenuation`) — I-4; this is the
separate host lineage.

**`D1.2` — admission lease at the dispatch choke point.** In
`dispatch_host_op_v1`, **after grant resolution**, acquire the admission lease
**atomically-with** `is_admissible(grant.node)`:

- **revoke before admission** → return the `Revoked` projection, **no backend
  call**;
- **admission before revoke** → proceed; the real result is never rewritten.

**`D1.3` — the path-side error identity.** Add `FileErrorCauseV1::Revoked` (a new
`IOError` cause beside `Io`/`Capability`), surfaced as `MkFileError <op> <path>
Revoked`. It must **not** collapse into malformed, closed, stale-generation, or
right-not-held.

## Acceptance criteria

**`AC-ORACLE-PATH`** — `seed-capabilities.md` `revoked-path-operation-is-distinct-
fileerror` turns green (it is RED until this increment). **Control:** the
distinct `Revoked` cause is asserted, not merely "an error occurs" — a test that
would still pass under `CapabilityDenied` is insufficient.

**`AC-LEASE-LINEARIZATION` — the in-flight boundary, a non-degenerate pair on a
shared op shape.** revoke-before-admission returns `Revoked` with **no backend
call**, AND admission-before-revoke proceeds and reports the **real** result —
the two states identical in every other respect. A single positive case is
green-vs-green under the exact swap this must catch (`COORDINATION §7`).

**`AC-ADMISSION-COMPLETENESS` — contingent build-time census (ADR-0022
trust-clause 4).** Assert ABI-R3's generated inventory covers every op that
reaches the admission gate, so no guarded op has a path that skips it. A skipping
op is a build break, not a runtime miss.

**`AC-NO-REGRESSION`** — green in CI. Targeted locally: `-p ken-host` (and any
consumer whose closure this dispatch edit changes — `AC-AFFECTED-CLOSURE`: cover
every target that loads a module whose closure the increment changes, diff-touched
or not). Never `--workspace`.

## Banned scope

- **DO NOT build concurrency machinery** (ADR-0022 reserves it for PX12); the
  lease/atomicity is a single-threaded invariant today.
- **Keep `CapabilityTableV1` and `ResourceTableV1` separate** (ADR-0021).
  Resource provenance is D2, not D1.
- **Generation continues to mean close/stale/reuse, never withdrawal.**

## Capability tier

**T2** (executing the Architect's fully-specified admission design under the
Architect's required per-candidate vote-of-record). Reassess at release: if the
admission-linearization correctness reads as invention rather than execution, or
the seat is mismatched, surface the tier to the operator before the kick. This
increment carries a stop rule — inability to represent the admission lease
without a value crossing a fn-return or a new IR variant is a hard stop to the
Architect (it would mean the option-(a) premise is false), not a workaround.
