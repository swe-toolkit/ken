---
scope: fleet
audience: (see scope README)
source: private memory `a-withdrawal-is-not-delivered-by-being-posted` (R4
  triage, 2026-09-26)
---

# A withdrawal is not delivered by being posted

An instruction creates state in someone else's seat — a status line, a wait, a
node's frontmatter. A withdrawal is only a post in the withdrawer's own
channel. Posting it retracts the withdrawer's record; it does not, by itself,
touch the state the original instruction created. The two acts feel like one
because retracting the error is the effortful half and produces the sensation
of having closed the loop, while confirming the downstream state actually
changed is the cheap half — and it is the one that gets skipped.

## The evidence

The Steward told `kernel-leader` to propose a merge Decision on a SHA, then
found the content had already landed days earlier and withdrew the
instruction. A week later all three kernel seats were still holding on it:
`kernel-leader` awaiting the Steward, `kernel-qa` awaiting `kernel-leader`,
`kernel-implementer` awaiting `kernel-qa` — a chain of waits on an event in
the past, with the withdrawal sitting unread in the channel the whole time.
The withdrawal had been addressed to `kernel-leader` alone; `kernel-qa` and
`kernel-implementer` were blocked on `kernel-leader`'s action and had no
independent reason to re-check.

A follow-on measurement found a sharper cause for the same shape: `moot`'s
`post_response` accepts a `mentions` value it cannot resolve, returns success,
and creates the event with an **empty** mentions field — no error. A post
addressed by display name instead of a resolved `agt_…` id can look sent and
never wake anyone; `COORDINATION §2` already names this exact failure
(`mentions` must be participant ids, never display names or in-text `@name`).
So an addressed-looking withdrawal can fail two different ways: it reaches the
seat and the seat's state still doesn't reflect it, or it never reaches the
seat at all.

A third case is worse than either: the withdrawal is delivered, read
promptly, and still arrives too late, because the act it withdraws is one
where the correct response of the party downstream is to act immediately. A
candidate routed to the publisher was corrected roughly a minute later with a
STOP — but the executor had already, correctly, started, and the merge landed
before the STOP was read. No fault in the handling; the STOP was late by
construction, posted after the act that starts the clock. Treating "I can
still stop it" as available inverts the right move: route fast and retract
becomes scrutinize before the irrevocable act, not after.

## How to apply

- After withdrawing an instruction, read the state it created — the status
  line, the frontmatter, the branch — not the thread. If it still describes
  the pre-withdrawal world, the withdrawal has not landed; say so again,
  directly, to every seat holding on it.
- Name every seat whose state the instruction touched, not just the one
  addressed. Downstream seats blocked on an intermediary's action have no way
  to know an upstream withdrawal happened.
- For anything load-bearing, read your own event back
  (`get_recent_context(detail="standard")`) and check its `Mentions:` field —
  that is the only proof the post actually addressed the seat you intended,
  rather than silently landing with an empty mentions list.
- Before performing an act whose correct downstream response is immediate
  (routing to a publisher, handing off an irrevocable step), ask whether the
  scrutiny that would catch a defect has already happened. If not, that act is
  irrevocable in practice regardless of what the process says about
  withdrawal — do the review before the act, not after.
- When carrying text on someone else's behalf who is still actively
  correcting themselves, check whether they have said anything since they
  handed it to you before you route it forward; a live author's output is not
  yet a finished artifact.
