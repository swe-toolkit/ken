---
scope: fleet
audience: (see scope README)
source: 2026-09-17, spec-author — two instances in one night, in two different
  artifact classes; drafted by spec-author, routed via spec-leader, placed by
  the Steward
---

# Text whose truth has an expiry sits in an artifact with no alarm

A claim like *"rejects X until D1"*, or a ruling later superseded, is **correct
when written and false later, with no edit in between.** The artifact has no
vehicle to learn that its condition passed, and **the wording is identical
before and after.**

Two instances, same night, different artifact classes:

    instruction doc   a ruling ("state SameMembers as a CLASS FIELD") superseded
                      by a later finding in the SAME review round. The chapter
                      implemented the new ruling; the issue's MUST still cited
                      the dead event id.
    spec prose        "...which rejects `‖A‖` in a type position until D1."
                      TRUE at commit (03:37:21, zero TruncBar arms in
                      parse_atom_type). D1 landed 05:34:08. 1h57m.

**Neither was an authoring error, and that is the whole difficulty.** Both were
accurate when committed. Both became false without being touched. **Both read
identically before and after**, so no re-reading finds them and no reviewer's
diff shows them.

## THE CITATION MAKES IT WORSE, NOT BETTER

A stale claim carrying a **real** event id or node name survives an audit: the
reader checks provenance, finds a real ruling that really said that, and is
confirmed. The superseding event, meanwhile, had **zero** occurrences anywhere
in `docs/program/`.

⇒ **The dead claim is discoverable; the thing that killed it is not.**

## HOW TO APPLY

- **Treat "until X lands" / "not yet" / "once Y ships" as a dated claim, not a
  description.** Before repeating or relying on one, check X's `status:` and
  its named deliverables on `main`. Both instances above were one command from
  refutation.
- **Prefer a status field to prose.** A node's `status:` can change; a sentence
  cannot. When a claim's truth depends on an event, put the dependency where a
  change is recorded, not where it must be remembered.
- **Do not fix a stale forward reference by writing a fresher one.** Replacing
  *"once D1 lands"* with *"see `NEWER-NODE`"* re-arms the same trap against a
  newer target, and the next reader inherits your name on the last touch.
- **When a ruling is superseded, the supersession must be discoverable at the
  artifact the ruling was APPLIED to**, not only in the thread where it was
  reversed. A one-sided retraction relocates which copy is stale onto the copy
  being deferred to.

## THE PAIR IN OPPOSITE DIRECTIONS IS THE STRONGEST TELL

`32-grammar.md §2` carried two position-blind reports of implementation state
in one parenthetical, and each went stale in whichever direction nobody
re-measured:

    "expression position is landed now"         OVERSTATES  (head only)
    "rejects ‖A‖ in a type position until D1"   UNDERSTATES (head annotation
                                                position parses today)

**A single overclaim reads as optimism and gets argued about as a judgment
call. A matched pair in opposite directions shows the vocabulary cannot express
the distinction at all** — which is a structural finding, not a wording one,
and it is what finally moved the fix from "correct the sentence" to "add the
position axis wherever the phrase appears."

Related: [[a-claim-accurate-about-something-narrower-than-its-reader-infers]].
