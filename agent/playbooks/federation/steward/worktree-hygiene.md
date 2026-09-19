# Worktree and target hygiene

Every role removes the scratch worktree it created when the task lands or is
abandoned (`COORDINATION.md §12b`). The Steward is a resource-emergency
backstop, not a periodic janitor.

## When the Steward acts

Act only when the watchdog finds less than 5 GiB free on the repo filesystem or
an obviously abandoned worktree prevents product work. Do not schedule routine
pruning and do not turn hygiene into a WP.

## Safe worktree removal

A removable worktree satisfies every condition:

1. it is not `/workspaces/ken` or a live role's primary worktree;
2. it has no staged, unstaged, or untracked changes;
3. its owner is absent or confirms the task is closed;
4. its commits are preserved by a branch or landed on `origin/main`.

## Removal commands

Inspect first:

```sh
git worktree list --porcelain
git -C <path> status --porcelain
git -C <path> branch --show-current
git -C <path> rev-parse HEAD
```

Remove without `--force`, then prune metadata:

```sh
git worktree remove <path>
git worktree prune -v
```

A refusal or dirty status means stop and contact the owner. Never delete or
reset it from outside.

## Cargo targets

Never delete a live seat's `target/`. The seat decides when to pay a cold
rebuild. Under a coordinated idle period, ask owners to clean their own targets.

Do not set one shared `CARGO_TARGET_DIR` across worktrees; cross-branch
invalidation and lock contention replace disk pressure with rebuild pressure.
Use the configured `sccache` for cross-seat compile reuse.

Worktrees under `/tmp` consume tmpfs RAM rather than repo-disk space. Attribute
the pressure before reclaiming anything.