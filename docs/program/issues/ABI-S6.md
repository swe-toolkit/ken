---
id: ABI-S6
title: "ordinary anonymous and file-backed mappings as opaque runtime-owned regions and bounded byte views"
status: active
owner: runtime
size: L
gate: none
tier: T1
depends_on: [ABI-S1]
blocks: []
github: null
origin: "docs/program/10-linux-abi-completion.md §4 Track S (the ABI-completion program), row ABI-S6. Node filed by the Steward 2026-07-25; framed and released 2026-09-09 on the operator's standing 'keep L1 on ABI/compiler work' direction after ABI-S1 (descriptor completion) merged. runtime-leader named ABI-S6 as the next ABI-B entry (evt_6sd89wq68prby): the explicit ABI-S1 successor, now unblocked, and the opaque-region + bounded-byte-view substrate that later MMIO builds on — with the steer to frame the lifetime/bounds/refusal boundary rather than assume an API shape."
---

> # WP frame. Authority: `docs/program/10-linux-abi-completion.md §4` Track S.
> Fixed inputs measured at `origin/main` `2dbe90af798bdd72aacf95ee45e60c2130ad75cc`
> (the ABI-S1-closed tip). The `ken-host` / `ken-runtime` / prelude surfaces this
> node touches are byte-identical to `97aeab6cb` (the frame SHA runtime-leader
> named); the only delta between them is the ABI-S1 node status flip and its
> gen-progress regen. Re-measure the fixed inputs at pickup. ABI-S1 (D1-D6) is
> merged.

## Objective

Model ordinary anonymous and file-backed memory mappings as **opaque
runtime-owned regions** exposed to Ken only as **bounded byte views** — never as
Ken pointers. A mapping is acquired, held under a revocation-governed lifetime,
read (and, where the mapping is writable, written) through bounds-checked views
that yield owned bytes, and released — with every failure surfaced as a stable
typed refusal, and with no memory address ever crossing into Ken code.

This supplies the mapping / lifetime / bounded-access substrate that the later
L2-8 MMIO work builds on. Getting the ownership and bounds shape right here is
what keeps MMIO from needing raw pointers in application Ken, which is a stated
exit condition of the whole program (§6).

## Design judgment (front-loaded)

1. **The opacity / lifetime / bounds substrate ALREADY EXISTS. Do not reinvent
   it — reuse or extend it.** The tree already carries a complete, `NativeTested`
   opaque-region-plus-bounded-view mechanism, built for PX8:
   - `ResourceKindV1::Buffer` (`crates/ken-host/src/effect_v1.rs:789`) is a
     resource kind alongside `FsHandle`, held in the generation-checked
     `ResourceTableV1` (`effect_v1.rs:1010`).
   - `BufferRegionV1` (`effect_v1.rs:908`) is an opaque host-owned `Vec<u8>` with
     an initialized window and a bounds-checked `initialized_slice(start,len)` —
     no raw pointer ever crosses out.
   - `HostOpV1::BufferAllocate` (`0x0402`) mints a `Buffer` resource token;
     `HostOpV1::BufferFreeze` (`0x0403`) turns a bounded span of that region into
     a `CanonicalReplyV1::Bytes` value (owned bytes on the wire). This is
     literally "read a bounded byte view out of an opaque runtime-owned region
     and hand back owned bytes, never a pointer."
   - PX8-SPAN-PROV (`effect_v1.rs:1970` and enforcement at `:2626`) binds a
     span's originating acquisition token to its target before any byte exposure,
     and shares `InvalidBounds` with ordinary bounds failures so the two are
     indistinguishable to an observer.
   - Ken sees `Resource k` as an opaque primitive (`PrimReduction::OpaqueType`,
     `prelude.rs:1818`) indexed by the closed `ResourceKind` sum; `BufferHandle`
     / `BufferSpan` are private-constructor wrappers whose constructors are
     stripped from the public name map. Ken cannot construct, project, or inspect
     a `Resource` value — it only flows opaquely from a host call to a checked
     proc.

   ABI-S6 is therefore NOT the invention of a region/view/opacity model. It is
   the addition of an **OS-backed** region to the model that already governs
   in-process buffers.

2. **What is genuinely new is the OS backend seam, and ABI-S6 is a SPLICE of two
   landed precedents, not an extension of either alone.** `BufferAllocate` /
   `BufferFreeze` have NO `HostEffectBackendV1` method — they are pure in-process
   `Vec<u8>` operations. A real `mmap` / `munmap` needs a backend method, shaped
   like the `FsHandle` path's `fs_resource_duplicate` (`effect_v1.rs:1946`) and
   `resource_close` (`:1955`). So the region-acquire / region-release side
   follows the FsHandle real-syscall-backend pattern, while the bounded-view
   side follows the Buffer region/span pattern. Picking one precedent wholesale
   is wrong; the deliverable is their disciplined composition.

3. **Lifetime is revocation lineage, reused verbatim — not a new lifetime
   notion.** A region resource is minted through the existing
   `ResourceTableV1::insert_owner` (`effect_v1.rs:1053`) with an explicit
   `RightSet` and a `RevocationNodeId` provenance, and resolved through a
   kind/generation/rights-checked accessor. Revocation of a region's lineage node
   must make the region and every view derived from it inadmissible, using the
   same `RevocationDomain` (`revocation_v1.rs`) mechanism ABI-S1 D5 used for
   descriptor aliases (`copy` shares a node, `attenuate` narrows). Unmap and
   revocation-close interact exactly as `resource_close` and admission do today
   (`admit_resources` / `ResourceAdmissionLeaseV1`, `effect_v1.rs:1330`): a
   revoked or unmapped region can never yield a live view.

4. **Never Ken pointers is a hard invariant with a landed mechanism to inherit.**
   The region is a third `ResourceOwnerV1` / `ResourceKindV1` participant resolved
   only through the generation-checked slot table; the token that names it is the
   opaque `{slot: u32, generation: u32}` pair (`effect_v1.rs:770`), never an
   address. Any bounded view is a Ken-opaque wrapper with a private constructor,
   following `BufferHandle` / `BufferSpan` exactly. No deliverable may expose the
   region's real address, an offset that is an address, or a projectable pointer.

5. **Lands `RepresentedUnavailable`; native promotion is a separate later node.**
   This follows the ABI-S1 precedent exactly (the entire descriptor slice landed
   `RepresentedUnavailable`, proved by
   `abi_s1_partial_keeps_descriptor_operations_represented_unavailable`,
   `effect_v1.rs:4537`). ABI-S6 delivers the surface, the region/view/lifetime
   representation, the error vocabulary, the inventory registration, and the
   backend trait seam — NOT the differential native `mmap` harness. A node that
   lands `RepresentedUnavailable` owes NOTHING in the `ken-runtime` Cranelift
   lowering and adds NO `SYS_*` pinned fact (ABI-S1 D5 added zero — there is no
   "SYS_DUP3" precedent; SYS_ facts are added only at native promotion). Native
   promotion (backend implementation of `mmap`/`munmap`, `availability` ->
   `NativeTested`, `NATIVE_TESTED_TARGETS_V1`, `SYS_MMAP`/`SYS_MUNMAP` facts, the
   three-dimensional Cranelift arms, the differential harness) is framed when
   this lands.

## D0 — the representation-shape ruling is the ARCHITECT's, not this frame's

Per runtime-leader's steer, this frame fixes the BOUNDARY, not the API shape.
There is one genuine representation fork, and it is the Architect's call on
measured evidence, not a Steward or ring assumption:

**Does an OS-backed mapping REUSE `ResourceKind::Buffer` / `BufferHandle` /
`BufferSpan` / PX8-SPAN-PROV (a mapping is "a buffer whose backing store is
OS-managed"), or does it mint a PARALLEL `ResourceKind::Mapping` +
`MappingHandle` / `MappingSpan` family?**

D0 is the first deliverable: the ring measures whether the existing `Buffer`
view contract can carry an OS-backed region's semantics — file-backed vs
anonymous backing, writable vs read-only protection, faulting/partial-residency,
and unmap-vs-free lifetime — WITHOUT weakening PX8-SPAN-PROV's origin binding or
the bounds guarantees, and **HARD-STOPS to the Architect** with that
measurement. The Architect rules the shape. Reasons this cannot be pre-decided:
reusing `Buffer` risks conflating "free this in-process allocation" with "unmap
this OS mapping" (a real safety distinction), while a parallel `Mapping` kind
risks exactly the duplicated ownership model the objective forbids. The frame
requires the boundary (judgments 1-5); the Architect chooses which representation
meets it. Do not author either shape before that ruling.

## Deliverables

D0 gates the rest. After the Architect's shape ruling, the remaining work is the
region-acquire/hold/view/release surface under that shape. Per-increment partial
landing is authorized. Each operation is threaded through every registration
site (see AC-INVENTORY-BUILD-BREAK) and lands `RepresentedUnavailable`.

- **D0 — representation-shape measurement + Architect ruling (reasoning-dense,
  first).** As above: measure the reuse-vs-parallel fork against the boundary,
  hard-stop to the Architect, land nothing until ruled.
- **D1 — region acquire + release under lifetime (load-bearing).** A
  mapping-acquire operation that mints an opaque runtime-owned region resource
  (kind per D0) via `insert_owner` with an explicit `RightSet` and revocation
  provenance, plus the `HostEffectBackendV1` method seam for the real
  `mmap`/`munmap` (shaped like `fs_resource_duplicate` / `resource_close`), and a
  region-release path. Anonymous backing is the base case; file-backed acquire
  takes a held `FsHandle` and shares/derives its revocation lineage. Deny by
  default: acquisition requires the governing right; a region cannot outlive a
  revocation of its lineage.
- **D2 — bounded byte views over the region.** Reads produce bounded,
  generation- and bounds-checked byte views that yield owned bytes on the wire
  (the `BufferFreeze` shape), with the span's origin bound to its region per
  PX8-SPAN-PROV. Where D0/D1 admit a writable mapping, writes go through the same
  bounds-checked view discipline. No raw address, and no address-shaped offset,
  crosses to Ken.
- **D3 — registration, inventory, and sentinel maintenance (build-break).** Every
  new operation is threaded through all registration sites (judgment 2 of AC
  below). The enumeration sentinels that hand-list the represented/deferred set
  (`abi_a3_completion_leaves_only_the_non_track_a_deferred_tail`,
  `effect_v1.rs:4564`) are extended to include the new operations; the
  descriptor-band sentinel (`try_from(0x0315) == Err`) stays UNTOUCHED (ABI-S6 is
  not a descriptor op — it takes the next free band-04 slot `0x0404` or a new
  band, per D0). Pinned ABI-fact inventory updated only as the represented
  surface requires; NO `SYS_*` fact is added (native promotion is a later node).

## Acceptance criteria (each with its control)

- **AC-OPAQUE-NO-POINTER (hard invariant).** No Ken-visible value produced by any
  ABI-S6 operation is, contains, or can be projected to, a memory address. The
  region token is the generation-checked slot pair; views are private-constructor
  opaque wrappers. Control: a construction/projection path that would expose an
  address or an address-shaped offset is uncompilable in Ken (no public
  constructor / the name is stripped from the public map), and a mutation that
  makes a view's byte offset an absolute address reddens a named test.
- **AC-BOUNDS-CHECKED (load-bearing).** Every byte a view yields lies within the
  region's valid window; an out-of-range or wrong-origin view is refused before
  any byte is exposed, sharing the `InvalidBounds` refusal so it is
  indistinguishable from an ordinary bounds failure (PX8-SPAN-PROV discipline).
  Control: a view request past the window, and a span whose origin token does not
  match its region, each red a named test; neutering the bounds/origin check
  leaves the suite green only if the control is vacuous — so the control must
  fail against a check-neutered tree.
- **AC-LIFETIME-REVOCATION.** A region and every view derived from it become
  inadmissible when the region's revocation lineage node is revoked, and a
  released/unmapped region yields no further live view. A file-backed region
  shares or derives its lineage from the source `FsHandle` so revoking the file
  revokes the mapping. Control: revoke the lineage (or release the region), then
  a subsequent view/read is refused with the stable typed refusal; a mutation
  that lets a view survive its region's revocation reddens a named test.
- **AC-REFUSAL-TYPED.** Every failure path — acquisition denied, backend
  (`mmap`) failure, out-of-bounds, revoked/closed region, malformed token —
  surfaces a stable typed refusal through the existing `ResourceErrorV1` /
  `SemanticErrorV1::Resource` (kind-generic) vocabulary or, if D0/Architect deem
  a mapping-specific arm necessary, an explicitly ruled addition. No raw errno,
  no silent fallback, no fail-open. Control: each failure path asserts its exact
  refusal; a path that drops the refusal identity or defaults open reds a named
  test.
- **AC-INVENTORY-BUILD-BREAK.** Each new operation is registered through every
  `HostOpV1` site — enum variant + opcode, `next_in_inventory`, `availability`,
  `is_ambient`, `capability_requirement`, `resource_admission_requirement`,
  request/reply schema, `dispatch_host_op_v1` arm, the `effect_abi_v1.catalog`
  row, the `effect_abi_probe.c` layout, the `abi_v1.rs` mirrored struct + decode,
  and the `effect_wire.rs` codec — so that dropping any one registration is a
  build break (`E0004` or a reddened derived-inventory test). Assertions are
  named memberships and properties, never total counts. Control: drop a
  registration -> the build or the derived-inventory test reds. NOTE: this is
  ~12-14 sites across three Rust files plus the catalog text and the C probe
  header, not five — budget accordingly.
- **AC-REPRESENTED.** New operations land `availability =
  RepresentedUnavailable`; `host_effect_wire_layout_v1` returns
  `OperationUnavailable` for them; the generated `effect_abi_v1.catalog` status
  matches; they are absent from `NATIVE_TESTED_TARGETS_V1`; and NO `SYS_*` fact is
  added. There is no `ken-runtime` Cranelift obligation. Control: the
  represented-tail sentinel and the fact-inventory anchor both reflect the new
  ops with no native-status or SYS_-fact change; a native-status flip reds a named
  test.
- **AC-RIGHT-BUDGET (measured constraint, not a deliverable).** `RightSet`
  (`capability.rs:94`) is a `u8` with 7 of 8 bits assigned — one bit remains. If
  the mapping needs a distinct capability right rather than reusing `READ` /
  `WRITE`, it consumes the last bit; a SECOND new right requires widening
  `RightSet` everywhere it crosses the wire (catalog, C probe, native lowering)
  and is a HARD STOP to the Steward + Architect. State in the candidate which
  rights the mapping surface uses and whether the last bit was consumed.
- **AC-AFFECTED-CLOSURE.** Cover every target that loads a changed module, not
  only diff-touched ones: the `effect_v1.rs` consumers in `ken-verify`
  (`imported_catalog_partition_is_exact_and_closed`), `ken-elaborator`
  (`export.rs` / `erasure.rs` / `prelude.rs`), the generated catalog data file,
  and the `ken-interp` reify path. Green in CI is the workspace verdict — build
  and test locally targeted only, never `--workspace` (COORDINATION §12).

## Gate, reviewers, sequencing

`gate: none` (node field). The change is in the host trust boundary (`ken-host`)
plus the prelude and the interp reify path, so the MERGE carries TCB at Steward
routing. Adding operations and a backend trait method to the existing resource
model is the authorized ABI-completion program (§4), NOT a new TCB grab — but the
region/lifetime/opacity design is soundness-bearing, and D0 is a genuine
representation fork. On each candidate: **runtime-leader owns the merge Decision;
Architect required review** (D0 shape ruling first; then verify opacity is
absolute, bounds and origin binding hold, lifetime is revocation-governed, unmap
and revocation-close interact safely, and no new TCB-growing primitive beyond the
resource model + one backend method was introduced) **+ Runtime QA** on the exact
SHA, then Steward M1-M4 -> lieutenant. **No Decision is required to RELEASE** this
node (the frame is the Steward's, the program is operator-authorized); the merge
Decision is assembled from the Architect + Runtime QA votes on each candidate.
Runtime owns the WP; Foundation collaborates on the Ken-visible view types
(§4 records Runtime + Foundation), but a WP has a single owner.

## Contention

`crates/ken-host` (`effect_v1.rs`, `lib.rs`, `abi_v1.rs`, `effect_wire.rs`,
`capability.rs`, `revocation_v1.rs`, `effect_abi_v1.catalog`,
`effect_abi_probe.c`), `crates/ken-elaborator/src/prelude.rs`, the
`crates/ken-interp` reify path, and downstream consumers in `ken-verify` and
`ken-elaborator` export/erasure. Re-measure at pickup. Measured contention-free
at `2dbe90af7`: `ken-host` / `ken-runtime` are idle post-ABI-S1; the leftover
`wp/ABI-S1-*` and `wp/ABI-A2-metadata-lt-respin` branches are spent squash-source
remnants of merged nodes (ABI-A2 status `merged`, its branch tip not an ancestor
of main — publisher squashes), NOT live work. Lane 3 (foundation) is on
`catalog/` Tier-D — disjoint. Lane 2 (language) is stood down. The concurrent doc
track (`library/`, `agent/`) is disjoint.

## Hard stop

Route to the Steward if: the D0 measurement shows neither reuse nor a parallel
kind can meet the boundary without a new TCB-growing primitive beyond the
resource model plus one backend method (that needs operator authorization); a
second distinct capability right is required after the last `RightSet` bit is
consumed; or opacity / bounds / lifetime cannot be held without exposing an
address to Ken. Any of those means the work as framed is not what the tree needs,
not that scope should bend. The D0 representation fork itself is a hard-stop to
the ARCHITECT (not the Steward) — it is a design ruling, not a scope question.
