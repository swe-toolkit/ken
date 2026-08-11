---
id: CI-OLD-PRESTATE-ROW-CURRENCY
title: "The `old`-capture flip pair still asserts pre-state elaboration is unavailable, which LANG-SPACE-PRESTATE-BIND made false -- and the soundness row's stated relation, `reject/reject at distinct gates, not a verdict flip`, is now the opposite of what the code does"
status: merged
owner: verify
size: S
gate: none
depends_on: [LANG-SPACE-PRESTATE-BIND]
blocks: []
github: https://github.com/swe-toolkit/ken/pull/1854
origin: Adversary finding evt_33wfx803mv0r7, measured on origin/main=5df41be0 against the LANG-SPACE-PRESTATE-BIND merge at 19006e37, and triaged by the Steward as a confirmed defect. The Adversary surfaced the first row; the Steward's triage found the second, which is the soundness half of the same pair.
---

## What happened

`LANG-SPACE-PRESTATE-BIND` merged at `19006e37`. It bound `s_pre`/`s_post` for
block-space operations and made `old(c)` denote the pre-state, discharging three
obligations by kernel `Refl`.

**Two conformance rows in `conformance/verify/spec-syntax/seed-spec-syntax.md`
still assert the capability is absent.** Both sit under the section headed
*"B. `old`-capture scope guard (`21 §6.4`) — the flip pair."*

## Row 1 — `old-resolves-in-space-op-ensures`

Its `expect (landed)` clause reads:

> **rejected at elaboration as unsupported**; the space-operation scope
> recognizes `old(n)`, but no obligation using it is emitted because pre-state
> elaboration is unavailable.

**Every clause of that is now false.** The landed elaborator accepts the
operation, emits the obligation, and discharges it. The row's own
`expect (model, deferred)` branch — *"once pre-state elaboration is available,
`old(n)` resolves to the pre-state projection and bare `n` to the post-state
projection"* — is a correct description of what the code now does.

**So the row is not merely stale; it holds the right answer in the branch
labelled deferred and the wrong one in the branch labelled landed.**

> ### Row 2 is the soundness row, and this is the part that matters
>
> `old-out-of-scope-rejects` is tagged `(soundness)`. Its `expect (landed)`
> states the **relation between the two rows**, not just its own verdict:
>
> > With the same `old(…)` syntax in a space-operation postcondition, scope
> > resolution recognizes `old` and elaboration proceeds to the later
> > unsupported-pre-state rejection. The landed relation is therefore
> > **reject/reject at distinct gates, not a verdict flip.**
>
> **That relation has inverted.** Pure code still rejects with
> `UnboundName("old")`. The space-operation case now **accepts**. So the pair is
> a genuine verdict flip — which is what the section title *"the flip pair"*
> always wanted, and what the row now explicitly denies.
>
> **The direction is worth stating.** The discrimination got *stronger*: a true
> accept/reject flip separates the two scopes better than two rejections at
> different gates ever did. **The defect is that the row disclaims the
> discrimination it now has**, so anyone reasoning from it will under-rate the
> guard and may go looking for a discriminator that is already there.

## Why this is a defect and not bookkeeping

A capability gate left asserting **absence** after the capability lands fails in
the worse direction. An assertion of presence-too-early gets caught the moment
someone runs it. **An assertion of absence reads as evidence the feature is not
there** — it is cited to justify deferring dependent work, and nothing reds.

`36 §4.3` is now fully implemented. These two rows are the only place in
`conformance/` still saying otherwise.

## Scope

**IN:** correcting both rows' `expect (landed)` clauses to the landed behavior;
retiring the two `[deferred — old/pre-state elaboration; OQ-Space model
decided]` tags **that this capability discharged**; restating row 2's relation
clause as the verdict flip it now is, with its `why` updated so the
absence-assertion argument matches.

**OUT:**

- **Any `crates/` change.** The elaborator is correct; the rows are wrong.
- **`OQ-Space` and `36 §4.4`.** Concurrency and isolation are still open and any
  deferral tag that names *them* stays.
- **`pub` and nested space placement**, still refused by
  `UnsupportedSpacePlacement` and still correctly recorded as unavailable.
- **A sweep of every deferred tag in `conformance/`.** Only the tags this
  capability discharged are in scope. A general currency sweep is a different
  node and should not be folded in here.
