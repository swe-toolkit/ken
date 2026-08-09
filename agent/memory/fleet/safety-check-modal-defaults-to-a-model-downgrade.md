# The "additional safety checks" modal needs NO answer — leave it alone

**Operator, 2026-07-29, verbatim:** *"Note that you don't need to select an
option on the 'additional safety checks' prompt. It will keep working
regardless."*

A Codex seat mid-turn can render:

```
Additional safety checks
This request requires additional safety checks, which can take extra time.
Hang tight or retry with a faster model for a quicker response, though it
may be less capable of handling complex requests.

› 1. Retry with a faster model
  2. Keep waiting
  3. Learn more
```

## THE ACTION IS: DO NOTHING

**This is not a stall, and it is not a delivery failure.** The request
proceeds while the prompt is on screen. ⇒ In a pane sweep, a seat showing this
modal is **working** — do not classify it as blocked, do not send it a keypress,
do not nudge it in the channel.

**Doing nothing is also the safest option**, because the only way to harm the
seat here is to interact with it (below).

## Why you must not reflex-`Enter` it

Option **1 is pre-selected**. The standing repair for a *stranded Codex
delivery* — a bare `Enter` — therefore **silently downgrades the seat's model**,
undoing an operator seating directive with no error, no log line, and no channel
event.

The two shapes look alike: a seat sitting still, apparently waiting on a
keypress. ⇒ **Before sending `Enter` to an apparently-stuck Codex seat, capture
the tail and check for a numbered option list.** If one is present, the seat is
not stranded — leave it. Any pane-sweep script that sends a bare `Enter` must
refuse when it sees numbered options.

## THE DETECTIVE HALF: once the modal clears, the FOOTER is the only trace

**Measured 2026-08-09 on `kernel-implementer`, mid-`D5`.** Everything above is a
**preventive** control — it stops you causing the downgrade. It says nothing
about finding one that already happened, and that gap is what cost this fleet a
turn.

| | value |
|---|---|
| configured, `moot.toml` | `gpt-5.6-sol`, `model_reasoning_effort = "medium"` |
| actually running, pane footer | `gpt-5.6-luna low` |

Sol is T1, luna is T3, and the effort dropped with it. **The modal was gone by
the time anyone looked.** It leaves no error, no log line, and no channel event
— so after it resolves, the running model line is the *entire* evidence that
anything happened.

⇒ **A pane read must capture the model footer, not just the state.** The modal
answers *is this seat alive*; the footer answers *at what tier*. Two different
questions, and only the first one had a rule.

**The trap that makes this specifically hard to catch:** proving the seat is
*working* feels like it settles the matter, and it does not. The Steward
measured a live PID with a queued build and correctly concluded "not wedged" —
a true statement about liveness that is silent about tier, from a check that
could never have surfaced the downgrade. **Liveness and tier are independent,
so a liveness instrument reports all-clear over this every time.**

**The cheap sweep, and the discriminator is fleet-relative:**

```sh
for s in $(tmux ls -F '#{session_name}' | grep '^moot-'); do
  printf '%-34s %s\n' "$s" \
    "$(tmux capture-pane -p -t "$s" | grep -oE '(gpt-5\.6-(sol|terra|luna)|Opus [0-9]+)( (low|medium|high))?' | tail -1)"
done
```

**One seat on luna while every peer is sol/terra is the signal** — a genuine
credit-window failover moves the whole fleet, so a lone outlier is a downgrade,
not a policy. Compare against `moot.toml`, never against the role: per
`MODELS.md` a seat's tier is an **observation**, and the Roles column is a
default that Runtime already legitimately inverts.

**Act on it at a turn boundary, not mid-turn.** The work product is usually
fine — a downgraded seat writes structurally correct code. What you cannot
accept at T3 is its **self-reported evidence** ("controls fired, mutations
reddened"), which is exactly what a handback asserts and what the next reviewer
would spend a full T1 cycle on. Hold the handback, reseat, resume. Killing
mid-turn destroys grounding to pre-empt a risk the review gates already cover.

## RETRACTED — "an unanswered modal blocks the turn"

An earlier version of this lesson claimed an unanswered modal **blocks** the turn
and froze the elapsed counter (`Working (8m 05s)` → `(8m 19s)` across 12 minutes
of wall clock), and prescribed clearing it as an urgent repair. **The operator's
ruling supersedes that.** The work continues; whatever the frozen counter was, it
was not a stopped turn.

⇒ **A frozen elapsed counter under this modal is NOT evidence of a blocked
seat.** That inference manufactured an urgent intervention out of a healthy seat
— which is the expensive direction, since the intervention itself is what can
downgrade the model.
