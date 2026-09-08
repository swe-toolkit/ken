# WP frame — PX9-INC2A (fs surface unification)

> PX9 increment 2, WP-A of two (A then B). Foundation lane, operator ruling
> 2026-09-08 (Pat: do INC2 now — deferring it is tech debt). Builds on the
> landed PX9-INC1 (`ede8b6b72`, the general `SystemError` type + classification
> + laws) and PX9-C (the `/spec` §1.8 anchor). Owning team: foundation. Size: M.
> Capability tier: T1 (mostly mechanical re-threading, but the revoked-identity
> unification + reifier `trusted_base()`-neutrality + the no-flatten boundary
> are soundness-bearing). Gate: none. Architect (`evt_6r9scjqs2qjbg`) is the
> REQUIRED reviewer on the candidate. WP-B (host-wire schema) is releasable
> second and gated on this surface; PX9 stays active until B lands.

## Objective

Make the filesystem domain flow through `SystemError` (the general type INC1
landed) instead of the parallel `FileError`, and unify the `revoked` identity at
the surface — folding `ResourceError.ResourceRevoked` into the one canonical
`IOError.Revoked`. No error semantics change: this is re-representation plus a
single-identity fold, not a reshape. Resource operations KEEP `ResourceError`
(only its revoked identity unifies) — flattening `ResourceError`'s lifecycle
arms into `SystemError`'s flat identity slot is the forbidden semantic change.

## Design judgment (front-loaded; Architect ruling `evt_6r9scjqs2qjbg`)

The original "re-thread ~16 ops onto SystemError" conflates two different
migrations; the Architect split them at the object DB (grounded `e68ecd79`):

- **FS ops migrate CLEANLY.** `FileError = MkFileError FileOperation (Option
  Bytes) IOError` (`prelude.rs:588`) maps field-for-field onto `SystemError`'s
  fs slice: `Operation = FilesystemOp FileOperation`, `ResourceRef =
  FilesystemResource (Option Bytes)`, `IOError`, `+ SafeContext`. Pure
  re-representation, zero semantics change.
- **RESOURCE ops DO NOT.** `ResourceError` (`prelude.rs:1812`) is a RICHER type
  — `ResourceHostIO IOError | Closed | MalformedResource | RightNotHeld Int Int |
  ReleaseFailed ResourceKind ResourceTraceIdentity IOError | ResourceKindMismatch
  ResourceKind ResourceKind | BufferLimit | AllocationFailed | InvalidOffset |
  InvalidBounds | NoProgress | ResourceRevoked`. Its lifecycle arms have NO
  representation in `SystemError`'s single flat `IOError` identity slot, and
  `SystemError`'s `ResourceRef` is a resource DESCRIPTOR (path/fd), not a
  lifecycle error. So resource ops keep `ResourceError`; only its `ResourceRevoked`
  identity unifies. "Re-thread ~16 ops" is really "11 fs ops -> SystemError + 6
  resource ops keep ResourceError with revoked-identity unification only."

## Fixed inputs (measured @ `e68ecd79`)

Current main `5a875142b` is the same on this surface. Line numbers drift;
re-confirm at your D0 branch cut.

- `FileError = MkFileError FileOperation (Option Bytes) IOError` — `prelude.rs:588`.
- `ResourceError` — `prelude.rs:1812` (the 12-arm type above).
- 11 FS ops returning `Result FileError _` (effect block `prelude.rs:2066-2082`):
  `ReadFile, WriteFile, AppendFile, Metadata, ReadDirectory, CreateDirectory,
  RemoveFile, RemoveDirectory, Rename, ChangeMode, PrivateFsOpen`.
- 6 resource ops returning `Result ResourceError _`: `PrivateFsHandleMetadata,
  PrivateResourceRelease, PrivateBufferAllocate, PrivateFsReadAt,
  PrivateFsWriteAt, PrivateBufferFreeze` (plus the wrapping procs
  `resourceMetadata`/`release`/`readAt`/... that thread these) — these KEEP
  `ResourceError`.
- Catalog blast radius for retiring `FileError` (~5 sites, MEASURE at D0):
  `Capability/Filesystem/Authority.ken.md` [2 hits], `Errors.ken.md` [3 hits].
- Wire revoked tags the reifier collapses: `FileErrorCauseV1::Revoked = detail 11`,
  `ResourceErrorV1::Revoked = detail 10`. The wire SCHEMA is UNTOUCHED in WP-A
  (that is WP-B); only the Ken-side reifier's decode changes.
- INC1's `trusted_base()` before/after guard shape (the zero-TCB check) — reuse it.

## Deliverables

1. **A1 (fs migration).** The 11 FS ops return `Result SystemError _`. Default:
   RETIRE `FileError` and migrate its ~5 catalog consumers to `SystemError`
   (blast radius is tiny). FALLBACK if D0 measures the consumer set larger than
   expected: keep `FileError` as a DERIVED fs-view — a total `SystemError ->
   FileError` projection function — never a re-minted parallel inductive. The
   implementer measures the consumer set at D0 and picks; the default is retire.
2. **A2 (resource revoked-identity only).** Retire `ResourceError.ResourceRevoked`
   in favour of the canonical identity via `ResourceHostIO (IOError.Revoked)`.
   `ResourceError`'s lifecycle structure is OTHERWISE UNCHANGED — do NOT flatten
   it.
3. **Reifier.** The Ken-side reifier produces `SystemError` from the UNCHANGED
   wire tags, collapsing the two wire revoked tags (detail 10 + detail 11) into
   the one `IOError.Revoked` at decode. Unify-not-collapse: the same identity is
   preserved (PX9-C §1.8). Wire schema untouched.
4. **Rider (1).** Fold the INC1 test docstring cite fix here: the header cites
   `spec/40-effects/41-system-effects.md §1.8`, which does not exist — §1.8 is in
   `spec/30-surface/38-ffi-io.md`. Comment-only; carries the INC1 approvals.

## Acceptance criteria

- **AC-FS-MIGRATED (the object):** the 11 fs ops' result type is `SystemError`;
  a filesystem error is constructed and projected correctly through the fs slice
  (`FilesystemOp`/`FilesystemResource`/`IOError`). `FileError` is retired (or a
  derived `SystemError -> FileError` view per the D0 measurement), with no
  parallel fs error inductive remaining.
- **AC-REVOKED-UNIFIED (discriminating control):** `ResourceError.ResourceRevoked`
  is gone; a revoked produced at EITHER former origin (fs and resource) names the
  one `IOError.Revoked` identity, which classifies `Permanent`. Control reds if
  either origin lands on a different identity or a neighbour.
- **AC-RESOURCE-LIFECYCLE-INTACT (the forbidden-change guard, load-bearing):**
  `ResourceError`'s non-revoked arms (`RightNotHeld`, `ResourceKindMismatch`,
  `ReleaseFailed`, `BufferLimit`, `AllocationFailed`, `InvalidOffset`,
  `InvalidBounds`, `NoProgress`, `Closed`, `MalformedResource`) are UNCHANGED —
  same arms, same payloads, no flattening into `SystemError`. Control: the arm
  set + payload types are byte-identical but for the removed `ResourceRevoked`.
- **AC-WIRE-SCHEMA-UNTOUCHED:** `crates/ken-host/src/abi_v1.rs` (the wire schema /
  detail tags) is NOT edited in WP-A — the two domain wire Revoked variants and
  their detail tags (10, 11) still exist; only the reifier's decode collapses
  them. (Retiring the wire variants is WP-B.) Control: `abi_v1.rs` diff empty.
- **AC-TCB-NEUTRAL:** the reifier's `trusted_base()` before/after delta is empty
  (INC1's guard shape) — the migration adds no trusted decision. Reviewer-checked
  and structurally guarded, as INC1's was.
- **AC-NO-SEMANTICS:** no retry-classification-model or kernel-law edit; INC1's
  laws and the sole-`RetryGuidance`-producer control are untouched (WP-A adds no
  composed `SystemError -> RetryGuidance` convenience).

## Scope / boundary (pin it)

WP-A is the SURFACE half: the Ken-side types, ops, and reifier decode. It does
NOT edit the host wire schema (`abi_v1.rs`) — that is WP-B. It does NOT flatten
`ResourceError`, add new domains (sockets/processes = PX10/PX11 pure additions),
or touch the classification model / kernel laws. Migration + surface
revoked-unification only. SafeContext follows INC1's reading (byte-free
`NoSafeContext | RedactedSafeContext`: `NoSafeContext` where the op carries no
authority-sensitive bytes, `RedactedSafeContext` where a path/descriptor would
otherwise leak) unless the operator narrows it.

## Contention check

Touches `crates/ken-elaborator/src/prelude.rs` (the fs/resource ops +
`FileError`/`ResourceError` surface), the Ken-side reifier, and the ~5 catalog
`.ken.md` consumers of `FileError`. Foundation is the sole lane on this surface
(runtime is on the kernel chain; language on its own track). No `abi_v1.rs`
(WP-B), no kernel/spec/conformance edit. TCB-neutral under AC-TCB-NEUTRAL.

## Sequencing

Releasable now (INC1 + PX9-C landed). WP-A is releasable INDEPENDENTLY of WP-B;
WP-B (host-wire revoked schema unification, `abi_v1.rs`, TCB-adjacent) is second
and gated on WP-A's surface. The Architect is the required reviewer on both
candidates (B especially — the host boundary). PX9 stays active until WP-B lands;
the four downstream (ABI-S1/S5, PX10/PX11) correctly wait for the unified
type+wire.
