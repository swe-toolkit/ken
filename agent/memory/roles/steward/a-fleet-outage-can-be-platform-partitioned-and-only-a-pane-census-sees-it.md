---
scope: roles/steward
audience: (see scope README)
source: private memory `a-fleet-outage-can-be-platform-partitioned-and-only-a-pane-census-sees-it`, `a-platform-level-gate-is-not-evaded-by-swapping-model-tier-within-that-platform` (R4 triage, 2026-09-26)
---

# A fleet outage can be platform-partitioned and only a pane census sees it

No convo-level read — `list_participants`, a status line, a since-cursor
poll — can distinguish a dead seat from a quiet one: a status is last-set
and never refreshed, and an event-driven seat correctly produces no events
when it has nothing to say. Only a `tmux` pane census sees the underlying
harness state. When a watchdog finds zero observable movement, first ask
whether the outage is partitioned along a boundary invisible from inside
any single seat — platform, tier, or account pool — before treating it as N
independent stalls.

## The census

Measured 2026-09-14: 16 of 23 live seats were dead on one account-level
quota wall (a multi-day reset), every one on the Codex/pi harness; every
Anthropic seat was alive, and every dead seat's status still read as active
work. A hand-rolled loop over `tmux list-sessions` + `capture-pane`, grepped
for the platform's usage-limit text, found it. `scripts/steward-pane-sweep.sh`
(per-seat busy/strand sheet) and `scripts/pane-busy.sh` (single busy/idle/
suspect verdict) — both present today — cover most of this ground now;
prefer them to a hand-rolled loop.

**Self-exclusion is required** (a bare census greps its own launch command
out of its own pane and reports itself walled) **and is not sufficient once
you broadcast the detector's signature** — quoting the walled text verbatim
in a shared-channel escalation lands it in every recipient's pane, so a
scrollback grep can no longer tell "walled" from "received a post that says
walled." Both are fixed the same way: read by **position** — a pane's last
content line, chrome and footer stripped — never by matching a string
anywhere in scrollback.

**A pane with no `tmux` session is a third state, distinct from walled.**
`tmux has-session -t moot-<role>` separates unseated from walled; a census
built from existing sessions never lists an absent one, so silence there
reads as health. Enumerate the roles you expect, not the sessions that
exist.

**Reset countdowns are relative offsets, not clock times**: `reset =
T_print + N`, `T_print <= now`, so `now + N` is only an upper bound — report
"no later than X." Equality of the raw countdown across seats is not proof
of one shared quota (prints happen at different moments); the test is
equality of `T_print + N`. An *unchanging* countdown across two readings
minutes apart is itself evidence the seat has not retried. Say "unbounded"
for a seat with no reset line, rather than generalizing from the ones that
have one.

## The footer separates SEATED from WALLED, not WORKING from STOPPED

A healthy footer (`ctx NN% · <tier>`) proves the seat is seated, nothing
more. Three seats held identical, healthy `ctx` readings 45 minutes apart —
all three were stopped, each holding an unsubmitted instruction in its
composer. **The discriminator is `ctx` compared across two readings
separated in time**: a working seat consumes context, so identical numbers
on both reads means nothing happened between them. Text sitting in an idle
seat's composer is the free corroborating tell (a wake typed and never
submitted) — the one case where reading the composer, rather than only the
footer, is warranted.

`tmux capture-pane | tail -3` returns both `ctx%` and the tier/model line in
one call — checking only `moot status`/`orientation()` and concluding "no
instrument reads context" mistakes two tools that are not the pane for
exhaustive coverage. Separately, `tmux list-panes -a -F '#{pane_title}'`
partitions the fleet with **no capture at all**: a pi-harness title reads
`π - <role>`, an Anthropic Claude Code title reads `✳ <text>` — zero-cost
evidence of which seats were ever reseated onto which platform.

A monitor keyed on **silence** (the stall's symptom) rather than **frozen
`ctx`** (its mechanism) is unsound, because a correctly idle seat is silent
too: a "no post in N minutes" keepalive eventually hit a seat correctly
holding a ruling, and its wake text ("proceed on the next step you named")
is standing pressure to pick up the nearest task — walking past the ruling
automatically. Write a keepalive to be correct when the seat is idle for a
good reason ("report your state, do not start anything"), never "proceed."

A short tail also cannot tell a modal from a passive notification — capture
enough scrollback to contain an event the seat already received (e.g. `-S
-60`) and check whether it acted afterward, rather than reading the newest
frame alone. Even a genuinely wedged operator-facing dialog is not a seat's
or a Steward's to answer on the human's behalf; surface it and route
around it.

## The partition boundary moves

2026-09-14: the discriminator was platform, off-diagonals clean — no
Anthropic seat down, no Codex seat alive — ruled out against tier (both
weaker- and stronger-tier Codex seats died, so a within-platform tier swap
evades nothing), `ctx%` (24-85% spread), and spend ($10-$423). Confirm a
partition claim by censusing the *whole* roster, not a sample.

2026-09-18: the same 2x2 failed on new data — seats on the same Codex model
string sat on both sides of the walled/working line. The real
discriminator was the account **pool**, invisible in any pane, footer, or
model string. You cannot infer blast radius from a walled seat's platform
once this is known; census the roster and read the blocked set off
directly, every time.

## Consequences worth pricing before acting

- A reseat is per-seat news — stating a wall's reset date at fleet scope
  after reseating one seat overclaims to whoever plans against it.
- A walled or unseated ring's last status keeps standing, including
  "awaiting Steward" on work already merged — verify a parked ring's named
  candidate by content before treating an await as real.
- A hold released by "the seat is back" can never be discharged by a
  roster read, which eventually reads "alive" regardless. Attach the
  availability check to the *routing act* (confirm the seat exists the
  moment you name it as an owner), not to a standing belief about an
  outage — status richness is not recency.
- Correct routing to a dead seat costs more than wrong routing to a live
  one (a live wrong seat bounces back in minutes; a dead one absorbs the
  item silently). Split the mechanism question (can this proceed in-lane)
  from the policy question (should it), and let the in-lane half continue.
- A fully-Anthropic priority lane can be unimpaired by a Codex-wide outage
  — check this before treating a partitioned outage as fleet-wide; it
  argues against a platform migration to revive lanes outside the stated
  priority. Do not kick a dead seat — it reaches nobody and makes the
  thread look answered.
- Live model/platform assignments can be uncommitted-dirty against the
  checked-in config; don't assume the repo's mapping is current.

## Clearing the block: layer, then tier direction

A block at the account/platform layer (quota, a content classifier,
policy, auth) applies to every tier on that platform, so a within-platform
tier swap is never the lever — only a different platform or an
account-level authorization clears it. A block at the model layer (context
limit, capability ceiling, an invalid model string) is where a
within-platform swap can genuinely help. A platform-wide content classifier
blocked one seat; moving it to another tier on the same platform changed
nothing, because the same classifier applies fleet-wide there too.

Even where the layer permits a swap, check tier direction against the
work: moving a seat down a tier to clear a block is acceptable only when
the node is not tier-sensitive. Clearing a block by downgrading tier on
soundness-bearing work is a different failure, not a workaround — a
plausible-but-wrong result that still has to be caught downstream. Name the
remedy the error itself names (often an account-level enrollment) rather
than reaching for a swap that is yours to perform but will not work.

## How to apply

- Treat every convo-level liveness read as unable to see death; run a pane
  census whenever a priority-lane crate stops moving.
- Self-exclude the census from its own pane, and never quote a detector's
  signature string into a channel the census will later read.
- Distinguish unseated (no session), walled (session, dead harness),
  stopped (healthy footer, no `ctx` movement), and working (`ctx` moves
  between two timed readings).
- Census the whole roster before naming a partition boundary, and re-check
  it each time — the boundary can move from platform to tier to account
  pool between outages.
- Before proposing a model swap for a blocked seat, classify which layer
  the block sits at, and check the swap's tier direction against whether
  the work is tier-sensitive.
