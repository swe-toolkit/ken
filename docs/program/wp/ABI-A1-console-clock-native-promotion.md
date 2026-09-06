# ABI-A1 — promote ConsoleRead and ClockWallNow to NativeTested with a normalized differential

**Owner:** Team Runtime (`runtime-leader` + `runtime-implementer` +
`runtime-qa`). **Branch:** `wp/ABI-A1`. **Size:** L. **Tier:** T1.
**Risk:** medium — it implements native execution at the host trust boundary and
edits the file every ABI consumer reads; the correctness content is the native
leg plus the differential design, not the flip.

> ## D0 REFRAME 2026-09-06 — native execution is absent; the native leg is now IN
> ## scope (subsume, don't proliferate).
>
> The first cut sized this M and assumed native `ConsoleRead`/`ClockWallNow`
> execution existed and needed only a differential. Runtime's D0 (runtime-leader
> evt_127e2d2gwmyx9, at base `05be823ad`) measured otherwise and took the frame's
> named hard stop cleanly: `ProcessHost` has neither `console_read` nor
> `clock_wall_now` (Unsupported/empty defaults); `ken_host_dispatch_v1` has no
> decode arm for either and falls to `-3`; the manifest request forms mark native
> execution deferred (`abi_v1.rs:152-262`). A differential would fake the boundary.
> Steward ruling: NativeTested *means* native execution, and building these two
> legs is committed ABI-program scope (`10-linux-abi-completion.md §3`: "it works
> in the interpreter is not the bar"), the same trust surface as the 13 existing
> NativeTested ops — so rather than mint a separate predecessor and hand off within
> one ring, the native leg is folded into this node (new deliverable `D-NATIVE`,
> size M→L). Same node, same ring, same `wp/ABI-A1` branch; nothing was started, so
> no rebase. Per-operation accepted partials are fine (ClockWallNow is the simpler
> leg).

**Authority:** `docs/program/10-linux-abi-completion.md §4`, Track A, ABI-A1
(`:124`, `:128`). **Status:** Steward frame, shovel-ready. `depends_on:
[ABI-REVOKE]` — merged (`3e1b21cf1`). Released to the runtime ring on the
operator's 2026-09-05 ABI-A ruling and 2026-09-06 concurrence to run Track A
now.

---

## 1. Why this is its own slice, and what the whole judgment is

Track A is split by **evidence shape, not by count.** ABI-A1 is the
console/clock slice: `ConsoleRead` and `ClockWallNow` are **nondeterministic
observations**. A wall-clock read returns a different instant on every call; a
console read returns whatever the injected stdin supplies. So the differential
that gates every other NativeTested operation — native execution byte-equal to
interpreter execution — **is the wrong instrument here**, and using it either
fails spuriously or is quietly weakened to pass.

**The whole T1 content of this WP is designing the normalized differential**:
the invariant that native and interpreter execution must share for a
nondeterministic observation, stated per operation, strong enough that a real
divergence still reddens it. That design does not generalize to A2/A3 (path
policy, directory mutation), which is why this is a separate slice.

## 2. Fixed inputs (measured at `origin/main = a8ee55b2f`; RE-MEASURE at your cut)

| path | fact |
|---|---|
| `crates/ken-host/src/effect_v1.rs` | blob `430a00df2c24c2bd0b55389c8057c4c2abb390bb` |
| `effect_v1.rs:138` `availability()` | exhaustive `match`, no wildcard |
| `effect_v1.rs:140` | `ConsoleRead => RepresentedUnavailable` — the flip target |
| `effect_v1.rs:144` | `ClockWallNow => RepresentedUnavailable` — the flip target |
| `effect_v1.rs:220` `NATIVE_TESTED_TARGETS_V1` | `[HostOpV1; 13]`, the promotion roster; add the two here → 15 |
| `effect_v1.rs:259` `native_tested_count` | `= NATIVE_TESTED_TARGETS_V1.len()` — recomputes automatically, do not hand-edit |
| `effect_wire.rs:96` / `:114` | `CanonicalRequestV1::ConsoleRead { stream, limit }` / `ClockWallNow` wire forms exist |
| `eval.rs:5988` | interpreter execution path for `ConsoleRead` exists |
| `crates/ken-cli/tests/rt_escape_second_resource_native.rs:94` | `assert_native_matches_interpreter` — the existing native-vs-interpreter differential harness pattern (the substrate you extend) |

**The native execution path and the differential harness pattern both already
exist** — this is a promotion, not a from-scratch build. D0 confirms whether
native `ConsoleRead`/`ClockWallNow` execution is wired end to end or needs the
last leg; if a native execution leg is genuinely absent, that is a hard-stop
finding (§6), not something to fabricate around.

## 3. The design, front-loaded: the normalized differential per operation

A NativeTested promotion means native execution is proven to agree with the
interpreter under a differential. For these two the agreement is **normalized**,
and the normalization is the deliverable. Author it as an explicit projection
applied to BOTH sides before comparison, never as a relaxed assertion on one
side.

- **`ClockWallNow`.** Do not compare instants. The honest agreement is
  structural and invariant-shaped: native and interpreter return the **same
  response shape and field layout**; two successive reads on one side are
  **monotone non-decreasing**; a read sits within a test-controlled plausible
  window. The normalizer erases the instant and keeps the shape and the
  ordering relation. A differential that asserts native instant == interpreter
  instant is the wrong instrument and must be shown to be so (AC-3).
- **`ConsoleRead`.** Drive both sides from the **same injected stdin fixture**
  (fixed bytes, a fixed EOF point). The agreement is over the **observation
  discipline**: same byte count returned for the same `limit`, same
  partial-read / EOF classification, same wire response for the same input —
  not over any wall-clock or scheduling artifact. The normalizer pins the input
  so the only remaining variation is the operation's own behavior.

## 4. Deliverables

- **`D0` — DONE (runtime-leader evt_127e2d2gwmyx9).** Native execution absent for
  both operations; the native leg is folded in as `D-NATIVE`. Re-derive the
  `effect_v1.rs` blob at your cut and report the exact differential harness you
  will extend (crate + test file) and the injected-input mechanism per operation.
- **`D-NATIVE` — implement native execution for both operations (the new
  load-bearing leg).** Following the existing NativeTested console pattern:
  - Add `console_read` and `clock_wall_now` to the `ProcessHost` impl alongside
    `console_write`/`console_flush`/`console_is_terminal`
    (`crates/ken-runtime/src/object_linker_packaging.rs:3684`). `ConsoleRead
    { stream, limit }` is a bounded `read` on the named stream; `ClockWallNow` is
    a wall-clock read (`CLOCK_REALTIME`-shaped), returning the manifest response
    form.
  - Add the decode/execute arms in `ken_host_dispatch_v1`
    (`crates/ken-host/src/abi_v1.rs:1179`) for `ConsoleRead` (`0x0101`) and
    `ClockWallNow` (`0x0201`), removing their "native execution deferred"
    dead-code markers (`:152-262`).
  - Update `every_deferred_operation_has_its_own_named_native_boundary_rejection`
    (`abi_v1.rs:1834`) so these two are no longer in the deferred-rejection set.
  Re-measure every line at your cut. If a leg needs a substrate this WP should not
  build (e.g. blocking-stdin lifecycle beyond what console-write's leg already
  assumes), that is a hard-stop finding (§6), not scope creep.
  - **Sequencing (Steward ruling 2026-09-06, evt_2eyphkzmew385): SPLIT,
    ClockWallNow FIRST.** ClockWallNow needs no new backend substrate; land it as
    an accepted per-op partial (native leg + differential + promotion + its own
    AC-4 catalog status flip), Architect + Runtime QA -> M1-M4 -> lieutenant, before
    ConsoleRead.
  - **ConsoleRead substrate AUTHORIZED within ABI-A1 (Architect design ruling, this
    thread; Steward scope authorization).** ConsoleRead's sound design is a NEW
    governed leaf planner node for a host-reply-generated persistent-store referent
    with exact singleton owner `{PersistentStore}` (the existing Absent/Scalar/
    SiteOperand each falsely state the child). It must have its own leaf resolution
    path, avoid operand reconciliation, be exhaustively handled across the backend
    matches, leave all existing mappings/owner sets byte-identical, and carry
    exact-owner mutation + real Chunk+Eof parity + ill-formedness + native-ID/role
    affected-closure checks. The Architect confirmed it is backend planner
    substrate, NOT a kernel/TCB addition, so it is inside this WP's native-execution
    scope: no separate node, no operator TCB call. It is the SECOND per-op partial.
- **`D1` — the normalized differentials.** One per operation, per §3: an
  explicit normalizer applied to both native and interpreter observation, then
  equality on the normalized value. State the invariant each encodes in prose
  alongside the assertion.
- **`D2` — the promotion.** Flip `availability()` at `:140` and `:144`
  `RepresentedUnavailable → NativeTested`; add both to
  `NATIVE_TESTED_TARGETS_V1`. `native_tested_count` follows automatically —
  do not touch it. **And flip the native/availability status of exactly these two
  operations in `effect_abi_v1.catalog` (`unavailable → native`)** — the ABI-R3
  closure invariant (`abi_v1.rs:1785-1796`) requires catalog-native-status iff
  runtime-NativeTested, so this status edit is part of the promotion, not a
  violation of it; the generated manifest hash changes honestly because status
  changed. Preserve every wire numeric id, record layout, arity, and
  `operation_count`.
- **`D3` — the negative control that proves each differential discriminates.**
  Per operation, a deliberately wrong native observation (a clock read that goes
  backwards; a console read that returns the wrong byte count for the fixture)
  must redden the normalized differential. A differential only ever run against
  the correct implementation is not evidence it can catch a regression.
- **`D4` — the exact-equality control, shown to be wrong.** Demonstrate (in a
  comment or an ignored companion, not a live gate) that a naive
  instant-equality differential for `ClockWallNow` fails on a correct
  implementation. This is what records WHY the normalizer exists, so the next
  reader does not "simplify" it back to exact equality.
- **`D5` — census tail.** State which `RepresentedUnavailable` operations remain
  after this WP (expect 7: the two Clock siblings, the five Fs ops A2/A3 cover,
  and Entropy), so A2/A3 are framed against a measured surface.

## 5. Acceptance criteria

- **`AC-0` — native execution actually runs (the D-NATIVE leg).** Control: with
  the decode arms in place, `ken_host_dispatch_v1` no longer falls to `-3` for
  `0x0101`/`0x0201` — each executes and returns its manifest response form; the
  `every_deferred_operation_has_its_own_named_native_boundary_rejection` test is
  updated so the two are removed from the deferred set and no other operation is.
  A differential that runs against a still-faked native boundary fails this AC.
- **`AC-1` — both operations are NativeTested and prove it under the normalized
  differential.** Control: a named test per operation runs native and
  interpreter, applies the normalizer, and asserts agreement; assert on the
  normalized observation, not "it runs".
- **`AC-2` — the differential discriminates (the load-bearing AC).** Control:
  D3's wrong-observation mutation reddens the exact named differential and names
  the operation. Run the mutation and show the red, then remove it and show
  green. A differential with no failing mutation is not accepted.
- **`AC-3` — the normalizer is necessary, not decorative.** Control: D4 shows an
  exact-equality differential for `ClockWallNow` failing on the correct
  implementation, so the normalization is load-bearing rather than a convenience.
- **`AC-4` — the wire contract is unchanged; only the two ops' availability
  status moves (AMENDED 2026-09-06 on runtime-leader evt_390ght47rw2t9).** The
  first cut banned every `effect_abi_v1.catalog` edit, which was wrong: the
  catalog carries availability status, and the ABI-R3 closure
  (`abi_v1.rs:1785-1796`) requires catalog-native-status iff runtime-NativeTested,
  so the promotion MUST flip exactly these two ops' status there. Control: `git
  diff` on `effect_abi_v1.catalog` shows a change to the native/availability status
  of `ConsoleRead` and `ClockWallNow` ONLY — every wire numeric id (`0x0101`,
  `0x0201`, all others), record layout, arity, and `operation_count` unchanged; no
  operation added or removed. The manifest hash changing as a consequence of the
  status change is correct, not a violation. Weakening or removing the
  catalog↔runtime availability closure to avoid the edit is unsound and banned.
- **`AC-5` — the derived closure stays green.** Control: the ABI-R3-style
  closure/partition tests in `effect_v1.rs` and `ken-verify/src/catalog.rs`
  stay green with `native_tested_count` now 15; no count assertion is
  hand-edited to match.
- **`AC-6` — direction stated.** This promotes two operations from unavailable to
  available; nothing previously available becomes unavailable. If anything
  regresses, stop.
- **`AC-7` — no-regression in CI (`COORDINATION §12`).** Targeted locally over
  the affected closure: name the exact `scripts/ken-cargo test -p <crate>`
  invocations across `ken-host`, `ken-verify`, `ken-cli` (the native
  differential tests), and any `ken-elaborator` export/erasure test that
  enumerates availability. Never `--workspace` — workspace-green means green in
  CI.

## 6. Banned scope and hard stops

- Do not add or remove an operation, and do not change any ABI numeric identity
  (`AC-4`). This WP changes only how two operations are classified and proven.
- Do not weaken the differential to exact-output equality for a nondeterministic
  observation, and do not make one side's assertion looser than the other's —
  the normalizer is symmetric and applied before comparison.
- No `spec/` or `conformance/` edit. No new crate dependency without routing.
- **Hard stop to the Steward** if: implementing a native leg needs a substrate
  this WP should not build (name it, and the predecessor it implies — the
  console-write leg is the reachable template, so a leg that needs materially more
  is the finding); or the honest normalized invariant for either operation cannot
  be stated without weakening it to vacuity (a design finding); or making the
  differential discriminate forces touching a wire identity.

## 7. Contention

`crates/ken-host/src/effect_v1.rs` is the trust-boundary file. Re-derive its
blob at pickup and check for a live foundation/runtime edit to it before
starting (ABI-R3-class inventory work and PX8-family error-id work have both
pinned it in the past). The lieutenant publishes serially; rebase the candidate
if `effect_v1.rs` moved under you.
