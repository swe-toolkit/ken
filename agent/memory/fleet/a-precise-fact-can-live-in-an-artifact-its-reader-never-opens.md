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

Related: [[a-requirement-in-an-advisory-section-is-never-discharged]],
[[a-mechanism-claim-in-a-comment-is-structurally-exempt-from-execution]],
[[a-stand-down-clause-lives-in-prose-where-no-gate-can-reach-it]].
