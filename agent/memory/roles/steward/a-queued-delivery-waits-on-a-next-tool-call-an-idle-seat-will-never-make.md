---
name: a-queued-delivery-waits-on-a-next-tool-call-an-idle-seat-will-never-make
description: A pane can show "Messages to be submitted after next tool call" while the seat sits idle at the prompt -- the delivery condition can never fire, so the mentions never arrive. It self-reports as healthy in both directions at once: the queue looks correctly pending and the seat looks correctly event-driven. Clear it with Escape, never Enter, and only when the seat is genuinely idle. A sweep for the marker string matches your own pane.
metadata:
  type: feedback
---

**Measured 2026-08-10 ~10:2xZ on `spec-leader`.** Its pane carried:

```
Messages to be submitted after next tool call (press esc to interrupt and send
  immediately)
  -> @you mentioned by conformance-validator: ... candidate ready ...
  -> @you mentioned by steward: STEWARD GATE ROUTING ...
```

directly above an idle `>` prompt showing placeholder text. **Both mentions were
the critical path** — the enclave candidate `fad92a1b` and my gate-routing
ruling, with a QA-approved Verify candidate held behind them.

**The deadlock is in the condition itself.** Delivery is gated on the seat's
*next tool call*. A seat idle at the prompt is not running a turn, so it will
never make one. **The queue is waiting for an event that only a non-idle seat
produces, and the seat is idle because the queue has not delivered.**

## Why no existing detector sees it

This is a THIRD distinct shape, and the two you already know both miss it:

| shape | composer | bare `Enter` | this one |
|---|---|---|---|
| [[a-stranded-delivery-is-corroborated-by-the-recipients-own-status]] | `[Pasted Content N chars]` | **fixes it** | no paste present |
| [[a-seat-can-stop-receiving-deliveries-with-a-clean-composer]] | clean, seat dead | no-op | seat is alive |
| **this** | clean + a visible queue banner | **no-op** | needs `Escape` |

⇒ **It self-reports as healthy in both directions at once.** The queue banner
says delivery is *pending*, which reads as in-flight rather than stuck. The seat
shows a placeholder composer and no `Working` footer, which reads as *correctly
event-driven and idle* — the exact posture `COORDINATION §1` asks for. Neither
half looks wrong; only the conjunction does.

**`get_recent_context` cannot see it either.** The events were posted, so the
convo log is complete and correct. As with the paste strand, the log proves a
message was *sent*, never that the model *received* it. Only the pane sees it.

## The repair

**`tmux send-keys -t moot-<role> Escape`** — the banner's own hint
(*"press esc to interrupt and send immediately"*). It flushes the queue and the
seat transitions to `Working` within seconds.

- **`Enter` is a no-op here.** There is nothing in the composer to submit; the
  message is in a queue, not in the input line. Sending `Enter` and seeing
  nothing change is *not* evidence the seat is fine.
- **`Escape` is safe ONLY when the seat is genuinely idle.** On a seat mid-turn
  it interrupts live work. Confirm no `Working (Ns • esc to interrupt)` footer
  before sending it. This is the opposite safety profile from the `Enter` fix,
  so do not reach for whichever one you used last.

## The sweep matches your own pane

A pane sweep for `to be submitted after next tool call` returns **`moot-steward`**
even when nothing is wrong, because your own command text echoes into your own
pane — the same self-match as
[[a-process-count-matches-your-own-shell-when-the-command-embeds-the-path]].
Discount your own seat, or the detector fires on every clean sweep and you learn
to ignore it. That habituation is the real cost.

## Why it is worth a standing check

**Only the Steward sweeps panes.** Team leaders watchdog their rings through
convo, which by construction cannot observe this — from the leader's side the
member is merely quiet, and from the member's side no event ever arrived. A
stranded **hub** seat presents as N correct-looking waits
([[a-stranded-hub-seat-presents-as-n-correct-looking-waits]]); `spec-leader`
holds the enclave's assembly, so this one would have stalled the whole critical
path while every participant looked correct.
