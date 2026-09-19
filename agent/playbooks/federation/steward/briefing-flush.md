---
name: briefing-flush
description: >-
  Maintain the optional operator briefing only when the operator requests it.
metadata:
  scope: federation/steward
---

# Briefing flush

## Standing rule

The briefing is not a standing Steward activity. Do not schedule a daily flush,
start a subagent for it, or publish a briefing-only commit unless the operator
has asked for the artifact.

## Brief content

When requested, `docs/program/diary/CURRENT-BRIEFING.md` contains one current
block and stays under 250 lines. Older blocks move unchanged to the dated diary.
Git history is not copied into the briefing.

## Requested flush

Start from a clean worktree.

## Move procedure

1. Keep the header and one current block.
2. Move older blocks unchanged to `diary/YYYY/Mon/DD.md` under a
   `## Steward briefing` heading.
3. Verify kept plus moved content reconstructs the original bytes.
4. Bundle the result with the next permitted program-doc publish.

## Publication rule

Do not summarize, rewrite, curate, or publish it separately. A briefing is an
operator view, not a second tracker or a durable execution authority.