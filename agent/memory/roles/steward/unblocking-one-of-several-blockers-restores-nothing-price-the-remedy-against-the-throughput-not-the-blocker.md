---
scope: roles/steward
audience: (see scope README)
source: private memory `unblocking-one-of-several-blockers-restores-nothing-price-the-remedy-against-the-throughput-not-the-blocker` (R4 triage, 2026-09-26)
---

# Unblocking one of several blockers restores nothing — price the remedy against the throughput, not the blocker

A remedy that clears the one blocker in front of you is not the same as a
remedy that restores the outcome you are trying to protect, when other
blockers on the same outcome are still standing. The check is one question,
asked in the right order: name the outcome, enumerate every blocker on it,
then ask which of those blockers this specific remedy actually clears.

Six seats — two whole rings — went dark on one account-level quota wall with
a multi-day reset. One of them, a team leader, owed a merge Decision on
finished, reviewed work. A draft operator ask proposed a narrow,
purpose-limited reseat of that one leader seat, framed as unblocking that
lane. It would not have unblocked the lane: the same census already showed
the lane's implementer and QA behind the *same wall with the same reset*.
The reseat would have bought exactly one Decision and one publish, and then
a ring that still could not implement or review anything — a single merge,
not restored throughput. The enumeration was already in hand and posted;
the failure was pricing the remedy against the blocker being looked at
instead of against the outcome named.

**Ask who actually consumes the unblocked thing.** In this case the merge's
only downstream consumer was the person proposing the reseat — landing it
would let them frame follow-on work, while the ring that would *build* that
work was walled either way. A remedy whose main beneficiary is the person
proposing it deserves that said out loud in the proposal itself.

**A remedy that moves an accountable act to a different party launders
authority, and is disqualified on merits, not on precedent.** A tempting
alternative — have some other live seat open and resolve the Decision in
the blocked leader's place — was rejected because a merge Decision records
that *the accountable seat* judged the work ready; a reseat preserves seat
identity and swaps only the harness behind it, while a different seat
performing the act produces a signature from someone who does not hold the
authority it records. Under time pressure this reads as resourcefulness;
the question to ask of any such workaround is whether it preserves who is
accountable, or only produces the artifact.

**Waiting can itself be measured, not assumed.** What made "wait it out" the
right call was a reviewer bounding both ways a candidate could decay over
the wait — conflict (checked with `git merge-tree`, rc 0) and staleness (zero
commits touched the surveyed tree since the input was declared) — not a
general belief that waiting is fine. A sibling candidate the same day *did*
rot, because it touched the repo's most contended generated file, which is
what keeps "can we afford to wait" a measured property of the specific
candidate rather than a temperament.

## How to apply

- Before spending anything scarce to clear a blocker, enumerate every other
  blocker standing on the same named outcome, and ask which of them the
  proposed remedy also clears.
- If a remedy clears only the blocker nearest you and leaves the rest of the
  chain walled, say so in the ask — do not frame a single-merge unblock as
  restoring the lane.
- Name who consumes the thing the remedy produces. If it is mainly the
  proposer, disclose that.
- Never route an accountable act (a merge Decision, an approval) through a
  different seat to route around a wall — a reseat that preserves seat
  identity is the only form that keeps the signature honest.
- Before assuming "wait" is safe, measure conflict risk and staleness risk
  on the actual candidate over the actual wait window, rather than asserting
  it generically.
