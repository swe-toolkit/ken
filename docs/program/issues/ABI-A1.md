---
id: ABI-A1
title: "promote ConsoleRead and ClockWallNow to NativeTested with differential evidence"
status: merged
owner: runtime
size: L
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
> **D0 REFRAME 2026-09-06 (size M→L):** runtime's D0 found native execution for
> both operations is absent (deferred), so the native leg (`ProcessHost`
> methods + `ken_host_dispatch_v1` decode arms) is folded into this node as
> `D-NATIVE` — subsume, don't proliferate. See the frame's D0 REFRAME block. Node
> stays active; same ring, same `wp/ABI-A1` branch (nothing was started).
>
> **CLOCKWALLNOW ACCEPTED PARTIAL LANDED 2026-09-06** at `266ccd383` (PR #3372,
> CI-green, blob-verified 18/18; Decision `dec_74fax0fkkyhw9`, QA
> `evt_4sm3qx1hzz4d5` + Architect `evt_1h9aa51bn40fm`). It adds real native 0x0201
> Clock execution, the normalized temporal differential, the generalized dead-arm
> trap (Architect ruling A, z3800), and Clock-only NativeTested promotion (13→14)
> with the exact 11-op deferred tail frozen. **Node STAYS ACTIVE** — its objective
> covers ConsoleRead too. The REMAINING increment is **ConsoleRead D-NATIVE**: it
> needs the z3750 governed-leaf owner planner node (host-reply-bytes owner,
> `{PersistentStore}`), which is AUTHORIZED within ABI-A1 (backend substrate, NOT
> TCB — no separate node, no operator call), and it returns to the Architect as
> its OWN fresh-SHA review when cut. ABI-A1 flips to merged only when ConsoleRead
> lands. A2/A3 follow as separate slices.

## Objective

Promote `ConsoleRead` and `ClockWallNow` to `NativeTested` with differential
evidence.

## Why this is its own slice

Track A is split **by evidence shape, not by count.** This slice is
console/clock: **nondeterministic observation**, so it needs a normalized
comparison rather than exact-output equality. That normalization is the whole
judgment content and does not generalize to the other two slices.
