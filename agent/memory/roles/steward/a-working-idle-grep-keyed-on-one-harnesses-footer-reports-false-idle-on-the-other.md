---
name: a-working-idle-grep-keyed-on-one-harnesses-footer-reports-false-idle-on-the-other
description: The working/idle discriminator is harness-specific. Grepping a pane for "esc to interrupt" reports FALSE IDLE on a Claude Code seat mid-tool-call, which renders a spinner word plus an elapsed timer instead. The failure direction is the dangerous one -- it invites a nudge that interrupts live work. Match several alternatives, and remember Codex prints "Worked for Nm" AFTER a turn ends.
metadata:
  type: feedback
---

**Measured 2026-08-10 ~11:5xZ.** A liveness sweep keyed on `esc to interrupt`
reported `runtime-implementer: idle` while its leader had just posted that the
implementer was working. The pane actually showed:

```
● Running 2 shell commands · 32s…
  ⎿  $ scripts/ken-cargo test -p ken-runtime --lib 2>&1 | tail -4 (32s)
✶ Levitating… (2m 37s · ↓ 7.1k tokens)
```

**It was mid-`ken-cargo` run.** A nudge on that reading would have interrupted a
correctly-scoped targeted test under the machine-wide build flock.

## The discriminator is harness-specific, and the sweep spans both harnesses

| harness | working looks like | after the turn ends |
|---|---|---|
| Codex (`gpt-*`) | `Working (Nm Ns • esc to interrupt)` | `Worked for Nm` |
| Claude Code | a spinner word + `(Nm Ns · ↓ N.Nk tokens)`, often `Running N shell commands`; **`esc to interrupt` is not always present** | `❯` prompt, no timer |

⇒ **A single-string grep silently partitions the fleet into seats it can read
and seats it always reports idle.** The sweep looks complete because it visits
every pane and prints a verdict for each.

**And `Worked for Nm` is an IDLE marker, not a working one** — it is the
past-tense summary Codex prints when a turn finishes. A grep for `Work` matches
both states and gets the answer backwards half the time.

## The fix

Match **any** of these, rather than one:

```sh
grep -qE "esc to interrupt|Running [0-9]+ shell command|\([0-9]+m [0-9]+s"
```

The elapsed-timer alternative `([0-9]+m [0-9]+s` is the one that generalizes —
both harnesses print a running timer while a turn is live, and neither prints
one at an idle prompt.

## The repair that replaced this grep had the SAME bug, from the other side

**Measured 2026-08-14, and this is the part to read if you are short of time.**
After the miss above, the Steward stopped grepping `esc to interrupt` and
adopted a **composer-content** heuristic instead: *"genuine idle = a placeholder
prompt (`> Run /review on my current changes`)."* It went into the watchdog as
settled guidance.

**It is false in exactly the case that matters.** A Codex seat waiting on a
**background terminal** renders the placeholder prompt while genuinely working:

```
• Waiting for background terminal (25m 07s • esc to interrupt) · 1 background terminal running
  └ scripts/ken-cargo test -p ken-runtime --lib

› Run /review on my current changes
```

`runtime-implementer` was 25 minutes into a `-p ken-runtime --lib` run and the
composer said idle. **Two heuristics, opposite keys, same failure direction:**
both report false idle on a seat mid-build, and both invite the nudge that
interrupts it.

⇒ **The composer is never a liveness signal, in either direction.** Codex
restores its placeholder the moment the composer is empty, which includes every
moment the seat is busy but not typing. What actually moved between the two
states was the **timer line above it**, which is what the fix in this file
already said to match — and which the composer heuristic quietly stopped
consulting.

**Read the whole footer (`tail -14`), not one line.** A seat is working if
*anything* in the last few lines carries a running timer or a
`background terminal running` clause, whatever the composer shows.

## Why the direction matters

**False idle invites action; false busy invites waiting.** This detector fails
toward *action* — the sweep says a seat is idle with work assigned, which reads
as a stall, and the prescribed recovery is a nudge. **The recovery is what does
the damage**, because it lands on a seat that was fine. Compare
[[a-stranded-hub-seat-presents-as-n-correct-looking-waits]], where the failure
is toward waiting and merely costs time.

**Verify a stall before acting on one.** A second read a few seconds later, or
one full-pane capture instead of a keyword grep, distinguishes them for free.
The keyword grep is an optimization over reading the pane; when it disagrees
with what a leader just told you, **read the pane**.
