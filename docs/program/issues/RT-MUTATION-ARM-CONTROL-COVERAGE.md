---
id: RT-MUTATION-ARM-CONTROL-COVERAGE
title: "The backend's refusal-mutation corpus is 410 non-Exact arms across 76 enums, and we do not know how many drive a live control. An arm no control drives is a mutation the suite cannot detect -- it produces no signal at either end, so every base-vs-tip diff of outcomes is structurally blind to exactly it."
status: draft
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Named as the residual by runtime-implementer at evt_5wsfrftqzbdp0 (2026-09-15), after the Steward asked for a fallen-guard census during the ABI-S6 D5b regression hunt and the implementer showed the census was already complete by another route. The implementer attempted the coverage measurement, found the instrument untrustworthy, and deliberately reported no numbers. Steward-filed per COORDINATION section 2 and explicitly routed OUT of the D5b candidate."
---

> # FILED 2026-09-15 BY THE STEWARD AS `draft`, DELIBERATELY NOT RELEASED.
>
> **Queued behind the active lane.** This is a real measured gap, not lane-1
> work, and it must not compete with the ABI-S6 D5b repair. Release it when the
> lane allows.
>
> **Do not hand this to the D5b implementer as an addition.** They named the
> gap and correctly declined to build the instrument inside a red-CI repair.

## The finding

The backend's refusal-mutation corpus has **76 mutation enums and 410
non-`Exact` arms**. Each arm is intended to be driven by a control that asserts
the mutated compile is refused. **How many arms actually drive a live control is
unknown.**

An arm that drives nothing is not a weak test — it is a mutation the suite
**cannot detect at all**, while still counting toward the corpus's apparent
size. The corpus looks like coverage it may not have.

## Why the cheap measurement cannot find it

During the D5b hunt the question "which guards fell between the base and the
tip?" was answered completely and for free, by this argument:

> every probe is driven by a control; the base is 1011 passed / 0 failed, so
> every control passes at the base; therefore any guard that fell must surface
> as one of the tip's failures. It cannot hide.

That argument is sound, and it is exactly why it cannot be extended here. **An
arm with no live control produces no signal at the base and none at the tip**,
so it appears in no diff of outcomes. The method is structurally blind to
precisely the population this node is about. Answering it needs an instrument
that reasons about *what drives what*, not about outcomes.

## The attempt already made, and why its numbers were withheld

The implementer tried the obvious version — count arms never referenced by a
control — and rejected its output. Two defects, both disqualifying:

- it **undercounted source files**, so its denominator was wrong;
- it **could not distinguish an arm driven once inside a loop from an arm
  nobody drives**, which is the entire distinction being measured.

**No numbers from that attempt are recorded anywhere, deliberately.** A coverage
claim whose own coverage cannot be vouched for is worse than no claim, because
it will be cited. Do not go looking for them, and do not treat "410" as a
partial answer — it is the corpus size, not a coverage figure.

## Deliverables

- **D0 — a trustworthy instrument.** Attribute every mutation arm to the
  control(s) that drive it, **by construction rather than by grep**. It must
  handle an arm driven from inside a loop or a helper, and it must state its own
  file coverage so the denominator is checkable.
- **D1 — the census.** Every arm classified: driven, driven-only-vacuously, or
  undriven.
- **D2 — dispositions** for the undriven arms: wire a control, or delete the arm
  and record why. An arm nobody drives should not survive unexamined in either
  direction.

## Acceptance criteria

- **AC-1 (the instrument's own positive control, and this is the load-bearing
  one).** Seed a **known-undriven** arm and show the instrument reports it;
  seed a **known-driven** arm and show it does not. **Both arms, both
  directions, demonstrated.** An instrument for measuring coverage that has only
  ever been run on unknown input has not been shown to measure anything — which
  is exactly the defect that disqualified the first attempt.
- **AC-2.** The instrument states its file coverage, and that figure is checked
  against an independent count of the corpus's source files.
- **AC-3.** D1's census is reproducible: same tree, same numbers.
- **AC-4.** No regression, green in CI (never a local `--workspace` run;
  COORDINATION §12).

## Why this is worth an instrument rather than a spot check

The D5b arc produced one fallen guard, three moved guards and 28 over-refusals
from a single commit. Every one was found because a control existed and failed.
**The corpus is the fleet's primary evidence that the backend's refusal seats
work at all**, and its value is entirely a function of how much of it is live.
A sampled answer here would restate the problem — see
`prefer-a-structural-answer-over-a-sampled-one` in the corpus of lessons.

## Contention

Read-only over the mutation corpus and its controls; D2 may touch individual
arms. **Contends with any active backend arc** that is adding or editing
mutation arms — sequence it when the backend is quiet.
