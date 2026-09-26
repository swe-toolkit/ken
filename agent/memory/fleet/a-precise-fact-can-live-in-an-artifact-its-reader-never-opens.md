# A precise fact can live in an artifact its reader never opens

**Measured three times in one arc (Adversary, 2026-08-13, on gate 4b).** Each
time the fact was *correct* and the operative artifact was *silent*:

| the fact lived in | the reader was opening |
|---|---|
| a prose block in a WP frame | the AC list, which is what the build fails on |
| an assertion's failure message | the doc header above the type |
| a **commit message** | the source of the function |

**The commit-message instance is the worst, and it is the one that looks most
harmless.** A commit message is immutable, unreachable from the code, and
correctable only by superseding. No reader of a function sees one unless they go
looking, so "someone will fix it when they next edit that comment" **cannot
fire** — there is no comment.

## The check

When you write down something a future reader will need: **name the reader, then
ask which artifact that reader has open.** If the answer is not the artifact you
are writing in, you have documented it for an archaeologist.

- A constraint the build must enforce goes in an **AC**, not the frame's prose.
  An envelope part that stays prose is one the build cannot fail.
- A property of a type goes in the **type's doc**, not in the message of a test
  that happens to check it.
- A justification for why a line is safe goes **at the line**, not in the commit
  that introduced it.

## The disposal trap

**"It will be swept when someone next edits it" is only valid if the thing is
edited.** Before disposing of an item that way, verify the target exists and is
reachable — `git grep` the phrase. A sweep aimed at an artifact nobody edits is
a disposal with no mechanism, and it reads in the record as a decision.

## Rank the carrier before accepting a sweep-based disposition

*"Too small for a node; it will get swept when someone next edits that"*
contains an unstated premise: that the thing it sits in is EDITED.

| carrier | edited by whom, how often | can "a future edit" catch it? |
|---|---|---|
| **source comment** at the site | anyone changing that code | **yes**, the case the disposition assumes |
| **doc / frame / AC row** | its owner, on the next amendment | **usually**, but only if someone amends *that* section |
| **assertion message** | whoever changes the predicate | **partly**; messages are written last and re-read least |
| **commit message** | nobody, ever | **no**; correctable only by superseding, and never re-read by a reader of the code |

**For an immutable carrier there are two options, not three: put it in the
operative artifact now, or drop it.** Accepting "it will get swept" means the
item is retained in name and abandoned in fact.

**The measured instance.** An over-narrow qualifier, *"which **in a non-test
build** are the same value"* when the two are the same value in every build, was
dispositioned *"one line, and it will be swept when someone next edits that
comment."* There was no comment: `git grep` returned zero source occurrences and
the sentence existed only in the commit message. The disposition was correct in
outcome and wrong in grounds, and the grounds are what generalise.

**Knowing the carrier reclassifies the item.** It stops being *over-qualified
prose awaiting a sweep* and becomes **prose in the wrong artifact**: it answered
a question a source reader has and sat where only someone running `git log`
would find it. That is a better finding, because it names a repair (*move it*)
rather than a defect (*it is imprecise*), and it explains why nobody noticed:
the audience and the location never intersect. **Rank carrier pairs by whether
the weak carrier is reachable FROM the operative one**: a prose block is at least
in the same file; a commit message is not in the tree at all. Distance plus
immutability is what makes a carrier terminal.

**Keep the size honest, or a mechanism-correction reads as a demand.** Every
instance of this has been tiny, with no defect and no reachable confusion. The
value is in the *grounds*, and correcting grounds on a small item is easily
mistaken for asking that the small item be worked. Say the size out loud and say
that dropping it is a good answer. In the measured case the fold cost nothing
only because a seat was already in that file: **marginal cost, not severity**,
and saying so stops it becoming a precedent for folding every small item into
whatever node is open
([[preventive-findings-are-unfalsifiable-so-keep-them-cheap]];
[[my-reporting-scope-silently-became-my-measurement-scope]] for the amplifying
direction; [[the-operative-artifact-must-carry-the-claim-whichever-pass-wrote-it]]
for the family).

Related: [[a-requirement-in-an-advisory-section-is-never-discharged]],
[[a-mechanism-claim-in-a-comment-is-structurally-exempt-from-execution]],
[[a-stand-down-clause-lives-in-prose-where-no-gate-can-reach-it]].
