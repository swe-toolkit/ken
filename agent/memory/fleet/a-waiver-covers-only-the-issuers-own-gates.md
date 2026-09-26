---
scope: fleet
audience: (see scope README)
source: private memory `a-waiver-covers-only-the-issuers-own-gates`,
  `a-waiver-reaches-only-as-far-as-what-the-waiver-holder-owns` (R4 triage,
  2026-09-26)
---

# A waiver covers only the issuer's own gates

A reviewer or authority can validly waive a requirement they own — their own
review, their own vote. They cannot waive a requirement that binds a
different seat, even when both requirements sit adjacent in the same sentence
and both stand between a candidate and `main`. A waiver is a scope claim, and
nothing about how such sentences are written marks where the issuer's
authority stops.

## The evidence

A merge pre-authorization from the Architect read: *"no re-review, no new
Decision, no vote from me."* Clause by clause:

    "no re-review"     the Architect's own review     theirs to waive   VALID
    "no vote from me"   the Architect's own vote        theirs to waive   VALID
    "no new Decision"   the LEADER's artifact, required  NOT theirs       VOID

`COORDINATION §14b` binds the **Steward**, not the Architect: the Steward
"does not route on approvals for a different SHA, an unresolved Decision, or
an unverified scope." No Decision existed. The Architect's waiver of a
Decision purported to relieve a seat that was not even party to the
sentence.

The implementer relayed the sentence verbatim; the team leader routed on it
without a Decision. Both deferred to a review authority, which is normally
the correct move — the three clauses are grammatically identical, and nothing
on the surface distinguishes the two an author may issue from the one they
may not. This is a property of how such waivers get written, not a lapse in
how they were read, so the fix does not belong in the relay layer: installing
"audit whether the authority overstepped" as a relay duty would poison every
ordinary deference that fires correctly nearly every time.

The check belongs at the seat the requirement actually binds. Holding the
route and asking for a `propose_decision` + `resolve_decision`, with no new
review required, cost about four minutes and one post — cheap enough that
saying the cost out loud teaches the ring the gate is inexpensive, rather than
teaching it to route around the gate next time.

**A second, independent fault compounded the first.** Auditing the relay
against the Architect's own sentence (after the Architect declined to let the
implementer own the wording) turned up a one-word drift:

    Architect wrote   "no vote FROM ME"   scopes to ONE SEAT
    relay said         "no vote OWED"      says NOBODY owes one

The possessive was the whole scope, and dropping it widened a one-seat waiver
into a fleet-wide one. Either fault alone would have been caught by the other
check; together they passed. A paraphrase of a scope claim is a new scope
claim, not a relay of the original — forwarding someone's waiver means
quoting it, or diffing the paraphrase against the source before it moves.

## How to apply

- Before writing "no X is needed," name who owns X. Waive only what you
  personally would be the one to withhold; say the rest is unaffected rather
  than leaving it implied by omission.
- If a waiver would relieve **you** of a gate, check whether its author owns
  that gate — this check is cheap and reliable only at the seat the
  requirement actually binds, because that is the only seat that reads the
  rule as being about itself.
- When forwarding someone else's waiver, quote it verbatim or diff your
  paraphrase against the source sentence before sending. A dropped possessive
  or qualifier changes who the waiver covers.
- A Decision is the durable record of *why* a SHA is cited on `main`; a
  reviewer's own vote or review is theirs to skip, but the Decision artifact
  is not — it binds the routing seat, not the reviewer.
