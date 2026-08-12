---
name: the-operative-artifact-must-carry-the-claim-whichever-pass-wrote-it
description: The pass asymmetry runs in both directions — a loose summary in operative text fails toward a confusing red, a precise obligation stranded in prose while the operative row stays silent fails toward GREEN, and only the second is invisible
---

# The operative artifact must carry the claim, whichever pass wrote it

**Measured 2026-08-10 on `edae6e4c` (`RT-MATCH-RECURSOR-CONSUMERS` `D10`),
completing a shape I had seen twice from one side only.**

I had been filing this as: *the precise sentence and the loose summary are
written in different passes, and only the precise one gets proofread as a
claim* — with the loose one landing in the **operative** text (an assertion
message, a signature's doc) and the precise one safely in prose.

**This instance is the mirror, and it is the worse direction.**

A code comment's release condition was correctly delegated to a tracked node:
*"tracked at `KERNEL-NESTED-IND` `AC-K12`, consult it there."* The pointer
resolves and the node genuinely owns the condition. But the obligation —
*"do not report `AC-K12` green while the carried control is still
`#[ignore]`d"* — sits in a prose block **142 lines below** the `AC-K12` row,
with no forward reference, and the row itself is silent. `AC-K12` is the AC
table's **last** row, so a reader scanning the criteria stops exactly there.

⇒ **A discharge check reads the operative row.** The prose block even states its
own purpose — *"the condition belongs in a tracked node that a status check can
see"* — and a status check sees the table.

## Why this direction is the dangerous one

| where the claim is | where it is missing | failure direction |
|---|---|---|
| loose summary in the **operative** text | precision in prose | a confusing **red** — someone reads the message, goes looking, is annoyed |
| precise obligation in **prose** | operative row silent | a clean **green** — nobody looks, nothing fires, the AC discharges |

**Only the first generates evidence.** The second is a
[[hunt-the-stand-down-clause-it-lives-in-prose-no-gate-reads]] with no author:
nobody wrote *"don't look"*; the operative artifact simply never mentioned there
was anything to look at.

⇒ **The rule that covers both: the operative artifact carries the claim.** The
row, the assertion message, the AC cell, the acceptance criterion — whichever
artifact a checker or a reviewer consults *to decide*. Prose beside it is
commentary, and commentary is not consulted at decision time.

## Hunt it at the DELEGATION seam

The high-yield moment is when something stops stating a condition locally and
starts pointing elsewhere — exactly what this deliverable did, correctly, and
for a reason I had argued for. **Deleting the local statement is right; it is
also the moment the whole obligation comes to rest on one hop.** So:

1. follow the pointer and confirm it **resolves** (the `program_authority`
   class — [[a-type-narrowing-discharges-one-clause-and-a-producer-enumeration-carries-the-rest]]);
2. then confirm it **answers** — the target says the thing, not merely mentions
   the topic;
3. then confirm it answers **in the artifact the reader will consult**, not
   somewhere else in the same file.

I would have stopped at (2) and reported clean. Step 3 is the one this taught.

## Time the finding to the window where it fires

The blocker on that AC cleared **the same day**, so the discharge check was
imminent and the repair — one table cell — was worth naming then rather than
generally. **A preventive finding's value is concentrated in a window**; say
which window and why it is open now, or it reads as an abstract tidiness point
and gets weighed against
[[preventive-findings-are-unfalsifiable-so-keep-them-cheap]] with nothing on the
other side.

## An empty per-path `git show` means you are on one commit of a multi-commit tip

**The near-miss that got me to the finding at all.** The notification gave an
exact SHA and two declared paths. `git show <sha> -- <path>` returned **nothing**
for one of them.

**That reads as "this deliverable did not touch its declared path" and it is
not.** The SHA was a **branch tip** carrying `D9` and `D10` as separate commits;
`git show` displays one commit, so the other deliverable's whole diff was
invisible. `git diff <declared-base> <tip> -- <path>` showed it immediately.

⇒ **Empty output on a path the handoff declares is a signal about your RANGE,
not about the change.** The declared path count is the cross-check: if a
notification declares two paths and your command shows one, the instrument is
wrong. Diff from the **declared base**, never `<sha>^ <sha>` and never a bare
`git show`, whenever the tip may carry more than one commit.

**The failure direction is the bad one for this seat**: I would have reported on
one deliverable, said nothing about the other, and the silence would have read
as *hunted and clean* rather than *never looked*. Same family as
[[a-probe-truncated-before-the-grep-is-not-a-measurement]] — the pipeline lied,
not the grep.

## MY OWN BOUNDS STATEMENTS HAD THIS DEFECT

**Named by the Steward, 2026-08-11:** *"A merge with a clean adversary pass
reads as swept unless the unswept surface is written down."* He routed my
declared-unhunted list — the traversal, the cycle handling, the node-admission
predicate, the identity resolution — **into the node's record** rather than
leaving it in the thread.

⇒ **That is the defect I keep filing, in my own output.** A bounds statement in
a channel post is prose no gate reads; the durable artifact carries only the
verdict, and **the verdict is "no finding."** So a pass that swept one axis of a
four-axis merge lands in the record as a clean pass — the exact
operative-artifact-silent-while-the-precision-sits-in-prose shape, with me as
the author.

**The routing fix is the Steward's; the discipline is mine.** When I declare an
axis unhunted, that declaration is only worth what reaches an artifact. ⇒ **Say
explicitly, in the report, that the unhunted list belongs in the node record** —
do not rely on it being harvested. A reader six merges later sees the node, not
the thread.

## A lesson can graduate from your heuristic to the author's checklist

The standing question — *which distinction is this control set supposed to
survive* — moved upstream: it is now applied when **authoring** ACs and
controls, not only when hunting them. The consequence to expect, and not to
misread: **when it still fires on a pass, the frame failed rather than the
ring**, and my hit-rate on that class should fall.

⇒ **A falling find-rate on a class you promoted is the success condition, not a
capability loss.** Record which classes have graduated, or you will keep
scoring yourself against a population that was deliberately drained.

## Report the two things you could not break

Both deliverables repaired findings of mine, so I checked the two ways they
could have been hollow: whether the reader injection was a test-only seam (it is
not — the real assertion and the synthetic control go through **one** function),
and whether the redirect dangled (it does not). **Say both.** A report on your
own repaired finding that lists only a new defect reads as never satisfied.

## THE TWO-QUESTION TEST, and it is better than my statements of this

**Steward, 2026-08-12, closing the Row 2 arc.** A sentence in a durable artifact
has to survive two questions, one for each side:

1. **Reader's:** *would this still be right if the reader had none of the context
   I have?*
2. **Author's:** *if the reader wanted to check this, could they get to the
   evidence from here?*

**Citation passes both. Restatement passes neither. Deletion passes the first
and fails the second** — which is why removing a wrong claim is weaker than
replacing it with one that carries its ground.

⇒ That is the whole operative-artifact family in two lines, and it explains the
failures in this file better than the file did: a conclusion written as fixed
input **passes (1) and fails (2)**, so the next reader inherits something correct
and uncheckable, and cannot tell it from something incorrect and uncheckable.

## Do not absorb an error you inherited from an operative artifact

Same message, and it is aimed at a seat logging three defects when one was its
own: **"a seat that logs three of its own defects when one is its own
miscalibrates in the expensive direction — it starts distrusting reads that were
sound."**

⇒ **Attribute your own corrections as carefully as you attribute findings.** An
error inherited from published operative text is the artifact's, and taking it
personally trains you off correct instincts. ⚠ The distinction is sharp and
checkable: *did I derive this, or did I read it somewhere durable and carry it?*
Deriving it wrong is mine; carrying a published wrong ground is the artifact's,
and saying so is not deflection — **it is the same accuracy I demand of a
handback.**
