---
id: ABI-A3
title: "promote FsReadDirectory, FsCreateDirectory, FsRemoveFile, FsRemoveDirectory to NativeTested"
status: active
owner: runtime
size: M
gate: none
depends_on: [ABI-REVOKE, ABI-R3]
blocks: [ABI-S2]
github: null
origin: docs/program/10-linux-abi-completion.md §4 (the ABI-completion program); node filed by the Steward 2026-07-25 on the operator's directive to frame the remaining program. Agents cannot create tracked work (COORDINATION §2).
---

> ## Authority: `10-linux-abi-completion.md` §4 — read that, not this
>
> ## RELEASED 2026-09-07 — the shovel-ready frame is authored.
>
> `docs/program/wp/ABI-A3-directory-mutation-native-promotion.md` carries the
> deliverables, acceptance criteria, fixed inputs (measured at `3d85beb03`),
> negative controls, hard-stop protocol, and contention check. Node flipped
> `draft -> active` on release to the runtime ring; ABI-A2 completed
> (`3d85beb03`), and `depends_on: [ABI-REVOKE, ABI-R3]` are both merged. Build
> from the frame, not this tracker node.

## Objective

Promote `FsReadDirectory`, `FsCreateDirectory`, `FsRemoveFile`,
`FsRemoveDirectory` to `NativeTested`.

## Why this is its own slice, and why it also depends on ABI-R3

Split by evidence shape: **directory mutation**, whose distinguishing difficulty
is **ordering and partial-failure semantics**.

★ **It additionally depends on `ABI-R3`** (`10-linux-abi-completion.md:106`) so
the promotions land against a **derived** inventory rather than a hand-edited
one — the promotion is exactly the moment a hand-maintained list would drift.

⚠ **`ABI-S2` supersedes this slices whole-directory read** where streaming is
the honest shape. Do not entrench whole-directory read as the contract.
