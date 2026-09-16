---
id: RT-D5B-HOST-FILE-ACQUISITION-SURFACE
title: "Land the host and interpreter file-acquisition surface additively -- ken-host abi_v1/mapping_v1/lib, effect_v1, the PrivateMappingAcquireFile prelude global and the ken-interp dispatch path -- WITHOUT promoting MappingAcquireFile out of the represented-unavailable tail. Slice 4 of the wp/ABI-S6-d5b-file-backed drain and the one remaining cluster orthogonal to the contested HS18 backend surface: it touches zero cranelift files. The availability flip is four coordinated sites, two of them compiler-enforced and one of them inside cranelift_backend, so it is its own later slice gated on the artifact differential the code itself names."
status: ready
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-16, continuing operator directive 2026-09-16: 'factor small mergeable pieces out of the long string of commits and merge those... Focus on small incremental, achievable pieces. Must land as one commit is a trap that leads you to unworkable situations.' Slice 4 after RT-D5B-IMMEDIATE-BRIDGE-CLASSIFIER (slice 1, landed 10321a158), RT-D5B-BRIDGE-REALIZATION-PLANE (slice 2, landed 49e5ebfbe) and RT-D5B-LIVE-WIRING (slice 3, landed 67684fa5d), which together drained PR #3676's cranelift material. Fixed inputs measured by the Steward at backup tip 0d94d58b60e2e7bb045d7204efd138e17df68b7b against merge-base 4bf1ad362b5a5cfa512636087df5229ee57db706. The unflipped disposition is Architect ruling evt_3wtg8w8krmmt, grounded in ken-host/src/effect_v1.rs:249-251 -- 'Membership is a plan, not evidence' -- not in a reviewer's caution. Frame at docs/program/wp/RT-D5B-HOST-FILE-ACQUISITION-SURFACE.md."
---

# READ FIRST — THE AVAILABILITY BIT DOES NOT MOVE IN THIS SLICE

> The backup branch's version of this cluster promotes `MappingAcquireFile`
> from `RepresentedUnavailable` to `NativeTested` and flips catalog row `0407`
> from `unavailable` to `native`. **Those edits are dropped here.** The
> transplant is deliberately not verbatim.
>
> The criterion is the codebase's own, at `ken-host/src/effect_v1.rs:249-251`:
> *"Membership is a plan, not evidence: every operation remains
> `RepresentedUnavailable` until its artifact differential gates promote it
> explicitly."* `D4` (`cf894cdb5`) and `D5a-core` (`29f64ff6f`) being merged is
> a reason to expect that differential to pass; it is not the differential.
>
> **`AC-AVAIL` is stated as four sites, not as "the catalog is untouched."**
> A one-site AC cannot see a partial promotion, and three agreeing classifiers
> with one disagreeing is precisely this defect's shape. Verified at
> `origin/main` `44bfc228d`, all four agreeing at **10 unavailable / 25
> native**:
>
>     1  effect_abi_v1.catalog                       25 native / 10 unavailable
>     2  effect_v1.rs:193                            => RepresentedUnavailable
>     3  NATIVE_TESTED_TARGETS_V1                    [HostOpV1; 25], op ABSENT
>     4  static_transition/effects.rs ~:624   op NAMED in the 10-op => None arm
>
> Sites 3 and 4 enforce themselves — the array length is part of its type, and
> site 4 names its members rather than wildcarding them, so a partial promotion
> fails to compile. The AC exists to catch the two that do not.

# WHY THIS SLICE AND NOT THE POSTCALL SUBJECT

> `RT-D5B-POSTCALL-REFUSAL-MECHANISM` is held on `fn
> checked_ih_post_call_residual`, which reads **0 on `origin/main`** and **1 on
> the backup tip** (positive control: `impl` reads 45 in main's copy of that
> file, so the absence is real rather than an unresolved path). Landing it
> would clear that node's base hold — and it is still **not** the next slice,
> because it is not small. The symbol and its five call sites all sit in
> `cranelift_backend/lowering/core.rs`, inside the HS18 surface the Architect
> is still amending (amendment 8, `388bcd8d3`).
>
> **Site 4 above is the same constraint seen from the other end:** it lives in
> `cranelift_backend`, so the later availability-flip slice cannot be
> backend-free either. The host surface is the piece that is both independent
> and finishable, which is what the directive asks for.

# CONTENTION — ONE LIVE CANDIDATE, TEXTUALLY CLEAN AND SEMANTICALLY NOT

> Measured by the Steward 2026-09-16.
> `wp/LANG-CONSTRUCTOR-NAMESPACE-SHADOWING-GUARD` at
> `b7a9101ac026fcafa8cc6bac0ebfc0c88686dc31` touches
> `crates/ken-elaborator/src/prelude.rs` with a single-line insertion at
> `@@ -1450`. This slice's prelude hunks are at 291, 442, 1884, 2110, 2144,
> 2160, 2185, 2592, 2621, 2878.
>
> **Disjoint regions, and that is the hazard.** Both edit `register_prelude`;
> one adds a constructor-namespace shadowing guard, the other registers a new
> global (`PrivateMappingAcquireFile`). A guard that rejects shadowing
> constructor names is exactly what could reject a newly-added global — and
> because the hunks do not overlap, **no merge conflict will surface it.**
> Whichever lands second discovers it, in CI or not at all. A clean
> `merge-tree` is not evidence on this question.
>
> Whichever lands first, the second candidate owes the confirmation in its PR
> body that `PrivateMappingAcquireFile` survives the guard. The Steward carries
> that obligation to the other candidate.

# D0 IS A SEPARABILITY DETERMINATION AND IT MAY SHRINK THIS SLICE

> `effect_v1.rs` carries **47 hunks** and they are not confined to the
> file-acquisition surface: several rewrite `ResourceTableV1` lifecycle
> internals — admission leases (`finish_admission`), slot states
> (`Vacant`/`Closing`/`Retired`), and `ResourceReleaseReadinessV1`. That is a
> resource-table change sharing a file with a file-acquisition change, which is
> the shape the operator directive exists to break up.
>
> **D0 reports separable or not-separable with the dependency NAMED** — which
> promotion line requires which lifecycle change. "They're intertwined" does
> not discharge it. If separable, the lifecycle hunks leave this candidate and
> become slice 5, and the frame's `+503/-262` shrinks.

## Fixed inputs

Measured at backup tip `0d94d58b60e2e7bb045d7204efd138e17df68b7b` against
merge-base `4bf1ad362b5a5cfa512636087df5229ee57db706`. The branch's whole
residual is **21 commits, +18190/-6067 over 35 files**; this slice is the
separable 9-file part.

    4     0   crates/ken-elaborator/src/compiler_driver.rs
    17   29   crates/ken-elaborator/src/prelude.rs
    4     0   crates/ken-host/Cargo.toml
    1     1   crates/ken-host/effect_abi_v1.catalog      <- DROPPED, see READ FIRST
    49    0   crates/ken-host/src/abi_v1.rs
    329 231   crates/ken-host/src/effect_v1.rs
    10    0   crates/ken-host/src/lib.rs
    59    0   crates/ken-host/src/mapping_v1.rs
    30    1   crates/ken-interp/src/eval.rs
    ---------
    9 files, +503 / -262        cranelift files touched: ZERO (measured)

`AC-NO-BACKEND` turns that zero into a check: the candidate's file set greps
`cranelift` at count 0. It is what makes this slice reviewable without the HS18
closure, and it is the reason the slice exists.

## Acceptance criteria

See the frame. `AC-AVAIL` (four sites, 10/25), `AC-CONTROL` (the promotion's
own test `abi_s6_d5b_promotes_only_file_acquisition_from_the_unavailable_tail`,
at `effect_v1.rs:5411` on the backup tip and **absent from `main`** — measured
0 hits — adjusted to assert the held state rather than deleted), `AC-NO-BACKEND`,
`AC-D0`, `AC-NO-REGRESSION` (workspace-green in CI, never a local
`--workspace` run; local verification targeted to `-p ken-host`,
`-p ken-elaborator`, `-p ken-interp`).

## Reviewers and route

Architect (required soundness reviewer on ABI-S6) + runtime-QA. `crates/` is
touched, so the standing Adversary hunt is independent and M8 applies:
Steward M1-M4, then lieutenant M5-M9.
