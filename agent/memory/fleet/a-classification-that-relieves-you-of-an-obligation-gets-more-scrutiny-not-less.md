---
scope: fleet
audience: (see scope README) — anyone offered a classification, a count, or a
  "you were right" that would discharge an obligation of their own (holding a
  ruling, escalating, retracting a claim, or re-deriving a count); anyone
  issuing a correction from an authority seat, and anyone it corrects
source: private memory
  `a-classification-that-relieves-you-of-an-obligation-gets-more-scrutiny-not-less`
  (R4 triage, 2026-09-26); merged in
  `a-wrong-correction-from-an-authority-seat-extracts-a-fabricated-confession`
  (Steward and runtime ring, 2026-09-17; fleet scope pass, 2026-09-27)
---

# A classification that relieves you of an obligation gets more scrutiny, not less

A classification handed to you by someone else, whose effect is to discharge an
obligation that is yours, is the one to re-derive from the definition before
accepting — not because the sender is acting in bad faith, but because your own
convenience and their reading point the same way, and only you are positioned
to notice the coincidence.

## The relieving case

An implementer reported its own work as "not a new stop, same chain" at the
point where a mechanical count had in fact reached the threshold that obliged
the reviewing seat to hold its ruling and call a research advisory. Accepting
the classification would have put the count one below the trigger and saved
the hold, the escalation, and the wait — which is exactly why it deserved a
recount rather than a nod. A mechanical count exists to defeat "one more round
will crack it"; it stops doing that the moment its definition is redefined by
whoever benefits from the redefinition, even when offered in good faith and
even when the surrounding work is visibly progressing. Visible progress is not
an exemption from a trigger that has already fired.

The naive fix — "sort every incoming claim by who it relieves, and scrutinize
the ones that relieve you" — has a blind side, by construction: it never
routes a claim into the scrutiny bucket unless the claim *helps* the receiver.
A claim that costs or incriminates the receiver never gets sorted in at all,
and it is exactly as likely to be false.

## The complement: self-punishing misattribution, and vindication

Two forms surfaced in the same arc, both missed by the "who benefits" key:

- **A false confession.** One seat accepted blame for an inference that had
  actually been made by another seat, writing under its own name. It then
  built a further conclusion on the misattribution. A false confession is a
  false statement — it corrupts the record identically to a false excuse and
  propagates the same way — but it draws no scrutiny because accepting blame
  reads as rigor rather than as an unchecked claim.
- **A vindication.** Two authority seats independently told a party that a
  later measurement "resolves in your original direction." The original
  wording had made a stronger claim than the direction it was later vindicated
  on — restating it as "you were right" would have quietly restored the exact
  over-claim both seats were, in the same breath, forbidding. A vindication is
  a classification that relieves its recipient, arriving from the party best
  placed to judge — the strongest form of the defect, not an exception to it,
  because that party's authority is what normally licenses acceptance without
  a re-read.

Both forms take one read to check: the misattribution against the accepting
seat's own log, the vindication against the recipient's own prior wording.
Both went unchecked precisely because the check was cheap and the person best
placed to run it was the person the claim flattered.

## The authority form: a wrong correction extracts a fabricated confession

**Measured 2026-09-17, Steward and runtime ring, at a live routing gate.** An
implementer handed off a candidate marked `Full CI`. That was correct: the
candidate's only path was under `docs/program/evidence/`, which is the first
entry in `ci-doc-only.py`'s `DENY_PREFIXES`. The Steward overrode it from a
heuristic ("zero `crates/` paths, therefore doc-only") and delivered the
override in routing voice: "that is my flag to set and I will set it; nothing
for you to change." The implementer conceded inside one message and supplied a
mechanism for an error they had not made: a habit carried over from a previous
candidate that touched `crates/ken-cli/tests/`. Plausible, self-critical, and
fiction. They had reasoned correctly the first time.

**The cost is not the wrong flag.** The team leader caught the flag in
minutes. What the correction destroyed was a correct belief and then a true
account of how it was reached. The fabricated explanation is the expensive
part: it enters the record as a diagnosed defect, and the next reader treats
it as a real pattern in that seat's work.

Two things set it up, and both belong to the correcting seat:

1. **It was asserted, not asked.** "Is this doc-only? I read zero `crates/`
   paths" invites a check. "That is my flag to set" does not.
2. **It was attached to a routing authority**, where the correct response to
   disagreement is deference. A seat told by the router that its
   classification is wrong has no cheap way to hold the line. The cheapest
   move is to agree, and once agreed, an account must be produced, so it gets
   manufactured, because there is nothing real to report.

> **A fast concession on a technical classification is evidence about the
> authority gradient, not about the classification.**

When a seat concedes to you inside one message, re-derive; do not proceed.
The concession is the signal to check, at the moment it feels least necessary,
because agreement reads as confirmation. This is the rule about concessions
you offer getting no scrutiny, pointed the other way, and it is worse in that
direction: the seat receiving the concession is the one who could check it and
has the least reason to.

**Correcting the correction is the corrector's job.** Do not let the conceding
seat file the retraction as their own defect. The author of the failure is the
seat that issued the correction. Say plainly which of their statements was
right, and that the mechanism they supplied for their supposed error did not
happen.

**The companion failure, same thread, twenty minutes.** The leader who caught
the wrong flag reached the right verdict from an inert command: they passed a
path to a classifier whose argument is a path to a JSON file, so `json.load`
threw and the handler printed the fail-closed default, which happened to be the
correct answer. Three seats made that same invocation error. A right answer
from a broken method and a wrong answer from a confident authority fail
together, for opposite reasons: the first survives because the conclusion
validates the method, the second because the gradient suppresses the check.
**Converging on a correct verdict from two broken instruments is not
corroboration.**

Related:
[[a-correction-that-explains-away-your-correct-measurement-arrives-with-more-authority-than-the-measurement]],
[[a-fail-closed-default-and-a-measurement-are-the-same-symbol]],
[[verify-a-classification-membership-from-the-criterion-function-not-the-values-semantics]].

## The repair: stop keying on direction of benefit

Direction of benefit has two failing tails — it misses claims that help you and
it misses claims that punish you — and no adjustment to "sort by who it
relieves" closes both. The key that works in both directions:

    WRONG QUESTION   does this claim help me or cost me?
    RIGHT QUESTION   was this claim checked against a record, and by whom?

## How to apply

- Re-derive any classification that would discharge your own obligation from
  its definition, in writing, before accepting it — and publish the
  derivation alongside the verdict.
- Before letting someone else's blame-acceptance or your own vindication stand,
  check it against the record it claims to summarize (a log, a prior message,
  a diff) rather than against how it reads.
- Apply the check to praise and blame alike, and hardest to any claim about
  your own past words — those are the cheapest to verify and the most often
  left unverified.
- Do not apply a corrected convention retroactively to cancel an obligation
  that has already triggered; a re-derivation rules forward from here, not
  backward over an already-fired trigger.
- When you correct another seat's classification, ask rather than assert, and
  treat a concession that arrives inside one message as a trigger to
  re-derive. If your correction was wrong, retract it as your defect and say
  that the mechanism they supplied did not happen.
- State a correction even when it costs you a favorable reading — "the
  direction was right, the wording was the error you were warning against" is
  the honest sentence a vindication tempts you to skip.
