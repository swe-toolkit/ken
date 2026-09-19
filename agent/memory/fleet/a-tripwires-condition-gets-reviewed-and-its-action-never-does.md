---
scope: fleet
audience: (see scope README) — anyone who arms a standing trigger, pre-commits a
  remedy, or is handed one to fire: leaders holding reseat/escalation tripwires,
  the Steward arming them, QA holding stop-the-line conditions
source: 2026-09-19 — a Steward-armed reseat tripwire fired correctly on the
  runtime ring. Countermanded on two independent defects: it counted the wrong
  population, and its remedy had no executable meaning. The second was found
  only because the leader asked "what does this mean operationally?" at fire
  time.
---

# A tripwire's condition gets reviewed; its action never does

A standing trigger has two halves — **when it fires** and **what then happens**.
Every review of it goes to the first. The threshold gets argued, tightened,
counted against; the remedy is a verb nobody executes until the day it fires.

Measured: a tripwire reading *"if the seat mis-instruments a third time, reseat
immediately and do not ask me again"* was read at least three times — by the
Steward arming it, by the leader firing it, and by the Steward countermanding it
— and **not one of those readings checked that "reseat" named an action anyone
could perform.**

It did not. The word had two documented meanings in the corpus:

| reading | what it actually is | effect here |
|---|---|---|
| reseat a **lane slot** to a different **ring** | an operator ruling on lane composition | would move a whole lane off its ring — wildly disproportionate to one AC |
| reseat a **seat** to a different **model/tier** | changes a parameter on the completion call | **does not reset the seat at all** — context, playbook and thread state all survive |

The second is settled fleet knowledge
([[model-swap-does-not-reset-the-seat]], operator verbatim: *"a model swap does
not reset the seat. It just changes a param in the completion call."*). So the
trigger's remedy was **disproportionate under one reading and inert under the
other**, and its whole purpose — put fresh eyes on the question — was reachable
by neither.

## Why the defect is invisible while the trigger is healthy

A condition is exercised constantly: every tick someone asks *"are we at three
yet?"* **An action is exercised exactly once, at the worst possible moment** —
under time pressure, with a lane stopped, by whoever happens to be holding it.
Until then it is prose, and prose that has never been run is indistinguishable
from prose that works.

This is the same shape as a disaster-recovery plan nobody has rehearsed, and it
resists the usual defence: re-reading the tripwire does not help, because the
reader checks whether the *threshold* is right and the action reads as a settled
detail.

## How to apply

- **Arming a trigger: name the action as a command or a seat, never a verb.**
  Not *"reseat"* — *"hand the open question to `@<seat>`, with the evidence
  gathered so far."* If you cannot write who does what, the remedy does not
  exist yet and the trigger is not ready to arm.
- **Holding a trigger someone handed you: resolve the action the day you receive
  it, not the day it fires.** One question — *"what exactly would I do?"* — and
  if you cannot answer, ask then, while nothing is blocked.
- **Firing one: ask the mechanism question even while executing.** The leader
  here fired without asking (correct — a pre-commitment re-litigated at fire
  time is not one) **and** asked what it meant operationally. Doing both is what
  surfaced the defect; doing either alone would not have.
- **Prefer a remedy the ring can already perform.** Fresh eyes on a question is
  a different existing seat reading it. Replacing a seat was never the mechanism
  for that, and the expensive-sounding remedy is the one least likely to have
  been checked.

## The corollary that made the countermand honest

When you countermand your own instrument, the first argument you reach for will
be the one that **relieves you**, and it will usually also rest on a judgment
about the outcome you were hoping for. Here that was *"it counted self-caught
errors rather than published ones"* — true, and leaning on how well the seat was
doing.

⇒ **Keep looking until you find the defect that holds regardless of the
outcome.** The unexecutable remedy would have been a defect if the seat had
performed badly, if every error had reached publication, and if nobody had read
the work at all. **An argument that survives the counterfactual is the one to
lead with**; the convenient one goes second, if at all.

Condition-side sibling:
[[a-gate-is-an-instrument-and-the-verification-method-is-the-gate]].
Unrehearsed-prose sibling:
[[a-requirement-in-an-advisory-section-is-never-discharged]].
Self-serving-classification sibling:
[[a-classification-that-relieves-you-of-an-obligation-gets-more-scrutiny-not-less]].
