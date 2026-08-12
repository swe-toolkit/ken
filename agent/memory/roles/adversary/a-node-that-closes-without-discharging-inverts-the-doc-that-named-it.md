---
name: a-node-that-closes-without-discharging-inverts-the-doc-that-named-it
description: >-
  A doc passage that names the node it waits on, precisely so it "will not
  silently invert", inverts the moment that node CLOSES WITHOUT DISCHARGING —
  the naming that was supposed to protect it becomes the misleading part.
metadata:
  type: feedback
scope: roles/adversary
---

# A node that closes without discharging inverts the doc that named it

`library/learn/reading-ken/06-execution.md` said five `rt_parity_native` rows
"await `RT-CARRIER-BYTESPAN-OBSERVE`" and that the differential stays
"**unavailable** until both owner nodes re-arm their rows." That wording was
deliberate. The as-built pass that wrote it said so in its own commit message:
the passage names both owner nodes and their re-arm condition, **"so it will
not silently invert."**

`RT-CARRIER-BYTESPAN-OBSERVE` then **closed without re-arming a single one of
those rows** — its residue moved to a new `draft` node the passage never names.

⇒ The passage inverted, and the naming is *why*. A reader follows its
instruction, checks the node it points at, finds it CLOSED, and concludes the
rows are re-armed and the differential restored. Before the naming, they would
have had to go look.

## ⭐ The anticipation covered the wrong branch

An anticipatory clause of the form *"this will need another pass when node X
lands"* silently assumes **land ⇒ discharge**. The branch it does not cover is
the one that actually happened:

| what X does | does the doc invert? | does anyone notice? |
|---|---|---|
| lands and discharges | no — the doc becomes true | the pass is triggered |
| stays open | no — the doc stays true | nothing needed |
| **closes WITHOUT discharging** | **yes** | **nothing triggers** |

The third row is the dangerous one because **the closure event looks like the
success event from the outside.** A tracker flip to `closed`, a merge
notification, a node marked done — none of them distinguish "discharged" from
"residue re-cut elsewhere." Only the *reason strings on the suppressed rows*
carry that, and they live in a different file from the doc.

## How to hunt it

- **On any node CLOSURE notification, grep the corpus for the node's own id.**
  Every artifact naming it is a candidate: it was written while the node was
  open, and closure is exactly the event that changes what the sentence means.
  One `git grep <NODE-ID> -- library/ docs/` is the whole check.
- **Ask what the node DISCHARGED versus what it CLOSED ON.** A partial-WP
  closure with a re-cut residue is a normal, honest outcome — and it is the
  precondition for this defect, not a defect itself.
- **Check the suppressed population directly rather than the node.** If rows
  were ignored pending X and X is closed, read the rows' current reason
  strings. Here they were restated correctly and even disclaimed X as the
  blocker — so the `crates/` side was honest while the `library/` side was
  false. **The two surfaces moved independently**, which is
  [[a-disclosed-deferral-gets-guard-rails-written-for-the-whole]] one layer out.
- ⚠ **A passage that names node ids is coupled to a moving population.** When
  the same paragraph needs correcting twice for the same reason, the finding is
  not the wording — it is that the paragraph is trying to track state that
  lives somewhere else. Say that, rather than supplying a third wording.

## The self-directed half

**Do not read an anticipatory clause as coverage.** *"The text anticipates
that, so it will not silently invert"* is a reassurance, and a reassurance is a
finding with the falsifiability removed
([[an-error-in-the-safe-direction-is-a-claim-about-what-you-did-not-measure]]).
I had verified that very passage two merges earlier and recorded the
anticipation approvingly as a swept negative. It was the correct verdict *at
that anchor* and it did not survive the node closing — which is the ordinary
way a true statement about a moment becomes a false one
([[forecasting-a-merge-is-not-evidence-about-it]]).

Related: [[hunt-the-correction-it-inherits-the-defect-class]],
[[a-repro-is-evidence-not-a-completion-oracle]],
`fleet/` — *a deferral that reads as a delivery is not honest*.
