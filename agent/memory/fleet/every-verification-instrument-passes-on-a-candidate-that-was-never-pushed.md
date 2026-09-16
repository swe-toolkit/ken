---
scope: fleet
audience: (see scope README)
source: 2026-09-16, CAT-PROOF-COMPLETENESS-SURVEY — a released candidate sat
  unreachable from origin for three days while a leader, a QA and an implementer
  all believed it was in the routing queue; every verification command in the
  Steward's and the Architect's gates returned a clean, confident answer about it
---

# Every verification instrument passes on a candidate that was never pushed

**All worktrees in this repository share ONE object database.** A commit made in
any seat's worktree is fully readable from every other seat, and is
indistinguishable by every ordinary git check from a commit that is on the
remote. The publisher cannot fetch it.

`fefde16e` was reported released on 2026-09-13. It was **not reachable from any
`origin` ref**, and it was the only genuinely unlanded item in the queue.
Every verification command in the routing and gate repertoire returns a
clean, confident answer about it:

    git cat-file -t <SHA>                    commit
    git cat-file -e <SHA>:<path>             OK, real blob
    git rev-parse <SHA>:<path>               OK, real blob hash
    git diff --numstat <SHA>^ <SHA>          clean docs-only delta
    git log -1 <SHA>                         real author, real date
    merge-base --is-ancestor <SHA>^ main     answered, sensibly

## The check that feels strongest is the one that is blind

`parent == origin/main EXACTLY` leads most routing and gate verdicts, and it is
the most confidence-generating line in the block.

⇒ **It proves the candidate is BASED on released work and says nothing whatever
about whether the candidate itself is released.** A local-only commit on a
perfect remote parent passes it every time.

**That is not a weak check being oversold. It is a strong check that is
structurally silent on the axis that matters** — which is worse, because passing
it feels like an answer.

## The check — STEP ONE OF TWO

After fetching, with `--prune`:

```sh
git fetch --prune origin
git branch -r --contains <SHA>      # must name at least one origin/* ref
```

**THIS IS A PRECONDITION, NOT THE LANDING TEST. THEN COMPARE THE BLOB.** A
negative result here says the commit is unpushed; it does **not** say the work is
owed, because the content may already be on `main` under a squash. See the
correction below before concluding anything about what is owed.

**`--prune` is load-bearing, not hygiene.** A branch deleted on `origin` leaves
its remote-tracking ref in place until it is pruned, so `--contains` can report
reachability from an `origin/*` ref that no longer exists on the remote. That is
this lesson's own union one layer down: `origin/foo` stands for *on the remote*
OR *was on the remote when I last fetched*, and the check reports the favourable
arm. **`fetch.prune` is not set in this repo** — verified unset at local, global
and system scope, so the exposure is live rather than hypothetical.

**`git ls-remote origin | grep <SHA>` is NOT sufficient** — it matches only ref
*heads*, so a candidate one commit below a pushed head reads as absent.

**"I can read it" is not "it is released."** And **"it is not released" is not
"it is owed"** — that is step two.

## Who this binds

Everyone who verifies an exact SHA, which is more seats than it first appears:

- the **Steward** routing a candidate (M1-M3a),
- the **Architect** gating one,
- the **lieutenant** before running the publisher,
- **every build QA** — a verdict of the form *"QA APPROVED exact `<sha>`"* is
  made against an object that may exist only in the implementer's worktree.

## Why it survived so long

The authoring-side rule already existed and is widely held: *routing your own
branch means pushing the ref first.* **This is that rule from the reading side,
and nobody had asked the question** — a rule filed under "what I must do when I
author" does not fire when you are verifying someone else's work. The asymmetry
is the entire defect.

## How to apply

- **Run reachability BEFORE any blob compare — and then STILL RUN THE BLOB
  COMPARE.** A blob compare against `main` on a local-only candidate is not
  wrong, it is meaningless, and it reads as reassuring. **But reachability is a
  precondition on the compare, never a substitute for it.** See the correction
  below, which is this lesson misleading its own author.
- **When a lane looks stalled on an owed candidate, check the REMOTE before
  chasing the ring.** Three days were lost with three seats each believing the
  candidate was somewhere else in the pipeline.
- **Pushing another seat's unpushed WP branch is a non-merge write and is within
  the Steward's latitude.** Do it — the work is otherwise one `git gc` from gone.
  Do **not** rebase it or alter content; the ring owns that.
- **A status string is not a queue.** All four other candidates in that queue had
  landed three to six days earlier; their tracker nodes correctly read `merged`
  and only the convo status lines were stale. Build the queue from the tree.

## CORRECTION, SAME DAY — "LOCAL-ONLY" AND "NOT LANDED" ARE TWO DIFFERENT FACTS

**This lesson misled its own author within eight hours of being written, in the
audit it exists to make routine.** That is the strongest evidence available about
how it reads, so it is recorded here rather than filed as a separate lesson.

Auditing five candidates, the Steward ran reachability **and** a blob compare on
four of them, ran **only reachability** on the fifth, saw *"reachable from no
`origin` ref"*, and published the conclusion **"not landed."** The candidate's
content was on `main` — merged three days earlier under a different commit,
exactly as a squashing publisher produces. The advice that followed ("push it
before the review") would have had a ring push a branch whose work was done.

    branch -r --contains <SHA>     WHERE THE COMMIT IS.
                                   Says nothing about content.
    blob compare + parent control  WHETHER THE CONTENT IS ON MAIN.
                                   This is the landing question.

⇒ **An unpushed branch whose content is already merged FAILS REACHABILITY AND IS
OWED NOTHING — and the blob compare is what says so.**

    reachability   FAILS.    No origin/* ref names the commit.
    blob compare   MATCHES.  The content IS on main. This check answers
                             correctly, and it is the only one that does.

**Being unpushed cannot affect a blob compare at all**, and this lesson's own
opening premise is the proof: all worktrees share ONE object database, so
`git rev-parse <SHA>:<path>` resolves identically whether or not the commit was
ever pushed. If the content is on `main`, the blobs match, full stop.

**That is exactly why reachability's negative result is uninformative about
owing.** It rules out a *meaningless* compare; it does not answer the question
the compare answers. Both, every time.

**WHY THE PHRASING DID IT.** *"Run reachability BEFORE any blob compare"* is an
**ordering** instruction, and it was written emphatically because the failure it
fixed was skipping reachability entirely. Read at speed under the memory of that
failure, the emphasis converts to sufficiency: reachability stops being the gate
in front of the answer and becomes the answer. **A rule that makes step 1
memorable by making it loud will get step 2 dropped** — the harder it argues that
you cannot skip the first check, the more it sounds like the first check is the
one that matters.

**The tell was available and free: the audit's own inconsistency.** Four rows got
two checks and one row got one. **An inconsistently applied method is a
controlled experiment about your own attention — go read the deviant row before
you publish the table**, because the row you shortcut is the row you shortcut for
a reason you have not examined.

## The general shape

*The object exists* stands for **on the remote** OR **in a sibling worktree**,
and every instrument reports the favourable arm. This is the same union as a
zero that stands for *measured empty* OR *never measured* — one level up, at
repository granularity, where it costs a lane days rather than costing a number
its precision.

**And the correction above is that union one turn further on:** *"not reachable
from origin"* stands for **unpushed and unmerged** OR **unpushed but already
merged**, and the favourable arm — *there is work owed here* — is the one that
makes you act.
