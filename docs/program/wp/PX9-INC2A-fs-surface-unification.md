# WP frame — PX9-INC2A (fs -> system additive bridge)

> PX9 increment 2, WP-A of two (A then B). Foundation lane, operator ruling
> 2026-09-08 (Pat: do INC2 now — deferring it is tech debt). Builds on the
> landed PX9-INC1 (`ede8b6b72`, the general `SystemError` type + classification
> + laws) and PX9-C (the `/spec` §1.8 anchor). Owning team: foundation. Size:
> S-M. Capability tier: T1 (the bridge is mechanical, but its totality +
> monotone-under-PX10/PX11 property + the no-producer-rekey boundary are the
> soundness-bearing content). Gate: none. Architect (`evt_6tz8jecxhhkas`)
> reviews as DESIGN-FIT — this is NOT a required-TCB gate, because nothing TCB
> moves in A (see AC-TCB-NEUTRAL). WP-B (all revoked unification, host wire) is
> releasable second and gated on this surface; PX9 stays active until B lands.

## Recut notice (2026-09-08, Architect D0 ruling `evt_6tz8jecxhhkas`)

This frame was RECUT after the foundation D0 census refuted the original WP-A
sizing. The original frame said "the 11 fs ops return `Result SystemError _`"
(a trunk migration). D0 measured that reading at **322 `FileError` lines / 45
active files** and the Architect re-measured and confirmed: the producer is
keyed to the flat `FileError` shape (`SynthesizedFixedConstructorRole::FileError`
at `semantic_ir.rs:95`, planner role sites `aggregates.rs:3304/3356..3392`,
interpreter reifier `file_error_value` at `eval.rs:4458`, erasure ctor pin
`erasure.rs:8062`), so the trunk migration is a producer re-key across ~45
files — TCB-adjacent, not M, not TCB-neutral. **The `SystemError -> FileError`
derived-view fallback is ALSO withdrawn: it is total only while fs is the sole
domain and goes PARTIAL at PX10/PX11.**

**The deciding argument is the frame's own additivity guarantee.** PX9.md
promises each later domain adds `Operation`/`ResourceRef` arms "as pure
additions, never a reshape." A trunk reading (fs ops return `SystemError`;
consumers `match MkSystemError`) BREAKS that: the moment a `SocketOp` arm joins
`Operation`, every fs consumer's match goes non-exhaustive — a reshape at every
new domain. Only keeping fs on its own domain type and injecting UP honors
"pure additions." So the operative design is the injection direction, and all
three trunk readings are rejected.

## Objective

Make the filesystem domain REACHABLE through the cross-domain `SystemError`
trunk (the type INC1 landed) WITHOUT migrating fs off `FileError`. Add one
total, permanent, additive `.ken` bridge `file_error_to_system : FileError ->
SystemError`. `FileError`, all 11 fs op signatures, and the fs reifier/planner
role stay UNCHANGED. Cross-domain uniform handling happens at `SystemError` via
this bridge (and, later, one `*_to_system` injection per domain); domain code
stays total on its own domain type. This is exactly "`SystemError` reaches
beyond filesystem" with zero reshape.

All revoked unification — the Ken-side resource-revoked decode collapse AND the
host-wire schema fold — moves to WP-B (it re-keys a producer role, so the TCB
boundary falls on the WP boundary, not inside A).

## Fixed inputs (measured @ `6b34528df`, Architect re-measured)

Line numbers drift; re-confirm at your D0 branch cut.

- `FileError = MkFileError FileOperation (Option Bytes) IOError`
  (`prelude.rs:588`). UNCHANGED by this WP.
- `SystemError = MkSystemError (FilesystemOp _) (FilesystemResource _) IOError
  SafeContext` — `prelude.rs` decl at 613/615/617/620 (INC1). The bridge is
  authored right after this decl.
- Producer keyed to the flat `FileError` shape (all UNTOUCHED in A):
  `SynthesizedFixedConstructorRole::FileError` (`semantic_ir.rs:95`); planner
  role sites (`aggregates.rs:3304/3356..3392`); interpreter reifier
  `file_error_value` (`eval.rs:4458`); erasure ctor pin (`erasure.rs:8062`).
- `SystemError` has ZERO production consumers today (only the INC1
  classification test) — the bridge is the first.
- INC1 test docstring cite bug (rider): the header of
  `crates/ken-elaborator/tests/px9_system_error_classification.rs` cites
  `spec/40-effects/41-system-effects.md §1.8`, which does not exist — §1.8 is in
  `spec/30-surface/38-ffi-io.md`. Comment-only.

## Deliverables

1. **The bridge (the whole substantive deliverable).** Add to prelude, right
   after the `SystemError` decl, a total kernel-checked function:

   ```
   file_error_to_system (error : FileError) : SystemError =
     match error {
       MkFileError operation resource identity |->
         MkSystemError (FilesystemOp operation) (FilesystemResource resource)
                       identity NoSafeContext
     }
   ```

   Total (`MkFileError` is the sole ctor), permanent (fs is a forever-subdomain
   of system), monotone (survives PX10/PX11 untouched). `SafeContext =
   NoSafeContext` at the bridge — the honest byte-free default; SafeContext
   population is out of INC2A scope.
2. **Rider (1).** Fold the INC1 test docstring cite fix: the header cites
   `spec/40-effects/41-system-effects.md §1.8` -> `spec/30-surface/38-ffi-io.md`.
   Comment-only; carries the INC1 approvals.

## Acceptance criteria

- **AC-BRIDGE-TOTAL (the object):** `file_error_to_system` typechecks and is
  total — the single `MkFileError` arm is exhaustive over `FileError`; a
  constructed `FileError` maps to a `MkSystemError` whose `Operation` is
  `FilesystemOp`, `ResourceRef` is `FilesystemResource`, `IOError` is carried
  unchanged, and `SafeContext` is `NoSafeContext`. Witnessed by an elaborated
  Ken program constructing a `FileError` and projecting through the bridge.
- **AC-NO-FS-SIGNATURE-CHANGE (the boundary that makes A honest):** none of the
  11 fs op signatures change — they still return `Result FileError _`.
  `FileError`'s data is byte-identical. Control: the fs op result-type list and
  the `FileError` decl are unchanged from `6b34528df`.
- **AC-PRODUCER-UNTOUCHED (no reifier/planner widening):** the fs producer role
  is unchanged — `SynthesizedFixedConstructorRole::FileError`, the planner role
  sites, `file_error_value`, and the erasure ctor pin still emit/expect
  `MkFileError`. Control: those Rust sites' diff is empty.
- **AC-TCB-NEUTRAL (vacuous, and that is the point):** `trusted_base()`
  before/after delta is empty because A touches NO host/reifier/planner surface
  at all — the bridge is an ordinary kernel-checked `.ken` function over
  already-produced values. No `crates/ken-host/` edit, no reifier edit.
- **AC-NO-REVOKED-WORK-IN-A:** no `ResourceError.ResourceRevoked` change, no
  wire-tag collapse, no `abi_v1.rs` edit — all of that is WP-B. Control: the
  resource-revoked decode path and `abi_v1.rs` diff are empty in A.
- **AC-NO-SEMANTICS:** no retry-classification-model or kernel-law edit; INC1's
  laws and the sole-`RetryGuidance`-producer control are untouched (A adds no
  composed `SystemError -> RetryGuidance` convenience).

## Scope / boundary (pin it)

WP-A is the ADDITIVE-BRIDGE half only: one `.ken` function plus the comment-only
cite fix. It does NOT migrate fs off `FileError`, does NOT touch the fs
reifier/planner producer, does NOT do any revoked unification (Ken-side decode
collapse or host wire — both WP-B), does NOT edit `abi_v1.rs`, and does NOT add
new domains (sockets/processes = PX10/PX11, each a future `*_to_system`
injection). `SystemError` is the cross-domain trunk each domain injects UP into;
fs is the first injection. SafeContext follows INC1's reading (byte-free
`NoSafeContext | RedactedSafeContext`); the bridge emits `NoSafeContext`
unconditionally in A, and per-op `RedactedSafeContext` population is deferred
(out of INC2A scope) unless the operator narrows.

## Contention check

Touches `crates/ken-elaborator/src/prelude.rs` (the bridge, additively, after
the `SystemError` decl) and one test-file comment. No fs op signature, no
reifier/planner, no `abi_v1.rs`, no kernel/spec/conformance edit. Foundation is
the sole lane on this surface (runtime is on the kernel chain; language on its
own track). Genuinely TCB-neutral under AC-TCB-NEUTRAL (vacuous).

## Sequencing

Releasable now (INC1 + PX9-C landed). WP-A is releasable INDEPENDENTLY of WP-B;
WP-B (all revoked unification — Ken-side decode collapse + host-wire schema,
`abi_v1.rs`, TCB-adjacent) is second and gated on WP-A's surface. The Architect
is design-fit reviewer on A and REQUIRED reviewer on B (the host boundary). PX9
stays active until WP-B lands; the four downstream (ABI-S1/S5, PX10/PX11)
correctly wait for the unified type+wire.
