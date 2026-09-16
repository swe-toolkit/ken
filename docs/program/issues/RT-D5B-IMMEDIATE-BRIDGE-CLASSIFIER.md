---
id: RT-D5B-IMMEDIATE-BRIDGE-CLASSIFIER
title: "Land the plan-independent immediate-bridge classifier as a standalone module on main, exercised by unit tests over RuntimeExpr that fail without it. Slice 1 of the PR #3676 re-cut: the stack admits no leaf extraction because its dependency root is 17 files and +3194/-648, so the mechanism is re-cut from main with the closed PR's branch kept as a read-only reference."
status: ready
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-16, on operator directive 2026-09-16: 'factor small mergeable pieces out of the long string of commits and merge those... Small achievable pieces, not one monolithic PR.' Architect decomposition ruling evt_2a3q52kafnsvw fixes the Stratum A / Stratum B seam and rules that inert is the WRONG criterion for a first slice. Steward build measurement evt_5rgn3rhnm25ts confirms Stratum A compiles at origin/main 80d3ff78042d47b841ad64165f3ee4f50b524f4c with a non-vacuity control. Frame at docs/program/wp/RT-D5B-IMMEDIATE-BRIDGE-CLASSIFIER.md. Steward-filed per COORDINATION section 2."
---

> ## RELEASED to Team Runtime 2026-09-16 — `ready`, size M, tier T1
>
> **Implementation base is `origin/main` at
> `80d3ff78042d47b841ad64165f3ee4f50b524f4c`.** PR #3676 is CLOSED. Its branch
> `wp/ABI-S6-d5b-file-backed` is kept at
> `0f71ab5b9267781ae1d91bc654011cad42b926af` as a **read-only reference** — it
> is our own work, so there is no clean-room question, but **nothing is cherry-
> picked from it and nothing is stacked on it.**
>
> **Do not lift the branch's `+31` hunk on `static_transition.rs`.** It reads as
> registration and is not: it carries the `InlineBridgeNoCall` enum variant and
> a live `owns_seat` rewrite. This WP's diff to that file is **one line**. See
> the frame's §4 — it is the single most likely way to get this slice wrong.
>
> **`AC-2` is the point of the WP.** Tests that fail without the module, with
> both runs cited. An inert module's green proves only that the crate still
> compiles; the Architect ruled that criterion out explicitly, and the
> measurement backs it — the slice alone emits twelve `never used` warnings and
> nothing else.
>
> **The tests are new authorship.** The 36 commits proved this classifier out
> through the plan, so there are no direct unit tests to carry across. That is
> why this is T1 and M rather than a file move, and it is where the hour goes.
>
> **The dead-code window is RULED and there is no open decision left.**
> Architect `evt_5rbgwyamv4y2n` ruled **(a): accept the twelve warnings, add no
> `#[allow(dead_code)]`** — against this frame's earlier recommendation. **In
> this slice the warnings are not debris, they are the instrument:** they are
> the only live indicator that the module is on no live path, which is the
> property `AC-3` defines the slice by. Frame §7a carries the full reasoning.
>
> **`AC-6` through `AC-9` were added after the ruling and they are where the
> turn goes.** `AC-6` (provenance per case) exists because **`AC-2` measures
> COUPLING, not FAITHFULNESS** — stub the classifier and a test whose
> expectation was read off the classifier still goes red. `AC-7` covers the six
> refusal points, `AC-8` the one flag interaction, `AC-9` is a `D0` precondition.
>
> **The tests are NOT uniformly new authorship** — frame §7 splits semantic
> expectations (must be lifted, with three attested positives cited verbatim)
> from structural ones (author freely). The `StaticHostOperation` cause has zero
> plan-level ancestors and is declared new intent by name.

Read the frame: `docs/program/wp/RT-D5B-IMMEDIATE-BRIDGE-CLASSIFIER.md`.
