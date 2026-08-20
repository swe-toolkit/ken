# Adding an agent to the fleet: provision, credential, seat

Steward task procedure. Read at the point of use. Governing playbook:
`../steward.md`.

Stand up one new seat end-to-end: create its convo identity, obtain its
credential, register it locally, focus it on the space, and launch it. Verified
live 2026-08-20 against `https://mootup.io` (tenant `ten_37rekknwzp000`) while
adding the merge lieutenant.

The convo API is the authority. The moot CLI's own provisioners are **not** the
current path (see Dead paths). The current create route lives in the convo
server source at `local/refs/convo/backend/api/routes/actors.py` — an authorized
infra read (it is convo's own tooling, not a clean-room Ken reference).

## What you own here, and what you do not

Yours: the whole sequence below. Not yours: choosing that a seat *should* exist
(operator direction), the harness/model tier (`../MODELS.md`, moot.toml), and
the seat's playbook (its startup_prompt and skill). This file only wires an
already-decided seat into the running fleet.

## Prerequisites

- **A human PAT** — prefix `mootup_pat_`. Sponsoring an agent is a human-only
  act; an agent api_key (`convo_key_`) gets `403 "Only humans can sponsor
  agents"` / `"Only humans can list agents"`. The operator's PAT is at
  `.mootup/credentials`; `moot login` also stores one at `~/.moot/credentials`
  (TOML, `[default].token`). The PAT needs the `actor:write` scope.
- **The moot.toml block** for the seat — `[agents.<name>]` with `harness`,
  `model`, `effort`, env, and `startup_prompt`. Add it to the **live**
  `/workspaces/ken/moot.toml` (the main-repo copy, not a worktree copy).
- **The harness credential.** Claude Code seats need nothing extra. pi + OpenAI
  is OAuth, not an api_key: one interactive `/login` → "ChatGPT Plus/Pro
  (Codex)" writes `~/.pi/agent/auth.json`, and because pi seats share `$HOME`
  that single login serves every pi seat. The pi moot.toml block therefore
  carries **no** `${secret:...}` / api-key env, unlike the OpenRouter pi blocks.

Handle the PAT and every returned api_key as secret: never echo to the
transcript, never commit, never send to an unrelated service.

## The mechanism (current, confirmed live)

Run this as one script so the plaintext key never lands in a shell variable you
print. Back up `.moot/actors.json` first; write it back atomically at `0o600`.

1. **Create the identity.** `POST /api/agents`, Bearer the human PAT, body
   `{"role": "<name>", "agent_profile": "<= 512-char descriptor"}`. Tenant and
   sponsor derive from the token — the body must **not** carry `tenant_id` or
   `sponsor_id` (Pydantic drops them silently; sending them is a sponsor-escalation
   tell, not an error). `role` becomes the actor's `display_name`. Returns `201`
   with the new actor and a `registration_ticket_id` (SEC-5 — **not** a plaintext
   key).
2. **Exchange the ticket for the key.** `POST
   /api/registration-tickets/{ticket_id}/exchange`, **no auth header** (the
   ticket is the auth: single-use, 5-min TTL, IP-rate-limited). Returns
   `{actor_id, api_key, api_key_prefix, ...}`.
   - **Tolerate both shapes.** Read `api_key` off the create response first; only
     if it is absent, exchange the `registration_ticket_id`. A deployment that
     predates SEC-5 returns the plaintext key directly. The live server returns
     the ticket.
3. **Register it locally.** Load `.moot/actors.json`, set
   `actors["<role_key>"] = {actor_id, api_key, display_name}` where
   `role_key = display_name.lower().replace(" ", "_")` and matches the moot.toml
   `[agents.<name>]` name. Write back at `0o600` under the `0o700` `.moot/`. The
   moot runtime reads only `entry["api_key"]` and `actor["actor_id"]`; the
   top-level `space_id` is shared by all entries.
4. **Focus it on the space.** `PUT /api/actors/focus`, Bearer the PAT, body
   `{"actor_ids": ["<actor_id>"], "space_id": "<spc_...>"}` (sponsor-only).
   Returns `200` with `{"updated": ["<actor_id>"]}` — an empty `updated` means
   the PAT does not sponsor that actor. The seat's first `orientation()` joins
   its focus space as a side effect, so this is what puts it in the room — no
   separate join call.
5. **Seat it.** `moot down <name>` then `moot exec <name>`. A seat launched
   before its actors.json entry existed must be restarted to re-read the
   credential and initialize its convo tools. `moot exec` writes tmux session
   `moot-<name>`.

Ken space id: `spc_4q7g0se87rgje`. Confirm the current id from
`.moot/actors.json` top-level `space_id` rather than trusting this line.

## Verify

Read the seat's pane, not just the tmux listing:

```sh
tmux capture-pane -t moot-<name> -p -S -40 | grep -v '^$' | tail -30
```

Working looks like: `mcp__convo__orientation` runs (no `convo tools
unavailable: missing apiKey/actorId`), the model produces output (for pi/OpenAI,
no `No API key found for openai-codex. Use /login`), and the footer shows
`spc_... · N unread` — the seat is in the space. A fresh `moot exec` seat may
stop at the dev-channels modal: answer digit `1`, not Enter (`../steward.md`
watchdog 2e).

Confirm the identity registered: `GET /api/actors/me/agents` with the PAT lists
it; `is_connected: true` on its participant record once the seat is up.

## Dead paths — do not spend time here

- `moot config provision` — POSTs the legacy `/api/tenants/{tenant_id}/agents`,
  now `404`. Writes a stray `.agents.json` (empty). Remove that file if it
  appears.
- `moot init` — the CLI's own provisioner, but it **refuses** when
  `.moot/actors.json` already exists. `--force` rotates **all** existing keys
  (breaks the running fleet); `--adopt-fresh-install` overwrites `CLAUDE.md`,
  `.claude/`, and `.devcontainer`. None is surgical for adding one seat.
- The CLI's `scaffold.py` reads `api_key` straight off the rotate-key / create
  response. The live server returns a SEC-5 **ticket** there instead, so the CLI
  lags the server — which is why the steps above go direct to the API and handle
  both shapes.

## Removing or re-homing a seat

- **Delete an identity you created:** `DELETE /api/actors/{actor_id}` as its
  sponsor. The create is reversible; a mistaken seat is not permanent.
- **Move a live key to another install:** `POST /api/actors/{actor_id}/rotate-key`
  (sponsor or self). If the agent `is_connected` elsewhere it returns `409`
  unless you send `X-Force-Rotate: true`; the clean form is `POST
  /api/actors/{actor_id}/release` first, then rotate without the header.
