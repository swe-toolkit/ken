---
id: RT-D5B-BRIDGE-REALIZATION-PLANE
title: "Land the immediate-bridge realization plane -- the derivation, the plan field, and the feature-gated mutation harness -- on top of the classifier slice 1 landed, exercised by unit tests that fail without it and still NOT wired into the live planning path. Slice 2 of the PR #3676 re-cut."
status: merged
owner: runtime
size: M
gate: none
tier: T1
depends_on: [RT-D5B-IMMEDIATE-BRIDGE-CLASSIFIER]
blocks: []
github: null
origin: "Steward, 2026-09-16, continuing the operator directive 2026-09-16: 'factor small mergeable pieces out of the long string of commits and merge those... Small achievable pieces, not one monolithic PR.' Successor to RT-D5B-IMMEDIATE-BRIDGE-CLASSIFIER, landed 10321a158bc69cf50e0cb753e1096fe10fa43ed1 and verified green on run 35055338045 (26 jobs, 26 success, full mode). Frame at docs/program/wp/RT-D5B-BRIDGE-REALIZATION-PLANE.md, landed 7abb681fab644fc13db4a85129e348cecdcbe6d0. Steward-filed per COORDINATION section 2."
---

> ## MERGED 2026-09-16 at `49e5ebfbe501a80c6ff2cd7104e4115e0696ba49`
>
> **Verified by blob and by tree identity, not by ancestry** — the publisher
> squashes, so a routed commit is an ancestor of nothing. All four paths are
> byte-identical between the approved tip
> `568d5d7eedad3d5ec957b0ee42189019b1cb0710` and `main`, and
> `git merge-tree --write-tree origin/main 568d5d7ee` reproduces `main`'s own
> tree exactly.
>
> ### The defining property holds at the LANDED tree
>
> `publish_immediate_bridge_realization_plan` and its three siblings have
> **no production caller anywhere in the crate** at `49e5ebfbe`, which is the
> property this slice was defined by. `StaticTransitionPlan` derives
> **`Clone` only** — no `PartialEq`, `Hash`, `Ord` or `Debug` — so the new
> `BTreeMap` field cannot alter any comparison, keying or rendering of the
> plan. **The unwired claim is structural, not an inspection.**
>
> ### The section 3 traps, as built
>
> All three stand, verified at the reference; frame §3d is the durable record.
> The candidate avoided `§3a` by construction — it left the `semantic_ir` and
> `responses` re-export blocks alone and authored no classifier re-export —
> which is why `static_transition.rs` came in at 2 hunks rather than the 3 §3
> predicts. **2 is correct for a candidate that avoids the trap.**
>
> **`§3a` is larger than the frame states: NINETEEN live names, not five.**
> The reference's `use` rewrite is two replacement hunks; `semantic_ir`
> deletes 5 and `responses` deletes 14, all 19 live on `main`, and the two
> replacement names have zero hits. Carried into slice 3's frame with the
> per-name counts.
>
> ### Publisher note — THIRD instance of the same defect
>
> ```
> PR #3750                        merged  2026-09-16T06:25:17Z
> pull_request run 35063626351   created  2026-09-16T06:25:15Z, in_progress
> ```
>
> Merged with `--doc-only` on a four-file `crates/` diff, **two seconds after
> its own CI run was created**, bypassing the pre-merge matrix that was the
> sole reason this candidate was sequenced last. Nothing unreviewed reached
> `main` and the post-merge push run classifies from `main`'s own diff.
> **The gap is that it ran after the merge instead of before it.** Third
> instance; recorded on `PUB-DOC-ONLY-UNVALIDATED-AGAINST-ITS-DIFF`, which is
> where the closure lives.
>
> ### Successor
>
> `RT-D5B-LIVE-WIRING` — slice 3, the live wiring, and the last of the re-cut.
>
> The release banner below is retained as the record of what was asked for.
>
> ## RELEASED to Team Runtime 2026-09-16 — `ready`, size M, tier T1
>
> **Implementation base is `origin/main` at
> `7abb681fab644fc13db4a85129e348cecdcbe6d0`**, which is the commit that landed
> this WP's own frame. PR #3676 stays CLOSED and its branch
> `0f71ab5b9267781ae1d91bc654011cad42b926af` stays a **read-only reference** —
> our own work, so no clean-room question, but **nothing is cherry-picked and
> nothing is stacked on it.**
>
> **Slice 1's predecessor is on `main` and verified.** The classifier landed at
> `10321a158` and run `35055338045` completed 26 jobs, 26 success in full mode
> — the workspace verification slice 1 did not get on its first run, because a
> doc-only merge cancelled it. That is closed and the measurement is in hand.
>
> **THREE TRAPS, ALL MEASURED, AND NONE OF THEM IS THE ONE SLICE 1 HAD.** Frame
> §3 carries each with its counts. In brief:
>
> - **§3a — the `use` hunk is a REPLACEMENT.** At the reference it replaces the
>   `semantic_ir` re-export block rather than adding beside it, and the five
>   names in that block are live on `main` (`BoolMatchCaseOrdinals` 7 files,
>   `ConstructorIdentity` 13, `FieldIdentity` 8, `SynthesizedConstructorRole`
>   11, `SynthesizedFixedConstructorRole` 10). **Author the `immediate_bridge`
>   re-export as a NEW block. `AC-4` is the control.**
> - **§3b — the field hunk adds TWO fields and the second does not exist.**
>   `CheckedIhPostCallConsumer` has **zero** hits on `main` under `crates/`, so
>   a verbatim lift **does not compile**. Add one field. `AC-5` is the control.
> - **§3c — slice 1's trap has not gone away.** The `+31` hunk still carries
>   `InlineBridgeNoCall` and the `owns_seat` rewrite. Excluded from slice 1,
>   excluded here, and it belongs to slice 3. `AC-6` is the control.
>
> **Sixty-three of the sixty-nine hunks on `static_transition.rs` are import
> reorganization and unrelated test-support churn** — 38 commits of evolution
> that has nothing to do with this plane. This WP's diff to that file is **3
> hunks**, and to `construction.rs` **1 hunk**. If your diff is bigger than
> that, you are lifting churn.
>
> **`AC-2` is the point of the WP, and `AC-7` is what keeps it honest.** The
> Stratum B functions are `pub(super)` and directly callable from unit tests
> against a constructed `StaticTransitionPlan`, so this slice is tested by
> calling them — **not** by wiring them. **`AC-2` measures COUPLING, not
> FAITHFULNESS**: a test whose expectation was read off the implementation goes
> red against a stub while proving nothing, which is why `AC-7` requires every
> expectation to be declared **semantic** (lifted from an attested ancestor,
> cited verbatim by file and line) or **structural** (authored freely).
>
> **The dead-code warnings are the INSTRUMENT, not debris.** Slice 1's ruling
> (Architect `evt_5rbgwyamv4y2n` — accept the warnings, add no
> `#[allow(dead_code)]`) governs this slice unchanged. They are the only live
> indicator that the plane is on no production path, which is the property
> `AC-3` defines the slice by. `AC-9` requires the delta be stated, not
> suppressed; slice 1's figure was 88 -> 101.
>
> **`construction.rs:1463` IS OUT OF SCOPE and so is `InlineBridgeNoCall`.** A
> candidate touching either is slice 3 and should be **bounced, not reviewed**
> (frame §5). This WP adds the field and its `BTreeMap::new()` initializer —
> forced, because the struct must construct — and stops there.
>
> **Contention:** `static_transition.rs` and `construction.rs` are both
> high-traffic. Coordinate with any in-flight runtime candidate touching
> `planning/`.
>
> **Sequencing:** the runtime ring is mid-`RT-IGNORED-PASSING-ROWS-DISPOSITION`
> (rows 6 and 7 readmitted, 8-11 open). **This node is released, not preempting
> — the runtime leader sequences it.** The two do not contend: the disposition
> node works `crates/ken-cli/tests/` and `ken-host`, this one works
> `ken-runtime/src/cranelift_backend/planning/`.

Read the frame: `docs/program/wp/RT-D5B-BRIDGE-REALIZATION-PLANE.md`.

## Why this is T1 rather than a file move

The seven Stratum B items are about 282 lines at the reference and carry across
largely unchanged. **The hour does not go into moving them.** It goes into
`AC-2` and `AC-7`, as it did in slice 1: the 36 commits proved this plane out
**through the live plan**, so there are no direct unit tests to lift and the
coverage has to be authored against `derive_`, `build_`, `publish_` and
`validate_` directly.

## Local build discipline

Targeted only, through `scripts/ken-cargo`, `-p ken-runtime`. **Never
`--workspace`.** The workspace build, the `--locked` gate and the conformance
suite run in CI. `COORDINATION §12`.
