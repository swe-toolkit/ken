---
id: ABI-FSCREATE-RECURSIVE-CONTRACT
title: "FsCreateDirectory's `recursive` flag is structurally inert -- resolve the behavioral contract: give it `mkdir -p` meaning, or retire the dead field"
status: draft
owner: spec
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Registered by the Steward 2026-09-07 from ABI-A3's D0 hard-stop (runtime-implementer evt_4k04typ5a45mg) and the Architect's §6 contract ruling (evt_3hfxk94v9f9n7). ABI-A3 blessed the INERT BEHAVIOR as its promoted NativeTested contract; the Architect explicitly did NOT bless the inert FIELD as an intended-forever contract, and directed that its resolution is a separate Spec-owned behavioral-contract question routed via the Steward -- not ABI-A3's scope."
---

> ## This is a tracker node, not a shovel-ready frame.
>
> It registers a measured contract-completeness gap so it is not lost. The
> behavioral-contract call (give the flag meaning, or retire it) is the Spec
> enclave's (`COORDINATION §4`: behavioral contract is Spec's lane). No frame is
> authored, and no work is released, until the operator's lane roster admits it —
> it queues behind the active lanes. Filed low-priority; nothing is blocked on it.

## The gap

`FsCreateDirectory` (`0x0306`) carries the `recursive:u64` field of
`FsRecursivePathRequestV1` (`abi_v1.rs:200`). At `origin/main = e9fc7fcad` that
field is **structurally inert**: the landed `lib.rs::create_directory(parent,
leaf)` is a single `mkdirat` with no recursive parameter, so the flag has no
branch to reach. `recursive=true` and `recursive=false` behave identically —
single leaf; `NotFound` on a missing parent; `AlreadyExists` on an existing
target — and the interpreter matches.

ABI-A3 promoted `FsCreateDirectory` to `NativeTested` on the honest ground that
**native execution agrees with the interpreter** over this inert behavior
(`native==interpreter`, both `NotFound` on a missing parent). That promotion is
correct and complete; it is a **faithfulness** claim, not a **completeness**
claim about the create semantics.

## Why it still needs a ruling

A promoted op carries an ABI request field with no observable effect. A future
caller passing `recursive=true` expecting `mkdir -p` semantics silently gets
single-leaf `NotFound` instead — a latent surface gap on a shipped op. Two
honest resolutions, and the choice is a behavioral-contract call:

1. **Give the flag meaning** — `recursive=true` creates the missing parent chain
   (`mkdir -p`). This is a **new capability** (a parent-chain primitive beyond
   the landed single `mkdirat`), and would be its own build WP once the spec
   owes it.
2. **Retire the dead field** — if the spec does not owe parent-chain creation,
   the `recursive` field on `FsCreateDirectory` is dead weight and its removal is
   a wire/schema change (arity 3 → 2 for this op), also spec-governed.

Ground the choice against the ABI/FS chapter of the spec and
`docs/program/10-linux-abi-completion.md`; report what the spec settles versus
what it leaves open, the same shape ABI-A3's own `D1` fork used.

## What this is NOT

- **Not ABI-A3's scope.** A3 is a promotion; it does not give the flag meaning
  or grow the ABI. This node is where that question lives. See the A3 frame's §3
  `FsCreateDirectory` clause and `AC-2`.
- **Not a blocker.** A3 completes independently; this node can be resolved before
  or after A3 lands, on the operator's schedule.
- **Not a soundness or TCB item.** No `trusted_base()` delta; a host/ABI
  behavioral-contract completeness question.
