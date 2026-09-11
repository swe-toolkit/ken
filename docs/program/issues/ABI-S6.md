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

# D5b NATIVE HARD-STOP 2 — AMENDED IN PLACE 2026-09-11 (Steward scope call)

> # D5b native-lowering track, HARD STOP 2 (distinct from the surface HS#1-3
> # closed by the offset drop below). Architect ruling evt_139wvys9mv3z4
> # (thr_7wy5wy45p7abm), grounded at exact b94c5ae7. The prior operation-iota
> # attempt (eliminate the FSOp operation-dispatch match entirely inside
> # lower_computational_producer_construct) was ONE LAYER SHALLOW and is REVERTED:
> # direct descent through Predeclared(6) reaches the effect but BYPASSES the sole
> # seam that constructs the Specialization(1) response owner's context frame.
> #
> # ARCHITECT CLASSIFICATION: a proven bypass of an EXISTING authority — NOT a new
> # availability kind, owner transfer, continuation form, or carried aggregate.
> # The exact ruled repair (4 steps, full detail in the event) REUSES the landed
> # creation-site frame: widen the immediate checked-IH bridge's closed descriptor
> # by ONE exact shape (CheckedComputationalIHSlots{body: Match{scrutinee:
> # Var(operation_field)}} under a statically-known ITree::Vis), run the existing
> # assemble_continuation_call_operands / ConstructedContextFrame route
> # (core.rs:10853-10859) in DirectEmission to build the frame, THEN iota-select
> # the statically-known operation case and bind its lowered fields directly — no
> # FSOp carrier. Keyed STRUCTURALLY, never on ctor_551/MappingAcquireFile/FSOp/
> # origin numbers. Full "not authorized" fence + required controls/discriminators
> # in the event (no new claim/coordinate/slot kind; no owner transfer; no
> # continuation/carrier form; no verify_entry_frame relaxation; MappingAllocate
> # scalar path byte-unchanged; all D5b native/interp differentials green).
> #
> # STEWARD SCOPE CALL: AMEND D5b IN PLACE — do NOT cut a predecessor. Grounds:
> # (1) the bounded bridge-widening has NO independent consumer — it exists solely
> # to co-locate D5b's operation selection with its response ownership, and its
> # ONLY acceptance is D5b's own native==interp COW differential, so a predecessor
> # node would have no standalone green (the B-consumes-unbuilt-A decomposition
> # defect) and would lengthen the priority lane's critical path for zero benefit;
> # (2) one predicate, one differential, one reviewer set. This amendment
> # EXPLICITLY LIFTS the frame's own "frozen wire / no new form / no bridge
> # widening" ban FOR THIS ONE BOUNDED SHAPE ONLY (the closed-descriptor widening
> # above), reconciling the ban with the authorization the Architect just granted —
> # the D8m closed-descriptor comment and its exhaustive controls are UPDATED, not
> # contradicted silently. Nothing else in the frozen-wire / no-4th-op ban relaxes.
> # This is packaging, not design (Architect); it grows no TCB (the fence forbids
> # it), so it is the Steward's call and needs no operator sign-off.
> #
> # RUNTIME: HOLDS CLEAN at b94c5ae7 (backend/COW work + offset-less prelude
> # alignment PRESERVED) until this kick; then builds the ruled route. Candidate
> # returns to Architect (required) + runtime-qa + CI -> Steward M1-M4 ->
> # lieutenant. Any THIRD hard stop on this same static-operation/response-owner
> # question returns to the Architect BEFORE a new attempt (research trigger not
> # yet fired).

> # D5b RECUT 2026-09-11 (Steward) — §1b HS#3 STRUCTURAL CLOSURE, not a point fix
> # (Architect ruling evt_7c1adrctc3qf1, thr_7wy5wy45p7abm). D5b hit a hard stop
> # (runtime-implementer evt_6gejx0s8665qw): the frozen §1.9 FileBacked route in
> # withMapping must eval `eq_int offset 0` in-body before emitting the offset-less
> # MappingAcquireFile wire; calling eq_int on the carried-eliminated offset hits
> # the BoundaryCarrier wall — the SAME as the rejected D5a equality precheck.
> # Architect confirmed this is the 3rd hard-stop KEYED on the FORMING PREDICATE
> # (see below), so the fix is the general SURFACE RULE, not a 4th point ruling.
> #
> # THE STRUCTURAL CLOSURE (the ruled 1:1 surface-wire rule): the checked mapping
> # surface must be in 1:1 correspondence with the frozen wire — every checked
> # parameter passes through to a wire field; NONE is validate-and-discard. Entries
> # 1 (mapView token dropped -> window-direct) and 2 (mapWrite window-length dropped
> # -> payload carries extent) already closed this way; entry 3 gets the same move:
> # DROP the FileBacked offset field, `FileBacked (Resource FsHandle) Int Int` ->
> # `FileBacked (Resource FsHandle) Int` (length only), matching the offset-less
> # wire — make-illegal-states-unrepresentable, no eq_int, no BoundaryCarrier.
> #
> # RECUT SCOPE (replaces the point-fix framing): (a) §1.9 mapping-surface
> # 1:1-with-wire correction dropping the FileBacked offset (Path B, SPEC's call —
> # deciding question routed to spec-author/spec-leader: is a non-zero file offset
> # ever meaningful, or always 0? Expected always-0 per z4080's offset-less wire =>
> # drop it, §1.9 + seed-mapping update, Architect review + CV Spec-lane ->
> # spec-leader gate; Path A = a frozen-wire ABI change = Steward rescope, NOT
> # expected as it contradicts z4080); (b) a WHOLE-SURFACE validate-and-discard
> # CENSUS (Anonymous length, read window/mapBytes, mapWrite, protection,
> # withMapping source) proving every param is wire-carried 1:1 or removed = no 4th
> # entry (three known, two closed, this closes the third); (c) the checked-surface/
> # prelude alignment (drop the FileBacked offset field + remove the eq_int site),
> # riding the D5b WP. RETAINED as VALID (Architect): D3/D4/D5a landed; the D5b
> # native mmap-of-fd/MAP_PRIVATE backend + 0x0407 promotion + COW differential in
> # WIP ae014a36 (wp/ABI-S6-d5b-file-backed) — dropping the offset removes the
> # boundary check and turns the intentionally-red COW differential green. Runtime
> # ring HOLDS the boundary red (no in-body check, no carrier machinery — Architect)
> # until the surface correction lands. NO research pull (Architect §1a: prior art
> # has nothing further; the ITree single-unconditional-Vis advisory already
> # supports the shape; the predicate is our own surface/wire mismatch, 1:1 is
> # known-best). Runtime seat: implementer gpt-5.6-sol/high = T1. The CV seed-case-3
> # ResourceKindMismatch(Mapping) residual is a small nonblocking companion on a
> # DIFFERENT axis — not part of D5b (see the LANDED banner below).
> #
> # D5a-surface D1 LANDED 2026-09-11 (Steward) — origin/main 8c6136fa3 currently;
> # the D1 candidate bf84c1b1 landed at d8bbef963, blob-verified 3/3
> # (abi_s6_mapping_surface_native.rs, prelude.rs, px8f_buffer_io_surface.rs). Its
> # prerequisite, the §1.9 FORK below, resolved PATH B: the spec-author correction
> # (offset-based mapWrite window IS the payload extent) landed at d27bd8d13 (PR
> # #3483, spec-only), which discharged the mismatch seed and let D1 build. Path A
> # (independent length seat, host write-wire prerequisite) did NOT fire — no
> # Steward rescope. Gates on exact bf84c1b1: Runtime QA + Architect M4 + Adversary
> # NO-DEFECT (evt_78eaxbvtjbde8) + CV APPROVE (evt_5fgekq). ABI-S6 STAYS ACTIVE —
> # two things remain: (1) D5b MAP_PRIVATE file-backed COW seed (still RED/deferred,
> # the three-op wire frozen); (2) a CV RESIDUAL — seed case 3
> # ResourceKindMismatch(Mapping) arm is unnetted, so the merge is unblocked but
> # seed-case-3-full-green is gated on a small Mapping->buffer-op differential
> # (follow-on requested by CV). Decide post-land whether the residual folds into
> # continuing D5a or needs its own small coverage node (like
> # RT-MAPPING-DISPATCH-CONTROL-COVERAGE) — small, nonblocking. The FORMING
> # PREDICATE (native mapping surface admits no in-body control at an access site)
> # did NOT reach its HS#3 structural-closure trigger on this landing.
> #
> # D5a-surface D1 HS#2 (mapWrite bounds class) — Architect ruling 2026-09-10
> # (evt_5xsn7eb40xj8j, thr_2b9zky9skt6hc). CONFORMANCE / SURFACE-HONESTY gap, NOT
> # a soundness hole: the landed mapWrite bounds the write by the PAYLOAD extent
> # (host-checks start + bytes.len() <= extent), so no OOB/unchecked access — TCB
> # intact. The unmet promise is §1.9's claim that the DECLARED window (offset,
> # length) is checked: mapWrite drops the length on the frozen wire
> # (PrivateMappingWriteView carries no length seat), so mapping_window_length is
> # silently ignored. Semantic relation ruled EXACT; the declared write-window
> # length is REDUNDANT (the payload Bytes carries the extent; a write window
> # larger than its payload is incoherent under MAP_PRIVATE COW). The checked-
> # surface conditional precheck is REJECTED — it violates §1.9's own
> # no-in-body-response-transform invariant (764-766) and re-hits the native
> # BoundaryCarrier wall; do NOT add a wire field / 4th op / continuation
> # primitive, and do NOT fund a runtime-lowering change to transport a
> # conditional Ret/Vis at a mapping access.
> #
> # THE §1.9 FORK (contract-level, Spec's call — deciding question routed to
> # spec-author/spec-leader): is an INDEPENDENTLY-declared write-window length
> # (one that can differ from the payload) a meaningful part of the mapWrite
> # contract, or is the write window ALWAYS the payload extent?
> #   - Path B (Architect RECOMMENDED, expected under MAP_PRIVATE/COW scope):
> #     correct §1.9 to state mapWrite's checked window IS the payload extent
> #     [offset, offset+len(bytes)), bounds-checked against the extent (the landed
> #     host already does this); MappingWindow.length is not an independent write
> #     parameter. Spec-only (spec/ + conformance/ seed-mapping update), NO ABI
> #     change, NO build-lane rescope. Then Architect required review + CV
> #     Spec-lane. The landed mapWrite STANDS as correct for the binding property.
> #   - Path A (only if Spec names a real reason the independent length must be
> #     honored): add a length seat to the write op (host bounds-checks declared
> #     (offset,length) vs extent AND enforces length == bytes.len(); single
> #     unconditional Vis). Frozen-wire ABI change rippling to interp/native/
> #     planner MappingWriteView consumers — the Steward rescopes a small host-side
> #     write-wire PREREQUISITE node. Architect required review binds it.
> # STEWARD rescope/prerequisite call is made AFTER Spec answers (recorded here so
> # it is not lost). Meanwhile the mismatch seed case is BLOCKED-ON-§1.9-FORK, not
> # on the runtime ring; the held WIP 64453ef6 (matching-extent mapWrite bounds
> # control + 1/4097 acquisition parity) is VALID and becomes the candidate once
> # the fork resolves.
> #
> # SYMPTOM INVENTORY — D5a-surface composition (Architect §1a; HS#2 = z-count 2):
> #   1. (z4091 / HS#1) mapView mints a MappingSpan with no frozen-wire
> #      counterpart, forcing a body-level response transform native cannot
> #      transport — an abstract view-token carrying no invariant the request
> #      window does not. Resolved by the window-direct §1.9 respin (b33f8ac9b).
> #   2. (z4116 / HS#2) mapWrite's declared-window-length check cannot be enforced
> #      over the frozen wire without a checked-surface conditional Ret/Vis, which
> #      native cannot transport (BoundaryCarrier) — the mapping access site cannot
> #      carry any response shape beyond a single unconditional Vis.
> #   3. (HS#3, D5b) FileBacked carries a file offset the offset-less
> #      MappingAcquireFile wire drops after an `eq_int offset 0` check —
> #      BoundaryCarrier over the carried eliminated offset — keyed on a
> #      validate-and-discard checked-surface parameter absent from the frozen wire.
> #      Same predicate as 1-2; closed STRUCTURALLY by the 1:1 surface-wire rule
> #      (drop the offset field), not a point fix (Architect evt_7c1adrctc3qf1).
> # FORMING PREDICATE — NOW FIRED as the §1b structural-closure trigger (HS#3
> # landed 2026-09-11): "the checked mapping surface carries a VALIDATE-AND-DISCARD
> # parameter the frozen wire does not carry, forcing an in-body computation over a
> # carried eliminated value native owner-lowering cannot transport — every op can
> # only emit a single unconditional Vis." The three entries are ONE defect;
> # Architect ruled the general surface rule (checked surface 1:1 with the frozen
> # wire — every param wire-carried or removed, NONE validate-and-discard), NOT a
> # 4th point ruling. NO research pull (Architect §1a: prior art has nothing
> # further; ITree single-unconditional-Vis advisory already supports the shape).
> # Path B CONFIRMED by spec-author (evt_2gbnzp1qepvc8): a non-zero file offset is
> # never meaningful for FileBacked (always 0) — §1.9 drops the offset field; no
> # Steward rescope (Path A did not fire).
> #
> # D5a-surface D1 RE-RELEASED 2026-09-10 (Steward) — all three prerequisites
> # LANDED; runtime ring resume authorized. The held multi-op acceptance resumes
> # on current main d70db3299. The prerequisite frame (5bf1915e4, HS#4) named
> # RT-MAPPING-MULTIOP-DISPATCH as the blocker for D5a-surface's remaining multi-op
> # acceptance; it MERGED at 1c48b6c5c (blob-verified 17/17). With that, the three
> # things D5a-surface D1 composes over are all on main:
> #   - D5a-core native anonymous wire (29f64ff6f) — real mmap/munmap three-op wire;
> #   - window-direct §1.9 surface contract (b33f8ac9b) — mapView/MappingSpan
> #     dropped, MappingWindow passed directly to mapBytes/mapWrite (this is what
> #     dissolved the D0 continuation blocker, Architect evt_56e6jx62s0qpb);
> #   - RT-MAPPING general N>=2 same-producer dispatch (1c48b6c5c) — the runtime
> #     machinery that lets a SECOND mapping effect sequence and execute.
> # So D0 is resolved (via the respin) and D1 is buildable. RESUME D5a-surface D1
> # on a base carrying d70db3299: build the three checked procs (withMapping /
> # mapBytes / mapWrite) over the frozen three-op wire, discharge the TWO achievable
> # seed cases (4 KiB granule + opacity/bounds) with controls intact, and the
> # multi-op acceptance (a second mapping effect sequenced after the first executes
> # and matches interp) that RT-MAPPING unblocked. The MAP_PRIVATE COW seed STAYS
> # RED (BLOCKED-ON-D5b), the three-op wire is FROZEN, no fourth op, no §1.9
> # alteration — route to Steward+Architect on any of those (hard stops unchanged).
> # Re-measure the fixed inputs at pickup. Reviewers: Runtime QA + Architect
> # (required, TCB-adjacent) + Adversary -> Steward M1-M4 -> lieutenant.
> #
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

CURRENT POSITION (2026-09-10, measured at `origin/main` `9d614db31`): D0-D4 have
merged — the represented `Mapping` operation surface (acquire / bounded views /
registration) landed on **anonymous** and **file-backed** backing, fail-closed
and `RepresentedUnavailable` (D3 PR #3444 adversary M8 NO DEFECT; D4 file-backed
acquire landed cf894cdb5). The normative **§1.9 mapping-surface contract + seed**
landed (`cd62283b8`, spec `30-surface/38-ffi-io.md §1.9` +
`conformance/surface/ffi-io/seed-mapping.md`), so the checked surface and its
discriminating negatives are now pinned. **The Architect then SPLIT the native
D5a increment into D5a-core and D5a-surface** (relayed by runtime-leader
evt_580zc1y1jknn0 / evt_6x2krr2wfvax8): D5a-core owns the native anonymous
promotion (the frozen three-op wire), currently a candidate `7beecdb4e` on
`wp/ABI-S6-native-anonymous` under fresh Runtime-QA + Architect review; D5a-surface
owns the checked §1.9 composition over that frozen wire + the seed discharge (the
increment framed below). The node stays `active` across increments.

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
  Whichever increment (D2 or D3) first makes the parallel `Mapping`
  `ResourceKind` tag reachable to the native reifier also closes the reification
  totality gap — see AC-NATIVE-REIFICATION-TOTAL.
- **D4 — file-backed mapping acquire (the FsHandle-backed backing axis).** D1-D3
  landed the represented `Mapping` surface on anonymous backing; D4 adds the
  file-backed acquire path — a mapping-acquire that takes a **held `FsHandle`**
  and shares/derives its revocation lineage from it (AC-LIFETIME-REVOCATION
  already binds this), exercising the source-`FsHandle` lineage/rights rule and
  the file-backed backing axis. The `HostEffectBackendV1` seam for the real
  `mmap`-of-fd / `munmap` is shaped as in D1 but lands `RepresentedUnavailable`;
  no `SYS_*` fact (native promotion is a separate later increment). Deny by
  default: acquisition requires the governing right on the source handle, and a
  file-backed region cannot outlive a revocation of the file's lineage.
  **The capacity-governance fork (AC-MAPPING-CAPACITY-GOVERNANCE) is a hard stop
  to the ARCHITECT before implementation** — whether the mapping surface adopts
  per-mapping and invocation-wide limits or an explicit unbounded-but-graceful
  contract is a design ruling, not this frame's call; land nothing on that axis
  until ruled.
- **D5a-core — native anonymous promotion (the frozen three-op wire).** Promotes
  the anonymous `Mapping` acquire/view/release wire from `RepresentedUnavailable`
  to native (real `mmap`/`munmap`), closing the native-reifier totality gap for
  the `Mapping` tag (AC-NATIVE-REIFICATION-TOTAL). Owned by the Architect's
  split; a separate candidate `7beecdb4e` on `wp/ABI-S6-native-anonymous`
  under fresh Runtime-QA + Architect review against the seven core gates (native
  reachability, opacity, 4 KiB shared accounting, MAP_PRIVATE/munmap, reifier
  totality, no new right/TCB, seed still honestly undischarged). Not this frame's
  deliverable — recorded here for the increment plan; it routes to the Steward on
  its own QA/Architect approval.
- **D5a-surface — the checked §1.9 composition over the frozen three-op wire
  (D0-first; THIS is the framed/released increment).** Compose the public checked
  Mapping surface — `withMapping` / `mapBytes` / `mapWrite`, exactly the **three**
  `spec/30-surface/38-ffi-io.md §1.9` procs — over the frozen three-op native wire
  D5a-core delivers, and DISCHARGE the **two ACHIEVABLE** RED-UNTIL-BUILT seed
  cases (4 KiB granule + opacity/bounds) in
  `conformance/surface/ffi-io/seed-mapping.md`. The MAP_PRIVATE COW seed case
  DEFERS to D5b (file acquisition) — see AC-SEED-DISCHARGE-D5A and the D5b
  deliverable below. Grounded on the landed **window-direct** §1.9 contract
  (`b33f8ac9b`, the respin that dropped `mapView` / private `MappingSpan` and
  passes `MappingWindow` directly to `mapBytes` / `mapWrite`; supersedes the
  earlier `cd62283b8` four-proc contract).
  - **D0 — RESOLVED 2026-09-10 (Architect evt_56e6jx62s0qpb -> §1.9 window-direct
    respin, landed `b33f8ac9b`).** The bracket did NOT compose natively over the
    original four-proc contract — `mapView` returned a `MappingSpan`, a
    capture-bearing continuation the native path cannot own. The Architect ruled
    the defect UPSTREAM: `mapView` / private `MappingSpan` did not earn their keep
    (they copied the `BufferSpan` mechanism without the capping condition that
    justifies it; a mapping's live subrange equals its window when live, so
    `MappingSpan` carried no invariant `MappingWindow` doesn't). The fix was the
    window-direct §1.9 respin, NOT a native continuation prerequisite; the
    represented-K seam is not needed. The original D0 characterization follows for
    the record.
  - **D0 (original) — characterize the continuation prerequisite; HARD-STOP
    if unmet.**
    `withMapping`'s bracket body `MappingHandle -> HostIO a (ResourceBodyResult e
    r)` composes a checked resource bracket exactly as `withBuffer` (`§1.7.1`) /
    the `FsHandle` acquire/release bracket do. D0 measures whether that checked
    bracket composition is buildable on the EXISTING resource-bracket /
    continuation machinery, or whether it requires a continuation capability the
    runtime does not yet have (the represented-K seam). Report the exact
    prerequisite and HARD-STOP to the Steward + Architect if it is unmet. Do NOT
    build new continuation machinery here, do NOT alter the §1.9 contract to route
    around it, and do NOT add an operation — any of those is out of this increment's
    scope and is a distinct node the Architect must rule.
  - **D1 (D0 cleared via the respin) — build the checked composition + discharge
    the two achievable seed cases.** Implement the three checked procs
    (`withMapping` / `mapBytes` / `mapWrite`) over the frozen three-op wire and
    turn the TWO achievable seed cases green WITH their discriminating controls
    intact: the fixed 4 KiB host-independent granule (1 B -> 4096, 4097 B -> 8192;
    native == interpreted), and opacity + bounds (no raw address in any Ken value
    or token; out-of-range `mapBytes` / `mapWrite` is a fail-visible
    `ResourceError`; a wrong-kind token is `ResourceKindMismatch` naming
    `Mapping`; `ReadOnly` refuses `mapWrite`). The MAP_PRIVATE COW case STAYS RED,
    BLOCKED-ON-D5b (file acquisition / `MappingAcquireFile`) — NOT weakened or
    deleted; discharging it is D5b's acceptance. Rationale (Steward ruling A,
    Architect-confirmed evt_5en9jgjb24the): COW is intrinsically file-backed —
    "writes through a file mapping do not reach the backing file" has NO anonymous
    analog — so it needs `withMapping (FileBacked ...)` -> `MappingAcquireFile`, a
    fourth op outside D5a-surface's frozen-three-op / no-new-op boundary.
  - **Constraints (hard stops, per the Architect split).** No alteration of the
    §1.9 contract to match D5a-core; no new operation beyond the three §1.9 procs
    (`withMapping` / `mapBytes` / `mapWrite`); the three-op native wire is frozen.
    Route to the Steward + Architect on any of these rather than absorbing it.
- **D5b — native file-backed mapping acquisition + the MAP_PRIVATE COW discharge
  (the COW successor; framed here per Steward ruling A, Architect-confirmed
  evt_5en9jgjb24the).** Promote the file-acquisition op `MappingAcquireFile` — the
  `withMapping (FileBacked ...)` route that D4 landed and the D5a-core split
  explicitly kept `RepresentedUnavailable` (Architect z4088) — to native (real
  `mmap`-of-fd / `munmap`), so a real file-backed MAP_PRIVATE mapping can be
  exercised. This is the fourth op OUTSIDE D5a-surface's frozen three-op wire, so
  it is its own increment (Architect's z4085/z4088 staging: D5a = anonymous
  MappingAllocate / ReadView / WriteView + ResourceRelease; D5b = file
  acquisition). DISCHARGE the deferred MAP_PRIVATE COW seed case
  (`conformance/surface/ffi-io/seed-mapping.md` case 1, keyed
  `BLOCKED-ON-ABI-S6-D5b`): a write through a MAP_PRIVATE file mapping is observed
  in-mapping but does NOT reach the backing file (the ordinary-file read observes
  the ORIGINAL bytes), so a MAP_SHARED / write-through surface reds — the
  discriminating control intact; a green-vs-green pass against its named
  non-conforming implementation is vacuous and is a HARD STOP, not a discharge.
  **The Architect is D5b's REQUIRED reviewer** (evt_5en9jgjb24the): the COW
  discriminator is soundness-relevant, and the file-backed lineage/rights rule (a
  file-backed region shares/derives its revocation lineage from the source
  `FsHandle` per AC-LIFETIME-REVOCATION) is the Architect's gate. The
  capacity-governance fork (AC-MAPPING-CAPACITY-GOVERNANCE) applies to any
  file-acquisition capacity ruling not already settled at D4.

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
- **AC-NATIVE-REIFICATION-TOTAL (D2/D3 seam; adversary bounded obs on D1,
  evt_7cnqjqpd9zv1e / lesson bbc87dc2b).** D0 ruled a PARALLEL `Mapping`
  `ResourceKind` and D1 landed it as an unreachable, fail-closed tag (correctly
  NOT touching the native reifier — no opcode makes it reachable). The native
  `ResourceKind` reification path is therefore currently NON-total over the
  closed sum for the `Mapping` tag: no `SynthesizedFixedConstructorRole::
  ResourceKindMapping` role, `ALL = [Self; 49]`, and `resource_kind_value` has
  only two `DynamicConstructor` alternatives — a native `Mapping` resource error
  would hit `malformed_dynamic_constructor_trap` rather than reify. In the SAME
  increment (D2 bounded-view / D3 registration) that first makes the `Mapping`
  tag reachable to the native reifier, extend that path: add
  `SynthesizedFixedConstructorRole::ResourceKindMapping` plus its process symbol
  and the third `resource_kind_value` alternative (native lowering,
  `crates/ken-runtime` `lowering/effects.rs` ~`:4052`), so the reifier is total
  over the closed `ResourceKind` sum. Control: a native `Mapping` resource error
  reifies to its typed Ken constructor rather than trapping; a mutation dropping
  the `ResourceKindMapping` alternative reds a named test. This is NOT a D1
  defect — D1's tag is unreachable and fail-closed — it binds whichever
  increment first makes the tag reachable.
- **AC-MAPPING-CAPACITY-GOVERNANCE (D4; adversary bounded obs on D3,
  evt_n041yasg5m26).** `MappingAllocate` currently has NO capacity governor:
  `try_new_anonymous` (`effect_v1.rs:1044`) rejects `length == 0` and fails
  gracefully on a huge length (`try_reserve_exact` -> `AllocationFailed`, no
  OOM-abort), but there is no per-mapping cap and no invocation-wide live-mapping
  accounting — unlike `BufferAllocate`, which enforces `per_buffer_max_capacity`
  (1 MiB, sealed catalog `buffer.per_buffer_max_capacity|1048576`) and
  `invocation_max_live_capacity` at `insert_buffer` (`effect_v1.rs:1266`). This
  is inert in D1-D3 (`MappingAllocate` is unreachable from Ken source — no
  producer syntax) but goes LIVE the moment a source program can emit
  `MappingAllocate` with an attacker-influenced `length: u64` or multiplicity: a
  represented-side resource-exhaustion vector capped only by the process
  allocator. D4 must CONSCIOUSLY rule — as an Architect hard stop BEFORE
  implementation (see Hard stop) — whether a `mapping.per_mapping_max_capacity`
  and an invocation-wide mapping limit belong in the sealed ABI, or whether
  unbounded-but-graceful is the intended contract (mappings are semantically
  meant to be larger than buffers, so unbounded MAY be by design). Whichever is
  ruled, it is a DELIBERATE ABI ruling, not a silent gap. Control: if a limit is
  adopted, an allocation past the per-mapping cap and the (N+1)th live mapping
  past the invocation cap each red a named test, and the control must fail
  against a governor-neutered tree (non-vacuous); if unbounded-but-graceful is
  ruled, a huge / many-mapping request still fails closed with the typed refusal
  (never an OOM-abort) and that graceful-failure path is asserted.
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
- **AC-SEED-DISCHARGE-D5A (D5a-surface — the point of the increment; scoped to the
  TWO achievable cases per Steward ruling A, Architect-confirmed
  evt_5en9jgjb24the).** The **two** anonymous-composition-achievable
  `conformance/surface/ffi-io/seed-mapping.md` cases flip from RED-UNTIL-BUILT to
  GREEN, each with its discriminating control still refuting a non-conforming
  implementation — the pair, not a lone positive: the **4 KiB granule** case — the
  charged sizes are exactly 4096/4096/8192 and NATIVE == INTERPRETED, so a
  `sysconf(_SC_PAGESIZE)`-derived or byte-granular rule reds (host-independent by
  construction); and the **opacity + bounds** case — no raw address in any Ken
  value or token, out-of-range `mapBytes` / `mapWrite` is a fail-visible
  `ResourceError`, a wrong-kind token is `ResourceKindMismatch` naming `Mapping`,
  and `ReadOnly` refuses `mapWrite`. **The MAP_PRIVATE COW case is OUT of
  D5a-surface's scope and STAYS RED, keyed `BLOCKED-ON-ABI-S6-D5b`.** It is
  intrinsically file-backed — "writes through a file mapping do not reach the
  backing file" has NO anonymous analog — so discharging it needs
  `MappingAcquireFile`, the fourth op D5a-surface's frozen wire forbids; it is
  discharged by D5b, NOT here, and its discriminating assertions are NOT weakened
  or deleted in the interim. Control: neutering either achievable discriminator
  (host-page accounting, address exposure, unchecked view) reds the matching case;
  a case that passes green-vs-green against its named non-conforming implementation
  is vacuous and does NOT discharge. The seed text is not altered to make a case
  pass — a case that only passes after weakening its control is a HARD STOP.
  **Honesty of the gated axis (capability-gate lifecycle).** While the COW case is
  dormant behind D5b, the live opacity/bounds case and every case's
  native==interpreted parity assertion keep opacity, bounds-not-clamp, and parity
  enforced across the whole D5b interval — the axis has a live enforcer throughout,
  never an unguarded gap.

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

The D4 mapping capacity-governance fork (AC-MAPPING-CAPACITY-GOVERNANCE:
per-mapping / invocation-wide limit versus an explicit unbounded-but-graceful
contract) is likewise a hard stop to the ARCHITECT, before D4 implementation. It
is a security-relevant ABI-shape ruling — like the D0 fork, a design decision,
not a scope question — and D4 lands nothing on that axis until the Architect
rules it.
