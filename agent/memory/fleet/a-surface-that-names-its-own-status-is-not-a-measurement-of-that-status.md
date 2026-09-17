---
name: a-surface-that-names-its-own-status-is-not-a-measurement-of-that-status
description: "A check, job, flag or region whose NAME asserts a status does not
  thereby compute it. Four instruments measured in one evening each reported a
  verdict they were not computing: an aggregate gate that fails identically
  whether a test failed or never ran, a job named 'findings non-blocking' being
  treated as blocking, --doc-only reporting green on a matrix it skipped, and a
  cfg-gated region reviewed as design that no build compiles. Read what the
  surface COMPUTES, not what it is called."
metadata:
  node_type: memory
  type: feedback
  scope: fleet
---

# A surface that names its own status is not a measurement of that status

Measured 2026-09-16/17, four instances inside one evening, each found by a
different seat, each having already misled someone.

| the surface | what its name asserts | what it actually computes |
|---|---|---|
| `build + test` | the build and tests passed | its only step is *"All test jobs passed"* — **fails identically whether a shard FAILED or was CANCELLED** |
| `verify-realized-shard-union` | the shard partition is verified | same shape: it fails because its inputs are missing, not because they disagreed |
| `ignored-row sweep (findings non-blocking)` | this job does not block | nothing enforces that; it was **treated as blocking** |
| `--doc-only` publish, reporting green | CI passed on this commit | the heavy matrix was **skipped**; green means *not run* |
| `#[cfg(feature = "...")]` region | reviewed code | **no permitted local build compiles it** — it has the review status of a doc comment |

## The one that cost the most

A runtime candidate was Architect-approved and QA-approved at `1035/0/2`, and
**called four functions defined nowhere in the tree**. The call sites sat under
`#[cfg(feature = "px8-ds-test-support")]`; that package's `default = []`, so the
mandated `-p ken-runtime` compiled them out and was genuinely green. CI
activates the union every workspace member demands — including a sibling's
`[dev-dependencies]` — and reds.

**Nobody misused an instrument.** `COORDINATION §12` mandates targeted local
builds, that is operator law, and the defect is that the mandated instrument
*cannot reach* a feature-gated region. **235 such regions in that candidate's
nine files, zero compiled by any permitted local build.**

## Why the naming is what does the damage

A surface with a neutral name gets checked. A surface whose name contains the
conclusion gets **read instead of checked** — the name supplies the answer the
reader came for, so the reader stops. `build + test` is trusted precisely
because its name says the two things you wanted to know.

> **The failure is not that these surfaces are wrong. Each is correct about
> something.** It is that **the name and the computation answer different
> questions**, and only the name is visible at the point of use.

⇒ This is the same family as
[[an-instrument-that-reports-a-verdict-cannot-distinguish-inapplicable-from-false]]
and [[a-reassurance-message-that-cannot-fail-is-worse-than-no-check]]. The new
part is the **tell**: the name contains a status word.

## How to apply

1. **When a surface's name contains a status word — passed, verified,
   non-blocking, only, green, clean — go read what it computes.** That is the
   trigger. It costs one click to the job's steps or one `grep` to the config.
2. **Never read a conclusion off an aggregate.** Report the leaves by name.
   `test shard 1..8/8` individually, not `build + test`. An aggregate cannot
   distinguish *failed* from *never ran*, and those have opposite meanings:
   one is a defect to fix, the other is a **measurement you do not have**.
3. **A green you did not watch execute is `unmeasured`, not `passing`.**
   Especially a `--doc-only` green, and especially a targeted `-p <crate>` green
   on a candidate touching a feature-gated region.
4. **For a `cfg`-gated region, ask whether it RESOLVES, not whether it reads
   well.** Reviewing gated code as design without asking whether it compiles is
   reviewing a doc comment. The cheap closure is a check-only build with the
   feature union the workspace actually demands:

       scripts/ken-cargo check -p <crate> --features <union>

   **Derive the union from the `[dev-dependencies]` that demand it. Do not
   memorize anyone's feature list** — it drifts, and a memorized list is the
   defect in [[deliverability-is-part-of-mechanism-selection]] wearing different
   clothes. Carry the query, not the answer.
5. **When you report a red, say which of the two it is.** "CI is red" is not a
   finding; *"two aggregate gates failed because every shard was cancelled"* is,
   and it licenses a completely different next action.
