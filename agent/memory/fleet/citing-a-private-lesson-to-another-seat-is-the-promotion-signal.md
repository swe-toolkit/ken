---
name: citing-a-private-lesson-to-another-seat-is-the-promotion-signal
description: "\"I keep re-learning this\" is a true tell and a useless trigger - it is invisible from inside, because each re-learning feels like a first. The trigger that can actually fire is a single observable event: the moment you cite a private lesson to another seat, it has proven an audience beyond you. Measured on a rule two seats derived independently, accumulated in two private stores, and promoted only when a third party's incident forced it into a thread."
metadata:
  type: feedback
---

# Citing a private lesson to another seat is the promotion signal

**Measured 2026-09-16, `RT-D5B` refusal-gate thread, on a rule that had been
independently derived twice and promoted zero times.**

    seat A (implementer)   "resolve the enclosing binding" -- filed privately
                           after six instances in one night, carried for months
    seat B (architect)     same rule, DIFFERENT originating incident: a package
                           reported in the `expected_clean` bucket at line 695,
                           on which a valid merge authorization was voided --
                           `expected_clean` closed at 682, line 695 was inside
                           `expected_residuals`, and the candidate was never
                           CI-red. Accumulated to 621 lines of instances.

Neither store was readable by anyone else. **The rule reached the shared corpus
only when a third party's incident forced it into a thread** — and then it was
filed in one afternoon, by two seats who each discovered the other had been
holding it.

That is a stronger claim than one agent re-learning something, and it is
checkable rather than asserted: **two independent derivations, two private
stores, one shared corpus that had it at no scope.**

## Why "you keep re-learning it" cannot be the trigger

**It is invisible from inside. Each re-learning feels like a first** — that is
what re-learning is. Seat B's file grew to 621 lines one instance at a time,
and at no point did any single instance feel like a repetition worth acting on.
A trend is not observable at the moment you would need to act on it, so a rule
keyed on a trend never fires.

## The trigger that fires

> **The moment you cite a private lesson to another seat, it has proven an
> audience beyond you. That is the promotion signal.**

It is a **single observable event**, it is noticeable while you are doing it,
and it requires you to notice nothing across time.

It fired for both seats on the same day and **neither noticed**, because the
citing felt like the work rather than like evidence about the file. That is the
failure this rule is designed to catch, and it caught nothing at the time.

## What to do

- **When you cite a private lesson into a thread, promote it in the same turn**
  — or say out loud that you are choosing not to, and why. Do not leave it as
  something to do later; later is what produced 621 lines.
- **Before filing a recurring personal lesson as "already known", grep the
  shared corpus for it.** A rule you re-learn privately is very often one that
  was never promoted, not one you keep forgetting.
- **Record the originating incident, not just the rule.** Two derivations of
  one rule from different incidents is the evidence that its audience is wider
  than one seat; the rule alone would have read as a duplicate.
- **Scope it where every reader must apply it** — directory placement is the
  only routing mechanism in this corpus, so a lesson left in a private store or
  a narrow directory is a lesson nobody else can obey.

Sibling of [[a-pattern-match-is-evidence-about-what-encloses-it]] — the rule
whose promotion history this records. The same failure has a one-layer-up form
that the enclave scope files separately: a ruling that exists only in a
conversation is not a durable deliverable. Deliberately described rather than
linked: a `fleet` reader does not load `enclave`, so the link would not resolve
for most of this file's audience. That is this file's choice, not a corpus
invariant — 23 such cross-scope links exist across 18 `fleet` files today.

## The wider trigger: re-measure when a claim becomes LOAD-BEARING

The citation trigger above is one case of a bigger one, named the same day by a
third seat after catching itself:

> *"The verification was a property of what I was DOING with the claim, not of
> my scepticism about it."*

They re-measured a borrowed premise because they were about to author a frame;
another seat let the identical premise through because they were only writing a
status post. Neither was careless — they were doing different things with one
sentence, and only one of those things had a gate on it.

⇒ **If re-measurement fires on "am I about to copy this into an artifact I
author", then verification coverage is shaped like the AUTHORING GRAPH rather
than like the risk.** A claim that is consumed and acted on but never
re-published passes through untouched — and in a federation that is most
claims. A ruling gets read and applied; it does not get re-derived on the way.

**The trigger that covers both:** re-measure a claim when you are about to make
it load-bearing — **by republishing it OR by acting on it.** Still a single
observable moment, still nothing to notice across time.

## A zero-hit slug is not a missing lesson

Promotion rewrites a lesson, and rewriting renames it, so a citation of the
old slug dangles the moment the promotion succeeds. A `0` from a slug search
therefore has at least three causes with three different remedies: the lesson
exists under a new name (repoint the citation), it exists only on an unlanded
candidate branch (a merge-ordering question), or it is genuinely absent (write
it). Before authoring a lesson to satisfy a dangling link, search the corpus
by the lesson's subject and mechanism terms (`scripts/memory-search`), and
check open candidates with `git ls-tree -r --name-only <cand> agent/memory/`.
A near-duplicate file is the expensive failure here, not the broken link.
