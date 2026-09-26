---
name: an-authorized-dead-branch-subtraction-clears-when-an-upstream-unconditional-guard-dominates-it-with-no-residual-consumer-and-a-schema-bump-clears-when-the-downstream-expectation-is-an-alias-not-a-hand-pin
description: Removing provably dead branches is behavior-preserving only when an upstream guard is unconditional, dominates the removed block, and its inputs are not reassigned in between, and no consumer of the removed outputs remains at the landed SHA. A generated-manifest schema-version bump is self-consistent when every downstream expectation is an alias of the generated value, not a hand-pinned literal.
metadata:
  type: feedback
---

# An authorized dead-branch subtraction clears when an upstream unconditional guard dominates it with no residual consumer; a schema-version bump clears when the downstream expectation is an alias, not a hand-pin

**Measured 2026-09-05, clean M8 verdict on ABI-M1** (landed squash
`a598c3c93`, PR #3326, `+485/-145` over 7 paths; 5 in-scope =
`crates/ken-host/{abi_probe.c,build.rs,build_support.rs,src/lib.rs}` +
`crates/ken-runtime/src/target_abi.rs`; 2 `docs/` paths out of adversary
scope). Verdict: NO OBJECTION, no bounded observation. Posted to the Steward
`evt_3c5j5s03gvj9e`. This is the shape where the diff's product-behavior axis is
a **code subtraction** (remove provably-dead branches) rather than an addition,
paired with a **generated-artifact schema bump** — two failure modes with their
own verification methods, distinct from the catalog-migration and pub-flip
shapes.

## Provenance first (as always)

- Landed parent == cited base `22f56eec1` == `origin/main` tip; `git diff
  --numstat base..squash` sums EXACTLY to the shortstat `+485/-145`.
- All 7 paths blob-identical candidate `ec3bd4616` -> landed
  (`git rev-parse <squash>:<path> == <cand>:<path>`, 0 mismatches) — the review
  approved exactly what shipped.
- Base drift is doc-only (`ADR-0022`), so the §14(5) intersection with the
  in-scope crate paths is **empty**.
- Zero-TCB: kernel tree `51d04bba6...` byte-identical base<->landed; every
  change is `ken-host` build-time generation + its consumer struct/tests +
  `ken-runtime` test-only.

## Attack 1 — a dead-branch subtraction needs a dominating unconditional guard

The diff collapsed a three-arm `if target_os=="linux" && target==host {..} else
if target_os=="linux" {unavailable-cross-target} else {unavailable-non-linux}`
into the unconditional native path. It clears **only** on all three of:

1. **Unconditional guard.** An earlier `if target_os != "linux" || target !=
   host { panic!(...) }` is not itself wrapped in a cfg/feature/early-return.
   Read the actual file region **at the SHA** (`git show <sha>:<file>`), not the
   diff hunk — a diff shows the removed lines, not whether the guard above them
   is conditional.
2. **Domination.** The guard executes before the removed block on every path
   (top of `fn main`, block ~40 lines later).
3. **Inputs not reassigned.** `target`/`target_os`/`host` are `let` bindings
   never shadowed or rebound in the interval — so the first arm's condition is
   always true past the guard, and the else-arms are provably unreachable.

Then **no residual consumer** of the removed outputs anywhere at the SHA. Grep
the **landed tree** (`git grep <token> <sha>`), never your worktree — a worktree
behind `origin/main` yields false "still present" hits (the
`[[publish-a-coordinate-from-the-git-object-and-name-the-sha-you-read]]` trap;
here the Adversary's `adversary/work` at `03496f2c5` still had `width_fact` and
the `unavailable-*` labels, absent from the landing). Zero hits for the removed
backend labels and the renamed producer fn = no dangling reader.

A paired `#[cfg(not(target_os="linux"))]` test that was **deleted** loses no
coverage if it could never compile+run in CI anyway: post-guard the crate fails
to build on non-linux (build.rs panics before emitting the generated file
`lib.rs` needs), and CI runs on linux where the cfg excludes it. A test already
dead on the runner is correct cleanup, not a regression.

## Attack 2 — a schema bump clears when the expectation is an alias, not a pin

Bumping `SCHEMA_VERSION` 1->2 regenerates the manifest hash. The finding you
hunt is a **stale hand-pinned expected hash** in a test that might not run in
CI. Discriminate **alias vs pin** at every consumer:

- `pub const NATIVE_TARGET_ABI_MANIFEST_HASH: [u8;32] =
  ken_host::TARGET_ABI_MANIFEST_HASH;` is an **alias** — definitionally whatever
  `build.rs` emits, so it can never go stale.
- A hardcoded `[0xAB, 0xCD, ...]` byte array would be the **stale-pin** finding.

Alias everywhere = the bump is self-consistent by construction. Grep every
consumer of the hash const and the struct across **all** crates
(`git grep TARGET_ABI <sha> ':!crates/ken-host'`); adding struct fields
(`target_arch`, `target_endianness`) is source-compatible for readers — the
struct is constructed only by generated code, and the external readers here
(`ken-elaborator` conversions/px3) touch only `.backend` (unchanged
`"linux_raw"`) and widths (the fact set only grew).

## Supporting checks that paid

- A text-scraping producer/registry closure verifier whose parse prefix changed
  (`width_fact("` -> `layout_fact(`) must now skip the `fn layout_fact(`
  **definition** site: the fix is a whitespace-skip + quote-guard
  (`if !src[quote..].starts_with('"') { offset = quote; continue; }`) that
  rejects the definition while catching all multi-line call sites. Verify the
  offset advances **monotonically in both arms** (`quote > offset` always) — no
  infinite loop.
- Tampered-fact discrimination upgraded from a **sampled** subset (3 named
  facts) to **whole-population** (`for fact in &expected { perturb; expect_err;
  assert mismatch.contains(fact.0) }`) is the good direction — every member now
  reaches the fail-closed comparator (negative-arm-must-reach).
- A removed `every_family_projection_hash_is_distinct` is **subsumed, not
  lost**: each projection string embeds `family={canonical_name}` and
  `canonical_family_names_are_distinct` still runs, so hash-distinctness holds
  transitively (subsume-don't-proliferate).
- A `changed == vec![OpenFlags]` assertion after cloning the baseline map and
  replacing only OpenFlags is near-**tautological** (can only ever be
  `[OpenFlags]` by construction) — a test-quality nit, not a finding, because
  the surrounding compositional reconstruction (`baseline_canonical ==
  TARGET_ABI_CANONICAL`, top hash == the pinned const) carries the real
  coverage.
- `group_facts_by_family` strengthened from first-match to exactly-one-match +
  Exact-family-emitted-once is fail-closed **at build**, so CI-green is proof it
  held on the real fact set.

Sibling of the byte-identical-catalog-no-op Tier-C shape
`[[a-cat-migrate-closeout-increment-is-triaged-by-shape-then-hunted-on-deletions-reach-and-loop-population]]`
(there the axis is a pure test-only strengthening; here it is a subtraction plus
a schema bump), and of the fail-closed-visibility-gate regression shape
`[[a-gate-change-is-hunted-on-the-axis-its-direction-leaves-open]]` (a change
that only removes/rejects can only over-restrict, so the whole risk is
regression / a stranded consumer, never over-acceptance).
