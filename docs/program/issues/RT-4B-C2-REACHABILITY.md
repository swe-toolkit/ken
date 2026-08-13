---
id: RT-4B-C2-REACHABILITY
title: "Establish whether a witness driving through `ken-elaborator` can reach the `D2f` observation at all, and at what cost -- the question 4b opened with, never answered, and the reason every 4b measurement so far has been taken on in-crate D2j fixtures instead of the real C2 source; the answer decides whether the reach node re-points or 4b's honest status becomes blocked-on-cross-crate-expressibility rather than awaiting-a-count"
status: ready
owner: runtime
size: S
gate: none
depends_on: []
blocks: [RT-4B-UNIQUENESS-GATE-REACH]
github: null
origin: Architect evt_dnmsmt5wsmmd, reshaping the Steward's intended re-pointing question one level up after evt_6hfw027f43cgg established that the (4,2,0,2,1) measurement was taken on perturbed D2j comparators rather than C2. Framed by the Steward 2026-08-13.
---

## What this is

**This is a question, not a repair.** Deliverable: **can a witness driving
through `ken-elaborator` reach the `D2f` observation, and at what cost?**

Nothing about 4b moves until that is answered, and **it is the same wall 4b
opened with.** Nothing since has moved it.

## Why every 4b measurement so far missed C2

The `D2f` observation is `#[cfg(test)]` inside `ken-runtime`.
`#[cfg(test)]` is active only when compiling that crate's **own** tests.
`C2_MIXED_SOURCE` drives through `ken-elaborator`, which depends on
`ken-runtime` and therefore links a build **where those calls do not exist.**

⇒ **Every 4b increment has gone in-crate because in-crate is where the
instrument is reachable.** That is not a series of implementer choices; it is
the shape of the obstacle. `RT-4B-ENUMERATION-INPUT-SIZE` measured D2j
comparators — two of them **perturbed so fusion cannot form** — and the reading
drawn from it was withdrawn.

## The two outcomes, and they are NOT the same tracker entry

| answer | consequence |
|---|---|
| **reachable at acceptable cost** | `RT-4B-UNIQUENESS-GATE-REACH` re-points at C2 and everything downstream follows. |
| **not reachable at acceptable cost** | **4b's honest status is BLOCKED ON CROSS-CRATE GATE EXPRESSIBILITY — not "awaiting a measurement."** The tracker says so in those words. |

> **The second outcome is the one a future seat would most easily misread as
> *someone just needs to run the count*.** It is not. If the answer is no, no
> count is available to run, and saying "pending a measurement" would hide a
> wall behind a chore.

## What to report

1. **Whether it is achievable**, and by what mechanism — a feature gate, an
   installable sink, or something else. **Name the mechanism; do not build it.**
2. **What it costs**, in the terms that decide it: production footprint if any,
   whether it survives the "no production API, no second observer" constraints
   the Architect set for 4b, and whether it changes any artifact.
3. **If several mechanisms work, say so and rank them.** The choice is a
   Steward/Architect call, not the implementer's.

## Not this node

- **Building the mechanism.** This reports; it does not implement. A candidate
  that lands a feature gate has answered a question nobody asked yet.
- **Counting anything.** Reach is the successor and is conditional on this.
- **Attribution**, which is conditional on reach.
- **Relaxing the 4b envelope** — no second observer, no production API. If the
  only achievable mechanism breaks one of those, **that is the finding**, and
  it is the Architect's to rule on, not something to absorb.
- Enumeration, classifier, checker, marker, fusion-candidate, representation,
  ledger or closure-boundary repair. Gates 5 and 6 held; production unarmed.
