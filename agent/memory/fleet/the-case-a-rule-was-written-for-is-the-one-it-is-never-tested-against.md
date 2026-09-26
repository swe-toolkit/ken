---
scope: fleet
audience: (see scope README)
source: private memory `the-case-a-rule-was-written-for-is-the-one-it-is-never-tested-against` (R4 triage, 2026-09-26)
---

# A rule's founding case is the one it is never tested against

A rule is written *because* of one case. That case therefore arrives
**already decided** — it is the motivation for the rule, not a test input —
so the finished rule is never actually run against it. It is simultaneously
the most important case the rule will ever face and the only one guaranteed
to skip the check. A rule that cannot correctly sort its own founding case
is a rule-shaped sentence: it still reads as a rule, because it names a real
distinction, but it decides nothing.

## The instance

`RT-CARRIER-PRODUCER-OCCURRENCE` §4, 2026-09-18. A ban on re-baselining a
test's expected value got a carve-out to let a genuine repair through, keyed
on **where the expected value came from**. But replacing an assertion
involves *two* values pointing opposite ways — the old one (the defect's own
fingerprint) and the new one — so the case the carve-out existed for
satisfied **both** cells. The rule was routed before anyone ran it against
its own example.

## The check

Before shipping a criterion, run it on the example that prompted it and
write down, in the artifact, which cell that example lands in. If it lands
in two cells, or in none, there is no rule yet. This is cheap and it fires
reliably — it is not a vague discipline like "think about edge cases",
because the founding case is always known and always singular.

**Prefer a discriminator answerable from the artifact over one resting on a
narrative about provenance.** The rejected version asked *where a value came
from* — a claim about history that nobody can check by opening the file, and
that is asserted by whoever the answer relieves of an obligation. The
working replacement asked instead: *does the artifact state a derivation
that would have predicted this value before the run?* Same intent, but
decidable by reading the artifact, with no appeal to provenance and no
reliance on the author's account of their own process. When two phrasings of
a criterion are available, take the one whose input is the artifact over the
one whose input is a narrative — especially when the narrative would be
supplied by the party the ruling favors
([[a-classification-that-relieves-you-of-an-obligation-gets-more-scrutiny-not-less]]).

## Why this cost more than a comment

The defective draft went to the reviewer and to the publisher in the same
minute. Only one of those is reversible: doc-only lands fast, and the STOP
arrived after the publisher had already launched, so the bad rule landed.
The founding-case check belongs **before** the irrevocable act, not after —
the concrete reason scrutiny cannot be scheduled after routing
([[a-route-names-a-sha-but-the-publisher-pushes-a-ref]]).

## Sibling shape: a preamble rule does not bind sections appended later

Same day, a different artifact: a ledger's opening section stated an
instrument rule outright — the population must be measured at the
implementation base, not by a source grep — and, forty lines later, named
its own blind spot: a macro-generated row that a source grep can neither
confirm nor deny, where only the harness listing answers. The first closure
record appended under both of those sections measured the new population
with a source grep anyway, and headed it "measured."

Nothing decayed and nothing was forgotten. The closure was a *new*
measurement, and a new measurement does not inherit the standard the file
set for the old one unless someone re-reads that standard. The preamble is
read when the document is being read and skipped when the document is being
appended to — and appending is exactly when the rule is load-bearing.

## How to apply

- The moment you write "the exception is X" or "this is banned unless Y",
  find the case that made you write it and state its cell out loud, in the
  artifact, before shipping the rule.
- Before adding a measurement, a row, or a count to an existing evidence
  file, re-read how that file says its numbers must be taken, and choose
  your instrument after that, not before.
- Say **reconciled**, not **measured**, until the document's own named
  oracle has actually answered.
- Watch for retreating from a refuted claim to a *weaker version of the same
  move* and calling it a correction — check what the surviving claim
  actually shares before asserting any of it; a shared precondition is not
  the same as a shared mechanism.
