# WP frame — `RT-D5B-HOST-FILE-ACQUISITION-SURFACE`

**Owner:** Team Runtime · **Size:** M · **Risk:** low (strictly additive after
the §4 ruling — the availability bit does not move, and two of the four sites
that would have to move are compiler-enforced) · **Tier:** T1 · **Gate:** none ·
**Deps:** none — touches no cranelift file and does not depend on the HS18
closure

**Origin:** operator directive 2026-09-16: *"factor small mergeable pieces out
of the long string of commits and merge those... Focus on small incremental,
achievable pieces. 'Must land as one commit.' is a trap that leads you to
unworkable situations."* Slice 4 of the `wp/ABI-S6-d5b-file-backed` drain, after
`RT-D5B-IMMEDIATE-BRIDGE-CLASSIFIER` (slice 1, `10321a158`),
`RT-D5B-BRIDGE-REALIZATION-PLANE` (slice 2, `49e5ebfbe`) and
`RT-D5B-LIVE-WIRING` (slice 3, `67684fa5d`).

## 1. Objective

Land the **host and interpreter** side of file-backed mapping acquisition:
the `MappingAcquireFile` operation's host surface, its elaborator prelude
global, and its interpreter evaluation path.

This is the one remaining cluster on the branch that is **orthogonal to the
contested backend work.** Slices 1-3 drained PR #3676's cranelift material;
this slice touches no cranelift file at all, so it can be reviewed and merged
without waiting on the HS18 closure.

## 2. Fixed inputs, measured

Measured by the Steward at backup tip
`0d94d58b60e2e7bb045d7204efd138e17df68b7b` against merge-base
`4bf1ad362b5a5cfa512636087df5229ee57db706`, which is the base the branch
actually forks from. **The residual the branch still carries as a whole is 21
commits, +18190/-6067 over 35 files** — this slice is the separable 9-file part
of it.

    4     0   crates/ken-elaborator/src/compiler_driver.rs
    17   29   crates/ken-elaborator/src/prelude.rs
    4     0   crates/ken-host/Cargo.toml
    1     1   crates/ken-host/effect_abi_v1.catalog
    49    0   crates/ken-host/src/abi_v1.rs
    329 231   crates/ken-host/src/effect_v1.rs
    10    0   crates/ken-host/src/lib.rs
    59    0   crates/ken-host/src/mapping_v1.rs
    30    1   crates/ken-interp/src/eval.rs
    ---------
    9 files, +503 / -262        cranelift files touched: ZERO (measured)

**The zero is a fixed input, not a hope.** `git diff --name-only` over this
file set greps `cranelift` at count 0. If a candidate's file set puts a
cranelift path in this slice, the cut was wrong and the slice stops.

### 2a. The `-262` is NOT bookkeeping, and the frame will not pretend it is

`effect_v1.rs` carries **47 hunks**, and they are not confined to the
promotion. Several rewrite `ResourceTableV1` lifecycle internals — admission
leases (`finish_admission`), slot states (`Vacant`/`Closing`/`Retired`), and
release readiness (`ResourceReleaseReadinessV1`). **Those are a resource-table
change that happens to share a file with a file-acquisition change**, which is
exactly the shape the operator directive exists to break up.

The promotion's own deletions are small and legible:

    -  Self::MappingAcquireFile => HostOpAvailabilityV1::RepresentedUnavailable
    -  pub const NATIVE_TESTED_TARGETS_V1: [HostOpV1; 25]     (array grows by one)

⇒ **D0 of this WP is a separability determination, and it may shrink the
slice.** See §3 D0. Do not assume the resource-table material has to ride
along; do not assume it can be left behind either. Measure it.

## 3. Deliverables

**D0 — SEPARABILITY, and it gates the rest of the cut.** Determine whether the
`ResourceTableV1` lifecycle hunks in `effect_v1.rs` are independent of the
`MappingAcquireFile` promotion. Report one of:

- **Separable** — then this WP is the promotion only, the lifecycle hunks
  become slice 5, and §2's numbers shrink. This is the preferred outcome and
  the one the directive points at.
- **Not separable, with the reason named** — the promotion depends on the new
  lifecycle states. Then they land together and D0's answer is the
  justification for a slice this size.

A bare "they're intertwined" does not discharge D0. Name the dependency: which
promotion line requires which lifecycle change.

**D1 — the host surface.** `abi_v1.rs`, `mapping_v1.rs`, `lib.rs`
(`resource_raw_fd_v1`), and the `px8-ds-test-support` feature in `Cargo.toml`.

**D2 — NOT the availability promotion.** The backup branch's version of this
cluster flips `MappingAcquireFile` to `NativeTested` and catalog row `0407` to
`native`. **Those edits are dropped from this slice** per §4's ruling; the
transplant is not a verbatim copy and the four sites in `AC-AVAIL` are what
says so. The control test in `AC-CONTROL` must be adjusted to assert the held
state, not deleted.

**D3 — prelude and interpreter.** The `PrivateMappingAcquireFile` global and
the `ken-interp/src/eval.rs` dispatch path.

## 4. RULED: LAND IT UNFLIPPED. The availability bit does NOT move here.

**Architect ruling `evt_3wtg8w8krmmt`, 2026-09-16.** The slice lands the
surface additively; `MappingAcquireFile` stays `RepresentedUnavailable` and the
catalog row `0407` stays `unavailable`. **The ruling is grounded in the
codebase's own stated criterion, not in a reviewer's caution** —
`ken-host/src/effect_v1.rs:249-251`:

    /// PX5's intended promotion set. Membership is a plan, not evidence: every
    /// operation remains `RepresentedUnavailable` until its artifact differential
    /// gates promote it explicitly.

⇒ Promotion requires an **artifact differential, per operation, explicitly.**
An unproven flip is not a judgment call that could go either way; it is the
thing that sentence forbids. `D4` (`cf894cdb5`) and `D5a-core` (`29f64ff6f`)
being merged is a reason to *expect* the differential to pass. It is not the
differential.

### 4a. The flip is FOUR coordinated sites, and two of them are compiler-enforced

Verified by the Steward at `origin/main` `44bfc228d`, independently of the
ruling:

    1  effect_abi_v1.catalog            25 native / 10 unavailable
    2  effect_v1.rs:193                 MappingAcquireFile => RepresentedUnavailable
    3  NATIVE_TESTED_TARGETS_V1    [HostOpV1; 25], MappingAcquireFile ABSENT
    4  static_transition/effects.rs:~624   10-op `=> None` arm, op NAMED

All four agree at 10/25. Two enforce themselves: site 3's length is part of the
array's type, and site 4 names its members rather than wildcarding them — its
own comment says *"naming them is what makes promoting one to the admitted set
a compile error here rather than an operation whose seats silently answer
`None`."* A partial promotion does not compile.

**Site 4 lives in `cranelift_backend`.** That is a sequencing fact worth
carrying: **the later flip slice cannot be backend-free**, so it cannot be cut
while the HS18 surface is contested the way this slice can. This is the
structural reason the surface and the flip are two slices rather than one
reviewer's preference.

**One precision, from the Architect's census and worth repeating so nobody
over-reads it:** the `Op::MappingAcquireFile` occurrence at `effect_v1.rs:147`
and the arm at `:442` are **not** availability discriminators — `:442`
co-lists `ConsoleRead`, which is `native`. Only site 4's arm tracks the
unavailable set. An arm that merely mentions the op proves nothing about its
availability.

### 4b. This classifier has already failed once, in the safer direction

`effect_v1.rs:152-157` records it:

    /// This was a membership test against the native set with an `else`
    /// fallback, so a new operation was silently classified
    /// `RepresentedUnavailable` -- a plausible-looking default, which is what
    /// let it survive review.

That failure was *toward* unavailable and still earned a soundness comment and
an exhaustive rewrite. A flip on an unproven backend fails the other way: a
loud `OperationUnavailable` refusal becomes a native execution failure at a
site advertising support.

## 5. Acceptance

**AC-AVAIL (the bar). ALL FOUR availability sites are unchanged, and the AC is
stated as four sites rather than as "the catalog is untouched."** A one-site AC
cannot see a partial promotion, and three agreeing classifiers with one
disagreeing is exactly this defect's shape.

    1  effect_abi_v1.catalog       byte-identical to its origin/main blob
       control: git rev-parse <cand>:crates/ken-host/effect_abi_v1.catalog
                == the origin/main blob   (and row 0407 still reads `unavailable`)
    2  effect_v1.rs                MappingAcquireFile => RepresentedUnavailable
    3  NATIVE_TESTED_TARGETS_V1    still [HostOpV1; 25], MappingAcquireFile absent
    4  static_transition/effects.rs  MappingAcquireFile still named in the
                                      10-op `=> None` arm

Counts stay at **10 unavailable / 25 native** on every side. Note sites 3 and 4
are compiler-enforced, so a candidate that breaks them fails to build rather
than failing review — the AC exists to catch the two that are not.

**The flip is a later slice and its AC is the artifact differential the code
already names** (`effect_v1.rs:249-251`), not a reviewer's assessment that the
backend looks ready. That slice necessarily touches `cranelift_backend` (site
4), so it cannot be cut while the HS18 surface is contested.

**AC-CONTROL. The promotion's own control test rides with it.**
`abi_s6_d5b_promotes_only_file_acquisition_from_the_unavailable_tail`
(`effect_v1.rs:5411` on the backup tip) is **absent from `main`** — measured 0
hits. It must be present and passing in the candidate, and under the **hold**
branch of `AC-AVAIL` it must be adjusted to assert the held state rather than
deleted. A control that is dropped because it contradicts the chosen branch is
the defect, not the fix.

**AC-NO-BACKEND. The candidate's file set contains no cranelift path.**
Control: `git diff --name-only origin/main..<cand> | grep -c cranelift` is `0`.
This is what makes the slice reviewable without the HS18 closure, and it is the
reason the slice exists.

**AC-D0. D0's determination is recorded in the PR body** with the named
dependency or the named independence. If separable, the lifecycle hunks are
**not** in this candidate.

**AC-NO-REGRESSION. Workspace-green in CI**, not a local `--workspace` run.
Local verification is targeted only: `scripts/ken-cargo` scoped to
`-p ken-host`, `-p ken-elaborator`, `-p ken-interp`.

## 6. Contention

`crates/ken-host/`, `crates/ken-elaborator/src/prelude.rs`,
`crates/ken-elaborator/src/compiler_driver.rs`, `crates/ken-interp/src/eval.rs`.

`prelude.rs` and `compiler_driver.rs` are the two files outside Runtime's usual
surface.

**CONTENTION CHECK RUN BY THE STEWARD, 2026-09-16. NO LIVE CONTENTION — and
the first pass of this check got it wrong in the alarming direction, so the
refutation is recorded rather than the finding quietly dropped.**

A branch scan surfaced `wp/LANG-CONSTRUCTOR-NAMESPACE-SHADOWING-GUARD` at
`b7a9101ac026fcafa8cc6bac0ebfc0c88686dc31` touching `prelude.rs`. **That ref is
a fossil:** its node reads `status: merged` on `origin/main` and the branch is
**273 commits behind** main. A branch ref is not a live candidate; the node's
status is the instrument, and the branch scan is not.

The guard is therefore already **on** `main`, which makes the real question a
precondition rather than a race — and it is answered:

    guard_constructor_spelling (data.rs:131) fires only when
      globals.get(name) exists AND env.constructor(existing_id).is_some()
      AND the id is not one of the declaration's own constructors

⇒ It rejects a new **constructor** whose spelling collides with an existing
**constructor**. `PrivateMappingAcquireFile` is a primitive global, not a data
constructor, so it does not reach the guard's predicate.

**And the transplant-signature worry is refuted too.** The guard added a
`&mut elab.ctor_decl_spans` parameter to `elab_data_decl`, and this slice's
base (`4bf1ad362`) predates it — so a transplant that touched a data
declaration would fail to compile against current `main`. It touches **zero**
`elab_data_decl` call sites (measured). Nothing to carry.

**No obligation transfers to any other candidate.** Recorded because the first
answer was "textually clean, semantically not" and the measurement says
otherwise; a contention warning left standing in a released frame costs the
ring real time chasing a hazard that is not there.

## 7. Why this slice and not the POSTCALL subject

The obvious alternative cut is the code
`RT-D5B-POSTCALL-REFUSAL-MECHANISM` is held on —
`fn checked_ih_post_call_residual`, which reads **0 on `origin/main`** and
**1 on the backup tip** (positive control: `impl` reads 45 in main's copy of
that file, so the absence is real and not an unresolved path). Landing it would
clear that node's base hold.

**It is not the next slice, because it is not small.** The symbol and its five
call sites all live in `cranelift_backend/lowering/core.rs`, which is one of
the branch's largest changed files and sits inside the contested HS18 surface.
Cutting it means cutting into the closure the Architect is still amending
(amendment 8, `388bcd8d3`). The host cluster is the piece that is both
independent and finishable, which is what the directive asks for.

`RT-D5B-POSTCALL-REFUSAL-MECHANISM` correctly stays `ready` and base-held. Its
`MECH-2` deliverable is answered and closed; the node is held on its base, not
its content.
