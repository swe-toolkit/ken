---
name: a-prose-repair-changes-clauses-beyond-the-one-it-was-scoped-to
description: A repair to one clause of a message or comment is authored at the grain of the sentence or the artifact, not the phrase, so it silently deletes neighbouring clauses the finding never implicated or re-adds a fact an earlier ruling deliberately removed. Review a prose repair against what shared its sentence and against what earlier rulings excluded, not only against what it claims to fix.
metadata:
  type: feedback
---

# A prose repair changes clauses beyond the one it was scoped to

Two instances in one day (2026-08-16), in opposite directions. In both the
repair did what it was asked, and the damage was to a clause nobody asked about.

| instance | collateral | why it is invisible |
|---|---|---|
| `ec2b4a1eb..e23a18aee`, a `+6/-8` string-only diff | two clauses **deleted** from a refusal message | the whole line changed, so the diff shows no separate removal |
| `63644c71d..f2f20703c`, a `+6/-3` comment-only cut | an excluded measurement **re-added** to a comment | the earlier exclusion is not in the diff, the comment or the node |

## Deletion: a wording fix scoped to a sentence takes the clauses that share it

A refusal was reworded because a prior finding refuted **transfer**: the message
had implied a later consumption might discharge an earlier obligation. **The
rewording works**: scoping to *"this recognition's own"* and *"never"* closes
the reading.

⇒ **But the edit replaced the whole sentence, and only one phrase was
implicated.** Gone with it: the phrase naming **where** a consumer would have to
be, and the final clause giving the **consequence**: *"denotes a value
containing the callable and has no runtime representation."* The result states
its condition twice and its consequence never; two of three surviving clauses
say *not consumed*, and none says why that is fatal.

⇒ ***When a finding implicates a PHRASE, check what else lives in the sentence
the author rewrote.*** A correction is scoped to the smallest thing the author
can edit, which is usually the sentence, not the phrase, so the collateral is
whatever shared the sentence.

**Check whether the string is printed before ranking it.**

```rust
impl fmt::Display for UnsupportedLowering {
    write!(f, "{}: {}", self.construct, self.reason)
```

`reason` reaches the user verbatim, so this is a compile diagnostic, not an
internal label. ***One grep for the `Display` impl decides whether a message
finding is cosmetic or a regression in the product.*** Do it before choosing how
hard to push.

⇒ ***A refusal that describes only the state it is in gives the reader nothing
to do.*** The consequence clause is what makes it a diagnosis rather than a
report, and it is the clause most likely to be dropped, because it reads as
commentary.

**Offer the restoration, not just the loss.** The finding wrote the replacement
sentence out: it keeps the scoping that closed the misreading, keeps the deleted
temporal phrase deleted, and restores the site and the consequence. **On a
wording finding, a proposed sentence is the whole deliverable**; otherwise the
author must re-derive the constraint set you already worked out, and the
cheapest path back is reverting to the text the finding rejected.

## Reinstatement: a second repair does not inherit the first ruling's exclusion

Three edits, one comment, three nodes:

1. A reviewer **excluded** a per-test measurement from the comment: those
   returns precede the crossing that would populate the field, so folding them
   in would *"raise the tally while weakening what it means."*
2. A finding bounded what that measurement proves: the agreement is **forced,
   not observed**. Accepted, and **recorded in the node**.
3. A later node, repairing an unrelated clause, **added the measurement to the
   comment**, without the bound.

⇒ ***The correction lives in the node; the claim lives where readers land.*** A
deliberately-excluded fact gets back in one repair later with nobody having
decided to reinstate it: the second author is fixing a different sentence and
has no reason to know the first ruling exists.

⇒ ***When a repair lands on an artifact you have already hunted, diff it against
what earlier rulings REMOVED, not only against what it claims to add.***

**Sequence is not implication.** The added text says the empty observations came
*"before its expected refusal."* That states the order and not the cause: a
reader learns the refusal came after, not that the refusal is **why** the set
was empty. The clause that makes the sentence honest is the one carrying the
implication, and prose naturally supplies the sequence instead, because the
sequence is what the run looked like. **Check where such a sentence sits**: here
it landed one sentence above a much larger tally, so eighteen
structurally-determined observations now read as neighbours of eighty-one
independent ones.

**The placement principle runs both ways.** The same reviewer had put one
earlier observation in a **merged, closed** node rather than a successor,
because that is where the next reader of the affected criterion lands. Correct,
and the same test says the bound here belongs in the comment, not the node file.
***Ask where the reader of THIS sentence will be standing***, every time; a
correction filed at the right altitude for one artifact is at the wrong one for
another.

## How to apply

- For any prose repair, list the clauses of every sentence it rewrote and check
  each survived or was meant to go.
- Keep a list of what earlier rulings removed from an artifact you hunt, and
  diff each later repair against it.
- On a message finding, check the `Display` path first, then propose the full
  replacement sentence.

Related:
[[a-proposed-sentence-lands-verbatim-and-carries-its-defects-into-durable-text]]
and [[the-operative-artifact-must-carry-the-claim-whichever-pass-wrote-it]].
