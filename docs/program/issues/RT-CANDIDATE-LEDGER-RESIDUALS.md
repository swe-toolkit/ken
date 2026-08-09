---
id: RT-CANDIDATE-LEDGER-RESIDUALS
title: "Two named population questions on the merged candidate/disposition ledger were never reached, and the node that could have covered them is closed"
status: ready
owner: runtime
size: S
gate: none
depends_on: [RT-CONTINUATION-EDGE-DISPOSITION]
blocks: []
github: null
origin: Adversary hunts on merged dea1e064, f61b0b0d and 82b93e0c named both items NOT REACHED across three consecutive reports. Filed 2026-08-09 at #6i's closure, on the Adversary's closing observation that the node closing changes their status. Steward-filed (COORDINATION §2).
---

> # THESE ARE PERMANENT RESIDUALS OF A CLOSED NODE, AND THAT IS WHY THIS EXISTS

**[[RT-CONTINUATION-EDGE-DISPOSITION]] (#6i) is MERGED and closed.** While it was
in flight, these two were in-flight residuals that a later deliverable might
have covered. **It closed without reaching them, so nothing remaining in that
node will — they close only if something routes them out.** This node is that
routing.

**Neither is a defect.** Three Adversary hunts returned **no defect** on this
mechanism. These are **named gaps in coverage**, filed so a merged node does not
read as an audited one.

## The two questions

### 1. The shared resolved-continuation funnel, unaudited for a dropped consumer

`DirectCall` settles only after the **shared resolved-continuation funnel**
returns `Ok`, and that funnel covers **both** retained-frame and detached-result
consumers. It was described as a *"fourth shared-funnel correction"*, so the seat
has moved repeatedly.

**A funnel that unifies two consumers is exactly where one can be silently
dropped**, and this family's ordering is **measured rather than obvious** — the
`ComposedCall` seat's ordering was corrected only after settling-first was found
to preempt the law and replace `d8f`'s expected message.

### 2. Whether a case can fall BETWEEN the two split controls

`D2` split one conflated control into a **composite early-close** control for
candidate totality and an **isolated `D5a`** control for the discharge equality.
The split was correct — the composite was conflating two authorities. **But a
split control is where a case can fall between the halves and be asserted by
neither**, and that was never checked.

## What is owed

**Measurement, not repair.** For (1): enumerate every consumer reaching the
shared funnel and show each is observed, with a control that reds if one is
dropped. For (2): establish whether the two controls' domains are exhaustive
over the candidate population, or exhibit a case in neither.

**If either turns up a real gap, ROUTE IT — do not repair it here.** This node is
sized `S` as a measurement and a repair would re-size it.

> ### THE INVARIANT A FOURTH CONSUMPTION PATH WOULD BREAK
>
> *The pending feed is the one thing visible at both times — the composed claim
> is RECORDED during lowering and PROMOTED after verification.*
>
> ⇒ A fourth path is **a consumption decided AFTER the bridge that leaves NO
> lowering-time record.** `pending_composed_discharges` is currently the only
> such feed, and **nothing structurally forces a future post-verification
> promotion mechanism to register one.** Question (1) is where that would show
> up first.

## Do not re-file, already accepted

`D1`'s refusal retained as `HISTORICAL_D1_REFUSAL`, deliberately free and not an
oracle. The shared-`const` between production refusal and control, **offered and
declined** — two independent literals is the safe direction. `effects.rs`'s six
per-role fixtures, characterized but unverified for mutual discrimination.

## Sequencing

**Not urgent and it blocks nothing** — deliberately `blocks: []`. It is backlog
with a real frame, not frontier work, and it must not displace
[[RT-JOIN-ORIGIN-ATTRIBUTION]], which blocks [[KERNEL-NESTED-IND]].

**Do not start it under the two-lane cap without the Steward counting live lanes
first.**
