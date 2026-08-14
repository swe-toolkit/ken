---
id: PROG-TRACKER-MERGE-DRIVER
title: "Two docs candidates in flight ALWAYS conflict on generated IMPLEMENTATION-PROGRESS.md and nowhere else -- and the recorded reason merge=union was rejected is FALSE at the current generator, so D0 re-derives the warrant before anything is built"
status: ready
owner: steward
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Registered by the Architect at evt_2kf7xke2q2nvc on 2026-08-14 as the structural fix for a collision the Steward had been treating as an incident (it had responded by holding its own publishes, which serializes the coordination lane behind whatever docs candidate is in flight). Filed by the Steward rather than carried, because a registered fix with no node is indistinguishable from a fix nobody intends to make. Measured at origin/main 0936ef28."
---

## What this is

**A generated file that is a mandatory path on every frontmatter-touching
candidate is a standing conflict source, and it compounds under concurrent doc
activity.** `docs/program/IMPLEMENTATION-PROGRESS.md` is generated from
`docs/program/issues/` by `scripts/gen-progress.sh`, and
`.github/workflows/ci.yml` runs `gen-progress.sh --check`, so a candidate
**cannot drop it** to avoid the collision.

⇒ Any two candidates that add or flip an issue node conflict on **exactly that
one path**, every time, with no content disagreement between them.

**Measured 2026-08-14:** `LANG-GADT-SEQUENCE-TRACKER-GAP`'s candidate was recut
twice inside one hour for this and nothing else. The four deliverable blobs were
byte-identical across all three tips. Two ring hops bought zero content change.

## THE RECORDED REASON FOR REJECTING `merge=union` DOES NOT SURVIVE THE SCRIPT

**This is the reason `D0` exists and it is the first thing to settle.**

The fix was registered with the note that `merge=union` **"doubles the
timestamp/count and reds `--check`."** At `0936ef28` that warrant is false, and
the generator says so in its own comment:

- `gen-progress.sh` emits one data line under `## Last generated`:
  `<UTC timestamp> — from <N> issue file(s) in ...`.
- `--check` builds `TS_PATTERN` and runs `grep -vE "$TS_PATTERN"` over **both**
  the freshly generated output and the committed file before diffing, with the
  comment that the line *"embeds a live timestamp by design (it is
  informational, not load-bearing), so idempotency is judged on content with
  that one line normalized out of both sides."*
- **The pattern matches the count as well as the timestamp** -- `— from [0-9]+
  issue file` is inside it.

⇒ A union merge producing **two** such lines has both of them stripped from the
committed side, and the check never sees either. **The doubling is invisible to
`--check`.**

**The conclusion may still be right; the stated warrant is not.** The surviving
candidate reason is **row order and row duplication in the status table**, which
`--check` does compare: the generator emits one row per issue ordered by
`sort` over the issue filenames (`sorted_files`, and each file is `<ID>.md`, so
the order is a plain lexicographic sort on ID). A union of two conflicting
hunks concatenates them **in hunk order**, which is not the generator's order,
and can also emit an ID twice when both sides rewrote the same row.

**`D0` establishes which of these is true by running it.** Do not inherit either
the original claim or this node's reasoning about it.

> **Why this is written out at length rather than corrected quietly.** The
> conclusion "use a custom driver, not union" is very likely correct and was
> reached by someone with more context than this frame. **A frame that
> transcribes a false warrant hands the next reader a premise they will not
> re-derive**, and this repo has now been bitten by that shape more than once.
> Separate the conclusion from its warrant and re-measure the warrant.

## THE HAZARD THAT DECIDES THE DESIGN

**A driver almost certainly cannot discharge this by calling
`scripts/gen-progress.sh`.** The generator reads `docs/program/issues/` **from
the working tree**. A merge driver is invoked while the merge is being computed,
and under `merge-ort` the result is assembled in memory and written to the
working tree afterwards -- so at driver time the working tree holds the *pre-merge*
state, not the merged `issues/` directory. During a rebase that is the state of
`onto`, which is missing the very commit being replayed.

**If that holds, the driver must derive its answer from `%O`/`%A`/`%B` alone**
and never shell out to the generator. That is feasible because the table is a
deterministic, sorted projection: union the rows from `%A` and `%B`, resolve any
ID appearing in both, re-sort by ID, and emit a single header line.

**`D1` confirms or refutes the ordering hazard by measurement before choosing.**
It is stated here as reasoning, not as a fixed input -- it has not been run.

## Deliverables

**`D0` -- re-derive why `merge=union` is insufficient, at your base.** Construct
the two-candidate collision, apply `merge=union` to the path, and report what
`gen-progress.sh --check` actually says. **Report the timestamp/count line and
the table rows separately** -- they have different answers and the registered
reason conflated them. If union turns out to be sufficient, **that is the
finding and the node is discharged by a one-line `.gitattributes` entry.**

**`D1` -- confirm the working-tree hazard, then choose the mechanism.** Does a
merge driver invoked on this path see the merged `docs/program/issues/`, or the
pre-merge tree? Answer it by instrumenting a driver that records what it sees,
not by reading git's documentation. The answer selects between "driver
regenerates" and "driver derives from `%A`/`%B`".

**`D2` -- the driver, plus its `.gitattributes` entry.** Whatever `D1` selects.
It must be correct when **either** side is the one that added a row, and when
both sides rewrote the same ID's row -- that last case is a genuine content
conflict and **leaving a conflict for a human is the right answer there**, not a
silent pick.

**`D3` -- registration, and the fact that it is per-clone.** A driver named in
`.gitattributes` does nothing until `git config merge.<name>.driver` is set in
each clone. Name where that happens for this repo's worktrees and make it
happen there. **A driver that is registered in `.gitattributes` and configured
nowhere is worse than no driver**, because the path silently falls back to the
default and the failure is invisible.

**`D4` -- state what CI does.** CI checks out a merge result; it does not run
the driver. **Nothing in this node may make a CI check depend on the driver
being configured.** Say so explicitly and show that `gen-progress.sh --check`
is unchanged.

## Acceptance criteria

**`AC-1` -- a REPRODUCED collision, resolved by the mechanism, then
`gen-progress.sh --check` green.** Two branches off one base, each adding a
distinct issue node and regenerating the tracker. Merge one, rebase the other.
**Name the two branches and paste what `--check` printed.** A driver that was
never made to face the collision is not evidence.

**`AC-2` -- the `merge=union` question is answered with a run, not an
argument.** `D0`'s output, both halves reported separately. **If the registered
reason is confirmed rather than refuted, say what made it true** -- this frame
asserts it is false at `0936ef28` and that assertion is itself checkable.

**`AC-3` -- the both-sides-rewrote-one-ID case leaves a conflict.** Demonstrate
it. Automatic resolution of a real content disagreement is the failure mode this
whole mechanism could introduce, and it would be silent.

**`AC-4` -- the driver never invokes `scripts/gen-progress.sh`**, unless `D1`
measured that it safely can, in which case quote the measurement. This is the
criterion that stops the obvious implementation from shipping unverified.

**`AC-5` -- no change to `scripts/gen-progress.sh`'s output.** The generator is
the source of truth for the tracker's content and this node does not alter it.
Its `--check` normalization is a fixed input, not something to adjust to make a
driver pass.

**`AC-6` -- no-regression, in CI.** `COORDINATION §12` -- the venue is CI, never
a local `--workspace` run.

## Sizing

**`S`.** The driver is small and the registration is one line. **The
measurement is the work**: `D0` and `D1` are two experiments, and the one-hour
target applies to them. If `D1` shows no driver can be correct here, **hand back
that finding and stop** -- the node closes on the measurement, and the fallback
is the procedure already in use (rebase, let it conflict, re-run the generator,
never hand-resolve), which costs a ring hop but is not wrong.

## Contention

**`.gitattributes`, `scripts/`, and wherever `D3` lands the git config.** No
build ring currently holds any of those. **This node does not add or flip an
issue node**, so its own candidate does not regenerate the tracker and does not
collide with the thing it is fixing.

## Not this node

- **Not a change to `gen-progress.sh`.** See `AC-5`.
- **Not a change to what the tracker contains** or how it is ordered.
- **Not the Steward's publish cadence.** Ordering merges is the current
  mitigation and it stays whatever this node concludes; the mitigation is not a
  deliverable here.
- **Not a general `.gitattributes` pass** for other generated files. One path,
  one collision class. If another generated file has the same shape, that is a
  finding to report, not scope to absorb.
