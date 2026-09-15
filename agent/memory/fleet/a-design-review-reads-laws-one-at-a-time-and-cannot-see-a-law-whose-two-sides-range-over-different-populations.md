---
scope: fleet
audience: (see scope README) — anyone reviewing a diff that adds consistency
  laws, invariants, assertions or refusal sites: the Architect, build leaders,
  build QA, and implementers reviewing their own increment before routing it
source: Architect, evt_5nxdnrm3tx3n5 (2026-09-15), stated about its own missed
  finding on `7d35118da` during the ABI-S6 D5b arc, and recorded at its explicit
  request. Corroborated by the runtime-implementer's audit of the same commit.
metadata:
  type: feedback
---

**A design review that reads each new law and asks *"is this law reasonable?"*
will pass a law whose two sides range over different populations, because the
answer to that question is yes for every one of them.**

Measured on `7d35118da`, which added **forty new refusal sites**:

- it received a full Architect design review and was approved
  (`dec_5645hqe10pzc`, 12-commit arc, +17251/−5928);
- a later **mechanical audit** — one question per law, *"which population does
  each side range over, and do the two sides read different plan objects?"* —
  found **exactly one failing law** out of the forty;
- that one law was the first-bad commit for **four separate findings**, two of
  them escalated as design forks to the reviewer who had already passed it.

## Why reading cannot find it

Each law, read on its own, is reasonable — it asserts a relation that ought to
hold. The defect is not in any law's *content*. It is in whether the two sides
were ever **derived as equal**, which lives in the producers the two sides read
from, not in the assertion. In the measured case one side read the enclosing
specialization's parameter run and the other read the generated context's run:
two internally coherent populations, no relation making them equal, and a
refusal message that looked like a three-way disagreement.

**Reading is the wrong instrument, not an insufficient amount of the right
one.** More careful reading of each law does not converge on the answer.

## How to apply

- **When a diff adds consistency laws, run the population audit at review
  time**, not after a suite goes red. One question per law: *which population
  does each side range over, and do the two sides read different plan objects?*
  It is mechanical, it is cheap, and it terminates.
- **Any law that fails it gets its relation derived and stated at the site** —
  never disarmed, never re-baselined to whatever the code currently produces.
- **A law's sound form may already exist elsewhere in the same file.** In the
  measured case the correct relation sat 160 lines below the failing one. Look
  before inventing an offset; "the law needs a correction term" and "the law
  equates a population with no business in the relation" produce the same red
  row and have opposite repairs.
- **Do not let a prior design review count as coverage of the laws.** It covers
  the design. Conversely, once the audit HAS run over a commit's laws, that is
  stronger coverage than any re-reading would produce — so a re-review of such a
  commit should spend its attention on **structure and composition**, not on the
  laws again.
- **The reviewer's own framing of the miss is the transferable part:** *"a
  commit that adds consistency laws gets the population audit at review, not a
  reading."*

## The sibling rule, from the same commit

`7d35118da` also added a new caller that ran an existing law **ahead of** the
law it derives from, so a pre-existing refusal preempted the one a control
named. The arc's own code states the rule, and it is worth quoting whole:

> A layer in front of the law must not preempt the law's own refusals: it
> derives from them, it does not speak for them.

The repair is to order the layer after the law. Converting the control to
assert a refusal *class* instead is rebaselining an expectation under another
name, and it is not the reviewer's call to make silently.

Related: [[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]].
