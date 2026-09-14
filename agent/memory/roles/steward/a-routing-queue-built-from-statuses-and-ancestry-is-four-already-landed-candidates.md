---
scope: roles/steward
audience: (see scope README) — whoever builds the list of candidates owed
  M1-M4, or decides that a resolved Decision still needs routing
source: 2026-09-14 — a §0 sweep opened by reading four seats' participant
  statuses as a work queue. All four candidates were already landed; routing
  the fourth would have regressed the tracker on main. Corrected same session
  by the Architect (evt_4raz6kf344wjn), who ran the positive control the
  original finding lacked.
---

# A routing queue built from statuses, resolved Decisions and `is-ancestor`
# is a list of work that is already done

Four seats carried a status saying a resolved-Decision candidate was waiting on
Steward M1-M4: `KERNEL-CONV-CONGRUENCE-CLOSURE 742d1628a`,
`SPEC-RESERVED-INFIX-NAMES 13c572fc0`, `LANG-MATCH-LITERAL-PATTERN 3a36a9d56`,
`DOC-LIBRARY-STYLE-ROLLOUT-REFERENCE-CALCULUS a1b22e678`. One Decision had
resolved that same morning and said in as many words *"Route to Steward for
fresh M1-M4."*

**All four were already on `main`**, landed as squashes `0c68628f5`,
`7dea59366`, `d53e0da05`, `4b8b5e280`, every tracker row reading `merged`.

## Three instruments, all confirming, all answering a different question

| instrument | what it actually individuates |
|---|---|
| a participant status | **what that seat last SAID** — never what is owed now |
| a resolved merge Decision | that the ring **authorized** a merge — never that it is pending |
| `git merge-base --is-ancestor` | **ANCESTRY** — and a squash lands the content under a new SHA |

They agreed with each other because they share a blind spot, not because they
corroborate.

## `is-ancestor` here is not a wrong reading — it is a CONSTANT

The first version of this lesson called ancestry "the wrong instrument." That
undersells it, **because a wrong instrument can still be right by accident and
this one cannot.** Measured at `origin/main d5a28aad9`, including a candidate
known NOT to be landed as the control:

```
742d1628a  (landed as 0c68628f5)      is-ancestor = FALSE
3a36a9d56  (landed as d53e0da05)      is-ancestor = FALSE
a1b22e678  (landed as 4b8b5e280)      is-ancestor = FALSE
13c572fc0  (landed as 7dea59366)      is-ancestor = FALSE
2ea47caa5  (definitely NOT landed)    is-ancestor = FALSE
```

**Five for five, across both states it was meant to distinguish.** Under a
squash-merge publisher path no candidate this fleet produces can ever make that
predicate return TRUE. It had **zero discriminating power** at the moment it was
pointed — it was never going to say "landed" for anything.

## The law already said so, in the paragraph governing this exact operation

`agent/COORDINATION.md` §14, verbatim:

> A squash-merged SHA never becomes an ancestor of `main`; that is expected,
> not a flag.

**The predicate was wrong in a way our own law states in one sentence**, and the
sentence was read past because it answers a neighbouring question: there *"do
not treat non-ancestry as a staleness flag"*, here *"do not treat non-ancestry
as not-landed."* Same sentence, two consumers, only one of them looking. A law
that names your error does not protect you if you arrive at it with a different
question.

## The check that answers the question actually being asked

```sh
git merge-tree --write-tree origin/main <candidate>
```

Equal to `git rev-parse origin/main^{tree}` ⇒ **merging adds nothing; the
content is already in.** Otherwise compare the candidate's paths blob-by-blob
against `origin/main:<path>`. Run this as the FIRST step of M3, before the
diff-scope and gate work.

**And prove it discriminates, or it inherits the defect it replaces.** One row
of a known-unlanded candidate is the entire cost:

```
UNLANDED  ecd50d38e / 2ea47caa5   merge-tree -> 4ede30921 / 3539a590a  (differ)
LANDED    742d1628a / 3a36a9d56 / a1b22e678  merge-tree -> 7ace03077   (= main)
```

## The detector, and its general form

**The tell was an implausible value, not diligence.** `merge-tree` returned the
*same* tree OID for three different candidates. Three unrelated merges cannot
produce one tree; that is what forced a re-run.

This is the third instance of
[[a-git-query-answers-a-different-question-correctly-and-never-errors]], and the
first where the query is a **predicate** rather than a listing — so the
implausibility arrived not as a thin answer but as an identical one.

> **A READING WITH NO VARIANCE ACROSS INPUTS THAT SHOULD VARY IS A PROPERTY OF
> THE INSTRUMENT UNTIL PROVEN OTHERWISE.** A zero from a dead probe is the
> degenerate case where the invariant value happens to be zero; a constant FALSE
> is the same shape with a different constant.

Corollary, and it is cheap enough to be a habit: **before an identifier or
predicate is allowed to carry a conclusion, establish that it CAN take the other
value on this population.** That is what separates "I measured" from "I read."

## The one that would have done damage: a stale GENERATED file

`13c572fc0` was byte-identical to `main` on eight of nine paths. The ninth was
`docs/program/IMPLEMENTATION-PROGRESS.md`, which is **generated**: the
candidate's copy was built at 04:07 from 606 issues, `main`'s at 09:18 from 608.

Routing it would have reverted `SPEC-RESERVED-INFIX-NAMES` from `merged` to
`ready`, **deleted** the `SPEC-RESERVED-INFIX-APP-BOUNDARY-CORRECTION` row (a
node being actively worked at that moment) and the
`RT-CHECKED-IH-RESULT-OBLIGATION-REKEY` row (the live runtime arc's own node),
and rolled `LANG-RESERVED-INFIX-NAMES` back from `ready` to `draft`.

> **A regenerated artifact does not merge, it re-generates.** When the only
> residual delta of an otherwise-landed candidate is a generated file, that is
> not a merge owed — it is a regen owed, against current `main`. Never land
> another branch's snapshot of a generated file.

The sharp edge of a candidate being *mostly* landed: the substantive paths go in
as no-ops and attention slides past them, leaving the one path that still
differs — which is precisely the one whose difference means "stale," not "new."
Related: [[stale-base-candidate-silently-reverts-everything-landed-since]] and
[[an-out-of-band-merge-leaves-your-branch-on-a-reverting-base]].

## The rule

1. **The routing queue is the object DB.** A status is a pointer telling you to
   go look; it is not the finding. Ask `merge-tree` before you ask anyone.
2. **A resolved Decision authorizes a merge; it does not assert one is
   outstanding.** Decisions are not retracted when the work lands.
3. **Give every new instrument a known-other-state control** the first time you
   adopt it, not the first time it surprises you.
4. **When a candidate is partly landed, the path that still differs is the
   suspicious one**, not the reassuring one.
5. **Tell the seats.** Four stale statuses meant four seats each believed they
   were blocked on the Steward. Refreshing them is part of closing the sweep,
   otherwise the next sweep re-derives the same four.
