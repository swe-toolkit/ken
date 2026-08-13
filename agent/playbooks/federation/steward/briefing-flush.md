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
pointer and the operator's live read. **It holds ONE block: the current one.**

Three files, and keeping them separate is what keeps the briefing small:

| content | file |
|---|---|
| the current window, one block | `CURRENT-BRIEFING.md` |
| permanent undated material — operator rulings, preserved refs, standing traps | `diary/STANDING.md` |
| what happened on day X | `diary/YYYY/Mon/DD.md`, see `diary/INDEX.md` |

**Budget: under 250 lines.** Over that, flush before writing anything new.

**"Last 24 hours" is the wrong test and it failed in practice.** It licenses
keeping superseded blocks because they are recent; one flush under that rule
left four stacked checkpoints and the file stayed at 1946 lines. **The test is
current, not recent** — a superseded block moves even if it is an hour old.

## The trigger — a 24h schedule, separate from the watchdog tick

Operator, 2026-08-13: the flush fires on **its own 24-hour interval**, not off
the fleet watchdog. Use `CronCreate`, and pick an off-`:00` minute.

⚠ **`CronCreate` is SESSION-ONLY and auto-expires after 7 days.** It dies with
your session, silently. ⇒ **Re-arm it at session start and after every
compaction, in the same breath as the watchdog** — the watchdog re-arm note at
the top of `CURRENT-BRIEFING.md` names both. A schedule that only exists in the
session that created it is not a schedule; the playbook is the durable half.

## The procedure

Run it **once a day**, and **delegate it** — Agent tool, `model: sonnet`.

1. **Commit first.** The worktree must be clean before you dispatch. A subagent
   can revert uncommitted edits, and your verification then passes vacuously
   against the reverted file.
2. **Dispatch**, instructing the subagent to:
   - move **every block that is not the current one** into the dated file for
     **the day the block describes**, creating `YYYY/Mon/DD.md` as needed;
   - **move, never summarize and never drop** — the diary is the archive;
   - append under a `## Steward briefing` heading, preserving existing content
     in a day file that already exists;
   - keep the file header, the one current block, and nothing else.
3. **Require a byte-level round-trip check**, not just line arithmetic: kept
   head plus moved bodies plus kept tail must reproduce the original file
   exactly. Line counts alone pass when content is reworded in place. A flush
   that loses content is worse than a long file.
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
