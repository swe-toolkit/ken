---
id: ABI-A2
title: "promote FsAppendFile, FsMetadata, FsRename to NativeTested"
status: active
owner: runtime
size: L
gate: none
depends_on: [ABI-REVOKE]
blocks: []
github: null
origin: docs/program/10-linux-abi-completion.md §4 (the ABI-completion program); node filed by the Steward 2026-07-25 on the operator's directive to frame the remaining program. Agents cannot create tracked work (COORDINATION §2).
---

> ## RELEASED 2026-09-06 to the runtime ring (lane-1), the ABI-A track's next
> ## slice after ABI-A1 completed (Clock + ConsoleRead NativeTested, merged
> ## `2db854e97`).
>
> The shovel-ready WP frame is
> `docs/program/wp/ABI-A2-fs-metadata-rename-native-promotion.md` — read that,
> not this tracker node. It carries the fixed inputs (measured at `2db854e97`),
> the front-loaded design (native FS execution is ABSENT, so the native leg is in
> scope — sized L not the stub's M — on a shared scoped-root/rights/no-follow
> resolution substrate; a normalized differential for `FsMetadata`'s volatile
> fields; a state-transition differential for `FsRename`; path-policy negative
> controls as the A2-specific evidence), deliverables `D0`/`D-NATIVE`/`D1`-`D5`,
> acceptance criteria with negative controls, banned scope and hard stops, and the
> contention check on `effect_v1.rs`. `depends_on: [ABI-REVOKE]` merged.
>
> **The path policy is already landed** (`§4` ABI-R1: scoped roots, rights,
> symlink, no-follow "have landed"), so there is NO open design question and no
> enclave route — this WP EXERCISES the policy (`capability.rs`,
> `native_effect_v1.rs`), it does not design it. **`FsMetadata`'s host-reply-bytes
> owner** reuses ConsoleRead's `HostResponseReferent{PersistentStore}` governed
> leaf where the shape matches; a distinct owner, if required, is backend planner
> substrate authorized within this WP (NOT TCB, no separate node, no operator
> call), exactly as ConsoleRead's was. A3 (directory mutation, `depends_on
> ABI-R3`) follows this slice.

## Objective

Promote `FsAppendFile`, `FsMetadata`, `FsRename` to `NativeTested`.

## Why this is its own slice

Split by evidence shape: this is **metadata/rename**, whose distinguishing
difficulty is **path-policy interaction** (scoped roots, rights, symlink policy,
no-follow resolution) rather than nondeterminism or partial failure.
