---
scope: roles/steward
audience: (see scope README)
source: private memory `a-pane-status-describing-progress-says-nothing-about-liveness` (R4 triage, 2026-09-26)
---

# A pane status describing progress says nothing about liveness

A seat's closing status block reads like evidence of live work, but it is a
turn-ending summary — written at the moment a turn stops, not while anything
is happening. Liveness comes from one thing only: the footer's verb form. A
live footer reads as an active verb with an elapsed counter (`Working (Nm
Ns)`, `esc to interrupt`); a finished footer reads as a past-tense verb with
a total (`Worked for Nm Ns`). The prose above the footer is not evidence
either way, no matter how detailed.

Measured while sweeping a build ring: a seat's pane showed a finished footer
(`Worked for 24m 22s`) over a detailed status block — base verified,
mandatory rows passing, work described as "active." The status read as a
seat provably making progress, so the sweep declined to nudge it and
reported it as building. It was not. It had ended its turn well before that,
posted nothing to the channel since pickup, and was holding all of that
described progress as uncommitted worktree state — invisible to anyone who
had not opened its pane.

The failure was not a missing detector. The finished-footer signal and the
"a seat can answer in its own pane and never post" pattern were both already
known and cited in the same breath as the wrong conclusion — the status
text's *content* overrode both, because prose describing progress reads like
a seat making progress. The richer the status, the more it looks alive, and
a seat that stops mid-task tends to write a *longer* summary than one that
finishes cleanly, which is exactly backwards from what a reader wants.

## How to apply

- **Read the footer's verb form for liveness, nothing else.** A finished
  verb plus a total elapsed time is a stopped seat regardless of how
  detailed or current-sounding the prose above it reads.
- **Read the status block only for what to rescue, never for whether to
  act.** It can be an accurate list of what was done; it is never evidence
  the seat is still doing it.
- A finished footer with no channel post since pickup is a parked seat, full
  stop. The first move on a parked seat is usually to preserve or commit
  whatever it was holding, not to resume it blind — a seat that stops
  mid-task typically has nothing committed, and reviving it before rescuing
  that state can destroy the work the long status block was describing.
- Route the wake through the seat's leader rather than reaching straight
  into an implementer's pane, and expect that a seat can have answered its
  leader already, in its own pane, without that answer ever reaching the
  shared channel.
