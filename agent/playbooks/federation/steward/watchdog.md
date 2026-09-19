# Steward watchdog

The watchdog is a liveness backstop. It detects stalled product flow; it does
not create work, edit workflow, reconcile history, or improve process.

## Arm it

At session start and after compaction, load the private interval prompt from
`../steward-watchdog-tick-prompt.txt` with `set_interval`. Re-arming replaces the
old interval. Never use a public scheduled call or a hand-written polling loop.

Disarm when no authorized lane, routed candidate, or active ring needs watching.

## Tick order

Stop after the first item that requires action.

## Availability checks

1. **Resource emergency.** Check available RAM and the repo filesystem. If
   either is near the stated floor in the prompt, reclaim only safe scratch and
   report the affected owner.
2. **Held finished work.** A routed or approved candidate waiting on the
   Steward or lieutenant is the first product action.

## Product-movement checks

3. **Active lanes.** Read the compact table in `steward/lanes.md`. For each
   lane, confirm the named active WP has one seat working or a named blocker
   with an owner and event.
4. **Dropped delivery.** If a kicked seat is idle, repair transport on the
   existing thread anchor and confirm `Working`.
5. **WIP audit.** If an implementer has worked 60 minutes since its last
   kickoff, hard stop, ruling, audit, or handoff, request the Architect audit
   defined in `escalation.md`.

## Runway check

6. **Runway.** If an active WP is near landing and its one immediate successor
   is not ready, frame that product successor. Do not create alternatives,
   inventories, or cleanup nodes.

If all six are clear, post nothing and stop.

## Pane reads

Capture enough history to distinguish a spinner or completed turn from an idle
composer. If the capture does not include the evidence needed for a verdict,
report unknown; do not infer idle from a narrow or empty tail.

Rouse the ring leader, not individual workers, except when the leader is also
down and the active product lane would otherwise stop. A repeated need to rouse
workers is a leader-watchdog defect to report, not a duty for the Steward to
absorb.

## What the watchdog must not do

A tick never:

- edits `agent/**`, scripts, workflow files, frames, or tracker prose;
- updates `lanes.md` merely because time passed;
- harvests lessons or opens process WPs;
- reads closed-WP history to improve its narrative;
- asks the Adversary for a review;
- posts an all-clear message;
- treats total commit count or management commits as product progress.

Only an observed stall earns a message. Name the blocked event and the seat that
owes the next action.