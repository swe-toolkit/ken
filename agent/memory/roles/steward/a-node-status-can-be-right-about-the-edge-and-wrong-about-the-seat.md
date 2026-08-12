---
scope: roles/steward
audience: (see scope README)
source: 2026-08-12 — three of the four `active` tracker nodes had no seat on
  them, and the one that mattered made an idle Kernel ring read as unblocking
---

# A node's status can be right about the edge and wrong about the seat

`KERNEL-NESTED-IND` read *"blocked by `RT-DYNAMIC-ARM-SCALAR-MERGE` (status:
active)"*. Both halves of that line are generated, and only one was true. The
**edge** was correct — Kernel really is blocked on that node. The **endpoint's
status** was stale: its accepted partial had merged, the Runtime ring had moved
to another node, and **nobody was building it.**

A reader of that line — including the operator, whose view of progress is this
file — concludes Kernel's unblock is in progress. It was on the shelf.

## Why the edge sweep cannot find this

A sibling node in the same graph records the *undeclared-edge* defect: a
dependency that existed only in prose, so `gen-progress.sh` showed a node
active with no blockers. The repair there was to declare both edges.

**This is the same failure direction reached by the opposite route.** Here the
edge is declared and correct, and the lie is one hop away in the node it points
at. A sweep that checks *"is every prose dependency also an edge?"* passes
cleanly over it, because nothing about the edge is wrong.

⇒ **Two independent checks, and neither implies the other:**

| check | question |
|---|---|
| edge currency | does every real dependency appear in `depends_on`? |
| **status currency** | **for every node whose status claims a team is on it, name the seat** |

## The status that is a claim about the world, not about the node

`merged`, `closed`, `draft`, `ready` are all properties of the **item**. Only
`active` is a claim about **someone else** — that a team is building it right
now. It is therefore the one status that goes stale without anything touching
the file, and the only one a seat read can falsify.

⇒ **Run `active` against the seats, not against the frames.** Grep the `active`
nodes, then read each owning ring's status. The measured hit rate the first time
this was run was **three of four**.

## What the legend can and cannot express

Of the three false `active`s, only one was safely correctable. Two were
*blocked* nodes with landed partials, and the legend has no started-but-paused
state: `ready` is false (deps unmet) and `draft` is false and lossier (they are
framed, with most deliverables merged). **Leaving those alone is correct** — the
Blockers section already states their truth, and forcing them into a wrong state
to satisfy a sweep trades one lie for another.

The one that flipped had `depends_on: []`, a shovel-ready frame, and no seat.
`ready` is exactly that, and the flip put it on the releasable frontier where it
belongs.

## How to apply

- **Do not read a status correction as a rollback**, and do not let a reader do
  so either. Lead the node's amendment with what is still landed — here, *"`c1`
  merged at `7bfc8ae5` and stays merged"* — before the status line.
- **Correcting the shelf is not re-ranking the lane.** Making a stalled node
  visible is what lets a priority be re-examined against real state; it does not
  pre-empt the operator's ordering, and saying so in the artifact keeps it from
  reading as a lane request.
- **A status you cannot correct without losing information is a signal about
  the legend, not a licence to force it.** Record why you left it.

**The inverse is already recorded**, and the pair is the whole picture:
[[an-atomic-sibling-node-needs-active-not-a-dependency-edge]] is in-flight work
reading `ready`; this is shelved work reading `active`. One puts a node nobody
may take onto the frontier; the other hides a node nobody is taking. Check both
directions in the same pass. See also
[[closing-a-node-strands-sequencing-prose-that-no-edge-check-sees]] for the
third member: an edge check that is clean and a claim that is still false.
