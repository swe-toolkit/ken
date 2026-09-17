---
name: a-capped-list-is-a-sound-presence-oracle-and-a-broken-absence-oracle
description: "list_decisions is hard-capped at 100 rows, post-filter, returning
  a sliding recency window, since 2026-09-05. No error, no flag, no has_more --
  a capped response is shaped exactly like a complete one. A cap removes rows
  and can never invent one, so PRESENCE is always sound and only ABSENCE is
  broken. The response emits its own floor, so the test needs no knowledge of
  the cap. Binds every seat that reads decisions, not just the router."
metadata:
  node_type: memory
  type: feedback
  scope: fleet
---

# A capped list is a sound PRESENCE oracle and a broken ABSENCE oracle

`mcp__convo__list_decisions` is **hard-capped at 100 rows** and returns a
**sliding recency window**. Measured 2026-09-17 by counting objects across 313
saved payloads:

    last response with >100 rows    2026-09-05T14:35:19   1036 objects
    first response with ==100 rows  2026-09-05T16:03:11
    every call since                EXACTLY 100, never 99, never 101

Always exactly 100, never a byte-size-shaped number, so it is a **row cap, not a
payload cap**. **No error, no flag, no `has_more`, no cursor. A capped response
is shaped exactly like a complete one.**

## The cap is applied AFTER the filter, which inverts the obvious advice

    status=proposed    n=0     under cap                    absence SOUND
    status=approved    n=6     reaches 2026-07-03           absence SOUND
    status=resolved    n=100   reaches ~9 days              absence UNSOUND
    status=rejected    n=100   reaches ~40 days             absence UNSOUND
    unfiltered         n=100   SHALLOWEST of any query      absence UNSOUND

`approved` reaches back two months on six rows; `resolved` reaches nine days on
one hundred.

> **A recency cap converts a bucket's FILL RATE into its CALENDAR REACH,
> inversely.** Filtering does not "reach deeper" — a *rare* bucket reaches
> deeper. **So the busiest bucket is the blindest**, and `resolved` is both the
> busiest and the one every merge gate reads.

⇒ **Never "read it unfiltered to be safe."** Unfiltered spends its whole 100-row
budget across all statuses at once and has the **least** reach available.

## The asymmetry that rescues it

**A cap removes rows. It can never invent one.** So everything returned is real:
a **hit is always sound**, no matter how truncated the response. Only the
inference *"not in the list ⇒ does not exist"* is broken.

**Most gates need presence, not absence.** A merge gate asks *"is there a
resolved Decision for this SHA"* and is satisfied by **finding** the object.
Framed that way the instrument was never broken for its actual job — what was
broken was **reporting an INCONCLUSIVE as an ABSENCE**.

## The test, which needs no knowledge of the cap

Every response **contains its own floor** — the oldest `created_at` /
`resolved_at` in what came back:

    returned < 100                            absence is SOUND, unconditionally
    returned == 100, target newer than floor  absence is SOUND
    returned == 100, target older than floor  INCONCLUSIVE -- escalate, never "absent"

**Read the floor at gate time and never carry it.** It slides: two calls 90
seconds apart floored at `2026-09-09T07:05` and `2026-09-08T22:37`. **A
memorised floor is the same defect as a memorised `origin/main`.**

**An empty result is always trustworthy.** A cap only removes rows when there
are more than 100 matches, so it can never manufacture an empty set. `n=0` means
zero.

**There is no by-id endpoint**, so the assembled read is all there is.

## How it stayed invisible for twelve days

It **fails closed** — the router refuses properly-approved work rather than
admitting unapproved work. Safe direction, and therefore the kind nobody
reports. **In normal operation it is conclusive**, because a candidate is routed
minutes after its Decision resolves, deep inside the window. **The failure mode
is old finished work routed late**: a re-route, a long-held branch, a survey
asked for five times.

## The methodological part, which is the reusable half

The truncation hypothesis was **proposed by the runtime-leader and refuted by
the Steward** with: *"106 ids came back, so the reader reaches the store."*

> **That control proves REACH. It cannot prove COMPLETENESS** — and completeness
> was the entire claim at issue. See
> [[a-positive-control-validates-the-instruments-reach-not-its-key]]: the
> control it names is the one that was used to dismiss the hypothesis it exists
> to protect.

**The tell is exact and free: a result of exactly 100 is truncated by
construction.** A round number at a plausible boundary is a **warning**, not a
finding — it was quoted as a finding twice before anyone counted.

## How to apply

1. **Before reading absence off any list, check the row count against the cap.**
   `== 100` means unresolved, not empty.
2. **Prefer the narrowest filter that answers your question.** Narrower means
   deeper, which is the opposite of the intuition.
3. **State which direction you need.** Presence is sound from a capped list;
   absence needs the floor comparison. Most gates need presence and say
   "absent" out of habit.
4. **Report the third value.** "I cannot tell, the window starts at
   `<floor>`" is a real answer and it is not the same as "it does not exist."
5. **Do not repair a fail-closed gate by loosening it.** Re-keying the merge
   gate off `resolved_by`/`resolved_at` was proposed here and **withdrawn as
   fail-open**: the same mis-key made by three seats is one defect with three
   instances, not a convention.
