---
name: architect-self-compact
description: The Architect's self-compaction mechanics — the tmux path, the detached resume watcher, and why request_context_reset is broken in this harness. Read at the point of compacting, not while reviewing.
scope: federation
---

# Architect self-compaction: the mechanics

Architect task procedure. Read at the point of use. Governing playbook:
`../architect.md` section 3, which owns *when* to compact and what
`ARCHITECT-STATE.md` must hold. This file is *how*.

## Do NOT use `request_context_reset`

**Operator, 2026-07-02.**
It is **broken in this local harness**: it hunts for a moot-managed
`convo-<role>` session that does not exist here and fails with *"No tmux
session 'convo-architect' found."* **That error message is naming the bug,
not a target** — do **not** then retry `tmux … -t convo-architect`; there is
no such window. The **only** reliable self-compact is the `tmux send-keys`
path pointed at **your own** window, and the windows are named `moot-<role>`
(yours is `moot-architect`):

```bash
# 1) Launch the DETACHED resume watcher FIRST — it outlives this turn AND the
#    compaction, waits for `/compact` to finish, then sends the `resume`:
nohup scripts/postcompact-resume.sh moot-architect >/tmp/pcr-architect.log 2>&1 & disown
# 2) THEN queue your own /compact (fires at turn end) and make it your LAST action:
tmux send-keys -t moot-architect -l '/compact' ; sleep 2 ; tmux send-keys -t moot-architect Enter
```

The two-step (type `/compact`, wait ~2s, then a **separate** `Enter`) avoids
the fused-keystroke race that leaves `❯ /compact` sitting unsent on the input
line. `/compact` fires at the **end of the current turn**, so make it your
**last action** — finish refreshing `ARCHITECT-STATE.md` first. You
self-compact only; you never compact another agent (that is the Steward's
job, via the same `moot-<role>` tmux path — `moot compact` is no-op-prone).

## The `resume` is fired by a DETACHED watcher, not a buffered message

**Operator, 2026-07-11 — a self-compact leaves you IDLE, not resumed.**
`/compact` returns your seat to an empty `❯` prompt and **nothing re-invokes
it**; you would sit idle until roused. The old fix — type `resume` right after
`/compact` and hope the host buffers it behind the compaction — is a **race**:
the `resume` is sent while your turn is still active (the queued `/compact`
fires only at turn end), so it can land as its own live turn instead of
post-compaction. The reliable fix **decouples** the resume-send from your turn
lifecycle: `scripts/postcompact-resume.sh` launched **detached** (step 1 above,
*before* you send `/compact`) keeps polling your pane, catches the
`Compacting…` window, waits for it to clear, and only **then** sends `resume`.
Because it is a separate process it is immune to the turn/compaction lifecycle.
The post-compact re-orient hook (`scripts/hooks/reorient-post-compact.sh`) then
re-orients you and you continue your in-flight review autonomously. (A hook
alone cannot trigger the resume — it only shapes the next turn's context, not
whether one happens; that is why an external sender is required.) This is
self-compaction only.

