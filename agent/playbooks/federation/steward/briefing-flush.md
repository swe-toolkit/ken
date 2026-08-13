---
name: briefing-flush
description: >-
  Keep docs/program/diary/CURRENT-BRIEFING.md to the last 24 hours by flushing
  older blocks into the dated diary, daily, on a delegated T2 subagent.
metadata:
  scope: federation/steward
---

# Briefing flush

`docs/program/diary/CURRENT-BRIEFING.md` is the Steward's durable resume
pointer and the operator's live read. It holds **the last 24 hours and nothing
else.**

Everything older belongs in the dated diary, `docs/program/diary/YYYY/Mon/DD.md`
(`diary/INDEX.md` describes that store). **Budget: under 250 lines.** Over
that, flush before writing anything new.

## The procedure

Run it **once a day**, and **delegate it** — Agent tool, `model: sonnet`.

1. **Commit first.** The worktree must be clean before you dispatch. A subagent
   can revert uncommitted edits, and your verification then passes vacuously
   against the reverted file.
2. **Dispatch**, instructing the subagent to:
   - move every block older than 24 hours into the dated file for **the day the
     block describes**, creating `YYYY/Mon/DD.md` as needed;
   - **move, never summarize and never drop** — the diary is the archive;
   - append under a `## Steward briefing` heading, preserving existing content
     in a day file that already exists;
   - leave **standing, undated content in place**: operator rulings, preserved
     refs, standing traps, and the where-durable-law-lives pointers.
3. **Verify it moved rather than deleted, before committing.** Old total should
   equal the new briefing plus everything added to dated files. A flush that
   loses content is worse than a long file.
4. Commit and publish doc-only.

## Why delegated

The judgement is "which day does this block belong to" — mechanical, and
deterministically checkable by line count. That is exactly the delegation
criterion in `MODELS.md`. Doing it yourself spends T1 tokens on filing.

## The failure mode, because it is silent

Nothing reds when the briefing grows. It just costs every future resume more
context, and the operator reads it. It reached **4648 lines / 273 KB across 19
unflushed days** before anyone noticed — having already been "rewritten to be
small" once, which is the tell: **a file whose only enforcement is your own
restraint gets flushed on a schedule or not at all.**

The second tell is in the writing. When a block opens by explaining how to
distrust the file, or carries its own "this section is superseded" markers, the
file is already doing the archive's job. Flush it; do not annotate it.

## Not this procedure

- **Rewriting or re-organizing the briefing's live block.** That is
  `steward.md` §1's deprioritized "briefing rewrites" and it waits for an idle
  moment. The flush does not — it is a delegated dispatch costing one tool
  call.
- **Editing the dated diary's older content.** Append only.
- **Deciding what is worth keeping.** Everything is kept; only its location
  changes.
