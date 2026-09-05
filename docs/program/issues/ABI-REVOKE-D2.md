---
id: ABI-REVOKE-D2
title: "resource provenance + close-after-drain settlement — a resource-token-only op cannot bypass revocation, and owned fds close only after admitted leases drain; turns the two resource oracle cases green"
status: draft
owner: runtime
size: M
gate: none
depends_on: [ABI-REVOKE-D1]
blocks: []
github: null
origin: "Architect decomposition of ABI-REVOKE, evt_30z9y81yxvdyv (2026-09-05). Third of three increments (D0 S + D1 M + D2 M); it closes the batch, so ABI-REVOKE completes when D2 lands. Queued behind ABI-REVOKE-D1; the Steward RE-RELEASES explicitly once D1 lands. Cut per COORDINATION §2."
---

> # QUEUED — NOT YET RELEASED
>
> This node is `draft` and behind `ABI-REVOKE-D1`. The Steward flips it
> `ready`/`active` and kicks the ring after D1 lands. The Architect noted D1 and
> D2 may be built as one M increment (they share the dispatch edit) at the ring's
> discretion — that is a sequencing call at D1's release, not a reason to start
> D2 early. Re-measure every anchor at release.

## Objective

Give acquired resources the provenance of the authority that acquired them, so a
resource-token-only operation cannot bypass revocation by holding a resource
instead of a capability; and settle owned OS resources honestly — close only
after all already-admitted leases drain. Turns the two resource-side oracle cases
green and closes the ABI-REVOKE batch.

## Deliverables (Architect D2, verbatim in substance)

**`D2.1` — resource provenance.** `ResourceTableV1` slots retain the acquiring
authority's `RevocationNodeId`. A resource-token-only op
(`FsReadAt`/`FsWriteAt`/metadata) admits against the resource's provenance node
**and its ancestors** — so revocation cannot be bypassed by consuming only a
resource token. Tables stay **separate** (ADR-0021), both consulting the one
domain; generation still means close/stale/reuse, never withdrawal. A duplicated
resource inherits the same node unless a future explicit reauthorization
establishes a different sponsor — **do not invent multi-sponsor "any live grant
wins" semantics.**

**`D2.2` — the resource-side error identity.** Add the nullary
`ResourceErrorV1::Revoked`, distinct from `Closed` / `MalformedResource` /
`RightNotHeld` / `ResourceKindMismatch`. This is the "`Revoked` added downstream
by ABI-REVOKE" arm PX8 deliberately gated — authority-withdrawal, **outside**
PX8's bounded §1.7 population, so it does not reopen PX8, but it **will** require
handling in the closed-sum exhaustive matches (expected, not a regression —
`COORDINATION §7`'s no-`_ =>` completeness discipline).

**`D2.3` — close-after-drain settlement.** An owned fd closes only after all
admitted leases drain; `ReleaseFailed` recorded once (ADR-0021); settlement
failure does not reopen authority. Never close or reuse an fd while an admitted
operation may borrow it.

## Acceptance criteria

**`AC-ORACLE-RESOURCE`** — `seed-capabilities.md`
`revoked-resource-operation-is-distinct-resourceerror` turns green, asserting the
distinct nullary `Revoked` (not `Closed`/`RightNotHeld`).

**`AC-ORACLE-SETTLEMENT`** — `revoke-admission-race-preserves-real-settlement`
turns green: an operation admitted before revoke reports its real settlement, and
the fd closes only after that lease drains. Non-degenerate pair — a revoke landing
one step earlier flips the outcome to `Revoked`-with-no-effect.

**`AC-PROVENANCE-BYPASS` — the bypass is closed.** Holding only a resource token
whose acquiring authority was revoked is denied. **Control:** clear the resource
slot's provenance node and this denial must vanish — proving admission consults
the resource provenance, not only the capability path.

**`AC-EXHAUSTIVE`** — the new `Revoked` arm is handled in every closed-sum match
over `ResourceErrorV1` with **no `_ =>` catch-all** on a completeness-critical
match (`COORDINATION §7`).

**`AC-NO-REGRESSION`** — green in CI; targeted `-p ken-host` plus affected-closure
consumers. Never `--workspace`.

## Banned scope

- **DO NOT build concurrency machinery** — drain is a single-threaded invariant
  today; ADR-0022 reserves the concurrent linearization for PX12 and forbids
  inheriting today's proof by assertion.
- **No multi-sponsor semantics** (see `D2.1`).
- **Generation still means close/stale/reuse, never withdrawal.**

## Capability tier

**T2** (executing the Architect's specified provenance + settlement design under
the Architect's required per-candidate review). Same release-time reassessment as
D1.
