# Releasing and handing off a WP

Use this for a genuine new-WP boundary. It is not run for every message or
within-WP handoff. Governing playbook: `../steward.md`. Frame content:
`frame-authoring.md`.

## 1. Establish readiness

A WP is ready only when:

- it belongs to an operator-authorized lane;
- its dependencies are merged on `origin/main`;
- its short frame and issue file are landed and agree;
- any required design or scope ruling is settled;
- its expected path set does not conflict with another active or publishing WP.

Verify the named objects from the landed base:

```sh
git cat-file -e origin/main:<frame-path>
git cat-file -e origin/main:<issue-path>
git show origin/main:<issue-path> | head
```

Do not publish a kickoff that describes state which exists only on a local
branch.

## 2. Compact at the new-work boundary

Compact the receiving unit only because it is starting a new WP. Do not compact
for QA handoffs, questions, respins, review replies, or another increment of the
same active WP.

The unit must be quiescent and owe no unfinished action. Then run:

```sh
scripts/handoff-gate-compact.sh <leader> <implementer> <qa>
```

Use the Spec enclave's three roles when it is the receiving unit. Verify each
pane shows a context drop, active compaction, or queued compaction. If the unit
is mid-turn, wait for its clean seam; never compact away live work.

The script refreshes worktrees. If a worktree is dirty or carries unmerged work,
stop and ask its owner to resolve it. Do not reset another seat by hand.

## 3. Kick with a pointer, not a second frame

The kickoff is a top-level thread anchor and contains:

- WP id, lane, and exact landed base;
- frame and issue paths;
- deliverable name and tier;
- the next seat expected to act;
- this exact anchor instruction:

> This message is the thread anchor for `<WP-ID>`. Reply with
> `parent_event_id` set to this event's id; thereafter use its `thread_id`.
> Do not open a second thread for this WP and do not post about it at the space
> root.

Do not restate the frame, the tests, federation law, prior incidents, or the
whole campaign. The receiver reads the landed frame.

## 4. Confirm pickup

A successful post is not a successful kickoff. Confirm the leader or assigned
seat reaches `Working`. If it does not, repair transport once and re-check the
same anchor; do not create another thread.

Update `steward/lanes.md` by replacing the active-WP field. Bundle that small
change with the frame release or the next product-attached closeout. Do not
publish a standalone `ready -> active` or tracker-sync commit.

## Same-WP handoffs

Within an active WP, the owning leader routes implementer, QA, and review work.
The Steward does not compact the unit, repeat the kickoff, update lane state, or
restate the handoff. Intervene only for a scope fork, a dropped transport, or a
held finished candidate.

## Stay one release ahead

After release, identify the one immediate successor implied by the lane
objective. Frame it only if its dependency, product deliverable, and design are
already settled. A list of possible successors is not runway; it is speculative
management work.