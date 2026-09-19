# Merging: M0-M9

`COORDINATION.md §14b` is the authority. The Steward runs M0-M3a and stops after
posting `ROUTED:`. The lieutenant runs M4-M9. The Steward runs M4-M9 only when
no lieutenant is seated or the operator explicitly directs the fallback.

`merge-policy.md` decides whether a cut should land. This file executes that
decision.

## M0 — Check whether the content already landed

Squash merges break ancestry. Compare the candidate's own changed paths by blob
identity before doing any other work:

```sh
C=<candidate>; TIP=$(git rev-parse origin/main)
BASE=$(git merge-base "$C" "$TIP")
for f in $(git diff --name-only "$BASE" "$C"); do
  a=$(git rev-parse "$C:$f" 2>/dev/null)
  b=$(git rev-parse "$TIP:$f" 2>/dev/null)
  [ "$a" = "$b" ] && echo "IDENTICAL $f" || echo "DIFFERS $f"
done
```

All identical means nothing is owed. A differing busy shared file is not proof
that the candidate is unlanded; inspect whether the candidate's own added or
changed content is present on `main`.

## M1 — Verify the Decision and approvals

Read the Decision fresh. It must be `resolved` and name the exact candidate SHA.
Confirm every reviewer required by the candidate's current path scope approved
that same SHA. A verdict on a superseded SHA does not carry.

A capped decision listing proves presence but not absence. If an old Decision
falls outside the returned window, report the read as inconclusive rather than
inventing a missing gate.

## M2 — Verify the object and scope

Use the merge-base declared by the ring:

```sh
C=<candidate>; BASE=<declared-base>
git cat-file -e "$C^{commit}"
git merge-base "$C" origin/main
git log --oneline "$BASE..$C"
git diff --shortstat "$BASE...$C"
git diff --name-only "$BASE...$C"
```

The commit count, shortstat, and paths must match the handoff and the reviewed
range. Read any commit body that declares `not merge-ready`, `do-not-merge`, or
`do not build on`; ask the ring whether the named condition is resolved at the
tip.

For a non-empty overlap with changes on current `main`, inspect the semantic
union:

```sh
comm -12 \
  <(git diff --name-only "$BASE" "$C" | sort) \
  <(git diff --name-only "$BASE" origin/main | sort)
```

Classify CI mechanically with `scripts/ci-doc-only.py`. `doc-only` controls CI
cost; it does not decide who reviews. Apply `COORDINATION.md §14a` separately.

## M3 — Record cited-source impact

Check changed paths against `library/SOURCE-ATTESTATIONS`. A hit is a downstream
Librarian notification after landing, never a reason to add work to the build
ring or candidate.

## M3a — Authorize once

Before posting, ask: **is any required reviewer still expected on this exact
SHA?** If yes, do not post `ROUTED:`. A qualified `ROUTED, HOLD` is still an
authorization and is prohibited.

When every gate is complete, post one message mentioning the lieutenant:

```text
ROUTED: <full SHA>
branch: <branch>
base: <merge-base>
Decision: <resolved decision id>
approvals: <required exact-SHA approvals>
scope: <paths and shortstat from the merge-base>
CI class: <doc-only or full>
```

The Steward stops here. Once routed, the lieutenant owns execution end to end.

## M4 — Mint the executor token

```sh
export GH_TOKEN="$(/workspaces/ken/.devcontainer/mint-gh-token.sh)"
```

Never print the token.

## M5 — Publish and wait for CI

Before publishing, ensure no publisher already owns the window and no
`main` CI run would be cancelled by a new push. Then run:

```sh
scripts/scripted-pr-automerge.sh \
  --target <SHA> --title <title> \
  (--description <text> | --description-file <path>) [--doc-only]
```

Use the background path for code candidates. Do not fetch or mutate refs while
the publisher is active.

On red, stop. Attribute the failure to candidate, base, or infrastructure and
relay the evidence to the ring and Steward. The ring owns any respin; the
Steward verifies and routes the new SHA. Retry an infrastructure failure on the
same SHA only after confirming the SHA and PR head did not move.

## M6 — Verify the landing

Fetch and compare every path from the reviewed merge-base range against landed
`origin/main`:

```sh
git fetch origin --prune
for f in $(git diff --name-only <BASE>...<SHA>); do
  landed=$(git rev-parse "origin/main:$f" 2>/dev/null)
  routed=$(git rev-parse "<SHA>:$f" 2>/dev/null)
  [ "$landed" = "$routed" ] && echo "MATCH $f" || echo "DIFFER $f"
done
```

A path-count mismatch or `DIFFER` is a failed landing verification. Do not close
the WP until reconciled.

## M7 — Close product state in a batch

Record the landed squash SHA in the WP thread. Update the issue status and
regenerate `IMPLEMENTATION-PROGRESS.md`, but do not publish one management
commit per merge. Accumulate closeouts and publish at most one docs-only batch
when the routed product queue is empty or a successor release needs the state.

Never mix a frame rewrite, playbook change, lesson, or cleanup into the closeout
batch. Close only nodes whose product content was verified in M6.

## M8 — Notify the Adversary after code merges

For a merge carrying product code, compact the Adversary, verify the drop, then
notify it once with the landed squash SHA and changed paths. Rouse its pane.
Docs-only merges skip M8. Do not ask for a verdict and do not reply to reports
(`COORDINATION.md §10⁻a`).

## M9 — Close the loop

Notify the owning leader of the landed squash SHA. If a next slice is already
framed, authorized, dependency-clear, and selected by `steward/lanes.md`, the
Steward releases it through `release-and-handoff.md`. The lieutenant does not
invent, frame, or re-scope successors.

Return the executor worktree to its home branch at current `origin/main` and
remove temporary merge worktrees or refs.