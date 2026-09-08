# WP frame — PX9-INC2B (revoked unification: Ken decode + host wire, atomic)

> PX9 increment 2, WP-B of two (A then B). Foundation lane, operator ruling
> 2026-09-08 (Pat: do INC2 now). Gated on PX9-INC2A's surface (the additive
> `file_error_to_system` bridge). Owning team: foundation. Size: S-M. Capability
> tier: T1. Gate: none. **TCB-ADJACENT** — edits the host wire schema
> (`crates/ken-host/src/abi_v1.rs`) and re-keys a Ken-side producer role.
> **Architect (`evt_6tz8jecxhhkas` / `evt_6r9scjqs2qjbg`) is the REQUIRED
> reviewer on the candidate** (host boundary, soundness-bearing). PX9 stays
> active until this WP lands.

## Objective

Unify the `revoked` identity end to end, atomically, so there is ONE canonical
`revoked` — `IOError.Revoked` (classifies `Permanent`) — instead of the current
physical triplication (`FileErrorCauseV1::Revoked`, `ResourceErrorV1::Revoked`,
and the Ken-side domain arms). Two coupled halves land together:

1. **Ken-side decode collapse.** The resource-revoked reifier re-keys the
   nullary `ResourceRevoked` role to the applied `ResourceHostIO (IOError.Revoked)`
   canonical identity, and the Ken-side reifier collapses the two wire revoked
   tags at decode into the one `IOError.Revoked`. `ResourceError`'s lifecycle
   structure is OTHERWISE UNCHANGED (no flatten).
2. **Host-wire schema fold.** `abi_v1.rs` gains `Revoked` on `IoErrorIdentityV1`;
   both former origins route through `Io(IoErrorIdentityV1::Revoked)`; the two
   domain-specific wire `Revoked` variants + their detail tags (10, 11) retire.

These are cut together (not split A/B further) because the decode collapse
re-keys a producer role: the TCB boundary belongs on the WP boundary, so all of
it — Ken decode + host wire — falls inside this one TCB-adjacent WP, and A stays
genuinely TCB-neutral. (This is the Architect's `evt_6tz8jecxhhkas` correction to
the earlier `evt_6r9scjqs2qjbg` cut, which had the Ken-side decode collapse in A.)

## Fixed inputs (Architect-measured; re-confirm at your D0 branch cut)

Line numbers drift; the implementer re-measures at D0. Grounded at `6b34528df`.

- Wire revoked tags to retire: `FileErrorCauseV1::Revoked = detail 11`,
  `ResourceErrorV1::Revoked = detail 10` (Architect `evt_6r9scjqs2qjbg`).
- `IoErrorIdentityV1` — the identity vocabulary that gains `Revoked`
  (`crates/ken-host/src/abi_v1.rs`).
- Ken-side resource-revoked producer role: the nullary `ResourceRevoked` role
  re-keyed to applied `ResourceHostIO Revoked` — Architect measured ~28
  `ResourceRevoked` lines (`evt_6tz8jecxhhkas`); confirm the exact producer
  sites (semantic-IR role, planner emit, interpreter reifier, erasure ctor pin)
  at D0, the same site family PX9-INC2A's frame lists for the fs producer.
- `ResourceError = ... | ResourceRevoked` (`prelude.rs:1812`, the 12-arm type):
  only the `ResourceRevoked` arm's identity unifies; the other 11 lifecycle arms
  (`ResourceHostIO`, `Closed`, `MalformedResource`, `RightNotHeld`,
  `ReleaseFailed`, `ResourceKindMismatch`, `BufferLimit`, `AllocationFailed`,
  `InvalidOffset`, `InvalidBounds`, `NoProgress`) stay byte-identical.
- INC1's `trusted_base()` before/after guard shape (the zero-new-trust check) —
  reuse it around the reifier decode change.

## D0 must pin (ABI versioning)

Confirm whether retiring detail tags 10/11 and adding `IoErrorIdentityV1::Revoked`
needs an **ABI version bump** or is an **in-place V1 edit**. The tag reassignment
is the schema-version-sensitive part. The frame's design is neutral to the
answer; the implementer pins it at D0 and the candidate states which and why.

## Deliverables

1. **Host wire (`abi_v1.rs`).** Add `Revoked` to `IoErrorIdentityV1`; route
   `FileErrorCauseV1::Revoked` and `ResourceErrorV1::Revoked` through
   `Io(IoErrorIdentityV1::Revoked)`; retire the two domain wire `Revoked`
   variants + detail tags 10 and 11. No new host decode branch — revoked's
   schema PLACE moves from 2 domain slots to 1 identity slot; the errno->identity
   mapping is otherwise unchanged.
2. **Ken-side decode collapse.** Re-key the nullary `ResourceRevoked` producer
   role to applied `ResourceHostIO (IOError.Revoked)`; the reifier decodes both
   former wire origins to the one `IOError.Revoked`. `ResourceError` lifecycle
   otherwise unchanged.

## Acceptance criteria

- **AC-REVOKED-UNIFIED (discriminating control):** a `revoked` produced at
  EITHER former origin (fs `FileErrorCauseV1::Revoked` and resource
  `ResourceErrorV1::Revoked`) decodes/classifies to exactly the one
  `IOError.Revoked` = `Permanent`. A wire round-trip test reds if either origin
  lands on a different identity or a neighbour.
- **AC-RESOURCE-LIFECYCLE-INTACT (the forbidden-change guard, load-bearing):**
  `ResourceError`'s 11 non-revoked arms are byte-identical — same arms, same
  payload types, no flattening into `SystemError`. Control: the arm set +
  payload types differ from `6b34528df` only by the removed `ResourceRevoked`
  arm's re-key.
- **AC-WIRE-TRIPLICATION-GONE:** the two domain wire `Revoked` variants and
  detail tags 10, 11 no longer exist; `IoErrorIdentityV1::Revoked` is their sole
  successor. Control: a census of the wire schema finds one revoked identity.
- **AC-TCB-NEUTRALITY (host adds no new trusted decision):** reviewer-checked +
  structurally guarded — (a) pure information-preserving re-representation, no
  new host decode branch; (b) the round-trip control above; (c) the reifier's
  `trusted_base()` before/after delta is empty (INC1's guard shape). The host
  makes no new trust decision; revoked from either origin names the one identity.
- **AC-ABI-VERSION-PINNED:** the candidate states whether it bumped the ABI
  version or edited V1 in place, with the D0 evidence for the choice.
- **AC-NO-SEMANTICS:** no retry-model / kernel-law edit; no new domains; no
  `ResourceError`-lifecycle flatten; A's `file_error_to_system` bridge untouched.

## Scope / boundary (pin it)

WP-B is revoked unification ONLY, end to end and atomic: the host wire schema
fold + the Ken-side decode/producer re-key. It does NOT migrate fs off
`FileError` (that was rejected — fs stays on `FileError`, reachable via A's
bridge), does NOT flatten `ResourceError`'s lifecycle arms, does NOT add new
domains, and does NOT touch the retry-classification model or kernel laws.

## Contention check

Touches `crates/ken-host/src/abi_v1.rs` (wire schema) and the Ken-side
resource-revoked reifier/producer sites (`prelude.rs` + the reifier/planner/
erasure role family). TCB-adjacent — Architect REQUIRED reviewer + Foundation QA
+ CV (wire-schema/soundness). Foundation is the sole lane on this surface. Gated
on PX9-INC2A landing (A's bridge is A's surface; B needs it present).

## Sequencing

Released SECOND, after PX9-INC2A lands. On the candidate: my M1-M4 (Architect
REQUIRED reviewer, Foundation QA, CV). PX9 stays active until this lands; on its
land the four downstream (ABI-S1/S5, PX10/PX11) unblock — they wait for the
unified type+wire this WP completes.
