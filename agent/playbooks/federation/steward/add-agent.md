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

`MODELS.md` governs which tier the work requires — the tier-to-model table
(Sol = T1, Terra = T2, Luna = T3, by mass) and the rules for judging the work,
including that the line is the work and not the node id. **What it does not
report is any seat's live configuration:** its Roles column and its observation
blocks are dated and have been wrong in both directions.

**You will be tempted to skip this when the doc already says what you expect.**

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

## Keep the soundness-review edge on different model families

This governs one edge: the author whose product receives the Architect's
required soundness review. **The discriminator is model FAMILY.** Provider
diversity is preferred; provider, harness or effort differences alone do not
substitute for it.

- **When a capable alternative at the required tier exists, keep the author and
  the Architect on different model families.**
- **Check both before a reseat and before the review handoff**, so a chain that
  has already collapsed is visible rather than assumed intact.
- **When no capable alternative exists, state `same-model correlated review` in
  the handoff** and do not count agreement as independent corroboration. Do not
  waive it silently, and do not add a reviewer or a review hop.

Same-family review has correlated blind spots; it is not thereby no review.

**Read the block's comments, not only its keys.** A `grep` of `model =` lines
tells you what a seat is set to and never whether the edge is already known
broken — the block may already record the collapse, when it was surfaced, and
which seat to move to restore it.

## Negative scope

- **Credentials are not yours.** Never read another seat's `api_key`, never open
  or dump `.moot/actors.json`, and never propose a credential change. Resolve an
  `actor_id` with `scripts/moot-actor-id.sh <role>`.
- **The retired `integrator` seat stays retired.** Never rouse or reseat it.
- **`steward.md §7` still holds for every file it lists.** This covers seats and
  `moot.toml`, not the playbook corpus, federation law, `MODELS.md`, memory,
  skills trees, CI workflows, or publisher scripts.
