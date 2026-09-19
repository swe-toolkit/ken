---
scope: roles/steward
audience: (see scope README)
source: 2026-08-13 — kicked Language onto `LANG-RECORD-STACK-OVERFLOW`, which
  had merged as PR #2098 at `b4d38b8a` hours earlier and still read
  `status: ready`. The language leader blocked on a node-identity question
  rather than assigning it.
---

# A node's `status:` is a claim about the node, not evidence about the tree

Nothing flips a node to `merged` when its PR lands. **A human or an agent has to
do it, and on a busy day nobody does.** So `status: ready` means "nobody has
updated this field since it was written", which is **equally consistent with
never-started and with shipped-and-forgotten**.

I read `ready` on four Language nodes, picked the one with the strongest merit
argument, and kicked it. PR **#2098** was titled
`LANG-RECORD-STACK-OVERFLOW: record literals, and the dispatch-frame repair`
and had been **MERGED** at `b4d38b8a` — ten files, `+581/-2`. Two nodes'
deliverables were in it. Neither had been flipped.

## The compounding is the expensive part, not the wasted kick

The kick did not just name the wrong node. **Every claim I derived from the
frame went stale with it**, and each one read as freshly measured:

- I warned about a depth fixture that builds match arms with `=>`, which is not
  a Ken token, so it dies in the lexer before reaching depth one. **The same PR
  had already rebuilt that fixture to use `|->`.**
- I added a `blocks:` edge and a `depends_on:` edge sequencing another node
  behind it — both derived from the belief that the work was pending.
- I wrote a "the seat cannot work this node" correction into the frame, for a
  node that no longer needed a seat.

⇒ **A stale status is not one wrong fact. It is the root of every fact a frame
derives, so one unchecked field can make an entire kick wrong while each line
in it looks independently sourced.**

## The check, and it is two seconds

Before any kick, ask whether the work already landed — **do not ask the node**:

```
gh pr list --state merged --limit 120 --json number,title --jq '.[] | "\(.number) \(.title)"' | grep -F "<NODE-ID>"
```

or grep the mechanism itself on `origin/main`. For the node I released instead
(`LANG-SURFACE-LITERAL-ESCAPES`) the check was
`git grep 'escape' origin/main -- crates/ken-elaborator/src/lexer.rs` returning
**nothing** — that is evidence about the tree, and it is what a kick should rest
on.

**Sweeping the whole corpus for this costs one `gh` call**, and it is worth
doing once rather than per-kick: cross-check every non-merged node's `id:`
against merged PR titles. Done 2026-08-13 — the corpus was otherwise clean, so
this failure was isolated rather than systemic. Two apparent hits were both
correct: a multi-PR arc legitimately sits at `active` while partials land, and
a PR that **authorizes** a node does not deliver it.

## What the leader did, and it is the behaviour to protect

The language leader did not assign it. They blocked and asked me to identify
the node, because the kick disagreed with the tree they could see. **That cost
one round trip instead of an implementer turn.** A ring that challenges a
Steward kick when it contradicts the repository is working correctly, and the
kick is what should be doubted — I have the summary, they have the tree.

## 2026-09-19: THE SECOND FAILURE MODE, AND THE CHECK ABOVE IS BLIND TO IT

The case above is a node that **fully merged**. The harder one is a node
**partially executed**, and it defeats this file's own remedy.

`RT-COMPMATCH-TREE-SCRUTINEE` read `status: ready`. I sequenced it first on the
strength of its frame's stated shape — *"a single forcing step with a named
target and a binary outcome, NOT a peeling ladder."* The runtime leader stopped
the dispatch. Verified at the object:

    03976d2ac  ANCESTOR of origin/main, 2026-09-18T22:42:18Z
               "RT-COMPMATCH-TREE-SCRUTINEE D1+D2: find the consumer
                behind worker_return"
    touched    2 crates/ files
    did NOT touch  the node file or its wp frame

**`status: ready` was not even wrong here.** Work genuinely remained, so no
field needed flipping and nothing looks stale on inspection. The rot is in the
**body**: the node still presents its pre-work three-layer analysis as current,
with no record of D0/D1/D2's landed outcome or the thread's `D3: witness
RETIRED` verdict.

> **A node can be simultaneously correct in its status and false in its
> content.** The status field answers *"is this node finished?"* Nobody asks the
> field that decides a dispatch: *"is what this node SAYS still true?"*

### The check in this file does not fire

`gh pr list --state merged | grep -F "<NODE-ID>"` finds **nothing** — there is
no merged PR, because the work landed as one commit inside an arc that
continues. **The remedy written above is sound for the merged case and blind to
this one**, which is the more common one on a live lane.

The predicate that catches both, handed to the runtime ring to apply at the
point of use rather than as a tracker sweep:

```sh
git log --oneline origin/main --grep='<NODE-ID>'
```

**Commits the node file does not record ⇒ the node is stale, and the write-back
precedes the work.** It costs one command per node start, it needs no `gh`
credential, and it catches partial execution, which merged-PR matching cannot.

### What it cost, and it is the sizing, not the kick

The wasted dispatch was never the risk — the leader caught it. The damage was
that **I sized and ordered the lane off a property that had already been
consumed.** The binary-outcome shape was my entire reason for putting it ahead
of a node covering three times as many rows; that forcing step fired the day
before. ⇒ **When a premise is refuted, re-make the decision rather than
defending the conclusion** — a conclusion that survives its broken premise is
the one nobody re-examines. I reversed the order.

And the corollary for any estimate: **you cannot size what the node
misrepresents.** Separate the write-back from the work it unblocks, and size the
work only after the artifact tells the truth. Here the write-back is T2
transcription and does not need the T1 seat's post-compaction turn at all.

See also [[a-node-status-can-be-right-about-the-edge-and-wrong-about-the-seat]],
which is the sibling failure: that one is status-versus-**seat** (nobody is
building it), this one is status-versus-**tree** (it is already built).
And [[an-instruction-to-close-is-not-evidence-the-work-behind-it-is-done]] is
the same gap read from the other direction.
