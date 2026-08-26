---
name: gh-access
description: Who in the fleet may do what on GitHub, and how any seat mints a token to read CI Action logs. Lieutenant publishes; Steward writes under operator direction; everyone else is read-only. Read this before running any `gh` command or diagnosing a CI failure.
scope: tools
---

# GitHub access

The fleet has exactly one GitHub credential — the publisher App identity
(`04-git-and-integration.md §3`). This skill says who may use it for what, and
how a seat mints a token to **read** CI when a check goes red. It does not
restate the publisher mechanics (that is `lieutenant.md` and
`steward/merge-procedure.md`).

## Who may do what

| act | who | how |
|---|---|---|
| publish: push `wp/<ID>`, CI-gate, merge | **lieutenant** | scripted publisher path (`lieutenant.md`) |
| any other write, under operator direction or exceptional circumstance | **Steward** | corpus route (`agent/**` + `IMPLEMENTATION-PROGRESS.md`); or operate the publisher when no lieutenant is seated |
| **read** CI runs, logs, PR/check status | **any seat** | mint a token (below), read-only `gh` |

Everything not in the first two rows is read-only. A build or spec seat that
needs a write **routes it** — a `git_request` to the Steward for a merge, or to
the lieutenant for a publish. You do not push, merge, rerun, or open/close/edit
a PR yourself.

## Mint a token

```sh
export GH_TOKEN="$(/workspaces/ken/.devcontainer/mint-gh-token.sh)"
```

Valid ~1h; re-run before a long gap. Never persist it, echo it into a post, or
send it to any non-GitHub service. The devcontainer has `gh` but it is not
logged in — the token is how `gh` authenticates.

## Read a failing CI run

```sh
gh run list  -R swe-toolkit/ken --branch <wp-branch> -L 5   # find the run id
gh run view  <run-id> -R swe-toolkit/ken --log-failed       # only failed steps
gh pr checks <pr-number> -R swe-toolkit/ken                 # check roll-up
```

`--log-failed` is the one to reach for first: it prints just the failing steps,
not the whole workflow.

## The boundary is behavioral — treat it that way

The token you mint is that one publisher identity (above) — there is no
read-only variant — so a read-only seat holds a write-capable token and is
trusted not to mutate. A write you did not route is a policy breach, not a
shortcut.

- **You will be tempted to `gh run rerun` a red check while diagnosing.** A
  rerun is a write and is not yours: hand your diagnosis to your leader, and the
  respin (a real fix at a new SHA) or a flake rerun (a publish-path
  close/reopen, `merge-procedure.md`) happens there. Your job ends at the
  diagnosis.
- A green local read never authorizes a merge — CI on GitHub is the code gate,
  and merging is the lieutenant's act on an approved SHA.
