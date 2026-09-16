---
title: "Clearing a merge asks two questions, and the second one does not occur to you"
scope: fleet
recorded: 2026-09-16
recorded_by: steward
---

# Clearing a merge asks two questions. The second one does not occur to you.

**When you route or publish a candidate you naturally ask: what CI does this
change need?** For a doc-only node the answer is *none*, correctly.

**You do not naturally ask: what does merging this do to CI that is already
running?** That is a different question with a different answer, and nothing in
the routing checklist prompts it.

## The measured instance

2026-09-16. The Steward routed `PUB-DOC-ONLY-UNVALIDATED-AGAINST-ITS-DIFF` with
the words *"no urgency and no CI dependency."* Both halves were true of the
candidate. It merged as `c4a21bb0fa658477f04e670764485634485dd98f` at 04:35:31.

    04:22:22   run 35055338045  head 10321a158  mode=full
                                sweep + 8 shards + native-slow start
    04:35:34   the doc-only merge's run starts
               -> run 35055338045 CANCELLED, 13 minutes in

`ci.yml`'s concurrency group resolves to `ci-CI-refs/heads/main` for **every**
push to `main`, with `cancel-in-progress: true`. **The key does not depend on
what either change contains**, so a doc-only merge kills an in-flight `crates/`
run exactly as hard as another `crates/` merge would.

The cancelled run carried the `ignored-row sweep` — the only job that produces
`RT-IGNORED-PASSING-ROWS-DISPOSITION`'s `D0` measurement, which three seats were
waiting on. **The replacement run classified `doc-only` and skipped it**, so the
measurement was not delayed to the next run; there was no next run that produced
it.

## Why a guard on the flag would not have caught it

The merge was **legitimately** doc-only and correctly flagged. This is not the
`--doc-only`-disagrees-with-its-diff defect; the flag was right. **A candidate
can be perfectly cleared on every property of itself and still be the wrong
thing to merge at that minute.**

## What to do

**Before publishing to a shared ref, look at what is running on it.**

    gh run list --branch main --limit 3 --json databaseId,headSha,status,conclusion

If something is `in_progress` on that ref and carries jobs your fleet is waiting
on, **hold the merge or say plainly that you are cancelling it.** One command,
and it is the only thing that distinguishes "cleared to merge" from "safe to
merge now."

**A merge freeze is the tool for this.** It is cheap, it is reversible, and the
cost of holding a doc-only node for ten minutes is nothing against re-running a
40-minute matrix.

## Recovery, if it has already happened

**Re-run the cancelled run by id** — `gh run rerun <id>`, not `--failed`.

- A re-run by id **replays the original event payload**, so
  `github.event.before` is preserved and the path classification survives a
  later push to the same ref. Verified 2026-09-16: attempt 2's `classify` log
  rendered `base='96762c05bc6f3cb0bf184d57d4aa57c5d10fbfc4'`, the original
  before-SHA, not the new tip.
- **Use "re-run all jobs", not "re-run failed jobs."** `--failed` skips jobs
  that succeeded, including `classify-paths`, so there is no fresh
  classification to read.
- **Freeze merges first.** A re-run enters the same group and dies the same way.
- **Check the carry at USE time, not at run-start time.** A result measured at
  commit A is a claim about current `main` only while the inputs the job reads
  are identical — compare the tree objects, not the commit ids.

See [[a-squash-merge-lands-the-diff-from-the-merge-base-not-the-diff-from-main]]
and [[a-claim-accurate-about-something-narrower-than-its-reader-infers]] — "no CI
dependency" was accurate about the candidate and read as a claim about the
merge. The structural fix is filed as `CI-MAIN-RUNS-CANCEL-EACH-OTHER`.
