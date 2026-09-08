---
id: ABI-S1
title: "descriptor completion — seek, truncate, sync/data-sync, flags, duplication under explicit inheritance policy"
status: active
owner: runtime
size: M
gate: none
tier: T1
depends_on: [PX9]
blocks: [ABI-S6]
github: null
origin: "docs/program/10-linux-abi-completion.md §4 Track S (the ABI-completion program). Node filed by the Steward 2026-07-25; framed and released 2026-09-08 on the operator's standing 'keep L1 on ABI/compiler work' direction. runtime-leader confirmed ABI-S1 as the ABI-B entry (evt_34zy31p6pa5d0): dependency-ready on PX9 [merged], the unique stated predecessor of ABI-S6, and the node whose explicit descriptor-inheritance policy is the boundary later mapping (ABI-S6) and process (PX10) work must not retrofit."
---

> # WP frame. Authority: `docs/program/10-linux-abi-completion.md §4` Track S.
> Fixed inputs measured at `origin/main` `5291bb39ce0b2618fe399cdb7038f2ba6b68f398`
> (equivalently the steward/work tip, whose only delta is the doc-only lanes.md
> commit — the ken-host / prelude facts are byte-identical to origin/main). PX9 is
> merged. Re-measure the fixed inputs at pickup; ABI-S3 landed after this snapshot
> so `HostOpV1` is 25 operations, not the 22 in the program §2 table.

## Objective

Complete the synchronous descriptor surface on a held resource handle — seek,
truncate, sync and data-sync, descriptor flags, and duplication under an
explicit inheritance policy — each reporting failure through the merged PX9
`System.Error` and registered in the ABI-R3 derived inventory so that adding an
operation is a build break. Descriptor metadata is already delivered
(`FsHandleMetadata`); this node confirms that boundary rather than re-adding it.

## Design judgment (front-loaded)

1. **The inheritance policy IS the deliverable, not a parameter.** Duplication
   without an explicit inheritance policy is exactly how a descriptor leaks
   across a confinement boundary that was meant to hold it. Today the tree has
   NO explicit policy: every open unconditionally ORs `OFlags::CLOEXEC`
   (`crates/ken-host/src/lib.rs:261`, `:291`), and `try_clone()` (dup) is used
   only as a private helper (`lib.rs:235,341,375,384`) with no surfaced policy.
   The dup operation this node adds MUST take an explicit inheritance policy
   value, deny-by-default (close-on-exec set unless the policy explicitly opts a
   descriptor into inheritance). This is the boundary PX10's declarative spawn
   plan and ABI-S6's mappings must NOT retrofit — it is established here, once.

2. **Errors go through PX9, both layers.** PX9 landed as a Ken/prelude
   `SystemError` (`crates/ken-elaborator/src/prelude.rs:619`,
   `MkSystemError Operation ResourceRef IOError SafeContext`) and a host-wire
   `SemanticErrorV1::File(FileErrorIdentityV1{ operation, relative_path, cause })`
   (`crates/ken-host/src/effect_v1.rs:2787,2800`). Every new operation classifies
   its failures through the single host-neutral mapping `io_error_identity_v1`
   (`effect_v1.rs:2758`), carries `SemanticErrorV1::File` with its own `HostOpV1`
   as the operation identity, and adds a `FileOperation` arm in the prelude
   (`prelude.rs:584`) with an explicit `operation_idempotence` classification
   (`prelude.rs:654`). Raw errno survives only through the deliberate `Other`
   escape (`IOError.Other`, `IoErrorIdentityV1::Other(i32)`); no bare errno and
   no silent fallback. The two-axis retry model (transience × idempotence →
   `retry_guidance`, `prelude.rs:672`) has no error-only `retryable` shortcut —
   do not add one. NOTE: the prelude `FileOperation` and the Rust
   `FsCapabilityOperationV1` (`effect_v1.rs:2726`) are two separate hand-listed
   operation vocabularies; a new op touches each independently.

3. **Registration is by construction, not by hand.** The operation catalog is a
   closed `HostOpV1` enum (`effect_v1.rs:16`) walked by exhaustive wildcard-free
   matches, so a new variant that is not threaded through every site is a compile
   break (`E0004`), never a silent default. Each new operation is threaded
   through the five per-operation sites — the enum variant with an explicit
   `0x….` id, `next_in_inventory` (`:102`), `availability` (`:138`),
   `is_ambient` (`:172`), and its rights + request/reply schema — plus the wire
   codec (`effect_wire.rs:185`) and native realization (`abi_v1.rs`, `lib.rs`).
   Tests assert NAMED memberships and properties, never total counts
   (`catalog_is_closed_and_availability_is_exact`, `effect_v1.rs:3994`).

4. **New operations land `RepresentedUnavailable`; native promotion is a
   separate node.** This follows the landed ABI-S3 precedent (clocks/sleep/
   entropy landed `RepresentedUnavailable`, promotion deferred). It is the
   subsume-don't-proliferate call and it bounds this node to size M: ABI-S1
   delivers the surface, the error vocabulary, the inventory registration, and
   the inheritance policy — NOT the per-operation differential harness. Native
   promotion (flip `availability` → `NativeTested`, add to
   `NATIVE_TESTED_TARGETS_V1` at `:220`, flip the generated catalog status, land
   the differential per op) is a later ABI-A-style node, framed when this lands.
   The internal native helpers already exist (`file.seek`, `set_len`,
   `sync_all`, `try_clone`), so promotion will be cheap — but it is not this WP.

5. **Descriptor metadata is already delivered.** `FsHandleMetadata` (`0x030C`,
   `NativeTested`) is the fstat-shaped descriptor-metadata operation. ABI-S1 does
   NOT add a metadata operation; it confirms `FsHandleMetadata` satisfies the
   objective's "descriptor metadata" and holds the boundary against ABI-S4
   (`statx`-shaped metadata with field-availability bits) — field availability
   is S4, not here.

## Deliverables

Per-operation partial landing is authorized (the group is independent surface
work); land the mechanical operations first and the inheritance-policy dup last
as the reasoning-dense increment. Each deliverable is one operation threaded
through all registration sites (design judgment 3) reporting through PX9
(judgment 2), landing `RepresentedUnavailable` (judgment 4).

- **D1 — seek.** A cursor-position operation (`lseek`-shaped) on a held handle.
  Native helper `file.seek` already exists (`lib.rs:389,398,405`). Represent the
  whence/offset surface as typed values, not a raw int pair.
- **D2 — truncate.** `ftruncate`-shaped set-length on a held handle. Native
  helper `set_len` exists (used today only truncate-to-zero, `lib.rs:397`).
- **D3 — sync / data-sync.** `fsync` and `fdatasync`. The metadata-not-flushed
  distinction MUST be represented, not collapsed to one op — whether that is two
  operations or one operation with a typed mode is the ring's/Architect's call,
  but the two semantics must be separately expressible. Native helper
  `sync_all` exists; `sync_data` is its data-sync sibling.
- **D4 — descriptor flags.** Get/set of descriptor flags (`fcntl`
  `F_GETFD`/`F_SETFD` for the close-on-exec bit; `F_GETFL`/`F_SETFL` for status
  flags, to the extent needed at the floor). The close-on-exec / inheritance bit
  MUST route through the explicit inheritance-policy type of D5, not a raw flag
  integer — D4 and D5 share that type.
- **D5 — duplication under explicit inheritance policy (load-bearing, last).**
  A dup operation taking an explicit inheritance policy, deny-by-default. It
  attaches at the flag-composition sites that today hardcode `CLOEXEC`
  (`lib.rs:261,291`) and adds a policy field to its `CanonicalRequestV1` arm.
  Adding it extends the pinned syscall-fact inventory (`SYS_DUP3` etc. are
  absent from `EXPECTED_ABI_FACT_NAMES`, `lib.rs:1146-1185`). This is the
  Architect-reviewed, soundness-bearing increment.
- **D6 — metadata boundary (confirmation, likely no code).** Assert
  `FsHandleMetadata` satisfies the descriptor-metadata objective and that no new
  metadata operation is added here; `statx` field-availability is ABI-S4. If a
  genuine descriptor-metadata gap is found that S4 does not cover, HARD STOP to
  the Steward rather than widening scope.

## Acceptance criteria (each with its control)

- **AC-INHERITANCE-EXPLICIT (load-bearing).** The dup operation cannot be
  constructed without an explicit inheritance policy, and the default is
  deny-by-default (close-on-exec). Control: a construction path that omits the
  policy is rejected (uncompilable or a stable named refusal); a mutation that
  makes the policy default-to-inherit reddens a named test. Deny-by-default is
  proven, not asserted.
- **AC-INVENTORY-BUILD-BREAK.** Every new operation is registered through all
  five `HostOpV1` sites; removing any one registration is a build break.
  Assertions are named memberships and properties, never total counts. The
  `RepresentedUnavailable` tail sentinel
  (`abi_a3_completion_leaves_only_the_non_track_a_deferred_tail`,
  `effect_v1.rs:4044`) and the pinned ABI-fact / syscall-fact inventories
  (`lib.rs:1146`) are updated to include the new operations. Control: drop a
  registration → the derived-inventory test reds.
- **AC-ERROR-PX9.** Each operation's failure paths carry `SemanticErrorV1::File`
  with the operation's own `HostOpV1` identity on the wire and a `FileOperation`
  arm with an explicit `operation_idempotence` classification in the prelude.
  Control: a path that drops the operation identity or the classification reds a
  named test. No error-only `retryable` shortcut is introduced.
- **AC-REPRESENTED.** New operations land `availability = RepresentedUnavailable`
  and the generated `effect_abi_v1.catalog` status matches; there is no silent
  native fallback. Native promotion is explicitly out of scope (a later node).
- **AC-METADATA-BOUNDARY.** No new descriptor-metadata operation is added;
  `FsHandleMetadata` is asserted to cover the objective and `statx`
  field-availability is left to ABI-S4.
- **AC-AFFECTED-CLOSURE.** Cover every target that loads a changed module, not
  only diff-touched ones: the `effect_v1.rs` consumers in `ken-verify`
  (`imported_catalog_partition_is_exact_and_closed`), `ken-elaborator`
  (`export.rs`/`erasure.rs`), and the generated catalog data file. Green in CI
  is the workspace verdict — build/test locally targeted only, never
  `--workspace` (COORDINATION §12).

## Gate, reviewers, sequencing

`gate: none` (node field). The change is in the host trust boundary
(`ken-host`) plus the prelude, so the MERGE carries TCB at Steward routing.
Adding operations to the existing `HostOpV1` catalog is the authorized
ABI-completion program (§4), NOT a new TCB grab — but the inheritance-policy
design is soundness-bearing. On each candidate: **runtime-leader owns the merge
Decision; Architect required review** (verify the inheritance policy is explicit
and deny-by-default, that fsync/fdatasync stay distinct, and that no new
TCB-growing primitive beyond catalog operations was introduced) **+ Runtime QA**
on the exact SHA, then Steward M1-M4 → lieutenant. **No Decision is required to
RELEASE** this node (the frame is the Steward's, the program is operator-
authorized); the merge Decision is assembled from the Architect + Runtime QA
votes on each candidate.

## Contention

`crates/ken-host` (`effect_v1.rs`, `lib.rs`, `abi_v1.rs`, `effect_wire.rs`) and
`crates/ken-elaborator/src/prelude.rs`, with downstream consumers in
`ken-verify` and `ken-elaborator` export/erasure. Re-measure at pickup. The
concurrent doc track (`library/`, `agent/`) is disjoint. Lane 3 (foundation) is
on `catalog/Parsing` — disjoint from `ken-host`. Lane 2 (language) is on the FO/
Kripke prover in `ken-elaborator` — check for overlap with the prelude and
export/erasure edits, though the prover path is disjoint from the operation
vocabulary; re-measure before touching `ken-elaborator`.

## Hard stop

Route to the Steward if expressing the explicit inheritance policy requires a
new TCB-growing primitive beyond adding operations to the existing host op
catalog (that needs operator authorization); if the fsync/fdatasync distinction
cannot be represented distinctly; or if D6 surfaces a descriptor-metadata gap
that ABI-S4's `statx` scope does not cover. Any of those means the work as
framed is not what the tree needs, not that scope should bend.
