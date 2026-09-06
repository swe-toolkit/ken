# ABI-A1 — promote ConsoleRead and ClockWallNow to NativeTested with a normalized differential

**Owner:** Team Runtime (`runtime-leader` + `runtime-implementer` +
`runtime-qa`). **Branch:** `wp/ABI-A1`. **Size:** M. **Tier:** T1.
**Risk:** medium — small availability diff, but it edits the trust-boundary
file every ABI consumer reads and the correctness content is the differential
design, not the flip.

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

- **`D0` — re-measure at your cut.** Re-derive the `effect_v1.rs` blob and every
  line above; confirm the native execution legs for both operations. Report the
  exact differential harness you will extend (crate + test file) and, per
  operation, the injected-input mechanism. Any drift is D0's to correct before
  authoring.
- **`D1` — the normalized differentials.** One per operation, per §3: an
  explicit normalizer applied to both native and interpreter observation, then
  equality on the normalized value. State the invariant each encodes in prose
  alongside the assertion.
- **`D2` — the promotion.** Flip `availability()` at `:140` and `:144`
  `RepresentedUnavailable → NativeTested`; add both to
  `NATIVE_TESTED_TARGETS_V1`. `native_tested_count` follows automatically —
  do not touch it.
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
- **`AC-4` — no ABI numeric identity changes.** Control: `git diff` shows no edit
  to `effect_abi_v1.catalog`; wire identities `0x0101` (ConsoleRead) and `0x0201`
  (ClockWallNow) and `operation_count` are unchanged. This WP changes
  availability, never the wire contract.
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
- **Hard stop to the Steward** if: a native execution leg for either operation
  is genuinely absent (the promotion then needs a named predecessor, not a
  fabricated harness); or the honest normalized invariant for either operation
  cannot be stated without weakening it to vacuity (that is a design finding);
  or making the differential discriminate forces touching a wire identity.

## 7. Contention

`crates/ken-host/src/effect_v1.rs` is the trust-boundary file. Re-derive its
blob at pickup and check for a live foundation/runtime edit to it before
starting (ABI-R3-class inventory work and PX8-family error-id work have both
pinned it in the past). The lieutenant publishes serially; rebase the candidate
if `effect_v1.rs` moved under you.
