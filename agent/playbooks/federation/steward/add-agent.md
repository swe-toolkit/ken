# Managing seats

The fleet is the Steward's to run. Start, stop, restart, reseat, and change a
seat's model or tier as the product needs. Do not ask permission, and do not
route a seat change to the operator as a request.

## You own seats. The operator owns lanes.

A seat change never authorizes a lane. Lane count, ordering, and objectives stay
operator-owned in `steward/lanes.md`. Reseating the implementer on an authorized
lane is yours; standing up a ring for an unauthorized lane is not, however idle
the seats look.

## Read both authorities before any tier claim

They are different questions and either can be stale against the other:

- **what a seat is running now** — observe the live pane or footer;
- **what it will run at next launch** — `[agents.<role>]` in `moot.toml`.

An in-session model change moves the first without the second. **Read both
before a tier claim or a reseat, and reconcile a mismatch** rather than taking
whichever answer you expected. Observation after restart is the final check.

**You will be tempted to skip this when the doc already says what you expect.**

## What `MODELS.md` settles, and what it cannot

It governs which tier the work requires — the tier-to-model table
(Sol = T1, Terra = T2, Luna = T3, by mass) and the rules for judging the work,
including that the line is the work and not the node id. **What it does not
report is any seat's live configuration:** its Roles column and its observation
blocks are dated and have been wrong in both directions.

## The key split is what silently breaks a reseat

Changing `model` alone misconfigures the seat. Each harness carries a different
key set:

| seat | keys |
|---|---|
| pi | `model = "openai-codex/<m>"`, `harness = "pi"`, `effort`, `PI_*` `env`. No `model_reasoning_effort`. |
| Claude Code | Anthropic `model`, no `harness`, `effort`, no `env`. |
| codex | `harness = "codex"` and `model_reasoning_effort`, not `effort`. |

`moot.toml`'s header carries the current shapes and a consistency check. Read it
before editing: the requirement is invisible from a seat that is already correct.

## Reseat

1. Observe what the seat is running now, and read its `[agents.<role>]` block.
   Reconcile them before editing if they disagree.
2. Edit `model` **and** the full key set its harness requires.
3. Record in the block what changed and what you checked.
4. `moot down <role>`, then `moot up --only <role>`.
5. `moot status` to confirm RUNNING, then **confirm the new seating by
   observation** — never from the edit you just made.

## Reviewer independence is a property of context, not model

This governs one edge: the author whose product receives the Architect's
required soundness review.

Independence holds when the reviewer's verdict comes from its own measurement.
It collapses when the reviewer inherits the author's instrument or premise, or
rules on a shape it authored itself — whatever either seat is running.

**Do not spend a reseat or a provider swap on this.** They change what a seat
runs, not where its verdict comes from.

## Check independence at every review handoff

Not from the config: a `grep` of `model =` tells you what a seat is set to and
nothing about where a verdict came from. Ask instead:

1. Did the reviewer measure the claim, or restate the author's evidence?
2. Is it ruling on a shape it authored or already ruled?
3. Would its verdict change if the author's instrument were wrong?

**You will be tempted to skip this when the seats are on different models**,
because difference reads as independence. It is the weaker signal.

A wrong answer to any of the three means **state it in the handoff** and do
not count agreement as independent corroboration. Do not waive it silently,
and do not add a reviewer or a review hop.

## Negative scope

- **Credentials are not yours.** Never read another seat's `api_key`, never open
  or dump `.moot/actors.json`, and never propose a credential change. Resolve an
  `actor_id` with `scripts/moot-actor-id.sh <role>`.
- **The retired `integrator` seat stays retired.** Never rouse or reseat it.
- **`steward.md §7` still holds for every file it lists.** This covers seats and
  `moot.toml`, not the playbook corpus, federation law, `MODELS.md`, memory,
  skills trees, CI workflows, or publisher scripts.
