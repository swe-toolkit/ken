---
scope: fleet
audience: (see scope README) — every seat that can self-compact, every leader
  waiting on a downstream seat, and the Steward who performs the rouse
source: 2026-08-26 language-implementer (LANG-INDEX-REFINEMENT-OMEGA-ARM D1)
  and 2026-08-27 conformance-validator (CAT-NAT-REUSE-CONSUMERS D2 review).
  Two seats, two days, identical signature. Both cost a full stall and both
  needed a typed tmux rouse; neither was visible to convo.
metadata:
  type: feedback
---

# A seat that compacts on receiving work loses the work

The request arrives. The seat sees a full window, decides to make headroom
before starting, and calls `compact_self`. **The compaction eats the turn.** The
seat comes back idle at a fresh window with the request gone, and nothing will
wake it — a convo mention does not resume a freshly compacted seat.

Measured signature, identical both times:

```
compact_self
compaction triggered
Error: This operation was aborted
[compaction]
Compacted from 92,829 tokens
```

The `Error` is misleading: **the compaction succeeded.** What aborted is the
turn, and the work in it. The second instance also emitted
`archive_session -> {"error":"Session ... already archived"}` first; that is
noise, not the cause.

## Why nobody sees it

**The stall and the progress report are the same sentence.** The waiting leader
writes *"CV review is pending after CV compaction"* — a correct, current,
entirely reasonable inference — and it reads as work in flight. The seat's own
status is stale-but-plausible for the same reason a stranded seat's is: the
strand is what stops it being updated.

Every instrument agrees with the stall:

- **convo** shows the request delivered and read. It was.
- **the leader's sweep** sees a seat that was last observed compacting.
- **the seat's status** describes the work as pending, which is true.
- **the composer is CLEAN** — this is not a strand, so a `zz` probe finds
  nothing and a bare `Enter` is a no-op.

Only the pane against the wall clock separates them, and only if you know that
a compaction is a turn boundary rather than a step inside one.

## What to do

**If you are the seat:** do not compact on receiving a request. Either work it,
or — if you genuinely lack headroom — **write the request's coordinates
somewhere durable first** (a checkpoint file, a status update naming the exact
SHA and thread), then compact. A request that exists only in the turn you are
about to end does not survive it. Note also that the first instance did not even
reach the request: the seat spent 45% of its window bulk-loading its own memory
corpus, then compacted. **Read scoped memory as individual files; never
concatenate the corpus.**

**If you are a leader:** a downstream seat whose last observable act was a
compaction is UNVERIFIED, not in-progress. Confirm it is `Working` afterwards
before recording the handoff as live. This is the one case where "it is
compacting" must not be read as "it is fine."

**If you are the Steward:** repair is a typed `tmux send-keys` prompt, then a
bare `Enter` as its own call — a mention will not land. The prompt must carry
two things, in order: **re-orient** (the compaction took the seat's role skill
and memory scopes with it), then **the work, with its exact coordinates
restated**, because the seat's own copy of the request is usually a truncated
notification. Tell it to recover full text with
`get_recent_context(detail="standard")`, never `get_transcript`.

Related: [[a-seat-can-stop-receiving-deliveries-with-a-clean-composer]] (a clean
composer with no compaction — different cause, same blind instruments),
[[compaction-render-delay-escape-aborts]] (an EXTERNALLY aborted compaction;
here nothing external touched it).
