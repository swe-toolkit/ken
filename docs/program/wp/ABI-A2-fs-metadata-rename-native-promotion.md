# ABI-A2 — promote FsAppendFile, FsMetadata, FsRename to NativeTested, exercising the landed path policy

**Owner:** Team Runtime (`runtime-leader` + `runtime-implementer` +
`runtime-qa`). **Branch:** `wp/ABI-A2`. **Size:** L. **Tier:** T1.
**Risk:** medium — like ABI-A1 it implements native execution at the host trust
boundary and edits the file every ABI consumer reads; the correctness content is
the native FS leg, the path-policy exercise, and the per-operation differential,
not the flip.

**Authority:** `docs/program/10-linux-abi-completion.md §4`, Track A, ABI-A2.
**Status:** Steward frame, shovel-ready. `depends_on: [ABI-REVOKE]` — merged
(`3e1b21cf1`). Released to the runtime ring as the ABI-A track's next slice,
ABI-A1 having completed (Clock + ConsoleRead both NativeTested, node merged
`2db854e97`). A3 (directory mutation, `depends_on ABI-R3`) follows this slice.

> ## The path policy is ALREADY LANDED — this WP exercises it, it does not design
> ## it.
>
> A2's distinguishing difficulty is path-policy interaction (scoped roots,
> rights, symlink policy, no-follow resolution). **That policy has landed** —
> `10-linux-abi-completion.md §4` ABI-R1 exists precisely because the capability
> prose is now stale: "scoped roots, rights, symlink policy, and no-follow
> resolution have landed." So there is NO open path-policy design question and no
> route to the enclave. The T1 content is proving the native promotions
> **exercise** that policy — a metadata/rename inside the scoped root succeeds; an
> escape, a symlink under no-follow, or a missing right is refused with the policy
> error — not inventing it. The machinery lives in `crates/ken-host/src/
> capability.rs` and `crates/ken-runtime/src/native_effect_v1.rs` (measured
> below); read it, do not re-implement it.

---

## 1. Why this is its own slice, and what the whole judgment is

Track A is split by **evidence shape, not by count.** ABI-A1 was console/clock
(nondeterministic observation, a normalized differential). ABI-A2 is
**metadata/rename**, whose distinguishing difficulty is **path-policy
interaction**: the promotion is only honest if the native path exercises the
landed scoped-root / rights / symlink / no-follow policy and refuses exactly what
the policy refuses.

The whole T1 content of this WP is three coupled judgments, none of which
generalizes to A3 (directory mutation, ordering and partial-failure):

1. **A normalized differential for `FsMetadata`.** A stat result mixes
   deterministic fields (size, file type, permission mode) with volatile ones
   (mtime/atime/ctime, inode, nlink). Native and interpreter agree on the
   **normalized** projection that erases the volatile fields and keeps the
   structural ones — the same shape as ABI-A1's `ClockWallNow`, stated per field.
2. **A state-transition differential for `FsRename`.** Rename is not a value
   read; it is a filesystem state change. The agreement is over the observable
   **before/after directory state** (`a` present → after `rename(a,b)`, `a`
   absent and `b` present with `a`'s content) plus the result classification —
   not a byte-equal return value.
3. **Path-policy negative controls as first-class evidence.** For all three ops,
   an in-root/with-rights operation succeeds and an escape / symlink-under-
   no-follow / missing-right operation is **refused with the policy error**, on
   both native and interpreter. A promotion whose differential never drives the
   policy has not proven the native op is path-confined.

## 2. Fixed inputs (measured at `origin/main = 2db854e97`; RE-MEASURE at your cut)

| path | fact |
|---|---|
| `crates/ken-host/src/effect_v1.rs:26` | `FsAppendFile = 0x0303` (wire id) |
| `crates/ken-host/src/effect_v1.rs:27` | `FsMetadata = 0x0304` (wire id) |
| `crates/ken-host/src/effect_v1.rs:32` | `FsRename = 0x0309` (wire id) |
| `effect_v1.rs:149` / `:150` / `:155` | `FsAppendFile` / `FsMetadata` / `FsRename` `=> RepresentedUnavailable` — the three flip targets |
| `effect_v1.rs:220` `NATIVE_TESTED_TARGETS_V1` | `[HostOpV1; 15]` after ABI-A1; add these three → **18** |
| `effect_v1.rs:261` `native_tested_count` | `= NATIVE_TESTED_TARGETS_V1.len()` — recomputes automatically, do not hand-edit |
| `crates/ken-host/effect_abi_v1.catalog:62-68` | `FsAppendFile\|0303\|unavailable\|FsAppendFileRequestV1\|3`, `FsMetadata\|0304\|unavailable\|FsPathRequestV1\|2`, `FsRename\|0309\|unavailable\|FsRenameRequestV1\|3` — status `unavailable`, to flip to `native` |
| `crates/ken-host/src/capability.rs`, `crates/ken-runtime/src/native_effect_v1.rs` | the LANDED path-policy machinery (scoped roots, rights, symlink/no-follow). Read it; the native leg calls into it, it is not rebuilt |
| `crates/ken-runtime/src/cranelift_backend/lowering/effects.rs`, `.../planning/static_transition/aggregates.rs` | the `HostResponseReferent{class}` governed leaf ABI-A1 landed for ConsoleRead's host-reply bytes (owner `{PersistentStore}`) — the reuse candidate for `FsMetadata`'s host-generated result |
| `crates/ken-runtime/src/object_linker_packaging.rs` | ProcessHost impl — measured to have **no** `fs_metadata`/`fs_rename`/`fs_append` native method: native FS execution is ABSENT (see D0) |

**Native FS execution is absent** (measured: no ProcessHost FS methods), exactly
as ABI-A1 found for console/clock. So the native leg is in scope here too
(`D-NATIVE`), and this WP is L/T1, not the stub's M. D0 confirms the exact state
at your cut — a partial substrate in `native_effect_v1.rs` may already exist.

## 3. The design, front-loaded

A NativeTested promotion means native execution is proven to agree with the
interpreter under a differential. Author each normalizer/comparator as an
explicit projection applied to BOTH sides before comparison, never a relaxed
assertion on one side.

- **Shared native FS substrate (build once, all three ops use it).** Native
  `FsAppendFile`/`FsMetadata`/`FsRename` resolve their path argument through the
  landed scoped-root / rights / no-follow machinery (`capability.rs`,
  `native_effect_v1.rs`) before touching the filesystem, and map a policy refusal
  to the operation's existing error reply — no new error identity. Building this
  resolution+rights leg once is the bulk of `D-NATIVE`; the three ops differ only
  in what they do after a path resolves.
- **`FsAppendFile`** (the simplest reply — a count, no host-reply-bytes leaf).
  Deterministic given a fixture: the resulting file content is prior content ++
  appended bytes and the returned count is the appended length. The differential
  is exact on content and count; no normalization needed. Requires the write
  right; an append without it or outside the root is refused.
- **`FsMetadata`** (host-reply bytes → governed leaf; normalized differential).
  Normalizer erases mtime/atime/ctime, inode, nlink and keeps size, file type,
  permission mode; native and interpreter agree on the normalized projection.
  Its host-generated result bytes need an owner — **reuse ConsoleRead's
  `HostResponseReferent{class}` (`{PersistentStore}`) if the shape matches**; a
  distinct owner, if genuinely required, is backend planner substrate authorized
  within this WP (as ConsoleRead's was — NOT TCB, no separate node, no operator
  call), carrying the same exact-owner mutation + exhaustive-match + byte-identical
  existing-mappings obligations. No-follow: metadata on a symlink returns the
  **link's own** metadata, not the target's, and the differential asserts that.
- **`FsRename`** (state transition). The comparator is over the observable
  before/after directory state and the result classification, applied identically
  to native and interpreter runs against the same fixture tree. A rename whose
  source or destination escapes the scoped root, or crosses a symlink under
  no-follow, is refused; an in-root rename performs the exact state delta.

## 4. Deliverables

- **`D0` — measurement.** Re-derive the `effect_v1.rs` blob and the three
  availability arms at your cut; report whether any native FS execution leg exists
  (ProcessHost method, `ken_host_dispatch_v1` decode arm, or partial
  `native_effect_v1.rs` wiring) or is absent, and name the exact path-policy entry
  point in `capability.rs`/`native_effect_v1.rs` the native leg will call. If a
  needed substrate is genuinely missing beyond the landed policy, that is a
  hard-stop finding (§6), not fabrication.
- **`D-NATIVE` — implement native FS execution for the three ops** (the
  load-bearing leg), on the shared scoped-root/rights/no-follow resolution
  substrate (§3). Add the ProcessHost methods and the `ken_host_dispatch_v1`
  decode/execute arms for `0x0303`/`0x0304`/`0x0309`, removing their deferred
  markers, and update the deferred-boundary-rejection test so exactly these three
  leave the deferred set. Reuse ConsoleRead's governed leaf for `FsMetadata`'s
  reply bytes where the shape matches. Re-measure every line at your cut.
- **`D1` — the differentials, one per op** (§3): exact content/count for
  `FsAppendFile`; a normalized-projection equality for `FsMetadata` with the
  invariant each erased/kept field encodes stated in prose; a before/after
  state-transition comparator for `FsRename`. Applied symmetrically to native and
  interpreter.
- **`D2` — the path-policy negative controls** (the A2-specific evidence). Per
  op, a fixture that succeeds inside the scoped root with rights, AND one that is
  refused for each of: out-of-root escape, symlink under no-follow, missing right.
  Each refusal is asserted on BOTH native and interpreter with the same policy
  error. A promotion whose tests never drive a refusal fails this deliverable.
- **`D3` — the promotion.** Flip `availability()` at `:149`/`:150`/`:155`
  `RepresentedUnavailable → NativeTested`; add the three to
  `NATIVE_TESTED_TARGETS_V1` (→ 18); `native_tested_count` follows automatically
  — do not touch it. Flip exactly these three ops' status in
  `effect_abi_v1.catalog` (`unavailable → native`) — the ABI-R3 closure requires
  catalog-native-status iff runtime-NativeTested, so the manifest hash changes
  honestly. Preserve every wire numeric id, record layout, arity, and
  `operation_count`.
- **`D4` — the negative control that proves each differential discriminates.**
  A deliberately wrong native observation per op (an append that writes the wrong
  count; a metadata read whose kept field disagrees; a rename that leaves the
  source present) reddens the exact named differential. A differential only ever
  run against the correct implementation is not evidence.
- **`D5` — census tail.** State which `RepresentedUnavailable` operations remain
  after this WP (expect **7** → **4**: the two Clock siblings and Entropy stay;
  the four A3 directory ops remain), so A3 is framed against a measured surface.

## 5. Acceptance criteria

- **`AC-0` — native FS execution actually runs.** Control: `ken_host_dispatch_v1`
  no longer falls to its deferred rejection for `0x0303`/`0x0304`/`0x0309`; each
  executes and returns its manifest reply; the deferred-boundary-rejection test is
  updated so exactly these three are removed and no other op is. A differential
  against a still-faked boundary fails this AC.
- **`AC-1` — the three are NativeTested and prove it under their differential.**
  Control: a named test per op runs native and interpreter, applies the op's
  comparator (exact / normalized / state-transition), and asserts agreement on the
  compared projection, not "it runs".
- **`AC-2` — the path policy is exercised (the load-bearing A2 AC).** Control: for
  each op, the in-root success AND at least the escape, symlink-no-follow, and
  missing-right refusals run on both native and interpreter and agree; neutering
  the policy call in the native leg reddens these. A promotion that bypasses the
  policy check but keeps the happy path green fails this AC.
- **`AC-3` — each differential discriminates.** Control: D4's wrong-observation
  mutation reddens the exact named differential and names the op. Show the red,
  then remove it and show green.
- **`AC-4` — the `FsMetadata` normalizer is necessary, not decorative.** Control:
  an exact-equality metadata differential fails on a correct implementation
  (mtime/inode differ run to run), shown in a comment or ignored companion, so the
  normalization is load-bearing.
- **`AC-5` — the wire contract is unchanged; only these three ops' availability
  status moves.** Control: `git diff` on `effect_abi_v1.catalog` shows a status
  change (`unavailable → native`) for `FsAppendFile`/`FsMetadata`/`FsRename` ONLY
  — every wire numeric id, record layout, arity, and `operation_count` unchanged;
  no op added or removed. The manifest hash changing as a consequence is correct.
  Weakening the catalog↔runtime availability closure to avoid the edit is banned.
- **`AC-6` — the derived closure stays green** with `native_tested_count` now 18;
  no count assertion is hand-edited to match (`effect_v1.rs`,
  `ken-verify/src/catalog.rs`).
- **`AC-7` — direction stated.** This promotes three ops from unavailable to
  available; nothing previously available becomes unavailable. If anything
  regresses, stop.
- **`AC-8` — no-regression in CI (`COORDINATION §12`).** Targeted locally over the
  affected closure: name the exact `scripts/ken-cargo test -p <crate>` invocations
  across `ken-host`, `ken-runtime` (the native FS differential + policy tests),
  `ken-verify`, and any `ken-elaborator` availability-enumerating test. Never
  `--workspace` — workspace-green means green in CI.

## 6. Banned scope and hard stops

- Do not add or remove an operation, and do not change any ABI numeric identity
  (`AC-5`). This WP changes only how three operations are classified and proven.
- Do not re-implement or modify the landed path policy — call it. Do not weaken a
  differential to exact-output equality for a nondeterministic field, and do not
  make one side's assertion looser than the other's.
- Do not weaken or bypass the scoped-root/rights/no-follow check to make a happy
  path green — that is the exact unsoundness `AC-2` exists to catch.
- No `spec/` or `conformance/` edit. No new crate dependency without routing.
- **Hard stop to the Steward** if: a native leg needs a substrate beyond the
  landed path policy and the ConsoleRead governed-leaf template (name it, and the
  predecessor it implies); or an honest normalized/state-transition invariant for
  an op cannot be stated without weakening it to vacuity; or exercising the policy
  or making a differential discriminate forces touching a wire identity.

## 7. Contention

`crates/ken-host/src/effect_v1.rs` is the trust-boundary file (ABI-A1 pinned it,
ABI-R3-class inventory and PX8-family error-id work have pinned it before).
Re-derive its blob at pickup and check for a live foundation/runtime edit before
starting; the governed-leaf files under `cranelift_backend/` were just touched by
ABI-A1's ConsoleRead landing, so rebase cleanly onto `2db854e97` or later. The
lieutenant publishes serially; rebase the candidate if `effect_v1.rs` moved.
Per-operation accepted partials are fine — `FsAppendFile` is the natural first
(simplest reply, no governed leaf), then `FsMetadata`, then `FsRename`.
