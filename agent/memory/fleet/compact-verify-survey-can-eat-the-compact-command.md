---
scope: fleet
audience: (see scope README)
source: private memory `compact-verify-survey-can-eat-the-compact-command`;
  merged in 2026-09-27: fleet lesson `compaction-render-delay-escape-aborts`
  (private memory of the same name, CAT-3 kickoff gate)
---

# A tmux `/compact` can be eaten, delayed, or aborted — verify the live pane

## A Claude Code survey prompt can eat a `/compact` command

When I run the build-team compaction handoff gate
(`tmux send-keys -t moot-<role> "/compact"` → Enter, per member), a Claude Code
**feedback survey** ("How is Claude doing this session?
`1: Bad 2: Fine 3: Good 0: Dismiss`") can **pop up and silently intercept the
`/compact`** — the keystrokes go to the survey (which only accepts 0–3), the
command never runs, and the pane returns to an empty `❯` at the **original
ctx%**. If I only glance at "sent" or a transient progress bar, I'll wrongly
believe the member compacted → risk the exact stale-context kickoff Pat forbids.

**Live (2026-07-04, W-style-match-IH build kickoff):** leader (14%) and
implementer (19%) compacted to 0% first try, but **language-qa's `/compact` was
eaten by the survey twice** — it sat at 16% while I thought all three were done.
Only the ctx-drop verification (leader 0% / impl 0% / **qa still 16%**) caught
it.

**How to apply — this is why gate step 5 ("VERIFY THE DROP") is
non-negotiable:**
- **Dismiss the survey with its own `0: Dismiss`** (send `"0"` — surveys respond
  to the bare digit, no Enter), confirm it's gone, THEN send `/compact`.
- **Never trust the "sent" line or a momentary `▰▰▰ NN%` bar.** The only proof
  is **`ctx` actually falling to ~0%** on a `capture-pane` taken *after* the
  30–60s stale-display window. `Compacting conversation… NN%` climbing (with an
  elapsed timer) is the real in-progress signal; an unchanged ctx% with no
  compaction message ⇒ it didn't run ⇒ resend.
- A queued `/compact` behind a busy turn ("Press up to edit queued messages")
  fires at the turn boundary — fine; a redundant second one just reports "Not
  enough messages to compact." (harmless).
- **The slash-command autocomplete menu can swallow your Enter (2026-07-04, FS
  Runtime kickoff).** Typing `/compact` pops the `/`-command menu; a blind
  `send-keys Enter` fired too soon (even ~2s later, batched across 3 panes) left
  **all three at empty `❯` / unchanged ctx** — the submit didn't land. The
  reliable sequence: type `/compact`, `capture-pane` to confirm the input line
  reads `❯ /compact` (menu open), send Enter, then confirm the pane transitioned
  to **queued ("Press up to edit queued messages") or `Compacting…`** — only
  then is it accepted. Don't fire Enter and assume; confirm the transition, then
  poll ctx→0. (Same "surface signal lies" theme — the send returns success while
  nothing was submitted.)
- **The slash menu can fail to open at all, and the model then answers
  `/compact` in prose as ordinary chat (2026-09-04).** The reply is confident
  and on-topic ("Ready. All state is durable...") while ctx keeps climbing
  and no `Compacting…` bar ever appears, so it is the most convincing false
  positive of the three. Send `/` alone first, `capture-pane` to confirm the
  command list rendered, then send the filter text and a separate Enter. A
  reply that talks about compacting is not evidence that it happened.

Sibling of the compaction discipline in re read latest events immediately before
a stall nudge — same theme: the surface signal (sent / idle box / stale ctx)
lies; verify the underlying state.

## `/compact` has a render delay; Escape aborts it, so never send Escape

Running the §2c compact-gate (send `tmux send-keys /compact` + a separate
`Enter` to each enclave/team seat, then verify the ctx drop), there is a **real
render delay of several seconds** between the `Enter` and the
`✻ Compacting conversation…` progress bar appearing. So a capture taken
immediately after `Enter` shows an **empty prompt + unchanged ctx% + no
Compacting marker** — which looks identical to a swallowed/no-op command **even
when compaction is about to start**.

**The trap (CAT-3 kickoff gate):** on that ambiguous read the compacting seat
reflexively sent **Escape to "clear state" and re-typed `/compact`** — and
**Escape aborts an in-flight compaction** (`AbortError: Compaction canceled.`
appeared 3× in CV's scrollback; each retry started compaction, then the next
Escape killed it). About 4 cycles were burned fighting a compaction that was
firing fine each time.

**Fix / procedure:**
1. After `/compact`+`Enter`, **wait a full tool round-trip** (re-capture is
   enough delay) and **re-capture the FULL pane** (`tail -14`, not `tail -6`)
   before concluding anything — look for `Compacting conversation…` OR the
   `AbortError`/scrollback history.
2. **Never send Escape as a reset** during the gate — Escape = abort-compaction.
   If a seat truly didn't fire, just re-send `/compact`+`Enter` (a second
   `Enter` if the autocomplete palette ate the first), no Escape.
3. **Read the LIVE bottom `ctx N%` line specifically.**
   `grep 'Compacting' | head -1` catches a **stale scrollback** Compacting frame
   (top-to-bottom order); `grep 'ctx N%' | tail -1` can catch the
   mid-compaction status line (ctx doesn't update until compaction completes).
   Confirm with `tail -6` of the live pane.
4. A Sonnet seat (e.g. spec-leader) with the full playbook loaded has a
   **post-compaction floor ~16%**, not 0% — it still counts as compacted if it
   visibly ran the bar; don't chase it to 0.

Both halves of this lesson are "the send-keys didn't do what the pane's first
frame suggests"; verify by re-reading the live pane, not by assuming and
re-sending destructively.
