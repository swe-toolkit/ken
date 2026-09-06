---
id: ABI-REVOKE-D2
title: "resource provenance + close-after-drain settlement — a resource-token-only op cannot bypass revocation, and owned fds close only after admitted leases drain; turns the two resource oracle cases green"
status: merged
owner: runtime
size: M
gate: none
depends_on: [ABI-REVOKE-D1]
blocks: []
github: null
origin: "Architect decomposition of ABI-REVOKE, evt_30z9y81yxvdyv (2026-09-05). Third of three increments (D0 S + D1 M + D2 M); it closes the batch, so ABI-REVOKE completes when D2 lands. Queued behind ABI-REVOKE-D1; the Steward RE-RELEASES explicitly once D1 lands. Cut per COORDINATION §2."
---

> # RELEASED 2026-09-06 to the runtime ring (lane-1). ABI-REVOKE-D1 has LANDED
> # and is merged on main (respin 3c017b7f8 / node close 83670602e; kernel tree
> # byte-identical, zero-TCB confirmed). Base = current main 83670602e; re-measure
> # every anchor at the cut (the D1 landing moved lines). Kick evt_3qx9h34dj7n1e
> # (top-level anchor). D1+D2 bundling is OFF — the Architect's "one M increment"
> # note was contingent on releasing them together, and D1 landed on its own, so
> # D2 is its own increment. Released on the incumbent runtime seat
> # (gpt-5.6-sol/high, T1) though framed T2: D2 shares the dispatch edit with D1,
> # the seat is warm from the D1 repair, this closes the batch, and the lane's
> # next objective (PX9) is T1 — over-provision on one closing increment is cheaper
> # than reseat churn (recorded, not escalated). Architect is REQUIRED per-candidate
> # reviewer + Runtime QA on the exact SHA -> Steward M1-M4 -> lieutenant.

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

**`D2.2` — the resource-side error identity.** Add the nullary resource-side
authority-withdrawal arm, distinct from `Closed` / `MalformedResource` /
`RightNotHeld` / `ResourceKindMismatch`. This is the "`Revoked` added downstream
by ABI-REVOKE" arm PX8 deliberately gated — authority-withdrawal, **outside**
PX8's bounded §1.7 population, so it does not reopen PX8, but it **will** require
handling in the closed-sum exhaustive matches (expected, not a regression —
`COORDINATION §7`'s no-`_ =>` completeness discipline).

> **AMENDED 2026-09-06 — Gate-0 constructor-namespace ruling (Architect option B,
> evt_5pm9ve77jqr5e).** The elaborator has ONE flat constructor namespace
> (`data.rs` unconditional insert) + unqualified pattern resolution, so a second
> constructor spelled `Revoked` would shadow `IOError.Revoked` and there is no
> source form to disambiguate — type-directed/qualified resolution (option A) is
> FORECLOSED by the settled flat-namespace spec decision (§1). So the arm takes a
> DISTINCT SOURCE SPELLING: Ken source `ResourceRevoked`, Rust identity retained as
> `ResourceErrorV1::Revoked`, distinct `resource.*` wire identity (next code 10).
> Keep `IOError.Revoked` and every existing IOError match UNCHANGED. Do NOT
> collapse into `ResourceHostIO Revoked`. The duplicate-prelude-definition
> hardening is SEPARATE language debt, not D2 scope
> ([[LANG-CONSTRUCTOR-NAMESPACE-SHADOWING-GUARD]]). Runtime resumes from staged WIP
> `0b313f3bf`; Architect remains required reviewer on the fresh candidate.

**`D2.3` — close-after-drain settlement.** An owned fd closes only after all
admitted leases drain; `ReleaseFailed` recorded once (ADR-0021); settlement
failure does not reopen authority. Never close or reuse an fd while an admitted
operation may borrow it.

## Acceptance criteria

**`AC-ORACLE-RESOURCE`** — `seed-capabilities.md`
`revoked-resource-operation-is-distinct-resourceerror` turns green, asserting the
distinct nullary resource-authority arm (Ken source `ResourceRevoked`; not
`Closed`/`RightNotHeld`, and distinct from `IOError.Revoked`).

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
