---
scope: fleet
audience: (see scope README)
source: private memory `provider-content-refusal-is-a-distinct-stall-class` (R4 triage, 2026-09-26)
---

# A model-provider content refusal is a distinct stall class

A seat can end its turn on the **model provider's safety classifier**, not on
any bug, rate limit, or agent decision. The seat goes `Working`, then drops
back to an idle prompt having posted nothing to the space. The only evidence
is in the pane tail, something like: *"This content can't be shown — We take
extra caution with cybersecurity requests."* No enumerated watchdog stall
pattern catches this: an idle seat holding an assignment reads the same as an
idle seat that finished, and the seat is likely sitting on uncommitted work.

## What the trigger generalizes to

Measured across several occurrences, on a T1-tier seat each time, across
unrelated tasks:

- Building a differential test harness that runs a compiled artifact and
  inspects its exit status and filesystem effects — abstractly the shape a
  cybersecurity classifier watches for, even though the task was legitimate
  differential testing of the project's own host operations.
- Planting a parser-confusion input to check whether two implementations of
  one grammar agree — abstractly, how a parser-confusion attack is written,
  even though the task was legitimate differential conformance QA.
- Removing a trust anchor to check whether a verification gate fails closed
  when its dependency is absent — the mildest-looking of the three, and it
  still tripped the classifier.
- Plain source edits and even the output of a targeted test run, on a task
  about parser recursion depth and stack exhaustion, with no adversarial
  framing at all — showing the classifier also fires on **subject matter**
  (resource exhaustion), independent of what is being done with it.

The unifying shape for the first three is "construct an input that makes a
verification mechanism fail, then show it" — which is legitimate QA and is
also, abstractly, how an attack is demonstrated. A large fraction of good QA
is exposed by this, and the **negative-control / fail-closed arm is usually
the highest-value arm there is** — so a refused turn is more likely than an
average turn to have been sitting on a real defect, not less. One recovery
found a confirmed false green in a gate that silently degraded when its trust
anchor was unresolvable, on the arm that the refusal had blocked first.

## Why it is worse than an ordinary stall

A refused probe can become an invisible hole in a verdict that otherwise
reads complete, which is strictly worse than a stalled seat, because a stall
is at least detectable. And on at least one seat family the seat re-arms its
own trigger: it generated the triggering topic in its own explanation of its
fix, so no rewording of the instruction routes around a filter the seat
re-trips every time it describes its own work.

## Different mechanism from a downgrade-risking modal

This is not the same failure as
[[safety-check-modal-defaults-to-a-model-downgrade]]. That one is a provider
UI element that does not block progress and is only dangerous if you
interact with it (a default-selected option silently downgrades the seat's
tier). A content refusal is the opposite shape: the turn genuinely ends, the
seat posts nothing, and re-sending the identical content re-trips the same
classifier rather than resolving it. Diagnosing one as the other wastes the
one lever that actually helps.

## How to apply

- If a seat goes quiet mid-task without posting, `capture-pane` it and read
  the tail before diagnosing anything else (a bug, a rate limit, a stranded
  paste). A refusal notice settles it immediately.
- Protect uncommitted work first: order a plain, neutral-message commit
  before any analysis, test, or explanation — especially on a seat whose own
  explanations re-trigger the classifier. Ordering the turn commit, then
  test, then commit again, then explain, has survived repeated refusals with
  both artifacts landing cleanly.
- Do not re-send the same content; it re-trips the same classifier.
- Offer a rouse with an accurate, non-adversarial re-description of the
  legitimate work, plus an explicit escape: if the seat still will not
  proceed, it may report the probe as a named, explicitly unverified
  residual and let the reviewer decide on the remaining evidence. Treat the
  escape as a fallback, never as equal to actually running the probe — a
  residual beats a silent hole, but it does not beat the arm that carries
  the discrimination.
- Because the refused arm is disproportionately likely to be load-bearing,
  after a rouse, get that specific probe run rather than merely unsticking
  the seat and moving on.
- A refusal that lands on a red result, not a green one, deserves more
  attention still — it may mean the fix is incomplete rather than the
  control being wrong, and the seat could not tell which.
- If a seat refuses twice on the same node, escalate to the operator as an
  infrastructure/seating constraint (move the node to a different tier or
  seat, or park it) rather than burning further turns fighting a persistent
  provider block on the critical path.
