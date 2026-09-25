# Compaction

Compaction protects a clean work boundary. It is not a recurring management
project and it is never performed mid-turn.

## Steward self-compaction

Near 33% context, finish the current action, leave the worktree clean, and write
a short durable checkpoint containing:

- current lane actions;
- any routed-but-unlanded SHA;
- the next concrete act.

Then run:

```sh
moot compact steward
```

Stop the turn. On resume, re-orient, read the checkpoint and
`steward/lanes.md`, check unread mentions, and re-arm the watchdog.

## Team compaction

Compact a build team or Spec enclave only at a genuine new-WP boundary. Do not
compact for:

- another increment or respin of the same WP;
- QA or reviewer handoff;
- a question, ruling, or ordinary status change;
- a desire to lower a context percentage while the seat is working.

## Team preconditions


1. every member is quiescent;
2. no member owes an unfinished vote, answer, handoff, or unmerged artifact;
3. dirty or ahead worktrees have been resolved by their owners;
4. the new frame is already landed on `origin/main`.

## Team mechanism

Run the checked-in gate with every member named:

```sh
scripts/handoff-gate-compact.sh <leader> <implementer> <qa>
```

For the Spec enclave, name `spec-leader`, `spec-author`, and
`conformance-validator`. Verify each pane shows a context drop, active
compaction, or a queued compaction. A sent command without an observed result is
not a completed gate.

After the drop, post the kickoff and rouse any compacted no-poll seat. The order
is compact, verify, kick, rouse. A kickoff sent before compaction can be consumed
by the compaction.

## Pi seats

Pi seats have auto-compaction. Between work boundaries, let it operate. Do not
chase the former 25/33/45-percent thresholds. At a new-WP seam, the explicit gate
above still applies because old task context is disposable there.

The lieutenant has no new-WP seam in its continuous merge queue. Do not compact
it routinely; let it auto-compact or restart it only when the operator directs.

## Adversary

Not yours. The lieutenant compacts the Adversary at M8; the mechanism is in
`lieutenant.md`. It does not self-compact and takes no periodic schedule, so do
not give it one here.

## Failures

- If a pane is working, leave it alone.
- If the gate refuses a dirty worktree, the owner resolves it; the Steward does
  not reset another seat by hand.
- The gate resets only home branches (`<seat>/work`). A seat on another branch
  is switched home and that branch is left in place; if the switch fails, the
  seat is compacted without a reset. Tell the seat its WP branch in the kickoff.
- If context did not drop, retry only that idle pane and verify again.
- If a kickoff was sent before the drop, reuse the original WP thread anchor
  when re-delivering; never create a second thread.