# CI-OLD-PRESTATE-ROW-CURRENCY — the `old` flip pair contradicts the code

Owner: verify. Size: **S**. Node: [[CI-OLD-PRESTATE-ROW-CURRENCY]] (`ready`).

Origin: Adversary finding `evt_33wfx803mv0r7`, Steward-triaged as confirmed.
**Re-derive your merge-base from `origin/main`; do not take a SHA from this
frame.**

## What you are doing

`LANG-SPACE-PRESTATE-BIND` landed at `19006e37`. **Two rows in
`conformance/verify/spec-syntax/seed-spec-syntax.md` still assert that
pre-state elaboration is unavailable.** Correct them to the landed behavior.

**This is a one-file change and no `crates/` edit is in scope.** The elaborator
is right; the rows are wrong.

## Fixed inputs — measured, do not re-derive

At `origin/main = 5df41be0`. Both rows are under
*"B. `old`-capture scope guard (`21 §6.4`) — the flip pair."*

| row | its `expect (landed)` today | landed reality |
|---|---|---|
| `old-resolves-in-space-op-ensures` | *"rejected at elaboration as unsupported… no obligation using it is emitted"* | accepted; obligation emitted and `Refl`-discharged |
| `old-out-of-scope-rejects` **(soundness)** | *"reject/reject at distinct gates, **not a verdict flip**"* | pure code rejects `UnboundName("old")`; space-op **accepts** — a true flip |

The landed evidence is in `crates/ken-elaborator/tests/surf_space_cells_p1.rs`,
`ac_s7_old_in_space_uses_the_bound_pre_state`: three obligations on `inc`, each
discharged by kernel `Refl`, with the `+2` variant staying open under the same
certificate. `ac_s7_old_in_pure_code_remains_unbound` still pins the pure-code
`UnboundName(old)` side.

> ## ROW 2 IS THE ONE THAT MATTERS, AND ITS ERROR IS NOT A STALE VERDICT
>
> Row 2 is tagged `(soundness)` and its `expect (landed)` states **a relation
> between the two rows**, not just its own verdict. That relation has inverted.
>
> **The discrimination got stronger, not weaker.** An accept/reject flip
> separates the two scopes better than two rejections at different gates. **The
> defect is that the row disclaims the discrimination it now has.**
>
> ⇒ **Do not "fix" this by weakening the row.** The `why` clause currently
> argues the absence-assertion from the pre-state gate being unreachable. That
> argument is spent. **Replace it with the flip argument**: the same `old(…)`
> syntax resolves in a block-space `ensures` and is unbound in pure code, so the
> guard is the cell environment, not the spelling.

## Deliverable

Both rows' `expect (landed)` clauses corrected; row 2's relation and `why`
clauses restated as the verdict flip; and the two
`[deferred — old/pre-state elaboration; OQ-Space model decided]` tags **that
this capability discharged** retired.

## Acceptance criteria

**AC-1 — both rows state the landed behavior**, and row 1's former
`expect (model, deferred)` text is checked against the code rather than
promoted verbatim. It reads as correct; **confirm it, do not assume it.**

**AC-2 — the flip is stated as a flip.** Row 2 names the accepting side, the
rejecting side, and the guard that separates them (**cell environment, not
syntax**). **State the two verdicts explicitly.**

**AC-3 — the modifier form is not swept up.** `space proc` has no cells and
still refuses with `OldPreStateUnsupported`. **Five existing controls pin
that** — `v1_acceptance.rs:230`, `:245`, `let5_checking_mode_let.rs:85`,
`kenfmt_b3_layout.rs:122,126`. If any row implies `old` now works "in spaces"
without distinguishing block from modifier form, that is the same
under-discrimination `LANG-SPACE-PRESTATE-BIND`'s `AC-4` existed to prevent.
**Report the count.**

**AC-4 — retire only the tags this capability discharged.** Deferral tags
naming `OQ-Space` concurrency (`36 §4.4`), `pub`/nested placement, or anything
else stay. **Report how many tags you touched and how many bearing the same
text you deliberately left**, with the reason. A tag retired because it looked
similar is the failure here.

## Excluded scope

- **Any `crates/` change.** If a row cannot be made true without touching the
  elaborator, **stop and come back to me** — that would mean the capability is
  not what the merge claimed, which is a finding, not a fix.
- **A general `conformance/` currency sweep.** Only tags this capability
  discharged. The broader sweep is its own node.
- **`OQ-Space` / `36 §4.4`**, `pub` and nested space placement.

## Contention

One path: `conformance/verify/spec-syntax/seed-spec-syntax.md`. Language is on
`crates/ken-elaborator` under `LANG-SURFACE-IF`; Runtime is on
`crates/ken-runtime`. No intersection.

**This touches `conformance/`, so the merge Decision pulls a Spec vote.** Budget
for it in your handoff — it is a required gate, not a courtesy.

## Validation

The change is prose in a conformance seed. Run whatever seed-integrity or
schema check `conformance/` carries; **do not run a workspace build for a
markdown row.** Targeted only, never `--workspace`.

## Sizing

**S, and I expect well under an hour.** If it grows past one file, the growth is
the finding — hand it back rather than widening.
