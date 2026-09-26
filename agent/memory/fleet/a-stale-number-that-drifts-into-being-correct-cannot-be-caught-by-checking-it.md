---
scope: fleet
audience: (see scope README) — anyone carrying a headline count or metric in a
  standing prompt, handoff, or ledger reference across time
source: private memory
  `a-stale-number-that-drifts-into-being-correct-cannot-be-caught-by-checking-it`
  (R4 triage, 2026-09-26)
---

# A stale number that drifts into being correct cannot be caught by checking it

A figure recorded from a bad derivation can become true later, purely because
the world moved onto it while nobody was looking. Once that happens, every
later confirmation agrees with the number, so the error is only findable
against the source artifact's own arithmetic — never against the number
itself.

## Why this is worse than an ordinary stale figure

A standing prompt once carried a lane's headline metric as "22 attrs minus 8
exemptions equals 14 ignored rows." The authoritative ledger's own figures
were 23 listed and 15 after exemptions — both operands wrong by one. The
conclusion, 14, was nonetheless correct at read time, because between the
number being written and being read, one row had been cleared by an unrelated
commit, moving the true count from 15 to 14 and landing exactly on the wrong
answer.

Every check available agreed with the stale figure: re-counting the tree
agreed, and asking another seat would have agreed too. The error was visible
only against the ledger's own arithmetic — the artifact that produced the
number — never against the number as compared to the world.

An ordinary wrong figure announces itself the first time anyone measures. A
figure that has drifted into correctness is protected by every subsequent
verification, and the bad derivation underneath stays live: the next time the
world moves, it is wrong again, now with a track record of confirmations
behind it that makes it harder to question.

## How to apply

- Verify a carried number against the artifact that produced it, not against
  the world. Agreement with today's tree is consistent with a broken
  derivation; agreement with the source's own stated arithmetic is not.
- Record the derivation and its base alongside any carried count — "ledger at
  base `<sha>`, N listed minus M exempt" — so a future reader can re-run the
  argument instead of re-running the count.
- When a carried number is confirmed, ask "by what, and could that same check
  also have confirmed a wrong one?" That question is what separates a check
  from a coincidence.
- Reach for the instrument the source artifact itself names as authoritative,
  not the cheapest one available — a source grep can return numbers that are
  neither the intended population.
- A delta argument computed from commit history (e.g. `git log -S`) inherits
  the blind spot of whatever text it keys on; label it "delta argument, not a
  measurement" when that is what it is, so a reader knows not to treat it as
  ground truth.
