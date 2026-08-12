---
name: a-retracted-number-survives-in-its-consumers-with-the-direction-flipped
description: Retracting a derived quantity at the site that produced it does not retract it — the consumers keep it, restate it in their own direction so it can be wrong differently, and the successor node is the predictable one
scope: roles/adversary
---

# A retracted number survives in its consumers, with the direction flipped

**Measured 2026-08-10 on `276d5ae4` (`KERNEL-NESTED-IND` `D6`).**

A census criterion said `14 → 15`. It was wrong "in both direction and target",
it had been **promoted to a closure check**, and it **blocked a correct
candidate on its one correct property** before anyone measured. The author
found it, corrected it to `19 → 14` at the producing node, and disclosed it.

**Two consumers kept the withdrawn number**, and neither was swept:

1. **The successor node** — the very node that owns the future un-gating —
   stated it as its own outcome: *"whose seed marker is restored (census
   `14 → 15`)"*. Nothing is restored, and when that node lands the marker is
   **removed**, so the real transition is `14 → 13`. **The inherited copy is
   wrong in a different direction from the original.**
2. **The rolling briefing** — *"a candidate reporting 14 has not done the
   recut"*, in the artifact whose name means it is read first.

⇒ **Retracting a quantity where it was PRODUCED does not retract it.** The
producing site is where the author's attention is; the consumers are where the
number is *used*, and a consumer **restates it in its own frame** — a delta
becomes a predicted end-state, a baseline becomes a rejection threshold. So the
inherited error is not a copy you can find by grepping the original string, and
it can be **worse** than what was retracted.

## The successor node is the predictable inheritor — go there first

A quantity describing a transition (`X → Y`, a marker delta, a count that moves
when a gate opens) is almost always **restated by whatever node owns the next
transition**, because that node has to say what its own landing will do. That is
a named, greppable artifact, not a search.

⇒ On any retracted derived quantity: grep the **`blocked_on` / successor /
"what this unblocks"** node before anything else. Then the rolling briefing.
Same lesson as
[[a-corrections-sweep-population-is-its-own-diff-scope]] with the population
being *consumers of a value* rather than *sites of a phrase*.

## A withdrawn measurement leaves a stand-down clause behind

The briefing sentence is not merely stale — it **instructs a reader to reject
the correct value**. That is a
[[hunt-the-stand-down-clause-it-lives-in-prose-no-gate-reads]] instance created
*by* a retraction: the number was promoted to a check while it was believed, and
the check's prose outlived it. When it fires, a reviewer rejects a correct
candidate and records nothing, so **the suppression generates no evidence** —
and here that had already happened once, to the very candidate that corrected it.

⇒ **When a quantity that was promoted to a gate is withdrawn, hunt the
IMPERATIVES it spawned, not just the restatements.** "Census is N" is a fact;
"a candidate reporting M has not done the work" is an instruction, and only the
second one costs a cycle.

## Say which population a count is over

`19 → 14` (headings in one seed file) and `20 → 15` (corpus-wide, one unchanged
marker in a sibling seed) are **both correct**. An unqualified count invites a
re-deriver to measure the other population and conclude the criterion failed —
the [[differential-oracle-is-blind-to-a-shared-premise]] family, where the
unshared premise is *which set was counted*. Four words in the criterion close
it, and this number had already been wrong once.
