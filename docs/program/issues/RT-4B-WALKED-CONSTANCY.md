---
id: RT-4B-WALKED-CONSTANCY
title: "Two assertions landed in one candidate compose into a result neither states -- the five input populations read `(4, 2, 0, 2, 1)` identically whether fusion forms or is perturbed so it cannot, so `walked` discriminates input size and nothing downstream of it; the observation's own doc calls this a gap in attribution, which is a weaker claim than what was measured, and the next reader of a non-zero walked count is one node away"
status: ready
owner: runtime
size: XS
gate: none
depends_on: []
blocks: []
github: null
origin: Adversary finding evt_1a275gwshe00p on 2a1d87a2 (PR #2109), triaged confirmed by the Steward 2026-08-13 after independently reading both assertions at control.rs:3565-3566 and control.rs:3733. The Adversary ranked two repairs; the Steward took the first.
---

## What this is

**A comment-only correction to two doc sites. It changes no code, no test and
no assertion.**

## The measurement, which is already on `main`

`2a1d87a2` landed two assertions over the same five fields. Composed:

| cause | perturbed | keys / descriptors | `(walked, frames, recursive, slots, calls)` |
|---|---|---|---|
| `ExactSuffix` | yes, fusion cannot form | 0 / 0 | `(4, 2, 0, 2, 1)` |
| `CallIdentity` | yes, fusion cannot form | 0 / 0 | `(4, 2, 0, 2, 1)` |
| `Exact` | no, fusion forms | 1 / 1 | `(4, 2, 0, 2, 1)` |

**The perturbation moves the outcome from one key to zero and leaves all five
numbers unchanged.** `walked` is the size of the enumerator's *input*
population, which by construction does not move when a downstream relation
declines.

## Why the existing doc is not already this

The observation's doc says: *"A non-empty walked population beside empty keys
exhausts this observation route and licenses no conclusion about which planner
relation declined the candidates."*

That is a statement about **attribution** — we cannot tell *which* relation
declined. The measured fact is stronger: **the number does not move between
declining and not declining**, so it is not weak attribution, it is zero
information on that axis.

## Why now rather than later

`RT-4B-C2-REACHABILITY` is in flight and `RT-4B-UNIQUENESS-GATE-REACH` follows
it conditionally. **The first thing either would hand back is a non-zero
`walked` count**, and the 4b arc has already published six claims wider than
their instruments. This lands the constancy as a measured statement before a
seventh is drawn from it.

**It is deliberately independent of what C2-REACHABILITY answers.** If 4b turns
out blocked on cross-crate gate expressibility, `REACH` never runs and the
understated paragraph would sit there permanently.

## Not this node

- **Changing the instrument.** Recording the input population is a reasonable
  thing to do. The defect would be in what is claimed from it.
- Widening the `d2f_0` positive rows to carry the five fields. That was the
  Adversary's second-ranked option and it is rejected here: it duplicates a
  fact the identity test already pins.
- Enumeration, classifier, checker, marker, fusion-candidate, representation,
  ledger or closure-boundary repair. Gates 5 and 6 held; production unarmed.
