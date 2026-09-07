# ABI-A3 — promote FsReadDirectory, FsCreateDirectory, FsRemoveFile, FsRemoveDirectory to NativeTested, exercising the landed path policy

**Owner:** Team Runtime (`runtime-leader` + `runtime-implementer` +
`runtime-qa`). **Branch:** `wp/ABI-A3`. **Size:** M. **Tier:** T1.
**Risk:** medium — native execution at the host trust boundary and an edit to
the file every ABI consumer reads. The correctness content is directory-mutation
ordering and partial-failure semantics, the one net-new reply-bytes leaf, and the
per-operation differential — not the flip. Lighter than ABI-A2 because A2 already
built and landed the shared native-FS resolution substrate this WP reuses.

**Authority:** `docs/program/10-linux-abi-completion.md §4`, Track A, ABI-A3.
**Status:** Steward frame, shovel-ready. `depends_on: [ABI-REVOKE, ABI-R3]` —
both merged. Released to the runtime ring as the ABI-A track's final slice,
ABI-A2 having completed (FsAppendFile + FsMetadata + FsRename all NativeTested,
node merged `3d85beb03`). ABI-A3 is the last Track-A availability promotion; its
whole-directory read is provisional (see the ABI-S2 banner below).

> ## The path policy is ALREADY LANDED — this WP exercises it, it does not design
> ## it. The native-FS substrate is ALSO landed — this WP reuses it.
>
> A3's path argument resolves through the same landed scoped-root / rights /
> symlink-no-follow machinery A2 exercised, and through the same native-FS
> resolution leg A2 built. There is NO open path-policy design question and no
> route to the enclave for it. Reuse, do not rebuild:
> - **`ProcessHost::parent`** (`crates/ken-host/src/abi_v1.rs:372`) — walks
>   components from the scoped root, rejecting symlinks per-hop (no-follow),
>   returning `(parent RootedHandle, leaf PathComponent)`. It calls `Self::root`
>   (`:363`, rejects `Virtual` as `ScopeEscape`) and `Self::reject_symlink`
>   (`:329`).
> - **`Cap::mint_scoped`** (`crates/ken-host/src/capability.rs:241`) + the rights
>   check (`capability.rs:319 if !scope.rights.contains(right)`).
> - **The std-fs host primitives already in `crates/ken-host/src/lib.rs`:**
>   `read_directory:419`, `create_directory:997` (mkdirat), `remove:1010`
>   (unlinkat, `RemoveKind::File`/`Directory`), `remove_directory_tree:1023`
>   (`remove_dir_all`, for `recursive=true`).
>
> Read this machinery; the native leg calls into it, it is not re-implemented.

---

## 1. Why this is its own slice, and what the whole judgment is

Track A is split by **evidence shape, not by count.** ABI-A1 was console/clock
(nondeterministic observation, a normalized differential). ABI-A2 was
metadata/rename (path-policy interaction). ABI-A3 is **directory mutation**,
whose distinguishing difficulty is **ordering and partial-failure semantics**:
the promotion is only honest if the native path agrees with the interpreter on
an observation that is **order-independent where the OS gives no order** and on a
**failure classification** that matches at exactly the partial-failure edges
(target exists, parent missing, directory not empty, wrong node kind).

The whole T1 content of this WP is three coupled judgments:

1. **An order-independent differential for `FsReadDirectory`.** `readdir` order
   is unspecified by the OS. The listing is `Vec<DirEntryV1>` where each entry is
   `{ name, kind }` (names + node kinds). Native and interpreter agree on the
   **canonicalized set** — entries sorted by name before comparison — never on
   raw iteration order. Asserting raw order is a false differential that would
   flake or entrench a non-contract.
2. **A state-transition + failure-classification differential for the three
   mutations.** `FsCreateDirectory`, `FsRemoveFile`, `FsRemoveDirectory` are
   filesystem state changes, not value reads (the shape ABI-A2's FsRename
   established). Agreement is over the observable before/after directory state
   **and** the result classification, and it is non-vacuous only if it drives the
   partial-failure edges: create-where-target-exists, create-where-parent-missing
   (with vs without the `recursive` flag), remove-file-on-a-directory,
   remove-directory-that-is-non-empty (with vs without `recursive`).
3. **Path-policy negative controls as first-class evidence.** For all four ops an
   in-root/with-rights operation succeeds and an escape / symlink-under-no-follow
   / missing-right operation is **refused with the policy error**, on both native
   and interpreter — the same load-bearing AC ABI-A2 carried, reused unchanged.

None of this generalizes forward: ABI-S2 (directory **streaming**) supersedes
this slice's whole-directory read where streaming is the honest shape.

> ## Do NOT entrench whole-directory read (`10-linux-abi-completion.md §4`, S2).
>
> `ABI-S2` supersedes ABI-A3's whole-directory `FsReadDirectory` where streaming
> is honest. So the `DirectoryEntries(Vec<DirEntryV1>)` whole-listing reply is
> **provisional**: wire it as the honest shape for *this* slice, but do not build
> consumers, tests, or manifest contracts that assume whole-directory read is
> permanent, and do not add a "read the whole directory" convenience surface that
> S2 would then have to remove. The reply-bytes leaf you add here is the payload
> mechanism, not a commitment to unbounded listing.

## 2. Fixed inputs (measured at `origin/main = 3d85beb03`; RE-MEASURE at your cut)

| path | fact |
|---|---|
| `crates/ken-host/src/effect_v1.rs:28` / `:29` / `:30` / `:31` | `FsReadDirectory = 0x0305`, `FsCreateDirectory = 0x0306`, `FsRemoveFile = 0x0307`, `FsRemoveDirectory = 0x0308` (wire ids) |
| `effect_v1.rs:151` / `:152` / `:153` / `:154` | the four `=> RepresentedUnavailable` availability arms — the four flip targets |
| `effect_v1.rs:220` `NATIVE_TESTED_TARGETS_V1` | `[HostOpV1; 18]` after ABI-A2; add these four → **22** |
| `effect_v1.rs:264` `native_tested_count: NATIVE_TESTED_TARGETS_V1.len()` | recomputes automatically — do NOT hand-edit (field decl `:248`) |
| `crates/ken-host/effect_abi_v1.catalog:64-67` | `FsReadDirectory\|0305\|unavailable\|FsPathRequestV1\|2`, `FsCreateDirectory\|0306\|unavailable\|FsRecursivePathRequestV1\|3`, `FsRemoveFile\|0307\|unavailable\|FsPathRequestV1\|2`, `FsRemoveDirectory\|0308\|unavailable\|FsRecursivePathRequestV1\|3` — status `unavailable`, to flip to `native`. Create/RemoveDirectory carry the `recursive` flag (arity 3) |
| `effect_v1.rs:2795` / `:2873-2875` | `CanonicalReplyV1::DirectoryEntries(Vec<DirEntryV1>)`; `DirEntryV1 { name: Vec<u8>, kind: FsNodeKindV1 }` — names + kinds |
| `effect_v1.rs:2210-2212` | dispatch maps `FsReadDirectory` → `backend.fs_read_directory(..).map(CanonicalReplyV1::DirectoryEntries)` |
| `abi_v1.rs:1033` `set_reply` (no `DirectoryEntries` arm → falls to `REPLY_ERROR` catch-all `:1162`) | the reply-bytes leaf is **NOT wired** — this is the one net-new manifest/decode piece (see D-NATIVE) |
| `abi_v1.rs:1048-1061` | ConsoleRead's arena-backed `reply.bytes.data`/`.len` governed leaf — the SHAPE to reuse for `DirectoryEntries` serialization (FsMetadata's `bytes.len`-only null-data form at `:1091-1099` does NOT apply — ReadDirectory has a real payload) |
| `abi_v1.rs:372` `ProcessHost::parent` (+ `:363 root`, `:329 reject_symlink`, `:381 open_at`) | the reusable scoped-root/no-follow resolver — reuse, do not rebuild |
| `capability.rs:241` `mint_scoped`, `:319` rights check | the call-scoped capability + rights machinery |
| `lib.rs:419` `read_directory`, `:997` `create_directory`, `:1010` `remove(..,RemoveKind)`, `:1023` `remove_directory_tree` | the landed low-level host primitives the four backend methods call |
| `abi_v1.rs:525` / `:537` / `:557` (A2 backend methods) ; `abi_v1.rs:1410` / `:1432` / `:1450` (A2 dispatch arms) | the A2 precedent pattern to copy for the four new backend methods + four new `ken_host_dispatch_v1` decode/execute arms |

**All four directory ops are fully deferred** at the backend / dispatch /
manifest layers (measured: the `impl HostEffectBackendV1 for ProcessHost` block
`abi_v1.rs:416` implements none of them; they fall to the default
`Err(Unsupported)` stubs; no dispatch arms; no `DirectoryEntries` reply arm). The
only native work already present is the four `lib.rs` std-fs primitives and the
`parent` resolver. D0 confirms the exact state at your cut.

## 3. The design, front-loaded

A NativeTested promotion means native execution is proven to agree with the
interpreter under a differential. Author each comparator as an explicit
projection applied to BOTH sides before comparison, never a relaxed assertion on
one side.

- **Reuse the shared native-FS substrate (built by A2).** Each of the four native
  backend methods resolves its path through `ProcessHost::parent` +
  `mint_scoped`/rights before touching the filesystem, and maps a policy refusal
  to the op's existing error reply — no new error identity. The four methods then
  call their `lib.rs` primitive (`read_directory` / `create_directory` /
  `remove` / `remove_directory_tree`). Copy the A2 backend-method + dispatch-arm
  pattern (`abi_v1.rs:525…`, `:1410…`) for `0x0305`–`0x0308`.
- **`FsReadDirectory`** (the one net-new reply-bytes leaf). Serialize the
  `Vec<DirEntryV1>` (names + kinds) into `reply.bytes` via a new arena-backed
  `set_reply` arm modeled on ConsoleRead (`abi_v1.rs:1048-1061`), with a matching
  decode on the read side. The differential is over the **canonicalized** listing
  — entries sorted by name, each `{name, kind}` compared — native vs interpreter.
  Order is not part of the contract (see §1.1 and the ABI-S2 banner). Requires the
  read right; a read outside the root or across a no-follow symlink is refused.
- **`FsCreateDirectory`** (state transition). In-root create yields the directory
  present afterward. **The `recursive` flag is STRUCTURALLY INERT and A3 promotes
  it as-is** (D0 finding, Architect §6 ruling `evt_3hfxk94v9f9n7`): the landed
  `lib.rs::create_directory(parent, leaf)` is a single `mkdirat` with no recursive
  parameter, so the `recursive:u64` field of `FsRecursivePathRequestV1`
  (`abi_v1.rs:200`) has no branch to reach — `recursive=true` and `=false` are
  identical (single leaf; `NotFound` on a missing parent; `AlreadyExists` on an
  existing target), and the interpreter matches. **Assert both-`NotFound` on a
  missing parent with the target absent before/after; do NOT assert parent-chain
  creation.** Create where the target already exists is the exists-classification,
  asserted identically on both sides. Requires the write right. A3 is a promotion
  — it does not give the flag meaning. **Giving the flag `mkdir -p` semantics OR
  retiring the dead field is a separate Spec-owned behavioral-contract question,
  registered as [[ABI-FSCREATE-RECURSIVE-CONTRACT]], and is explicitly NOT A3's
  scope.**
- **`FsRemoveFile`** (state transition). In-root remove yields the file absent
  afterward; remove of a directory is refused with the wrong-kind classification;
  remove of a missing path is the not-found classification. Requires the write
  right.
- **`FsRemoveDirectory`** (state transition + partial-failure, the sharpest arm).
  `recursive=false` on a **non-empty** directory is refused with the not-empty
  classification (the directory is unchanged); on an empty directory it is
  removed. `recursive=true` removes the tree. **RULED non-transactional** (D0
  finding, Architect §6 ruling `evt_3hfxk94v9f9n7`): `remove_directory_tree`
  delegates directly to `std::fs::remove_dir_all` (`lib.rs:461-467`), which is
  non-atomic with no rollback and **no residual guarantee on a mid-traversal
  error**. The comparator's carve-out is therefore **keyed on the error CLASS,
  and classification equality is asserted on EVERY path**:
  - **Deterministic classes** — `recursive=true` success → empty root;
    `recursive=false` non-empty → `NotEmpty` with the tree intact: assert exact
    twin-root state AND classification.
  - **Mid-traversal-error class** — `PermissionDenied` and any IO error arising
    DURING removal: assert classification ONLY; the residual tree is unconstrained
    and native/interpreter may legitimately diverge there. Do NOT assert an exact
    residual and do NOT invent an allowed-residual invariant — that would test a
    `remove_dir_all` non-contract (reflect-don't-extend).

## 4. Deliverables

- **`D0` — measurement + the partial-failure contract.** Re-derive the
  `effect_v1.rs` blob, the four availability arms, the four catalog rows, and the
  `set_reply` gap at your cut. Report the exact reply-bytes leaf shape you will
  add for `DirectoryEntries`. **Report the failure classifications** the landed
  primitives actually produce at each partial-failure edge (target exists,
  parent missing, non-empty rmdir, wrong-kind remove) and the recursive-removal
  partial-failure guarantee — grounded in the `lib.rs` primitives, not invented.
  If a needed classification or observability is genuinely missing, that is a
  hard-stop finding (§6). **DISCHARGED** (runtime-implementer
  `evt_4k04typ5a45mg`): D0 measured cleanly and fired two §6 hard-stops — the
  inert `recursive`-create flag and the non-deterministic recursive-removal
  residual. Both are RULED (Steward scope + Architect §6 contract,
  `evt_3hfxk94v9f9n7`) and folded into §3 above; D-NATIVE is the live deliverable.
- **`D-NATIVE` — native execution for the four ops.** Add the four ProcessHost
  backend methods and the four `ken_host_dispatch_v1` decode/execute arms for
  `0x0305`–`0x0308`, on the reused resolution substrate (§3), removing their
  deferred markers; add the `DirectoryEntries` reply-bytes leaf (`set_reply` arm
  + decode). Update the deferred-boundary-rejection test so exactly these four
  leave the deferred set and no other op does.
- **`D1` — the differentials** (§3): a canonicalized (name-sorted) `{name,kind}`
  listing equality for `FsReadDirectory`; before/after state-transition +
  classification comparators for the three mutations, each driving its
  partial-failure edge. Applied symmetrically to native and interpreter.
- **`D2` — the path-policy negative controls.** Per op, a fixture that succeeds
  in-root with rights, AND one refused for each of: out-of-root escape, symlink
  under no-follow, missing right. Each refusal asserted on BOTH native and
  interpreter with the same policy error. A promotion whose tests never drive a
  refusal fails this deliverable.
- **`D3` — the promotion.** Flip `availability()` at `:151`–`:154`
  `RepresentedUnavailable → NativeTested`; add the four to
  `NATIVE_TESTED_TARGETS_V1` (→ 22); `native_tested_count` follows automatically
  — do not touch it. Flip exactly these four ops' status in
  `effect_abi_v1.catalog` (`unavailable → native`) so the ABI-R3 closure
  (catalog-native-status iff runtime-NativeTested) keeps the manifest hash honest.
  Preserve every wire numeric id, record layout, arity, `recursive`-flag schema,
  and `operation_count`.
- **`D4` — the negative control that proves each differential discriminates.** A
  deliberately wrong native observation per op (a listing missing an entry or
  with a wrong kind; a create that leaves the target absent; a remove that leaves
  the target present; a non-empty rmdir that reports success) reddens the exact
  named differential and names the op. Show the red, then remove it and show
  green.
- **`D5` — census tail.** State which `RepresentedUnavailable` operations remain
  after this WP (expect **7 → 3**: `ClockMonotonicNow`, `ClockSleepUntil`,
  `EntropyRandomBytes` stay; the four directory ops are promoted here). Track A is
  then complete.

## 5. Acceptance criteria

- **`AC-0` — native execution actually runs.** Control: `ken_host_dispatch_v1` no
  longer falls to its deferred rejection for `0x0305`–`0x0308`; each executes and
  returns its manifest reply (and `FsReadDirectory` returns a populated
  `reply.bytes` listing, not the `REPLY_ERROR` catch-all); the
  deferred-boundary-rejection test is updated so exactly these four are removed.
  A differential against a still-faked boundary fails this AC.
- **`AC-1` — the four are NativeTested and prove it under their differential.**
  Control: a named test per op runs native and interpreter, applies the op's
  comparator (canonicalized-listing / state-transition+classification), and
  asserts agreement on the compared projection, not "it runs".
- **`AC-2` — partial-failure edges are non-vacuous, asserted AS THE BASE
  BEHAVES** (amended per the §6 ruling `evt_3hfxk94v9f9n7`). Control: each
  mutation's differential drives its edge and native/interpreter agree, with the
  classification asserted on every path:
  - **create-target-exists** → `AlreadyExists`, both sides, target present
    before/after.
  - **create-parent-missing, recursive=true AND recursive=false** → both
    `NotFound`, both sides, target absent before/after. **The two arms are
    identical (the flag is inert); do NOT assert parent-chain creation** — that
    is not A3's contract (§3, [[ABI-FSCREATE-RECURSIVE-CONTRACT]]).
  - **remove-wrong-kind** (remove-file on a directory) → the wrong-kind
    classification, both sides, node present before/after.
  - **non-empty rmdir, recursive=false** → `NotEmpty` with the tree intact
    (deterministic): assert classification AND exact before/after state.
  - **recursive rmdir mid-traversal error** → classification ONLY; the residual
    is unconstrained (§3 removal carve-out). Classification equality still holds.

  A differential that only exercises the success path fails this AC; so does one
  that asserts an exact residual on the mid-traversal-error class.
- **`AC-3` — the path policy is exercised (reused from ABI-A2).** Control: for
  each op the in-root success AND at least the escape, symlink-no-follow, and
  missing-right refusals run on both native and interpreter and agree; neutering
  the policy call in the native leg reddens these.
- **`AC-4` — the listing differential is order-independent.** Control: a fixture
  whose native and interpreter `readdir` return the same entries in a **different
  order** still passes (the comparator canonicalizes); and a fixture missing or
  mis-kinding an entry fails. An assertion that would flip on iteration order
  fails this AC — order is not the contract (ABI-S2 owns streaming).
- **`AC-5` — each differential discriminates.** Control: D4's wrong-observation
  mutation reddens the exact named differential and names the op. Show the red,
  then green.
- **`AC-6` — no-regression, in CI (`COORDINATION §12`).** Targeted:
  `-p ken-host`, `-p ken-runtime`. Preserve all wire ids, layouts, arities, and
  `operation_count`; only the four availability arms, the array length, the four
  catalog statuses, and the new reply leaf change.

## 6. Hard-stop protocol

If D0 finds the landed `lib.rs` primitives do not expose a partial-failure
classification or a recursive-removal guarantee stable enough for a
deterministic differential, **stop and report it** — do not fabricate a
guarantee or relax the comparator to hide it. The likely shapes: a
recursive-removal that fails partway with no observable contract on the residual
tree, or a classification the primitive collapses (e.g. not-empty vs wrong-kind
indistinguishable). Report which op, which edge, and what the primitive actually
does; the Steward folds an Architect ruling on the honest contract, exactly as
ABI-A2's FsMetadata payload was amended. A native-execution substrate gap beyond
the landed policy is a finding, not a blocker to route around.

## 7. Contention check

Touches `crates/ken-host/` (`effect_v1.rs`, `abi_v1.rs`, `effect_abi_v1.catalog`,
`lib.rs`) and `crates/ken-runtime/` test/harness surfaces. **Disjoint from the
language lane** (ken-elaborator) and **the foundation lane** (catalog/) — no
file-level contention with LANG-FIXITY-DECL-SURFACE or the Tier-C migration. The
one shared-slot risk is the build/test slot under `scripts/ken-cargo` (the
laptop's single lock); runtime validations are short. No enclave route (the path
policy and native substrate are landed).

## 8. Banned scope

- **Do not design or re-open the path policy or the native-FS resolver.** Landed
  (ABI-R1 / ABI-A2); exercise and reuse them.
- **Do not entrench whole-directory read.** ABI-S2 supersedes it with streaming;
  the `DirectoryEntries` reply is the honest shape for this slice only.
- **Do not grow `trusted_base()`.** This is availability promotion at the host
  boundary, not a kernel or trusted-base change; the diff carries zero
  `trusted_base()` delta.
- **Do not assert raw `readdir` order** anywhere in the differential.
- **Do not hand-edit `native_tested_count`** — it follows the array length.

**Architect is a required reviewer** on the candidate (the partial-failure
contract and the new reply-bytes leaf are the soundness-bearing content).
