# ADR 0022 — Runtime revocation membrane (bounded root-space authority projection)

- **Status:** Accepted.
- **Date:** 2026-09-05.
- **Deciders:** Architect, grounding the Architect sizing ruling
  `dec_p1dv4gw6bsc2` and the operator's settled authority strategy; the
  behavioral contract merged separately (PR #865, `dec_hf76w9mvzvt9`).
- **Relates to:** ADR 0004 (states authority is attenuable/revocable but chooses
  no mechanism — this ADR supersedes it on mechanism), ADR 0021 (resource
  lifetime, the sibling `ResourceTableV1`, and the settlement / `ReleaseFailed`
  recorded-once discipline this ADR inherits), ADR 0019 (capability evolution
  and the single process-admission that constructs the unique `ProcessContext`),
  ADR 0017 (scoped-capability trust posture). Discharges the deferral in
  `spec/60-security/62-authority.md` §4.3 and §H ("mechanism deferred →
  `40-runtime`").

## Context

`spec/60-security/62-authority.md` §4 (merged, behavioral) makes authority
**revocable and transitive**: `revoke` closes a capability identity and every
identity attenuated from it, to any depth, but not its parent or siblings;
`attenuate` creates a child identity linked to its parent; copying a capability
preserves that identity; a resource acquired under an authority stays governed
by the same lineage, so consuming a resource token cannot bypass revocation.
Revocation has one semantic denial with two exact public projections
(`IOError.Revoked` via `MkFileError`, and the nullary `ResourceError.Revoked`),
and **admission is the linearization point**: a revoke that wins the race denies
with no guarded backend operation; an admission that wins finishes and reports
its real result, never rewritten. Revocation promises neither rollback nor
cancellation. `spec` §4.3 states plainly that this closes only the current
OS-operation runtime face for Ken's **implicit root execution space**, and that
the runtime representation and isolation argument are a `40-runtime` ADR choice —
this ADR.

The conformance oracle cases
(`conformance/security/capabilities/seed-capabilities.md`:
`revoked-path-operation-is-distinct-fileerror`,
`revoked-resource-operation-is-distinct-resourceerror`,
`revoke-admission-race-preserves-real-settlement`) are `(oracle)` and RED UNTIL
this membrane lands. `attenuate`/`revoke` stay **non-Ken-visible host management
actions** (they resolve as `UnboundName` in Ken; §3.2, I-4); the elaborator
already carries the *static* contract (`RevocationHandle`,
`check_revocation_transitive`, `attenuate`/`discharge_attenuation` in
`crates/ken-elaborator/src/capabilities.rs`), kernel-re-checked, and this ADR
must not reopen it.

The runtime **substrate exists but the revocation operation does not**.
`ProcessContext` (`crates/ken-host/src/abi_v1.rs`) is today the unique,
synchronous, `Box`-owned holder of `CapabilityTableV1` and `ResourceTableV1`,
torn down exactly once. `CapabilityTableV1` (`crates/ken-host/src/effect_v1.rs`)
is append-only (insert/resolve; no bump, remove, or revoke); its
`CapabilityTraceIdentity` is a bare `String` with **no parent edge** — so
transitivity has no runtime substrate yet. `ResourceTableV1` carries a
`ResourceTraceIdentityV1(u64)` with no revocation link. The runtime `store::Space`
is a memory/reclamation unit only; it has no validity cell to flip. So the
membrane is ~90% new runtime design over a settled behavioral contract.

## Decision

Build a **bounded authority projection for Ken's current implicit root execution
space**, owned by `ProcessContext`. Do **not** build general multi-space runtime
realization of surface `space` (`spec/36` §4); that stays deferred.

### The revocation domain — one mutable owner

`ProcessContext` gains exactly **one** host-trusted `RevocationDomain`, the
authority projection of the implicit root space. It is the **only** component
that mutates revocation validity or lineage. It lives on `ProcessContext`, not on
`store::Space` (which stays memory/reclamation): authority and reclamation are
different axes. There is one domain per process, minted at admission (ADR 0019,
before any capability is inserted) and torn down with `ProcessContext`.

### `RevocationNodeId` representation — opaque, monotonic, non-reused

The domain allocates **opaque, monotonic, non-Ken-visible** `RevocationNodeId`
values, never reused within a process. A `RevocationNodeId` is an erased identity
(the same shape discipline as the opaque `CapabilityTokenV1`/`ResourceTokenV1`
slot+generation handles, in a distinct id space) — **no raw pointer or reference
to a validity cell crosses the host/ABI boundary**. Ken never names or observes a
`RevocationNodeId`; it is host-internal bookkeeping.

### Lineage and attenuation — a parent-linked tree

Each grant carries its `RevocationNodeId`. **Copying** a capability preserves the
node identity (same id). **Attenuation** creates a **child node with a parent
link** — the lineage edge that `CapabilityTraceIdentity`'s bare string lacks
today. **Revoking** a node closes that node and **every descendant, to any
depth**, and never its parent or siblings. Descendant closure is a property of
the tree, computed by the domain; it is not inferred from a per-leaf flag alone
(see admission).

### Admission is the linearization point

Every guarded backend operation acquires an **admission lease atomically with a
live-ancestry check**: admission succeeds only while the addressed node **and
every ancestor** are live. A cached leaf-live bit is **insufficient** unless its
invalidation on any ancestor revoke is part of the design (i.e. a revoke must
invalidate every descendant's cached liveness, or admission must re-walk
ancestry) — this ADR requires the ancestry check to be sound, not a stale cache.
The two observable outcomes are exactly separated at admission:

- **revoke before admission** → the operation returns the appropriate `Revoked`
  projection and **no guarded OS backend operation occurs**;
- **admission before revoke** → the admitted operation may finish and returns its
  **real** result; a later revoke does **not** rewrite it, and a side effect may
  already have committed.

Revocation closes new admissions immediately; it promises neither rollback nor
cancellation. Cancellation, if ever added, is a separate operation and cannot be
inferred from revocation.

### Resource provenance — revocation cannot be bypassed through a resource token

Every capability grant slot retains its `RevocationNodeId`, and **every resource
acquired under that authority retains the same node provenance in its resource
slot**. So a later `FsReadAt`/`FsWriteAt`/metadata operation that consumes only a
resource token still admits against the resource's provenance node and its
ancestors — it cannot bypass revocation by holding a resource rather than a
capability. `CapabilityTableV1` and `ResourceTableV1` stay **separate tables**
(ADR 0021): grant authority and object lifetime are different axes that **consult
the same revocation domain**. `generation` continues to mean close/stale/reuse,
**never withdrawal** — withdrawal is the domain's, not the generation counter's.

### Settlement and close-after-drain

An owned OS resource is closed only **after all already-admitted leases on it
drain** — never close or reuse an fd while an admitted operation may still borrow
it. Its close success or `ReleaseFailed` outcome is recorded **exactly once**
under ADR 0021's resource identity and settlement discipline, and **settlement
failure does not reopen authority**. Today's unique synchronous `ProcessContext`
drains immediately in ordinary dispatch, so this is a single-threaded invariant
now; **PX12 / future concurrency must preserve the same admission/lease/drain
contract**.

### Error identity — fixed before PX9, absorbed once

The membrane maps a revoked denial to the two spec-pinned projections and **must
not collapse** either into a neighbour: `IOError.Revoked` (a new `IOError` cause
beside and distinct from `CapabilityDenied`) for path/capability operations, and
the nullary `ResourceError.Revoked` (distinct from `Closed`, `MalformedResource`,
`RightNotHeld`, `ResourceKindMismatch`, `ResourceHostIO _`) for resource-token
operations. This identity is fixed here **deliberately before PX9**, so PX9's
cross-domain `System.Error` absorbs the revoked identity **once** rather than
retrofitting it. This ADR does **not** design `System.Error` (that is PX9 /
Foundation); it only fixes an identity compatible with a later single absorption.

### What stays out (bounded, honest)

- No general runtime realization of surface `space` (`36 §4`); the projection is
  bounded to the implicit root space.
- No Ken-visible `attenuate`/`revoke`; the domain API is host-internal (I-4).
- No cross-space forwarder, controlling space cell, or region lifetime is made
  normative here beyond what the bounded root projection requires.

## Trust statement

This is **runtime-trusted component isolation, not a kernel theorem** — the same
posture as ADR 0021's trust statement. The isolation argument is structural and
closed-world:

1. **One mutable owner.** Revocation validity and lineage are mutated only by the
   single `RevocationDomain` on the unique `ProcessContext`.
2. **Opaque tokens.** Grants and resources carry opaque `RevocationNodeId`
   values, not shared mutable host references; no validity cell crosses the ABI.
3. **Domain-only mutation.** Only the domain flips validity or edits lineage;
   nothing else in the runtime can withdraw or re-grant authority.
4. **Closed admission boundary.** The dispatcher's **generated operation
   inventory** (ABI-R3) is what closes the admission boundary — every guarded
   operation is enumerated and must admit; a new operation that skipped admission
   would be a build break, not a silent bypass.
5. **Lease-gated borrowing.** A backend borrow of an OS resource is reachable
   **only through a live admission lease**, so no guarded backend operation runs
   outside an admitted, live-ancestry window.

The claim is bounded to the current synchronous single-`ProcessContext` face and
is discriminator-tested (the three conformance oracle cases), never kernel
`proved`.

## Decomposition

- **ABI-R3 lands first** (already sequenced: `PX8 → ABI-R3 → ABI-REVOKE`,
  `dec_p1dv4gw6bsc2`). Its generated operation inventory is trust-statement
  clause 4 — the closed admission boundary — and is what makes a new guarded
  operation a build break.
- **ABI-REVOKE** then adds the `RevocationDomain` + `RevocationNodeId` on
  `ProcessContext`, the lineage tree, admission leases with the live-ancestry
  check, resource provenance wiring, close-after-drain, and the two `Revoked`
  host mappings — turning the three `(oracle)` cases green.
- It gates **ABI-A1/ABI-A2/ABI-A3 and PX9**; PX9 absorbs the revoked identity
  once, after this lands.

Sizing is the Steward's, on this ADR.

## Rejected alternatives

- **A validity cell/pointer crossing the ABI.** Rejected: it would put a shared
  mutable host reference behind an opaque token, breaking trust-statement clauses
  2–3. Only opaque ids cross.
- **A cached leaf-live bit without ancestor invalidation.** Rejected as unsound:
  it misses an ancestor revoke; admission must check the full ancestry (or a
  cache must carry a sound invalidation-on-ancestor-revoke proof).
- **Generation-bump as withdrawal.** Rejected: `generation` means
  close/stale/reuse; overloading it with withdrawal conflates object lifetime and
  grant authority, which ADR 0021 keeps separate.
- **A single merged capability+resource table consulting itself.** Rejected: ADR
  0021's separation is retained; both tables consult one domain.
- **Collapsing `Revoked` into `CapabilityDenied`/`Closed`/`ResourceHostIO`.**
  Rejected: the two `Revoked` projections are distinct denial identities per
  `spec` §4.1, and PX9 must absorb them without ambiguity.
- **Closing an fd while an admitted operation may borrow it.** Rejected:
  close-after-drain is required so an admitted operation never borrows a closed
  fd.
- **General multi-space runtime realization now.** Rejected as out of scope: the
  bounded root-space projection is what the current OS-operation face needs;
  full `space` realization stays deferred (`spec` §4.3).
