---
id: ABI-A1
title: "promote ConsoleRead and ClockWallNow to NativeTested with differential evidence"
status: active
owner: runtime
size: M
gate: none
depends_on: [ABI-REVOKE]
blocks: []
github: null
origin: docs/program/10-linux-abi-completion.md §4 (the ABI-completion program); node filed by the Steward 2026-07-25 on the operator's directive to frame the remaining program. Agents cannot create tracked work (COORDINATION §2).
---

> ## RELEASED 2026-09-06 to the runtime ring (lane-1).
>
> The shovel-ready WP frame is
> `docs/program/wp/ABI-A1-console-clock-native-promotion.md` — read that, not
> this tracker node. It carries the fixed inputs (measured at `origin/main
> a8ee55b2f`), the front-loaded design (the normalized differential for a
> nondeterministic observation — the whole T1 content), deliverables, acceptance
> criteria with negative controls, banned scope, hard stops, and the contention
> check on `effect_v1.rs`. Released on the operator's 2026-09-05 ABI-A ruling and
> 2026-09-06 concurrence to run Track A now; `depends_on: [ABI-REVOKE]` merged
> (`3e1b21cf1`). A2 (`FsAppendFile`/`FsMetadata`/`FsRename`, path policy) and A3
> (directory mutation, `depends_on ABI-R3`) follow this slice.

## Objective

Promote `ConsoleRead` and `ClockWallNow` to `NativeTested` with differential
evidence.

## Why this is its own slice

Track A is split **by evidence shape, not by count.** This slice is
console/clock: **nondeterministic observation**, so it needs a normalized
comparison rather than exact-output equality. That normalization is the whole
judgment content and does not generalize to the other two slices.
