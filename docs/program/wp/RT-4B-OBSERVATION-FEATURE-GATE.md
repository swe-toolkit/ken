# RT-4B-OBSERVATION-FEATURE-GATE — re-gate the D2f observation behind an off-by-default feature, and prove it inert across two compilations

**Owner: runtime. Size: M. Gate: none — inside 4b's envelope as the Architect
wrote it (`evt_4a1pf1jfmdemd`). No exception needed.**

**Base: re-derive `origin/main` at cut time.** Fixed inputs measured at
`9f22d70c`.

## Fixed inputs

| fact | site |
|---|---|
| the record, storage, note/take and the sole write — all `#[cfg(test)]` | `crates/ken-runtime/src/cranelift_backend/lowering/core.rs:543-589`, filled at `:2182-2194` |
| the runtime toggle that proves the RECORDING inert | `D2F_GATE_OBSERVATION_ENABLED`, same file `:575-576`, set at `:586-589` |
| the existing single-build identity test | `lowering/core/tests/control.rs:3661` — `r3_4b_input_observation_is_artifact_identical_when_disabled` |
| the real witness and its crate | `crates/ken-elaborator/tests/r3_c2_source_mixed_branch.rs:84`, via `compile_native_program_sources` |
| the dependency direction | `ken-elaborator` depends on `ken-runtime`; **not** the reverse |
| the rejected feature and why | `px8-ds-test-support` carries unrelated mutation and census support |

## D1 — re-gate the existing observation, do not build a second

Extend the **existing** record, storage, note/take and sole write so they are
compiled under an off-by-default Runtime feature **as well as** under
`#[cfg(test)]`, and expose a feature-scoped accessor `ken-elaborator` can read.

**Extend, never duplicate.** A parallel record with its own storage is the
second observer the envelope forbids, and it is what the rejected
`px8-ds-test-support` route amounts to.

## D2 — prove the feature inert across TWO COMPILATIONS

Compile the same target twice — once with the feature enabled, once without —
and compare the produced artifacts byte-for-byte.

**This is not the landed proof under a new name.** The landed test flips a
thread-local inside one build. That establishes the recording is inert. **A
feature is a compile-time property**, so only two compilations can establish the
feature is.

> **Cargo unifies features across a build graph.** A `ken-elaborator`
> dev-dependency enabling `ken-runtime/<feature>` compiles `ken-runtime` with it
> for **everything in that build** — so a feature-on and a feature-off artifact
> cannot both exist in one compilation. Any construction that looks like one
> test toggling a switch has measured the old claim under the new name.

> **The two compilations must not share a Cargo target directory.** Sharing one
> makes the second build overwrite the first in place, and the comparison then
> reads one artifact against itself. Give each its own `CARGO_TARGET_DIR`.

**Local runs are targeted only — `scripts/ken-cargo`, `-p <crate>` or `--test
<name>`. Never `--workspace`** (operator hard rule; `agent/COORDINATION.md §12`).
Workspace-green means green in CI.

## Acceptance criteria

The first five are the Architect's stated review conditions, written as ACs
because **an envelope part that stays prose is one the build cannot fail.**

- **AC-1 — the feature gates ONLY the recording and the accessor.** Never plan
  construction, never its inputs, never a branch that can change what the plan
  contains. **If gating it requires touching a construction path, stop.**
- **AC-2 — the accessor is `#[doc(hidden)]` and feature-scoped, and this frame
  says it is not a supported API.** Under the feature it is genuinely public to
  `ken-elaborator`; the only thing keeping it from becoming a surface people
  build on is that it is documented as not one. **Say so at the accessor.**
- **AC-3 — default-off in EVERY manifest**, stated explicitly. **A feature
  anything enables transitively by default is a production API with extra
  steps** — check the dependent manifests, not only the defining one.
- **AC-4 — identity is proven by comparing artifacts from two compilations, one
  with the feature enabled and one without. A runtime toggle within a single
  build does not discharge it.** Name both invocations and both target
  directories in the report.
- **AC-5 — the existing observation is EXTENDED, not duplicated.** One record,
  one storage, one note/take, one write site. A reviewer must be able to see
  there is still exactly one of each.
- **AC-6 — a mutation reds the identity comparison specifically.** Making the
  enabled build differ must fail on the byte comparison, not on a proxy. **A
  comparison that cannot fail is the same artifact as one that passes.**
- **AC-7 — the report states what this does NOT establish.** It makes a C2
  measurement possible. It says nothing about whether the planner fuses for C2,
  nothing about any enumerator exit, and it does not re-point the reach node.

## Pre-stated licensing — read BEFORE reporting

| outcome | what it licenses |
|---|---|
| **feature lands, two-compilation identity holds** | `RT-4B-UNIQUENESS-GATE-REACH` becomes lawful and re-points at C2. **A measurement becomes possible — no result from it is thereby licensed.** |
| **identity FAILS across the two compilations** | **That is a finding, not a defeat.** It means the feature is not inert and the mechanism is wrong. Return it; do not repair the comparison until it passes. |
| **gating cannot be confined to recording and accessor** | AC-1's hard stop. The mechanism does not fit the envelope and that is the Architect's to re-rule. |

> **This node cannot conclude anything about the planner.** The unperturbed D2j
> rows already show it fusing; that was never the open question, and a report
> that reads as settling it has over-claimed.

## Banned scope

- A second observer, a parallel record, or a second storage — see AC-5.
- An always-compiled accessor. That is the production API the envelope forbids.
- `px8-ds-test-support`, or neutralizing or splitting it. Ruled out.
- Counting, attributing, or measuring the planner's behaviour on any witness.
- Re-pointing `RT-4B-UNIQUENESS-GATE-REACH`. This unblocks it; it is not it.
- Enumeration, classifier, checker, marker, fusion-candidate, representation,
  ledger or closure-boundary repair. Gates 5 and 6 held; production unarmed.

## Hard stops — return to the Steward

- **Gating cannot be confined to the recording and the accessor.**
- **The two enabled/disabled artifacts differ.**
- **The mutation cannot red the identity comparison specifically.**
- **Any manifest would enable the feature by default or transitively.**

## Sequencing and contention

Runtime, one lane, after `RT-4B-WALKED-CONSTANCY` hands back — that node is
comment-only in `lowering/core.rs` and `control.rs`, the same files this one
edits, so they must not run beside each other.

Touches `crates/ken-runtime/src/cranelift_backend/lowering/core.rs`,
`crates/ken-runtime/Cargo.toml`, `crates/ken-elaborator/Cargo.toml` and
`crates/ken-elaborator/tests/`.
